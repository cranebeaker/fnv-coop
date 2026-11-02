//! `Havok SDK/hkpvehiclelinearcastwheelcollide.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The PDB has no vehicle types: the layouts and their names are taken from what the code reads
//! (the names follow the Havok SDK's `hkpVehicleLinearCastWheelCollide`, unconfirmed).
//!
//! The wheel collider keeps one 0x60-byte wheel state per wheel in the array at `+0x10` (data
//! `+0x10`, size `+0x14`, capacity and flags `+0x18`): the wheel's `hkpAabbPhantom` (`+0`), its
//! cylinder shape (`+4`), the wheel's transform (`+0x10`, a rotation of three columns and the
//! translation at `+0x40`) and the point the cast goes to (`+0x50`). A listener object sits at
//! `+0x1C` (`hkpRejectChassisListener`; its overlap-listener base is at `+0x24`, the address
//! every function that registers it passes) and holds the chassis collidable at `+0x28`.
//!
//! The functions that take a wheel number as a stack word read only its low byte (`MOVZX`).
//! Where the exe builds that word from a one-byte counter in a stack slot, the three upper bytes
//! are stale stack contents the callees never read; here they are zero.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `hkpVehicleLinearCastWheelCollide` (names unconfirmed).
    pub struct HkpVehicleLinearCastWheelCollide: 0x34 {
        /// `m_collisionFilterInfo`, given to every phantom.
        0x0c m_collision_filter_info: u32,
        /// The wheel states' data pointer.
        0x10 m_wheel_states: Ptr,
        /// The number of wheel states.
        0x14 m_wheel_states_size: i32,
        /// The array's capacity (low 30 bits) and flags (bit 31: the storage is not owned).
        0x18 m_wheel_states_capacity_and_flags: u32,
        /// The chassis collidable the listener rejects (`m_chassis` of `hkpRejectChassisListener`).
        0x28 m_chassis: Ptr,
        /// `m_maxExtraPenetration` of the linear cast input.
        0x2c m_max_extra_penetration: f32,
        /// `m_startPointTolerance` of the linear cast input.
        0x30 m_start_point_tolerance: f32,
    }
}

/// The memory router (`00c85750`, no arguments): its field `+0x10` is the heap allocator
/// (slot `4` allocates, slot `0x10` frees an array).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkpCylinderShape` constructor (`this` = the block, then the two end points, the radius and
/// the convex radius).
const CYLINDER_SHAPE_CONSTRUCTOR: u32 = 0x00d5_7aa0;
/// `hkpCollidable::BoundingVolumeData` constructor (`this` only).
const BOUNDING_VOLUME_DATA_CONSTRUCTOR: u32 = 0x00d0_1a60;
/// `hkQuaternion::setMul` (`this` = the result, then the two factors).
const QUATERNION_SET_MUL: u32 = 0x00c6_6320;
/// `hkRotation::set` (`this` = the rotation, then the quaternion).
const ROTATION_SET: u32 = 0x00cb_2d90;
/// Velocity of a body at a point (`this` = the body, then the point and the result).
const GET_POINT_VELOCITY: u32 = 0x00ca_0c40;
/// `hkAabbUtil::calcAabb` (cdecl: transform, two vectors, a float, the result).
const CALC_AABB: u32 = 0x00d2_6230;
/// `hkpWorld::addPhantom` and `removePhantom` (`this` = the world, then the phantom).
const ADD_PHANTOM: u32 = 0x00c9_36a0;
const REMOVE_PHANTOM: u32 = 0x00c9_2c70;
/// `hkVector4::setTransformedInversePos` and `setTransformedPos` (`this` = the result, then the
/// transform and the point).
const SET_TRANSFORMED_INVERSE_POS: u32 = 0x00c7_f690;
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
/// `hkpCollidable` constructor (`this`, then the shape, the transform and a zero).
const COLLIDABLE_CONSTRUCTOR: u32 = 0x00cd_2d80;
/// `hkpAabbPhantom::linearCast` (`this` = the phantom, then the collidable, the input, the
/// collector and the start-point collector).
const LINEAR_CAST: u32 = 0x00cf_2880;
/// `hkpRootCdPoint::operator=` (`this` = the destination, then the source).
const ROOT_CD_POINT_ASSIGN: u32 = 0x0062_12c0;
/// `hkpAabbPhantom::setAabb` (`this` = the phantom, then the box).
const SET_AABB: u32 = 0x00cf_24a0;
/// `hkpAabbPhantom` constructor (`this` = the block, then the box and the filter info).
const AABB_PHANTOM_CONSTRUCTOR: u32 = 0x00cf_2cb0;
/// `hkReferencedObject` add and release reference (`this` only).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkpPhantom::addPhantomOverlapListener` and `removePhantomOverlapListener` (`this` = the
/// phantom, then the listener).
const ADD_OVERLAP_LISTENER: u32 = 0x00d0_7de0;
const REMOVE_OVERLAP_LISTENER: u32 = 0x00d0_7c80;
/// `hkpRejectChassisListener` destructor (`this` = the listener).
const LISTENER_DESTRUCTOR: u32 = 0x00e3_60f0;
/// `hkArrayUtil::_reserve` (cdecl: allocator, array, capacity, element size) and the
/// grow-by-one helper (cdecl: allocator, array, element size).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;

/// This unit's own functions, which call one another by address.
const CREATE_WHEEL_SHAPE: u32 = 0x00e3_6120;
const CALC_AABB_OF_WHEEL: u32 = 0x00e3_6700;
const UPDATE_WHEEL_STATE: u32 = 0x00e3_67d0;
const CENTER_WHEEL_CONTACT_POINT: u32 = 0x00e3_6900;
const CONSTRUCT: u32 = 0x00e3_6b20;

/// The vtable of the class (`010dec3c`), the vtables of its listener's two parts (`010debf4` at
/// `+0x1C`, `010debe4` at `+0x24`; the constructor first stores `010c925c` at `+0x24`), the base
/// object's vtable the destructor leaves (`0102e388`) and the vtable of the linear cast's
/// collector (`010c476c`).
const VTABLE: u32 = 0x010d_ec3c;
const LISTENER_VTABLE: u32 = 0x010d_ebf4;
const LISTENER_BASE_VTABLE_FIRST: u32 = 0x010c_925c;
const LISTENER_BASE_VTABLE: u32 = 0x010d_ebe4;
const BASE_VTABLE: u32 = 0x0102_e388;
const COLLECTOR_VTABLE: u32 = 0x010c_476c;

/// Sign masks: lanes 0 to 2 (`010c4c00`, lane 3 clear) and all four lanes (`010c44d0`).
const SIGN_MASK_XYZ: u32 = 0x010c_4c00;
const SIGN_MASK_ALL: u32 = 0x010c_44d0;
/// Floats the exe copies from its data: `0104da4c` (initial `m_maxExtraPenetration` and
/// `m_startPointTolerance`), `0104da48` (the collector's initial distance) and `01012054`
/// (`-1.0`, stored in every collidable record).
const INITIAL_TOLERANCE: u32 = 0x0104_da4c;
const COLLECTOR_DISTANCE: u32 = 0x0104_da48;
const COLLIDABLE_MINUS_ONE: u32 = 0x0101_2054;

/// The double `0.5` at `01011588`, the double `-1.0` at `0101a6b0` and the float `-0.0` at
/// `010c4990`.
const HALF: f64 = 0.5;
const MINUS_ONE: f64 = -1.0;
const MINUS_ZERO: f32 = -0.0;

const STATE_SIZE: u32 = 0x60;
const WHEEL_INFO_SIZE: u32 = 0xe0;
const WHEEL_PARAMS_SIZE: u32 = 0x28;
const SUSPENSION_SIZE: u32 = 0x30;

/// Copies a four-lane vector (`MOVAPS`).
fn copy_vector(e: &mut Engine, from: u32, to: u32) {
    for i in 0..4 {
        let word = e.mem.u32(from.wrapping_add(4 * i));
        e.mem.set_u32(to.wrapping_add(4 * i), word);
    }
}

/// The four lanes at `at`.
fn lanes(e: &Engine, at: u32) -> [f32; 4] {
    std::array::from_fn(|i| e.mem.f32(at.wrapping_add(4 * i as u32)))
}

/// The sum of lanes 0 to 2 the way the SSE code adds them: `z + (y + x)`.
fn sum_xyz(values: [f32; 4]) -> f32 {
    values[2] + (values[1] + values[0])
}

/// The exe's pointer test before a listener argument: `this + 0x1C` is null only for a null
/// `this`, which gives a null listener; otherwise the listener base at `this + 0x24`.
fn overlap_listener(this: u32) -> u32 {
    if this.wrapping_add(0x1c) == 0 {
        0
    } else {
        this.wrapping_add(0x24)
    }
}

/// Allocates a block of `size` bytes through the router's allocator (slot `4`) and stores the
/// 16-bit size at `+4`.
fn allocate_object(e: &mut Engine, size: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![size]).u32();
    e.mem.set_u16(block + 4, size as u16);
    block
}

// Translated from 00e36120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleLinearCastWheelCollide::createWheelShape` (Xbox PDB), `(this, width, radius)`;
/// `this` is not read. With `h` = `float(width * 0.5)`, the shape is an `hkpCylinderShape` of
/// `radius` along the z axis, from `(0, 0, h, 0)` to the end point with the signs of lanes 0 to
/// 2 flipped (the mask at `010c4c00`; lane 3 is not flipped), in a 0x60-byte block of the
/// router's allocator with its size stored. The convex radius is `0.0`. Returns what the
/// constructor returns (the block).
pub fn create_wheel_shape(
    e: &mut Engine,
    _this: Ptr<HkpVehicleLinearCastWheelCollide>,
    width: f32,
    radius: f32,
) -> u32 {
    let half = (f64::from(x87_quiet(width)) * HALF) as f32;
    let end_a = [0.0f32, 0.0, half, 0.0].map(f32::to_bits);
    let mask: [u32; 4] = std::array::from_fn(|i| e.mem.u32(SIGN_MASK_XYZ + 4 * i as u32));
    let end_b: [u32; 4] = std::array::from_fn(|i| mask[i] ^ end_a[i]);
    let block = allocate_object(e, 0x60);
    e.with_stack(0x20, |e, vectors| {
        let a = vectors.addr();
        let b = a + 0x10;
        for (i, (first, second)) in end_a.iter().zip(end_b.iter()).enumerate() {
            e.mem.set_u32(a + 4 * i as u32, *first);
            e.mem.set_u32(b + 4 * i as u32, *second);
        }
        e.call(
            CYLINDER_SHAPE_CONSTRUCTOR,
            &args![block, a, b, x87_quiet(radius), 0.0f32],
        )
        .u32()
    })
}

// Translated from 00e361b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleLinearCastWheelCollide::collideWheels` (Xbox PDB), `(this, _, instance,
/// records)`; the first stack word is not read. `instance` is the vehicle; its wheel count is
/// the byte at `[instance + 0x1C] + 0x20`. For every wheel `j` in turn (`records` moves on by
/// 0x60): `j_word` = `j`; slot `0x40` `(flag slot, instance, j_word, cast record)` answers the
/// address of a byte (it is the byte of the first argument); when the byte is not zero, slot
/// `0x44` `(instance, j_word, cast record, records_j)` runs, otherwise slot `0x48` `(instance,
/// j_word, records_j)`; then slot `0x2C` `(instance, j_word, records_j)`. The flag slot (one
/// byte) and the 0x64-byte cast record are locals.
pub fn collide_wheels(
    e: &mut Engine,
    this: Ptr<HkpVehicleLinearCastWheelCollide>,
    _unused_1: u32,
    instance: Ptr,
    records: u32,
) {
    let data = e.mem.u32(instance.addr() + 0x1c);
    let count = e.mem.u8(data + 0x20);
    if count == 0 {
        return;
    }
    let mut record = records;
    let mut wheel = 0u8;
    e.with_stack(0x70, |e, frame| {
        let flag_slot = frame.addr();
        let cast_record = frame.addr() + 0x10;
        loop {
            let wheel_word = u32::from(wheel);
            let flag = e
                .vcall(
                    this.addr(),
                    0x40,
                    &args![flag_slot, instance, wheel_word, cast_record],
                )
                .u32();
            if e.mem.u8(flag) != 0 {
                e.vcall(
                    this.addr(),
                    0x44,
                    &args![instance, wheel_word, cast_record, record],
                );
            } else {
                e.vcall(this.addr(), 0x48, &args![instance, wheel_word, record]);
            }
            e.vcall(this.addr(), 0x2c, &args![instance, wheel_word, record]);
            wheel = wheel.wrapping_add(1);
            record = record.wrapping_add(0x60);
            if wheel >= count {
                break;
            }
        }
    });
}

// Translated from 00e36250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleLinearCastWheelCollide::determineNearestHit` (Xbox PDB), `(this, wheel word,
/// entries)`. The number of entries is slot `0x34` `(wheel word)` (signed); entry `j` is 0x40
/// bytes at `entries + 0x28 + j * 0x40`. An entry whose word at `+8` is not zero offers the hit
/// whose address is its word at `+0`; the hit with the smallest float at `+0x1C` wins, the
/// first one on a tie (a candidate replaces the best only when its float is strictly smaller,
/// so a NaN never replaces and the first hit stays even when it is a NaN). Returns the winning
/// hit, `0` for none. (The exe unrolls the loop four times; the order of the comparisons is the
/// same.)
pub fn determine_nearest_hit(
    e: &mut Engine,
    this: Ptr<HkpVehicleLinearCastWheelCollide>,
    wheel_word: u32,
    entries: Ptr,
) -> u32 {
    let count = e.vcall(this.addr(), 0x34, &args![wheel_word]).i32();
    let mut best = 0u32;
    for index in 0..count {
        let entry = entries
            .addr()
            .wrapping_add(0x28)
            .wrapping_add((index as u32).wrapping_mul(0x40));
        if e.mem.u32(entry + 8) == 0 {
            continue;
        }
        let candidate = e.mem.u32(entry);
        if best == 0 {
            best = candidate;
        } else {
            let candidate_fraction = e.mem.f32(candidate.wrapping_add(0x1c));
            let best_fraction = e.mem.f32(best.wrapping_add(0x1c));
            if candidate_fraction < best_fraction {
                best = candidate;
            }
        }
    }
    best
}

// Translated from 00e36360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleLinearCastWheelCollide::getTotalNumCommands` (Xbox PDB): the sum, as a signed
/// word, of slot `0x34` `(wheel word)` over the wheels. The wheel word is the low byte of `this`
/// replaced by the wheel number (the exe keeps the one-byte counter in a stack slot that held
/// `this`), the counter is a byte compared as unsigned against the signed wheel count.
pub fn get_total_num_commands(e: &mut Engine, this: Ptr<HkpVehicleLinearCastWheelCollide>) -> i32 {
    let count = e.get(this, HkpVehicleLinearCastWheelCollide::m_wheel_states_size);
    let mut total = 0i32;
    let mut wheel = 0u8;
    if count > 0 {
        loop {
            let wheel_word = (this.addr() & 0xffff_ff00) | u32::from(wheel);
            let commands = e.vcall(this.addr(), 0x34, &args![wheel_word]).i32();
            wheel = wheel.wrapping_add(1);
            total = total.wrapping_add(commands);
            if i32::from(wheel) >= count {
                break;
            }
        }
    }
    total
}

// Translated from 00e363a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleLinearCastWheelCollide::getNumCommands` (Xbox PDB), `(this, wheel word)`: runs
/// slot `0x28` of the wheel's phantom (no arguments; unconfirmed what it does) and returns the
/// word at `phantom + 0xD4`, the size of the phantom's array of overlapping collidables (the
/// array's data pointer is at `+0xD0`).
pub fn get_num_commands(
    e: &mut Engine,
    this: Ptr<HkpVehicleLinearCastWheelCollide>,
    wheel_word: u32,
) -> i32 {
    let wheel = wheel_word & 0xff;
    let states = e
        .get(this, HkpVehicleLinearCastWheelCollide::m_wheel_states)
        .addr();
    let phantom = e
        .mem
        .u32(states.wrapping_add(wheel.wrapping_mul(STATE_SIZE)));
    e.vcall(phantom, 0x28, &args![]);
    e.mem.i32(phantom + 0xd4)
}

// Translated from 00e363d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleLinearCastWheelCollide::buildLinearCastCommands` (Xbox PDB), `(this, vehicle,
/// simulation, records, inputs, outputs)`; the two words after `this` are not read. `records`
/// holds one 0x50-byte collidable record per wheel, `inputs` one 0x40-byte command per cast and
/// `outputs` one 0x70-byte result per cast. Returns the number of casts. For every wheel `j`
/// (a signed byte counter, wheel states at `[this + 0x10] + j * 0x60`):
/// 1. unless `records` is null the record is built: the wheel's shape at `+0`, `0xFFFFFFFF` at
///    `+4`, the address of the wheel's transform (`state + 0x10`) at `+8`, `0xFF` at `+0x1A`
///    and zero bytes elsewhere up to `+0x1C`, the `BoundingVolumeData` constructor on `+0x20`,
///    the float at `01012054` (`-1.0`) at `+0x4C`, then the byte `0xEC` at `+0x19` (the low byte
///    of the record minus that of `record + 0x14`) and zeros at `+0x11` to `+0x13`;
/// 2. slot `0x28` of the phantom runs (no arguments); the phantom's overlapping collidables are
///    the array at `phantom + 0xD0` (size at `+0xD4`, read once). For every collidable `k` (a
///    signed byte counter): `inputs + 0x20` = the record, `+0x24` = the collidable, `+0` =
///    the vector at `state + 0x40`, `+0x28` = `outputs`, `+0x10` = the vector at `state +
///    0x50`, `+0x2C` = 1, `+0x30` and `+0x38` = 0; `inputs` moves on by 0x40, `outputs` by
///    0x70 and the cast count goes up;
/// 3. `records` moves on by 0x50.
pub fn build_linear_cast_commands(
    e: &mut Engine,
    this: Ptr<HkpVehicleLinearCastWheelCollide>,
    _unused_1: u32,
    _unused_2: u32,
    records: u32,
    inputs: u32,
    outputs: u32,
) -> i32 {
    let count = e.get(this, HkpVehicleLinearCastWheelCollide::m_wheel_states_size);
    let (mut record, mut input, mut output) = (records, inputs, outputs);
    let mut total = 0i32;
    let mut wheel = 0i8;
    if count <= 0 {
        return 0;
    }
    loop {
        let states = e
            .get(this, HkpVehicleLinearCastWheelCollide::m_wheel_states)
            .addr();
        let state = states.wrapping_add((i32::from(wheel) as u32).wrapping_mul(STATE_SIZE));
        if record != 0 {
            let shape = e.mem.u32(state + 4);
            e.mem.set_u32(record, shape);
            e.mem.set_u32(record + 0xc, 0);
            e.mem.set_u8(record + 0x10, 0);
            e.mem.set_u32(record + 4, 0xffff_ffff);
            e.mem.set_u32(record + 8, state + 0x10);
            e.mem.set_u32(record + 0x14, 0);
            e.mem.set_u8(record + 0x18, 0);
            e.mem.set_u32(record + 0x1c, 0);
            e.mem.set_u8(record + 0x1a, 0xff);
            e.mem.set_u8(record + 0x19, 0x7f);
            e.call(BOUNDING_VOLUME_DATA_CONSTRUCTOR, &args![record + 0x20]);
            let minus_one = x87_quiet_bits(e.mem.u32(COLLIDABLE_MINUS_ONE));
            e.mem.set_u32(record + 0x4c, minus_one);
            e.mem.set_u8(
                record + 0x19,
                (record as u8).wrapping_sub(record.wrapping_add(0x14) as u8),
            );
            e.mem.set_u8(record + 0x11, 0);
            e.mem.set_u16(record + 0x12, 0);
        }
        let phantom = e.mem.u32(state);
        e.vcall(phantom, 0x28, &args![]);
        let phantom = e.mem.u32(state);
        let array = phantom.wrapping_add(0xd0);
        let overlaps = e.mem.i32(array + 4);
        let mut overlap = 0i8;
        if overlaps > 0 {
            loop {
                total = total.wrapping_add(1);
                e.mem.set_u32(input + 0x20, record);
                let data = e.mem.u32(array);
                let collidable = e
                    .mem
                    .u32(data.wrapping_add((i32::from(overlap) as u32).wrapping_mul(4)));
                e.mem.set_u32(input + 0x24, collidable);
                copy_vector(e, state + 0x40, input);
                e.mem.set_u32(input + 0x28, output);
                output = output.wrapping_add(0x70);
                overlap = overlap.wrapping_add(1);
                copy_vector(e, state + 0x50, input + 0x10);
                e.mem.set_u32(input + 0x2c, 1);
                e.mem.set_u32(input + 0x30, 0);
                e.mem.set_u32(input + 0x38, 0);
                input = input.wrapping_add(0x40);
                if i32::from(overlap) >= overlaps {
                    break;
                }
            }
        }
        record = record.wrapping_add(0x50);
        wheel = wheel.wrapping_add(1);
        if i32::from(wheel) >= count {
            break;
        }
    }
    total
}

// Translated from 00e36510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleLinearCastWheelCollide::getCollisionOutputFromCastResult` (Xbox PDB), `(this,
/// instance, wheel word, cast, out)`; `this` is not read. `info` = the wheel's record at
/// `[instance + 0x48] + wheel * 0xE0`, `f` = the float at `[[instance + 0x34] + 8] + wheel *
/// 0x30 + 0x20`, `stiffness` = the float at `[[instance + 0x1C]] + 0x84`. The first 0x20 bytes
/// of `cast` are copied to `out`; `body` = the word at `cast + 0x48`: `other` = `body` plus its
/// signed byte at `+0x10` when the byte at `body + 0x18` is `1`, else `0` (the exe then reads
/// the float at address `0x90` and faults); `out + 0x24` = `other`, `out + 0x20` = the float at
/// `other + 0x90`, `out + 0x28` = the word at `cast + 0x4C`, `out + 0x2C` = `0xFFFFFFFF`,
/// `out + 0x48` = `float(cast.f1C * f)`. With `n` = the vector at `out + 0x10` and `a` = the
/// vector at `info + 0x80`, `d = z + (y + x)` of their lane products:
/// - when `d < -stiffness` (false for a NaN): the velocities of the chassis body (`[instance +
///   0x18]`) and of `other` at the point `out` are fetched (`00ca0c40`, results in two stack
///   vectors `va`, `vb`), `r = float(-1.0 / d)`, `out + 0x4C = float(float(z + (y + x) of
///   (va - vb) * n) * r)` and `out + 0x50 = r`;
/// - otherwise `out + 0x4C = 0.0` and `out + 0x50 = float(1.0 / stiffness)`.
pub fn get_collision_output_from_cast_result(
    e: &mut Engine,
    _this: Ptr<HkpVehicleLinearCastWheelCollide>,
    instance: Ptr,
    wheel_word: u32,
    cast: Ptr,
    out: Ptr,
) {
    let wheel = wheel_word & 0xff;
    let instance_at = instance.addr();
    let (cast_at, out_at) = (cast.addr(), out.addr());
    let info = e
        .mem
        .u32(instance_at + 0x48)
        .wrapping_add(wheel.wrapping_mul(WHEEL_INFO_SIZE));
    let suspension = e.mem.u32(e.mem.u32(instance_at + 0x34) + 8);
    let f = x87_quiet(
        e.mem.f32(
            suspension
                .wrapping_add(wheel.wrapping_mul(SUSPENSION_SIZE))
                .wrapping_add(0x20),
        ),
    );
    copy_vector(e, cast_at, out_at);
    copy_vector(e, cast_at + 0x10, out_at + 0x10);
    let body = e.mem.u32(cast_at + 0x48);
    let other = if e.mem.u8(body + 0x18) == 1 {
        body.wrapping_add(i32::from(e.mem.i8(body + 0x10)) as u32)
    } else {
        0
    };
    e.mem.set_u32(out_at + 0x24, other);
    let velocity_word = x87_quiet_bits(e.mem.u32(other.wrapping_add(0x90)));
    e.mem.set_u32(out_at + 0x20, velocity_word);
    let hit_body = e.mem.u32(cast_at + 0x4c);
    e.mem.set_u32(out_at + 0x28, hit_body);
    e.mem.set_u32(out_at + 0x2c, 0xffff_ffff);
    let fraction = x87_quiet(e.mem.f32(cast_at + 0x1c));
    e.mem
        .set_f32(out_at + 0x48, (f64::from(fraction) * f64::from(f)) as f32);
    let axis = lanes(e, info + 0x80);
    let normal = lanes(e, out_at + 0x10);
    let d = sum_xyz(std::array::from_fn(|i| axis[i] * normal[i]));
    let data = e.mem.u32(instance_at + 0x1c);
    let limit = -x87_quiet(e.mem.f32(data + 0x84));
    if d < limit {
        let chassis = e.mem.u32(instance_at + 0x18);
        let (va, vb) = e.with_stack(0x20, |e, vectors| {
            let a = vectors.addr();
            e.call(GET_POINT_VELOCITY, &args![chassis, out, a]);
            e.call(GET_POINT_VELOCITY, &args![other, out, a + 0x10]);
            (lanes(e, a), lanes(e, a + 0x10))
        });
        let r = (MINUS_ONE / f64::from(d)) as f32;
        let normal = lanes(e, out_at + 0x10);
        let relative = sum_xyz(std::array::from_fn(|i| (va[i] - vb[i]) * normal[i]));
        e.mem
            .set_f32(out_at + 0x4c, (f64::from(relative) * f64::from(r)) as f32);
        e.mem.set_f32(out_at + 0x50, r);
    } else {
        e.mem.set_f32(out_at + 0x4c, 0.0);
        let data = e.mem.u32(instance_at + 0x1c);
        let stiffness = x87_quiet(e.mem.f32(data + 0x84));
        e.mem
            .set_f32(out_at + 0x50, (1.0 / f64::from(stiffness)) as f32);
    }
}

// Translated from 00e36670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleLinearCastWheelCollide::getCollisionOutputWithoutHit` (Xbox PDB), `(this, instance,
/// wheel word, out)`; `this` is not read. With `info` and `f` as in
/// `getCollisionOutputFromCastResult` (`00e36510`): `out + 0x24` = 0, `out + 0x48` = `f`,
/// `out + 0x4C` = 0.0, the vector at `info + 0x60` is copied to `out`, lanes 0 to 2 of `out +
/// 0x10` = the sign-flipped lanes of the vector at `info + 0x80` (mask `010c44d0`; the fourth
/// lane is written back unchanged and then replaced), `out + 0x20` = 0.0, `out + 0x50` = 1.0
/// and `out + 0x1C` = `f` (every float copy quiets a signalling NaN).
pub fn get_collision_output_without_hit(
    e: &mut Engine,
    _this: Ptr<HkpVehicleLinearCastWheelCollide>,
    instance: Ptr,
    wheel_word: u32,
    out: Ptr,
) {
    let wheel = wheel_word & 0xff;
    let instance_at = instance.addr();
    let out_at = out.addr();
    let info = e
        .mem
        .u32(instance_at + 0x48)
        .wrapping_add(wheel.wrapping_mul(WHEEL_INFO_SIZE));
    let suspension = e.mem.u32(e.mem.u32(instance_at + 0x34) + 8);
    let f_bits = x87_quiet_bits(
        e.mem.u32(
            suspension
                .wrapping_add(wheel.wrapping_mul(SUSPENSION_SIZE))
                .wrapping_add(0x20),
        ),
    );
    e.mem.set_u32(out_at + 0x24, 0);
    e.mem.set_u32(out_at + 0x48, f_bits);
    e.mem.set_f32(out_at + 0x4c, 0.0);
    copy_vector(e, info + 0x60, out_at);
    for lane in 0..3 {
        let mask = e.mem.u32(SIGN_MASK_ALL + 4 * lane);
        let word = e.mem.u32(info + 0x80 + 4 * lane);
        e.mem.set_u32(out_at + 0x10 + 4 * lane, word ^ mask);
    }
    e.mem.set_f32(out_at + 0x20, 0.0);
    e.mem.set_f32(out_at + 0x50, 1.0);
    e.mem.set_u32(out_at + 0x1c, f_bits);
}

// Translated from 00e36700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleLinearCastWheelCollide::calcAabbOfWheel` (Xbox PDB), `(this, instance, wheel word,
/// aabb)`. `p` = the wheel's parameters at `[[instance + 0x1C] + 0x8C] + wheel * 0x28` (radius
/// at `+0`, width at `+8`), `f` as in `getCollisionOutputFromCastResult` (`00e36510`). With
/// `h1 = float((f + radius) * 0.5)` and `h2 = float(0.5 * width)` the box is calculated
/// (`00d26230`, cdecl) from the wheel's transform (`[this + 0x10] + wheel * 0x60 + 0x10`), the
/// vectors `(radius bits, h1, h2, 0.0)` and `(0.0, float(-0.0 - h1), 0.0, 0.0)`, `0.0` and
/// the `aabb` pointer.
pub fn calc_aabb_of_wheel(
    e: &mut Engine,
    this: Ptr<HkpVehicleLinearCastWheelCollide>,
    instance: Ptr,
    wheel_word: u32,
    aabb: Ptr,
) {
    let wheel = wheel_word & 0xff;
    let instance_at = instance.addr();
    let data = e.mem.u32(instance_at + 0x1c);
    let params = e
        .mem
        .u32(data + 0x8c)
        .wrapping_add(wheel.wrapping_mul(WHEEL_PARAMS_SIZE));
    let suspension = e.mem.u32(e.mem.u32(instance_at + 0x34) + 8);
    let f = x87_quiet(
        e.mem.f32(
            suspension
                .wrapping_add(wheel.wrapping_mul(SUSPENSION_SIZE))
                .wrapping_add(0x20),
        ),
    );
    let radius_bits = e.mem.u32(params);
    let radius = x87_quiet(f32::from_bits(radius_bits));
    let h1 = ((f64::from(f) + f64::from(radius)) * HALF) as f32;
    let width = x87_quiet(e.mem.f32(params + 8));
    let h2 = (HALF * f64::from(width)) as f32;
    let states = e
        .get(this, HkpVehicleLinearCastWheelCollide::m_wheel_states)
        .addr();
    let transform = states
        .wrapping_add(wheel.wrapping_mul(STATE_SIZE))
        .wrapping_add(0x10);
    let first = [radius_bits, h1.to_bits(), h2.to_bits(), 0.0f32.to_bits()];
    let second = [
        0.0f32.to_bits(),
        (MINUS_ZERO - h1).to_bits(),
        0.0f32.to_bits(),
        0.0f32.to_bits(),
    ];
    e.with_stack(0x20, |e, vectors| {
        let a = vectors.addr();
        for (i, (lane_a, lane_b)) in first.iter().zip(second.iter()).enumerate() {
            e.mem.set_u32(a + 4 * i as u32, *lane_a);
            e.mem.set_u32(a + 0x10 + 4 * i as u32, *lane_b);
        }
        e.call(CALC_AABB, &args![transform, a, a + 0x10, 0.0f32, aabb]);
    });
}

// Translated from 00e367d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleLinearCastWheelCollide::updateWheelState` (Xbox PDB), `(this, instance, wheel
/// word)`. `info` and `f` as in `getCollisionOutputFromCastResult` (`00e36510`), `state` = the
/// wheel's state. A quaternion is the product (`00c66320`) of the chassis body's at
/// `[instance + 0x18] + 0x160` and the one at `info + 0xB0`; it sets the rotation at `state +
/// 0x10` (`00cb2d90`). The vector at `info + 0x50` is copied to `state + 0x40`, and `state +
/// 0x50` = `f * (info + 0x80) + (info + 0x50)` lane by lane in `float` arithmetic.
pub fn update_wheel_state(
    e: &mut Engine,
    this: Ptr<HkpVehicleLinearCastWheelCollide>,
    instance: Ptr,
    wheel_word: u32,
) {
    let wheel = wheel_word & 0xff;
    let instance_at = instance.addr();
    let info = e
        .mem
        .u32(instance_at + 0x48)
        .wrapping_add(wheel.wrapping_mul(WHEEL_INFO_SIZE));
    let states = e
        .get(this, HkpVehicleLinearCastWheelCollide::m_wheel_states)
        .addr();
    let state = states.wrapping_add(wheel.wrapping_mul(STATE_SIZE));
    let body = e.mem.u32(instance_at + 0x18);
    e.with_stack(0x10, |e, quaternion| {
        e.call(
            QUATERNION_SET_MUL,
            &args![quaternion, body + 0x160, info + 0xb0],
        );
        e.call(ROTATION_SET, &args![state + 0x10, quaternion]);
    });
    copy_vector(e, info + 0x50, state + 0x40);
    let suspension = e.mem.u32(e.mem.u32(instance_at + 0x34) + 8);
    let f = e.mem.f32(
        suspension
            .wrapping_add(wheel.wrapping_mul(SUSPENSION_SIZE))
            .wrapping_add(0x20),
    );
    let direction = lanes(e, info + 0x80);
    let origin = lanes(e, info + 0x50);
    for (i, lane) in direction.iter().enumerate() {
        let value = f * lane + origin[i];
        e.mem.set_f32(state + 0x50 + 4 * i as u32, value);
    }
}

// Translated from 00e36870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleLinearCastWheelCollide::addToWorld` (Xbox PDB), `(this, world)`: adds every
/// wheel's phantom (the word at the start of its state) with `hkpWorld::addPhantom` (`this` =
/// the world).
pub fn add_to_world(e: &mut Engine, this: Ptr<HkpVehicleLinearCastWheelCollide>, world: Ptr) {
    let count = e.get(this, HkpVehicleLinearCastWheelCollide::m_wheel_states_size);
    for index in 0..count {
        let states = e
            .get(this, HkpVehicleLinearCastWheelCollide::m_wheel_states)
            .addr();
        let phantom = e
            .mem
            .u32(states.wrapping_add((index as u32).wrapping_mul(STATE_SIZE)));
        e.call(ADD_PHANTOM, &args![world, phantom]);
    }
}

// Translated from 00e368a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleLinearCastWheelCollide::removeFromWorld` (Xbox PDB), `(this)`: removes every
/// wheel's phantom with `hkpWorld::removePhantom`, the world being the word at `phantom + 8`.
pub fn remove_from_world(e: &mut Engine, this: Ptr<HkpVehicleLinearCastWheelCollide>) {
    let count = e.get(this, HkpVehicleLinearCastWheelCollide::m_wheel_states_size);
    for index in 0..count {
        let states = e
            .get(this, HkpVehicleLinearCastWheelCollide::m_wheel_states)
            .addr();
        let phantom = e
            .mem
            .u32(states.wrapping_add((index as u32).wrapping_mul(STATE_SIZE)));
        let world = e.mem.u32(phantom + 8);
        e.call(REMOVE_PHANTOM, &args![world, phantom]);
    }
}

// Translated from 00e368d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleLinearCastWheelCollide::setCollisionFilterInfo` (Xbox PDB), `(this, info)`:
/// stores `info` at `this + 0xC` and, in every wheel's phantom that is not null, at `phantom +
/// 0x2C`.
pub fn set_collision_filter_info(
    e: &mut Engine,
    this: Ptr<HkpVehicleLinearCastWheelCollide>,
    info: u32,
) {
    let count = e.get(this, HkpVehicleLinearCastWheelCollide::m_wheel_states_size);
    e.set(
        this,
        HkpVehicleLinearCastWheelCollide::m_collision_filter_info,
        info,
    );
    for index in 0..count {
        let states = e
            .get(this, HkpVehicleLinearCastWheelCollide::m_wheel_states)
            .addr();
        let phantom = e
            .mem
            .u32(states.wrapping_add((index as u32).wrapping_mul(STATE_SIZE)));
        if phantom != 0 {
            e.mem.set_u32(phantom + 0x2c, info);
        }
    }
}

// Translated from 00e36900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleLinearCastWheelCollide::centerWheelContactPoint` (Xbox PDB), `(this, instance,
/// wheel word, point)`; `instance` is not read. With `t` = the wheel's transform (`[this +
/// 0x10] + wheel * 0x60 + 0x10`): `a` = `t` applied inversely to the point (`00c7f690`), its
/// lane 2 set to 0.0, and the point is replaced by `t` applied to `a` (`00c7f640`, all four
/// lanes of the result).
pub fn center_wheel_contact_point(
    e: &mut Engine,
    this: Ptr<HkpVehicleLinearCastWheelCollide>,
    _unused_1: u32,
    wheel_word: u32,
    point: Ptr,
) {
    let wheel = wheel_word & 0xff;
    let states = e
        .get(this, HkpVehicleLinearCastWheelCollide::m_wheel_states)
        .addr();
    let transform = states
        .wrapping_add(wheel.wrapping_mul(STATE_SIZE))
        .wrapping_add(0x10);
    e.with_stack(0x20, |e, vectors| {
        let a = vectors.addr();
        let b = a + 0x10;
        e.call(SET_TRANSFORMED_INVERSE_POS, &args![a, transform, point]);
        e.mem.set_f32(a + 8, 0.0);
        e.call(SET_TRANSFORMED_POS, &args![b, transform, a]);
        copy_vector(e, b, point.addr());
    });
}

// Translated from 00e36950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleLinearCastWheelCollide::updateBeforeCollisionDetection` (Xbox PDB), `(this,
/// instance)`. For every wheel `j` (the count is read once, a byte counter): `updateWheelState`
/// (`00e367d0`) `(instance, j)`, `calcAabbOfWheel` (`00e36700`) `(instance, j, box)` with a
/// 32-byte local box, and `setAabb` (`00cf24a0`) on the wheel's phantom with the box.
pub fn update_before_collision_detection(
    e: &mut Engine,
    this: Ptr<HkpVehicleLinearCastWheelCollide>,
    instance: Ptr,
) {
    let count = e.get(this, HkpVehicleLinearCastWheelCollide::m_wheel_states_size);
    if count <= 0 {
        return;
    }
    let mut wheel = 0u8;
    e.with_stack(0x20, |e, aabb| loop {
        let wheel_word = u32::from(wheel);
        e.call(UPDATE_WHEEL_STATE, &args![this, instance, wheel_word]);
        e.call(CALC_AABB_OF_WHEEL, &args![this, instance, wheel_word, aabb]);
        let states = e
            .get(this, HkpVehicleLinearCastWheelCollide::m_wheel_states)
            .addr();
        let phantom = e
            .mem
            .u32(states.wrapping_add(wheel_word.wrapping_mul(STATE_SIZE)));
        e.call(SET_AABB, &args![phantom, aabb]);
        wheel = wheel.wrapping_add(1);
        if i32::from(wheel) >= count {
            break;
        }
    });
}

// Translated from 00e369d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleLinearCastWheelCollide::castSingleWheel` (Xbox PDB), `(this, found, instance,
/// wheel word, hit)`; `instance` is not read. Returns `found`. With `state` = the wheel's state:
/// a local collidable is built (`00cd2d80`) from the state's shape, the transform at `state +
/// 0x10` and `0`, its filter info word (`+0x1C`) set to `this + 0xC`. A local linear cast input
/// holds the vector at `state + 0x50`, then `m_maxExtraPenetration` and `m_startPointTolerance`
/// of `this` (float copies); a local collector holds the vtable `010c476c`, the float at
/// `0104da48` at `+4` and `+0x2C`, and `0` at `+0x30`. `hkpAabbPhantom::linearCast` runs on the
/// state's phantom with the collidable, the input, the collector and `0`. When the collector's
/// word at `+0x30` is not zero afterwards the point at `collector + 0x10` is copied to `hit`
/// (`006212c0`) and the byte at `found` is `1`, otherwise it is `0`. (The exe also stores
/// `0x34000000` into the input's two float slots first; both are overwritten before the cast.)
pub fn cast_single_wheel(
    e: &mut Engine,
    this: Ptr<HkpVehicleLinearCastWheelCollide>,
    found: Ptr,
    _unused_1: u32,
    wheel_word: u32,
    hit: Ptr,
) -> Ptr {
    let wheel = wheel_word & 0xff;
    let states = e
        .get(this, HkpVehicleLinearCastWheelCollide::m_wheel_states)
        .addr();
    let state = states.wrapping_add(wheel.wrapping_mul(STATE_SIZE));
    let hit_found = e.with_stack(0x100, |e, frame| {
        let at = frame.addr();
        let collidable = at + 0xb0;
        let input = at + 0x10;
        let collector = at + 0x30;
        let shape = e.mem.u32(state + 4);
        e.call(
            COLLIDABLE_CONSTRUCTOR,
            &args![collidable, shape, state + 0x10, 0u32],
        );
        let filter = e.get(
            this,
            HkpVehicleLinearCastWheelCollide::m_collision_filter_info,
        );
        e.mem.set_u32(collidable + 0x1c, filter);
        copy_vector(e, state + 0x50, input);
        let penetration = e.get(
            this,
            HkpVehicleLinearCastWheelCollide::m_max_extra_penetration,
        );
        e.mem.set_f32(input + 0x10, x87_quiet(penetration));
        let tolerance = e.get(
            this,
            HkpVehicleLinearCastWheelCollide::m_start_point_tolerance,
        );
        e.mem.set_f32(input + 0x14, x87_quiet(tolerance));
        let distance = x87_quiet_bits(e.mem.u32(COLLECTOR_DISTANCE));
        e.mem.set_u32(collector + 0x2c, distance);
        e.mem.set_u32(collector + 4, distance);
        e.mem.set_u32(collector, COLLECTOR_VTABLE);
        e.mem.set_u32(collector + 0x30, 0);
        let phantom = e.mem.u32(state);
        e.call(
            LINEAR_CAST,
            &args![phantom, collidable, input, collector, 0u32],
        );
        let hit_found = e.mem.u32(collector + 0x30) != 0;
        if hit_found {
            e.call(ROOT_CD_POINT_ASSIGN, &args![hit, collector + 0x10]);
        }
        hit_found
    });
    e.mem.set_u8(found.addr(), u8::from(hit_found));
    found
}

// Translated from 00e36aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleLinearCastWheelCollide::wheelCollideCallback` (Xbox PDB): a jump to
/// `centerWheelContactPoint` (`00e36900`) with the same arguments, `(this, instance, wheel word,
/// point)`.
pub fn wheel_collide_callback(
    e: &mut Engine,
    this: Ptr<HkpVehicleLinearCastWheelCollide>,
    instance: Ptr,
    wheel_word: u32,
    point: Ptr,
) {
    e.call(
        CENTER_WHEEL_CONTACT_POINT,
        &args![this, instance, wheel_word, point],
    );
}

// Translated from 00e36ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleLinearCastWheelCollide::getPhantoms` (Xbox PDB), `(this, array)`: appends every
/// wheel's phantom to the `hkArray` at `array` (data `+0`, size `+4`, capacity and flags `+8`).
/// When the size equals the capacity (the low 30 bits) the array is grown first (`00c90940`
/// with the router's allocator, the array and `4`).
pub fn get_phantoms(e: &mut Engine, this: Ptr<HkpVehicleLinearCastWheelCollide>, array: Ptr) {
    let count = e.get(this, HkpVehicleLinearCastWheelCollide::m_wheel_states_size);
    let array_at = array.addr();
    for index in 0..count {
        let states = e
            .get(this, HkpVehicleLinearCastWheelCollide::m_wheel_states)
            .addr();
        let phantom = e
            .mem
            .u32(states.wrapping_add((index as u32).wrapping_mul(STATE_SIZE)));
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let capacity = e.mem.u32(array_at + 8) & 0x3fff_ffff;
        if e.mem.u32(array_at + 4) == capacity {
            let allocator = e.mem.u32(router + 0x10);
            e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
        }
        let size = e.mem.u32(array_at + 4);
        let data = e.mem.u32(array_at);
        e.mem
            .set_u32(data.wrapping_add(size.wrapping_mul(4)), phantom);
        e.mem.set_u32(array_at + 4, size.wrapping_add(1));
    }
}

// Translated from 00e36b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor (the map gives `00e36b20` no name), `(this)`; returns `this`. Stores the class's
/// vtable `010dec3c` at `+0`, the 16-bit `1` at `+6` and `+0x22`, `0x80000000` at `+0x18` (an
/// empty array that owns nothing), zero at `+0x10` and `+0x14`, the listener's vtables
/// (`010c925c` then `010debe4` at `+0x24`, `010debf4` at `+0x1C`), the float at `0104da4c`
/// at `+0x2C` and `+0x30` (copies, a signalling NaN is quieted), and the bytes `0` at `+8` and
/// `2` at `+9`.
pub fn construct(
    e: &mut Engine,
    this: Ptr<HkpVehicleLinearCastWheelCollide>,
) -> Ptr<HkpVehicleLinearCastWheelCollide> {
    let at = this.addr();
    let tolerance = x87_quiet_bits(e.mem.u32(INITIAL_TOLERANCE));
    e.mem.set_u32(at, VTABLE);
    e.mem.set_u16(at + 6, 1);
    e.mem.set_u32(at + 0x18, 0x8000_0000);
    e.mem.set_u32(at + 0x10, 0);
    e.mem.set_u32(at + 0x14, 0);
    e.mem.set_u16(at + 0x22, 1);
    e.mem.set_u32(at + 0x24, LISTENER_BASE_VTABLE_FIRST);
    e.mem.set_u32(at + 0x1c, LISTENER_VTABLE);
    e.mem.set_u32(at + 0x24, LISTENER_BASE_VTABLE);
    e.mem.set_u32(at + 0x2c, tolerance);
    e.mem.set_u32(at + 0x30, tolerance);
    e.mem.set_u8(at + 8, 0);
    e.mem.set_u8(at + 9, 2);
    this
}

// Translated from 00e36b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleLinearCastWheelCollide::~hkpVehicleLinearCastWheelCollide` (Xbox PDB), `(this)`.
/// Stores the vtable `010dec3c`, then for every wheel state: releases the shape (`00c905b0`);
/// when the 16-bit word at `this + 4` and the one at `phantom + 4` are both not zero removes
/// the listener from the phantom (`00d07c80`, the listener being `this + 0x24`); releases the
/// phantom. Then the listener is destroyed (`00e360f0` on `this + 0x1C`), the size is cleared
/// and, unless bit 31 of the capacity word is set, the array's data is freed through the
/// router's allocator (slot `0x10`, data and capacity times 0x60); data is cleared, the capacity
/// word set to `0x80000000` and the vtable to the base object's `0102e388`.
pub fn destructor(e: &mut Engine, this: Ptr<HkpVehicleLinearCastWheelCollide>) {
    let at = this.addr();
    let count = e.get(this, HkpVehicleLinearCastWheelCollide::m_wheel_states_size);
    e.mem.set_u32(at, VTABLE);
    for index in 0..count {
        let states = e
            .get(this, HkpVehicleLinearCastWheelCollide::m_wheel_states)
            .addr();
        let state = states.wrapping_add((index as u32).wrapping_mul(STATE_SIZE));
        let shape = e.mem.u32(state + 4);
        e.call(REMOVE_REFERENCE, &args![shape]);
        let phantom = e.mem.u32(state);
        if e.mem.u16(at + 4) != 0 && e.mem.u16(phantom.wrapping_add(4)) != 0 {
            e.call(
                REMOVE_OVERLAP_LISTENER,
                &args![phantom, overlap_listener(at)],
            );
        }
        e.call(REMOVE_REFERENCE, &args![phantom]);
    }
    e.call(LISTENER_DESTRUCTOR, &args![at + 0x1c]);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.get(
        this,
        HkpVehicleLinearCastWheelCollide::m_wheel_states_capacity_and_flags,
    );
    e.set(
        this,
        HkpVehicleLinearCastWheelCollide::m_wheel_states_size,
        0,
    );
    if (capacity as i32) >= 0 {
        let data = e
            .get(this, HkpVehicleLinearCastWheelCollide::m_wheel_states)
            .addr();
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff).wrapping_mul(STATE_SIZE)],
        );
    }
    e.set(
        this,
        HkpVehicleLinearCastWheelCollide::m_wheel_states,
        Ptr::new(0),
    );
    e.set(
        this,
        HkpVehicleLinearCastWheelCollide::m_wheel_states_capacity_and_flags,
        0x8000_0000,
    );
    e.mem.set_u32(at, BASE_VTABLE);
}

// Translated from 00e36cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleLinearCastWheelCollide::init` (Xbox PDB), `(this, instance)`. `n` = the wheel
/// count, the byte at `[instance + 0x1C] + 0x20`.
/// 1. When the wheel state array is empty: it is reserved for `n` states (`00c908c0` with the
///    router's allocator, the array, `max(2 * capacity, n)` when the capacity (low 30 bits) is
///    below `n`, and 0x60), its size set to `n`, and for every wheel `j` the shape is made with
///    `createWheelShape` (`00e36120`) `(this, width, radius)` from the wheel's parameters at
///    `[[instance + 0x1C] + 0x8C] + j * 0x28` (radius at `+0`, width at `+8`, both read as
///    quieted floats) and stored at `state + 4`;
/// 2. `this + 0x28` = `[instance + 0x18] + 0x10`;
/// 3. for every wheel `j`: `updateWheelState` (`00e367d0`) `(instance, j)`, `calcAabbOfWheel`
///    (`00e36700`) `(instance, j, box)` into a 32-byte local, a 0xE0-byte block from the
///    router's allocator (its size stored at `+4`) made into an `hkpAabbPhantom` (`00cf2cb0`
///    with the box and the filter info at `this + 0xC`) which is stored at `state + 0`, and
///    `addPhantomOverlapListener` (`00d07de0`) on it with the listener (`this + 0x24`).
pub fn init(e: &mut Engine, this: Ptr<HkpVehicleLinearCastWheelCollide>, instance: Ptr) {
    let at = this.addr();
    let instance_at = instance.addr();
    let data = e.mem.u32(instance_at + 0x1c);
    let count = u32::from(e.mem.u8(data + 0x20));
    if e.get(this, HkpVehicleLinearCastWheelCollide::m_wheel_states_size) == 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = (e.get(
            this,
            HkpVehicleLinearCastWheelCollide::m_wheel_states_capacity_and_flags,
        ) & 0x3fff_ffff) as i32;
        if capacity < count as i32 {
            let doubled = capacity.wrapping_add(capacity);
            let wanted = if (count as i32) < doubled {
                doubled
            } else {
                count as i32
            };
            e.call(
                ARRAY_RESERVE,
                &args![allocator, at + 0x10, wanted as u32, STATE_SIZE],
            );
        }
        e.set(
            this,
            HkpVehicleLinearCastWheelCollide::m_wheel_states_size,
            count as i32,
        );
        let mut wheel = 0u8;
        if count != 0 {
            loop {
                let params = e.mem.u32(e.mem.u32(instance_at + 0x1c) + 0x8c);
                let entry = params.wrapping_add(u32::from(wheel).wrapping_mul(WHEEL_PARAMS_SIZE));
                let radius = x87_quiet(e.mem.f32(entry));
                let states = e
                    .get(this, HkpVehicleLinearCastWheelCollide::m_wheel_states)
                    .addr();
                let state = states.wrapping_add(u32::from(wheel).wrapping_mul(STATE_SIZE));
                let width = x87_quiet(e.mem.f32(entry + 8));
                let shape = e
                    .call(CREATE_WHEEL_SHAPE, &args![this, width, radius])
                    .u32();
                e.mem.set_u32(state + 4, shape);
                wheel = wheel.wrapping_add(1);
                if u32::from(wheel) >= count {
                    break;
                }
            }
        }
    }
    let chassis = e.mem.u32(instance_at + 0x18);
    e.set(
        this,
        HkpVehicleLinearCastWheelCollide::m_chassis,
        Ptr::new(chassis.wrapping_add(0x10)),
    );
    let mut wheel = 0u8;
    if count == 0 {
        return;
    }
    loop {
        let wheel_word = u32::from(wheel);
        e.call(UPDATE_WHEEL_STATE, &args![this, instance, wheel_word]);
        e.with_stack(0x20, |e, aabb| {
            e.call(CALC_AABB_OF_WHEEL, &args![this, instance, wheel_word, aabb]);
            let block = allocate_object(e, 0xe0);
            let filter = e.get(
                this,
                HkpVehicleLinearCastWheelCollide::m_collision_filter_info,
            );
            let phantom = e
                .call(AABB_PHANTOM_CONSTRUCTOR, &args![block, aabb, filter])
                .u32();
            let states = e
                .get(this, HkpVehicleLinearCastWheelCollide::m_wheel_states)
                .addr();
            let state = states.wrapping_add(wheel_word.wrapping_mul(STATE_SIZE));
            e.mem.set_u32(state, phantom);
            let phantom = e.mem.u32(state);
            e.call(ADD_OVERLAP_LISTENER, &args![phantom, overlap_listener(at)]);
        });
        wheel = wheel.wrapping_add(1);
        if u32::from(wheel) >= count {
            break;
        }
    }
}

// Translated from 00e36e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleLinearCastWheelCollide::clone` (Xbox PDB), `(this, body, phantoms)`: `body` is
/// the new vehicle's chassis body (the clone's `+0x28` is `body + 0x10`), `phantoms` an
/// `hkArray` of the phantoms the clone uses (data pointer at `+0`), one per wheel (the
/// parameter names are unconfirmed). A 0x34-byte block of the router's allocator (its size
/// stored at `+4`) is made into a collider (`00e36b20`); its wheel state array is reserved for
/// `this`'s `n` states (`00c908c0`, allocator, array, `max(2 * capacity, n)` when the capacity
/// is below `n`, 0x60) and its size set to `n`. For every wheel `j`: the clone's state takes
/// `this`'s shape (reference added, `00c90510`) and the new phantom `j` (reference added); the
/// phantom's overlap listener `this + 0x24` is removed and the clone's added; the clone's
/// `+0x28` is set to `body + 0x10`. Then the filter info (`+0xC`) and `+0x2C`, `+0x30` (float
/// copies) are copied. Returns the clone.
pub fn clone_wheel_collide(
    e: &mut Engine,
    this: Ptr<HkpVehicleLinearCastWheelCollide>,
    body: Ptr,
    phantoms: Ptr,
) -> Ptr<HkpVehicleLinearCastWheelCollide> {
    let at = this.addr();
    let block = allocate_object(e, 0x34);
    let clone = e
        .call(CONSTRUCT, &args![block])
        .ptr::<HkpVehicleLinearCastWheelCollide>();
    let count = e.get(this, HkpVehicleLinearCastWheelCollide::m_wheel_states_size);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = (e.get(
        clone,
        HkpVehicleLinearCastWheelCollide::m_wheel_states_capacity_and_flags,
    ) & 0x3fff_ffff) as i32;
    if capacity < count {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if count < doubled { doubled } else { count };
        e.call(
            ARRAY_RESERVE,
            &args![allocator, clone.addr() + 0x10, wanted as u32, STATE_SIZE],
        );
    }
    e.set(
        clone,
        HkpVehicleLinearCastWheelCollide::m_wheel_states_size,
        count,
    );
    for index in 0..count {
        let offset = (index as u32).wrapping_mul(STATE_SIZE);
        let new_state = e
            .get(clone, HkpVehicleLinearCastWheelCollide::m_wheel_states)
            .addr()
            .wrapping_add(offset);
        let old_state = e
            .get(this, HkpVehicleLinearCastWheelCollide::m_wheel_states)
            .addr()
            .wrapping_add(offset);
        let shape = e.mem.u32(old_state + 4);
        e.mem.set_u32(new_state + 4, shape);
        e.call(ADD_REFERENCE, &args![shape]);
        let phantom_data = e.mem.u32(phantoms.addr());
        let phantom = e
            .mem
            .u32(phantom_data.wrapping_add((index as u32).wrapping_mul(4)));
        e.mem.set_u32(new_state, phantom);
        e.call(ADD_REFERENCE, &args![phantom]);
        e.call(
            REMOVE_OVERLAP_LISTENER,
            &args![phantom, overlap_listener(at)],
        );
        e.call(
            ADD_OVERLAP_LISTENER,
            &args![phantom, overlap_listener(clone.addr())],
        );
        e.set(
            clone,
            HkpVehicleLinearCastWheelCollide::m_chassis,
            Ptr::new(body.addr().wrapping_add(0x10)),
        );
    }
    let filter = e.get(
        this,
        HkpVehicleLinearCastWheelCollide::m_collision_filter_info,
    );
    e.set(
        clone,
        HkpVehicleLinearCastWheelCollide::m_collision_filter_info,
        filter,
    );
    let penetration = e.get(
        this,
        HkpVehicleLinearCastWheelCollide::m_max_extra_penetration,
    );
    e.set(
        clone,
        HkpVehicleLinearCastWheelCollide::m_max_extra_penetration,
        x87_quiet(penetration),
    );
    let tolerance = e.get(
        this,
        HkpVehicleLinearCastWheelCollide::m_start_point_tolerance,
    );
    e.set(
        clone,
        HkpVehicleLinearCastWheelCollide::m_start_point_tolerance,
        x87_quiet(tolerance),
    );
    clone
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e3_6120,
            create_wheel_shape(Ptr<HkpVehicleLinearCastWheelCollide>, f32, f32) -> u32
        ),
        entry!(
            0x00e3_61b0,
            collide_wheels(Ptr<HkpVehicleLinearCastWheelCollide>, u32, Ptr, u32)
        ),
        entry!(
            0x00e3_6250,
            determine_nearest_hit(Ptr<HkpVehicleLinearCastWheelCollide>, u32, Ptr) -> u32
        ),
        entry!(
            0x00e3_6360,
            get_total_num_commands(Ptr<HkpVehicleLinearCastWheelCollide>) -> i32
        ),
        entry!(
            0x00e3_63a0,
            get_num_commands(Ptr<HkpVehicleLinearCastWheelCollide>, u32) -> i32
        ),
        entry!(
            0x00e3_63d0,
            build_linear_cast_commands(
                Ptr<HkpVehicleLinearCastWheelCollide>,
                u32,
                u32,
                u32,
                u32,
                u32,
            ) -> i32
        ),
        entry!(
            0x00e3_6510,
            get_collision_output_from_cast_result(
                Ptr<HkpVehicleLinearCastWheelCollide>,
                Ptr,
                u32,
                Ptr,
                Ptr,
            )
        ),
        entry!(
            0x00e3_6670,
            get_collision_output_without_hit(Ptr<HkpVehicleLinearCastWheelCollide>, Ptr, u32, Ptr)
        ),
        entry!(
            0x00e3_6700,
            calc_aabb_of_wheel(Ptr<HkpVehicleLinearCastWheelCollide>, Ptr, u32, Ptr)
        ),
        entry!(
            0x00e3_67d0,
            update_wheel_state(Ptr<HkpVehicleLinearCastWheelCollide>, Ptr, u32)
        ),
        entry!(
            0x00e3_6870,
            add_to_world(Ptr<HkpVehicleLinearCastWheelCollide>, Ptr)
        ),
        entry!(
            0x00e3_68a0,
            remove_from_world(Ptr<HkpVehicleLinearCastWheelCollide>)
        ),
        entry!(
            0x00e3_68d0,
            set_collision_filter_info(Ptr<HkpVehicleLinearCastWheelCollide>, u32)
        ),
        entry!(
            0x00e3_6900,
            center_wheel_contact_point(Ptr<HkpVehicleLinearCastWheelCollide>, u32, u32, Ptr)
        ),
        entry!(
            0x00e3_6950,
            update_before_collision_detection(Ptr<HkpVehicleLinearCastWheelCollide>, Ptr)
        ),
        entry!(
            0x00e3_69d0,
            cast_single_wheel(Ptr<HkpVehicleLinearCastWheelCollide>, Ptr, u32, u32, Ptr) -> Ptr
        ),
        entry!(
            0x00e3_6aa0,
            wheel_collide_callback(Ptr<HkpVehicleLinearCastWheelCollide>, Ptr, u32, Ptr)
        ),
        entry!(
            0x00e3_6ab0,
            get_phantoms(Ptr<HkpVehicleLinearCastWheelCollide>, Ptr)
        ),
        entry!(
            0x00e3_6b20,
            construct(
                Ptr<HkpVehicleLinearCastWheelCollide>,
            ) -> Ptr<HkpVehicleLinearCastWheelCollide>
        ),
        entry!(
            0x00e3_6b80,
            destructor(Ptr<HkpVehicleLinearCastWheelCollide>)
        ),
        entry!(
            0x00e3_6cc0,
            init(Ptr<HkpVehicleLinearCastWheelCollide>, Ptr)
        ),
        entry!(
            0x00e3_6e20,
            clone_wheel_collide(
                Ptr<HkpVehicleLinearCastWheelCollide>,
                Ptr,
                Ptr,
            ) -> Ptr<HkpVehicleLinearCastWheelCollide>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, new_log, Log,
    };
    use std::rc::Rc;

    /// Base address of the collider's logging vtable slots.
    const SLOT_BASE: u32 = 0x0900_0000;
    /// Base address of the allocator's logging vtable slots.
    const ALLOCATOR_BASE: u32 = 0x0900_8000;

    /// An object of `size` bytes whose vtable slots `0` to `0x4C` are logging doubles at
    /// `base + slot`, all answering 0.
    fn object(e: &mut Engine, log: &Log, base: u32, size: u32) -> u32 {
        let vtable = e.mem.alloc(0x50);
        for slot in (0..0x50).step_by(4) {
            log_double(e, log, base + slot, 0);
            e.mem.set_u32(vtable + slot, base + slot);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    /// A collider (0x40 bytes, logging vtable at `SLOT_BASE`) over wheel states whose phantoms
    /// are `phantoms`; returns the collider and its states.
    fn collider(e: &mut Engine, log: &Log, phantoms: &[u32]) -> (u32, u32) {
        let this = object(e, log, SLOT_BASE, 0x40);
        let states = e.mem.alloc(0x60 * phantoms.len().max(1) as u32);
        for (i, phantom) in phantoms.iter().enumerate() {
            e.mem.set_u32(states + 0x60 * i as u32, *phantom);
        }
        e.mem.set_u32(this + 0x10, states);
        e.mem.set_i32(this + 0x14, phantoms.len() as i32);
        (this, states)
    }

    /// Makes the memory router answer a router whose allocator (a logging object at
    /// `ALLOCATOR_BASE`) hands out `blocks` in turn from slot 4 (the last one again after that);
    /// returns the allocator.
    fn install_allocator(e: &mut Engine, log: &Log, blocks: &[u32]) -> u32 {
        let allocator = object(e, log, ALLOCATOR_BASE, 0x10);
        let blocks = blocks.to_vec();
        let sink = Rc::clone(log);
        let mut next = 0usize;
        e.register_double(ALLOCATOR_BASE + 4, move |_, a| {
            sink.borrow_mut().push((ALLOCATOR_BASE + 4, a.to_vec()));
            let block = blocks[next.min(blocks.len() - 1)];
            next += 1;
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        allocator
    }

    /// A vehicle instance with `wheels` wheels: its data (`+0x1C`: wheel count at `+0x20`, the
    /// parameters at `+0x8C`), the suspension (`+0x34`, array at `+8`), the wheel records
    /// (`+0x48`) and a chassis body (`+0x18`).
    fn new_instance(e: &mut Engine, wheels: u8) -> u32 {
        let n = u32::from(wheels).max(1);
        let instance = e.mem.alloc(0x80);
        let data = e.mem.alloc(0x100);
        e.mem.set_u8(data + 0x20, wheels);
        let params = e.mem.alloc(0x28 * n);
        e.mem.set_u32(data + 0x8c, params);
        let suspension = e.mem.alloc(0x20);
        let lengths = e.mem.alloc(0x30 * n);
        e.mem.set_u32(suspension + 8, lengths);
        let info = e.mem.alloc(0xe0 * n);
        let body = e.mem.alloc(0x200);
        e.mem.set_u32(instance + 0x18, body);
        e.mem.set_u32(instance + 0x1c, data);
        e.mem.set_u32(instance + 0x34, suspension);
        e.mem.set_u32(instance + 0x48, info);
        instance
    }

    /// The parameters, suspension entry and record of wheel `wheel` of `instance`.
    fn params(e: &Engine, instance: u32, wheel: u32) -> u32 {
        e.mem.u32(e.mem.u32(instance + 0x1c) + 0x8c) + 0x28 * wheel
    }

    fn suspension(e: &Engine, instance: u32, wheel: u32) -> u32 {
        e.mem.u32(e.mem.u32(instance + 0x34) + 8) + 0x30 * wheel
    }

    fn info(e: &Engine, instance: u32, wheel: u32) -> u32 {
        e.mem.u32(instance + 0x48) + 0xe0 * wheel
    }

    fn set_lanes(e: &mut Engine, at: u32, values: [f32; 4]) {
        for (i, value) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *value);
        }
    }

    #[test]
    fn the_wheel_shape_is_a_cylinder_along_the_z_axis() {
        let mut e = engine();
        let log = new_log();
        for lane in 0..3 {
            e.mem.set_u32(SIGN_MASK_XYZ + 4 * lane, 0x8000_0000);
        }
        let block = e.mem.alloc(0x60);
        let allocator = install_allocator(&mut e, &log, &[block]);
        let sink = Rc::clone(&log);
        e.register_double(CYLINDER_SHAPE_CONSTRUCTOR, move |e, a| {
            let mut words = a.to_vec();
            for vector in [a[1], a[2]] {
                words.extend((0..4).map(|i| e.mem.u32(vector + 4 * i)));
            }
            sink.borrow_mut().push((CYLINDER_SHAPE_CONSTRUCTOR, words));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x40);
        let shape = e
            .call(CREATE_WHEEL_SHAPE, &args![this, 2.0f32, 0.25f32])
            .u32();
        assert_eq!(shape, block);
        assert_eq!(e.mem.u16(block + 4), 0x60);
        assert_eq!(calls(&log, ALLOCATOR_BASE + 4), vec![vec![allocator, 0x60]]);
        let seen = calls(&log, CYLINDER_SHAPE_CONSTRUCTOR);
        assert_eq!(seen.len(), 1);
        let words = &seen[0];
        assert_eq!(words[0], block);
        assert_eq!(words[3], 0.25f32.to_bits());
        assert_eq!(words[4], 0);
        // Half the width up (lane 2), and its mirror with lanes 0 to 2 flipped, lane 3 not.
        assert_eq!(words[5..9], [0, 0, 1.0f32.to_bits(), 0]);
        assert_eq!(
            words[9..13],
            [0x8000_0000, 0x8000_0000, (-1.0f32).to_bits(), 0]
        );
    }

    #[test]
    fn collide_wheels_casts_each_wheel_then_fills_its_record() {
        let mut e = engine();
        let log = new_log();
        let (this, _) = collider(&mut e, &log, &[]);
        let vehicle = new_instance(&mut e, 2);
        let flag = e.mem.alloc(8);
        log_double(&mut e, &log, SLOT_BASE + 0x40, flag);
        let records = e.mem.alloc(0xc0);
        for found in [1u8, 0] {
            log.borrow_mut().clear();
            e.mem.set_u8(flag, found);
            e.call(0x00e3_61b0, &args![this, 0x77u32, vehicle, records]);
            let order: Vec<u32> = log.borrow().iter().map(|(a, _)| a - SLOT_BASE).collect();
            let fill = if found == 1 { 0x44 } else { 0x48 };
            assert_eq!(order, vec![0x40, fill, 0x2c, 0x40, fill, 0x2c]);
            // Slot 0x40: (this, flag slot, instance, wheel, cast record).
            let casts = calls(&log, SLOT_BASE + 0x40);
            assert_eq!(casts.len(), 2);
            assert_eq!(casts[0][0], this);
            assert_eq!(casts[0][2..4], [vehicle, 0]);
            assert_eq!(casts[1][2..4], [vehicle, 1]);
            assert_ne!(casts[0][1], 0);
            assert_eq!(casts[0][1], casts[1][1]);
            assert_eq!(casts[0][4], casts[1][4]);
            let fills = calls(&log, SLOT_BASE + fill);
            if found == 1 {
                assert_eq!(fills[0], vec![this, vehicle, 0, casts[0][4], records]);
                assert_eq!(
                    fills[1],
                    vec![this, vehicle, 1, casts[1][4], records + 0x60]
                );
            } else {
                assert_eq!(fills[0], vec![this, vehicle, 0, records]);
                assert_eq!(fills[1], vec![this, vehicle, 1, records + 0x60]);
            }
            assert_eq!(
                calls(&log, SLOT_BASE + 0x2c),
                vec![
                    vec![this, vehicle, 0, records],
                    vec![this, vehicle, 1, records + 0x60]
                ]
            );
        }
        // No wheels: nothing is called.
        let empty = new_instance(&mut e, 0);
        log.borrow_mut().clear();
        e.call(0x00e3_61b0, &args![this, 0u32, empty, records]);
        assert!(log.borrow().is_empty());
    }

    /// The entries `determineNearestHit` reads: `(flag, fraction)` each; returns the buffer and
    /// the hits' addresses.
    fn entries(e: &mut Engine, specs: &[(bool, f32)]) -> (u32, Vec<u32>) {
        let buffer = e.mem.alloc(0x28 + 0x40 * specs.len() as u32);
        let mut hits = Vec::new();
        for (i, (flag, fraction)) in specs.iter().enumerate() {
            let hit = e.mem.alloc(0x40);
            e.mem.set_f32(hit + 0x1c, *fraction);
            let entry = buffer + 0x28 + 0x40 * i as u32;
            e.mem.set_u32(entry, hit);
            e.mem.set_u32(entry + 8, u32::from(*flag));
            hits.push(hit);
        }
        (buffer, hits)
    }

    /// Runs `determineNearestHit` over `specs`; returns the answer and the hits.
    fn nearest(e: &mut Engine, log: &Log, this: u32, specs: &[(bool, f32)]) -> (u32, Vec<u32>) {
        log_double(e, log, SLOT_BASE + 0x34, specs.len() as u32);
        let (buffer, hits) = entries(e, specs);
        let best = e.call(0x00e3_6250, &args![this, 0x55u32, buffer]).u32();
        (best, hits)
    }

    #[test]
    fn the_nearest_hit_is_the_strictly_smallest_fraction() {
        let mut e = engine();
        let log = new_log();
        let (this, _) = collider(&mut e, &log, &[]);
        let specs = [
            (false, 0.01),
            (true, 0.5),
            (true, 0.25),
            (true, 0.25),
            (true, f32::NAN),
            (true, 0.1),
        ];
        let (best, hits) = nearest(&mut e, &log, this, &specs);
        assert_eq!(best, hits[5]);
        assert_eq!(calls(&log, SLOT_BASE + 0x34), vec![vec![this, 0x55]]);
        // A tie keeps the first of the two and a NaN never replaces.
        let (best, hits) = nearest(&mut e, &log, this, &specs[..5]);
        assert_eq!(best, hits[2]);
        // A NaN that comes first stays: nothing compares smaller than it.
        let (best, hits) = nearest(&mut e, &log, this, &[(true, f32::NAN), (true, 0.3)]);
        assert_eq!(best, hits[0]);
        // No offered hit: none.
        assert_eq!(nearest(&mut e, &log, this, &[(false, 1.0)]).0, 0);
        assert_eq!(nearest(&mut e, &log, this, &[]).0, 0);
    }

    #[test]
    fn the_total_adds_every_wheels_command_count() {
        let mut e = engine();
        let log = new_log();
        let (this, _) = collider(&mut e, &log, &[0, 0, 0]);
        log_double(&mut e, &log, SLOT_BASE + 0x34, 2);
        assert_eq!(e.call(0x00e3_6360, &args![this]).i32(), 6);
        // The wheel word is the low byte of `this` replaced by the wheel number.
        let base = this & 0xffff_ff00;
        let words: Vec<u32> = calls(&log, SLOT_BASE + 0x34).iter().map(|c| c[1]).collect();
        assert_eq!(words, vec![base, base | 1, base | 2]);
        let (none, _) = collider(&mut e, &log, &[]);
        assert_eq!(e.call(0x00e3_6360, &args![none]).i32(), 0);
    }

    #[test]
    fn the_command_count_is_the_phantoms_overlap_count() {
        let mut e = engine();
        let log = new_log();
        let first = object(&mut e, &log, 0x0900_1000, 0x100);
        let second = object(&mut e, &log, 0x0900_2000, 0x100);
        e.mem.set_i32(first + 0xd4, 5);
        e.mem.set_i32(second + 0xd4, 7);
        let (this, _) = collider(&mut e, &log, &[first, second]);
        // Only the low byte of the wheel word counts.
        assert_eq!(e.call(0x00e3_63a0, &args![this, 0xaabb_cc01u32]).i32(), 7);
        assert_eq!(calls(&log, 0x0900_2028), vec![vec![second]]);
        assert!(calls(&log, 0x0900_1028).is_empty());
    }

    #[test]
    fn building_the_commands_makes_a_record_per_wheel_and_a_command_per_cast() {
        let mut e = engine();
        let log = new_log();
        e.mem.set_u32(COLLIDABLE_MINUS_ONE, 0xbf80_0000);
        log_double(&mut e, &log, BOUNDING_VOLUME_DATA_CONSTRUCTOR, 0);
        let first = object(&mut e, &log, 0x0900_1000, 0x100);
        let second = object(&mut e, &log, 0x0900_2000, 0x100);
        for (phantom, collidables) in [(first, vec![0xc0u32, 0xc1]), (second, vec![0xc2])] {
            let data = e.mem.alloc(8);
            for (i, collidable) in collidables.iter().enumerate() {
                e.mem.set_u32(data + 4 * i as u32, *collidable);
            }
            e.mem.set_u32(phantom + 0xd0, data);
            e.mem.set_i32(phantom + 0xd4, collidables.len() as i32);
        }
        let (this, states) = collider(&mut e, &log, &[first, second]);
        for (wheel, shape) in [0x1111u32, 0x2222].iter().enumerate() {
            let state = states + 0x60 * wheel as u32;
            e.mem.set_u32(state + 4, *shape);
            set_lanes(&mut e, state + 0x40, [1.0 + wheel as f32, 2.0, 3.0, 4.0]);
            set_lanes(&mut e, state + 0x50, [5.0, 6.0, 7.0 + wheel as f32, 8.0]);
        }
        let records = e.mem.alloc(0xa0);
        let inputs = e.mem.alloc(0xc0);
        let outputs = 0x5000_0000u32;
        let total = e
            .call(
                0x00e3_63d0,
                &args![this, 1u32, 2u32, records, inputs, outputs],
            )
            .i32();
        assert_eq!(total, 3);
        for (wheel, shape) in [0x1111u32, 0x2222].iter().enumerate() {
            let record = records + 0x50 * wheel as u32;
            let state = states + 0x60 * wheel as u32;
            assert_eq!(e.mem.u32(record), *shape);
            assert_eq!(e.mem.u32(record + 4), 0xffff_ffff);
            assert_eq!(e.mem.u32(record + 8), state + 0x10);
            assert_eq!(e.mem.u8(record + 0x19), 0xec);
            assert_eq!(e.mem.u8(record + 0x1a), 0xff);
            assert_eq!(e.mem.u32(record + 0x4c), 0xbf80_0000);
        }
        assert_eq!(
            calls(&log, BOUNDING_VOLUME_DATA_CONSTRUCTOR),
            vec![vec![records + 0x20], vec![records + 0x70]]
        );
        assert_eq!(calls(&log, 0x0900_1028), vec![vec![first]]);
        assert_eq!(calls(&log, 0x0900_2028), vec![vec![second]]);
        for (j, &(wheel, collidable)) in [(0u32, 0xc0u32), (0, 0xc1), (1, 0xc2)].iter().enumerate()
        {
            let input = inputs + 0x40 * j as u32;
            let state = states + 0x60 * wheel;
            assert_eq!(e.mem.u32(input + 0x20), records + 0x50 * wheel);
            assert_eq!(e.mem.u32(input + 0x24), collidable);
            assert_eq!(e.mem.u32(input + 0x28), outputs + 0x70 * j as u32);
            assert_eq!(lanes(&e, input), lanes(&e, state + 0x40));
            assert_eq!(lanes(&e, input + 0x10), lanes(&e, state + 0x50));
            assert_eq!(e.mem.u32(input + 0x2c), 1);
        }
        // A null record array builds no records but the commands still point at it.
        let (single, single_states) = collider(&mut e, &log, &[first]);
        e.mem.set_u32(single_states + 4, 0x3333);
        log.borrow_mut().clear();
        let more = e.mem.alloc(0x80);
        let total = e
            .call(0x00e3_63d0, &args![single, 0u32, 0u32, 0u32, more, outputs])
            .i32();
        assert_eq!(total, 2);
        assert!(calls(&log, BOUNDING_VOLUME_DATA_CONSTRUCTOR).is_empty());
        assert_eq!(e.mem.u32(more + 0x20), 0);
    }

    /// A vehicle with one wheel for the collision outputs: stiffness 2 (`+0x84` of the data),
    /// suspension float 3, the wheel record's direction `axis`. Returns the instance.
    fn output_vehicle(e: &mut Engine, axis: [f32; 4]) -> u32 {
        let vehicle = new_instance(e, 1);
        let data = e.mem.u32(vehicle + 0x1c);
        e.mem.set_f32(data + 0x84, 2.0);
        let entry = suspension(e, vehicle, 0);
        e.mem.set_f32(entry + 0x20, 3.0);
        let record = info(e, vehicle, 0);
        set_lanes(e, record + 0x80, axis);
        vehicle
    }

    #[test]
    fn a_cast_hit_gives_the_relative_velocity_along_the_cast() {
        let mut e = engine();
        let log = new_log();
        let vehicle = output_vehicle(&mut e, [0.0, 0.0, -4.0, 0.0]);
        let block = e.mem.alloc(0x200);
        let body = block + 0x100;
        let other = block + 0xe0;
        e.mem.set_u8(body + 0x18, 1);
        e.mem.set_i8(body + 0x10, -0x20);
        e.mem.set_f32(other + 0x90, 7.0);
        let cast = e.mem.alloc(0x60);
        set_lanes(&mut e, cast, [1.0, 2.0, 3.0, 4.0]);
        set_lanes(&mut e, cast + 0x10, [0.0, 0.0, 1.0, 0.5]);
        e.mem.set_u32(cast + 0x48, body);
        e.mem.set_u32(cast + 0x4c, 0x4242);
        let chassis = e.mem.u32(vehicle + 0x18);
        let sink = Rc::clone(&log);
        e.register_double(GET_POINT_VELOCITY, move |e, a| {
            sink.borrow_mut().push((GET_POINT_VELOCITY, a.to_vec()));
            let z = if a[0] == chassis { 5.0 } else { 1.0 };
            for (i, lane) in [0.0f32, 0.0, z, 0.0].iter().enumerate() {
                e.mem.set_f32(a[2] + 4 * i as u32, *lane);
            }
            Ret::default()
        });
        let out = e.mem.alloc(0x60);
        let this = e.mem.alloc(0x40);
        e.call(0x00e3_6510, &args![this, vehicle, 0u32, cast, out]);
        assert_eq!(lanes(&e, out), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(lanes(&e, out + 0x10), [0.0, 0.0, 1.0, 0.5]);
        assert_eq!(e.mem.u32(out + 0x24), other);
        assert_eq!(e.mem.f32(out + 0x20), 7.0);
        assert_eq!(e.mem.u32(out + 0x28), 0x4242);
        assert_eq!(e.mem.u32(out + 0x2c), 0xffff_ffff);
        assert_eq!(e.mem.f32(out + 0x48), 1.5);
        // d = -4 < -2: r = -1 / -4 = 0.25 and the relative velocity along the normal is 4.
        assert_eq!(e.mem.f32(out + 0x4c), 1.0);
        assert_eq!(e.mem.f32(out + 0x50), 0.25);
        let velocities = calls(&log, GET_POINT_VELOCITY);
        assert_eq!(velocities.len(), 2);
        assert_eq!(velocities[0][..2], [chassis, out]);
        assert_eq!(velocities[1][..2], [other, out]);
        // A shallow direction (d = -1, not below -2) or a NaN gives the soft output.
        let record = info(&e, vehicle, 0);
        for axis in [[0.0, 0.0, -1.0, 0.0], [0.0, 0.0, f32::NAN, 0.0]] {
            set_lanes(&mut e, record + 0x80, axis);
            e.call(0x00e3_6510, &args![this, vehicle, 0u32, cast, out]);
            assert_eq!(e.mem.f32(out + 0x4c), 0.0);
            assert_eq!(e.mem.f32(out + 0x50), 0.5);
        }
        assert_eq!(calls(&log, GET_POINT_VELOCITY).len(), 2);
    }

    #[test]
    fn a_miss_gives_the_reverse_direction_and_the_full_suspension() {
        let mut e = engine();
        let vehicle = output_vehicle(&mut e, [1.0, -2.0, 3.0, 9.0]);
        let record = info(&e, vehicle, 0);
        set_lanes(&mut e, record + 0x60, [1.5, 2.5, 3.5, 4.5]);
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK_ALL + 4 * lane, 0x8000_0000);
        }
        let entry = suspension(&e, vehicle, 0);
        e.mem.set_u32(entry + 0x20, 0x7f80_0001);
        let out = e.mem.alloc(0x60);
        e.mem.set_u32(out + 0x24, 0x99);
        e.mem.set_u32(out + 0x1c, 0x1234);
        let this = e.mem.alloc(0x40);
        e.call(0x00e3_6670, &args![this, vehicle, 0u32, out]);
        assert_eq!(lanes(&e, out), [1.5, 2.5, 3.5, 4.5]);
        assert_eq!(lanes(&e, out + 0x10)[..3], [-1.0, 2.0, -3.0]);
        // The suspension float is copied through the x87 stack, which quiets a signalling NaN.
        assert_eq!(e.mem.u32(out + 0x1c), 0x7fc0_0001);
        assert_eq!(e.mem.u32(out + 0x48), 0x7fc0_0001);
        assert_eq!(e.mem.f32(out + 0x20), 0.0);
        assert_eq!(e.mem.u32(out + 0x24), 0);
        assert_eq!(e.mem.f32(out + 0x4c), 0.0);
        assert_eq!(e.mem.f32(out + 0x50), 1.0);
    }

    #[test]
    fn the_wheel_box_is_half_the_extents_around_the_origin() {
        let mut e = engine();
        let log = new_log();
        let vehicle = new_instance(&mut e, 1);
        let wheel = params(&e, vehicle, 0);
        e.mem.set_f32(wheel, 0.5);
        e.mem.set_f32(wheel + 8, 0.25);
        let entry = suspension(&e, vehicle, 0);
        e.mem.set_f32(entry + 0x20, 1.5);
        let (this, states) = collider(&mut e, &log, &[0]);
        let sink = Rc::clone(&log);
        e.register_double(CALC_AABB, move |e, a| {
            let mut words = a.to_vec();
            for vector in [a[1], a[2]] {
                words.extend((0..4).map(|i| e.mem.u32(vector + 4 * i)));
            }
            sink.borrow_mut().push((CALC_AABB, words));
            Ret::default()
        });
        let aabb = e.mem.alloc(0x20);
        // The wheel word's upper bytes are ignored.
        e.call(CALC_AABB_OF_WHEEL, &args![this, vehicle, 0x4400u32, aabb]);
        let seen = calls(&log, CALC_AABB);
        assert_eq!(seen.len(), 1);
        let words = &seen[0];
        assert_eq!(words[0], states + 0x10);
        assert_eq!(words[3], 0);
        assert_eq!(words[4], aabb);
        // h1 = (1.5 + 0.5) * 0.5 = 1, h2 = 0.5 * 0.25.
        let to_bits = |values: [f32; 4]| values.map(f32::to_bits);
        assert_eq!(words[5..9], to_bits([0.5, 1.0, 0.125, 0.0]));
        assert_eq!(words[9..13], to_bits([0.0, -1.0, 0.0, 0.0]));
    }

    #[test]
    fn the_wheel_state_follows_the_chassis_and_the_suspension() {
        let mut e = engine();
        let log = new_log();
        let vehicle = new_instance(&mut e, 1);
        let record = info(&e, vehicle, 0);
        set_lanes(&mut e, record + 0x50, [1.0, 2.0, 3.0, 4.0]);
        set_lanes(&mut e, record + 0x80, [0.5, 1.0, 2.0, 4.0]);
        let entry = suspension(&e, vehicle, 0);
        e.mem.set_f32(entry + 0x20, 2.0);
        let (this, states) = collider(&mut e, &log, &[0]);
        log_double(&mut e, &log, ROTATION_SET, 0);
        log_double(&mut e, &log, QUATERNION_SET_MUL, 0);
        e.call(UPDATE_WHEEL_STATE, &args![this, vehicle, 0u32]);
        let body = e.mem.u32(vehicle + 0x18);
        let rotations = calls(&log, ROTATION_SET);
        assert_eq!(rotations.len(), 1);
        let quaternion = rotations[0][1];
        assert_eq!(rotations[0][0], states + 0x10);
        assert_eq!(
            calls(&log, QUATERNION_SET_MUL),
            vec![vec![quaternion, body + 0x160, record + 0xb0]]
        );
        assert_eq!(lanes(&e, states + 0x40), [1.0, 2.0, 3.0, 4.0]);
        // 2 * (0.5, 1, 2, 4) + (1, 2, 3, 4).
        assert_eq!(lanes(&e, states + 0x50), [2.0, 4.0, 7.0, 12.0]);
    }

    #[test]
    fn adding_to_the_world_adds_every_phantom() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, ADD_PHANTOM, 0);
        let (this, _) = collider(&mut e, &log, &[0x11, 0x22, 0x33]);
        e.call(0x00e3_6870, &args![this, 0x9000u32]);
        assert_eq!(
            calls(&log, ADD_PHANTOM),
            vec![vec![0x9000, 0x11], vec![0x9000, 0x22], vec![0x9000, 0x33]]
        );
        let (none, _) = collider(&mut e, &log, &[]);
        e.call(0x00e3_6870, &args![none, 0x9000u32]);
        assert_eq!(calls(&log, ADD_PHANTOM).len(), 3);
    }

    #[test]
    fn removing_from_the_world_uses_each_phantoms_world() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, REMOVE_PHANTOM, 0);
        let phantoms = [e.mem.alloc(0x20), e.mem.alloc(0x20)];
        e.mem.set_u32(phantoms[0] + 8, 0x7001);
        e.mem.set_u32(phantoms[1] + 8, 0x7002);
        let (this, _) = collider(&mut e, &log, &phantoms);
        e.call(0x00e3_68a0, &args![this]);
        assert_eq!(
            calls(&log, REMOVE_PHANTOM),
            vec![vec![0x7001, phantoms[0]], vec![0x7002, phantoms[1]]]
        );
    }

    #[test]
    fn the_filter_info_goes_to_every_existing_phantom() {
        let mut e = engine();
        let log = new_log();
        let phantoms = [e.mem.alloc(0x40), 0, e.mem.alloc(0x40)];
        let (this, _) = collider(&mut e, &log, &phantoms);
        e.call(0x00e3_68d0, &args![this, 0xcafe_f00du32]);
        assert_eq!(e.mem.u32(this + 0xc), 0xcafe_f00d);
        assert_eq!(e.mem.u32(phantoms[0] + 0x2c), 0xcafe_f00d);
        assert_eq!(e.mem.u32(phantoms[2] + 0x2c), 0xcafe_f00d);
    }

    #[test]
    fn centering_the_contact_point_moves_it_through_the_wheel_transform() {
        let mut e = engine();
        let log = new_log();
        let sink = Rc::clone(&log);
        e.register_double(SET_TRANSFORMED_INVERSE_POS, move |e, a| {
            sink.borrow_mut()
                .push((SET_TRANSFORMED_INVERSE_POS, a.to_vec()));
            set_lanes(e, a[0], [1.0, 2.0, 3.0, 4.0]);
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(SET_TRANSFORMED_POS, move |e, a| {
            sink.borrow_mut().push((SET_TRANSFORMED_POS, a.to_vec()));
            let input = lanes(e, a[2]);
            set_lanes(
                e,
                a[0],
                [2.0 * input[0], 2.0 * input[1], 2.0 * input[2], 9.0],
            );
            Ret::default()
        });
        let (this, states) = collider(&mut e, &log, &[0, 0]);
        let point = e.mem.alloc(0x10);
        // Wheel 1; the unused word and the wheel word's upper bytes are ignored.
        e.call(
            CENTER_WHEEL_CONTACT_POINT,
            &args![this, 0xdeadu32, 0x0f01u32, point],
        );
        // The inverse transform's lane 2 is cleared before the point is transformed back.
        assert_eq!(lanes(&e, point), [2.0, 4.0, 0.0, 9.0]);
        let transform = states + 0x60 + 0x10;
        let inverse = calls(&log, SET_TRANSFORMED_INVERSE_POS);
        assert_eq!(inverse.len(), 1);
        assert_eq!(inverse[0][1..], [transform, point]);
        let forward = calls(&log, SET_TRANSFORMED_POS);
        assert_eq!(forward.len(), 1);
        assert_eq!(forward[0][1..], [transform, inverse[0][0]]);
        assert_ne!(forward[0][0], inverse[0][0]);
    }

    #[test]
    fn the_wheel_collide_callback_jumps_to_the_centering() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, CENTER_WHEEL_CONTACT_POINT, 0);
        e.call(0x00e3_6aa0, &args![0x10u32, 0x20u32, 0x30u32, 0x40u32]);
        assert_eq!(
            calls(&log, CENTER_WHEEL_CONTACT_POINT),
            vec![vec![0x10, 0x20, 0x30, 0x40]]
        );
    }

    #[test]
    fn the_pre_collision_update_refreshes_state_box_and_phantom_per_wheel() {
        let mut e = engine();
        let log = new_log();
        for address in [UPDATE_WHEEL_STATE, CALC_AABB_OF_WHEEL, SET_AABB] {
            log_double(&mut e, &log, address, 0);
        }
        let (this, _) = collider(&mut e, &log, &[0x11, 0x22]);
        e.call(0x00e3_6950, &args![this, 0x6000u32]);
        let order: Vec<u32> = log.borrow().iter().map(|(a, _)| *a).collect();
        assert_eq!(
            order,
            vec![
                UPDATE_WHEEL_STATE,
                CALC_AABB_OF_WHEEL,
                SET_AABB,
                UPDATE_WHEEL_STATE,
                CALC_AABB_OF_WHEEL,
                SET_AABB
            ]
        );
        let boxes = calls(&log, CALC_AABB_OF_WHEEL);
        assert_eq!(boxes[0][..3], [this, 0x6000, 0]);
        assert_eq!(boxes[1][..3], [this, 0x6000, 1]);
        assert_eq!(boxes[0][3], boxes[1][3]);
        assert_eq!(
            calls(&log, UPDATE_WHEEL_STATE),
            vec![vec![this, 0x6000, 0], vec![this, 0x6000, 1]]
        );
        assert_eq!(
            calls(&log, SET_AABB),
            vec![vec![0x11, boxes[0][3]], vec![0x22, boxes[0][3]]]
        );
    }

    /// What `castSingleWheel` did in `run_cast`.
    struct CastRun {
        e: Engine,
        log: Log,
        states: u32,
        found: u32,
        hit: u32,
        returned: u32,
    }

    /// Runs `castSingleWheel` on wheel 1 of two while the linear cast answers `hit_found`.
    fn run_cast(hit_found: bool) -> CastRun {
        let mut e = engine();
        let log = new_log();
        e.mem.set_u32(COLLECTOR_DISTANCE, 0x7f7f_ffee);
        let (this, states) = collider(&mut e, &log, &[0, 0x7a7a]);
        e.mem.set_u32(this + 0xc, 0xabcd);
        e.mem.set_f32(this + 0x2c, 0.125);
        e.mem.set_f32(this + 0x30, 0.5);
        let state = states + 0x60;
        e.mem.set_u32(state + 4, 0x7777);
        set_lanes(&mut e, state + 0x50, [9.0, 8.0, 7.0, 6.0]);
        log_double(&mut e, &log, COLLIDABLE_CONSTRUCTOR, 0);
        log_double(&mut e, &log, ROOT_CD_POINT_ASSIGN, 0);
        let sink = Rc::clone(&log);
        e.register_double(LINEAR_CAST, move |e, a| {
            let mut words = a.to_vec();
            words.push(e.mem.u32(a[1] + 0x1c));
            words.extend((0..4).map(|i| e.mem.u32(a[2] + 4 * i)));
            words.extend([0x10, 0x14].map(|offset| e.mem.u32(a[2] + offset)));
            words.extend([0, 4, 0x2c, 0x30].map(|offset| e.mem.u32(a[3] + offset)));
            sink.borrow_mut().push((LINEAR_CAST, words));
            if hit_found {
                e.mem.set_u32(a[3] + 0x30, 1);
            }
            Ret::default()
        });
        let found = e.mem.alloc(8);
        e.mem.set_u8(found, 0x55);
        let hit = e.mem.alloc(0x40);
        let returned = e
            .call(0x00e3_69d0, &args![this, found, 0u32, 0xff01u32, hit])
            .u32();
        CastRun {
            e,
            log,
            states,
            found,
            hit,
            returned,
        }
    }

    #[test]
    fn a_wheel_cast_that_hits_copies_the_hit_point() {
        let run = run_cast(true);
        assert_eq!(run.returned, run.found);
        assert_eq!(run.e.mem.u8(run.found), 1);
        let state = run.states + 0x60;
        let seen = calls(&run.log, LINEAR_CAST);
        assert_eq!(seen.len(), 1);
        let words = &seen[0];
        let (collidable, input, collector) = (words[1], words[2], words[3]);
        assert_eq!(words[0], 0x7a7a);
        assert_eq!(words[4], 0);
        assert_eq!(collidable, input + 0xa0);
        assert_eq!(collector, input + 0x20);
        // The collidable's filter word, the input (the target, then the two floats) and the
        // collector (vtable, distance twice, hit flag before the cast).
        assert_eq!(
            words[5..],
            [
                0xabcd,
                9.0f32.to_bits(),
                8.0f32.to_bits(),
                7.0f32.to_bits(),
                6.0f32.to_bits(),
                0.125f32.to_bits(),
                0.5f32.to_bits(),
                COLLECTOR_VTABLE,
                0x7f7f_ffee,
                0x7f7f_ffee,
                0
            ]
        );
        assert_eq!(
            calls(&run.log, COLLIDABLE_CONSTRUCTOR),
            vec![vec![collidable, 0x7777, state + 0x10, 0]]
        );
        assert_eq!(
            calls(&run.log, ROOT_CD_POINT_ASSIGN),
            vec![vec![run.hit, collector + 0x10]]
        );
    }

    #[test]
    fn a_wheel_cast_that_misses_reports_nothing() {
        let run = run_cast(false);
        assert_eq!(run.returned, run.found);
        // The byte held 0x55 before the cast and is overwritten with 0.
        assert_eq!(run.e.mem.u8(run.found), 0);
        assert!(calls(&run.log, ROOT_CD_POINT_ASSIGN).is_empty());
    }

    #[test]
    fn getting_the_phantoms_grows_a_full_array() {
        let mut e = engine();
        let log = new_log();
        let allocator = install_allocator(&mut e, &log, &[0]);
        let (this, _) = collider(&mut e, &log, &[0x111, 0x222]);
        let array = e.mem.alloc(0xc);
        let data = e.mem.alloc(4);
        e.mem.set_u32(data, 0x99);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, 1);
        e.mem.set_u32(array + 8, 1);
        let sink = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            sink.borrow_mut().push((ARRAY_RESERVE_MORE, a.to_vec()));
            let array = a[1];
            let size = e.mem.u32(array + 4);
            let old = e.mem.u32(array);
            let new = e.mem.alloc(0x40);
            for i in 0..size {
                let word = e.mem.u32(old + 4 * i);
                e.mem.set_u32(new + 4 * i, word);
            }
            e.mem.set_u32(array, new);
            e.mem.set_u32(array + 8, 8);
            Ret::default()
        });
        e.call(0x00e3_6ab0, &args![this, array]);
        assert_eq!(e.mem.u32(array + 4), 3);
        let data = e.mem.u32(array);
        let words: Vec<u32> = (0..3).map(|i| e.mem.u32(data + 4 * i)).collect();
        assert_eq!(words, vec![0x99, 0x111, 0x222]);
        // Only the first append found the array full.
        assert_eq!(
            calls(&log, ARRAY_RESERVE_MORE),
            vec![vec![allocator, array, 4]]
        );
    }

    #[test]
    fn the_constructor_sets_the_vtables_and_the_defaults() {
        let mut e = engine();
        // A signalling NaN in the exe's float is quieted by the copy.
        e.mem.set_u32(INITIAL_TOLERANCE, 0x7f80_0001);
        let this = e.mem.alloc(0x40);
        let returned = e.call(CONSTRUCT, &args![this]).u32();
        assert_eq!(returned, this);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 0x18), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u16(this + 0x22), 1);
        assert_eq!(e.mem.u32(this + 0x1c), LISTENER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x24), LISTENER_BASE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x2c), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 0x30), 0x7fc0_0001);
        assert_eq!(e.mem.u8(this + 8), 0);
        assert_eq!(e.mem.u8(this + 9), 2);
    }

    #[test]
    fn the_destructor_releases_shapes_and_phantoms_and_frees_the_array() {
        let mut e = engine();
        let log = new_log();
        let allocator = install_allocator(&mut e, &log, &[0]);
        for address in [
            REMOVE_REFERENCE,
            REMOVE_OVERLAP_LISTENER,
            LISTENER_DESTRUCTOR,
        ] {
            log_double(&mut e, &log, address, 0);
        }
        let phantoms = [e.mem.alloc(0x20), e.mem.alloc(0x20)];
        // Only the first phantom has a memory size, so only it loses the listener.
        e.mem.set_u16(phantoms[0] + 4, 0xe0);
        let (this, states) = collider(&mut e, &log, &phantoms);
        e.mem.set_u32(states + 4, 0xa1);
        e.mem.set_u32(states + 0x60 + 4, 0xa2);
        e.mem.set_u16(this + 4, 0x34);
        e.mem.set_u32(this + 0x18, 2);
        e.call(0x00e3_6b80, &args![this]);
        let order = log.borrow().clone();
        assert_eq!(
            order,
            vec![
                (REMOVE_REFERENCE, vec![0xa1]),
                (REMOVE_OVERLAP_LISTENER, vec![phantoms[0], this + 0x24]),
                (REMOVE_REFERENCE, vec![phantoms[0]]),
                (REMOVE_REFERENCE, vec![0xa2]),
                (REMOVE_REFERENCE, vec![phantoms[1]]),
                (LISTENER_DESTRUCTOR, vec![this + 0x1c]),
                (ALLOCATOR_BASE + 0x10, vec![allocator, states, 2 * 0x60]),
            ]
        );
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0x8000_0000);
        // Storage that is not owned (bit 31 of the capacity word) is not freed.
        let (borrowed, _) = collider(&mut e, &log, &[]);
        e.mem.set_u32(borrowed + 0x18, 0x8000_0002);
        log.borrow_mut().clear();
        e.call(0x00e3_6b80, &args![borrowed]);
        let order = log.borrow().clone();
        assert_eq!(order, vec![(LISTENER_DESTRUCTOR, vec![borrowed + 0x1c])]);
        assert_eq!(e.mem.u32(borrowed), BASE_VTABLE);
    }

    #[test]
    fn init_makes_a_shape_and_a_phantom_for_every_wheel() {
        let mut e = engine();
        let log = new_log();
        let blocks = [e.mem.alloc(0xe0), e.mem.alloc(0xe0)];
        let allocator = install_allocator(&mut e, &log, &blocks);
        let vehicle = new_instance(&mut e, 2);
        for wheel in 0..2u32 {
            let at = params(&e, vehicle, wheel);
            e.mem.set_f32(at, 0.5 + wheel as f32);
            e.mem.set_f32(at + 8, 0.25 + wheel as f32);
        }
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x18, 0x8000_0000);
        e.mem.set_u32(this + 0xc, 0xabcd);
        let states = e.mem.alloc(0xc0);
        let sink = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            sink.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            e.mem.set_u32(a[1], states);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let sink = Rc::clone(&log);
        let mut made = 0u32;
        e.register_double(CREATE_WHEEL_SHAPE, move |_, a| {
            sink.borrow_mut().push((CREATE_WHEEL_SHAPE, a.to_vec()));
            made += 1;
            Ret {
                eax: 0x5100 + made,
                ..Ret::default()
            }
        });
        for address in [UPDATE_WHEEL_STATE, CALC_AABB_OF_WHEEL, ADD_OVERLAP_LISTENER] {
            log_double(&mut e, &log, address, 0);
        }
        let sink = Rc::clone(&log);
        e.register_double(AABB_PHANTOM_CONSTRUCTOR, move |_, a| {
            sink.borrow_mut()
                .push((AABB_PHANTOM_CONSTRUCTOR, a.to_vec()));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.call(0x00e3_6cc0, &args![this, vehicle]);
        assert_eq!(e.mem.i32(this + 0x14), 2);
        assert_eq!(
            calls(&log, ARRAY_RESERVE),
            vec![vec![allocator, this + 0x10, 2, 0x60]]
        );
        // The width comes first: (this, width, radius).
        assert_eq!(
            calls(&log, CREATE_WHEEL_SHAPE),
            vec![
                vec![this, 0.25f32.to_bits(), 0.5f32.to_bits()],
                vec![this, 1.25f32.to_bits(), 1.5f32.to_bits()]
            ]
        );
        assert_eq!(e.mem.u32(states + 4), 0x5101);
        assert_eq!(e.mem.u32(states + 0x60 + 4), 0x5102);
        assert_eq!(e.mem.u32(states), blocks[0]);
        assert_eq!(e.mem.u32(states + 0x60), blocks[1]);
        assert_eq!(e.mem.u16(blocks[0] + 4), 0xe0);
        assert_eq!(e.mem.u16(blocks[1] + 4), 0xe0);
        let chassis = e.mem.u32(vehicle + 0x18);
        assert_eq!(e.mem.u32(this + 0x28), chassis + 0x10);
        let boxes = calls(&log, CALC_AABB_OF_WHEEL);
        assert_eq!(boxes.len(), 2);
        assert_eq!(boxes[0][..3], [this, vehicle, 0]);
        assert_eq!(boxes[1][..3], [this, vehicle, 1]);
        assert_eq!(
            calls(&log, AABB_PHANTOM_CONSTRUCTOR),
            vec![
                vec![blocks[0], boxes[0][3], 0xabcd],
                vec![blocks[1], boxes[1][3], 0xabcd]
            ]
        );
        assert_eq!(
            calls(&log, UPDATE_WHEEL_STATE),
            vec![vec![this, vehicle, 0], vec![this, vehicle, 1]]
        );
        assert_eq!(
            calls(&log, ADD_OVERLAP_LISTENER),
            vec![vec![blocks[0], this + 0x24], vec![blocks[1], this + 0x24]]
        );
        assert_eq!(calls(&log, ALLOCATOR_BASE + 4).len(), 2);
        // A second call finds the states and does not make the shapes again.
        e.call(0x00e3_6cc0, &args![this, vehicle]);
        assert_eq!(calls(&log, ARRAY_RESERVE).len(), 1);
        assert_eq!(calls(&log, CREATE_WHEEL_SHAPE).len(), 2);
    }

    #[test]
    fn cloning_moves_the_wheel_states_onto_the_new_phantoms() {
        let mut e = engine();
        let log = new_log();
        e.mem.set_u32(INITIAL_TOLERANCE, 0x3400_0000);
        let block = e.mem.alloc(0x34);
        let allocator = install_allocator(&mut e, &log, &[block]);
        for address in [ADD_REFERENCE, REMOVE_OVERLAP_LISTENER, ADD_OVERLAP_LISTENER] {
            log_double(&mut e, &log, address, 0);
        }
        let new_states = e.mem.alloc(0xc0);
        let sink = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            sink.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            e.mem.set_u32(a[1], new_states);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let (this, states) = collider(&mut e, &log, &[0xaa, 0xbb]);
        e.mem.set_u32(states + 4, 0xc1);
        e.mem.set_u32(states + 0x60 + 4, 0xc2);
        e.mem.set_u32(this + 0xc, 0x1234);
        e.mem.set_u32(this + 0x2c, 0x7f80_0001);
        e.mem.set_u32(this + 0x30, 0x3f00_0000);
        let phantom_data = e.mem.alloc(8);
        e.mem.set_u32(phantom_data, 0x601);
        e.mem.set_u32(phantom_data + 4, 0x602);
        let phantoms = e.mem.alloc(0xc);
        e.mem.set_u32(phantoms, phantom_data);
        let clone = e.call(0x00e3_6e20, &args![this, 0x7000u32, phantoms]).u32();
        assert_eq!(clone, block);
        assert_eq!(e.mem.u16(block + 4), 0x34);
        assert_eq!(e.mem.u32(block), VTABLE);
        assert_eq!(e.mem.i32(clone + 0x14), 2);
        assert_eq!(
            calls(&log, ARRAY_RESERVE),
            vec![vec![allocator, clone + 0x10, 2, 0x60]]
        );
        assert_eq!(e.mem.u32(new_states + 4), 0xc1);
        assert_eq!(e.mem.u32(new_states + 0x60 + 4), 0xc2);
        assert_eq!(e.mem.u32(new_states), 0x601);
        assert_eq!(e.mem.u32(new_states + 0x60), 0x602);
        assert_eq!(
            calls(&log, ADD_REFERENCE),
            vec![vec![0xc1], vec![0x601], vec![0xc2], vec![0x602]]
        );
        assert_eq!(
            calls(&log, REMOVE_OVERLAP_LISTENER),
            vec![vec![0x601, this + 0x24], vec![0x602, this + 0x24]]
        );
        assert_eq!(
            calls(&log, ADD_OVERLAP_LISTENER),
            vec![vec![0x601, clone + 0x24], vec![0x602, clone + 0x24]]
        );
        assert_eq!(e.mem.u32(clone + 0x28), 0x7010);
        assert_eq!(e.mem.u32(clone + 0xc), 0x1234);
        assert_eq!(e.mem.u32(clone + 0x2c), 0x7fc0_0001);
        assert_eq!(e.mem.u32(clone + 0x30), 0x3f00_0000);
    }
}
