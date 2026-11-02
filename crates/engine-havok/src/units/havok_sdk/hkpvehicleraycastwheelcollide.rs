//! `Havok SDK/hkpvehicleraycastwheelcollide.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpconvexradiusbuilder::{
    addps, mulps, shufps, splat, subps, sum_of_three,
};
use crate::units::havok_sdk::hkpsphereshape::{
    copy_vec, load_bits, load_vec, maxps, minps, store_bits, store_vec, zip_with, SIGN_MASK,
};

// hkpVehicleRayCastWheelCollide has no type in the Xbox PDB, so its layout is described by the
// offsets the PC code uses (roles unconfirmed): `+0xc` the collision filter info, `+0x10` the
// `hkpAabbPhantom` used for the casts, `+0x14` the embedded `hkpRejectChassisListener` (a
// phantom overlap listener, `+0x1c` the address the phantom is given) and `+0x20` the chassis
// collidable. The vehicle (`hkpVehicleInstance`) keeps its wheel records at `+0x48` (`0xe0`
// bytes each: the ray start at `+0x50`, the ray end at `+0x60`, the ray direction at `+0x80`)
// and the wheel count (signed byte) at `[+0x1c] + 0x20`.

/// Memory router (`00c85750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkpWorld::addPhantom` (`ECX` the world, then the phantom).
const WORLD_ADD_PHANTOM: u32 = 0x00c9_36a0;
/// `hkpWorld::removePhantom` (`ECX` the world, then the phantom).
const WORLD_REMOVE_PHANTOM: u32 = 0x00c9_2c70;
/// `hkpAabbPhantom::setAabb` (`ECX` the phantom, then the AABB).
const PHANTOM_SET_AABB: u32 = 0x00cf_24a0;
/// `hkpAabbPhantom::castRay` (`ECX` the phantom, then the ray input and the output).
const PHANTOM_CAST_RAY: u32 = 0x00cf_26b0;
/// `hkpAabbPhantom::hkpAabbPhantom` (`ECX` the memory, then the AABB and the filter info).
const PHANTOM_CONSTRUCT: u32 = 0x00cf_2cb0;
/// `hkpPhantom::addPhantomOverlapListener` (`ECX` the phantom, then the listener).
const ADD_OVERLAP_LISTENER: u32 = 0x00d0_7de0;
/// `hkpPhantom::removePhantomOverlapListener` (`ECX` the phantom, then the listener).
const REMOVE_OVERLAP_LISTENER: u32 = 0x00d0_7c80;
/// Adds a reference to the object in `ECX` (role unconfirmed).
const ADD_REFERENCE: u32 = 0x00c9_0510;
/// Removes a reference from the object in `ECX` (`removeReference`).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkpRejectChassisListener::~hkpRejectChassisListener` (`ECX` the listener).
const REJECT_CHASSIS_LISTENER_DESTRUCTOR: u32 = 0x00e3_60f0;
/// `hkArray` grow (cdecl): the allocator, the array and the element size.
const ARRAY_GROW: u32 = 0x00c9_0940;
/// The vtable of this class and of the base the destructor ends with.
const WHEEL_COLLIDE_VTABLE: u32 = 0x010e_5ae4;
const WHEEL_COLLIDE_BASE_VTABLE: u32 = 0x0102_e388;
/// A `double` constant (`.rdata`, `-1.0`) the hit branch of
/// `getCollisionOutputFromCastResult` divides by the approach speed.
const NEGATIVE_ONE: u32 = 0x0101_a6b0;

/// The address handed to the phantom as this object's overlap listener (`this + 0x1c`, or null
/// when `this + 0x14` wraps to zero).
fn listener_address(this: u32) -> u32 {
    if this.wrapping_add(0x14) != 0 {
        this.wrapping_add(0x1c)
    } else {
        0
    }
}

// Translated from 00e41e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleRayCastWheelCollide::updateBeforeCollisionDetection` (Xbox PDB name, engine map).
/// Thiscall, one stack word (`RET 4`): `vehicle`. Has slot `0x34` (`calcWheelsAABB`) fill a local
/// AABB for the vehicle and gives it to the phantom (`00cf24a0`).
pub fn hkp_vehicle_ray_cast_wheel_collide_update_before_collision_detection(
    e: &mut Engine,
    this: Ptr,
    vehicle: u32,
) {
    let t = this.addr();
    e.with_stack(0x30, |e, frame| {
        let aabb = frame.addr();
        e.vcall(t, 0x34, &args![vehicle, aabb]);
        let phantom = e.mem.u32(t + 0x10);
        e.call(PHANTOM_SET_AABB, &args![phantom, aabb]);
    });
}

// Translated from 00e41eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleRayCastWheelCollide::castSingleWheel` (Xbox PDB name, engine map). Thiscall, two
/// stack words (`RET 8`): `wheel` (a wheel record) and `output`. Builds the ray input on the
/// stack (the ray start and end copied from the wheel record `+0x50` and `+0x60`, the byte at
/// `+0x20` set to `1`, the filter info at `+0x24` from `this + 0xc`) and casts it with the
/// phantom (`00cf26b0`).
pub fn hkp_vehicle_ray_cast_wheel_collide_cast_single_wheel(
    e: &mut Engine,
    this: Ptr,
    wheel: u32,
    output: u32,
) {
    let t = this.addr();
    e.with_stack(0x30, |e, frame| {
        let input = frame.addr();
        copy_vec(e, wheel + 0x50, input);
        copy_vec(e, wheel + 0x60, input + 0x10);
        let filter = e.mem.u32(t + 0xc);
        e.mem.set_u32(input + 0x24, filter);
        e.mem.set_u8(input + 0x20, 1);
        let phantom = e.mem.u32(t + 0x10);
        e.call(PHANTOM_CAST_RAY, &args![phantom, input, output]);
    });
}

// Translated from 00e41ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleRayCastWheelCollide::addToWorld` (Xbox PDB name, engine map). Fastcall, one stack
/// word (`RET 4`): `world`. Adds the phantom (`this + 0x10`) to the world (`00c936a0`).
pub fn hkp_vehicle_ray_cast_wheel_collide_add_to_world(e: &mut Engine, this: Ptr, world: u32) {
    let phantom = e.mem.u32(this.addr() + 0x10);
    e.call(WORLD_ADD_PHANTOM, &args![world, phantom]);
}

// Translated from 00e41f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleRayCastWheelCollide::removeFromWorld` (Xbox PDB name, engine map). Fastcall, no
/// stack word (plain `RET`). Removes the phantom (`this + 0x10`) from the world it is in (the
/// word at phantom `+8`) with `00c92c70`.
pub fn hkp_vehicle_ray_cast_wheel_collide_remove_from_world(e: &mut Engine, this: Ptr) {
    let phantom = e.mem.u32(this.addr() + 0x10);
    let world = e.mem.u32(phantom + 8);
    e.call(WORLD_REMOVE_PHANTOM, &args![world, phantom]);
}

// Translated from 00e41f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleRayCastWheelCollide::setCollisionFilterInfo` (Xbox PDB name, engine map). Thiscall,
/// one stack word (`RET 4`): `info`. Stores it at `+0xc` and, when there is a phantom
/// (`+0x10`), at phantom `+0x2c` too.
pub fn hkp_vehicle_ray_cast_wheel_collide_set_collision_filter_info(
    e: &mut Engine,
    this: Ptr,
    info: u32,
) {
    let t = this.addr();
    e.mem.set_u32(t + 0xc, info);
    let phantom = e.mem.u32(t + 0x10);
    if phantom != 0 {
        e.mem.set_u32(phantom + 0x2c, info);
    }
}

// Translated from 00e41f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleRayCastWheelCollide::init` (Xbox PDB name, engine map). Thiscall, one stack word
/// (`RET 4`): `vehicle`. Has slot `0x34` (`calcWheelsAABB`) fill a local AABB, takes `0xe0`
/// bytes (header word `+4` set to `0xe0`) from the allocator (slot 4 of `[router + 0x10]`),
/// constructs the phantom in it (`00cf2cb0`: the AABB and `this + 0xc`) as `this + 0x10`, sets
/// `this + 0x20` to the vehicle's chassis (`+0x18`) plus `0x10` and adds this object's
/// listener to the phantom (`00d07de0`).
pub fn hkp_vehicle_ray_cast_wheel_collide_init(e: &mut Engine, this: Ptr, vehicle: u32) {
    let t = this.addr();
    e.with_stack(0x30, |e, frame| {
        let aabb = frame.addr();
        e.vcall(t, 0x34, &args![vehicle, aabb]);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let block = e.vcall(allocator, 4, &args![0xe0u32]).u32();
        e.mem.set_u16(block + 4, 0xe0);
        let filter = e.mem.u32(t + 0xc);
        let phantom = e.call(PHANTOM_CONSTRUCT, &args![block, aabb, filter]).u32();
        e.mem.set_u32(t + 0x10, phantom);
        let chassis = e.mem.u32(vehicle + 0x18);
        e.mem.set_u32(t + 0x20, chassis.wrapping_add(0x10));
        e.call(ADD_OVERLAP_LISTENER, &args![phantom, listener_address(t)]);
    });
}

// Translated from 00e41fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleRayCastWheelCollide::~hkpVehicleRayCastWheelCollide` (Xbox PDB name, engine map).
/// Fastcall, no stack word (plain `RET`). Puts the vtable (`010e5ae4`) back; when there is a
/// phantom (`+0x10`) removes this object's listener from it (`00d07c80`) if the chassis
/// (`+0x20`) is set and releases it (`00c905b0`); destroys the embedded listener at `+0x14`
/// (`00e360f0`) and ends with the base vtable (`0102e388`).
pub fn hkp_vehicle_ray_cast_wheel_collide_destructor(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.mem.set_u32(t, WHEEL_COLLIDE_VTABLE);
    let phantom = e.mem.u32(t + 0x10);
    if phantom != 0 {
        if e.mem.u32(t + 0x20) != 0 {
            e.call(
                REMOVE_OVERLAP_LISTENER,
                &args![phantom, listener_address(t)],
            );
        }
        e.call(REMOVE_REFERENCE, &args![phantom]);
    }
    e.call(
        REJECT_CHASSIS_LISTENER_DESTRUCTOR,
        &args![t.wrapping_add(0x14)],
    );
    e.mem.set_u32(t, WHEEL_COLLIDE_BASE_VTABLE);
}

// Translated from 00e42010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleRayCastWheelCollide::calcWheelsAABB` (Xbox PDB name, engine map). Two stack words
/// (`RET 8`): `vehicle` and `aabb` (`ECX` is not read). Writes the bounds of every wheel's ray
/// start and end to the AABB: the lane-wise `MINPS` at `+0` and `MAXPS` at `+0x10`, starting
/// from the first wheel and folding in the others (the wheel count is a signed byte).
pub fn hkp_vehicle_ray_cast_wheel_collide_calc_wheels_aabb(
    e: &mut Engine,
    _this: Ptr,
    vehicle: u32,
    aabb: u32,
) {
    let first = e.mem.u32(vehicle + 0x48);
    let start = load_vec(e, first + 0x50);
    let end = load_vec(e, first + 0x60);
    let mut low = zip_with(start, end, minps);
    let mut high = zip_with(start, end, maxps);
    store_vec(e, aabb, low);
    store_vec(e, aabb + 0x10, high);
    let wheels = e.mem.i8(e.mem.u32(vehicle + 0x1c) + 0x20);
    if wheels > 1 {
        let mut index = 1i32;
        let mut offset = 0xe0u32;
        loop {
            let info = e.mem.u32(vehicle + 0x48).wrapping_add(offset);
            low = zip_with(low, load_vec(e, info + 0x60), minps);
            store_vec(e, aabb, low);
            low = zip_with(low, load_vec(e, info + 0x50), minps);
            store_vec(e, aabb, low);
            high = zip_with(high, load_vec(e, info + 0x60), maxps);
            store_vec(e, aabb + 0x10, high);
            high = zip_with(high, load_vec(e, info + 0x50), maxps);
            store_vec(e, aabb + 0x10, high);
            index += 1;
            offset = offset.wrapping_add(0xe0);
            let wheels = e.mem.i8(e.mem.u32(vehicle + 0x1c) + 0x20);
            if index >= i32::from(wheels) {
                break;
            }
        }
    }
}

// Translated from 00e420c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleRayCastWheelCollide::collideWheels` (Xbox PDB name, engine map). Thiscall, three
/// stack words (`RET 0xc`): `_unused_1` (not read), `vehicle` and `outputs` (an array of
/// `0x60`-byte collision outputs). For every wheel (the byte at `[vehicle + 0x1c] + 0x20`,
/// unsigned here) it clears a `0x60`-byte ray output (hit fraction `1.0` at `+0x10`, `-1` at
/// `+0x14` and `+0x20`, `0` at `+0x40` and `+0x50`), has slot `0x38` (`castSingleWheel`) cast the
/// wheel's ray into it, then calls slot `0x3c` (`getCollisionOutputFromCastResult`: the
/// vehicle, the wheel index, the ray output, the collision output) when the root collidable at
/// `+0x50` of the ray output is set, else slot `0x40` (`getCollisionOutputWithoutHit`: the
/// vehicle, the index, the collision output), and finally slot `0x2c` (the vehicle, the index,
/// the collision output). The index is pushed as a whole stack dword whose upper three bytes
/// are never written by the exe; the callees read only the low byte (slot `0x2c` is a
/// `RET 0xc` stub), so they are passed as zero.
pub fn hkp_vehicle_ray_cast_wheel_collide_collide_wheels(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    vehicle: u32,
    outputs: u32,
) {
    let t = this.addr();
    let wheels = e.mem.u8(e.mem.u32(vehicle + 0x1c) + 0x20);
    if wheels == 0 {
        return;
    }
    e.with_stack(0x60, |e, frame| {
        let ray = frame.addr();
        let mut index = 0u8;
        let mut offset = 0u32;
        let mut output = outputs;
        loop {
            e.mem.set_u32(ray + 0x10, 0x3f80_0000);
            e.mem.set_u32(ray + 0x14, 0xffff_ffff);
            e.mem.set_u32(ray + 0x20, 0xffff_ffff);
            e.mem.set_u32(ray + 0x40, 0);
            e.mem.set_u32(ray + 0x50, 0);
            let info = e.mem.u32(vehicle + 0x48).wrapping_add(offset);
            e.vcall(t, 0x38, &args![info, ray]);
            let word = u32::from(index);
            if e.mem.u32(ray + 0x50) == 0 {
                e.vcall(t, 0x40, &args![vehicle, word, output]);
            } else {
                e.vcall(t, 0x3c, &args![vehicle, word, ray, output]);
            }
            e.vcall(t, 0x2c, &args![vehicle, word, output]);
            offset = offset.wrapping_add(0xe0);
            index = index.wrapping_add(1);
            output = output.wrapping_add(0x60);
            if index >= wheels {
                break;
            }
        }
    });
}

// Translated from 00e42190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleRayCastWheelCollide::getCollisionOutputFromCastResult` (Xbox PDB name, engine map).
/// Four stack words (`RET 0x10`; `ECX` is not read): `vehicle`, `wheel` (the low byte is the
/// wheel index), `cast` (the ray output) and `output` (the `0x60`-byte collision output).
/// With the vehicle's per-wheel rest length `rest` (`[[vehicle + 0x34] + 8] + 0x20 + 0x30 *
/// index`), the wheel parameter `base` (`[data + 0x8c] + 0x28 * index`) and the hit fraction
/// `f` (`cast + 0x10`): `output + 0x10` takes the first three lanes of the ray output's normal
/// (lane 3 kept), `+0x28 .. +0x48` copies `cast + 0x20 .. +0x40`, `+0x24` the body owning the
/// root collidable (`cast + 0x50`: the collidable plus its signed byte at `+0x10` when its
/// byte at `+0x18` is `1`, else null), `+0x48` and `+0x1c` the contact distance
/// `((base + rest) * f) - base`, `+0` the hit position `scaled * direction + start` (the wheel
/// record's `+0x80` and `+0x50`; `scaled` the product before the subtraction), `+0x20` the
/// owner's float at `+0x90`. When the dot product of the normal with the direction is below
/// minus the data block's `+0x84` threshold the approach speed is worked out from the
/// chassis (`vehicle + 0x18`) and the owner (position `+0x140`, linear velocity `+0x1b0`,
/// angular velocity `+0x1c0`) as `r = -1.0 / dot` (the double at `0101a6b0`):
/// `+0x4c = r * (relative velocity . normal)`, `+0x50 = r`; otherwise `+0x4c = 0` and `+0x50 =
/// 1.0 / threshold`.
pub fn hkp_vehicle_ray_cast_wheel_collide_get_collision_output_from_cast_result(
    e: &mut Engine,
    _this: Ptr,
    vehicle: u32,
    wheel: u32,
    cast: u32,
    output: u32,
) {
    let index = wheel & 0xff;
    let data = e.mem.u32(vehicle + 0x1c);
    let holder = e.mem.u32(vehicle + 0x34);
    let rest_address = e
        .mem
        .u32(holder + 8)
        .wrapping_add(0x20)
        .wrapping_add(index.wrapping_mul(0x30));
    let rest = f32::from_bits(x87_quiet_bits(e.mem.u32(rest_address)));
    let info = e
        .mem
        .u32(vehicle + 0x48)
        .wrapping_add(index.wrapping_mul(0xe0));
    let normal = load_bits(e, cast);
    let tail = e.mem.u32(output + 0x1c);
    store_bits(e, output + 0x10, [normal[0], normal[1], normal[2], tail]);
    for word in 0..8 {
        let value = e.mem.u32(cast + 0x20 + 4 * word);
        e.mem.set_u32(output + 0x28 + 4 * word, value);
    }
    let collidable = e.mem.u32(cast + 0x50);
    let owner = if e.mem.u8(collidable + 0x18) == 1 {
        collidable.wrapping_add(i32::from(e.mem.i8(collidable + 0x10)) as u32)
    } else {
        0
    };
    e.mem.set_u32(output + 0x24, owner);
    let parameters = e.mem.u32(data + 0x8c);
    let base = f32::from_bits(x87_quiet_bits(
        e.mem.u32(parameters.wrapping_add(index.wrapping_mul(0x28))),
    ));
    let fraction = e.mem.f32(cast + 0x10);
    let scaled = ((f64::from(base) + f64::from(rest)) * f64::from(fraction)) as f32;
    let distance = (f64::from(scaled) - f64::from(base)) as f32;
    e.mem.set_f32(output + 0x48, distance);
    let direction = load_vec(e, info + 0x80);
    let start = load_vec(e, info + 0x50);
    let position = addps(mulps(splat(scaled), direction), start);
    store_vec(e, output, position);
    e.mem.set_f32(output + 0x1c, distance);
    let owner_value = x87_quiet_bits(e.mem.u32(owner.wrapping_add(0x90)));
    e.mem.set_u32(output + 0x20, owner_value);
    let normal = load_vec(e, output + 0x10);
    let along = sum_of_three(mulps(normal, direction));
    let threshold = x87_quiet(e.mem.f32(data + 0x84));
    if -f64::from(threshold) > f64::from(along) {
        let ratio = (e.mem.f64(NEGATIVE_ONE) / f64::from(along)) as f32;
        let chassis = e.mem.u32(vehicle + 0x18);
        let from_chassis = subps(position, load_vec(e, chassis + 0x140));
        let from_owner = subps(position, load_vec(e, owner.wrapping_add(0x140)));
        let owner_spin = load_vec(e, owner.wrapping_add(0x1c0));
        let chassis_spin = load_vec(e, chassis + 0x1c0);
        let owner_part = mulps(
            shufps(from_owner, from_owner, 0xc9),
            shufps(owner_spin, owner_spin, 0xd2),
        );
        let owner_other = mulps(
            shufps(from_owner, from_owner, 0xd2),
            shufps(owner_spin, owner_spin, 0xc9),
        );
        let owner_velocity = addps(
            subps(owner_other, owner_part),
            load_vec(e, owner.wrapping_add(0x1b0)),
        );
        let chassis_part = mulps(
            shufps(from_chassis, from_chassis, 0xd2),
            shufps(chassis_spin, chassis_spin, 0xc9),
        );
        let chassis_other = mulps(
            shufps(from_chassis, from_chassis, 0xc9),
            shufps(chassis_spin, chassis_spin, 0xd2),
        );
        let chassis_velocity = addps(
            subps(chassis_part, chassis_other),
            load_vec(e, chassis + 0x1b0),
        );
        let relative = mulps(subps(chassis_velocity, owner_velocity), normal);
        let speed = sum_of_three(relative);
        e.mem
            .set_f32(output + 0x4c, (f64::from(speed) * f64::from(ratio)) as f32);
        e.mem.set_f32(output + 0x50, ratio);
    } else {
        e.mem.set_u32(output + 0x4c, 0);
        let threshold = x87_quiet(e.mem.f32(data + 0x84));
        e.mem
            .set_f32(output + 0x50, (1.0f64 / f64::from(threshold)) as f32);
    }
}

// Translated from 00e423d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleRayCastWheelCollide::getCollisionOutputWithoutHit` (Xbox PDB name, engine map).
/// Three stack words (`RET 0xc`; `ECX` is not read): `vehicle`, `wheel` (the low byte is the
/// wheel index) and `output` (the `0x60`-byte collision output). Writes the no-contact output:
/// `+0` the wheel record's ray end (`+0x60`), `+0x10` its negated direction (`+0x80` with the
/// sign bits flipped; lane 3 kept), `+0x1c` and `+0x48` the rest length (`[[vehicle + 0x34] + 8]
/// + 0x20 + 0x30 * index`), `+0x24`, `+0x20` and `+0x4c` zero, `+0x50` `1.0`.
pub fn hkp_vehicle_ray_cast_wheel_collide_get_collision_output_without_hit(
    e: &mut Engine,
    _this: Ptr,
    vehicle: u32,
    wheel: u32,
    output: u32,
) {
    let index = wheel & 0xff;
    let holder = e.mem.u32(vehicle + 0x34);
    let rest_address = e
        .mem
        .u32(holder + 8)
        .wrapping_add(0x20)
        .wrapping_add(index.wrapping_mul(0x30));
    let rest = x87_quiet_bits(e.mem.u32(rest_address));
    let info = e
        .mem
        .u32(vehicle + 0x48)
        .wrapping_add(index.wrapping_mul(0xe0));
    e.mem.set_u32(output + 0x24, 0);
    e.mem.set_u32(output + 0x48, rest);
    e.mem.set_u32(output + 0x4c, 0);
    copy_vec(e, info + 0x60, output);
    let mask = load_bits(e, SIGN_MASK);
    let direction = load_bits(e, info + 0x80);
    let tail = e.mem.u32(output + 0x1c);
    store_bits(
        e,
        output + 0x10,
        [
            direction[0] ^ mask[0],
            direction[1] ^ mask[1],
            direction[2] ^ mask[2],
            tail,
        ],
    );
    e.mem.set_u32(output + 0x20, 0);
    e.mem.set_u32(output + 0x50, 0x3f80_0000);
    e.mem.set_u32(output + 0x1c, rest);
}

// Translated from 00e42460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleRayCastWheelCollide::buildRayCastCommands` (Xbox PDB name, engine map). Thiscall,
/// five stack words (`RET 0x14`): `vehicle`, `info` (a pointer, possibly null), `argument`,
/// `commands` (`0x50` bytes per wheel) and `outputs` (`0x60` bytes per wheel). Has the phantom
/// (`this + 0x10`) run slot `0x28` (no arguments) and reads its count at `+0xd4`; when that is
/// not positive returns `0`. Otherwise writes one command per wheel: the wheel record's `+0x50`
/// and `+0x60` vectors at `+0` and `+0x10`, `this + 0xc` at `+0x20`, `info + 0x10` (or `0` for a
/// null `info`) at `+0x24`, `[info + 0x20]` at `+0x30`, `argument` at `+0x34`, the phantom's
/// `+0xd0` at `+0x38`, the count at `+0x3c`, the wheel's output at `+0x40`, `1` at `+0x44`,
/// `0` at `+0x48` and the byte at `+0x4c`. Returns the wheel count (a signed byte).
pub fn hkp_vehicle_ray_cast_wheel_collide_build_ray_cast_commands(
    e: &mut Engine,
    this: Ptr,
    vehicle: u32,
    info: u32,
    argument: u32,
    commands: u32,
    outputs: u32,
) -> i32 {
    let t = this.addr();
    let phantom = e.mem.u32(t + 0x10);
    e.vcall(phantom, 0x28, &args![]);
    let count = e.mem.i32(e.mem.u32(t + 0x10) + 0xd4);
    if count < 1 {
        return 0;
    }
    let wheels = e.mem.i8(e.mem.u32(vehicle + 0x1c) + 0x20);
    if wheels > 0 {
        let mut remaining = u32::from(wheels as u8);
        let mut offset = 0u32;
        let mut command = commands;
        let mut output = outputs;
        loop {
            let first = e.mem.u32(vehicle + 0x48);
            copy_vec(e, first.wrapping_add(offset).wrapping_add(0x50), command);
            copy_vec(
                e,
                first.wrapping_add(offset).wrapping_add(0x60),
                command + 0x10,
            );
            let filter = e.mem.u32(t + 0xc);
            e.mem.set_u32(command + 0x20, filter);
            let linked = if info == 0 {
                0
            } else {
                info.wrapping_add(0x10)
            };
            e.mem.set_u32(command + 0x24, linked);
            let word = e.mem.u32(info.wrapping_add(0x20));
            e.mem.set_u32(command + 0x30, word);
            e.mem.set_u32(command + 0x34, argument);
            e.mem.set_u8(command + 0x4c, 0);
            let phantom_word = e.mem.u32(e.mem.u32(t + 0x10) + 0xd0);
            e.mem.set_u32(command + 0x38, phantom_word);
            e.mem.set_u32(command + 0x40, output);
            e.mem.set_u32(command + 0x3c, count as u32);
            e.mem.set_u32(command + 0x44, 1);
            e.mem.set_u32(command + 0x48, 0);
            offset = offset.wrapping_add(0xe0);
            command = command.wrapping_add(0x50);
            output = output.wrapping_add(0x60);
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
    }
    i32::from(wheels)
}

// Translated from 00e42550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialises a freshly allocated `hkpVehicleRayCastWheelCollide` (fastcall, no stack word; the
/// engine map has no name for it): the vtable `010e5ae4` at `+0`, `1` as a 16-bit word at `+6`
/// and `+0x1a`, `0` at `+0xc` and `+0x10`, the listener vtable `010debf4` at `+0x14` and
/// `010debe4` at `+0x1c` (after `010c925c`, the purecall table, which is overwritten at once),
/// the bytes `0` at `+8` and `1` at `+9`. Returns the object.
pub fn fn_00e42550(e: &mut Engine, this: Ptr) -> Ptr {
    let t = this.addr();
    e.mem.set_u16(t + 6, 1);
    e.mem.set_u32(t, WHEEL_COLLIDE_VTABLE);
    e.mem.set_u32(t + 0xc, 0);
    e.mem.set_u32(t + 0x10, 0);
    e.mem.set_u16(t + 0x1a, 1);
    e.mem.set_u32(t + 0x1c, 0x010c_925c);
    e.mem.set_u32(t + 0x14, 0x010d_ebf4);
    e.mem.set_u32(t + 0x1c, 0x010d_ebe4);
    e.mem.set_u8(t + 8, 0);
    e.mem.set_u8(t + 9, 1);
    this
}

// Translated from 00e42590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleRayCastWheelCollide::clone` (Xbox PDB name, engine map). Thiscall, two stack words
/// (`RET 8`): `body` and `source` (a pointer to a pointer to the phantom to share). Takes
/// `0x24` bytes from the allocator (header word `+4` set to `0x24`), initialises them (`fn_00e42550`),
/// sets the copy's phantom (`+0x10`) to `[[source]]`, adds a reference to it (`00c90510`), moves
/// the overlap listener from this object to the copy (`00d07c80`, `00d07de0`), sets the copy's
/// `+0x20` to `body + 0x10` and its filter info (`+0xc`) from this object. Returns the copy.
pub fn hkp_vehicle_ray_cast_wheel_collide_clone(
    e: &mut Engine,
    this: Ptr,
    body: u32,
    source: u32,
) -> Ptr {
    let t = this.addr();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x24u32]).u32();
    e.mem.set_u16(block + 4, 0x24);
    let copy = fn_00e42550(e, Ptr::new(block)).addr();
    let phantom = e.mem.u32(e.mem.u32(source));
    e.mem.set_u32(copy + 0x10, phantom);
    e.call(ADD_REFERENCE, &args![phantom]);
    e.call(
        REMOVE_OVERLAP_LISTENER,
        &args![phantom, listener_address(t)],
    );
    e.call(
        ADD_OVERLAP_LISTENER,
        &args![phantom, listener_address(copy)],
    );
    e.mem.set_u32(copy + 0x20, body.wrapping_add(0x10));
    let filter = e.mem.u32(t + 0xc);
    e.mem.set_u32(copy + 0xc, filter);
    Ptr::new(copy)
}

// Translated from 00e42610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleRayCastWheelCollide::getPhantoms` (Xbox PDB name, engine map). Thiscall, one stack
/// word (`RET 4`): `array` (an `hkArray` of pointers). Appends the phantom (`this + 0x10`),
/// first growing the array through the allocator (`00c90940`, element size 4) when its size
/// equals its capacity.
pub fn hkp_vehicle_ray_cast_wheel_collide_get_phantoms(e: &mut Engine, this: Ptr, array: u32) {
    let phantom = e.mem.u32(this.addr() + 0x10);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    if e.mem.u32(array + 4) == e.mem.u32(array + 8) & 0x3fff_ffff {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_GROW, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), phantom);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e4_1e70,
            hkp_vehicle_ray_cast_wheel_collide_update_before_collision_detection(Ptr, u32)
        ),
        entry!(
            0x00e4_1eb0,
            hkp_vehicle_ray_cast_wheel_collide_cast_single_wheel(Ptr, u32, u32)
        ),
        entry!(
            0x00e4_1ef0,
            hkp_vehicle_ray_cast_wheel_collide_add_to_world(Ptr, u32)
        ),
        entry!(
            0x00e4_1f00,
            hkp_vehicle_ray_cast_wheel_collide_remove_from_world(Ptr)
        ),
        entry!(
            0x00e4_1f10,
            hkp_vehicle_ray_cast_wheel_collide_set_collision_filter_info(Ptr, u32)
        ),
        entry!(
            0x00e4_1f30,
            hkp_vehicle_ray_cast_wheel_collide_init(Ptr, u32)
        ),
        entry!(
            0x00e4_1fc0,
            hkp_vehicle_ray_cast_wheel_collide_destructor(Ptr)
        ),
        entry!(
            0x00e4_2010,
            hkp_vehicle_ray_cast_wheel_collide_calc_wheels_aabb(Ptr, u32, u32)
        ),
        entry!(
            0x00e4_20c0,
            hkp_vehicle_ray_cast_wheel_collide_collide_wheels(Ptr, u32, u32, u32)
        ),
        entry!(
            0x00e4_2190,
            hkp_vehicle_ray_cast_wheel_collide_get_collision_output_from_cast_result(
                Ptr, u32, u32, u32, u32
            )
        ),
        entry!(
            0x00e4_23d0,
            hkp_vehicle_ray_cast_wheel_collide_get_collision_output_without_hit(Ptr, u32, u32, u32)
        ),
        entry!(0x00e4_2460, hkp_vehicle_ray_cast_wheel_collide_build_ray_cast_commands(Ptr, u32, u32, u32, u32, u32) -> i32),
        entry!(0x00e4_2550, fn_00e42550(Ptr) -> Ptr),
        entry!(0x00e4_2590, hkp_vehicle_ray_cast_wheel_collide_clone(Ptr, u32, u32) -> Ptr),
        entry!(
            0x00e4_2610,
            hkp_vehicle_ray_cast_wheel_collide_get_phantoms(Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Base of the doubles that stand in for vtable slots (`SLOT_BASE + slot offset`).
    const SLOT_BASE: u32 = 0x0900_0000;

    /// An engine with the exe's data range mapped and an empty call log.
    fn setup() -> (Engine, Log) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        (e, Rc::new(RefCell::new(Vec::new())))
    }

    /// Doubles that only log their arguments (and return zero) for each address.
    fn log_callees(e: &mut Engine, log: &Log, addresses: &[u32]) {
        for &address in addresses {
            let sink = Rc::clone(log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn slot_calls(log: &Log, slot: u32) -> Vec<Vec<u32>> {
        calls_to(log, SLOT_BASE + slot)
    }

    /// An object whose vtable has logging doubles (returning zero) at the given slot offsets.
    fn object_with_slots(e: &mut Engine, log: &Log, slots: &[u32]) -> u32 {
        let object = e.mem.alloc(0x400);
        let table = e.mem.alloc(0x100);
        e.mem.set_u32(object, table);
        for &slot in slots {
            e.mem.set_u32(table + slot, SLOT_BASE + slot);
            log_callees(e, log, &[SLOT_BASE + slot]);
        }
        object
    }

    /// A memory router whose allocator hands out `block` for the slot 4 call.
    fn memory(e: &mut Engine, log: &Log, block: u32) -> (u32, u32) {
        let router = e.mem.alloc(0x20);
        let allocator = object_with_slots(e, log, &[4]);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sink = Rc::clone(log);
        e.register_double(SLOT_BASE + 4, move |_, a| {
            sink.borrow_mut().push((SLOT_BASE + 4, a.to_vec()));
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        (router, allocator)
    }

    /// A vehicle with `wheels` wheel records, a data block holding the count, and everything
    /// else zero: (vehicle, data block, wheel records).
    fn vehicle(e: &mut Engine, wheels: u8) -> (u32, u32, u32) {
        let vehicle = e.mem.alloc(0x80);
        let data = e.mem.alloc(0xa0);
        let infos = e.mem.alloc(0xe0 * 4);
        e.mem.set_u8(data + 0x20, wheels);
        e.mem.set_u32(vehicle + 0x1c, data);
        e.mem.set_u32(vehicle + 0x48, infos);
        (vehicle, data, infos)
    }

    #[test]
    fn update_before_collision_detection_sets_the_phantom_aabb() {
        let (mut e, log) = setup();
        let this = object_with_slots(&mut e, &log, &[0x34]);
        e.mem.set_u32(this + 0x10, 0x6000);
        let sink = Rc::clone(&log);
        e.register_double(SLOT_BASE + 0x34, move |e, a| {
            sink.borrow_mut().push((SLOT_BASE + 0x34, a.to_vec()));
            e.mem.set_u32(a[2], 0xabcd);
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(PHANTOM_SET_AABB, move |e, a| {
            let mut logged = a.to_vec();
            logged.push(e.mem.u32(a[1]));
            sink.borrow_mut().push((PHANTOM_SET_AABB, logged));
            Ret::default()
        });
        e.call(0x00e4_1e70, &args![this, 0x77u32]);
        let built = slot_calls(&log, 0x34);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][..2], [this, 0x77]);
        assert_eq!(
            calls_to(&log, PHANTOM_SET_AABB),
            vec![vec![0x6000, built[0][2], 0xabcd]]
        );
    }

    #[test]
    fn cast_single_wheel_builds_the_ray_input() {
        let (mut e, log) = setup();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, 0x77);
        e.mem.set_u32(this + 0x10, 0x6000);
        let wheel = e.mem.alloc(0x80);
        for k in 0..8u32 {
            e.mem.set_u32(wheel + 0x50 + 4 * k, 0x100 + k);
        }
        let sink = Rc::clone(&log);
        e.register_double(PHANTOM_CAST_RAY, move |e, a| {
            let mut logged = a.to_vec();
            logged.extend((0..8).map(|k| e.mem.u32(a[1] + 4 * k)));
            logged.push(u32::from(e.mem.u8(a[1] + 0x20)));
            logged.push(e.mem.u32(a[1] + 0x24));
            sink.borrow_mut().push((PHANTOM_CAST_RAY, logged));
            Ret::default()
        });
        e.call(0x00e4_1eb0, &args![this, wheel, 0x5555u32]);
        let calls = calls_to(&log, PHANTOM_CAST_RAY);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][0], 0x6000);
        assert_eq!(calls[0][2], 0x5555);
        assert_eq!(
            calls[0][3..],
            [0x100, 0x101, 0x102, 0x103, 0x104, 0x105, 0x106, 0x107, 1, 0x77]
        );
    }

    #[test]
    fn add_to_world_adds_the_phantom() {
        let (mut e, log) = setup();
        log_callees(&mut e, &log, &[WORLD_ADD_PHANTOM]);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, 0x6000);
        e.call(0x00e4_1ef0, &args![this, 0x7777u32]);
        assert_eq!(
            calls_to(&log, WORLD_ADD_PHANTOM),
            vec![vec![0x7777, 0x6000]]
        );
    }

    #[test]
    fn remove_from_world_removes_the_phantom_from_its_world() {
        let (mut e, log) = setup();
        log_callees(&mut e, &log, &[WORLD_REMOVE_PHANTOM]);
        let this = e.mem.alloc(0x20);
        let phantom = e.mem.alloc(0x20);
        e.mem.set_u32(phantom + 8, 0x7777);
        e.mem.set_u32(this + 0x10, phantom);
        e.call(0x00e4_1f00, &args![this]);
        assert_eq!(
            calls_to(&log, WORLD_REMOVE_PHANTOM),
            vec![vec![0x7777, phantom]]
        );
    }

    #[test]
    fn set_collision_filter_info_updates_the_phantom_when_there_is_one() {
        let (mut e, _) = setup();
        let this = e.mem.alloc(0x20);
        let phantom = e.mem.alloc(0x40);
        e.call(0x00e4_1f10, &args![this, 0x11u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0x11);
        assert_eq!(e.mem.u32(phantom + 0x2c), 0);
        e.mem.set_u32(this + 0x10, phantom);
        e.call(0x00e4_1f10, &args![this, 0x22u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0x22);
        assert_eq!(e.mem.u32(phantom + 0x2c), 0x22);
    }

    #[test]
    fn init_builds_the_phantom_and_registers_the_listener() {
        let (mut e, log) = setup();
        let this = object_with_slots(&mut e, &log, &[0x34]);
        e.mem.set_u32(this + 0xc, 0x66);
        let block = e.mem.alloc(0x100);
        let (_, allocator) = memory(&mut e, &log, block);
        let sink = Rc::clone(&log);
        e.register_double(SLOT_BASE + 0x34, move |e, a| {
            sink.borrow_mut().push((SLOT_BASE + 0x34, a.to_vec()));
            e.mem.set_u32(a[2], 0xabcd);
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(PHANTOM_CONSTRUCT, move |e, a| {
            let mut logged = a.to_vec();
            logged.push(e.mem.u32(a[1]));
            sink.borrow_mut().push((PHANTOM_CONSTRUCT, logged));
            Ret {
                eax: 0x6000,
                ..Ret::default()
            }
        });
        log_callees(&mut e, &log, &[ADD_OVERLAP_LISTENER]);
        let vehicle = e.mem.alloc(0x40);
        e.mem.set_u32(vehicle + 0x18, 0x8000);
        e.call(0x00e4_1f30, &args![this, vehicle]);
        let built = slot_calls(&log, 0x34);
        assert_eq!(built, vec![vec![this, vehicle, built[0][2]]]);
        assert_eq!(slot_calls(&log, 4), vec![vec![allocator, 0xe0]]);
        assert_eq!(e.mem.u16(block + 4), 0xe0);
        assert_eq!(
            calls_to(&log, PHANTOM_CONSTRUCT),
            vec![vec![block, built[0][2], 0x66, 0xabcd]]
        );
        assert_eq!(e.mem.u32(this + 0x10), 0x6000);
        assert_eq!(e.mem.u32(this + 0x20), 0x8010);
        assert_eq!(
            calls_to(&log, ADD_OVERLAP_LISTENER),
            vec![vec![0x6000, this + 0x1c]]
        );
    }

    #[test]
    fn destructor_releases_the_phantom_and_the_listener() {
        let (mut e, log) = setup();
        log_callees(
            &mut e,
            &log,
            &[
                REMOVE_OVERLAP_LISTENER,
                REMOVE_REFERENCE,
                REJECT_CHASSIS_LISTENER_DESTRUCTOR,
            ],
        );
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x10, 0x6000);
        e.mem.set_u32(this + 0x20, 0x8010);
        e.call(0x00e4_1fc0, &args![this]);
        assert_eq!(
            calls_to(&log, REMOVE_OVERLAP_LISTENER),
            vec![vec![0x6000, this + 0x1c]]
        );
        assert_eq!(calls_to(&log, REMOVE_REFERENCE), vec![vec![0x6000]]);
        assert_eq!(
            calls_to(&log, REJECT_CHASSIS_LISTENER_DESTRUCTOR),
            vec![vec![this + 0x14]]
        );
        assert_eq!(e.mem.u32(this), WHEEL_COLLIDE_BASE_VTABLE);
        log.borrow_mut().clear();
        e.mem.set_u32(this + 0x20, 0);
        e.call(0x00e4_1fc0, &args![this]);
        assert!(calls_to(&log, REMOVE_OVERLAP_LISTENER).is_empty());
        assert_eq!(calls_to(&log, REMOVE_REFERENCE).len(), 1);
        log.borrow_mut().clear();
        e.mem.set_u32(this + 0x10, 0);
        e.call(0x00e4_1fc0, &args![this]);
        assert!(calls_to(&log, REMOVE_REFERENCE).is_empty());
        assert_eq!(calls_to(&log, REJECT_CHASSIS_LISTENER_DESTRUCTOR).len(), 1);
    }

    #[test]
    fn calc_wheels_aabb_folds_every_wheel_into_the_bounds() {
        let (mut e, _) = setup();
        let (vehicle, data, infos) = vehicle(&mut e, 3);
        let rays: [([f32; 4], [f32; 4]); 3] = [
            ([1.0, 5.0, 3.0, 9.0], [2.0, 4.0, 6.0, 0.0]),
            ([0.0, 7.0, 3.0, 1.0], [3.0, 3.0, 3.0, 3.0]),
            ([-1.0, 2.0, 2.0, 2.0], [2.0, 2.0, 10.0, 2.0]),
        ];
        for (k, (start, end)) in rays.into_iter().enumerate() {
            store_vec(&mut e, infos + 0xe0 * k as u32 + 0x50, start);
            store_vec(&mut e, infos + 0xe0 * k as u32 + 0x60, end);
        }
        let aabb = e.mem.alloc(0x20);
        e.call(0x00e4_2010, &args![0u32, vehicle, aabb]);
        assert_eq!(load_vec(&e, aabb), [-1.0, 2.0, 2.0, 0.0]);
        assert_eq!(load_vec(&e, aabb + 0x10), [3.0, 7.0, 10.0, 9.0]);
        e.mem.set_u8(data + 0x20, 1);
        e.call(0x00e4_2010, &args![0u32, vehicle, aabb]);
        assert_eq!(load_vec(&e, aabb), [1.0, 4.0, 3.0, 0.0]);
        assert_eq!(load_vec(&e, aabb + 0x10), [2.0, 5.0, 6.0, 9.0]);
    }

    #[test]
    fn collide_wheels_picks_the_output_call_by_the_cast_result() {
        let (mut e, log) = setup();
        let this = object_with_slots(&mut e, &log, &[0x38, 0x3c, 0x40, 0x2c]);
        let (vehicle, data, infos) = vehicle(&mut e, 2);
        let sink = Rc::clone(&log);
        e.register_double(SLOT_BASE + 0x38, move |e, a| {
            let mut logged = a.to_vec();
            logged.extend([0x10, 0x14, 0x20, 0x40].map(|k| e.mem.u32(a[2] + k)));
            sink.borrow_mut().push((SLOT_BASE + 0x38, logged));
            if a[1] == infos {
                e.mem.set_u32(a[2] + 0x50, 0x1234);
            }
            Ret::default()
        });
        let outputs = e.mem.alloc(0x200);
        e.call(0x00e4_20c0, &args![this, 0u32, vehicle, outputs]);
        let casts = slot_calls(&log, 0x38);
        assert_eq!(casts.len(), 2);
        let ray = casts[0][2];
        assert_eq!(casts[0][..2], [this, infos]);
        assert_eq!(casts[0][3..], [0x3f80_0000, 0xffff_ffff, 0xffff_ffff, 0]);
        assert_eq!(casts[1][..3], [this, infos + 0xe0, ray]);
        assert_eq!(
            slot_calls(&log, 0x3c),
            vec![vec![this, vehicle, 0, ray, outputs]]
        );
        assert_eq!(
            slot_calls(&log, 0x40),
            vec![vec![this, vehicle, 1, outputs + 0x60]]
        );
        assert_eq!(
            slot_calls(&log, 0x2c),
            vec![
                vec![this, vehicle, 0, outputs],
                vec![this, vehicle, 1, outputs + 0x60]
            ]
        );
        log.borrow_mut().clear();
        e.mem.set_u8(data + 0x20, 0);
        e.call(0x00e4_20c0, &args![this, 0u32, vehicle, outputs]);
        assert!(log.borrow().is_empty());
    }

    /// A vehicle for the collision-output functions: wheel `1` has rest length `0.25`,
    /// parameter `0.5`, a ray from `(10, 20, 30, 1)` along `(0, -1, 0, 0)` and ends at
    /// `(1, 2, 3, 4)`; the data block's threshold is `0.5`. Returns (vehicle, data, wheel
    /// record, chassis body).
    fn output_vehicle(e: &mut Engine) -> (u32, u32, u32, u32) {
        let (vehicle, data, infos) = vehicle(e, 2);
        let holder = e.mem.alloc(0x10);
        let rests = e.mem.alloc(0x100);
        e.mem.set_u32(holder + 8, rests);
        e.mem.set_f32(rests + 0x30 + 0x20, 0.25);
        e.mem.set_u32(vehicle + 0x34, holder);
        let parameters = e.mem.alloc(0x80);
        e.mem.set_f32(parameters + 0x28, 0.5);
        e.mem.set_u32(data + 0x8c, parameters);
        e.mem.set_f32(data + 0x84, 0.5);
        let body = e.mem.alloc(0x200);
        e.mem.set_u32(vehicle + 0x18, body);
        let info = infos + 0xe0;
        store_vec(e, info + 0x50, [10.0, 20.0, 30.0, 1.0]);
        store_vec(e, info + 0x60, [1.0, 2.0, 3.0, 4.0]);
        store_vec(e, info + 0x80, [0.0, -1.0, 0.0, 0.0]);
        (vehicle, data, info, body)
    }

    #[test]
    fn get_collision_output_from_cast_result_works_out_the_contact() {
        let (mut e, _) = setup();
        let (vehicle, data, _, body) = output_vehicle(&mut e);
        e.mem.set_f64(NEGATIVE_ONE, -1.0);
        let owner = e.mem.alloc(0x200);
        let collidable = owner + 0x20;
        e.mem.set_u8(collidable + 0x18, 1);
        e.mem.set_u8(collidable + 0x10, 0xe0);
        e.mem.set_f32(owner + 0x90, 7.5);
        store_vec(&mut e, body + 0x140, [1.0, 2.0, 3.0, 0.0]);
        store_vec(&mut e, body + 0x1c0, [0.0, 0.0, 1.0, 0.0]);
        store_vec(&mut e, body + 0x1b0, [1.0, 0.0, 0.0, 0.0]);
        let cast = e.mem.alloc(0x80);
        store_vec(&mut e, cast, [0.0, 1.0, 0.0, 9.0]);
        e.mem.set_f32(cast + 0x10, 0.5);
        for k in 0..8u32 {
            e.mem.set_u32(cast + 0x20 + 4 * k, 0x100 + k);
        }
        e.mem.set_u32(cast + 0x50, collidable);
        let output = e.mem.alloc(0x60);
        e.mem.set_u32(output + 0x1c, 0xdead);
        e.call(0x00e4_2190, &args![0u32, vehicle, 1u32, cast, output]);
        assert_eq!(load_vec(&e, output), [10.0, 19.625, 30.0, 1.0]);
        assert_eq!(load_vec(&e, output + 0x10), [0.0, 1.0, 0.0, -0.125]);
        assert_eq!(e.mem.u32(output + 0x20), 7.5f32.to_bits());
        assert_eq!(e.mem.u32(output + 0x24), owner);
        for k in 0..8u32 {
            assert_eq!(e.mem.u32(output + 0x28 + 4 * k), 0x100 + k);
        }
        assert_eq!(e.mem.f32(output + 0x48), -0.125);
        assert_eq!(e.mem.f32(output + 0x4c), 9.0);
        assert_eq!(e.mem.f32(output + 0x50), 1.0);
        e.mem.set_f32(data + 0x84, 1.5);
        e.call(0x00e4_2190, &args![0u32, vehicle, 1u32, cast, output]);
        assert_eq!(e.mem.f32(output + 0x4c), 0.0);
        assert_eq!(e.mem.f32(output + 0x50), (1.0f64 / 1.5f64) as f32);
    }

    #[test]
    fn get_collision_output_without_hit_writes_the_free_ray() {
        let (mut e, _) = setup();
        let (vehicle, _, info, _) = output_vehicle(&mut e);
        store_bits(&mut e, SIGN_MASK, [0x8000_0000; 4]);
        store_vec(&mut e, info + 0x80, [0.0, -1.0, 2.0, 3.0]);
        let output = e.mem.alloc(0x60);
        for k in 0..0x18u32 {
            e.mem.set_u32(output + 4 * k, 0xdead);
        }
        e.call(0x00e4_23d0, &args![0u32, vehicle, 1u32, output]);
        assert_eq!(load_vec(&e, output), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(
            load_bits(&e, output + 0x10),
            [
                0x8000_0000,
                1.0f32.to_bits(),
                (-2.0f32).to_bits(),
                0.25f32.to_bits()
            ]
        );
        assert_eq!(e.mem.u32(output + 0x20), 0);
        assert_eq!(e.mem.u32(output + 0x24), 0);
        assert_eq!(e.mem.f32(output + 0x48), 0.25);
        assert_eq!(e.mem.f32(output + 0x4c), 0.0);
        assert_eq!(e.mem.f32(output + 0x50), 1.0);
    }

    #[test]
    fn build_ray_cast_commands_writes_one_command_per_wheel() {
        let (mut e, log) = setup();
        let phantom = object_with_slots(&mut e, &log, &[0x28]);
        e.mem.set_u32(phantom + 0xd0, 0xd0d0);
        e.mem.set_u32(phantom + 0xd4, 3);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, 0x66);
        e.mem.set_u32(this + 0x10, phantom);
        let (vehicle, data, infos) = vehicle(&mut e, 2);
        for k in 0..2u32 {
            store_vec(&mut e, infos + 0xe0 * k + 0x50, [k as f32, 1.0, 2.0, 3.0]);
            store_vec(&mut e, infos + 0xe0 * k + 0x60, [4.0, 5.0, k as f32, 6.0]);
        }
        let info = e.mem.alloc(0x40);
        e.mem.set_u32(info + 0x20, 0x99);
        let commands = e.mem.alloc(0x100);
        let got = e.call(
            0x00e4_2460,
            &args![this, vehicle, info, 0x55u32, commands, 0x1000u32],
        );
        assert_eq!(got.i32(), 2);
        assert_eq!(slot_calls(&log, 0x28), vec![vec![phantom]]);
        for k in 0..2u32 {
            let command = commands + 0x50 * k;
            assert_eq!(load_vec(&e, command), [k as f32, 1.0, 2.0, 3.0]);
            assert_eq!(load_vec(&e, command + 0x10), [4.0, 5.0, k as f32, 6.0]);
            assert_eq!(e.mem.u32(command + 0x20), 0x66);
            assert_eq!(e.mem.u32(command + 0x24), info + 0x10);
            assert_eq!(e.mem.u32(command + 0x30), 0x99);
            assert_eq!(e.mem.u32(command + 0x34), 0x55);
            assert_eq!(e.mem.u32(command + 0x38), 0xd0d0);
            assert_eq!(e.mem.u32(command + 0x3c), 3);
            assert_eq!(e.mem.u32(command + 0x40), 0x1000 + 0x60 * k);
            assert_eq!(e.mem.u32(command + 0x44), 1);
            assert_eq!(e.mem.u32(command + 0x48), 0);
            assert_eq!(e.mem.u8(command + 0x4c), 0);
        }
        e.mem.set_u8(data + 0x20, 0);
        assert_eq!(
            e.call(
                0x00e4_2460,
                &args![this, vehicle, info, 0x55u32, commands, 0x1000u32]
            )
            .i32(),
            0
        );
        e.mem.set_u32(phantom + 0xd4, 0);
        e.mem.set_u8(data + 0x20, 2);
        e.mem.set_u32(commands + 0x44, 0xbeef);
        assert_eq!(
            e.call(
                0x00e4_2460,
                &args![this, vehicle, info, 0x55u32, commands, 0x1000u32]
            )
            .i32(),
            0
        );
        assert_eq!(e.mem.u32(commands + 0x44), 0xbeef);
    }

    #[test]
    fn fn_00e42550_initialises_the_object() {
        let (mut e, _) = setup();
        let this = e.mem.alloc(0x40);
        for k in 0..0x10u32 {
            e.mem.set_u32(this + 4 * k, 0xaaaa_aaaa);
        }
        let got = e.call(0x00e4_2550, &args![this]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u32(this), 0x010e_5ae4);
        assert_eq!(e.mem.u32(this + 4), 0x0001_aaaa);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u8(this + 8), 0);
        assert_eq!(e.mem.u8(this + 9), 1);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0x010d_ebf4);
        assert_eq!(e.mem.u16(this + 0x1a), 1);
        assert_eq!(e.mem.u32(this + 0x1c), 0x010d_ebe4);
    }

    #[test]
    fn clone_shares_the_phantom_and_moves_the_listener() {
        let (mut e, log) = setup();
        let block = e.mem.alloc(0x40);
        let (_, allocator) = memory(&mut e, &log, block);
        log_callees(
            &mut e,
            &log,
            &[ADD_REFERENCE, REMOVE_OVERLAP_LISTENER, ADD_OVERLAP_LISTENER],
        );
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xc, 0x31);
        let cell = e.mem.alloc(8);
        e.mem.set_u32(cell, 0x7000);
        let source = e.mem.alloc(8);
        e.mem.set_u32(source, cell);
        let got = e.call(0x00e4_2590, &args![this, 0x8000u32, source]).u32();
        assert_eq!(got, block);
        assert_eq!(slot_calls(&log, 4), vec![vec![allocator, 0x24]]);
        assert_eq!(e.mem.u16(block + 4), 0x24);
        assert_eq!(e.mem.u32(block), 0x010e_5ae4);
        assert_eq!(e.mem.u32(block + 0x10), 0x7000);
        assert_eq!(e.mem.u32(block + 0x20), 0x8010);
        assert_eq!(e.mem.u32(block + 0xc), 0x31);
        assert_eq!(calls_to(&log, ADD_REFERENCE), vec![vec![0x7000]]);
        assert_eq!(
            calls_to(&log, REMOVE_OVERLAP_LISTENER),
            vec![vec![0x7000, this + 0x1c]]
        );
        assert_eq!(
            calls_to(&log, ADD_OVERLAP_LISTENER),
            vec![vec![0x7000, block + 0x1c]]
        );
    }

    #[test]
    fn get_phantoms_appends_and_grows_when_full() {
        let (mut e, log) = setup();
        let (_, allocator) = memory(&mut e, &log, 0);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, 0x6000);
        let data = e.mem.alloc(8);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, 1);
        e.mem.set_u32(array + 8, 2);
        e.mem.set_u32(data, 0x5000);
        let grown = e.mem.alloc(0x20);
        let sink = Rc::clone(&log);
        e.register_double(ARRAY_GROW, move |e, a| {
            sink.borrow_mut().push((ARRAY_GROW, a.to_vec()));
            let old = e.mem.u32(a[1]);
            for k in 0..2 {
                let word = e.mem.u32(old + 4 * k);
                e.mem.set_u32(grown + 4 * k, word);
            }
            e.mem.set_u32(a[1], grown);
            e.mem.set_u32(a[1] + 8, 4);
            Ret::default()
        });
        e.call(0x00e4_2610, &args![this, array]);
        assert_eq!(e.mem.u32(array + 4), 2);
        assert_eq!(e.mem.u32(data + 4), 0x6000);
        assert!(calls_to(&log, ARRAY_GROW).is_empty());
        e.call(0x00e4_2610, &args![this, array]);
        assert_eq!(calls_to(&log, ARRAY_GROW), vec![vec![allocator, array, 4]]);
        assert_eq!(e.mem.u32(array + 4), 3);
        assert_eq!(e.mem.u32(grown + 8), 0x6000);
    }
}
