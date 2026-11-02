//! `Havok SDK/hkprigidbody.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use std::cmp::Ordering;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00ca9210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00ca_80a0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00ca9210(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00ca_80a0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// The motion base constructor (`00c86530`, `this`, `(position, rotation, 0)`) every motion
/// type below starts with.
const MOTION_CONSTRUCT: u32 = 0x00c8_6530;
/// The world's operation queue (`00c91160`, `this` = the world, `(operation)`), used while
/// the world's lock count (+0x94) is non-zero.
const WORLD_QUEUE_OPERATION: u32 = 0x00c9_1160;
/// `hkpEntity::isActive` (Xbox PDB, `00c9bff0`, `this` = the entity, `(out)`): returns a
/// pointer to a byte, non-zero when the entity is active.
const ENTITY_IS_ACTIVE: u32 = 0x00c9_bff0;
/// `hkpEntity::activate` (Xbox PDB, `00c9c1d0`).
const ENTITY_ACTIVATE: u32 = 0x00c9_c1d0;
/// `hkpWorldCallbackUtil::fireInactiveEntityMoved` (Xbox PDB, `00d017b0`, cdecl `(world,
/// entity)`).
const FIRE_INACTIVE_ENTITY_MOVED: u32 = 0x00d0_17b0;
/// `hkpSimulation::collideEntitiesBroadPhaseDiscrete` (Xbox PDB, `00cf9c60`, cdecl
/// `(entities, count, world)`).
const COLLIDE_ENTITIES_BROAD_PHASE: u32 = 0x00cf_9c60;
/// Two world calls that finish a critical section (`00c91120`, `00c91140`, `this` = the
/// world).
const WORLD_FLUSH_FIRST: u32 = 0x00c9_1120;
const WORLD_FLUSH_SECOND: u32 = 0x00c9_1140;
/// `hkpWorldOperationUtil::setRigidBodyMotionType` (Xbox PDB, `00cb6f70`, cdecl `(body, a, b,
/// c)`).
const SET_RIGID_BODY_MOTION_TYPE: u32 = 0x00cb_6f70;
/// `hkpWorldCallbackUtil::fireEntitySetMotionType` (Xbox PDB, `00d006e0`, cdecl `(world,
/// entity)`) and `hkpEntityCallbackUtil::fireEntitySetMotionType` (`00cffda0`, cdecl
/// `(entity)`).
const WORLD_FIRE_SET_MOTION_TYPE: u32 = 0x00d0_06e0;
const ENTITY_FIRE_SET_MOTION_TYPE: u32 = 0x00cf_fda0;
/// The `float` helpers of `createDynamicRigidMotion` (cdecl `(float, float)`, result in
/// ST0): `00c9e0f0` answers the smaller of its two floats and `00c98210` the larger (read from
/// their bodies in Ghidra; their names are unknown).
const FLOAT_MIN: u32 = 0x00c9_e0f0;
const FLOAT_MAX: u32 = 0x00c9_8210;
/// `hkUFloat8::operator=(const float&)` (Xbox PDB, `00ca9360`, `this`, `(pointer to float)`).
const UFLOAT8_ASSIGN: u32 = 0x00ca_9360;
/// The motion constructors `createDynamicRigidMotion` selects (`this` = the buffer,
/// `(position, rotation)`): `00ca9270` sphere, `00d1e690` box, `00d21930` (type 6),
/// `00d1dfa0` keyframed, `00ca9240` character, `00d1e160` fixed.
const SPHERE_MOTION_CONSTRUCT: u32 = 0x00ca_9270;
const BOX_MOTION_CONSTRUCT: u32 = 0x00d1_e690;
const MOTION_TYPE_SIX_CONSTRUCT: u32 = 0x00d2_1930;
const KEYFRAMED_MOTION_CONSTRUCT: u32 = 0x00d1_dfa0;
const CHARACTER_MOTION_CONSTRUCT: u32 = 0x00ca_9240;
const FIXED_MOTION_CONSTRUCT: u32 = 0x00d1_e160;
/// `0.8` as a `double` (`010217d8`) and the keyframed motion's `1.0e6f` (`0108cb00`).
const INERTIA_RATIO: u32 = 0x0102_17d8;
const KEYFRAMED_DAMPING: u32 = 0x0108_cb00;
/// Vtables the two constructors below write.
const CHARACTER_MOTION_VTABLE: u32 = 0x010c_6ea4;
const SPHERE_MOTION_VTABLE: u32 = 0x010c_6d54;

// Translated from 00ca9240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `hkpCharacterMotion` (RTTI of the vtable `010c6ea4` confirmed): the motion
/// base constructor with a trailing 0, the vtable, and motion type 7 at +8.
pub fn fn_00ca9240(e: &mut Engine, this: Ptr, position: Ptr, rotation: Ptr) -> Ptr {
    e.call(MOTION_CONSTRUCT, &args![this, position, rotation, 0u32]);
    e.mem.set_u32(this.addr(), CHARACTER_MOTION_VTABLE);
    e.mem.set_u8(this.addr() + 8, 7);
    this
}

// Translated from 00ca9270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `hkpSphereMotion` (RTTI of the vtable `010c6d54` confirmed): the motion
/// base constructor with a trailing 0, the vtable, and motion type 2 at +8.
pub fn fn_00ca9270(e: &mut Engine, this: Ptr, position: Ptr, rotation: Ptr) -> Ptr {
    e.call(MOTION_CONSTRUCT, &args![this, position, rotation, 0u32]);
    e.mem.set_u32(this.addr(), SPHERE_MOTION_VTABLE);
    e.mem.set_u8(this.addr() + 8, 2);
    this
}

// Translated from 00ca8860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRigidBody::setPositionAndRotation` (Xbox PDB name; the body matches): calls the
/// motion embedded at +0xE0 (vtable slot 0x38 with the two arguments), then `00ca8700` on
/// the body.
pub fn hkp_rigid_body_set_position_and_rotation(
    e: &mut Engine,
    this: Ptr,
    position: Ptr,
    rotation: Ptr,
) {
    e.vcall(this.addr() + 0xe0, 0x38, &args![position, rotation]);
    fn_00ca8700(e, this);
}

// Translated from 00ca8420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRigidBody::setMotionType` (Xbox PDB): when the body's world (+8) is locked (its +0x94
/// is non-zero) the change is queued as an operation (type 4 at +0, the body at +4 and the
/// three arguments as bytes at +8, +9 and +0xA); otherwise it is applied now: the world
/// operation `00cb6f70`, then the world's and the entity's callbacks.
pub fn hkp_rigid_body_set_motion_type(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
) {
    let world = e.mem.u32(this.addr() + 8);
    if world != 0 && e.mem.u32(world + 0x94) != 0 {
        e.with_stack(0xc, |e, operation| {
            let op = operation.addr();
            e.mem.set_u8(op, 4);
            e.mem.set_u32(op + 4, this.addr());
            e.mem.set_u8(op + 8, first as u8);
            e.mem.set_u8(op + 9, second as u8);
            e.mem.set_u8(op + 0xa, third as u8);
            e.call(WORLD_QUEUE_OPERATION, &args![world, operation]);
        });
        return;
    }
    e.call(
        SET_RIGID_BODY_MOTION_TYPE,
        &args![this, first, second, third],
    );
    let world = e.mem.u32(this.addr() + 8);
    if world != 0 {
        e.call(WORLD_FIRE_SET_MOTION_TYPE, &args![world, this]);
    }
    e.call(ENTITY_FIRE_SET_MOTION_TYPE, &args![this]);
}

// Translated from 00ca8700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRigidBody::updateBroadphaseAndResetCollisionInformationOfWarpedBody` (Xbox PDB): with
/// no world (+8) nothing happens; with the world locked (+0x94 non-zero) the update is
/// queued as an operation of type 0x16 with the body at +4. Otherwise the lock count becomes
/// 1 while the world's simulation (+8, vtable slot 0x20) gets the body as a one-entity list
/// with 7, the broad phase is updated when the body has a collidable (+0x10), an inactive
/// body is activated (when the world allows it, +0xD6, and the body is not fixed, +0xE8
/// is not 5) and reported moved, the world's listener (+0x5C, slot 0x14) is called with 0,
/// and when the count returns to 0 with no pending flag (+0x9C) the two world flushes run.
pub fn fn_00ca8700(e: &mut Engine, body: Ptr) {
    let world = e.mem.u32(body.addr() + 8);
    if world == 0 {
        return;
    }
    if e.mem.u32(world + 0x94) != 0 {
        e.with_stack(8, |e, operation| {
            e.mem.set_u8(operation.addr(), 0x16);
            e.mem.set_u32(operation.addr() + 4, body.addr());
            e.call(WORLD_QUEUE_OPERATION, &args![world, operation]);
        });
        return;
    }
    e.mem.set_u32(world + 0x94, 1);
    let simulation = e.mem.u32(world + 8);
    e.with_stack(4, |e, entities| {
        e.mem.set_u32(entities.addr(), body.addr());
        e.vcall(simulation, 0x20, &args![entities, 1u32, world, 7u32]);
        if e.mem.u32(body.addr() + 0x10) != 0 {
            e.call(COLLIDE_ENTITIES_BROAD_PHASE, &args![entities, 1u32, world]);
        }
    });
    let active = e.with_stack(4, |e, out| {
        let flag = e.call(ENTITY_IS_ACTIVE, &args![body, out]).u32();
        e.mem.u8(flag) != 0
    });
    if !active {
        if e.mem.u8(world + 0xd6) != 0 && e.mem.u8(body.addr() + 0xe8) != 5 {
            e.call(ENTITY_ACTIVATE, &args![body]);
        }
        e.call(FIRE_INACTIVE_ENTITY_MOVED, &args![world, body]);
    }
    let listener = e.mem.u32(world + 0x5c);
    if listener != 0 {
        e.vcall(listener, 0x14, &args![0u32]);
    }
    let lock_count = e.mem.u32(world + 0x94).wrapping_sub(1);
    e.mem.set_u32(world + 0x94, lock_count);
    if lock_count == 0 && e.mem.u8(world + 0x9c) == 0 {
        if e.mem.u32(world + 0x8c) != 0 {
            e.call(WORLD_FLUSH_FIRST, &args![world]);
        }
        if e.mem.u32(world + 0xa4) == 1 && e.mem.u32(world + 0x90) != 0 {
            e.call(WORLD_FLUSH_SECOND, &args![world]);
        }
    }
}

// Translated from 00ca8240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRigidBody::createDynamicRigidMotion` (Xbox PDB): builds the motion for `motion_type`
/// in `buffer` with the constructor of its class (a null buffer gives a null motion): 1 picks
/// sphere (`00ca9270`) or box (`00d1e690`) from the diagonal of the inertia matrix (box
/// unless the smallest of the three entries is above 0.8 of the largest; an unordered
/// comparison also gives box), 2 sphere, 3 box, 4 keyframed (`00d1dfa0`; both damping values
/// become the constant `1.0e6` and the virtual set-up calls are skipped), 6 `00d21930`,
/// 7 character, anything else fixed. For every type but 4 the motion's vtable slots 0x1C
/// (the inertia), 0x2C (`sixth`) and 0xC (`mass`) are called. Then the two damping values
/// are assigned to the `hkUFloat8`s at +0xBC and +0xBD of the motion. The exe pushes nine
/// words and the caller removes them.
#[allow(clippy::too_many_arguments)]
pub fn hkp_rigid_body_create_dynamic_rigid_motion(
    e: &mut Engine,
    motion_type: u32,
    position: Ptr,
    rotation: Ptr,
    mass: f32,
    inertia: Ptr,
    sixth: u32,
    mut first_damping: f32,
    mut second_damping: f32,
    buffer: Ptr,
) {
    let constructor = match motion_type.wrapping_sub(1) {
        0 => {
            let diagonal = |e: &Engine, off: u32| x87_quiet(e.mem.f32(inertia.addr() + off));
            let (a, b, c) = (diagonal(e, 0), diagonal(e, 0x14), diagonal(e, 0x28));
            let inner_min = e.call(FLOAT_MIN, &args![b, c]).f32();
            let smallest = e.call(FLOAT_MIN, &args![a, inner_min]).f32();
            let inner_max = e.call(FLOAT_MAX, &args![b, c]).f32();
            let largest = e.call(FLOAT_MAX, &args![a, inner_max]).f64();
            let ratio: f64 = e.global(INERTIA_RATIO);
            // `FCOMPP` then `TEST AH,0x41`: box unless `smallest > largest * 0.8`.
            if f64::from(smallest) > largest * ratio {
                SPHERE_MOTION_CONSTRUCT
            } else {
                BOX_MOTION_CONSTRUCT
            }
        }
        1 => SPHERE_MOTION_CONSTRUCT,
        2 => BOX_MOTION_CONSTRUCT,
        3 => {
            let damping = x87_quiet(e.global::<f32>(KEYFRAMED_DAMPING));
            first_damping = damping;
            second_damping = damping;
            KEYFRAMED_MOTION_CONSTRUCT
        }
        5 => MOTION_TYPE_SIX_CONSTRUCT,
        6 => CHARACTER_MOTION_CONSTRUCT,
        _ => FIXED_MOTION_CONSTRUCT,
    };
    let motion = if buffer.addr() != 0 {
        e.call(constructor, &args![buffer, position, rotation])
            .u32()
    } else {
        0
    };
    if motion_type != 4 {
        e.vcall(motion, 0x1c, &args![inertia]);
        e.vcall(motion, 0x2c, &args![sixth]);
        e.vcall(motion, 0xc, &args![x87_quiet(mass)]);
    }
    e.with_stack(8, |e, floats| {
        e.mem.set_f32(floats.addr(), first_damping);
        e.mem.set_f32(floats.addr() + 4, second_damping);
        e.call(UFLOAT8_ASSIGN, &args![motion + 0xbc, floats]);
        e.call(UFLOAT8_ASSIGN, &args![motion + 0xbd, floats.byte_add(4)]);
    });
}

/// The vtable `hkpRigidBody` writes at +0 (`010c7888`, the one RTTI names `hkpRigidBody`'s
/// scalar deleting destructor in).
const RIGID_BODY_VTABLE: u32 = 0x010c_7888;
/// The base destructor `00ca80a0` jumps to (`00c9cf60`, `this`).
const ENTITY_BASE_DESTRUCT: u32 = 0x00c9_cf60;
/// The entity constructor (`00c9cce0`, `this`, `(shape)`): it stores the shape word at +0x10.
const ENTITY_CONSTRUCT: u32 = 0x00c9_cce0;
/// `hkReferencedObject::addReference` and `removeReference` (`00c90510`, `00c905b0`, `this`).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// The world's broad-phase calls around a shape change (cdecl `(world, entity)`): `00cb5c10`
/// removes the entity, `00cb59f0` adds it back.
const REMOVE_ENTITY_BROAD_PHASE: u32 = 0x00cb_5c10;
const ADD_ENTITY_BROAD_PHASE: u32 = 0x00cb_59f0;
/// `00c9c040` (`this` = the body, `(world, shape)`) and the two callbacks `00d00640` (cdecl
/// `(world, entity)`) and `00cffcf0` (cdecl `(entity)`) the shape change fires.
const ENTITY_SET_CACHED_SHAPE_DATA: u32 = 0x00c9_c040;
const WORLD_FIRE_SHAPE_SET: u32 = 0x00d0_0640;
const ENTITY_FIRE_SHAPE_SET: u32 = 0x00cf_fcf0;
/// The motion calls the body wraps: `00c90ed0` (`this` = the motion, three bytes of the
/// world), `00c86420` (`this` = the motion, a class number) and `00c86430` (the mass, in ST0).
const MOTION_SET_DEACTIVATION_FLAGS: u32 = 0x00c9_0ed0;
const MOTION_SET_DEACTIVATION_CLASS: u32 = 0x00c8_6420;
const MOTION_GET_MASS: u32 = 0x00c8_6430;
/// `bhkRigidBody`-side velocity setters the constructor calls (`this` = the body, `(vector)`).
const SET_LINEAR_VELOCITY: u32 = 0x0056_16d0;
const SET_ANGULAR_VELOCITY: u32 = 0x0056_1800;
/// The cdecl helper `00d21960` (`(shape)`) answering a byte limit for the shape-key level,
/// compared by the constructor.
const SHAPE_KEY_LEVEL_LIMIT: u32 = 0x00d2_1960;
/// The text stream the constructor's assertion writes through: `00cb1600` (`this` = the
/// stream, `(buffer, size)`), `00cc01c0` (`this`, `(text)`) and its destructor `00cc00e0`.
const STREAM_INIT: u32 = 0x00cb_1600;
const STREAM_WRITE_TEXT: u32 = 0x00cc_01c0;
const STREAM_DESTRUCT: u32 = 0x00cc_00e0;
/// The error reporter object the constructor reports through (a pointer at `01268360`; its
/// slot 0xC takes `(1, 0x458a1bb2, text, file, line)`).
const ERROR_REPORTER: u32 = 0x0126_8360;
/// The assertion text (`Shape key storage insufficient to store all possible ...`) and the
/// file name (`.\Entity\hkpRigidBody.cpp`) with the line number 0x149 it reports.
const ASSERT_MESSAGE: u32 = 0x010c_78d0;
const ASSERT_FILE: u32 = 0x010c_78b0;
/// The cinfo constructor `00cf5ea0` (`this` = the cinfo).
const CINFO_CONSTRUCT: u32 = 0x00cf_5ea0;
/// `hkStringPtr::operator=` (`00cb0c10`, `this`, `(string)`).
const STRING_ASSIGN: u32 = 0x00cb_0c10;
/// The array growth routine of `hkpWorldObject::copyProperties` (`00c908c0`, cdecl
/// `(allocator, array, capacity, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// The CRT square root (`00ec6040`): the operand in ST0, the result in ST0.
const X87_SQRT: u32 = 0x00ec_6040;
/// The constant address `getAabb` (shape slot 0x1C) takes as its first argument.
const SHAPE_TRANSFORM: u32 = 0x0126_8370;
/// Floats and doubles of the exe's data read below: `0.5f`, `0.2` (a double), `0.1f`, the
/// double `3.4028...e38` the cached value is compared with, `-1.0f`, the float `3.4028...e38`
/// and the table of `hkUFloat8` damping values.
const HALF: u32 = 0x0101_6248;
const EXTENT_SCALE: u32 = 0x010c_78a8;
const EXTENT_DEFAULT: u32 = 0x0101_e2bc;
const UNSET_VALUE_DOUBLE: u32 = 0x0104_e138;
const MINUS_ONE: u32 = 0x0101_2054;
const UNSET_VALUE: u32 = 0x0104_da48;
const DAMPING_TABLE: u32 = 0x010c_7948;
/// The vtable of the block `clone` makes for the extra data at +0x1F8.
const EXTRA_DATA_VTABLE: u32 = 0x010c_6c64;

/// Whether `value <= 0.0` with both sides ordered (`FLDZ`, `FCOMP`, `TEST AH,1`, `JNZ` skips
/// when `0.0 < value` or the compare is unordered).
fn is_non_positive(value: f32) -> bool {
    matches!(
        value.partial_cmp(&0.0),
        Some(Ordering::Less | Ordering::Equal)
    )
}

/// Allocates `size` bytes through the memory router's allocator (slot 4), as every block
/// the body functions make is.
fn allocate_block(e: &mut Engine, size: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.vcall(allocator, 4, &args![size]).u32()
}

/// Copies `len` bytes (the exe's `MOVAPS` or dword copies).
fn copy_bytes(e: &mut Engine, from: u32, to: u32, len: u32) {
    let data = e.mem.bytes(from, len);
    e.mem.write(to, &data);
}

/// An `FLD dword` / `FSTP dword` copy: a signalling NaN comes out quiet.
fn quiet_copy(e: &mut Engine, from: u32, to: u32) {
    let bits = x87_quiet_bits(e.mem.u32(from));
    e.mem.set_u32(to, bits);
}

/// The world of `body` (+8) when it is non-null and its lock count (+0x94) is non-zero, the
/// test every queued setter starts with.
fn locked_world(e: &Engine, body: Ptr) -> Option<u32> {
    let world = e.mem.u32(body.addr() + 8);
    if world != 0 && e.mem.u32(world + 0x94) != 0 {
        Some(world)
    } else {
        None
    }
}

// Translated from 00ca80a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body: stores the rigid body vtable and continues into the base destructor
/// (`00c9cf60`, a jump, so its result is not known).
pub fn fn_00ca80a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), RIGID_BODY_VTABLE);
    e.call(ENTITY_BASE_DESTRUCT, &args![this]);
}

// Translated from 00ca80b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRigidBody::getMotionState` (Xbox PDB): the address of the motion state at +0xF0.
pub fn hkp_rigid_body_get_motion_state(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0xf0))
}

// Translated from 00ca80c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the byte at +0xE9 (set by `00ca8630`) is anything but 0xFF.
pub fn fn_00ca80c0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0xe9) != 0xff
}

// Translated from 00ca80d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Register-convention helper (`ECX` = `vector`, `EDX` = `target`, plain `RET`): finds the
/// smallest of the vector's three floats (each comparison keeps the second operand unless the
/// second is greater, so an unordered compare keeps the second) and, when it is below 0.5,
/// stores `smallest * 0.2` (a double multiply rounded to float) at `target + 0x4C`; otherwise
/// the float constant at `0101e2bc`.
pub fn fn_00ca80d0(e: &mut Engine, vector: Ptr, target: Ptr) {
    let v = vector.addr();
    let first = x87_quiet(e.mem.f32(v));
    let second = x87_quiet(e.mem.f32(v + 4));
    let third = x87_quiet(e.mem.f32(v + 8));
    let low = if second > first { first } else { second };
    let smallest = if third > low { low } else { third };
    let limit: f32 = e.global(HALF);
    if smallest < limit {
        let scale: f64 = e.global(EXTENT_SCALE);
        e.mem
            .set_f32(target.addr() + 0x4c, (f64::from(smallest) * scale) as f32);
    } else {
        quiet_copy(e, EXTENT_DEFAULT, target.addr() + 0x4c);
    }
}

// Translated from 00ca8150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRigidBody::updateCachedShapeInfo` (Xbox PDB): asks the shape (slot 0x1C, with the
/// constant transform address, a tolerance of 0.0 and a 32-byte local for the minimum and
/// maximum) for its box; `out` gets `maximum - minimum` (four float lanes). The float at
/// +0x190 becomes the square root of the larger of the two sums of squared distances from the
/// centre at +0x170 (summed as `z + (y + x)`, one for the maximum and one for the
/// minimum; the minimum's is taken unless it is strictly smaller).
pub fn hkp_rigid_body_update_cached_shape_info(e: &mut Engine, this: Ptr, shape: Ptr, out: Ptr) {
    let t = this.addr();
    let distance = e.with_stack(0x20, |e, bounds| {
        let b = bounds.addr();
        e.vcall(shape.addr(), 0x1c, &args![SHAPE_TRANSFORM, 0.0f32, bounds]);
        let min = [0u32, 1, 2, 3].map(|i| e.mem.f32(b + 4 * i));
        let max = [0u32, 1, 2, 3].map(|i| e.mem.f32(b + 0x10 + 4 * i));
        for (i, (high, low)) in (0u32..).zip(max.iter().zip(min.iter())) {
            e.mem.set_f32(out.addr() + 4 * i, high - low);
        }
        let centre = [0u32, 1, 2, 3].map(|i| e.mem.f32(t + 0x170 + 4 * i));
        let squares = |edge: &[f32; 4]| {
            let lane = |i: usize| (edge[i] - centre[i]) * (edge[i] - centre[i]);
            lane(2) + (lane(1) + lane(0))
        };
        let from_max = squares(&max);
        let from_min = squares(&min);
        if from_min < from_max {
            from_max
        } else {
            from_min
        }
    });
    let root = e.call(X87_SQRT, &args![f64::from(distance)]).f64();
    e.mem.set_f32(t + 0x190, x87_quiet(root as f32));
}

// Translated from 00ca84b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRigidBody::setShape` (Xbox PDB): with the body's world (+8) locked (+0x94 non-zero)
/// the change is queued (record type 5, the body at +4, the shape at +8) and `false` is
/// returned. Otherwise the world's lock count is raised and the entity leaves the broad phase
/// (`00cb5c10`), the shape at +0x10 is replaced (added reference on the new one, removed on
/// the old), the cached info is updated, and, when there was an old shape and the float at
/// +0x5C is not the double `3.4028...e38`, +0x5C becomes -1.0. When +0x5C is not above 0.0
/// (or is ordered and equal) `00ca80d0` recomputes it from the box extents. The world gets
/// `00c9c040` and `00d00640`, the entity `00cffcf0`, the entity returns to the broad phase
/// (`00cb59f0`), the lock count drops and, at 0 with no pending flag (+0x9C), the two world
/// flushes run. Returns `true`.
pub fn hkp_rigid_body_set_shape(e: &mut Engine, this: Ptr, shape: Ptr) -> bool {
    let t = this.addr();
    let world = e.mem.u32(t + 8);
    if world != 0 {
        if e.mem.u32(world + 0x94) != 0 {
            e.with_stack(0xc, |e, record| {
                e.mem.set_u8(record.addr(), 5);
                e.mem.set_u32(record.addr() + 4, t);
                e.mem.set_u32(record.addr() + 8, shape.addr());
                e.call(WORLD_QUEUE_OPERATION, &args![world, record]);
            });
            return false;
        }
        // The exe tests the world for null a second time here; it cannot be null.
        let count = e.mem.u32(world + 0x94).wrapping_add(1);
        e.mem.set_u32(world + 0x94, count);
        let world = e.mem.u32(t + 8);
        e.call(REMOVE_ENTITY_BROAD_PHASE, &args![world, this]);
    }
    let old = e.mem.u32(t + 0x10);
    e.mem.set_u32(t + 0x10, shape.addr());
    e.call(ADD_REFERENCE, &args![shape]);
    if old != 0 {
        e.call(REMOVE_REFERENCE, &args![old]);
    }
    e.with_stack(0x10, |e, extent| {
        hkp_rigid_body_update_cached_shape_info(e, this, shape, extent);
        if old != 0 {
            let value = x87_quiet(e.mem.f32(t + 0x5c));
            let unset: f64 = e.global(UNSET_VALUE_DOUBLE);
            if f64::from(value) != unset {
                quiet_copy(e, MINUS_ONE, t + 0x5c);
            }
        }
        if is_non_positive(e.mem.f32(t + 0x5c)) {
            fn_00ca80d0(e, extent, Ptr::new(t + 0x10));
        }
    });
    let world = e.mem.u32(t + 8);
    if world != 0 {
        e.call(ENTITY_SET_CACHED_SHAPE_DATA, &args![this, world, shape]);
    }
    let world = e.mem.u32(t + 8);
    if world != 0 {
        e.call(WORLD_FIRE_SHAPE_SET, &args![world, this]);
    }
    e.call(ENTITY_FIRE_SHAPE_SET, &args![this]);
    let world = e.mem.u32(t + 8);
    if world != 0 {
        e.call(ADD_ENTITY_BROAD_PHASE, &args![world, this]);
        let world = e.mem.u32(t + 8);
        let count = e.mem.u32(world + 0x94).wrapping_sub(1);
        e.mem.set_u32(world + 0x94, count);
        if count == 0 && e.mem.u8(world + 0x9c) == 0 {
            if e.mem.u32(world + 0x8c) != 0 {
                e.call(WORLD_FLUSH_FIRST, &args![world]);
            }
            if e.mem.u32(world + 0xa4) == 1 && e.mem.u32(world + 0x90) != 0 {
                e.call(WORLD_FLUSH_SECOND, &args![world]);
            }
        }
    }
    true
}

// Translated from 00ca85f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRigidBody::setCenterOfMassLocal` (Xbox PDB): passes the vector to the embedded motion
/// (+0xE0, slot 0x2C), then refreshes the cached info of the shape at +0x10.
pub fn hkp_rigid_body_set_center_of_mass_local(e: &mut Engine, this: Ptr, centre: Ptr) {
    e.vcall(this.addr() + 0xe0, 0x2c, &args![centre]);
    let shape = e.mem.u32(this.addr() + 0x10);
    e.with_stack(0x10, |e, extent| {
        hkp_rigid_body_update_cached_shape_info(e, this, Ptr::new(shape), extent);
    });
}

// Translated from 00ca8630 (decompiled, FalloutNV.exe 1.4.0.525)
/// With `flag` 0 the byte at +0xE9 becomes 0xFF and the words at +0xEA and +0xEC 0. Otherwise
/// the low nibble of the byte at +0xD4 goes to +0xE9 and `00c90ed0` runs on the embedded
/// motion (+0xE0) with the bytes at +0x305, +0x306 and +0x307 of the world (+8), or with
/// 0, 0, 0 when the body has no world.
pub fn fn_00ca8630(e: &mut Engine, this: Ptr, flag: u8) {
    let t = this.addr();
    if flag == 0 {
        e.mem.set_u8(t + 0xe9, 0xff);
        e.mem.set_u16(t + 0xea, 0);
        e.mem.set_u16(t + 0xec, 0);
        return;
    }
    let world = e.mem.u32(t + 8);
    let nibble = e.mem.u8(t + 0xd4) & 0xf;
    let (first, second, third) = if world != 0 {
        (
            u32::from(e.mem.u8(world + 0x305)),
            u32::from(e.mem.u8(world + 0x306)),
            u32::from(e.mem.u8(world + 0x307)),
        )
    } else {
        (0, 0, 0)
    };
    e.mem.set_u8(t + 0xe9, nibble);
    e.call(
        MOTION_SET_DEACTIVATION_FLAGS,
        &args![t + 0xe0, first, second, third],
    );
}

// Translated from 00ca86c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x1F8 when the motion type (+0xE8) is 4 or 5, else 0.
pub fn fn_00ca86c0(e: &mut Engine, this: Ptr) -> u32 {
    let motion = e.mem.u8(this.addr() + 0xe8);
    if motion == 4 || motion == 5 {
        e.mem.u32(this.addr() + 0x1f8)
    } else {
        0
    }
}

// Translated from 00ca8800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRigidBody::setPosition` (Xbox PDB): the embedded motion's slot 0x30, then `00ca8700`.
pub fn hkp_rigid_body_set_position(e: &mut Engine, this: Ptr, position: Ptr) {
    e.vcall(this.addr() + 0xe0, 0x30, &args![position]);
    fn_00ca8700(e, this);
}

// Translated from 00ca8830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRigidBody::setRotation` (Xbox PDB): the embedded motion's slot 0x34, then `00ca8700`.
pub fn hkp_rigid_body_set_rotation(e: &mut Engine, this: Ptr, rotation: Ptr) {
    e.vcall(this.addr() + 0xe0, 0x34, &args![rotation]);
    fn_00ca8700(e, this);
}

// Translated from 00ca8890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRigidBody::setTransform` (Xbox PDB): the embedded motion's slot 0x3C, then `00ca8700`.
pub fn hkp_rigid_body_set_transform(e: &mut Engine, this: Ptr, transform: Ptr) {
    e.vcall(this.addr() + 0xe0, 0x3c, &args![transform]);
    fn_00ca8700(e, this);
}

// Translated from 00ca88c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Passes the float (as a quieted bit pattern) to the embedded motion's slot 0xC.
pub fn fn_00ca88c0(e: &mut Engine, this: Ptr, value: f32) {
    e.vcall(this.addr() + 0xe0, 0xc, &args![x87_quiet(value)]);
}

// Translated from 00ca88e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Passes the float (as a quieted bit pattern) to the embedded motion's slot 0x10.
pub fn fn_00ca88e0(e: &mut Engine, this: Ptr, value: f32) {
    e.vcall(this.addr() + 0xe0, 0x10, &args![x87_quiet(value)]);
}

// Translated from 00ca8920 (decompiled, FalloutNV.exe 1.4.0.525)
/// A jump into the embedded motion's slot 0x1C (`this` = +0xE0) that keeps the caller's one
/// stack word, `out`; the result of that slot is not known, so none is returned here.
pub fn fn_00ca8920(e: &mut Engine, this: Ptr, out: Ptr) {
    e.vcall(this.addr() + 0xe0, 0x1c, &args![out]);
}

// Translated from 00ca8960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves the body: with the world locked, queues operation 0x18 (the body at +4, a 0x20-byte
/// block at +8 allocated through the memory router, holding the two 16-byte arguments, the
/// block has no size word); otherwise `hkpRigidBody::setPositionAndRotation` (`00ca8860`).
pub fn fn_00ca8960(e: &mut Engine, this: Ptr, position: Ptr, rotation: Ptr) {
    if let Some(world) = locked_world(e, this) {
        e.with_stack(0xc, |e, record| {
            e.mem.set_u8(record.addr(), 0x18);
            e.mem.set_u32(record.addr() + 4, this.addr());
            let block = allocate_block(e, 0x20);
            e.mem.set_u32(record.addr() + 8, block);
            copy_bytes(e, position.addr(), block, 0x10);
            copy_bytes(e, rotation.addr(), block + 0x10, 0x10);
            e.call(WORLD_QUEUE_OPERATION, &args![world, record]);
        });
    } else {
        hkp_rigid_body_set_position_and_rotation(e, this, position, rotation);
    }
}

/// Shared body of `00ca8a80` and `00ca8b80`: with the world locked, queues `operation` (the
/// body at +4, the three floats of `vector` copied at +8, +0xC and +0x10); otherwise
/// activates the body and jumps into the embedded motion's `slot` with `vector`.
fn queue_or_forward_vector(e: &mut Engine, this: Ptr, vector: Ptr, operation: u8, slot: u32) {
    if let Some(world) = locked_world(e, this) {
        e.with_stack(0x14, |e, record| {
            let r = record.addr();
            e.mem.set_u8(r, operation);
            e.mem.set_u32(r + 4, this.addr());
            quiet_copy(e, vector.addr(), r + 8);
            quiet_copy(e, vector.addr() + 4, r + 0xc);
            quiet_copy(e, vector.addr() + 8, r + 0x10);
            e.call(WORLD_QUEUE_OPERATION, &args![world, record]);
        });
    } else {
        e.call(ENTITY_ACTIVATE, &args![this]);
        e.vcall(this.addr() + 0xe0, slot, &args![vector]);
    }
}

// Translated from 00ca8a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues operation 0x1B with the vector when the world is locked, else activates the body
/// and calls the embedded motion's slot 0x4C (a jump, so no result is returned).
pub fn fn_00ca8a80(e: &mut Engine, this: Ptr, vector: Ptr) {
    queue_or_forward_vector(e, this, vector, 0x1b, 0x4c);
}

// Translated from 00ca8b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues operation 0x1D with the vector when the world is locked, else activates the body
/// and calls the embedded motion's slot 0x54 (a jump, so no result is returned).
pub fn fn_00ca8b80(e: &mut Engine, this: Ptr, vector: Ptr) {
    queue_or_forward_vector(e, this, vector, 0x1d, 0x54);
}

// Translated from 00ca8af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// With the world locked, queues operation 0x1C (the body at +4, a 0x20-byte block at +8 with
/// the second argument at +0 and the first at +0x10, no size word); otherwise activates the
/// body and calls the embedded motion's slot 0x50 with both arguments.
pub fn fn_00ca8af0(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) {
    if let Some(world) = locked_world(e, this) {
        e.with_stack(0xc, |e, record| {
            e.mem.set_u8(record.addr(), 0x1c);
            e.mem.set_u32(record.addr() + 4, this.addr());
            let block = allocate_block(e, 0x20);
            e.mem.set_u32(record.addr() + 8, block);
            copy_bytes(e, second.addr(), block, 0x10);
            copy_bytes(e, first.addr(), block + 0x10, 0x10);
            e.call(WORLD_QUEUE_OPERATION, &args![world, record]);
        });
    } else {
        e.call(ENTITY_ACTIVATE, &args![this]);
        e.vcall(this.addr() + 0xe0, 0x50, &args![first, second]);
    }
}

// Translated from 00ca8bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes a byte at `out`: 0 without a shape (+0x10), else 1 unless a size check fails. For
/// motion types other than 4 and 5 and shape kinds (+0xC of the shape) 0xA or 8 the count
/// from slot 4 of the object slot 0x10 of the shape returns must not exceed 100. Shape kind
/// 0x17 or 7 asks the same of the object embedded at shape + 0x10, kind 9 likewise (using the
/// shape read first), and kind 0xF always fails.
/// Answers `out`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
pub fn fn_00ca8bf0(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let t = this.addr();
    let shape = e.mem.u32(t + 0x10);
    if shape == 0 {
        e.mem.set_u8(out.addr(), 0);
        return out;
    }
    let motion = e.mem.u8(t + 0xe8);
    let kind = e.mem.u32(shape + 0xc);
    let mut fits = true;
    if motion != 5 && motion != 4 && (kind == 0xa || kind == 8) {
        let inner = e.vcall(shape, 0x10, &args![]).u32();
        if e.vcall(inner, 4, &args![]).i32() > 0x64 {
            fits = false;
        }
    }
    // Kinds 0x17 and 7 have separate but identical blocks in the exe.
    let embedded = match kind {
        0x17 | 7 => Some(e.mem.u32(t + 0x10) + 0x10),
        0xf => {
            fits = false;
            None
        }
        9 => Some(shape + 0x10),
        _ => None,
    };
    if let Some(object) = embedded {
        if e.vcall(object, 4, &args![]).i32() > 0x64 {
            fits = false;
        }
    }
    e.mem.set_u8(out.addr(), u8::from(fits));
    out
}

// Translated from 00ca8c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRigidBody::hkpRigidBody` (Xbox PDB): builds the body from a construction info.
/// Runs the entity constructor with the word at cinfo+4 (the shape), stores the vtable, the bytes and
/// words (cinfo+0xC to +0x8C, cinfo+0xE to +0xAA, cinfo+0 to +0x2C) and the shape (cinfo+8:
/// reference added, the old one at +0x210 released, stored there with a second reference).
/// Motion type 5 (cinfo+0xB4) builds a fixed motion in the embedded buffer (+0xE0) with the
/// two damping values at the cinfo's +0xA8 and +0xAC, selects class 1, and +0x5C becomes
/// the float at cinfo+0xB0 when that is above 0.0 or unordered, else the float constant
/// at `0104da48`. Other types go through `createDynamicRigidMotion` (`00ca8240`), then
/// the class from cinfo+0xB6, the two velocities and +0x5C = cinfo+0xB0. Then `00ca8630` runs
/// with cinfo+0xB5 != 0, the floats at +0x194 and +0x198 are copied from cinfo+0x94 and
/// +0x98, and with a shape the cached info is refreshed (and `00ca80d0` run when +0x5C is not
/// above 0.0). The byte at +0x2A is cinfo+0xB7 unless that is 0xFF (then 0 for motion type 5,
/// else 1 + (cinfo+0xB4 != 4)), +0xD0 = cinfo+0xB8, +0x21 gets bit 0 set when cinfo+0xBB is
/// non-zero, the floats at +0x90 and +0x94 come from cinfo+0xA0 and +0xA4, the word at +0x1FE
/// from cinfo+0x9E. The byte at +0xD1 is cinfo+0xBA, or, when that is negative, the limit
/// `00d21960` answers for that word; a positive cinfo+0xBA larger than that limit
/// reports an assertion through the error reporter. Last the byte at +0xD2 = cinfo+0xB9.
/// Returns `this`.
pub fn hkp_rigid_body_ctor(e: &mut Engine, this: Ptr, cinfo: Ptr) -> Ptr {
    let t = this.addr();
    let c = cinfo.addr();
    let shape_word = e.mem.u32(c + 4);
    e.call(ENTITY_CONSTRUCT, &args![this, shape_word]);
    e.mem.set_u32(t, RIGID_BODY_VTABLE);
    let flag = e.mem.u8(c + 0xc);
    e.mem.set_u8(t + 0x8c, flag);
    let word = e.mem.u16(c + 0xe);
    e.mem.set_u16(t + 0xaa, word);
    let first = e.mem.u32(c);
    e.mem.set_u32(t + 0x2c, first);
    let held = e.mem.u32(c + 8);
    if held != 0 {
        e.call(ADD_REFERENCE, &args![held]);
        let previous = e.mem.u32(t + 0x210);
        if previous != 0 {
            e.call(REMOVE_REFERENCE, &args![previous]);
        }
        e.mem.set_u32(t + 0x210, held);
        e.call(ADD_REFERENCE, &args![held]);
    }
    let motion = e.mem.u8(c + 0xb4);
    let extent_bits = if motion == 5 {
        let buffer = t.wrapping_add(0xe0);
        let built = if buffer != 0 {
            e.call(FIXED_MOTION_CONSTRUCT, &args![buffer, c + 0x10, c + 0x20])
                .u32()
        } else {
            0
        };
        e.call(UFLOAT8_ASSIGN, &args![built + 0xbc, c + 0xa8]);
        e.call(UFLOAT8_ASSIGN, &args![built + 0xbd, c + 0xac]);
        e.call(MOTION_SET_DEACTIVATION_CLASS, &args![built, 1u32]);
        e.mem.set_u32(t + 0x18, t + 0xf0);
        if is_non_positive(e.mem.f32(c + 0xb0)) {
            x87_quiet_bits(e.mem.u32(UNSET_VALUE))
        } else {
            x87_quiet_bits(e.mem.u32(c + 0xb0))
        }
    } else {
        let mass = x87_quiet(e.mem.f32(c + 0x90));
        let first_damping = x87_quiet(e.mem.f32(c + 0xa8));
        let second_damping = x87_quiet(e.mem.f32(c + 0xac));
        hkp_rigid_body_create_dynamic_rigid_motion(
            e,
            i32::from(motion as i8) as u32,
            Ptr::new(c + 0x10),
            Ptr::new(c + 0x20),
            mass,
            Ptr::new(c + 0x50),
            c + 0x80,
            first_damping,
            second_damping,
            Ptr::new(t.wrapping_add(0xe0)),
        );
        let class = i32::from(e.mem.u8(c + 0xb6) as i8) as u32;
        e.call(MOTION_SET_DEACTIVATION_CLASS, &args![t + 0xe0, class]);
        e.call(SET_LINEAR_VELOCITY, &args![this, c + 0x30]);
        e.call(SET_ANGULAR_VELOCITY, &args![this, c + 0x40]);
        e.mem.set_u32(t + 0x18, t + 0xf0);
        x87_quiet_bits(e.mem.u32(c + 0xb0))
    };
    e.mem.set_u32(t + 0x5c, extent_bits);
    let enabled = u8::from(e.mem.u8(c + 0xb5) != 0);
    fn_00ca8630(e, this, enabled);
    quiet_copy(e, c + 0x94, t + 0x194);
    quiet_copy(e, c + 0x98, t + 0x198);
    let shape = e.mem.u32(t + 0x10);
    if shape != 0 {
        e.with_stack(0x10, |e, extent| {
            hkp_rigid_body_update_cached_shape_info(e, this, Ptr::new(shape), extent);
            if is_non_positive(e.mem.f32(t + 0x5c)) {
                fn_00ca80d0(e, extent, Ptr::new(t + 0x10));
            }
        });
    }
    let quality = e.mem.u8(c + 0xb7);
    let quality = if quality != 0xff {
        quality
    } else if e.mem.u8(t + 0xe8) == 5 {
        0
    } else {
        u8::from(e.mem.u8(c + 0xb4) != 4) + 1
    };
    e.mem.set_u8(t + 0x2a, quality);
    let byte = e.mem.u8(c + 0xb8);
    e.mem.set_u8(t + 0xd0, byte);
    if e.mem.u8(c + 0xbb) != 0 {
        let flags = e.mem.u8(t + 0x21) | 1;
        e.mem.set_u8(t + 0x21, flags);
    }
    quiet_copy(e, c + 0xa0, t + 0x90);
    quiet_copy(e, c + 0xa4, t + 0x94);
    let word = e.mem.u16(c + 0x9e);
    e.mem.set_u16(t + 0x1fe, word);
    let level = e.mem.u8(c + 0xba);
    if (level as i8) < 0 {
        let limit = e.call(SHAPE_KEY_LEVEL_LIMIT, &args![shape_word]).u32() as u8;
        e.mem.set_u8(t + 0xd1, limit);
    } else {
        e.mem.set_u8(t + 0xd1, level);
        if level != 0 {
            let limit = e.call(SHAPE_KEY_LEVEL_LIMIT, &args![shape_word]).u32() as u8;
            if level < limit {
                e.with_stack(0x210, |e, stream| {
                    let text = stream.addr() + 0x10;
                    e.call(STREAM_INIT, &args![stream, text, 0x200u32]);
                    e.call(STREAM_WRITE_TEXT, &args![stream, ASSERT_MESSAGE]);
                    let reporter = e.mem.u32(ERROR_REPORTER);
                    e.vcall(
                        reporter,
                        0xc,
                        &args![1u32, 0x458a_1bb2u32, text, ASSERT_FILE, 0x149u32],
                    );
                    e.call(STREAM_DESTRUCT, &args![stream]);
                });
            }
        }
    }
    let byte = e.mem.u8(c + 0xb9);
    e.mem.set_u8(t + 0xd2, byte);
    this
}

// Translated from 00ca8f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRigidBody::getCinfo` (Xbox PDB): fills the construction info from the body, the
/// reverse of the constructor: bytes, words and floats (float copies quiet signalling NaNs),
/// the 16-byte vectors (+0x1B0, +0x1C0, +0x120, +0x160, +0x170 to cinfo+0x30, +0x40, +0x10,
/// +0x20, +0x80), the mass from `00c86430` at cinfo+0x90, the inertia from the motion's
/// slot 0x14 into cinfo+0x50, the two damping values from the table at `010c7948` indexed by
/// the bytes at +0x19C and +0x19D, the shape and the other words. cinfo+0x9C gets the word at
/// +0x1FE shifted to the high half as a float bit pattern.
pub fn hkp_rigid_body_get_cinfo(e: &mut Engine, this: Ptr, cinfo: Ptr) {
    let t = this.addr();
    let c = cinfo.addr();
    let bit = e.mem.u8(t + 0x21) & 1;
    e.mem.set_u8(c + 0xbb, bit);
    let byte = e.mem.u8(t + 0xd0);
    e.mem.set_u8(c + 0xb8, byte);
    let byte = e.mem.u8(t + 0xd1);
    e.mem.set_u8(c + 0xba, byte);
    let word = e.mem.u16(t + 0xaa);
    e.mem.set_u16(c + 0xe, word);
    let enabled = fn_00ca80c0(e, this);
    e.mem.set_u8(c + 0xb5, u8::from(enabled));
    let held = e.mem.u32(t + 0x210);
    e.mem.set_u32(c + 8, held);
    quiet_copy(e, t + 0x90, c + 0xa0);
    let byte = e.mem.u8(t + 0x8c);
    e.mem.set_u8(c + 0xc, byte);
    quiet_copy(e, t + 0x94, c + 0xa4);
    quiet_copy(e, t + 0x194, c + 0x94);
    quiet_copy(e, t + 0x198, c + 0x98);
    copy_bytes(e, t + 0x1b0, c + 0x30, 0x10);
    copy_bytes(e, t + 0x1c0, c + 0x40, 0x10);
    let mass = e.call(MOTION_GET_MASS, &args![t + 0xe0]).f32();
    e.mem.set_f32(c + 0x90, x87_quiet(mass));
    e.vcall(t + 0xe0, 0x14, &args![c + 0x50]);
    let byte = e.mem.u8(t + 0xe8);
    e.mem.set_u8(c + 0xb4, byte);
    let byte = e.mem.u8(t + 0x19e);
    e.mem.set_u8(c + 0xb6, byte);
    let index = u32::from(e.mem.u8(t + 0x19c));
    quiet_copy(e, DAMPING_TABLE + 4 * index, c + 0xa8);
    let index = u32::from(e.mem.u8(t + 0x19d));
    quiet_copy(e, DAMPING_TABLE + 4 * index, c + 0xac);
    copy_bytes(e, t + 0x120, c + 0x10, 0x10);
    copy_bytes(e, t + 0x160, c + 0x20, 0x10);
    copy_bytes(e, t + 0x170, c + 0x80, 0x10);
    let shape = e.mem.u32(t + 0x10);
    e.mem.set_u32(c + 4, shape);
    let first = e.mem.u32(t + 0x2c);
    e.mem.set_u32(c, first);
    quiet_copy(e, t + 0x5c, c + 0xb0);
    let byte = e.mem.u8(t + 0x2a);
    e.mem.set_u8(c + 0xb7, byte);
    let shifted = (i32::from(e.mem.u16(t + 0x1fe) as i16) << 16) as u32;
    e.mem.set_u32(c + 0x9c, x87_quiet_bits(shifted));
}

// Translated from 00ca90c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRigidBody::clone` (Xbox PDB, fastcall): builds a construction info (`00cf5ea0`, then
/// `getCinfo`), allocates a 0x220-byte block (size word 0x220 at +4) and constructs the new
/// body in it, copies the 0x120 bytes of the embedded motion at +0xE0, and when +0x1F8 holds
/// an extra block makes a 0x120-byte copy of it (the allocation's size word, vtable
/// `010c6c64` and the word at +0x118 are written first and then overwritten by the copy).
/// The new body's +0x18 is the result of its own slot 0x10, the properties are copied
/// (`00ca92a0`), the string at +0x74 is assigned with bit 0 cleared (`00cb0c10`), +0xC is
/// copied, and a non-null +0xA0 is cloned through its slot 0xC. Returns the new body.
pub fn hkp_rigid_body_clone(e: &mut Engine, this: Ptr) -> Ptr {
    let t = this.addr();
    let new = e.with_stack(0xc0, |e, cinfo| {
        e.call(CINFO_CONSTRUCT, &args![cinfo]);
        hkp_rigid_body_get_cinfo(e, this, cinfo);
        let block = allocate_block(e, 0x220);
        e.mem.set_u16(block + 4, 0x220);
        hkp_rigid_body_ctor(e, Ptr::new(block), cinfo).addr()
    });
    copy_bytes(e, t + 0xe0, new + 0xe0, 0x120);
    let extra = e.mem.u32(t + 0x1f8);
    if extra != 0 {
        let copy = allocate_block(e, 0x120);
        e.mem.set_u16(copy + 4, 0x120);
        e.mem.set_u32(copy + 0x118, 0);
        e.mem.set_u32(copy, EXTRA_DATA_VTABLE);
        e.mem.set_u32(new + 0x1f8, copy);
        let source = e.mem.u32(t + 0x1f8);
        copy_bytes(e, source, copy, 0x120);
    }
    let motion_state = e.vcall(new, 0x10, &args![]).u32();
    e.mem.set_u32(new + 0x18, motion_state);
    hkp_world_object_copy_properties(e, Ptr::new(new), this);
    let name = e.mem.u32(t + 0x74) & 0xffff_fffe;
    e.call(STRING_ASSIGN, &args![new + 0x74, name]);
    let word = e.mem.u32(t + 0xc);
    e.mem.set_u32(new + 0xc, word);
    let property = e.mem.u32(t + 0xa0);
    if property != 0 {
        let cloned = e.vcall(property, 0xc, &args![new]).u32();
        e.mem.set_u32(new + 0xa0, cloned);
    }
    Ptr::new(new)
}

// Translated from 00ca92a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldObject::copyProperties` (Xbox PDB): copies the array of 16-byte elements at +0x78
/// (data pointer, size at +4, capacity and flags at +8) from `other` into this one. When the
/// capacity (low 30 bits) is below the other's size the array is grown to the larger of
/// twice the capacity and that size (`00c908c0` through the memory router's allocator). The
/// first `min(size, other size)` elements are copied, the rest are copied only where the
/// destination address is non-null, and the size becomes the other's.
pub fn hkp_world_object_copy_properties(e: &mut Engine, this: Ptr, other: Ptr) {
    let array = this.addr() + 0x78;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let size = e.mem.u32(array + 4) as i32;
    let wanted = e.mem.u32(other.addr() + 0x7c) as i32;
    let allocator = e.mem.u32(router + 0x10);
    let keep = if wanted > size { size } else { wanted };
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < wanted {
        let doubled = capacity.wrapping_add(capacity);
        let grown = if wanted < doubled { doubled } else { wanted };
        e.call(ARRAY_RESERVE, &args![allocator, array, grown, 0x10u32]);
    }
    let mut source = e.mem.u32(other.addr() + 0x78);
    let mut dest = e.mem.u32(array);
    for _ in 0..keep {
        copy_bytes(e, source, dest, 0x10);
        source = source.wrapping_add(0x10);
        dest = dest.wrapping_add(0x10);
    }
    let skipped = (keep as u32) << 4;
    let mut source = e.mem.u32(other.addr() + 0x78).wrapping_add(skipped);
    let mut dest = e.mem.u32(array).wrapping_add(skipped);
    let remaining = wanted.wrapping_sub(keep);
    for _ in 0..remaining {
        if dest != 0 {
            copy_bytes(e, source, dest, 0x10);
        }
        source = source.wrapping_add(0x10);
        dest = dest.wrapping_add(0x10);
    }
    e.mem.set_u32(array + 4, wanted as u32);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ca_9210, deleting_destructor_00ca9210(Ptr, u8) -> Ptr),
        entry!(
            0x00ca_8240,
            hkp_rigid_body_create_dynamic_rigid_motion(u32, Ptr, Ptr, f32, Ptr, u32, f32, f32, Ptr)
        ),
        entry!(
            0x00ca_8420,
            hkp_rigid_body_set_motion_type(Ptr, u32, u32, u32)
        ),
        entry!(0x00ca_8700, fn_00ca8700(Ptr)),
        entry!(
            0x00ca_8860,
            hkp_rigid_body_set_position_and_rotation(Ptr, Ptr, Ptr)
        ),
        entry!(0x00ca_9240, fn_00ca9240(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00ca_9270, fn_00ca9270(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00ca_80a0, fn_00ca80a0(Ptr)),
        entry!(0x00ca_80b0, hkp_rigid_body_get_motion_state(Ptr) -> Ptr),
        entry!(0x00ca_80c0, fn_00ca80c0(Ptr) -> bool),
        entry!(0x00ca_80d0, fn_00ca80d0(Ptr, Ptr)),
        entry!(
            0x00ca_8150,
            hkp_rigid_body_update_cached_shape_info(Ptr, Ptr, Ptr)
        ),
        entry!(0x00ca_84b0, hkp_rigid_body_set_shape(Ptr, Ptr) -> bool),
        entry!(
            0x00ca_85f0,
            hkp_rigid_body_set_center_of_mass_local(Ptr, Ptr)
        ),
        entry!(0x00ca_8630, fn_00ca8630(Ptr, u8)),
        entry!(0x00ca_86c0, fn_00ca86c0(Ptr) -> u32),
        entry!(0x00ca_8800, hkp_rigid_body_set_position(Ptr, Ptr)),
        entry!(0x00ca_8830, hkp_rigid_body_set_rotation(Ptr, Ptr)),
        entry!(0x00ca_8890, hkp_rigid_body_set_transform(Ptr, Ptr)),
        entry!(0x00ca_88c0, fn_00ca88c0(Ptr, f32)),
        entry!(0x00ca_88e0, fn_00ca88e0(Ptr, f32)),
        entry!(0x00ca_8920, fn_00ca8920(Ptr, Ptr)),
        entry!(0x00ca_8960, fn_00ca8960(Ptr, Ptr, Ptr)),
        entry!(0x00ca_8a80, fn_00ca8a80(Ptr, Ptr)),
        entry!(0x00ca_8af0, fn_00ca8af0(Ptr, Ptr, Ptr)),
        entry!(0x00ca_8b80, fn_00ca8b80(Ptr, Ptr)),
        entry!(0x00ca_8bf0, fn_00ca8bf0(Ptr, Ptr) -> Ptr),
        entry!(0x00ca_8c90, hkp_rigid_body_ctor(Ptr, Ptr) -> Ptr),
        entry!(0x00ca_8f50, hkp_rigid_body_get_cinfo(Ptr, Ptr)),
        entry!(0x00ca_90c0, hkp_rigid_body_clone(Ptr) -> Ptr),
        entry!(0x00ca_92a0, hkp_world_object_copy_properties(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00ca9210_frees_only_when_bit_zero_is_set() {
        check(0x00ca_9210, 0x00ca_80a0);
    }
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Registers a double at each address that logs `(address, arguments)` and answers
    /// `eax` / `st0` from `answers` (default 0).
    fn log_calls(e: &mut Engine, addresses: &[u32]) -> Log {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        for &address in addresses {
            let log = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        log
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    #[test]
    fn character_and_sphere_constructors_write_vtable_and_type() {
        let mut e = test_engine();
        let log = log_calls(&mut e, &[MOTION_CONSTRUCT]);
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00ca_9240, &args![this, 5u32, 6u32]).u32(), this);
        assert_eq!(e.mem.u32(this), CHARACTER_MOTION_VTABLE);
        assert_eq!(e.mem.u8(this + 8), 7);
        assert_eq!(e.call(0x00ca_9270, &args![this, 7u32, 8u32]).u32(), this);
        assert_eq!(e.mem.u32(this), SPHERE_MOTION_VTABLE);
        assert_eq!(e.mem.u8(this + 8), 2);
        assert_eq!(
            calls(&log, MOTION_CONSTRUCT),
            vec![vec![this, 5, 6, 0], vec![this, 7, 8, 0]]
        );
    }

    #[test]
    fn set_position_and_rotation_calls_the_embedded_motion_then_updates() {
        let mut e = test_engine();
        let log = log_calls(&mut e, &[0x0900_0001]);
        let body = e.mem.alloc(0x100);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x38, 0x0900_0001);
        e.mem.set_u32(body + 0xe0, vtable);
        e.call(0x00ca_8860, &args![body, 11u32, 12u32]);
        assert_eq!(calls(&log, 0x0900_0001), vec![vec![body + 0xe0, 11, 12]]);
    }

    #[test]
    fn set_motion_type_queues_when_the_world_is_locked() {
        let mut e = test_engine();
        let queued = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&queued);
        e.register_double(WORLD_QUEUE_OPERATION, move |e, a| {
            sink.borrow_mut().push((
                e.mem.u8(a[1]),
                e.mem.u32(a[1] + 4),
                e.mem.bytes(a[1] + 8, 3),
            ));
            Ret::default()
        });
        let world = e.mem.alloc(0x100);
        e.mem.set_u32(world + 0x94, 1);
        let body = e.mem.alloc(0x100);
        e.mem.set_u32(body + 8, world);
        e.call(0x00ca_8420, &args![body, 0x101u32, 2u32, 3u32]);
        assert_eq!(*queued.borrow(), vec![(4, body, vec![1, 2, 3])]);
    }

    #[test]
    fn set_motion_type_applies_when_the_world_is_not_locked() {
        let mut e = test_engine();
        let log = log_calls(
            &mut e,
            &[
                SET_RIGID_BODY_MOTION_TYPE,
                WORLD_FIRE_SET_MOTION_TYPE,
                ENTITY_FIRE_SET_MOTION_TYPE,
                WORLD_QUEUE_OPERATION,
            ],
        );
        let world = e.mem.alloc(0x100);
        let body = e.mem.alloc(0x100);
        e.mem.set_u32(body + 8, world);
        e.call(0x00ca_8420, &args![body, 1u32, 2u32, 3u32]);
        e.mem.set_u32(body + 8, 0);
        e.call(0x00ca_8420, &args![body, 4u32, 5u32, 6u32]);
        let order: Vec<u32> = log.borrow().iter().map(|(a, _)| *a).collect();
        assert_eq!(
            order,
            vec![
                SET_RIGID_BODY_MOTION_TYPE,
                WORLD_FIRE_SET_MOTION_TYPE,
                ENTITY_FIRE_SET_MOTION_TYPE,
                SET_RIGID_BODY_MOTION_TYPE,
                ENTITY_FIRE_SET_MOTION_TYPE,
            ]
        );
        assert_eq!(
            calls(&log, SET_RIGID_BODY_MOTION_TYPE),
            vec![vec![body, 1, 2, 3], vec![body, 4, 5, 6]]
        );
        assert_eq!(
            calls(&log, WORLD_FIRE_SET_MOTION_TYPE),
            vec![vec![world, body]]
        );
    }

    /// A world with a simulation object (slot 0x20 double at `0x0900_0020`), a listener
    /// (slot 0x14 double at `0x0900_0014`) and a body.
    fn world_rig(e: &mut Engine) -> (u32, u32, Log) {
        let log = log_calls(
            e,
            &[
                0x0900_0020,
                0x0900_0014,
                COLLIDE_ENTITIES_BROAD_PHASE,
                ENTITY_ACTIVATE,
                FIRE_INACTIVE_ENTITY_MOVED,
                WORLD_FLUSH_FIRST,
                WORLD_FLUSH_SECOND,
                WORLD_QUEUE_OPERATION,
            ],
        );
        let simulation_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(simulation_vtable + 0x20, 0x0900_0020);
        let simulation = e.mem.alloc(0x10);
        e.mem.set_u32(simulation, simulation_vtable);
        let listener_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(listener_vtable + 0x14, 0x0900_0014);
        let listener = e.mem.alloc(0x10);
        e.mem.set_u32(listener, listener_vtable);
        let world = e.mem.alloc(0x100);
        e.mem.set_u32(world + 8, simulation);
        e.mem.set_u32(world + 0x5c, listener);
        let body = e.mem.alloc(0x100);
        e.mem.set_u32(body + 8, world);
        (world, body, log)
    }

    #[test]
    fn warped_body_update_without_a_world_or_with_a_locked_one() {
        let mut e = test_engine();
        let (world, body, log) = world_rig(&mut e);
        e.mem.set_u32(body + 8, 0);
        e.call(0x00ca_8700, &args![body]);
        assert!(log.borrow().is_empty());
        e.mem.set_u32(body + 8, world);
        e.mem.set_u32(world + 0x94, 2);
        e.call(0x00ca_8700, &args![body]);
        assert_eq!(log.borrow().len(), 1);
        assert_eq!(calls(&log, WORLD_QUEUE_OPERATION)[0][0], world);
        assert_eq!(e.mem.u32(world + 0x94), 2);
    }

    #[test]
    fn warped_body_update_of_an_inactive_body_activates_and_flushes() {
        let mut e = test_engine();
        let (world, body, log) = world_rig(&mut e);
        // isActive answers a pointer to a zero byte.
        let flag = e.mem.alloc(4);
        e.register_double(ENTITY_IS_ACTIVE, move |_, _| Ret {
            eax: flag,
            ..Ret::default()
        });
        e.mem.set_u32(body + 0x10, 0x1234);
        e.mem.set_u8(world + 0xd6, 1);
        e.mem.set_u32(world + 0x8c, 1);
        e.mem.set_u32(world + 0xa4, 1);
        e.mem.set_u32(world + 0x90, 1);
        e.call(0x00ca_8700, &args![body]);
        let order: Vec<u32> = log.borrow().iter().map(|(a, _)| *a).collect();
        assert_eq!(
            order,
            vec![
                0x0900_0020,
                COLLIDE_ENTITIES_BROAD_PHASE,
                ENTITY_ACTIVATE,
                FIRE_INACTIVE_ENTITY_MOVED,
                0x0900_0014,
                WORLD_FLUSH_FIRST,
                WORLD_FLUSH_SECOND,
            ]
        );
        let simulation_call = &calls(&log, 0x0900_0020)[0];
        assert_eq!(simulation_call[2..], [1, world, 7]);
        assert_eq!(e.mem.u32(world + 0x94), 0);
    }

    #[test]
    fn warped_body_update_of_an_active_body_skips_activation() {
        let mut e = test_engine();
        let (world, body, log) = world_rig(&mut e);
        let flag = e.mem.alloc(4);
        e.mem.set_u8(flag, 1);
        e.register_double(ENTITY_IS_ACTIVE, move |_, _| Ret {
            eax: flag,
            ..Ret::default()
        });
        e.mem.set_u8(world + 0x9c, 1);
        e.call(0x00ca_8700, &args![body]);
        let order: Vec<u32> = log.borrow().iter().map(|(a, _)| *a).collect();
        assert_eq!(order, vec![0x0900_0020, 0x0900_0014]);
        assert_eq!(e.mem.u32(world + 0x94), 0);
    }

    /// `createDynamicRigidMotion` with every callee a double; the motion object has the
    /// slots 0x1C, 0x2C and 0xC.
    fn motion_rig(e: &mut Engine) -> (u32, Log) {
        let log = log_calls(
            e,
            &[
                SPHERE_MOTION_CONSTRUCT,
                BOX_MOTION_CONSTRUCT,
                MOTION_TYPE_SIX_CONSTRUCT,
                KEYFRAMED_MOTION_CONSTRUCT,
                CHARACTER_MOTION_CONSTRUCT,
                FIXED_MOTION_CONSTRUCT,
                UFLOAT8_ASSIGN,
                0x0900_001c,
                0x0900_002c,
                0x0900_000c,
            ],
        );
        e.set_global(INERTIA_RATIO, 0.800_000_011_920_929f64);
        e.set_global(KEYFRAMED_DAMPING, 1.0e6f32);
        e.register(FLOAT_MIN, |_, a| Ret {
            st0: f64::from(propagate_nan(a, f32::min)),
            ..Ret::default()
        });
        e.register(FLOAT_MAX, |_, a| Ret {
            st0: f64::from(propagate_nan(a, f32::max)),
            ..Ret::default()
        });
        let vtable = e.mem.alloc(0x40);
        for slot in [0x1c, 0x2c, 0xc] {
            e.mem.set_u32(vtable + slot, 0x0900_0000 + slot);
        }
        let motion = e.mem.alloc(0x100);
        e.mem.set_u32(motion, vtable);
        // The constructors answer the motion.
        for constructor in [
            SPHERE_MOTION_CONSTRUCT,
            BOX_MOTION_CONSTRUCT,
            MOTION_TYPE_SIX_CONSTRUCT,
            KEYFRAMED_MOTION_CONSTRUCT,
            CHARACTER_MOTION_CONSTRUCT,
            FIXED_MOTION_CONSTRUCT,
        ] {
            let sink = Rc::clone(&log);
            e.register_double(constructor, move |_, a| {
                sink.borrow_mut().push((constructor, a.to_vec()));
                Ret {
                    eax: motion,
                    ..Ret::default()
                }
            });
        }
        (motion, log)
    }

    /// `op` of the two floats, NaN when either is.
    fn propagate_nan(a: &[u32], op: fn(f32, f32) -> f32) -> f32 {
        let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
        if x.is_nan() || y.is_nan() {
            f32::NAN
        } else {
            op(x, y)
        }
    }

    fn inertia(e: &mut Engine, diagonal: [f32; 3]) -> u32 {
        let matrix = e.mem.alloc(0x30);
        for (i, v) in diagonal.iter().enumerate() {
            e.mem.set_f32(matrix + 0x14 * i as u32, *v);
        }
        matrix
    }

    #[allow(clippy::too_many_arguments)]
    fn create(e: &mut Engine, motion_type: u32, matrix: u32, buffer: u32) {
        e.call(
            0x00ca_8240,
            &args![
                motion_type,
                1u32,
                2u32,
                3.5f32,
                matrix,
                0x55u32,
                0.25f32,
                0.5f32,
                buffer
            ],
        );
    }

    #[test]
    fn dynamic_motion_picks_sphere_or_box_from_the_inertia_diagonal() {
        let mut e = test_engine();
        let (motion, log) = motion_rig(&mut e);
        let buffer = e.mem.alloc(0x100);
        let similar = inertia(&mut e, [1.0, 0.9, 1.0]);
        create(&mut e, 1, similar, buffer);
        assert_eq!(
            calls(&log, SPHERE_MOTION_CONSTRUCT),
            vec![vec![buffer, 1, 2]]
        );
        let lopsided = inertia(&mut e, [1.0, 0.5, 1.0]);
        create(&mut e, 1, lopsided, buffer);
        assert_eq!(calls(&log, BOX_MOTION_CONSTRUCT), vec![vec![buffer, 1, 2]]);
        // Not a number: unordered, so box.
        let unordered = inertia(&mut e, [f32::NAN, 1.0, 1.0]);
        create(&mut e, 1, unordered, buffer);
        assert_eq!(calls(&log, BOX_MOTION_CONSTRUCT).len(), 2);
        // The set-up calls of the first creation.
        assert_eq!(calls(&log, 0x0900_001c)[0], vec![motion, similar]);
        assert_eq!(calls(&log, 0x0900_002c)[0], vec![motion, 0x55]);
        assert_eq!(calls(&log, 0x0900_000c)[0], vec![motion, 3.5f32.to_bits()]);
        assert_eq!(calls(&log, UFLOAT8_ASSIGN)[0][0], motion + 0xbc,);
    }

    #[test]
    fn dynamic_motion_by_type() {
        let mut e = test_engine();
        let (motion, log) = motion_rig(&mut e);
        let buffer = e.mem.alloc(0x100);
        let matrix = inertia(&mut e, [1.0, 1.0, 1.0]);
        for (motion_type, constructor) in [
            (2, SPHERE_MOTION_CONSTRUCT),
            (3, BOX_MOTION_CONSTRUCT),
            (6, MOTION_TYPE_SIX_CONSTRUCT),
            (7, CHARACTER_MOTION_CONSTRUCT),
            (5, FIXED_MOTION_CONSTRUCT),
            (99, FIXED_MOTION_CONSTRUCT),
            (0, FIXED_MOTION_CONSTRUCT),
        ] {
            let before = calls(&log, constructor).len();
            create(&mut e, motion_type, matrix, buffer);
            assert_eq!(
                calls(&log, constructor).len(),
                before + 1,
                "type {motion_type}"
            );
        }
        // The two damping values reach the hkUFloat8s.
        let assigned = calls(&log, UFLOAT8_ASSIGN);
        assert_eq!(assigned[0][0], motion + 0xbc);
        assert_eq!(assigned[1][0], motion + 0xbd);
    }

    #[test]
    fn dynamic_keyframed_motion_forces_the_damping_and_skips_the_setup() {
        let mut e = test_engine();
        let (motion, log) = motion_rig(&mut e);
        let buffer = e.mem.alloc(0x100);
        let matrix = inertia(&mut e, [1.0, 1.0, 1.0]);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(UFLOAT8_ASSIGN, move |e, a| {
            sink.borrow_mut().push((a[0], e.mem.f32(a[1])));
            Ret::default()
        });
        create(&mut e, 4, matrix, buffer);
        assert_eq!(
            calls(&log, KEYFRAMED_MOTION_CONSTRUCT),
            vec![vec![buffer, 1, 2]]
        );
        assert!(calls(&log, 0x0900_001c).is_empty());
        assert_eq!(
            *seen.borrow(),
            vec![(motion + 0xbc, 1.0e6), (motion + 0xbd, 1.0e6)]
        );
    }
}

#[cfg(test)]
mod rigid_body_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    type CallLog = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;
    type Records = Rc<RefCell<Vec<(u32, Vec<u8>)>>>;
    type Seen = Rc<RefCell<Vec<Vec<u32>>>>;
    type Sizes = Rc<RefCell<Vec<u32>>>;

    /// Doubles for the motion's virtual slots live at `SLOT_BASE + slot`.
    const SLOT_BASE: u32 = 0x0900_0200;
    const BOUNDS_DOUBLE: u32 = 0x0900_0100;
    const ALLOCATE_DOUBLE: u32 = 0x0900_0110;

    /// An engine with the exe's data range and the import page mapped, as the tests run
    /// without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Registers a double at each address that appends `(address, arguments)` to `log`.
    fn double_into(e: &mut Engine, log: &CallLog, addresses: &[u32]) {
        for &address in addresses {
            let log = Rc::clone(log);
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
    }

    fn double(e: &mut Engine, addresses: &[u32]) -> CallLog {
        let log: CallLog = Rc::new(RefCell::new(Vec::new()));
        double_into(e, &log, addresses);
        log
    }

    /// A logging double that answers `eax`.
    fn answer(e: &mut Engine, log: &CallLog, address: u32, eax: u32) {
        let log = Rc::clone(log);
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax,
                ..Ret::default()
            }
        });
    }

    fn calls(log: &CallLog, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn order(log: &CallLog) -> Vec<u32> {
        log.borrow().iter().map(|(a, _)| *a).collect()
    }

    /// A table whose listed slots are logging doubles at `SLOT_BASE + slot`.
    fn vtable_with(e: &mut Engine, log: &CallLog, slots: &[u32]) -> u32 {
        let table = e.mem.alloc(0x80);
        let addresses: Vec<u32> = slots.iter().map(|slot| SLOT_BASE + slot).collect();
        for (&slot, &address) in slots.iter().zip(&addresses) {
            e.mem.set_u32(table + slot, address);
        }
        double_into(e, log, &addresses);
        table
    }

    fn slot_calls(log: &CallLog, slot: u32) -> Vec<Vec<u32>> {
        calls(log, SLOT_BASE + slot)
    }

    fn install_sqrt(e: &mut Engine) {
        e.register_double(X87_SQRT, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32)).sqrt(),
            ..Ret::default()
        });
    }

    fn install_constants(e: &mut Engine) {
        e.set_global(HALF, 0.5f32);
        e.set_global(EXTENT_SCALE, 0.2f64);
        e.set_global(EXTENT_DEFAULT, 0.1f32);
        e.set_global(UNSET_VALUE_DOUBLE, f64::from(f32::MAX));
        e.set_global(MINUS_ONE, -1.0f32);
        e.set_global(UNSET_VALUE, f32::MAX);
    }

    /// A shape vtable whose slot 0x1C double writes `min` at the out pointer and `max` 0x10
    /// after it; also answers the argument lists it saw.
    fn install_bounds(e: &mut Engine, min: [f32; 4], max: [f32; 4]) -> (u32, Seen) {
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(BOUNDS_DOUBLE, move |e, a| {
            sink.borrow_mut().push(a.to_vec());
            for (i, (low, high)) in (0u32..).zip(min.iter().zip(max.iter())) {
                e.mem.set_f32(a[3] + 4 * i, *low);
                e.mem.set_f32(a[3] + 0x10 + 4 * i, *high);
            }
            Ret::default()
        });
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x1c, BOUNDS_DOUBLE);
        (vtable, seen)
    }

    fn shape_with(e: &mut Engine, vtable: u32) -> u32 {
        let shape = e.mem.alloc(0x40);
        e.mem.set_u32(shape, vtable);
        shape
    }

    /// Captures `(world, first `length` bytes of the record)` of every queued operation.
    fn capture_queue(e: &mut Engine, length: u32) -> Records {
        let seen: Records = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(WORLD_QUEUE_OPERATION, move |e, a| {
            sink.borrow_mut().push((a[0], e.mem.bytes(a[1], length)));
            Ret::default()
        });
        seen
    }

    /// The bytes of a record: the kind, three zero bytes, then the words.
    fn record(kind: u8, words: &[u32]) -> Vec<u8> {
        let mut bytes = vec![kind, 0, 0, 0];
        for word in words {
            bytes.extend_from_slice(&word.to_le_bytes());
        }
        bytes
    }

    fn word_at(bytes: &[u8], at: usize) -> u32 {
        u32::from_le_bytes(bytes[at..at + 4].try_into().unwrap())
    }

    /// A memory router whose allocator's slot 4 hands out zeroed blocks; answers the sizes
    /// asked and the allocator object.
    fn install_allocator(e: &mut Engine) -> (Sizes, u32) {
        let sizes = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&sizes);
        e.register_double(ALLOCATE_DOUBLE, move |e, a| {
            sink.borrow_mut().push(a[1]);
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        (sizes, allocator)
    }

    fn fill(e: &mut Engine, address: u32, seed: u8) {
        for i in 0..0x10u8 {
            e.mem.set_u8(address + u32::from(i), seed.wrapping_add(i));
        }
    }

    #[test]
    fn destructor_body_stores_the_vtable_then_runs_the_base() {
        let mut e = test_engine();
        let log = double(&mut e, &[ENTITY_BASE_DESTRUCT]);
        let body = e.mem.alloc(0x40);
        e.call(0x00ca_80a0, &args![body]);
        assert_eq!(e.mem.u32(body), RIGID_BODY_VTABLE);
        assert_eq!(calls(&log, ENTITY_BASE_DESTRUCT), vec![vec![body]]);
    }

    #[test]
    fn motion_state_address_and_enabled_flag() {
        let mut e = test_engine();
        let body = e.mem.alloc(0x240);
        assert_eq!(e.call(0x00ca_80b0, &args![body]).u32(), body + 0xf0);
        e.mem.set_u8(body + 0xe9, 0xff);
        assert!(!e.call(0x00ca_80c0, &args![body]).bool());
        e.mem.set_u8(body + 0xe9, 0);
        assert!(e.call(0x00ca_80c0, &args![body]).bool());
        e.mem.set_u8(body + 0xe9, 3);
        assert!(e.call(0x00ca_80c0, &args![body]).bool());
    }

    #[test]
    fn smallest_extent_scales_below_one_half_and_uses_the_default_otherwise() {
        let mut e = test_engine();
        install_constants(&mut e);
        let vector = e.mem.alloc(0x10);
        let target = e.mem.alloc(0x80);
        let run = |e: &mut Engine, v: [f32; 3]| {
            for (i, value) in (0u32..).zip(v) {
                e.mem.set_f32(vector + 4 * i, value);
            }
            e.mem.set_u32(target + 0x4c, 0xdead_beef);
            e.call(0x00ca_80d0, &args![vector, target]);
            e.mem.f32(target + 0x4c)
        };
        assert_eq!(
            run(&mut e, [0.4, 0.3, 0.45]),
            (f64::from(0.3f32) * 0.2) as f32
        );
        assert_eq!(
            run(&mut e, [0.2, 0.6, 0.9]),
            (f64::from(0.2f32) * 0.2) as f32
        );
        assert_eq!(
            run(&mut e, [0.6, 0.1, 0.55]),
            (f64::from(0.1f32) * 0.2) as f32
        );
        assert_eq!(run(&mut e, [0.9, 0.8, 0.7]), 0.1f32);
        // Exactly 0.5 is not below the limit.
        assert_eq!(run(&mut e, [0.5, 0.5, 0.5]), 0.1f32);
        // An unordered compare keeps the second operand, so the NaN ends up smallest.
        assert_eq!(run(&mut e, [1.0, 1.0, f32::NAN]), 0.1f32);
    }

    #[test]
    fn cached_shape_info_stores_the_extent_and_the_root_of_the_larger_distance() {
        let mut e = test_engine();
        install_sqrt(&mut e);
        let (vtable, seen) = install_bounds(&mut e, [-2.0, -2.0, -2.0, 0.0], [1.0, 1.0, 1.0, 0.0]);
        let shape = shape_with(&mut e, vtable);
        let body = e.mem.alloc(0x240);
        let out = e.mem.alloc(0x10);
        e.call(0x00ca_8150, &args![body, shape, out]);
        let lanes = [0u32, 1, 2, 3].map(|i| e.mem.f32(out + 4 * i));
        assert_eq!(lanes, [3.0, 3.0, 3.0, 0.0]);
        // The minimum's squares sum to 12, the maximum's to 3.
        assert_eq!(e.mem.f32(body + 0x190), 12.0f64.sqrt() as f32);
        let first = seen.borrow()[0].clone();
        assert_eq!(first[..3], [shape, SHAPE_TRANSFORM, 0]);
        // With the centre moved the maximum's sum (8) is the larger.
        let (vtable, _) = install_bounds(&mut e, [0.0, 0.0, 0.0, 0.0], [1.0, 2.0, 2.0, 0.0]);
        let shape = shape_with(&mut e, vtable);
        e.mem.set_f32(body + 0x170, 1.0);
        e.call(0x00ca_8150, &args![body, shape, out]);
        assert_eq!(e.mem.f32(body + 0x190), 8.0f64.sqrt() as f32);
    }

    /// Doubles for everything `setShape` calls; the new shape reports a box of
    /// (0.4, 0.3, 0.45).
    fn shape_change_rig(e: &mut Engine, with_world: bool) -> (u32, u32, u32, CallLog) {
        install_constants(e);
        install_sqrt(e);
        let log = double(
            e,
            &[
                REMOVE_ENTITY_BROAD_PHASE,
                ADD_REFERENCE,
                REMOVE_REFERENCE,
                ENTITY_SET_CACHED_SHAPE_DATA,
                WORLD_FIRE_SHAPE_SET,
                ENTITY_FIRE_SHAPE_SET,
                ADD_ENTITY_BROAD_PHASE,
                WORLD_FLUSH_FIRST,
                WORLD_FLUSH_SECOND,
            ],
        );
        let (vtable, _) = install_bounds(e, [0.0; 4], [0.4, 0.3, 0.45, 0.0]);
        let shape = shape_with(e, vtable);
        let body = e.mem.alloc(0x240);
        let world = if with_world { e.mem.alloc(0x400) } else { 0 };
        e.mem.set_u32(body + 8, world);
        (body, world, shape, log)
    }

    #[test]
    fn set_shape_with_a_locked_world_queues_the_change() {
        let mut e = test_engine();
        let (body, world, shape, log) = shape_change_rig(&mut e, true);
        e.mem.set_u32(world + 0x94, 1);
        let queued = capture_queue(&mut e, 0xc);
        assert!(!e.call(0x00ca_84b0, &args![body, shape]).bool());
        assert_eq!(*queued.borrow(), vec![(world, record(5, &[body, shape]))]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(body + 0x10), 0);
        assert_eq!(e.mem.u32(world + 0x94), 1);
    }

    #[test]
    fn set_shape_without_a_world_or_an_old_shape_keeps_a_positive_value() {
        let mut e = test_engine();
        let (body, _, shape, log) = shape_change_rig(&mut e, false);
        e.mem.set_f32(body + 0x5c, 5.0);
        assert!(e.call(0x00ca_84b0, &args![body, shape]).bool());
        assert_eq!(e.mem.u32(body + 0x10), shape);
        assert_eq!(e.mem.f32(body + 0x5c), 5.0);
        assert_eq!(
            *log.borrow(),
            vec![
                (ADD_REFERENCE, vec![shape]),
                (ENTITY_FIRE_SHAPE_SET, vec![body])
            ]
        );
        let square = |x: f32| x * x;
        let sum = square(0.45) + (square(0.3) + square(0.4));
        assert_eq!(e.mem.f32(body + 0x190), f64::from(sum).sqrt() as f32);
    }

    #[test]
    fn set_shape_with_an_old_shape_resets_the_value_unless_it_is_the_unset_marker() {
        let mut e = test_engine();
        let (body, _, shape, log) = shape_change_rig(&mut e, false);
        let old = 0x0555_5550;
        e.mem.set_u32(body + 0x10, old);
        e.mem.set_f32(body + 0x5c, 5.0);
        assert!(e.call(0x00ca_84b0, &args![body, shape]).bool());
        // Reset to -1.0, which is not above 0.0, so the box extent decides.
        assert_eq!(e.mem.f32(body + 0x5c), (f64::from(0.3f32) * 0.2) as f32);
        assert_eq!(calls(&log, REMOVE_REFERENCE), vec![vec![old]]);
        // The marker value is left alone, and being positive it is not recomputed.
        e.mem.set_f32(body + 0x5c, f32::MAX);
        assert!(e.call(0x00ca_84b0, &args![body, shape]).bool());
        assert_eq!(e.mem.f32(body + 0x5c), f32::MAX);
    }

    #[test]
    fn set_shape_with_an_unlocked_world_runs_the_whole_sequence() {
        let mut e = test_engine();
        let (body, world, shape, log) = shape_change_rig(&mut e, true);
        e.mem.set_u32(world + 0x8c, 1);
        e.mem.set_u32(world + 0xa4, 1);
        e.mem.set_u32(world + 0x90, 1);
        let old = 0x0555_5550;
        e.mem.set_u32(body + 0x10, old);
        e.mem.set_f32(body + 0x5c, f32::MAX);
        assert!(e.call(0x00ca_84b0, &args![body, shape]).bool());
        assert_eq!(
            order(&log),
            vec![
                REMOVE_ENTITY_BROAD_PHASE,
                ADD_REFERENCE,
                REMOVE_REFERENCE,
                ENTITY_SET_CACHED_SHAPE_DATA,
                WORLD_FIRE_SHAPE_SET,
                ENTITY_FIRE_SHAPE_SET,
                ADD_ENTITY_BROAD_PHASE,
                WORLD_FLUSH_FIRST,
                WORLD_FLUSH_SECOND,
            ]
        );
        assert_eq!(
            calls(&log, REMOVE_ENTITY_BROAD_PHASE),
            vec![vec![world, body]]
        );
        assert_eq!(
            calls(&log, ENTITY_SET_CACHED_SHAPE_DATA),
            vec![vec![body, world, shape]]
        );
        assert_eq!(calls(&log, WORLD_FIRE_SHAPE_SET), vec![vec![world, body]]);
        assert_eq!(calls(&log, ADD_ENTITY_BROAD_PHASE), vec![vec![world, body]]);
        assert_eq!(calls(&log, WORLD_FLUSH_FIRST), vec![vec![world]]);
        assert_eq!(calls(&log, WORLD_FLUSH_SECOND), vec![vec![world]]);
        assert_eq!(e.mem.u32(world + 0x94), 0);
        // With the pending flag set neither flush runs.
        log.borrow_mut().clear();
        e.mem.set_u8(world + 0x9c, 1);
        assert!(e.call(0x00ca_84b0, &args![body, shape]).bool());
        assert!(!order(&log).contains(&WORLD_FLUSH_FIRST));
        assert!(!order(&log).contains(&WORLD_FLUSH_SECOND));
        assert_eq!(e.mem.u32(world + 0x94), 0);
    }

    #[test]
    fn center_of_mass_goes_to_the_motion_and_refreshes_the_cached_info() {
        let mut e = test_engine();
        install_sqrt(&mut e);
        let (vtable, seen) = install_bounds(&mut e, [0.0; 4], [3.0, 0.0, 0.0, 0.0]);
        let shape = shape_with(&mut e, vtable);
        let body = e.mem.alloc(0x240);
        e.mem.set_u32(body + 0x10, shape);
        let log: CallLog = Rc::new(RefCell::new(Vec::new()));
        let table = vtable_with(&mut e, &log, &[0x2c]);
        e.mem.set_u32(body + 0xe0, table);
        let centre = e.mem.alloc(0x10);
        e.call(0x00ca_85f0, &args![body, centre]);
        assert_eq!(slot_calls(&log, 0x2c), vec![vec![body + 0xe0, centre]]);
        assert_eq!(seen.borrow().len(), 1);
        assert_eq!(e.mem.f32(body + 0x190), 3.0);
    }

    #[test]
    fn deactivation_flags_follow_the_flag_and_the_world() {
        let mut e = test_engine();
        let log = double(&mut e, &[MOTION_SET_DEACTIVATION_FLAGS]);
        let body = e.mem.alloc(0x240);
        e.mem.set_u8(body + 0xe9, 3);
        e.mem.set_u16(body + 0xea, 0x1111);
        e.mem.set_u16(body + 0xec, 0x2222);
        e.call(0x00ca_8630, &args![body, 0u32]);
        assert_eq!(e.mem.u8(body + 0xe9), 0xff);
        assert_eq!(e.mem.u16(body + 0xea), 0);
        assert_eq!(e.mem.u16(body + 0xec), 0);
        assert!(log.borrow().is_empty());
        e.mem.set_u8(body + 0xd4, 0x35);
        e.call(0x00ca_8630, &args![body, 1u32]);
        assert_eq!(e.mem.u8(body + 0xe9), 5);
        let world = e.mem.alloc(0x400);
        e.mem.set_u8(world + 0x305, 7);
        e.mem.set_u8(world + 0x306, 8);
        e.mem.set_u8(world + 0x307, 9);
        e.mem.set_u32(body + 8, world);
        e.mem.set_u8(body + 0xd4, 0x2a);
        e.call(0x00ca_8630, &args![body, 1u32]);
        assert_eq!(e.mem.u8(body + 0xe9), 0xa);
        assert_eq!(
            calls(&log, MOTION_SET_DEACTIVATION_FLAGS),
            vec![vec![body + 0xe0, 0, 0, 0], vec![body + 0xe0, 7, 8, 9]]
        );
    }

    #[test]
    fn extra_word_is_only_answered_for_motion_types_four_and_five() {
        let mut e = test_engine();
        let body = e.mem.alloc(0x240);
        e.mem.set_u32(body + 0x1f8, 0x1234);
        for (motion, expected) in [(1u8, 0u32), (3, 0), (4, 0x1234), (5, 0x1234), (6, 0)] {
            e.mem.set_u8(body + 0xe8, motion);
            assert_eq!(e.call(0x00ca_86c0, &args![body]).u32(), expected);
        }
    }

    #[test]
    fn position_rotation_and_transform_setters_call_their_motion_slot() {
        let mut e = test_engine();
        let log: CallLog = Rc::new(RefCell::new(Vec::new()));
        let table = vtable_with(&mut e, &log, &[0x30, 0x34, 0x3c]);
        let body = e.mem.alloc(0x240);
        e.mem.set_u32(body + 0xe0, table);
        let (a, b, c) = (0x1111_1110, 0x2222_2220, 0x3333_3330);
        e.call(0x00ca_8800, &args![body, a]);
        e.call(0x00ca_8830, &args![body, b]);
        e.call(0x00ca_8890, &args![body, c]);
        assert_eq!(slot_calls(&log, 0x30), vec![vec![body + 0xe0, a]]);
        assert_eq!(slot_calls(&log, 0x34), vec![vec![body + 0xe0, b]]);
        assert_eq!(slot_calls(&log, 0x3c), vec![vec![body + 0xe0, c]]);
    }

    #[test]
    fn float_setters_pass_quieted_bit_patterns() {
        let mut e = test_engine();
        let log: CallLog = Rc::new(RefCell::new(Vec::new()));
        let table = vtable_with(&mut e, &log, &[0xc, 0x10]);
        let body = e.mem.alloc(0x240);
        e.mem.set_u32(body + 0xe0, table);
        e.call(0x00ca_88c0, &args![body, 0x7f80_0001u32]);
        e.call(0x00ca_88e0, &args![body, 0x3f80_0000u32]);
        assert_eq!(slot_calls(&log, 0xc), vec![vec![body + 0xe0, 0x7fc0_0001]]);
        assert_eq!(slot_calls(&log, 0x10), vec![vec![body + 0xe0, 0x3f80_0000]]);
    }

    #[test]
    fn slot_one_c_receives_the_callers_word() {
        let mut e = test_engine();
        let log: CallLog = Rc::new(RefCell::new(Vec::new()));
        let table = vtable_with(&mut e, &log, &[0x1c]);
        let body = e.mem.alloc(0x240);
        e.mem.set_u32(body + 0xe0, table);
        e.call(0x00ca_8920, &args![body, 0x4444_4440u32]);
        assert_eq!(slot_calls(&log, 0x1c), vec![vec![body + 0xe0, 0x4444_4440]]);
    }

    #[test]
    fn position_and_rotation_are_queued_when_the_world_is_locked() {
        let mut e = test_engine();
        let (sizes, _) = install_allocator(&mut e);
        let queued = capture_queue(&mut e, 0xc);
        let log: CallLog = Rc::new(RefCell::new(Vec::new()));
        let table = vtable_with(&mut e, &log, &[0x38]);
        let body = e.mem.alloc(0x240);
        e.mem.set_u32(body + 0xe0, table);
        let (first, second) = (e.mem.alloc(0x10), e.mem.alloc(0x10));
        fill(&mut e, first, 1);
        fill(&mut e, second, 0x41);
        // No world: applied at once through the motion's slot 0x38.
        e.call(0x00ca_8960, &args![body, first, second]);
        assert_eq!(
            slot_calls(&log, 0x38),
            vec![vec![body + 0xe0, first, second]]
        );
        assert!(queued.borrow().is_empty());
        let world = e.mem.alloc(0x100);
        e.mem.set_u32(world + 0x94, 1);
        e.mem.set_u32(body + 8, world);
        e.call(0x00ca_8960, &args![body, first, second]);
        let (queued_world, bytes) = queued.borrow()[0].clone();
        assert_eq!(queued_world, world);
        assert_eq!(bytes[0], 0x18);
        assert_eq!(word_at(&bytes, 4), body);
        let block = word_at(&bytes, 8);
        assert_eq!(*sizes.borrow(), vec![0x20]);
        let mut expected = e.mem.bytes(first, 0x10);
        expected.extend(e.mem.bytes(second, 0x10));
        assert_eq!(e.mem.bytes(block, 0x20), expected);
        assert_eq!(slot_calls(&log, 0x38).len(), 1);
    }

    #[test]
    fn vector_setters_queue_or_activate_and_forward() {
        for (address, kind, slot) in [(0x00ca_8a80u32, 0x1bu8, 0x4cu32), (0x00ca_8b80, 0x1d, 0x54)]
        {
            let mut e = test_engine();
            let queued = capture_queue(&mut e, 0x14);
            let log = double(&mut e, &[ENTITY_ACTIVATE]);
            let table = vtable_with(&mut e, &log, &[slot]);
            let body = e.mem.alloc(0x240);
            e.mem.set_u32(body + 0xe0, table);
            let vector = e.mem.alloc(0x10);
            e.mem.set_u32(vector, 0x3f80_0000);
            e.mem.set_u32(vector + 4, 0x7f80_0001);
            e.mem.set_u32(vector + 8, 0x4040_0000);
            // A world that is not locked takes the activate-and-forward path as well.
            let world = e.mem.alloc(0x100);
            e.mem.set_u32(body + 8, world);
            e.call(address, &args![body, vector]);
            assert_eq!(
                order(&log),
                vec![ENTITY_ACTIVATE, SLOT_BASE + slot],
                "slot {slot:#x}"
            );
            assert_eq!(slot_calls(&log, slot), vec![vec![body + 0xe0, vector]]);
            assert!(queued.borrow().is_empty());
            e.mem.set_u32(world + 0x94, 2);
            e.call(address, &args![body, vector]);
            let expected = record(kind, &[body, 0x3f80_0000, 0x7fc0_0001, 0x4040_0000]);
            assert_eq!(*queued.borrow(), vec![(world, expected)]);
            assert_eq!(log.borrow().len(), 2);
        }
    }

    #[test]
    fn pair_setter_queues_swapped_blocks_or_forwards_both_words() {
        let mut e = test_engine();
        let (sizes, _) = install_allocator(&mut e);
        let queued = capture_queue(&mut e, 0xc);
        let log = double(&mut e, &[ENTITY_ACTIVATE]);
        let table = vtable_with(&mut e, &log, &[0x50]);
        let body = e.mem.alloc(0x240);
        e.mem.set_u32(body + 0xe0, table);
        let (first, second) = (e.mem.alloc(0x10), e.mem.alloc(0x10));
        fill(&mut e, first, 1);
        fill(&mut e, second, 0x41);
        e.call(0x00ca_8af0, &args![body, first, second]);
        assert_eq!(order(&log), vec![ENTITY_ACTIVATE, SLOT_BASE + 0x50]);
        assert_eq!(
            slot_calls(&log, 0x50),
            vec![vec![body + 0xe0, first, second]]
        );
        let world = e.mem.alloc(0x100);
        e.mem.set_u32(world + 0x94, 1);
        e.mem.set_u32(body + 8, world);
        e.call(0x00ca_8af0, &args![body, first, second]);
        let (queued_world, bytes) = queued.borrow()[0].clone();
        assert_eq!(queued_world, world);
        assert_eq!(bytes[0], 0x1c);
        assert_eq!(word_at(&bytes, 4), body);
        let block = word_at(&bytes, 8);
        assert_eq!(*sizes.borrow(), vec![0x20]);
        // The second argument comes first in the block.
        let mut expected = e.mem.bytes(second, 0x10);
        expected.extend(e.mem.bytes(first, 0x10));
        assert_eq!(e.mem.bytes(block, 0x20), expected);
        assert_eq!(log.borrow().len(), 2);
    }

    #[test]
    fn size_check_follows_the_shape_kind_and_motion_type() {
        const COUNT_DOUBLE: u32 = 0x0900_0400;
        const INNER_DOUBLE: u32 = 0x0900_0404;
        let mut e = test_engine();
        let count = Rc::new(Cell::new(0i32));
        let answer_count = Rc::clone(&count);
        e.register_double(COUNT_DOUBLE, move |_, _| Ret {
            eax: answer_count.get() as u32,
            ..Ret::default()
        });
        let counted = e.mem.alloc(0x20);
        e.mem.set_u32(counted + 4, COUNT_DOUBLE);
        let inner = e.mem.alloc(0x10);
        e.mem.set_u32(inner, counted);
        e.register_double(INNER_DOUBLE, move |_, _| Ret {
            eax: inner,
            ..Ret::default()
        });
        let shape_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(shape_vtable + 0x10, INNER_DOUBLE);
        let out = e.mem.alloc(8);
        // No shape: the byte is 0.
        let body = e.mem.alloc(0x240);
        e.mem.set_u8(out, 0x55);
        e.call(0x00ca_8bf0, &args![body, out]);
        assert_eq!(e.mem.u8(out), 0);
        let cases: [(u8, u32, i32, u8); 14] = [
            (1, 0xa, 100, 1),
            (1, 0xa, 101, 0),
            (1, 8, 101, 0),
            (4, 8, 101, 1),
            (5, 0xa, 101, 1),
            (1, 0x17, 101, 0),
            (1, 0x17, 100, 1),
            (4, 0x17, 101, 0),
            (1, 7, 101, 0),
            (1, 9, 101, 0),
            (1, 9, 100, 1),
            (1, 0xf, 0, 0),
            (1, 3, 1000, 1),
            (1, 0xa, -1, 1),
        ];
        for (motion, kind, size, expected) in cases {
            count.set(size);
            let shape = e.mem.alloc(0x40);
            e.mem.set_u32(shape, shape_vtable);
            e.mem.set_u32(shape + 0xc, kind);
            e.mem.set_u32(shape + 0x10, counted);
            let body = e.mem.alloc(0x240);
            e.mem.set_u32(body + 0x10, shape);
            e.mem.set_u8(body + 0xe8, motion);
            e.mem.set_u8(out, 0x55);
            e.call(0x00ca_8bf0, &args![body, out]);
            assert_eq!(
                e.mem.u8(out),
                expected,
                "motion {motion} kind {kind:#x} count {size}"
            );
        }
    }

    #[test]
    fn fn_00ca8bf0_answers_its_result_pointer() {
        const COUNT_DOUBLE: u32 = 0x0900_0400;
        const INNER_DOUBLE: u32 = 0x0900_0404;
        let mut e = test_engine();
        let count = Rc::new(Cell::new(0i32));
        let answer_count = Rc::clone(&count);
        e.register_double(COUNT_DOUBLE, move |_, _| Ret {
            eax: answer_count.get() as u32,
            ..Ret::default()
        });
        let counted = e.mem.alloc(0x20);
        e.mem.set_u32(counted + 4, COUNT_DOUBLE);
        let inner = e.mem.alloc(0x10);
        e.mem.set_u32(inner, counted);
        e.register_double(INNER_DOUBLE, move |_, _| Ret {
            eax: inner,
            ..Ret::default()
        });
        let shape_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(shape_vtable + 0x10, INNER_DOUBLE);
        let out = e.mem.alloc(8);
        // No shape: the byte is 0.
        let body = e.mem.alloc(0x240);
        e.mem.set_u8(out, 0x55);
        // The exe leaves the result pointer in EAX at every RET; callers read through it.
        assert_eq!(e.call(0x00ca_8bf0, &args![body, out]).u32(), args![out][0]);
    }

    /// Doubles for everything the constructor calls; answers the log, the motion object
    /// the motion constructors return and the shape.
    fn constructor_rig(e: &mut Engine) -> (CallLog, u32, u32) {
        install_constants(e);
        install_sqrt(e);
        let log = double(
            e,
            &[
                ADD_REFERENCE,
                REMOVE_REFERENCE,
                UFLOAT8_ASSIGN,
                MOTION_SET_DEACTIVATION_CLASS,
                SET_LINEAR_VELOCITY,
                SET_ANGULAR_VELOCITY,
                MOTION_SET_DEACTIVATION_FLAGS,
                STREAM_INIT,
                STREAM_WRITE_TEXT,
                STREAM_DESTRUCT,
            ],
        );
        let sink = Rc::clone(&log);
        e.register_double(ENTITY_CONSTRUCT, move |e, a| {
            sink.borrow_mut().push((ENTITY_CONSTRUCT, a.to_vec()));
            e.mem.set_u32(a[0] + 0x10, a[1]);
            Ret::default()
        });
        let motion = e.mem.alloc(0x100);
        let table = vtable_with(e, &log, &[0x1c, 0x2c, 0xc]);
        e.mem.set_u32(motion, table);
        // The motion constructors store the motion type byte at +8, which is body + 0xE8.
        for (address, kind) in [
            (FIXED_MOTION_CONSTRUCT, 5u8),
            (BOX_MOTION_CONSTRUCT, 3),
            (KEYFRAMED_MOTION_CONSTRUCT, 4),
        ] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |e, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                e.mem.set_u8(a[0] + 8, kind);
                Ret {
                    eax: motion,
                    ..Ret::default()
                }
            });
        }
        answer(e, &log, SHAPE_KEY_LEVEL_LIMIT, 6);
        let reporter = e.mem.alloc(0x20);
        let reporter_table = e.mem.alloc(0x80);
        e.mem.set_u32(reporter_table + 0xc, SLOT_BASE + 0x80);
        double_into(e, &log, &[SLOT_BASE + 0x80]);
        e.mem.set_u32(reporter, reporter_table);
        e.set_global(ERROR_REPORTER, reporter);
        let (vtable, _) = install_bounds(e, [0.0; 4], [2.0, 2.0, 2.0, 0.0]);
        let shape = shape_with(e, vtable);
        (log, motion, shape)
    }

    /// A construction info with distinct values in every field.
    fn construction_info(e: &mut Engine, shape: u32, motion: u8) -> u32 {
        let c = e.mem.alloc(0xc0);
        e.mem.set_u32(c, 0xaabb);
        e.mem.set_u32(c + 4, shape);
        e.mem.set_u32(c + 8, shape);
        e.mem.set_u8(c + 0xc, 0x11);
        e.mem.set_u16(c + 0xe, 0x2233);
        e.mem.set_f32(c + 0x90, 12.5);
        e.mem.set_f32(c + 0x94, 1.5);
        e.mem.set_f32(c + 0x98, 2.5);
        e.mem.set_u16(c + 0x9e, 0x4455);
        e.mem.set_f32(c + 0xa0, 3.5);
        e.mem.set_f32(c + 0xa4, 4.5);
        e.mem.set_f32(c + 0xa8, 0.75);
        e.mem.set_f32(c + 0xac, 0.25);
        e.mem.set_f32(c + 0xb0, 0.25);
        e.mem.set_u8(c + 0xb4, motion);
        e.mem.set_u8(c + 0xb5, 1);
        e.mem.set_u8(c + 0xb6, 0xfe);
        e.mem.set_u8(c + 0xb7, 3);
        e.mem.set_u8(c + 0xb8, 7);
        e.mem.set_u8(c + 0xb9, 9);
        e.mem.set_u8(c + 0xba, 0);
        e.mem.set_u8(c + 0xbb, 1);
        c
    }

    #[test]
    fn constructor_with_a_fixed_motion_copies_the_construction_info() {
        let mut e = test_engine();
        let (log, motion, shape) = constructor_rig(&mut e);
        let c = construction_info(&mut e, shape, 5);
        let body = e.mem.alloc(0x240);
        let old = 0x0555_5550;
        e.mem.set_u32(body + 0x210, old);
        e.mem.set_u8(body + 0x21, 0x80);
        assert_eq!(e.call(0x00ca_8c90, &args![body, c]).u32(), body);
        assert_eq!(e.mem.u32(body), RIGID_BODY_VTABLE);
        assert_eq!(e.mem.u8(body + 0x8c), 0x11);
        assert_eq!(e.mem.u16(body + 0xaa), 0x2233);
        assert_eq!(e.mem.u32(body + 0x2c), 0xaabb);
        assert_eq!(e.mem.u32(body + 0x210), shape);
        assert_eq!(calls(&log, ENTITY_CONSTRUCT), vec![vec![body, shape]]);
        assert_eq!(calls(&log, ADD_REFERENCE), vec![vec![shape], vec![shape]]);
        assert_eq!(calls(&log, REMOVE_REFERENCE), vec![vec![old]]);
        assert_eq!(
            calls(&log, FIXED_MOTION_CONSTRUCT),
            vec![vec![body + 0xe0, c + 0x10, c + 0x20]]
        );
        assert_eq!(
            calls(&log, UFLOAT8_ASSIGN),
            vec![vec![motion + 0xbc, c + 0xa8], vec![motion + 0xbd, c + 0xac]]
        );
        assert_eq!(
            calls(&log, MOTION_SET_DEACTIVATION_CLASS),
            vec![vec![motion, 1]]
        );
        assert!(calls(&log, SET_LINEAR_VELOCITY).is_empty());
        assert_eq!(e.mem.u32(body + 0x18), body + 0xf0);
        // A positive value is copied, and then no recomputation takes place.
        assert_eq!(e.mem.f32(body + 0x5c), 0.25);
        assert_eq!(
            calls(&log, MOTION_SET_DEACTIVATION_FLAGS),
            vec![vec![body + 0xe0, 0, 0, 0]]
        );
        assert_eq!(e.mem.u8(body + 0xe9), 0);
        assert_eq!(e.mem.f32(body + 0x194), 1.5);
        assert_eq!(e.mem.f32(body + 0x198), 2.5);
        assert_eq!(e.mem.f32(body + 0x190), 12.0f64.sqrt() as f32);
        assert_eq!(e.mem.u8(body + 0x2a), 3);
        assert_eq!(e.mem.u8(body + 0xd0), 7);
        assert_eq!(e.mem.u8(body + 0x21), 0x81);
        assert_eq!(e.mem.f32(body + 0x90), 3.5);
        assert_eq!(e.mem.f32(body + 0x94), 4.5);
        assert_eq!(e.mem.u16(body + 0x1fe), 0x4455);
        assert_eq!(e.mem.u8(body + 0xd1), 0);
        assert_eq!(e.mem.u8(body + 0xd2), 9);
        assert!(calls(&log, SHAPE_KEY_LEVEL_LIMIT).is_empty());
        // The default quality of a fixed body is 0; a value of 0 or less becomes the
        // float constant at 0104da48 and, being positive, stays.
        let body = e.mem.alloc(0x240);
        e.mem.set_u8(c + 0xb7, 0xff);
        e.mem.set_f32(c + 0xb0, -1.0);
        e.mem.set_u8(c + 0xbb, 0);
        e.mem.set_u8(c + 0xb5, 0);
        e.call(0x00ca_8c90, &args![body, c]);
        assert_eq!(e.mem.u8(body + 0x2a), 0);
        assert_eq!(e.mem.u8(body + 0x21), 0);
        assert_eq!(e.mem.f32(body + 0x5c), f32::MAX);
        assert_eq!(e.mem.u8(body + 0xe9), 0xff);
    }

    #[test]
    fn constructor_with_a_dynamic_motion_builds_it_and_sets_velocities() {
        let mut e = test_engine();
        let (log, motion, shape) = constructor_rig(&mut e);
        let c = construction_info(&mut e, shape, 3);
        e.mem.set_f32(c + 0xb0, 0.0);
        e.mem.set_u8(c + 0xb7, 0xff);
        let body = e.mem.alloc(0x240);
        assert_eq!(e.call(0x00ca_8c90, &args![body, c]).u32(), body);
        assert_eq!(
            calls(&log, BOX_MOTION_CONSTRUCT),
            vec![vec![body + 0xe0, c + 0x10, c + 0x20]]
        );
        assert!(calls(&log, FIXED_MOTION_CONSTRUCT).is_empty());
        assert_eq!(slot_calls(&log, 0x1c), vec![vec![motion, c + 0x50]]);
        assert_eq!(slot_calls(&log, 0x2c), vec![vec![motion, c + 0x80]]);
        assert_eq!(slot_calls(&log, 0xc), vec![vec![motion, 12.5f32.to_bits()]]);
        let assigned: Vec<u32> = calls(&log, UFLOAT8_ASSIGN).iter().map(|a| a[0]).collect();
        assert_eq!(assigned, vec![motion + 0xbc, motion + 0xbd]);
        assert_eq!(
            calls(&log, MOTION_SET_DEACTIVATION_CLASS),
            vec![vec![body + 0xe0, 0xffff_fffe]]
        );
        assert_eq!(calls(&log, SET_LINEAR_VELOCITY), vec![vec![body, c + 0x30]]);
        assert_eq!(
            calls(&log, SET_ANGULAR_VELOCITY),
            vec![vec![body, c + 0x40]]
        );
        assert_eq!(e.mem.u32(body + 0x18), body + 0xf0);
        // The copied 0.0 is not above 0.0, so the extent (smallest 2.0, not below one half)
        // gives the default constant.
        assert_eq!(e.mem.f32(body + 0x5c), 0.1f32);
        assert_eq!(e.mem.u8(body + 0x2a), 2);
        // Motion type 4 gives quality 1.
        let body = e.mem.alloc(0x240);
        e.mem.set_u8(c + 0xb4, 4);
        e.call(0x00ca_8c90, &args![body, c]);
        assert_eq!(e.mem.u8(body + 0x2a), 1);
    }

    #[test]
    fn constructor_reports_a_shape_key_level_above_the_limit() {
        let cases = [
            (0x80u8, 6u8, false),
            (3, 3, true),
            (7, 7, false),
            (6, 6, false),
            (0, 0, false),
        ];
        for (level, stored, asserts) in cases {
            let mut e = test_engine();
            let (log, _, shape) = constructor_rig(&mut e);
            let c = construction_info(&mut e, shape, 5);
            e.mem.set_u8(c + 0xba, level);
            let body = e.mem.alloc(0x240);
            e.call(0x00ca_8c90, &args![body, c]);
            assert_eq!(e.mem.u8(body + 0xd1), stored, "level {level:#x}");
            let limit_calls = calls(&log, SHAPE_KEY_LEVEL_LIMIT);
            assert_eq!(limit_calls.len(), usize::from(level != 0));
            assert_eq!(
                limit_calls.first().map(|a| a[0]),
                (level != 0).then_some(shape)
            );
            assert_eq!(calls(&log, STREAM_INIT).len(), usize::from(asserts));
            assert_eq!(calls(&log, STREAM_DESTRUCT).len(), usize::from(asserts));
            if asserts {
                let init = &calls(&log, STREAM_INIT)[0];
                assert_eq!(init[2], 0x200);
                assert_eq!(
                    calls(&log, STREAM_WRITE_TEXT),
                    vec![vec![init[0], ASSERT_MESSAGE]]
                );
                assert_eq!(
                    slot_calls(&log, 0x80)
                        .iter()
                        .map(|a| a[1..].to_vec())
                        .collect::<Vec<_>>(),
                    vec![vec![1, 0x458a_1bb2, init[1], ASSERT_FILE, 0x149]]
                );
            }
        }
    }

    #[test]
    fn construction_info_is_read_back_from_the_body() {
        let mut e = test_engine();
        e.set_global(DAMPING_TABLE + 8, 0.02f32);
        e.set_global(DAMPING_TABLE + 12, 0.03f32);
        e.register_double(MOTION_GET_MASS, |_, _| Ret {
            st0: 12.5,
            ..Ret::default()
        });
        let table = e.mem.alloc(0x80);
        e.mem.set_u32(table + 0x14, SLOT_BASE + 0x14);
        e.register_double(SLOT_BASE + 0x14, |e, a| {
            e.mem.set_u32(a[1], 0xfeed);
            Ret::default()
        });
        let body = e.mem.alloc(0x240);
        e.mem.set_u32(body + 0xe0, table);
        e.mem.set_u8(body + 0x21, 3);
        e.mem.set_u8(body + 0xd0, 0x41);
        e.mem.set_u8(body + 0xd1, 0x42);
        e.mem.set_u16(body + 0xaa, 0x1234);
        e.mem.set_u8(body + 0xe9, 3);
        e.mem.set_u32(body + 0x210, 0x0777_7770);
        e.mem.set_u32(body + 0x90, 0x7f80_0001);
        e.mem.set_u8(body + 0x8c, 0x5a);
        e.mem.set_f32(body + 0x94, 2.0);
        e.mem.set_f32(body + 0x194, 3.0);
        e.mem.set_f32(body + 0x198, 4.0);
        for (offset, seed) in [
            (0x1b0u32, 0x10u8),
            (0x1c0, 0x20),
            (0x120, 0x30),
            (0x160, 0x40),
            (0x170, 0x50),
        ] {
            fill(&mut e, body + offset, seed);
        }
        e.mem.set_u8(body + 0xe8, 4);
        e.mem.set_u8(body + 0x19e, 0x7f);
        e.mem.set_u8(body + 0x19c, 2);
        e.mem.set_u8(body + 0x19d, 3);
        e.mem.set_u32(body + 0x10, 0x0888_8880);
        e.mem.set_u32(body + 0x2c, 0x999);
        e.mem.set_f32(body + 0x5c, 7.0);
        e.mem.set_u8(body + 0x2a, 5);
        e.mem.set_u16(body + 0x1fe, 0x8001);
        let c = e.mem.alloc(0xc0);
        e.call(0x00ca_8f50, &args![body, c]);
        assert_eq!(e.mem.u8(c + 0xbb), 1);
        assert_eq!(e.mem.u8(c + 0xb8), 0x41);
        assert_eq!(e.mem.u8(c + 0xba), 0x42);
        assert_eq!(e.mem.u16(c + 0xe), 0x1234);
        assert_eq!(e.mem.u8(c + 0xb5), 1);
        assert_eq!(e.mem.u32(c + 8), 0x0777_7770);
        assert_eq!(e.mem.u32(c + 0xa0), 0x7fc0_0001);
        assert_eq!(e.mem.u8(c + 0xc), 0x5a);
        assert_eq!(e.mem.f32(c + 0xa4), 2.0);
        assert_eq!(e.mem.f32(c + 0x94), 3.0);
        assert_eq!(e.mem.f32(c + 0x98), 4.0);
        assert_eq!(e.mem.bytes(c + 0x30, 0x10), e.mem.bytes(body + 0x1b0, 0x10));
        assert_eq!(e.mem.bytes(c + 0x40, 0x10), e.mem.bytes(body + 0x1c0, 0x10));
        assert_eq!(e.mem.f32(c + 0x90), 12.5);
        assert_eq!(e.mem.u32(c + 0x50), 0xfeed);
        assert_eq!(e.mem.u8(c + 0xb4), 4);
        assert_eq!(e.mem.u8(c + 0xb6), 0x7f);
        assert_eq!(e.mem.f32(c + 0xa8), 0.02);
        assert_eq!(e.mem.f32(c + 0xac), 0.03);
        assert_eq!(e.mem.bytes(c + 0x10, 0x10), e.mem.bytes(body + 0x120, 0x10));
        assert_eq!(e.mem.bytes(c + 0x20, 0x10), e.mem.bytes(body + 0x160, 0x10));
        assert_eq!(e.mem.bytes(c + 0x80, 0x10), e.mem.bytes(body + 0x170, 0x10));
        assert_eq!(e.mem.u32(c + 4), 0x0888_8880);
        assert_eq!(e.mem.u32(c), 0x999);
        assert_eq!(e.mem.f32(c + 0xb0), 7.0);
        assert_eq!(e.mem.u8(c + 0xb7), 5);
        assert_eq!(e.mem.u32(c + 0x9c), 0x8001_0000);
        // The enabled flag follows the byte at +0xE9.
        e.mem.set_u8(body + 0xe9, 0xff);
        e.call(0x00ca_8f50, &args![body, c]);
        assert_eq!(e.mem.u8(c + 0xb5), 0);
    }

    /// A rig for `clone`: the constructor's doubles plus the info builder and the
    /// allocator; the source body has a fixed motion and the info it reads back.
    fn clone_rig(e: &mut Engine) -> (CallLog, Sizes, u32) {
        let (log, _, shape) = constructor_rig(e);
        let (sizes, _) = install_allocator(e);
        double_into(e, &log, &[CINFO_CONSTRUCT, STRING_ASSIGN]);
        e.register_double(MOTION_GET_MASS, |_, _| Ret {
            st0: 12.5,
            ..Ret::default()
        });
        let table = e.mem.alloc(0x80);
        e.mem.set_u32(table + 0x14, SLOT_BASE + 0x14);
        double_into(e, &log, &[SLOT_BASE + 0x14]);
        answer(e, &log, 0x0900_0310, 0x0caf_e000);
        e.mem.set_u32(RIGID_BODY_VTABLE + 0x10, 0x0900_0310);
        let source = e.mem.alloc(0x240);
        e.mem.set_u32(source + 0xe0, table);
        e.mem.set_u8(source + 0xe8, 5);
        e.mem.set_u32(source + 0x10, shape);
        e.mem.set_u32(source + 0x210, shape);
        e.mem.set_u32(source + 0x74, 0x1235);
        e.mem.set_u32(source + 0xc, 0xabcd);
        (log, sizes, source)
    }

    #[test]
    fn clone_copies_the_motion_the_extra_block_and_the_properties() {
        let mut e = test_engine();
        let (log, sizes, source) = clone_rig(&mut e);
        let extra = e.mem.alloc(0x120);
        for i in 0..0x120u32 {
            e.mem.set_u8(extra + i, (i % 251) as u8 + 1);
        }
        e.mem.set_u32(source + 0x1f8, extra);
        let property = e.mem.alloc(0x20);
        let property_table = e.mem.alloc(0x20);
        e.mem.set_u32(property, property_table);
        e.mem.set_u32(property_table + 0xc, 0x0900_0320);
        answer(&mut e, &log, 0x0900_0320, 0x7777);
        e.mem.set_u32(source + 0xa0, property);
        let new = e.call(0x00ca_90c0, &args![source]).u32();
        assert_eq!(*sizes.borrow(), vec![0x220, 0x120]);
        assert_eq!(e.mem.u16(new + 4), 0x220);
        assert_eq!(e.mem.u32(new), RIGID_BODY_VTABLE);
        assert_eq!(calls(&log, CINFO_CONSTRUCT).len(), 1);
        assert_eq!(
            e.mem.bytes(new + 0xe0, 0x18),
            e.mem.bytes(source + 0xe0, 0x18)
        );
        let copy = e.mem.u32(new + 0x1f8);
        assert_ne!(copy, extra);
        assert_eq!(e.mem.bytes(copy, 0x120), e.mem.bytes(extra, 0x120));
        assert_eq!(e.mem.u32(new + 0x18), 0x0caf_e000);
        assert_eq!(calls(&log, STRING_ASSIGN), vec![vec![new + 0x74, 0x1234]]);
        assert_eq!(e.mem.u32(new + 0xc), 0xabcd);
        assert_eq!(calls(&log, 0x0900_0320), vec![vec![property, new]]);
        assert_eq!(e.mem.u32(new + 0xa0), 0x7777);
    }

    #[test]
    fn clone_without_extra_block_or_property_skips_both() {
        let mut e = test_engine();
        let (log, sizes, source) = clone_rig(&mut e);
        let new = e.call(0x00ca_90c0, &args![source]).u32();
        assert_eq!(*sizes.borrow(), vec![0x220]);
        assert_eq!(e.mem.u32(new + 0x1f8), 0);
        assert_eq!(e.mem.u32(new + 0xa0), 0);
        assert_eq!(calls(&log, STRING_ASSIGN), vec![vec![new + 0x74, 0x1234]]);
    }

    /// An object whose property array (+0x78 data, +0x7c size, +0x80 capacity) holds
    /// `elements` in a block of `capacity` elements (no block for 0).
    fn property_holder(e: &mut Engine, elements: &[[u32; 4]], capacity: u32) -> u32 {
        let holder = e.mem.alloc(0x100);
        let data = if capacity == 0 {
            0
        } else {
            e.mem.alloc(capacity * 16)
        };
        for (i, element) in (0u32..).zip(elements) {
            for (j, word) in (0u32..).zip(element) {
                e.mem.set_u32(data + 16 * i + 4 * j, *word);
            }
        }
        e.mem.set_u32(holder + 0x78, data);
        e.mem.set_u32(holder + 0x7c, elements.len() as u32);
        e.mem.set_u32(holder + 0x80, capacity | 0x8000_0000);
        holder
    }

    fn property_words(e: &Engine, holder: u32, count: u32) -> Vec<u32> {
        let data = e.mem.u32(holder + 0x78);
        (0..count * 4).map(|i| e.mem.u32(data + 4 * i)).collect()
    }

    #[test]
    fn copy_properties_grows_only_when_the_capacity_is_too_small() {
        let mut e = test_engine();
        let (_, allocator) = install_allocator(&mut e);
        let log: CallLog = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            sink.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            let old = e.mem.u32(a[1]);
            let size = e.mem.u32(a[1] + 4);
            let block = e.mem.alloc(a[2] * 16);
            let bytes = e.mem.bytes(old, size * 16);
            e.mem.write(block, &bytes);
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let source: Vec<[u32; 4]> = (1..=5u32).map(|i| [i, i + 10, i + 20, i + 30]).collect();
        let flat = |count: usize| -> Vec<u32> { source[..count].concat() };
        // Capacity 4 holds 3 elements: the one kept element and two more are copied.
        let other = property_holder(&mut e, &source[..3], 3);
        let this = property_holder(&mut e, &[[9, 9, 9, 9]], 4);
        e.call(0x00ca_92a0, &args![this, other]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(this + 0x7c), 3);
        assert_eq!(property_words(&e, this, 3), flat(3));
        // Capacity 2 and 5 wanted: grown to 5 (the other's size beats twice the capacity).
        let other = property_holder(&mut e, &source, 5);
        let this = property_holder(&mut e, &[], 2);
        e.call(0x00ca_92a0, &args![this, other]);
        assert_eq!(
            calls(&log, ARRAY_RESERVE),
            vec![vec![allocator, this + 0x78, 5, 0x10]]
        );
        assert_eq!(e.mem.u32(this + 0x7c), 5);
        assert_eq!(property_words(&e, this, 5), flat(5));
        // Capacity 2 and 3 wanted: twice the capacity (4) wins.
        log.borrow_mut().clear();
        let other = property_holder(&mut e, &source[..3], 3);
        let this = property_holder(&mut e, &[], 2);
        e.call(0x00ca_92a0, &args![this, other]);
        assert_eq!(
            calls(&log, ARRAY_RESERVE),
            vec![vec![allocator, this + 0x78, 4, 0x10]]
        );
        assert_eq!(property_words(&e, this, 3), flat(3));
        // A destination without a data block is skipped for the elements past the
        // kept ones.
        log.borrow_mut().clear();
        let other = property_holder(&mut e, &source[..1], 1);
        let this = property_holder(&mut e, &[], 1);
        e.mem.set_u32(this + 0x78, 0);
        e.call(0x00ca_92a0, &args![this, other]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(this + 0x78), 0);
        assert_eq!(e.mem.u32(this + 0x7c), 1);
    }
}
