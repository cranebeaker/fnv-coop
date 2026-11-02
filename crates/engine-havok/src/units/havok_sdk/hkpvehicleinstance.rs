//! `Havok SDK/hkpvehicleinstance.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpboxmotion::rows_times;
use crate::units::havok_sdk::hkpconvexradiusbuilder::{
    addps, mulps, shufps, splat, subps, sum_of_three,
};
use crate::units::havok_sdk::hkpsphereshape::{
    copy_vec, inverse_length, load_bits, load_vec, maxps, minps, profile_record, store_vec,
    zip_with, Vec4, PROFILE_END, SIGN_MASK,
};

// hkpVehicleInstance has no type in the Xbox PDB, so its layout is described by the offsets
// the PC code uses (roles unconfirmed unless said): `+0x18` the chassis rigid body, `+0x1c` the
// vehicle data block, `+0x20` .. `+0x38` the component objects (each called through its vtable),
// `+0x3c` the wheel-collide object, `+0x48` the array of wheel records (`0xe0` bytes each),
// `+0xa0` a byte flag per wheel, `+0xc0` a float per wheel. In the data block `+0x20` is the
// number of wheels (signed byte), `+0x8c` the array of per-wheel parameters (`0x28` bytes each),
// `+0x98` a table of signed bytes indexed by axle, `+0x9c` the wheel count (`int`).

/// `-0.5f` four times, the vector the wheel update adds to the squared `w` lane (`.rdata`).
const NEGATIVE_HALVES: u32 = 0x010c_71e0;
/// `hkpEntity::activate` (`ECX` the body).
const ENTITY_ACTIVATE: u32 = 0x00c9_c1d0;
/// `hkpMotion::getMass` (`ECX` the motion, mass in `ST0`).
const MOTION_GET_MASS: u32 = 0x00c8_6430;
/// The CRT square root: argument in `ST0` (a leading `f64`), result in `ST0`.
const X87_SQRT: u32 = 0x00ec_6040;
/// `hkVector4::setRotatedDir` (`ECX` the output, then the rotation and the input vector).
const SET_ROTATED_DIR: u32 = 0x00c7_f700;
/// The vector transform `00c7f640` (`ECX` the output, then the transform and the input).
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
/// `hkRigidMotionUtilApplyForcesAndBuildAccumulators` (cdecl): step information, a one-entry
/// list of motion pointers, the count `1`, `0` and the accumulator output.
const BUILD_ACCUMULATORS: u32 = 0x00d2_9830;
/// `00d72030` (`ECX` the output, one word: the accumulator).
const BUILD_AXLE_BASIS: u32 = 0x00d7_2030;
/// Profile label of the start record of `updateBeforeCollisionDetection` (`.rdata`).
const UPDATE_PROFILE_LABEL: u32 = 0x010e_b830;

/// The chassis speed: `SQRTSS` of the squared length of the three lanes at body `+0x1b0`.
fn chassis_speed(e: &Engine, this: u32) -> f32 {
    let body = e.mem.u32(this + 0x18);
    let velocity = load_vec(e, body + 0x1b0);
    sum_of_three(mulps(velocity, velocity)).sqrt()
}

/// `inverse_length` of a vector's three lanes applied to all four lanes in place (the
/// `RSQRTSS` normalisation the wheel code repeats).
fn normalise_in_place(e: &mut Engine, address: u32) {
    let v = load_vec(e, address);
    let scale = inverse_length(e, sum_of_three(mulps(v, v)));
    store_vec(e, address, mulps(splat(scale), v));
}

/// The address of wheel `wheel`'s parameter entry (`0x28` bytes) in the data block.
fn wheel_params(e: &Engine, this: u32, wheel: u32) -> u32 {
    let data = e.mem.u32(this + 0x1c);
    e.mem
        .u32(data + 0x8c)
        .wrapping_add(wheel.wrapping_mul(0x28))
}

// Translated from 00e36f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleInstance::getPhantoms` (Xbox PDB name, engine map): a tail jump to slot `0x14` of
/// the object at `+0x3c` (`ECX` becomes that object; the stack word is passed on, and `00e42610`
/// is the one implementation seen, `RET 4`). Returns what the callee leaves in `EAX`.
pub fn hkp_vehicle_instance_get_phantoms(e: &mut Engine, this: Ptr, phantoms: u32) -> u32 {
    let wheel_collide = e.mem.u32(this.addr() + 0x3c);
    e.vcall(wheel_collide, 0x14, &args![phantoms]).u32()
}

// Translated from 00e36f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Component step: copies the byte at `this + 0xb0` into `block + 0xd`, calls slot `0xc` of the
/// object at `this + 0x20` with (the float, `this`, the word at `this + 0x9c`, `block`), then
/// copies the byte at `block + 0xd` back to `this + 0xb0`. The float is passed through an
/// `FLD`/`FSTP` pair.
pub fn fn_00e36f60(e: &mut Engine, this: Ptr, value: f32, block: u32) {
    let t = this.addr();
    let flag = e.mem.u8(t + 0xb0);
    e.mem.set_u8(block + 0xd, flag);
    let word = e.mem.u32(t + 0x9c);
    let component = e.mem.u32(t + 0x20);
    e.vcall(component, 0xc, &args![x87_quiet(value), t, word, block]);
    let flag = e.mem.u8(block + 0xd);
    e.mem.set_u8(t + 0xb0, flag);
}

// Translated from 00e36fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Component step: fills `block` (`+0xc` the byte at data `+0x20`, `+0xd`..`+0xf` the bytes at
/// `this + 0xcc`..`+0xce`, `+0x10` the float at `this + 0xd0`), calls slot `0xc` of the object
/// at `this + 0x2c` with (the float, `this`, `block`) and copies the bytes `block + 0xd`..`+0xf`
/// and the float at `block + 0x10` back to `this + 0xcc`..`+0xd0`.
pub fn fn_00e36fa0(e: &mut Engine, this: Ptr, value: f32, block: u32) {
    let t = this.addr();
    let data = e.mem.u32(t + 0x1c);
    let wheels = e.mem.u8(data + 0x20);
    e.mem.set_u8(block + 0xc, wheels);
    for k in 0..3 {
        let byte = e.mem.u8(t + 0xcc + k);
        e.mem.set_u8(block + 0xd + k, byte);
    }
    let float_bits = x87_quiet_bits(e.mem.u32(t + 0xd0));
    e.mem.set_u32(block + 0x10, float_bits);
    let component = e.mem.u32(t + 0x2c);
    e.vcall(component, 0xc, &args![x87_quiet(value), t, block]);
    for k in 0..3 {
        let byte = e.mem.u8(block + 0xd + k);
        e.mem.set_u8(t + 0xcc + k, byte);
    }
    let float_bits = x87_quiet_bits(e.mem.u32(block + 0x10));
    e.mem.set_u32(t + 0xd0, float_bits);
}

// Translated from 00e37020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Component step with a two-float local: the floats at `this + 0xb4` and `this + 0xb8` are put
/// in an 8-byte local (in that order), slot `0xc` of the object at `this + 0x28` is called with
/// (the float, `this`, `word_a`, `word_b`, the local) and the local's floats are copied back.
pub fn fn_00e37020(e: &mut Engine, this: Ptr, value: f32, word_a: u32, word_b: u32) {
    let t = this.addr();
    let low = x87_quiet_bits(e.mem.u32(t + 0xb4));
    let high = x87_quiet_bits(e.mem.u32(t + 0xb8));
    let component = e.mem.u32(t + 0x28);
    e.with_stack(8, |e, local| {
        e.mem.set_u32(local.addr(), low);
        e.mem.set_u32(local.addr() + 4, high);
        e.vcall(
            component,
            0xc,
            &args![x87_quiet(value), t, word_a, word_b, local.addr()],
        );
        let high = x87_quiet_bits(e.mem.u32(local.addr() + 4));
        e.mem.set_u32(t + 0xb8, high);
        let low = x87_quiet_bits(e.mem.u32(local.addr()));
        e.mem.set_u32(t + 0xb4, low);
    });
}

// Translated from 00e37080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Component step: calls slot `0xc` of the object at `this + 0x38` with (the float, `this`,
/// `word`).
pub fn fn_00e37080(e: &mut Engine, this: Ptr, value: f32, word: u32) {
    let component = e.mem.u32(this.addr() + 0x38);
    e.vcall(component, 0xc, &args![x87_quiet(value), this.addr(), word]);
}

// Translated from 00e370a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleInstance::calcRPM` (Xbox PDB name, engine map): the float at `this + 0xb8`
/// (`FLD`, returned in `ST0`).
pub fn hkp_vehicle_instance_calc_rpm(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr() + 0xb8))
}

// Translated from 00e370b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialiser of a wheel record (`this` is the `0xe0`-byte record; constructor-shaped, class
/// unconfirmed): `+0` and `+0x10` take 16-byte constants (`01267e30`, `010c7190`) and
/// `+0x1c` becomes `1.0`; `+0x20`, `+0x50`, `+0x60`, `+0x70`, `+0x80`, `+0xa0`, `+0xc0`..`+0xd4`
/// become `0.0`; `+0x24 = 0`, `+0x28 = -1`; `+0x90` is the vector (the float at `01012068`, then
/// the float at `01011d78` in the other three lanes); `+0xb0` takes the 16-byte constant
/// `010c71b0`. The registers the exe shuffles (`XMM2`, `XMM3` upper lanes) never reach the
/// stored vector.
pub fn fn_00e370b0(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    copy_vec(e, 0x0126_7e30, t);
    copy_vec(e, 0x010c_7190, t + 0x10);
    e.mem.set_f32(t + 0x1c, 1.0);
    for offset in [0x50, 0x60, 0x80, 0xa0] {
        store_vec(e, t + offset, [0.0; 4]);
    }
    e.mem.set_f32(t + 0x20, 0.0);
    e.mem.set_f32(t + 0x70, 0.0);
    let lane_0: f32 = e.global(0x0101_2068);
    let fill: f32 = e.global(0x0101_1d78);
    e.mem.set_u32(t + 0x24, 0);
    e.mem.set_u32(t + 0x28, 0xffff_ffff);
    store_vec(e, t + 0x90, [lane_0, fill, fill, fill]);
    copy_vec(e, 0x010c_71b0, t + 0xb0);
    for offset in [0xc0, 0xc4, 0xc8, 0xcc, 0xd0, 0xd4] {
        e.mem.set_f32(t + offset, 0.0);
    }
}

// Translated from 00e37160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleInstance::handleFixedGroundAccum` (Xbox PDB name, engine map): `this` and the
/// first word are not used. Clears the accumulator at `accum`: `+8 = 0.0`, byte `+0 = 1` and the
/// vectors at `+0x10`, `+0x20`, `+0x30`, `+0x40`, `+0x50`, `+0x60`, `+0x70` become zero.
pub fn hkp_vehicle_instance_handle_fixed_ground_accum(
    e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    accum: u32,
) {
    e.mem.set_f32(accum + 8, 0.0);
    e.mem.set_u8(accum, 1);
    for offset in [0x30, 0x10, 0x20, 0x40, 0x50, 0x60, 0x70] {
        store_vec(e, accum + offset, [0.0; 4]);
    }
}

// Translated from 00e371a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Component step: calls slot `0xc` of the object at `this + 0x34` with (the float, `this`,
/// `word`, the word that `word_ref` points to).
pub fn fn_00e371a0(e: &mut Engine, this: Ptr, value: f32, word: u32, word_ref: u32) {
    let component = e.mem.u32(this.addr() + 0x34);
    let referenced = e.mem.u32(word_ref);
    e.vcall(
        component,
        0xc,
        &args![x87_quiet(value), this.addr(), word, referenced],
    );
}

// Translated from 00e371d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleInstance::applySuspensionForcesAtWheel` (Xbox PDB name, engine map). Thiscall,
/// three stack words (`RET 0xc`): a float, the wheel index and a float. The force vector is the
/// product of the two floats (x87, rounded to `float`) times the wheel record's vector at
/// `+0x10`. When the second float is above zero (NaN skips), the chassis body (`this + 0x18`)
/// is activated (`00c9c1d0`) and the force is applied at the wheel record's point (`+0x50`)
/// through slot `0x50` of the motion at body `+0xe0`. When the wheel record's `+0x24` names a
/// ground body, the force (negated by the float at parameter entry `+0x1c`) is limited to the
/// parameter entry's `+0x20` times that body's mass times the first float (x87 arithmetic) and
/// applied to that body's motion the same way; a longer force is scaled by `limit / length`
/// (the length through `00ec6040`).
pub fn hkp_vehicle_instance_apply_suspension_forces_at_wheel(
    e: &mut Engine,
    this: Ptr,
    factor: f32,
    wheel: u32,
    magnitude: f32,
) {
    let t = this.addr();
    let product = (f64::from(factor) * f64::from(magnitude)) as f32;
    let info = e.mem.u32(t + 0x48).wrapping_add(wheel.wrapping_mul(0xe0));
    let direction = load_vec(e, info + 0x10);
    let mut force = mulps(splat(product), direction);
    e.with_stack(16, |e, local| {
        let local = local.addr();
        store_vec(e, local, force);
        if 0.0 < magnitude {
            let body = e.mem.u32(t + 0x18);
            e.call(ENTITY_ACTIVATE, &args![body]);
            e.vcall(body + 0xe0, 0x50, &args![local, info + 0x50]);
            force = load_vec(e, local);
        }
        let ground = e.mem.u32(info + 0x24);
        if ground != 0 {
            let entry = wheel_params(e, t, wheel);
            let negated = x87_quiet_bits(e.mem.u32(entry + 0x1c)) ^ 0x8000_0000;
            let motion = ground + 0xe0;
            store_vec(e, local, mulps(splat(f32::from_bits(negated)), force));
            let mass = e.call(MOTION_GET_MASS, &args![motion]).f32();
            let allowed = e.mem.f32(entry + 0x20);
            let limit = ((f64::from(allowed) * f64::from(mass)) * f64::from(factor)) as f32;
            let scaled = load_vec(e, local);
            let length_squared = sum_of_three(mulps(scaled, scaled));
            if f64::from(length_squared) > f64::from(limit) * f64::from(limit) {
                let root = e.call(X87_SQRT, &args![f64::from(length_squared)]).f64() as f32;
                let inverse = (1.0f64 / f64::from(root)) as f32;
                let ratio = (f64::from(inverse) * f64::from(limit)) as f32;
                let vector = load_vec(e, local);
                store_vec(e, local, mulps(splat(ratio), vector));
            }
            e.vcall(motion, 0x50, &args![local, info + 0x50]);
        }
    });
}

/// `output[slot] = parameter * weighted + output[slot]` in x87 arithmetic, where `parameter`
/// is the float at `param_offset` of the wheel's parameter entry.
fn add_weighted(
    e: &mut Engine,
    this: u32,
    wheel: u32,
    param_offset: u32,
    output_slot: u32,
    weighted: f32,
) {
    let parameter = f64::from(e.mem.f32(wheel_params(e, this, wheel) + param_offset));
    let old = f64::from(e.mem.f32(output_slot));
    e.mem
        .set_f32(output_slot, (parameter * f64::from(weighted) + old) as f32);
}

// Translated from 00e37350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleInstance::getAxleParamsFromWheel` (Xbox PDB name, engine map). Thiscall, five
/// stack words (`RET 0x14`): the wheel index, `add_48`, `add_44`, `factor_4c` and the axle
/// output record (`output`). Forms the cross product of the wheel record's vectors `+0xa0` and
/// `+0x10`; when its squared length is below the tolerance at `0104da4c` (`1.19e-7`) the
/// product of the vector at `+0x80` with the first is used instead. It is normalised
/// (`RSQRTSS`, one Newton step, zero guard) and the wheel's contribution is added into
/// `output`: `reciprocal * info[0..4]` at `+0`; the torque `normal x info[0x10..]` at `+0x10`;
/// the normal at `+0x20`; at `+0x38`, `+0x3c`, `+0x40` the parameter entry's floats `+0xc`,
/// `+0x10`, `+0x14` times `info[0x20] * reciprocal` (x87); `add_44` at `+0x44`, `add_48` at
/// `+0x48`; byte `+0x50` becomes 1 when it was set or the per-wheel flag byte
/// (`this + 0xa0`) is; `+0x4c` gets `reciprocal * entry[0x18] * factor_4c`. `reciprocal` is
/// `1.0 / n` with `n` the signed byte of the table at data `+0x98` indexed by the entry's
/// axle byte (`+0x24`), rounded to `float`.
pub fn hkp_vehicle_instance_get_axle_params_from_wheel(
    e: &mut Engine,
    this: Ptr,
    wheel: u32,
    add_48: f32,
    add_44: f32,
    factor_4c: f32,
    output: u32,
) {
    let t = this.addr();
    let tolerance: f32 = e.global(0x0104_da4c);
    let data = e.mem.u32(t + 0x1c);
    let axle = i32::from(e.mem.i8(wheel_params(e, t, wheel) + 0x24)) as u32;
    let info = e.mem.u32(t + 0x48).wrapping_add(wheel.wrapping_mul(0xe0));
    let v0 = load_vec(e, info + 0xa0);
    let v1 = load_vec(e, info + 0x10);
    let v0_c9 = shufps(v0, v0, 0xc9);
    let v0_d2 = shufps(v0, v0, 0xd2);
    let v1_d2 = shufps(v1, v1, 0xd2);
    let v1_c9 = shufps(v1, v1, 0xc9);
    let mut cross = subps(mulps(v1_c9, v0_d2), mulps(v1_d2, v0_c9));
    if tolerance > sum_of_three(mulps(cross, cross)) {
        let v2 = load_vec(e, info + 0x80);
        cross = subps(
            mulps(shufps(v2, v2, 0xd2), v0_c9),
            mulps(shufps(v2, v2, 0xc9), v0_d2),
        );
    }
    let table = e.mem.u32(data + 0x98);
    let count = i32::from(e.mem.i8(table.wrapping_add(axle)));
    let reciprocal = (1.0f64 / f64::from(count)) as f32;
    let scale = inverse_length(e, sum_of_three(mulps(cross, cross)));
    let normal = mulps(splat(scale), cross);
    let weighted = (f64::from(e.mem.f32(info + 0x20)) * f64::from(reciprocal)) as f32;
    let info_head = load_vec(e, info);
    let head = load_vec(e, output);
    store_vec(e, output, addps(mulps(splat(reciprocal), info_head), head));
    let torque = subps(
        mulps(shufps(normal, normal, 0xc9), v1_d2),
        mulps(shufps(normal, normal, 0xd2), v1_c9),
    );
    let old_torque = load_vec(e, output + 0x10);
    store_vec(e, output + 0x10, addps(torque, old_torque));
    let old_normal = load_vec(e, output + 0x20);
    store_vec(e, output + 0x20, addps(old_normal, normal));
    add_weighted(e, t, wheel, 0xc, output + 0x38, weighted);
    add_weighted(e, t, wheel, 0x10, output + 0x3c, weighted);
    let was_set = e.mem.u8(output + 0x50) != 0;
    add_weighted(e, t, wheel, 0x14, output + 0x40, weighted);
    let old = f64::from(e.mem.f32(output + 0x44));
    e.mem
        .set_f32(output + 0x44, (f64::from(add_44) + old) as f32);
    let old = f64::from(e.mem.f32(output + 0x48));
    e.mem
        .set_f32(output + 0x48, (old + f64::from(add_48)) as f32);
    let flag = if was_set {
        1
    } else {
        let flags = e.mem.u32(t + 0xa0);
        u8::from(e.mem.u8(flags.wrapping_add(wheel)) != 0)
    };
    e.mem.set_u8(output + 0x50, flag);
    let entry = wheel_params(e, t, wheel);
    let last = (f64::from(reciprocal) * f64::from(e.mem.f32(entry + 0x18))) * f64::from(factor_4c);
    let old = f64::from(e.mem.f32(output + 0x4c));
    e.mem.set_f32(output + 0x4c, (last + old) as f32);
}

/// Memory router (`00c85750`): its field `+0x10` is the allocator, its field `+0` the top of the
/// stack allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (cdecl): the allocator, the array, the new capacity and the element size.
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkReferencedObject::addReference` (`ECX` the object; unconfirmed from the engine map range).
const ADD_REFERENCE: u32 = 0x00c9_0510;
/// `hkReferencedObject::removeReference` (`ECX` the object).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkStringPtr` assignment (`this`, the other string).
const STRING_PTR_ASSIGN: u32 = 0x00cb_0c30;
/// `hkpUnaryAction` constructor (`this`, the entity, `0`) and destructor (`this`).
const UNARY_ACTION_CONSTRUCTOR: u32 = 0x00d2_b260;
const UNARY_ACTION_DESTRUCTOR: u32 = 0x00d2_b2b0;
/// The rigid body's mass setter (`ECX` the body, one float) and inertia setter (`ECX` the body,
/// a pointer to a three-row matrix).
const BODY_SET_MASS: u32 = 0x00ca_88c0;
const BODY_SET_INERTIA: u32 = 0x00ca_8920;
/// Assignment of one wheel record (`0xe0` bytes; `this`, the source).
const RECORD_ASSIGN: u32 = 0x00e3_9300;
/// `0x7fffffff` in each of four lanes (`.rdata`).
const ABS_MASK: u32 = 0x0101_7d10;
/// Profile labels of the start records of `updateComponents` and
/// `stepVehicleUsingWheelCollideOutput` (`.rdata`).
const UPDATE_COMPONENTS_LABEL: u32 = 0x010e_b858;
const STEP_OUTPUT_LABEL: u32 = 0x010e_b86c;

/// The grow step the exe inlines for an `hkArray` (`array` is its pointer/size/capacity triple):
/// when the capacity (the word at `+8` without its two flag bits) is below `count`, it is
/// doubled, or `count` if that is not smaller than the double, and `hkArrayUtil::_reserve`
/// is called with the allocator of the memory router.
pub(crate) fn reserve_array(e: &mut Engine, array: u32, count: i32, element_size: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let mut capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < count {
        capacity = capacity.wrapping_add(capacity);
        if count >= capacity {
            capacity = count;
        }
        e.call(
            ARRAY_RESERVE,
            &args![allocator, array, capacity as u32, element_size],
        );
    }
}

/// The release step the exe inlines for an `hkArray`: size `0`; the buffer is returned to the
/// allocator (slot `0x10`, with the capacity times `element_size` as its size) when the
/// capacity word is not negative; then pointer `0` and capacity word `0x80000000`.
fn free_array(e: &mut Engine, array: u32, element_size: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if (capacity as i32) >= 0 {
        let buffer = e.mem.u32(array);
        e.vcall(
            allocator,
            0x10,
            &args![buffer, (capacity & 0x3fff_ffff).wrapping_mul(element_size)],
        );
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

/// The exact-size reallocation of `00e39480`: when the capacity is below `wanted`, the old buffer
/// is freed (if owned) and a new one of `wanted * element_size` bytes is taken from the
/// allocator (slot `0xc` takes the address of the wanted size and leaves the granted size in
/// it); the capacity word becomes the granted size divided by `element_size`.
fn reallocate_exact(e: &mut Engine, array: u32, wanted: i32, element_size: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let word = e.mem.u32(array + 8);
    let capacity = (word & 0x3fff_ffff) as i32;
    if capacity < wanted {
        if (word as i32) >= 0 {
            let old = e.mem.u32(array);
            e.vcall(
                allocator,
                0x10,
                &args![old, (capacity as u32).wrapping_mul(element_size)],
            );
        }
        let bytes = (wanted as u32).wrapping_mul(element_size);
        let (buffer, granted) = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), bytes);
            let buffer = e.vcall(allocator, 0xc, &args![slot.addr()]).u32();
            (buffer, e.mem.u32(slot.addr()))
        });
        e.mem.set_u32(array, buffer);
        e.mem
            .set_u32(array + 8, (granted as i32 / element_size as i32) as u32);
    }
}

// Translated from 00e375a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleInstance::perWheelBehavior` (Xbox PDB name, engine map). Thiscall, five stack
/// words (`RET 0x14`): a float, `forces_ref` and `inputs_ref` (each the address of a word that
/// holds the address of a float array indexed by wheel), `ground_by_axle` (an array of body
/// pointers indexed by axle) and `axle_output` (axle records of `0x60` bytes). The chassis
/// speed is `SQRTSS` of the squared length of the vector at body `+0x1b0`. For every wheel (the
/// signed byte at data `+0x20`) it applies the suspension force
/// (`hkp_vehicle_instance_apply_suspension_forces_at_wheel`), keeps in `ground_by_axle[axle]`
/// the wheel record's ground body (`+0x24`) when the slot is empty or holds a body whose `w`
/// lane (`+0x1a0`, lane 3) is above that of the new one, and adds the wheel's contribution to
/// its axle record (`hkp_vehicle_instance_get_axle_params_from_wheel`). The axle is the signed
/// byte at parameter entry `+0x24`.
pub fn hkp_vehicle_instance_per_wheel_behavior(
    e: &mut Engine,
    this: Ptr,
    step: f32,
    forces_ref: u32,
    inputs_ref: u32,
    ground_by_axle: u32,
    axle_output: u32,
) {
    let t = this.addr();
    let speed = chassis_speed(e, t);
    let count = i32::from(e.mem.i8(e.mem.u32(t + 0x1c) + 0x20));
    if count <= 0 {
        return;
    }
    let mut wheel = 0u32;
    loop {
        let axle = i32::from(e.mem.i8(wheel_params(e, t, wheel) + 0x24)) as u32;
        let info = e.mem.u32(t + 0x48).wrapping_add(wheel.wrapping_mul(0xe0));
        let forces = e.mem.u32(forces_ref);
        let force = x87_quiet(e.mem.f32(forces + 4 * wheel));
        hkp_vehicle_instance_apply_suspension_forces_at_wheel(
            e,
            this,
            x87_quiet(step),
            wheel,
            force,
        );
        let ground = e.mem.u32(info + 0x24);
        if ground != 0 {
            let slot = ground_by_axle.wrapping_add(axle.wrapping_mul(4));
            let current = e.mem.u32(slot);
            let replace = if current == 0 {
                true
            } else {
                let new_w = load_vec(e, ground + 0x1a0)[3];
                let old_w = load_vec(e, current + 0x1a0)[3];
                old_w > new_w
            };
            if replace {
                e.mem.set_u32(slot, ground);
            }
        }
        let inputs = e.mem.u32(inputs_ref);
        let forces = e.mem.u32(forces_ref);
        let input = x87_quiet(e.mem.f32(inputs + 4 * wheel));
        let force = x87_quiet(e.mem.f32(forces + 4 * wheel));
        hkp_vehicle_instance_get_axle_params_from_wheel(
            e,
            this,
            wheel,
            input,
            force,
            x87_quiet(speed),
            axle_output.wrapping_add(axle.wrapping_mul(0x60)),
        );
        wheel += 1;
        let count = i32::from(e.mem.i8(e.mem.u32(t + 0x1c) + 0x20));
        if wheel as i32 >= count {
            break;
        }
    }
}

// Translated from 00e376e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a scaled vector into `output + 0x20`: unless the float at data `+0x6c` is zero (a NaN
/// counts as nonzero), `output[0x20..0x30] += splat((this[0xbc] * data[0x6c]) * factor) *
/// data[0x30..0x40] * data[0x180..0x190]` (the scalar in x87, rounded to `float`; the vectors
/// per lane in `f32`).
pub fn fn_00e376e0(e: &mut Engine, this: Ptr, factor: f32, output: u32) {
    let t = this.addr();
    let data = e.mem.u32(t + 0x1c);
    let mass = e.mem.f32(data + 0x6c);
    if mass == 0.0 {
        return;
    }
    let scalar = ((f64::from(e.mem.f32(t + 0xbc)) * f64::from(mass)) * f64::from(factor)) as f32;
    let first = load_vec(e, data + 0x30);
    let second = load_vec(e, data + 0x180);
    let old = load_vec(e, output + 0x20);
    store_vec(
        e,
        output + 0x20,
        addps(mulps(mulps(splat(scalar), first), second), old),
    );
}

// Translated from 00e37740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleInstance::prepareFrictionSolverParams` (Xbox PDB name, engine map). Thiscall, two
/// stack words (`RET 8`): `step_info` and the solver record `output`. Builds the chassis
/// motion's accumulator at `output + 0xd0` (`00d29830` over a one-entry list holding the body's
/// motion, body `+0xe0`), copies the vector at data `+0x180` to `output + 0x100`, runs `00d72030`
/// on `output + 0x150` with the accumulator, rotates `splat(w) * body[0x180..]` (`w` the float at
/// body `+0x14c`) by the matrix at `output + 0x120` into `output + 0x170` (`setRotatedDir`) and
/// stores `(body[0x140..] - body[0x130..]) * splat(w)` at `output + 0x160`.
pub fn hkp_vehicle_instance_prepare_friction_solver_params(
    e: &mut Engine,
    this: Ptr,
    step_info: u32,
    output: u32,
) {
    let t = this.addr();
    let body = e.mem.u32(t + 0x18);
    let accumulator = output + 0xd0;
    e.with_stack(4, |e, list| {
        e.mem.set_u32(list.addr(), body + 0xe0);
        e.call(
            BUILD_ACCUMULATORS,
            &args![step_info, list.addr(), 1u32, 0u32, accumulator],
        );
    });
    let body = e.mem.u32(t + 0x18);
    let w = x87_quiet(e.mem.f32(body + 0x14c));
    let upper = load_vec(e, body + 0x140);
    let lower = load_vec(e, body + 0x130);
    let difference = mulps(subps(upper, lower), splat(w));
    let body_vector = load_vec(e, body + 0xf0 + 0x90);
    let scaled = mulps(splat(w), body_vector);
    let data = e.mem.u32(t + 0x1c);
    copy_vec(e, data + 0x180, output + 0x100);
    e.call(BUILD_AXLE_BASIS, &args![output + 0x150, accumulator]);
    e.with_stack(16, |e, local| {
        store_vec(e, local.addr(), scaled);
        e.call(
            SET_ROTATED_DIR,
            &args![output + 0x170, output + 0x120, local.addr()],
        );
    });
    store_vec(e, output + 0x160, difference);
}

// Translated from 00e37820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleInstance::calcKMPH` (Xbox PDB name, engine map): the chassis speed (`SQRTSS` of the
/// squared length of body `+0x1b0`) divided by the double at `01017b70` and multiplied by the
/// double at `01012640` in x87 arithmetic, rounded to `float` (`ST0`).
pub fn hkp_vehicle_instance_calc_kmph(e: &mut Engine, this: Ptr) -> f32 {
    let speed = chassis_speed(e, this.addr());
    let divisor = e.mem.f64(0x0101_7b70);
    let factor = e.mem.f64(0x0101_2640);
    ((f64::from(speed) / divisor) * factor) as f32
}

// Translated from 00e37880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleInstance::calcMPH` (Xbox PDB name, engine map): as `calcKMPH`, but the divisor is
/// the double at `010eb828`.
pub fn hkp_vehicle_instance_calc_mph(e: &mut Engine, this: Ptr) -> f32 {
    let speed = chassis_speed(e, this.addr());
    let divisor = e.mem.f64(0x010e_b828);
    let factor = e.mem.f64(0x0101_2640);
    ((f64::from(speed) / divisor) * factor) as f32
}

// Translated from 00e378e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleInstance::addToWorld` (Xbox PDB name, engine map): adds the chassis body
/// (`this + 0x18`) to the world with `00c914d0` (the world as `this`, the body and `1`), then
/// calls slot `0x20` of the wheel-collide object (`this + 0x3c`) with the world.
pub fn hkp_vehicle_instance_add_to_world(e: &mut Engine, this: Ptr, world: u32) {
    let body = e.mem.u32(this.addr() + 0x18);
    e.call(0x00c9_14d0, &args![world, body, 1u32]);
    let wheel_collide = e.mem.u32(this.addr() + 0x3c);
    e.vcall(wheel_collide, 0x20, &args![world]);
}

// Translated from 00e37910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleInstance::removeFromWorld` (Xbox PDB name, engine map): calls slot `0x24` of the
/// wheel-collide object (`this + 0x3c`), then removes the chassis body from its world
/// (`00c91620` on the world at body `+8`, with the address of a one-byte result slot on the
/// stack, which the exe never initialises, and the body).
pub fn hkp_vehicle_instance_remove_from_world(e: &mut Engine, this: Ptr) {
    let wheel_collide = e.mem.u32(this.addr() + 0x3c);
    e.vcall(wheel_collide, 0x24, &args![]);
    let body = e.mem.u32(this.addr() + 0x18);
    let world = e.mem.u32(body + 8);
    e.with_stack(4, |e, slot| {
        e.call(0x00c9_1620, &args![world, slot.addr() + 3, body]);
    });
}

// Translated from 00e37940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleInstance::updateBeforeCollisionDetection` (Xbox PDB name, engine map). Between a
/// profile start record (label `010eb830`) and an end record (label `010c41f8`): for every
/// wheel (the signed byte at data `+0x20`), with `entries` the array at `[this + 0x34] + 8`
/// (`0x30` bytes per wheel) and the body transform at body `+0xf0`, it sets the wheel record's
/// `+0x80` to the rotation (`00c7f700`) of `entries[wheel] + 0x10` and its `+0x50` to the
/// transform (`00c7f640`) of `entries[wheel]`, then stores `splat(entries[wheel][0x20] +
/// params[wheel][0]) * record[0x80..] + record[0x50..]` at the record's `+0x60`. Finally slot
/// `0x18` of the wheel-collide object (`this + 0x3c`) is called with `this`.
pub fn hkp_vehicle_instance_update_before_collision_detection(e: &mut Engine, this: Ptr) {
    profile_record(e, UPDATE_PROFILE_LABEL);
    let t = this.addr();
    let body = e.mem.u32(t + 0x18);
    let data = e.mem.u32(t + 0x1c);
    let transform = body + 0xf0;
    if e.mem.i8(data + 0x20) > 0 {
        let mut wheel = 0u32;
        loop {
            let entry_offset = wheel.wrapping_mul(0x30);
            let info = e.mem.u32(t + 0x48).wrapping_add(wheel.wrapping_mul(0xe0));
            let entries = e.mem.u32(e.mem.u32(t + 0x34) + 8);
            e.call(
                SET_ROTATED_DIR,
                &args![info + 0x80, transform, entries + entry_offset + 0x10],
            );
            let entries = e.mem.u32(e.mem.u32(t + 0x34) + 8);
            e.call(
                SET_TRANSFORMED_POS,
                &args![info + 0x50, transform, entries + entry_offset],
            );
            let entries = e.mem.u32(e.mem.u32(t + 0x34) + 8);
            let base = f64::from(e.mem.f32(entries + entry_offset + 0x20));
            let extra = f64::from(e.mem.f32(wheel_params(e, t, wheel)));
            let scalar = (base + extra) as f32;
            let rotated = load_vec(e, info + 0x80);
            let moved = load_vec(e, info + 0x50);
            store_vec(e, info + 0x60, addps(mulps(splat(scalar), rotated), moved));
            wheel += 1;
            let count = i32::from(e.mem.i8(e.mem.u32(t + 0x1c) + 0x20));
            if wheel as i32 >= count {
                break;
            }
        }
    }
    let wheel_collide = e.mem.u32(t + 0x3c);
    e.vcall(wheel_collide, 0x18, &args![t]);
    profile_record(e, PROFILE_END);
}

// Translated from 00e37aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleInstance::updateWheels` (Xbox PDB name, engine map). Thiscall, two stack words
/// (`RET 8`); the first is never read, the second is `source`, an array of `0x60`-byte wheel
/// descriptions. For every wheel (the signed byte at data `+0x20`) it copies into the wheel
/// record (`this + 0x48`, `0xe0` bytes each): `source[0x48]` (a float) to `+0x70`,
/// `source[0..0x20]` to `+0` and `+0x10`, the float `source[0x20]` to `+0x20`, the dwords
/// `source[0x24..0x48]` to `+0x24..+0x48`; then builds the direction `h * entries[wheel][0x10..]`
/// with `h` the product of the float at `this[0xc0][wheel]` and the double at `01011588`
/// (lane 3 forced to `1.0`), normalises it over four lanes (`RSQRTSS`, one Newton step) into
/// `+0xb0`, copies data `+0x50` to `+0x90` and stores into a local the vector `2 * ((d x n) *
/// n.w + ((n.w * n.w - 0.5) * d + (d . n) * n))` for `d = +0x90`, `n = +0xb0` (the constant
/// vector `010c71e0` is `-0.5` four times), which is rotated (`setRotatedDir`, with the body
/// transform at body `+0xf0`) into `+0xa0`. `entries` is `[this + 0x34] + 8`, `0x30` bytes per
/// wheel.
pub fn hkp_vehicle_instance_update_wheels(e: &mut Engine, this: Ptr, _unused_1: u32, source: u32) {
    let t = this.addr();
    let transform = e.mem.u32(t + 0x18) + 0xf0;
    if e.mem.i8(e.mem.u32(t + 0x1c) + 0x20) <= 0 {
        return;
    }
    let mut wheel = 0u32;
    loop {
        let from = source.wrapping_add(wheel.wrapping_mul(0x60));
        let info_offset = wheel.wrapping_mul(0xe0);
        let info = e.mem.u32(t + 0x48).wrapping_add(info_offset);
        let bits = x87_quiet_bits(e.mem.u32(from + 0x48));
        e.mem.set_u32(info + 0x70, bits);
        copy_vec(e, from, info);
        copy_vec(e, from + 0x10, info + 0x10);
        let bits = x87_quiet_bits(e.mem.u32(from + 0x20));
        e.mem.set_u32(info + 0x20, bits);
        let word = e.mem.u32(from + 0x24);
        e.mem.set_u32(info + 0x24, word);
        for offset in (0x28..0x48).step_by(4) {
            let infos = e.mem.u32(t + 0x48);
            let word = e.mem.u32(from + offset);
            e.mem
                .set_u32(infos.wrapping_add(info_offset) + offset, word);
        }
        let ground_scale = e.mem.f32(e.mem.u32(t + 0xc0) + 4 * wheel);
        let entries = e.mem.u32(e.mem.u32(t + 0x34) + 8);
        let half = e.mem.f64(0x0101_1588);
        let scalar = (f64::from(ground_scale) * half) as f32;
        let entry = load_vec(e, entries + wheel.wrapping_mul(0x30) + 0x10);
        let mut direction = mulps(splat(scalar), entry);
        direction[3] = 1.0;
        let squares = mulps(direction, direction);
        let total = (squares[3] + squares[1]) + (squares[2] + squares[0]);
        let scale = inverse_length(e, total);
        store_vec(e, info + 0xb0, mulps(splat(scale), direction));
        let data = e.mem.u32(t + 0x1c);
        copy_vec(e, data + 0x50, info + 0x90);
        let n = load_vec(e, info + 0xb0);
        let d = load_vec(e, info + 0x90);
        let product = mulps(d, n);
        let dot = product[2] + (product[1] + product[0]);
        let cross = subps(
            mulps(shufps(d, d, 0xd2), shufps(n, n, 0xc9)),
            mulps(shufps(d, d, 0xc9), shufps(n, n, 0xd2)),
        );
        let w = shufps(n, n, 0xff);
        let crossed = mulps(cross, w);
        let halves = load_vec(e, NEGATIVE_HALVES);
        let along_d = mulps(addps(mulps(w, w), halves), d);
        let along_n = mulps(splat(dot), n);
        let sum = addps(crossed, addps(along_d, along_n));
        let doubled = addps(sum, sum);
        e.with_stack(16, |e, local| {
            store_vec(e, local.addr(), doubled);
            e.call(
                SET_ROTATED_DIR,
                &args![info + 0xa0, transform, local.addr()],
            );
        });
        wheel += 1;
        let count = i32::from(e.mem.i8(e.mem.u32(t + 0x1c) + 0x20));
        if wheel as i32 >= count {
            break;
        }
    }
}

// Translated from 00e37d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleInstance::getAxleParams` (Xbox PDB name, engine map). Thiscall, five stack words
/// (`RET 0x14`): `step_info`, `grounds` (the ground body pointer of each wheel), `accumulators`
/// (`0x80` bytes per wheel), `bases` (`0x80` bytes per wheel) and `axles` (`0x60` bytes per
/// wheel; its two vectors at `+0x10` and `+0x20` are normalised in place, the words at `+0x30`
/// and `+0x34` receive pointers). For each of the `int` at data `+0x9c` wheels: a null ground
/// body clears the accumulator (byte `+0` 1, float `+8` 0, vectors `+0x10`..`+0x70` zero). A
/// ground whose byte `+0xe8` is 5 is handed to slot `0x3c` of `this` (with the accumulator).
/// A later wheel with the same ground as wheel 0 takes wheel 0's two pointers. Otherwise
/// `00d29830` builds the accumulator from the ground's motion (body `+0xe0`); when the motion's
/// `w` lane (`+0xc0`, lane 3) is above `chassis w * data[0x88]` (x87), the accumulator's vector
/// `+0x30` is scaled by `data[0x88] * (chassis w / motion w)`; `00d72030` then fills the base
/// record from the accumulator and the base's `+0x20` becomes `b1 * acc[0x60] + b0 * acc[0x50] +
/// b2 * acc[0x70]` for `b = splat(motion[0x6c]) * motion[0xa0..]` and its `+0x10`
/// `(motion[0x60..] - motion[0x50..]) * splat(motion[0x6c])`.
pub fn hkp_vehicle_instance_get_axle_params(
    e: &mut Engine,
    this: Ptr,
    step_info: u32,
    grounds: u32,
    accumulators: u32,
    bases: u32,
    axles: u32,
) {
    let t = this.addr();
    let data = e.mem.u32(t + 0x1c);
    if e.mem.i32(data + 0x9c) <= 0 {
        return;
    }
    let mut wheel = 0i32;
    loop {
        let axle = axles.wrapping_add((wheel as u32).wrapping_mul(0x60));
        let accumulator = accumulators.wrapping_add((wheel as u32).wrapping_mul(0x80));
        let base = bases.wrapping_add((wheel as u32).wrapping_mul(0x80));
        normalise_in_place(e, axle + 0x10);
        normalise_in_place(e, axle + 0x20);
        let ground = e
            .mem
            .u32(grounds.wrapping_add((wheel as u32).wrapping_mul(4)));
        e.mem.set_u32(axle + 0x30, accumulator);
        e.mem.set_u32(axle + 0x34, accumulator);
        if ground == 0 {
            e.mem.set_f32(accumulator + 8, 0.0);
            e.mem.set_u8(accumulator, 1);
            for offset in [0x30, 0x10, 0x20, 0x40, 0x50, 0x60, 0x70] {
                store_vec(e, accumulator + offset, [0.0; 4]);
            }
        } else if e.mem.u8(ground + 0xe8) == 5 {
            e.vcall(t, 0x3c, &args![ground, accumulator]);
        } else if wheel > 0 && ground == e.mem.u32(grounds) {
            let first_a = e.mem.u32(axles + 0x30);
            let first_b = e.mem.u32(axles + 0x34);
            e.mem.set_u32(axle + 0x30, first_a);
            e.mem.set_u32(axle + 0x34, first_b);
        } else {
            let motion = ground + 0xe0;
            e.with_stack(4, |e, list| {
                e.mem.set_u32(list.addr(), motion);
                e.call(
                    BUILD_ACCUMULATORS,
                    &args![step_info, list.addr(), 1u32, 0u32, accumulator],
                );
            });
            let body = e.mem.u32(t + 0x18);
            let chassis_w = load_vec(e, body + 0x1a0)[3];
            let data = e.mem.u32(t + 0x1c);
            let limit = x87_quiet(e.mem.f32(data + 0x88));
            let motion_w = load_vec(e, motion + 0xc0)[3];
            if f64::from(motion_w) > f64::from(chassis_w) * f64::from(limit) {
                let ratio = (f64::from(chassis_w) / f64::from(motion_w)) as f32;
                let factor = (f64::from(limit) * f64::from(ratio)) as f32;
                let old = load_vec(e, accumulator + 0x30);
                store_vec(e, accumulator + 0x30, mulps(splat(factor), old));
            }
            let scalar = x87_quiet(e.mem.f32(motion + 0x6c));
            let upper = load_vec(e, motion + 0x60);
            let lower = load_vec(e, motion + 0x50);
            let first = mulps(subps(upper, lower), splat(scalar));
            let motion_axes = load_vec(e, motion + 0xa0);
            let second = mulps(splat(scalar), motion_axes);
            e.call(BUILD_AXLE_BASIS, &args![base, accumulator]);
            e.mem.set_u32(axle + 0x34, base);
            let acc_60 = load_vec(e, accumulator + 0x60);
            let acc_50 = load_vec(e, accumulator + 0x50);
            let acc_70 = load_vec(e, accumulator + 0x70);
            let mixed = addps(
                mulps(shufps(second, second, 0x55), acc_60),
                mulps(shufps(second, second, 0x00), acc_50),
            );
            let mixed = addps(mixed, mulps(shufps(second, second, 0xaa), acc_70));
            store_vec(e, base + 0x20, mixed);
            store_vec(e, base + 0x10, first);
        }
        wheel += 1;
        let data = e.mem.u32(t + 0x1c);
        if wheel >= e.mem.i32(data + 0x9c) {
            break;
        }
    }
}

// Translated from 00e38080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, two stack words (`RET 8`): builds an 8-byte local from the floats at `step_info +
/// 8` and `+0xc` (`FLD`/`FSTP` copies) and calls the cdecl function `00e59a40` with (the local,
/// data `+0xa4`, `argument`, `this + 0x54`).
pub fn fn_00e38080(e: &mut Engine, this: Ptr, step_info: u32, argument: u32) {
    let t = this.addr();
    let first = x87_quiet_bits(e.mem.u32(step_info + 8));
    let second = x87_quiet_bits(e.mem.u32(step_info + 0xc));
    let data = e.mem.u32(t + 0x1c);
    e.with_stack(8, |e, local| {
        e.mem.set_u32(local.addr(), first);
        e.mem.set_u32(local.addr() + 4, second);
        e.call(
            0x00e5_9a40,
            &args![local.addr(), data + 0xa4, argument, t + 0x54],
        );
    });
}

// Translated from 00e380c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, one stack word (`RET 4`): the vector `splat(v.y) * body[0x100..] + splat(v.x) *
/// body[0xf0..] + splat(v.z) * body[0x110..]` (`v` the vector at `vector + 0x20`) is passed to
/// slot `0x44` of the chassis motion (body `+0xe0`), then `vector + 0x10` to slot `0x40`.
pub fn fn_00e380c0(e: &mut Engine, this: Ptr, vector: u32) {
    let body = e.mem.u32(this.addr() + 0x18);
    let column_y = load_vec(e, body + 0x100);
    let column_x = load_vec(e, body + 0xf0);
    let v = load_vec(e, vector + 0x20);
    let motion = body + 0xe0;
    let column_z = load_vec(e, motion + 0x30);
    let result = addps(
        addps(
            mulps(shufps(v, v, 0x55), column_y),
            mulps(shufps(v, v, 0x00), column_x),
        ),
        mulps(shufps(v, v, 0xaa), column_z),
    );
    e.with_stack(16, |e, local| {
        store_vec(e, local.addr(), result);
        e.vcall(motion, 0x44, &args![local.addr()]);
    });
    e.vcall(motion, 0x40, &args![vector + 0x10]);
}

// Translated from 00e38140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, three stack words (`RET 0xc`): a float, `records` (axle records of `0x60` bytes,
/// each with a pointer to an accumulator at `+0x30`) and `bodies` (a ground body per axle).
/// For each of the `int` at data `+0x9c` entries whose body is not null, whose byte `+0xe8` is
/// not 5 and, past the first, whose first two bodies differ: with `limit` the float times the
/// double at `01020758` rounded to `float`, the accumulator's vector `+0x20` times the matrix
/// whose rows are `+0x50`, `+0x60`, `+0x70` (the `UNPCK`/`SHUFPS` transpose of
/// `hkp_box_motion`'s helper) and the 16 bytes at `+0x10` are clamped to within `limit` of the
/// motion's vectors at `+0xe0` and `+0xd0` respectively (`MINPS`, `MAXPS` with the negated
/// limit through the sign mask at `010c44d0`) and passed to slots `0x44` and `0x40` of the
/// body's motion (body `+0xe0`).
pub fn fn_00e38140(e: &mut Engine, this: Ptr, factor: f32, records: u32, bodies: u32) {
    let t = this.addr();
    let data = e.mem.u32(t + 0x1c);
    if e.mem.i32(data + 0x9c) <= 0 {
        return;
    }
    let mut index = 0u32;
    loop {
        let body = e.mem.u32(bodies.wrapping_add(index.wrapping_mul(4)));
        let skipped = body == 0
            || e.mem.u8(body + 0xe8) == 5
            || (index != 0 && e.mem.u32(bodies) == e.mem.u32(bodies + 4));
        if !skipped {
            let motion = body + 0xe0;
            let record = e
                .mem
                .u32(records.wrapping_add(index.wrapping_mul(0x60)) + 0x30);
            let ten = e.mem.f64(0x0102_0758);
            let limit = (f64::from(factor) * ten) as f32;
            let row_0 = load_vec(e, record + 0x50);
            let row_1 = load_vec(e, record + 0x60);
            let row_2 = load_vec(e, record + 0x70);
            let v = load_vec(e, record + 0x20);
            let first = rows_times(v, row_0, row_1, row_2);
            let second = load_vec(e, record + 0x10);
            let linear = load_vec(e, motion + 0xe0);
            let angular = load_vec(e, motion + 0xd0);
            let sign = load_bits(e, SIGN_MASK);
            let negated = [0, 1, 2, 3].map(|lane| f32::from_bits(limit.to_bits() ^ sign[lane]));
            let clamp = |x: Vec4, centre: Vec4| -> Vec4 {
                let low = zip_with(subps(x, centre), [limit; 4], minps);
                addps(zip_with(low, negated, maxps), centre)
            };
            let first = clamp(first, linear);
            let second = clamp(second, angular);
            e.with_stack(32, |e, local| {
                store_vec(e, local.addr(), first);
                store_vec(e, local.addr() + 16, second);
                e.vcall(motion, 0x44, &args![local.addr()]);
                e.vcall(motion, 0x40, &args![local.addr() + 16]);
            });
        }
        index += 1;
        let data = e.mem.u32(t + 0x1c);
        if index as i32 >= e.mem.i32(data + 0x9c) {
            break;
        }
    }
}

// Translated from 00e38310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleInstance::applyResultsToWheelInfo` (Xbox PDB name, engine map). Thiscall, three
/// stack words (`RET 0xc`): a float, `axle_grounds` (a ground body per axle) and `axle_records`
/// (`0x60` bytes per axle, a pointer to an accumulator at `+0x30`). For every wheel (the signed
/// byte at data `+0x20`) with `axle` the signed byte at its parameter entry `+0x24` and `axle
/// data` the record at `this + 0x54 + 0x24 * axle`: wheel record `+0xc8` gets the axle data's
/// float `+8` when the wheel's ground body (`+0x24`) has byte `+0xe8` equal to 5, else `0.0`;
/// `+0xcc`, `+0xd4`, `+0xd0` get the axle data's floats `+0xc`, `+4`, `+0`. The body transform
/// (body `+0xf0`) rotates data `+0x40` into a local `w`. A wheel whose flag byte (`this +
/// 0xa0`) is set gets `+0xc0 = 0`; otherwise `u = dot(body[0x1b0..], w) + record[0xd0]` and,
/// when the axle has a ground body, `u -= dot(accumulator[0x10..], w)`; then `+0xc0 = u /
/// parameter[0]` and `+0xc4 += (+0xc0 * float)` (x87 arithmetic, rounded to `float` at each
/// store).
pub fn hkp_vehicle_instance_apply_results_to_wheel_info(
    e: &mut Engine,
    this: Ptr,
    factor: f32,
    axle_grounds: u32,
    axle_records: u32,
) {
    let t = this.addr();
    if e.mem.i8(e.mem.u32(t + 0x1c) + 0x20) <= 0 {
        return;
    }
    let mut wheel = 0u32;
    loop {
        let entry = wheel_params(e, t, wheel);
        let axle = i32::from(e.mem.i8(entry + 0x24)) as u32;
        let info = e.mem.u32(t + 0x48).wrapping_add(wheel.wrapping_mul(0xe0));
        let axle_data = t + 0x54 + axle.wrapping_mul(0x24);
        let ground = e.mem.u32(info + 0x24);
        let landed = if ground != 0 && e.mem.u8(ground + 0xe8) == 5 {
            x87_quiet_bits(e.mem.u32(axle_data + 8))
        } else {
            0
        };
        e.mem.set_u32(info + 0xc8, landed);
        for (from, to) in [(0xc, 0xcc), (4, 0xd4), (0, 0xd0)] {
            let bits = x87_quiet_bits(e.mem.u32(axle_data + from));
            e.mem.set_u32(info + to, bits);
        }
        let data = e.mem.u32(t + 0x1c);
        let body = e.mem.u32(t + 0x18);
        e.with_stack(16, |e, local| {
            e.call(
                SET_ROTATED_DIR,
                &args![local.addr(), body + 0xf0, data + 0x40],
            );
            let w = load_vec(e, local.addr());
            let chassis = e.mem.u32(t + 0x18);
            let velocity = load_vec(e, chassis + 0x1b0);
            let flags = e.mem.u32(t + 0xa0);
            let flagged = e.mem.u8(flags.wrapping_add(wheel)) != 0;
            let speed = sum_of_three(mulps(velocity, w));
            if flagged {
                e.mem.set_f32(info + 0xc0, 0.0);
            } else {
                let mut value = (f64::from(speed) + f64::from(e.mem.f32(info + 0xd0))) as f32;
                if e.mem.u32(axle_grounds.wrapping_add(axle.wrapping_mul(4))) != 0 {
                    let record = axle_records.wrapping_add(axle.wrapping_mul(0x60));
                    let accumulator = e.mem.u32(record + 0x30);
                    let direction = load_vec(e, accumulator + 0x10);
                    let along = sum_of_three(mulps(direction, w));
                    value = (f64::from(value) - f64::from(along)) as f32;
                }
                let divisor = e.mem.f32(wheel_params(e, t, wheel));
                let quotient = (f64::from(value) / f64::from(divisor)) as f32;
                e.mem.set_f32(info + 0xc0, quotient);
                let scaled = (f64::from(quotient) * f64::from(factor)) as f32;
                let old = f64::from(e.mem.f32(info + 0xc4));
                e.mem.set_f32(info + 0xc4, (f64::from(scaled) + old) as f32);
            }
        });
        wheel += 1;
        let count = i32::from(e.mem.i8(e.mem.u32(t + 0x1c) + 0x20));
        if wheel as i32 >= count {
            break;
        }
    }
}

// Translated from 00e384e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleInstance::calcCurrentPositionAndRotation` (Xbox PDB name, engine map). Thiscall,
/// five stack words (`RET 0x14`): `frame` (an object with a matrix at `+0xf0`..`+0x120` and a
/// quaternion at `+0x160`), `entries` (the array at `entries[2]`, `0x30` bytes per wheel), the
/// wheel index, `position` (output vector) and `rotation` (output quaternion). With `info`
/// the wheel record: a local quaternion from `setAxisAngle` (`00cb2450`) of the axis `info +
/// 0x90` and the angle `-info[0xc4]`, `local2 = frame.quaternion * info[0xb0]` (`00c66320`)
/// and `*rotation = local2 * local1`. The position is `entry.y * m[0x100] + entry.x * m[0xf0]
/// + (entry.z * m[0x110] + m[0x120])` plus `splat(s) * info[0x80..]` where `s` is the float at
/// `info + 0x70`, or `0.0` when that float is below the double at `01012060` (a NaN stays).
pub fn hkp_vehicle_instance_calc_current_position_and_rotation(
    e: &mut Engine,
    this: Ptr,
    frame: u32,
    entries: u32,
    wheel: u32,
    position: u32,
    rotation: u32,
) {
    let info = e
        .mem
        .u32(this.addr() + 0x48)
        .wrapping_add(wheel.wrapping_mul(0xe0));
    let angle = f32::from_bits(x87_quiet_bits(e.mem.u32(info + 0xc4)) ^ 0x8000_0000);
    e.with_stack(32, |e, locals| {
        let first = locals.addr();
        let second = locals.addr() + 16;
        e.call(0x00cb_2450, &args![first, info + 0x90, angle]);
        e.call(0x00c6_6320, &args![second, frame + 0x160, info + 0xb0]);
        e.call(0x00c6_6320, &args![rotation, second, first]);
    });
    let spin = x87_quiet(e.mem.f32(info + 0x70));
    let spin = if f64::from(spin) < e.mem.f64(0x0101_2060) {
        0.0
    } else {
        spin
    };
    let entry = e
        .mem
        .u32(entries + 8)
        .wrapping_add(wheel.wrapping_mul(0x30));
    let q = load_vec(e, entry);
    let column_y = load_vec(e, frame + 0x100);
    let column_x = load_vec(e, frame + 0xf0);
    let column_z = load_vec(e, frame + 0x110);
    let origin = load_vec(e, frame + 0x120);
    let low = addps(
        mulps(shufps(q, q, 0x55), column_y),
        mulps(shufps(q, q, 0x00), column_x),
    );
    let high = addps(mulps(shufps(q, q, 0xaa), column_z), origin);
    let result = addps(low, high);
    store_vec(e, position, result);
    let direction = load_vec(e, info + 0x80);
    store_vec(e, position, addps(mulps(splat(spin), direction), result));
}

// Translated from 00e385e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleInstance::simulateVehicle` (Xbox PDB name, engine map). Thiscall, four stack words
/// (`RET 0x10`): `step_info` (its float at `+8` is the time step `dt`), `vectors`, `forces_ref`
/// and `inputs_ref`. Between profile records (labels `010eb844` and `010c41f8`) it activates the
/// chassis body, calls slot `0x5c` of its motion (body `+0xe0`) with (`dt`, `vectors`) and slot
/// `0x60` with (`dt`, `vectors + 0x10`), calls slot `0xc` of the object at `this + 0x44`, when
/// there is one, with (`dt`, `this`), and runs the step on one stack frame of `0x3f0` bytes (the
/// exe's own layout, kept because the callees overlap its parts): two zeroed axle records of
/// `0x60` bytes at `+0x20`, a two-word table of ground bodies at `+0x18`, then
/// `perWheelBehavior`, `getAxleParams` (accumulators at `+0x1f0`, bases at `+0x2f0`),
/// `prepareFrictionSolverParams` on the axle records, `00e376e0` on `+0xf0`, `00e38080`,
/// `00e380c0` on `+0xf0`, `00e38140` and `applyResultsToWheelInfo`. The float at `+0xe0` (data
/// `+0x70`) is stored but the exe never reads it back.
pub fn hkp_vehicle_instance_simulate_vehicle(
    e: &mut Engine,
    this: Ptr,
    step_info: u32,
    vectors: u32,
    forces_ref: u32,
    inputs_ref: u32,
) {
    profile_record(e, 0x010e_b844);
    let t = this.addr();
    let body = e.mem.u32(t + 0x18);
    let dt = x87_quiet(e.mem.f32(step_info + 8));
    e.call(ENTITY_ACTIVATE, &args![body]);
    e.vcall(body + 0xe0, 0x5c, &args![dt, vectors]);
    let body = e.mem.u32(t + 0x18);
    e.call(ENTITY_ACTIVATE, &args![body]);
    e.vcall(body + 0xe0, 0x60, &args![dt, vectors + 0x10]);
    let observer = e.mem.u32(t + 0x44);
    if observer != 0 {
        e.vcall(observer, 0xc, &args![dt, t]);
    }
    e.with_stack(0x400, |e, frame| {
        let f = frame.addr();
        let records = f + 0x20;
        for record in [records, records + 0x60] {
            for offset in [0, 0x10, 0x20] {
                store_vec(e, record + offset, [0.0; 4]);
            }
            e.mem.set_u32(record + 0x30, 0);
            e.mem.set_u32(record + 0x34, 0);
            for offset in (0x38..0x50).step_by(4) {
                e.mem.set_f32(record + offset, 0.0);
            }
            e.mem.set_u8(record + 0x50, 0);
        }
        let grounds = f + 0x18;
        e.mem.set_u32(grounds, 0);
        e.mem.set_u32(grounds + 4, 0);
        hkp_vehicle_instance_per_wheel_behavior(
            e, this, dt, forces_ref, inputs_ref, grounds, records,
        );
        let data = e.mem.u32(t + 0x1c);
        let bits = x87_quiet_bits(e.mem.u32(data + 0x70));
        e.mem.set_u32(f + 0xe0, bits);
        hkp_vehicle_instance_get_axle_params(
            e,
            this,
            step_info,
            grounds,
            f + 0x1f0,
            f + 0x2f0,
            records,
        );
        hkp_vehicle_instance_prepare_friction_solver_params(e, this, step_info, records);
        fn_00e376e0(e, this, dt, f + 0xf0);
        fn_00e38080(e, this, step_info, records);
        fn_00e380c0(e, this, f + 0xf0);
        fn_00e38140(e, this, dt, records, grounds);
        hkp_vehicle_instance_apply_results_to_wheel_info(e, this, dt, grounds, records);
    });
    profile_record(e, PROFILE_END);
}

// Translated from 00e387e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleInstance::init` (Xbox PDB name, engine map). When the data block's byte `+0x190`
/// is zero, calls slot `0xc` of the data block with (`[this + 0x34] + 8`, the chassis body).
/// The chassis inertia is then rebuilt: the three data vectors at `+0x30`, `+0x40`, `+0x50`
/// (sign bits cleared with the mask at `01017d10`) are scaled by the floats at `+0x74`, `+0x78`,
/// `+0x7c`, summed and multiplied by the body mass (`00c86430` on the motion at body `+0xe0`);
/// the three lanes become the diagonal of a matrix (rows `0x10` bytes apart, the lanes copied
/// through `FLD`/`FSTP`) that is handed to `00ca8920` after the second mass read goes to
/// `00ca88c0`. The wheel record array (`+0x48`), the per-wheel byte array (`+0xa0`) and the
/// per-wheel float array (`+0xc0`) are then reserved for the wheel count (signed byte at data
/// `+0x20`) and sized to it; every record is initialised (`00e370b0`), the bytes are cleared and
/// the floats set to `0.0`, as are the fields `+0xac`, `+0xbc`, `+0xb4`, `+0xb8`, `+0xd0` and
/// the bytes `+0xcc`..`+0xce`. Finally slot `0xc` of the wheel-collide object (`+0x3c`) is called
/// with `this` and its byte `+8` is set to `1`.
pub fn hkp_vehicle_instance_init(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    let data = e.mem.u32(t + 0x1c);
    if e.mem.u8(data + 0x190) == 0 {
        let target = e.mem.u32(t + 0x34).wrapping_add(8);
        let body = e.mem.u32(t + 0x18);
        e.vcall(data, 0xc, &args![target, body]);
    }
    let data = e.mem.u32(t + 0x1c);
    let mask = load_bits(e, ABS_MASK);
    let scaled = |e: &Engine, vector: u32, scale: u32| -> Vec4 {
        let bits = load_bits(e, data + vector);
        let magnitude = [0, 1, 2, 3].map(|lane| f32::from_bits(bits[lane] & mask[lane]));
        mulps(magnitude, splat(e.mem.f32(data + scale)))
    };
    let first = scaled(&*e, 0x30, 0x74);
    let second = scaled(&*e, 0x40, 0x78);
    let third = scaled(&*e, 0x50, 0x7c);
    let sum = addps(addps(first, second), third);
    let body = e.mem.u32(t + 0x18);
    let mass = e.call(MOTION_GET_MASS, &args![body + 0xe0]).f32();
    let diagonal = mulps(splat(mass), sum);
    let second_mass = e.call(MOTION_GET_MASS, &args![body + 0xe0]).f32();
    e.with_stack(0x30, |e, matrix| {
        let m = matrix.addr();
        store_vec(e, m, [x87_quiet(diagonal[0]), 0.0, 0.0, 0.0]);
        store_vec(e, m + 0x10, [0.0, x87_quiet(diagonal[1]), 0.0, 0.0]);
        store_vec(e, m + 0x20, [0.0, 0.0, x87_quiet(diagonal[2]), 0.0]);
        let body = e.mem.u32(t + 0x18);
        e.call(BODY_SET_MASS, &args![body, x87_quiet(second_mass)]);
        let body = e.mem.u32(t + 0x18);
        e.call(BODY_SET_INERTIA, &args![body, m]);
    });
    let data = e.mem.u32(t + 0x1c);
    let wheels = i32::from(e.mem.i8(data + 0x20));
    reserve_array(e, t + 0x48, wheels, 0xe0);
    e.mem.set_u32(t + 0x4c, wheels as u32);
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(t + 0x4c) {
        let record = e.mem.u32(t + 0x48).wrapping_add(index.wrapping_mul(0xe0));
        fn_00e370b0(e, Ptr::new(record));
        index += 1;
    }
    let data = e.mem.u32(t + 0x1c);
    let wheels = i32::from(e.mem.i8(data + 0x20));
    reserve_array(e, t + 0xa0, wheels, 1);
    e.mem.set_u32(t + 0xa4, wheels as u32);
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(t + 0xa4) {
        let flags = e.mem.u32(t + 0xa0);
        e.mem.set_u8(flags.wrapping_add(index), 0);
        index += 1;
    }
    e.mem.set_f32(t + 0xac, 0.0);
    e.mem.set_f32(t + 0xbc, 0.0);
    let data = e.mem.u32(t + 0x1c);
    let wheels = i32::from(e.mem.i8(data + 0x20));
    reserve_array(e, t + 0xc0, wheels, 4);
    e.mem.set_u32(t + 0xc4, wheels as u32);
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(t + 0xc4) {
        let floats = e.mem.u32(t + 0xc0);
        e.mem.set_f32(floats.wrapping_add(index * 4), 0.0);
        index += 1;
    }
    e.mem.set_f32(t + 0xb4, 0.0);
    e.mem.set_f32(t + 0xb8, 0.0);
    for k in 0..3 {
        e.mem.set_u8(t + 0xcc + k, 0);
    }
    e.mem.set_f32(t + 0xd0, 0.0);
    let wheel_collide = e.mem.u32(t + 0x3c);
    e.vcall(wheel_collide, 0xc, &args![t]);
    let wheel_collide = e.mem.u32(t + 0x3c);
    e.mem.set_u8(wheel_collide + 8, 1);
}

// Translated from 00e38a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Component step with copies in and out of a result block (`target`). Thiscall, three stack
/// words (`RET 0xc`): a float, `word` and `target`. The per-wheel byte array of `this` (`+0xa0`,
/// `+0xa4` entries) is copied to the byte array at `target + 0xc` (reserved, size set), the float
/// array at `target` is reserved and sized to the same count (its contents are not written),
/// the float at `this + 0xac` is copied to `target + 0x18`, slot `0xc` of the object at
/// `this + 0x30` is called with (the float, `this`, `word`, `target`), the bytes at
/// `target + 0xc` (as many as its size at `target + 0x10`) are copied back to `this + 0xa0` and
/// the float at `target + 0x18` returns to `this + 0xac`.
pub fn fn_00e38a30(e: &mut Engine, this: Ptr, value: f32, word: u32, target: u32) {
    let t = this.addr();
    let wheels = e.mem.i32(t + 0xa4);
    reserve_array(e, target + 0xc, wheels, 1);
    e.mem.set_u32(target + 0x10, wheels as u32);
    let wheels = e.mem.i32(t + 0xa4);
    reserve_array(e, target, wheels, 4);
    e.mem.set_u32(target + 4, wheels as u32);
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(t + 0xa4) {
        let source = e.mem.u32(t + 0xa0);
        let byte = e.mem.u8(source.wrapping_add(index));
        let destination = e.mem.u32(target + 0xc);
        e.mem.set_u8(destination.wrapping_add(index), byte);
        index += 1;
    }
    let float_bits = x87_quiet_bits(e.mem.u32(t + 0xac));
    e.mem.set_u32(target + 0x18, float_bits);
    let component = e.mem.u32(t + 0x30);
    e.vcall(component, 0xc, &args![x87_quiet(value), t, word, target]);
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(target + 0x10) {
        let source = e.mem.u32(target + 0xc);
        let byte = e.mem.u8(source.wrapping_add(index));
        let destination = e.mem.u32(t + 0xa0);
        e.mem.set_u8(destination.wrapping_add(index), byte);
        index += 1;
    }
    let float_bits = x87_quiet_bits(e.mem.u32(target + 0x18));
    e.mem.set_u32(t + 0xac, float_bits);
}

// Translated from 00e38b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleInstance::hkpVehicleInstance` (Xbox PDB name, engine map): thiscall, one stack
/// word (`RET 4`), returns `this`. Runs the `hkpUnaryAction` constructor (`00d2b260`, with the
/// word and `0`), sets the vtable (`010df34c`), empties the three arrays (`+0x48`, `+0xa0`,
/// `+0xc0`: pointer and size `0`, capacity word `0x80000000`), clears the floats `+0x54`..`+0x98`
/// and the pointer fields `+0x1c`..`+0x44` and `+0x9c`.
pub fn hkp_vehicle_instance_constructor(e: &mut Engine, this: Ptr, entity: u32) -> Ptr {
    let t = this.addr();
    e.call(UNARY_ACTION_CONSTRUCTOR, &args![t, entity, 0u32]);
    e.mem.set_u32(t, 0x010d_f34c);
    for array in [0x48, 0xa0, 0xc0] {
        e.mem.set_u32(t + array, 0);
        e.mem.set_u32(t + array + 4, 0);
        e.mem.set_u32(t + array + 8, 0x8000_0000);
    }
    for offset in (0x54..=0x98).step_by(4) {
        e.mem.set_f32(t + offset, 0.0);
    }
    for offset in (0x1c..=0x44).step_by(4) {
        e.mem.set_u32(t + offset, 0);
    }
    e.mem.set_u32(t + 0x9c, 0);
    this
}

// Translated from 00e38bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleInstance::~hkpVehicleInstance` (Xbox PDB name, engine map): puts the vtable
/// (`010df34c`) back, releases (`00c905b0`) the objects at `+0x1c`, `+0x20`, `+0x24`, `+0x28`,
/// `+0x2c`, `+0x30`, `+0x34`, `+0x38`, `+0x3c`, `+0x44`, `+0x9c`, `+0x40` (in that order, each
/// when non-null), frees the arrays at `+0xc0`, `+0xa0` and `+0x48`, and ends by jumping to the
/// `hkpUnaryAction` destructor (`00d2b2b0`) with `this`.
pub fn hkp_vehicle_instance_destructor(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.mem.set_u32(t, 0x010d_f34c);
    for offset in [
        0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c, 0x44, 0x9c, 0x40,
    ] {
        let object = e.mem.u32(t + offset);
        if object != 0 {
            e.call(REMOVE_REFERENCE, &args![object]);
        }
    }
    free_array(e, t + 0xc0, 4);
    free_array(e, t + 0xa0, 1);
    free_array(e, t + 0x48, 0xe0);
    e.call(UNARY_ACTION_DESTRUCTOR, &args![t]);
}

// Translated from 00e38d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Component step with a local copy of the per-wheel floats. Thiscall, two stack words
/// (`RET 8`): a float and `word`. The float at `this + 0xbc` is put in a local, the floats at
/// `this + 0xc0` (`+0xc4` of them) are copied into a local `hkArray<float>` (reserved first),
/// slot `0xc` of the object at `this + 0x24` is called with (the float, `this`, `word`, the
/// address of the local float), then the local float returns to `this + 0xbc`, the array floats
/// are copied back and the local array's buffer is freed through the allocator (slot `0x10`)
/// when its capacity word is not negative. Every float copy is an `FLD`/`FSTP` pair.
pub fn fn_00e38d50(e: &mut Engine, this: Ptr, value: f32, word: u32) {
    let t = this.addr();
    let count = e.mem.i32(t + 0xc4);
    e.with_stack(16, |e, local| {
        let l = local.addr();
        let held = x87_quiet_bits(e.mem.u32(t + 0xbc));
        e.mem.set_u32(l, held);
        e.mem.set_u32(l + 4, 0);
        e.mem.set_u32(l + 8, 0);
        e.mem.set_u32(l + 12, 0x8000_0000);
        reserve_array(e, l + 4, count, 4);
        e.mem.set_u32(l + 8, count as u32);
        let mut index = 0u32;
        while (index as i32) < e.mem.i32(t + 0xc4) {
            let source = e.mem.u32(t + 0xc0);
            let bits = x87_quiet_bits(e.mem.u32(source.wrapping_add(index * 4)));
            let destination = e.mem.u32(l + 4);
            e.mem.set_u32(destination.wrapping_add(index * 4), bits);
            index += 1;
        }
        let component = e.mem.u32(t + 0x24);
        e.vcall(component, 0xc, &args![x87_quiet(value), t, word, l]);
        let held = x87_quiet_bits(e.mem.u32(l));
        e.mem.set_u32(t + 0xbc, held);
        let mut index = 0u32;
        while (index as i32) < e.mem.i32(t + 0xc4) {
            let source = e.mem.u32(l + 4);
            let bits = x87_quiet_bits(e.mem.u32(source.wrapping_add(index * 4)));
            let destination = e.mem.u32(t + 0xc0);
            e.mem.set_u32(destination.wrapping_add(index * 4), bits);
            index += 1;
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(l + 12);
        e.mem.set_u32(l + 8, 0);
        if (capacity as i32) >= 0 {
            let buffer = e.mem.u32(l + 4);
            e.vcall(
                allocator,
                0x10,
                &args![buffer, (capacity & 0x3fff_ffff) * 4],
            );
        }
    });
}

// Translated from 00e39400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor-shaped release of a pair of arrays (class unconfirmed; thiscall, no stack words):
/// the byte array at `this + 0xc` (pointer, size, capacity word) is emptied and its buffer freed
/// through the allocator (slot `0x10`) when the capacity word is not negative, then the same for
/// the float array at `this` (buffer size four bytes per capacity unit). Both end as pointer `0`,
/// size `0` and capacity word `0x80000000`.
pub fn fn_00e39400(e: &mut Engine, this: Ptr) {
    free_array(e, this.addr() + 0xc, 1);
    free_array(e, this.addr(), 4);
}

// Translated from 00e38e60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00e38e60 - the final loop adds `block[i]`, words of a block the
// function takes from the memory router's stack allocator (it moves the top by the wheel count
// times four, rounded up to 0x80) and never writes or hands to a callee; the exe reads whatever
// earlier users of that stack left there, which the translation can only read from engine memory.
/// `hkpVehicleInstance::updateComponents` (Xbox PDB name, engine map). Thiscall, five stack
/// words (`RET 0x14`): `step_info` (its float at `+8` is the time step `dt`, rounded to `float`
/// by the exe's `FSTP`), `wheel_source`, `vectors`, `inputs_ref` and `outputs_ref`. Between
/// profile records (labels `010eb858` and `010c41f8`) it runs, with `dt` as the float: `00e37aa0`
/// (`wheel_source`), `00e36f60` (local block `A`), `00e38d50` (`A`), `00e36fa0` (local block `B`),
/// `00e37020` (`A`, `B`), `00e38a30` (`A`, the local result object, a float array and a byte
/// array of the `00e39400` shape), `00e371a0` (`wheel_source`, `inputs_ref`) and `00e37080`
/// (`vectors`). Then, for every wheel `i` (signed byte at data `+0x20`), the float array that
/// `outputs_ref` points to gets `(result_floats[i] + block[i]) / params[i].float0` (x87
/// arithmetic, the sum rounded to `float` first; `params` the array at data `+0x8c`, `0x28`
/// bytes each). The router's stack top is restored and the result object released (`00e39400`).
/// The frame is laid out as in the exe (`A` at `+4`, `B` at `+0x14`, the saved block pointer at
/// `+0x1c`, the result object at `+0x28`).
pub fn hkp_vehicle_instance_update_components(
    e: &mut Engine,
    this: Ptr,
    step_info: u32,
    wheel_source: u32,
    vectors: u32,
    inputs_ref: u32,
    outputs_ref: u32,
) {
    let t = this.addr();
    profile_record(e, UPDATE_COMPONENTS_LABEL);
    e.with_stack(0x44, |e, frame| {
        let f = frame.addr();
        let block_a = f + 4;
        let block_b = f + 0x14;
        let saved_block = f + 0x1c;
        let result = f + 0x28;
        e.mem.set_u32(result, 0);
        e.mem.set_u32(result + 4, 0);
        e.mem.set_u32(result + 8, 0x8000_0000);
        e.mem.set_u32(result + 0xc, 0);
        e.mem.set_u32(result + 0x10, 0);
        e.mem.set_u32(result + 0x14, 0x8000_0000);
        let data = e.mem.u32(t + 0x1c);
        let wheels = i32::from(e.mem.i8(data + 0x20));
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let top = e.mem.u32(router);
        let reserved =
            ((wheels as u32).wrapping_mul(4).wrapping_add(0x7f) & 0xffff_ff80).wrapping_add(top);
        e.mem.set_u32(router, reserved);
        let dt = x87_quiet(e.mem.f32(step_info + 8));
        e.mem.set_u32(saved_block, top);
        hkp_vehicle_instance_update_wheels(e, this, dt.to_bits(), wheel_source);
        fn_00e36f60(e, this, dt, block_a);
        fn_00e38d50(e, this, dt, block_a);
        fn_00e36fa0(e, this, dt, block_b);
        fn_00e37020(e, this, dt, block_a, block_b);
        fn_00e38a30(e, this, dt, block_a, result);
        fn_00e371a0(e, this, dt, wheel_source, inputs_ref);
        fn_00e37080(e, this, dt, vectors);
        let data = e.mem.u32(t + 0x1c);
        if e.mem.i8(data + 0x20) > 0 {
            let outputs = e.mem.u32(outputs_ref);
            let mut index = 0u32;
            loop {
                let floats = e.mem.u32(result);
                let first = f64::from(e.mem.f32(floats.wrapping_add(index * 4)));
                let block = e.mem.u32(saved_block);
                let second = f64::from(e.mem.f32(block.wrapping_add(index * 4)));
                let sum = (first + second) as f32;
                let divisor = e.mem.f32(e.mem.u32(data + 0x8c).wrapping_add(index * 0x28));
                e.mem.set_f32(
                    outputs.wrapping_add(index * 4),
                    (f64::from(sum) / f64::from(divisor)) as f32,
                );
                index += 1;
                if index as i32 >= i32::from(e.mem.i8(data + 0x20)) {
                    break;
                }
            }
        }
        let block = e.mem.u32(saved_block);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(router, block);
        profile_record(e, PROFILE_END);
        fn_00e39400(e, Ptr::new(result));
    });
}

// Translated from 00e39030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleInstance::stepVehicleUsingWheelCollideOutput` (Xbox PDB name, engine map).
/// Thiscall, two stack words (`RET 8`): `step_info` and `collision_output`. Between profile
/// records (labels `010eb86c` and `010c41f8`) it builds the exe's local frame (`0xc4` bytes):
/// a float array of the wheel count at `+4` (buffer `+0x10`, capacity word `0x80000010`), a
/// scratch block of vectors at `+0x4c`, and a second array at `+0x74` (size the wheel count,
/// buffer `+0x80`, capacity word `0x80000010`); runs `updateComponents` (`step_info`,
/// `collision_output`, the vectors, the two arrays) then `simulateVehicle` (`step_info`, the
/// vectors, the two arrays), and frees both arrays' buffers through the allocator (slot `0x10`)
/// when their capacity words are not negative (the first array is also reset to empty).
pub fn hkp_vehicle_instance_step_vehicle_using_wheel_collide_output(
    e: &mut Engine,
    this: Ptr,
    step_info: u32,
    collision_output: u32,
) {
    let t = this.addr();
    profile_record(e, STEP_OUTPUT_LABEL);
    let data = e.mem.u32(t + 0x1c);
    let wheels = i32::from(e.mem.i8(data + 0x20));
    e.with_stack(0xc4, |e, frame| {
        let f = frame.addr();
        let outputs = f + 4;
        let vectors = f + 0x4c;
        let inputs = f + 0x74;
        e.mem.set_u32(inputs + 4, wheels as u32);
        e.mem.set_u32(inputs + 8, 0x8000_0010);
        e.mem.set_u32(inputs, f + 0x80);
        e.mem.set_u32(outputs + 4, wheels as u32);
        e.mem.set_u32(outputs + 8, 0x8000_0010);
        e.mem.set_u32(outputs, f + 0x10);
        hkp_vehicle_instance_update_components(
            e,
            this,
            step_info,
            collision_output,
            vectors,
            inputs,
            outputs,
        );
        hkp_vehicle_instance_simulate_vehicle(e, this, step_info, vectors, inputs, outputs);
        profile_record(e, PROFILE_END);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(outputs + 8);
        e.mem.set_u32(outputs + 4, 0);
        if (capacity as i32) >= 0 {
            let buffer = e.mem.u32(outputs);
            e.vcall(
                allocator,
                0x10,
                &args![buffer, (capacity & 0x3fff_ffff) * 4],
            );
        }
        e.mem.set_u32(outputs, 0);
        e.mem.set_u32(outputs + 8, 0x8000_0000);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(inputs + 8);
        e.mem.set_u32(inputs + 4, 0);
        if (capacity as i32) >= 0 {
            let buffer = e.mem.u32(inputs);
            e.vcall(
                allocator,
                0x10,
                &args![buffer, (capacity & 0x3fff_ffff) * 4],
            );
        }
    });
}

// Translated from 00e39190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleInstance::stepVehicle` (Xbox PDB name, engine map). Thiscall, one stack word
/// (`RET 4`): `step_info` (its float at `+8` is `dt`). Runs `updateBeforeCollisionDetection`,
/// takes a block of `wheels * 0x60` bytes (rounded up to 0x80) from the memory router's stack
/// allocator, has the wheel-collide object (`this + 0x3c`, slot `0x10`) fill it from (`dt`,
/// `this`, the block), runs `stepVehicleUsingWheelCollideOutput` on (`step_info`, the block),
/// restores the stack top, and frees the block through the allocator when its flag word
/// (`wheels | 0x80000000`) is not negative, which it always is (kept as in the exe).
pub fn hkp_vehicle_instance_step_vehicle(e: &mut Engine, this: Ptr, step_info: u32) {
    let t = this.addr();
    hkp_vehicle_instance_update_before_collision_detection(e, this);
    let data = e.mem.u32(t + 0x1c);
    let wheels = i32::from(e.mem.i8(data + 0x20)) as u32;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let top = e.mem.u32(router);
    let dt = x87_quiet(e.mem.f32(step_info + 8));
    let size = (wheels.wrapping_mul(3) << 5).wrapping_add(0x7f) & 0xffff_ff80;
    e.mem.set_u32(router, size.wrapping_add(top));
    let wheel_collide = e.mem.u32(t + 0x3c);
    let flags = wheels | 0x8000_0000;
    e.vcall(wheel_collide, 0x10, &args![dt, t, top]);
    hkp_vehicle_instance_step_vehicle_using_wheel_collide_output(e, this, step_info, top);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, top);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    if (flags as i32) >= 0 {
        let bytes = (flags & 0x3fff_ffff).wrapping_mul(3) << 5;
        e.vcall(allocator, 0x10, &args![top, bytes]);
    }
}

// Translated from 00e392f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleInstance::applyAction` (Xbox PDB name, engine map): a jump to `stepVehicle`
/// (`00e39190`) with the same `this` and stack word (`RET 4`).
pub fn hkp_vehicle_instance_apply_action(e: &mut Engine, this: Ptr, step_info: u32) {
    e.call(0x00e3_9190, &args![this, step_info]);
}

// Translated from 00e39480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy of a vehicle instance's state from `source` (same layout; class role unconfirmed, it
/// is called on a fresh instance by `clone`). Thiscall, one stack word (`RET 4`); returns
/// `this`. Copies the words `+4`..`+0x10` (two 16-bit halves at `+4`), the string at `+0x14`
/// (`00cb0c30` assign) and the words `+0x18`..`+0x44`. The wheel record array (`+0x48`) is
/// reserved for the source count, its first `min(own size, source count)` records are assigned
/// (`00e39300`) and the others are raw-copied (`0xe0` bytes each, skipped for a null
/// destination); then the count, the 18 words `+0x54`..`+0x98` (raw) and the word `+0x9c`. The
/// byte array (`+0xa0`) and the float array (`+0xc0`) are replaced by an exactly sized buffer
/// from the allocator (slot `0xc` takes the address of the wanted size and updates it; slot
/// `0x10` frees the old buffer) when the capacity is too small; their contents are copied
/// (floats through `FLD`/`FSTP`) with the size words, as are the float `+0xac`, byte `+0xb0`,
/// floats `+0xb4`..`+0xbc`, bytes `+0xcc`..`+0xce` and float `+0xd0`.
pub fn fn_00e39480(e: &mut Engine, this: Ptr, source: u32) -> Ptr {
    let t = this.addr();
    let s = source;
    let half = e.mem.u16(s + 4);
    e.mem.set_u16(t + 4, half);
    let half = e.mem.u16(s + 6);
    e.mem.set_u16(t + 6, half);
    for offset in [8, 0xc, 0x10] {
        let word = e.mem.u32(s + offset);
        e.mem.set_u32(t + offset, word);
    }
    e.call(STRING_PTR_ASSIGN, &args![t + 0x14, s + 0x14]);
    for offset in (0x18..=0x44).step_by(4) {
        let word = e.mem.u32(s + offset);
        e.mem.set_u32(t + offset, word);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let own_size = e.mem.i32(t + 0x4c);
    let wanted = e.mem.i32(s + 0x4c);
    let shared = wanted.min(own_size);
    let capacity = (e.mem.u32(t + 0x50) & 0x3fff_ffff) as i32;
    if capacity < wanted {
        let mut new_capacity = capacity.wrapping_add(capacity);
        if wanted >= new_capacity {
            new_capacity = wanted;
        }
        e.call(
            ARRAY_RESERVE,
            &args![allocator, t + 0x48, new_capacity as u32, 0xe0u32],
        );
    }
    let destination = e.mem.u32(t + 0x48);
    let origin = e.mem.u32(s + 0x48);
    for index in 0..shared.max(0) as u32 {
        let record = index.wrapping_mul(0xe0);
        e.call(
            RECORD_ASSIGN,
            &args![
                destination.wrapping_add(record),
                origin.wrapping_add(record)
            ],
        );
    }
    let rest = shared.max(0) as u32;
    let destination = e.mem.u32(t + 0x48).wrapping_add(rest.wrapping_mul(0xe0));
    let origin = e.mem.u32(s + 0x48).wrapping_add(rest.wrapping_mul(0xe0));
    for index in 0..wanted.wrapping_sub(shared).max(0) as u32 {
        let record = index.wrapping_mul(0xe0);
        if destination.wrapping_add(record) != 0 {
            for word in 0..0x38 {
                let value = e.mem.u32(origin.wrapping_add(record) + word * 4);
                e.mem
                    .set_u32(destination.wrapping_add(record) + word * 4, value);
            }
        }
    }
    e.mem.set_u32(t + 0x4c, wanted as u32);
    for word in 0..0x12 {
        let value = e.mem.u32(s + 0x54 + word * 4);
        e.mem.set_u32(t + 0x54 + word * 4, value);
    }
    let word = e.mem.u32(s + 0x9c);
    e.mem.set_u32(t + 0x9c, word);
    let count = e.mem.i32(s + 0xa4);
    reallocate_exact(e, t + 0xa0, count, 1);
    e.mem.set_u32(t + 0xa4, count as u32);
    let destination = e.mem.u32(t + 0xa0);
    let origin = e.mem.u32(s + 0xa0);
    for index in 0..count.max(0) as u32 {
        let byte = e.mem.u8(origin.wrapping_add(index));
        e.mem.set_u8(destination.wrapping_add(index), byte);
    }
    let bits = x87_quiet_bits(e.mem.u32(s + 0xac));
    e.mem.set_u32(t + 0xac, bits);
    let byte = e.mem.u8(s + 0xb0);
    e.mem.set_u8(t + 0xb0, byte);
    for offset in [0xb4, 0xb8, 0xbc] {
        let bits = x87_quiet_bits(e.mem.u32(s + offset));
        e.mem.set_u32(t + offset, bits);
    }
    let count = e.mem.i32(s + 0xc4);
    reallocate_exact(e, t + 0xc0, count, 4);
    let destination = e.mem.u32(t + 0xc0);
    e.mem.set_u32(t + 0xc4, count as u32);
    let origin = e.mem.u32(s + 0xc0);
    for index in 0..count.max(0) as u32 {
        let bits = x87_quiet_bits(e.mem.u32(origin.wrapping_add(index * 4)));
        e.mem.set_u32(destination.wrapping_add(index * 4), bits);
    }
    for offset in [0xcc, 0xcd, 0xce] {
        let byte = e.mem.u8(s + offset);
        e.mem.set_u8(t + offset, byte);
    }
    let bits = x87_quiet_bits(e.mem.u32(s + 0xd0));
    e.mem.set_u32(t + 0xd0, bits);
    this
}

// Translated from 00e39210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleInstance::clone` (Xbox PDB name, engine map). Thiscall, two stack words
/// (`RET 8`): `holder` (the body is the word `holder` points through twice) and `argument`
/// (passed on to the wheel-collide clone). Takes `0xd4` bytes from the allocator (slot `4`),
/// stores the size at `+4`, constructs the instance on it (`00e38b20`, with the body), copies
/// this instance's state in (`00e39480`), clears `+8` and `+0xc`, sets `+0x18` to the body,
/// clones the wheel-collide object (slot `0x1c` of the object at `this + 0x3c`, with the body
/// and `argument`; the clone's byte `+8` becomes `1`) into `+0x3c`, calls slot `0xc` of the object
/// at `this + 0x9c` and stores the result in the new `+0x9c`. Then adds a reference
/// (`00c90510`) to the new instance's `+0x1c`, `+0x20`, `+0x24`, `+0x28`, `+0x2c`, `+0x30`,
/// `+0x34`, `+0x38`, `+0x44` and, when this instance's `+0x40` is not null, to `+0x40`. Returns
/// the new instance.
pub fn hkp_vehicle_instance_clone(e: &mut Engine, this: Ptr, holder: u32, argument: u32) -> Ptr {
    let t = this.addr();
    let body = e.mem.u32(e.mem.u32(holder));
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0xd4u32]).u32();
    e.mem.set_u16(block + 4, 0xd4);
    let c = e.call(0x00e3_8b20, &args![block, body]).u32();
    e.call(0x00e3_9480, &args![c, t]);
    e.mem.set_u32(c + 8, 0);
    e.mem.set_u32(c + 0xc, 0);
    e.mem.set_u32(c + 0x18, body);
    let wheel_collide = e.mem.u32(t + 0x3c);
    let cloned = e.vcall(wheel_collide, 0x1c, &args![body, argument]).u32();
    e.mem.set_u32(c + 0x3c, cloned);
    e.mem.set_u8(cloned + 8, 1);
    let component = e.mem.u32(t + 0x9c);
    let result = e.vcall(component, 0xc, &args![]).u32();
    e.mem.set_u32(c + 0x9c, result);
    for offset in [0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x44] {
        let object = e.mem.u32(c + offset);
        e.call(ADD_REFERENCE, &args![object]);
    }
    if e.mem.u32(t + 0x40) != 0 {
        let object = e.mem.u32(c + 0x40);
        e.call(ADD_REFERENCE, &args![object]);
    }
    Ptr::new(c)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e3_6f50, hkp_vehicle_instance_get_phantoms(Ptr, u32) -> u32),
        entry!(0x00e3_6f60, fn_00e36f60(Ptr, f32, u32)),
        entry!(0x00e3_6fa0, fn_00e36fa0(Ptr, f32, u32)),
        entry!(0x00e3_7020, fn_00e37020(Ptr, f32, u32, u32)),
        entry!(0x00e3_7080, fn_00e37080(Ptr, f32, u32)),
        entry!(0x00e3_70a0, hkp_vehicle_instance_calc_rpm(Ptr) -> f32),
        entry!(0x00e3_70b0, fn_00e370b0(Ptr)),
        entry!(
            0x00e3_7160,
            hkp_vehicle_instance_handle_fixed_ground_accum(Ptr, u32, u32)
        ),
        entry!(0x00e3_71a0, fn_00e371a0(Ptr, f32, u32, u32)),
        entry!(
            0x00e3_71d0,
            hkp_vehicle_instance_apply_suspension_forces_at_wheel(Ptr, f32, u32, f32)
        ),
        entry!(
            0x00e3_7350,
            hkp_vehicle_instance_get_axle_params_from_wheel(Ptr, u32, f32, f32, f32, u32)
        ),
        entry!(
            0x00e3_75a0,
            hkp_vehicle_instance_per_wheel_behavior(Ptr, f32, u32, u32, u32, u32)
        ),
        entry!(0x00e3_76e0, fn_00e376e0(Ptr, f32, u32)),
        entry!(
            0x00e3_7740,
            hkp_vehicle_instance_prepare_friction_solver_params(Ptr, u32, u32)
        ),
        entry!(0x00e3_7820, hkp_vehicle_instance_calc_kmph(Ptr) -> f32),
        entry!(0x00e3_7880, hkp_vehicle_instance_calc_mph(Ptr) -> f32),
        entry!(0x00e3_78e0, hkp_vehicle_instance_add_to_world(Ptr, u32)),
        entry!(0x00e3_7910, hkp_vehicle_instance_remove_from_world(Ptr)),
        entry!(
            0x00e3_7940,
            hkp_vehicle_instance_update_before_collision_detection(Ptr)
        ),
        entry!(
            0x00e3_7aa0,
            hkp_vehicle_instance_update_wheels(Ptr, u32, u32)
        ),
        entry!(
            0x00e3_7d60,
            hkp_vehicle_instance_get_axle_params(Ptr, u32, u32, u32, u32, u32)
        ),
        entry!(0x00e3_8080, fn_00e38080(Ptr, u32, u32)),
        entry!(0x00e3_80c0, fn_00e380c0(Ptr, u32)),
        entry!(0x00e3_8140, fn_00e38140(Ptr, f32, u32, u32)),
        entry!(
            0x00e3_8310,
            hkp_vehicle_instance_apply_results_to_wheel_info(Ptr, f32, u32, u32)
        ),
        entry!(
            0x00e3_84e0,
            hkp_vehicle_instance_calc_current_position_and_rotation(Ptr, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00e3_85e0,
            hkp_vehicle_instance_simulate_vehicle(Ptr, u32, u32, u32, u32)
        ),
        entry!(0x00e3_87e0, hkp_vehicle_instance_init(Ptr)),
        entry!(0x00e3_8a30, fn_00e38a30(Ptr, f32, u32, u32)),
        entry!(0x00e3_8b20, hkp_vehicle_instance_constructor(Ptr, u32) -> Ptr),
        entry!(0x00e3_8bf0, hkp_vehicle_instance_destructor(Ptr)),
        entry!(0x00e3_8d50, fn_00e38d50(Ptr, f32, u32)),
        entry!(
            0x00e3_8e60,
            hkp_vehicle_instance_update_components(Ptr, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00e3_9030,
            hkp_vehicle_instance_step_vehicle_using_wheel_collide_output(Ptr, u32, u32)
        ),
        entry!(0x00e3_9190, hkp_vehicle_instance_step_vehicle(Ptr, u32)),
        entry!(0x00e3_92f0, hkp_vehicle_instance_apply_action(Ptr, u32)),
        entry!(0x00e3_9210, hkp_vehicle_instance_clone(Ptr, u32, u32) -> Ptr),
        entry!(0x00e3_9400, fn_00e39400(Ptr)),
        entry!(0x00e3_9480, fn_00e39480(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpsphereshape::{
        store_bits, HALF_F32, PROFILE_SLOT, THREE_F32, TLS_GET_VALUE,
    };
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Base of the doubles that stand in for vtable slots (`SLOT_BASE + slot offset`).
    const SLOT_BASE: u32 = 0x0900_0000;
    /// The allocator doubles: take a block (slot 4), take an exact-size block (slot `0xc`)
    /// and free (slot `0x10`; logged).
    const ALLOCATOR_TAKE: u32 = 0x0910_0004;
    const ALLOCATOR_EXACT: u32 = 0x0910_000c;
    const ALLOCATOR_FREE: u32 = 0x0910_0010;

    /// An engine with the exe's data range mapped and a profiling stream with no room
    /// (so the profile records are skipped).
    fn setup() -> (Engine, Log) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let stream = e.mem.alloc(0x10);
        e.set_global(PROFILE_SLOT, 5u32);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        (e, Rc::new(RefCell::new(Vec::new())))
    }

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

    /// A logging double for vtable slot `slot` that returns `eax`.
    fn slot_double(e: &mut Engine, log: &Log, slot: u32, eax: u32) {
        let sink = Rc::clone(log);
        let address = SLOT_BASE + slot;
        e.register_double(address, move |_, a| {
            sink.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax,
                ..Ret::default()
            }
        });
    }

    /// A double at `address` that logs its arguments with the one at `index` (the address of a
    /// vector) replaced by the vector's four lanes (as bits).
    fn vector_double(e: &mut Engine, log: &Log, address: u32, index: usize) {
        let sink = Rc::clone(log);
        e.register_double(address, move |e, a| {
            let mut logged = a[..index].to_vec();
            logged.extend(load_bits(e, a[index]));
            logged.extend_from_slice(&a[index + 1..]);
            sink.borrow_mut().push((address, logged));
            Ret::default()
        });
    }

    /// Writes a vtable whose slots (given by offset) are logging doubles into the word at `at`.
    fn install_vtable(e: &mut Engine, log: &Log, at: u32, slots: &[u32]) {
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(at, vtable);
        for &slot in slots {
            e.mem.set_u32(vtable + slot, SLOT_BASE + slot);
            slot_double(e, log, slot, 0);
        }
    }

    fn object_with_slots(e: &mut Engine, log: &Log, slots: &[u32]) -> u32 {
        let object = e.mem.alloc(0x40);
        install_vtable(e, log, object, slots);
        object
    }

    /// Doubles for the memory router (the returned `(router, allocator)`; the stack-allocator
    /// top at `router + 0` is `0`) and its allocator.
    fn memory(e: &mut Engine, log: &Log) -> (u32, u32) {
        let router = e.mem.alloc(0x20);
        let allocator = e.mem.alloc(0x10);
        e.put_vtable(
            0x0920_0000,
            &[0, ALLOCATOR_TAKE, 0, ALLOCATOR_EXACT, ALLOCATOR_FREE],
        );
        e.mem.set_u32(allocator, 0x0920_0000);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(ALLOCATOR_TAKE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register_double(ALLOCATOR_EXACT, |e, a| {
            let size = e.mem.u32(a[1]);
            Ret {
                eax: e.mem.alloc(size),
                ..Ret::default()
            }
        });
        log_callees(e, log, &[ALLOCATOR_FREE]);
        (router, allocator)
    }

    /// `hkArrayUtil::_reserve` double: logs, gives the array a zeroed buffer of the wanted
    /// capacity and records the capacity.
    fn array_reserve(e: &mut Engine, log: &Log) {
        let sink = Rc::clone(log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            sink.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            let buffer = e.mem.alloc(a[2].max(1) * a[3]);
            e.mem.set_u32(a[1], buffer);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
    }

    /// A vehicle: `(this, data)` with the body at `this + 0x18` and the wheel count in the
    /// data block.
    fn vehicle(e: &mut Engine, wheels: u8) -> (u32, u32) {
        let this = e.mem.alloc(0x100);
        let data = e.mem.alloc(0x1a0);
        let body = e.mem.alloc(0x200);
        e.mem.set_u32(this + 0x18, body);
        e.mem.set_u32(this + 0x1c, data);
        e.mem.set_u8(data + 0x20, wheels);
        (this, data)
    }

    /// A one-wheel vehicle with its tables: `(this, data, body, info, entry)` where `info` is
    /// the wheel record and `entry` the parameter entry; the chassis motion has slot `0x50`
    /// and the axle table makes the reciprocal `0.5`.
    fn wheel_fixture(e: &mut Engine, log: &Log) -> (u32, u32, u32, u32, u32) {
        let (this, data) = vehicle(e, 1);
        let body = e.mem.u32(this + 0x18);
        let info = e.mem.alloc(0xe0);
        let entry = e.mem.alloc(0x28);
        let table = e.mem.alloc(8);
        let flags = e.mem.alloc(8);
        e.mem.set_u32(this + 0x48, info);
        e.mem.set_u32(this + 0xa0, flags);
        e.mem.set_u32(data + 0x8c, entry);
        e.mem.set_u32(data + 0x98, table);
        e.mem.set_u8(table, 2);
        e.set_global(THREE_F32, 3.0f32);
        e.set_global(HALF_F32, 0.5f32);
        e.set_global(0x0104_da4c, 1.192_092_9e-7f32);
        install_vtable(e, log, body + 0xe0, &[0x50]);
        log_callees(e, log, &[ENTITY_ACTIVATE]);
        (this, data, body, info, entry)
    }

    fn close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < 1e-5,
            "{actual} is not close to {expected}"
        );
    }

    #[test]
    fn get_phantoms_jumps_to_slot_0x14_of_the_wheel_collide_object() {
        let (mut e, log) = setup();
        let (this, _) = vehicle(&mut e, 0);
        let wheel_collide = object_with_slots(&mut e, &log, &[0x14]);
        slot_double(&mut e, &log, 0x14, 0x1234);
        e.mem.set_u32(this + 0x3c, wheel_collide);
        let got = e.call(0x00e3_6f50, &args![this, 0x77u32]).u32();
        assert_eq!(got, 0x1234);
        assert_eq!(slot_calls(&log, 0x14), vec![vec![wheel_collide, 0x77]]);
    }

    #[test]
    fn fn_00e36f60_passes_the_flag_byte_through_the_block() {
        let (mut e, log) = setup();
        let (this, _) = vehicle(&mut e, 0);
        let component = object_with_slots(&mut e, &log, &[0xc]);
        e.mem.set_u32(this + 0x20, component);
        e.mem.set_u8(this + 0xb0, 1);
        e.mem.set_u32(this + 0x9c, 0x55);
        let block = e.mem.alloc(0x40);
        let sink = Rc::clone(&log);
        e.register_double(SLOT_BASE + 0xc, move |e, a| {
            sink.borrow_mut().push((SLOT_BASE + 0xc, a.to_vec()));
            let seen = e.mem.u8(a[4] + 0xd);
            e.mem.set_u8(a[4] + 0xd, seen + 8);
            Ret::default()
        });
        e.call(0x00e3_6f60, &args![this, 2.5f32, block]);
        let expected = vec![component, 2.5f32.to_bits(), this, 0x55, block];
        assert_eq!(slot_calls(&log, 0xc), vec![expected]);
        assert_eq!(e.mem.u8(this + 0xb0), 9);
    }

    #[test]
    fn fn_00e36fa0_copies_the_state_through_the_block() {
        let (mut e, log) = setup();
        let (this, data) = vehicle(&mut e, 3);
        let component = object_with_slots(&mut e, &log, &[0xc]);
        e.mem.set_u32(this + 0x2c, component);
        for (k, byte) in [4u8, 5, 6].into_iter().enumerate() {
            e.mem.set_u8(this + 0xcc + k as u32, byte);
        }
        e.mem.set_f32(this + 0xd0, 1.5);
        let block = e.mem.alloc(0x40);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(SLOT_BASE + 0xc, move |e, a| {
            let block = a[3];
            let bytes: Vec<u8> = (0xc..0x10).map(|k| e.mem.u8(block + k)).collect();
            sink.borrow_mut().push((bytes, e.mem.f32(block + 0x10)));
            for (k, byte) in [7u8, 8, 9].into_iter().enumerate() {
                e.mem.set_u8(block + 0xd + k as u32, byte);
            }
            e.mem.set_f32(block + 0x10, 2.5);
            Ret::default()
        });
        e.call(0x00e3_6fa0, &args![this, 0.5f32, block]);
        assert_eq!(*seen.borrow(), vec![(vec![3, 4, 5, 6], 1.5)]);
        let copied: Vec<u8> = (0..3).map(|k| e.mem.u8(this + 0xcc + k)).collect();
        assert_eq!(copied, vec![7, 8, 9]);
        assert_eq!(e.mem.f32(this + 0xd0), 2.5);
        assert_eq!(e.mem.u8(data + 0x20), 3);
    }

    #[test]
    fn fn_00e37020_round_trips_the_two_floats() {
        let (mut e, log) = setup();
        let (this, _) = vehicle(&mut e, 0);
        let component = object_with_slots(&mut e, &log, &[0xc]);
        e.mem.set_u32(this + 0x28, component);
        e.mem.set_f32(this + 0xb4, 1.0);
        e.mem.set_f32(this + 0xb8, 2.0);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(SLOT_BASE + 0xc, move |e, a| {
            sink.borrow_mut()
                .push((a.to_vec(), e.mem.f32(a[5]), e.mem.f32(a[5] + 4)));
            e.mem.set_f32(a[5], 3.0);
            e.mem.set_f32(a[5] + 4, 4.0);
            Ret::default()
        });
        e.call(0x00e3_7020, &args![this, 0.25f32, 0x11u32, 0x22u32]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(
            seen[0].0[..5],
            [component, 0.25f32.to_bits(), this, 0x11, 0x22]
        );
        assert_eq!((seen[0].1, seen[0].2), (1.0, 2.0));
        assert_eq!(e.mem.f32(this + 0xb4), 3.0);
        assert_eq!(e.mem.f32(this + 0xb8), 4.0);
    }

    #[test]
    fn fn_00e37080_calls_the_component_at_0x38() {
        let (mut e, log) = setup();
        let (this, _) = vehicle(&mut e, 0);
        let component = object_with_slots(&mut e, &log, &[0xc]);
        e.mem.set_u32(this + 0x38, component);
        e.call(0x00e3_7080, &args![this, 1.5f32, 0x33u32]);
        let expected = vec![component, 1.5f32.to_bits(), this, 0x33];
        assert_eq!(slot_calls(&log, 0xc), vec![expected]);
    }

    #[test]
    fn calc_rpm_returns_the_float_at_0xb8() {
        let (mut e, _) = setup();
        let (this, _) = vehicle(&mut e, 0);
        e.mem.set_f32(this + 0xb8, 1234.5);
        assert_eq!(e.call(0x00e3_70a0, &args![this]).f32(), 1234.5);
    }

    #[test]
    fn fn_00e370b0_initialises_a_wheel_record() {
        let (mut e, _) = setup();
        store_vec(&mut e, 0x0126_7e30, [1.0, 2.0, 3.0, 4.0]);
        store_vec(&mut e, 0x010c_7190, [5.0, 6.0, 7.0, 8.0]);
        store_vec(&mut e, 0x010c_71b0, [9.0, 10.0, 11.0, 12.0]);
        e.set_global(0x0101_2068, 1.0f32);
        e.set_global(0x0101_1d78, 0.25f32);
        let record = e.mem.alloc(0xe0);
        e.mem.write(record, &[0xff; 0xe0]);
        e.call(0x00e3_70b0, &args![record]);
        assert_eq!(load_vec(&e, record), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vec(&e, record + 0x10), [5.0, 6.0, 7.0, 1.0]);
        assert_eq!(e.mem.f32(record + 0x20), 0.0);
        assert_eq!(e.mem.u32(record + 0x24), 0);
        assert_eq!(e.mem.u32(record + 0x28), 0xffff_ffff);
        for offset in [0x50, 0x60, 0x80, 0xa0] {
            assert_eq!(load_vec(&e, record + offset), [0.0; 4]);
        }
        assert_eq!(e.mem.f32(record + 0x70), 0.0);
        assert_eq!(load_vec(&e, record + 0x90), [1.0, 0.25, 0.25, 0.25]);
        assert_eq!(load_vec(&e, record + 0xb0), [9.0, 10.0, 11.0, 12.0]);
        for offset in [0xc0, 0xc4, 0xc8, 0xcc, 0xd0, 0xd4] {
            assert_eq!(e.mem.f32(record + offset), 0.0);
        }
        assert_eq!(e.mem.u32(record + 0xd8), 0xffff_ffff);
    }

    #[test]
    fn handle_fixed_ground_accum_clears_the_accumulator() {
        let (mut e, _) = setup();
        let accum = e.mem.alloc(0x80);
        e.mem.write(accum, &[0xff; 0x80]);
        e.call(0x00e3_7160, &args![0u32, 0u32, accum]);
        assert_eq!(e.mem.u8(accum), 1);
        assert_eq!(e.mem.u8(accum + 1), 0xff);
        assert_eq!(e.mem.f32(accum + 8), 0.0);
        assert_eq!(e.mem.u32(accum + 0xc), 0xffff_ffff);
        for offset in (0x10..0x80).step_by(0x10) {
            assert_eq!(load_vec(&e, accum + offset), [0.0; 4]);
        }
    }

    #[test]
    fn fn_00e371a0_passes_the_referenced_word_on() {
        let (mut e, log) = setup();
        let (this, _) = vehicle(&mut e, 0);
        let component = object_with_slots(&mut e, &log, &[0xc]);
        e.mem.set_u32(this + 0x34, component);
        let reference = e.mem.alloc(8);
        e.mem.set_u32(reference, 0x99);
        e.call(0x00e3_71a0, &args![this, 0.75f32, 0x44u32, reference]);
        let expected = vec![component, 0.75f32.to_bits(), this, 0x44, 0x99];
        assert_eq!(slot_calls(&log, 0xc), vec![expected]);
    }

    #[test]
    fn apply_suspension_forces_applies_the_force_to_the_chassis() {
        let (mut e, log) = setup();
        let (this, _, body, info, _) = wheel_fixture(&mut e, &log);
        vector_double(&mut e, &log, SLOT_BASE + 0x50, 1);
        store_vec(&mut e, info + 0x10, [0.0, 0.0, 1.0, 0.0]);
        e.call(0x00e3_71d0, &args![this, 2.0f32, 0u32, 3.0f32]);
        assert_eq!(calls_to(&log, ENTITY_ACTIVATE), vec![vec![body]]);
        let lanes = [0, 0, 6.0f32.to_bits(), 0];
        let mut expected = vec![body + 0xe0];
        expected.extend(lanes);
        expected.push(info + 0x50);
        assert_eq!(slot_calls(&log, 0x50), vec![expected]);
    }

    #[test]
    fn apply_suspension_forces_limits_the_ground_reaction() {
        let (mut e, log) = setup();
        let (this, _, _, info, entry) = wheel_fixture(&mut e, &log);
        let ground = e.mem.alloc(0x100);
        install_vtable(&mut e, &log, ground + 0xe0, &[0x50]);
        vector_double(&mut e, &log, SLOT_BASE + 0x50, 1);
        e.register(MOTION_GET_MASS, |_, _| Ret {
            st0: 2.0,
            ..Ret::default()
        });
        e.register(X87_SQRT, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).sqrt(),
            ..Ret::default()
        });
        store_vec(&mut e, info + 0x10, [0.0, 0.0, 1.0, 0.0]);
        e.mem.set_u32(info + 0x24, ground);
        e.mem.set_f32(entry + 0x1c, 1.0);
        e.mem.set_f32(entry + 0x20, 0.5);
        e.call(0x00e3_71d0, &args![this, 2.0f32, 0u32, -3.0f32]);
        assert!(calls_to(&log, ENTITY_ACTIVATE).is_empty());
        let mut expected = vec![ground + 0xe0];
        expected.extend([0, 0, 2.0f32.to_bits(), 0]);
        expected.push(info + 0x50);
        assert_eq!(slot_calls(&log, 0x50), vec![expected]);
    }

    #[test]
    fn get_axle_params_from_wheel_adds_the_wheel_to_the_axle() {
        let (mut e, log) = setup();
        let (this, _, _, info, entry) = wheel_fixture(&mut e, &log);
        store_vec(&mut e, info, [1.0, 2.0, 3.0, 4.0]);
        store_vec(&mut e, info + 0x10, [0.0, 1.0, 0.0, 0.0]);
        e.mem.set_f32(info + 0x20, 4.0);
        store_vec(&mut e, info + 0xa0, [1.0, 0.0, 0.0, 0.0]);
        for (offset, value) in [(0xc, 3.0f32), (0x10, 5.0), (0x14, 7.0), (0x18, 11.0)] {
            e.mem.set_f32(entry + offset, value);
        }
        let flags = e.mem.u32(this + 0xa0);
        e.mem.set_u8(flags, 1);
        let output = e.mem.alloc(0x60);
        e.call(
            0x00e3_7350,
            &args![this, 0u32, 0.125f32, 0.25f32, 2.0f32, output],
        );
        assert_eq!(load_vec(&e, output), [0.5, 1.0, 1.5, 2.0]);
        let torque = load_vec(&e, output + 0x10);
        close(torque[0], 1.0);
        assert_eq!(&torque[1..], &[0.0, 0.0, 0.0]);
        let normal = load_vec(&e, output + 0x20);
        close(normal[2], -1.0);
        assert_eq!((normal[0], normal[1], normal[3]), (0.0, 0.0, 0.0));
        assert_eq!(e.mem.f32(output + 0x38), 6.0);
        assert_eq!(e.mem.f32(output + 0x3c), 10.0);
        assert_eq!(e.mem.f32(output + 0x40), 14.0);
        assert_eq!(e.mem.f32(output + 0x44), 0.25);
        assert_eq!(e.mem.f32(output + 0x48), 0.125);
        assert_eq!(e.mem.f32(output + 0x4c), 11.0);
        assert_eq!(e.mem.u8(output + 0x50), 1);
    }

    /// A double at `address` that logs its arguments and writes `value` to the vector its first
    /// argument points at.
    fn vector_writer(e: &mut Engine, log: &Log, address: u32, value: Vec4) {
        let sink = Rc::clone(log);
        e.register_double(address, move |e, a| {
            sink.borrow_mut().push((address, a.to_vec()));
            store_vec(e, a[0], value);
            Ret::default()
        });
    }

    fn vec_from_bits(bits: &[u32]) -> Vec4 {
        [0, 1, 2, 3].map(|lane| f32::from_bits(bits[lane]))
    }

    #[test]
    fn per_wheel_behavior_applies_forces_and_collects_the_axle() {
        let (mut e, log) = setup();
        let (this, _, body, info, entry) = wheel_fixture(&mut e, &log);
        let ground = e.mem.alloc(0x1c0);
        install_vtable(&mut e, &log, ground + 0xe0, &[0x50]);
        vector_double(&mut e, &log, SLOT_BASE + 0x50, 1);
        e.register(MOTION_GET_MASS, |_, _| Ret {
            st0: 1.0,
            ..Ret::default()
        });
        store_vec(&mut e, body + 0x1b0, [3.0, 4.0, 0.0, 0.0]);
        store_vec(&mut e, info, [1.0, 2.0, 3.0, 4.0]);
        store_vec(&mut e, info + 0x10, [0.0, 1.0, 0.0, 0.0]);
        store_vec(&mut e, info + 0xa0, [1.0, 0.0, 0.0, 0.0]);
        e.mem.set_f32(info + 0x20, 4.0);
        e.mem.set_u32(info + 0x24, ground);
        e.mem.set_f32(entry + 0x18, 2.0);
        let forces = e.mem.alloc(8);
        e.mem.set_f32(forces, 0.5);
        let forces_ref = e.mem.alloc(8);
        e.mem.set_u32(forces_ref, forces);
        let inputs = e.mem.alloc(8);
        e.mem.set_f32(inputs, 0.25);
        let inputs_ref = e.mem.alloc(8);
        e.mem.set_u32(inputs_ref, inputs);
        let grounds = e.mem.alloc(8);
        let output = e.mem.alloc(0x60);
        e.call(
            0x00e3_75a0,
            &args![this, 1.0f32, forces_ref, inputs_ref, grounds, output],
        );
        let applied = slot_calls(&log, 0x50);
        assert_eq!(applied.len(), 2);
        assert_eq!(applied[0][0], body + 0xe0);
        assert_eq!(applied[0][1..5], [0, 0.5f32.to_bits(), 0, 0]);
        assert_eq!(applied[1][0], ground + 0xe0);
        assert_eq!(applied[1][1..5], [0x8000_0000; 4]);
        assert_eq!(e.mem.u32(grounds), ground);
        assert_eq!(e.mem.f32(output + 0x44), 0.5);
        assert_eq!(e.mem.f32(output + 0x48), 0.25);
        assert_eq!(e.mem.f32(output + 0x4c), 5.0);
        close(e.mem.f32(output + 0x28), -1.0);
    }

    #[test]
    fn fn_00e376e0_adds_the_scaled_vector_unless_the_mass_is_zero() {
        let (mut e, _) = setup();
        let (this, data) = vehicle(&mut e, 0);
        e.mem.set_f32(data + 0x6c, 2.0);
        e.mem.set_f32(this + 0xbc, 3.0);
        store_vec(&mut e, data + 0x30, [1.0, 2.0, 3.0, 4.0]);
        store_vec(&mut e, data + 0x180, [1.0, 1.0, 2.0, 2.0]);
        let output = e.mem.alloc(0x40);
        store_vec(&mut e, output + 0x20, [1.0; 4]);
        e.call(0x00e3_76e0, &args![this, 0.5f32, output]);
        assert_eq!(load_vec(&e, output + 0x20), [4.0, 7.0, 19.0, 25.0]);
        e.mem.set_f32(data + 0x6c, 0.0);
        e.call(0x00e3_76e0, &args![this, 0.5f32, output]);
        assert_eq!(load_vec(&e, output + 0x20), [4.0, 7.0, 19.0, 25.0]);
    }

    #[test]
    fn prepare_friction_solver_params_builds_the_solver_record() {
        let (mut e, log) = setup();
        let (this, data) = vehicle(&mut e, 0);
        let body = e.mem.u32(this + 0x18);
        let sink = Rc::clone(&log);
        e.register_double(BUILD_ACCUMULATORS, move |e, a| {
            let mut logged = a.to_vec();
            logged.push(e.mem.u32(a[1]));
            sink.borrow_mut().push((BUILD_ACCUMULATORS, logged));
            Ret::default()
        });
        log_callees(&mut e, &log, &[BUILD_AXLE_BASIS]);
        vector_double(&mut e, &log, SET_ROTATED_DIR, 2);
        store_vec(&mut e, body + 0x140, [5.0, 6.0, 7.0, 2.0]);
        store_vec(&mut e, body + 0x130, [1.0, 2.0, 3.0, 0.0]);
        store_vec(&mut e, body + 0x180, [1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, data + 0x180, [9.0; 4]);
        let output = e.mem.alloc(0x180);
        e.call(0x00e3_7740, &args![this, 0x1234u32, output]);
        let accumulator = output + 0xd0;
        let built = calls_to(&log, BUILD_ACCUMULATORS);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][0], 0x1234);
        assert_eq!(built[0][2..], [1, 0, accumulator, body + 0xe0]);
        assert_eq!(
            calls_to(&log, BUILD_AXLE_BASIS),
            vec![vec![output + 0x150, accumulator]]
        );
        let mut rotated = vec![output + 0x170, output + 0x120];
        rotated.extend([2.0f32, 0.0, 0.0, 0.0].map(f32::to_bits));
        assert_eq!(calls_to(&log, SET_ROTATED_DIR), vec![rotated]);
        assert_eq!(load_vec(&e, output + 0x100), [9.0; 4]);
        assert_eq!(load_vec(&e, output + 0x160), [8.0, 8.0, 8.0, 4.0]);
    }

    #[test]
    fn calc_kmph_and_calc_mph_convert_the_chassis_speed() {
        let (mut e, _) = setup();
        let (this, _) = vehicle(&mut e, 0);
        let body = e.mem.u32(this + 0x18);
        store_vec(&mut e, body + 0x1b0, [3.0, 4.0, 0.0, 0.0]);
        e.mem.set_f64(0x0101_7b70, 1000.0);
        e.mem.set_f64(0x0101_2640, 3600.0);
        e.mem.set_f64(0x010e_b828, 1609.344);
        let kmph = e.call(0x00e3_7820, &args![this]).f32();
        assert_eq!(kmph, ((5.0f64 / 1000.0) * 3600.0) as f32);
        let mph = e.call(0x00e3_7880, &args![this]).f32();
        assert_eq!(mph, ((5.0f64 / 1609.344) * 3600.0) as f32);
    }

    #[test]
    fn add_to_world_adds_the_body_then_the_wheels() {
        let (mut e, log) = setup();
        let (this, _) = vehicle(&mut e, 0);
        let body = e.mem.u32(this + 0x18);
        let wheel_collide = object_with_slots(&mut e, &log, &[0x20]);
        e.mem.set_u32(this + 0x3c, wheel_collide);
        log_callees(&mut e, &log, &[0x00c9_14d0]);
        e.call(0x00e3_78e0, &args![this, 0x777u32]);
        assert_eq!(calls_to(&log, 0x00c9_14d0), vec![vec![0x777, body, 1]]);
        assert_eq!(slot_calls(&log, 0x20), vec![vec![wheel_collide, 0x777]]);
    }

    #[test]
    fn remove_from_world_removes_the_wheels_then_the_body() {
        let (mut e, log) = setup();
        let (this, _) = vehicle(&mut e, 0);
        let body = e.mem.u32(this + 0x18);
        e.mem.set_u32(body + 8, 0x888);
        let wheel_collide = object_with_slots(&mut e, &log, &[0x24]);
        e.mem.set_u32(this + 0x3c, wheel_collide);
        log_callees(&mut e, &log, &[0x00c9_1620]);
        e.call(0x00e3_7910, &args![this]);
        let order: Vec<u32> = log.borrow().iter().map(|(a, _)| *a).collect();
        assert_eq!(order, vec![SLOT_BASE + 0x24, 0x00c9_1620]);
        let removed = calls_to(&log, 0x00c9_1620);
        assert_eq!((removed[0][0], removed[0][2]), (0x888, body));
    }

    #[test]
    fn update_before_collision_detection_places_the_wheel_rays() {
        let (mut e, log) = setup();
        let (this, _, body, info, entry) = wheel_fixture(&mut e, &log);
        let tracker = e.mem.alloc(0x10);
        let entries = e.mem.alloc(0x30);
        e.mem.set_u32(tracker + 8, entries);
        e.mem.set_u32(this + 0x34, tracker);
        e.mem.set_f32(entries + 0x20, 1.5);
        e.mem.set_f32(entry, 0.5);
        let wheel_collide = object_with_slots(&mut e, &log, &[0x18]);
        e.mem.set_u32(this + 0x3c, wheel_collide);
        vector_writer(&mut e, &log, SET_ROTATED_DIR, [0.0, 1.0, 0.0, 0.0]);
        vector_writer(&mut e, &log, SET_TRANSFORMED_POS, [10.0, 20.0, 30.0, 1.0]);
        e.call(0x00e3_7940, &args![this]);
        assert_eq!(
            calls_to(&log, SET_ROTATED_DIR),
            vec![vec![info + 0x80, body + 0xf0, entries + 0x10]]
        );
        assert_eq!(
            calls_to(&log, SET_TRANSFORMED_POS),
            vec![vec![info + 0x50, body + 0xf0, entries]]
        );
        assert_eq!(load_vec(&e, info + 0x60), [10.0, 22.0, 30.0, 1.0]);
        assert_eq!(slot_calls(&log, 0x18), vec![vec![wheel_collide, this]]);
    }

    #[test]
    fn update_wheels_copies_the_description_and_orients_the_wheel() {
        let (mut e, log) = setup();
        let (this, data, body, info, _) = wheel_fixture(&mut e, &log);
        let tracker = e.mem.alloc(0x10);
        let entries = e.mem.alloc(0x30);
        e.mem.set_u32(tracker + 8, entries);
        e.mem.set_u32(this + 0x34, tracker);
        let scales = e.mem.alloc(8);
        e.mem.set_f32(scales, 2.0);
        e.mem.set_u32(this + 0xc0, scales);
        store_vec(&mut e, entries + 0x10, [0.0, 0.0, 1.0, 0.0]);
        store_vec(&mut e, data + 0x50, [1.0, 0.0, 0.0, 0.0]);
        e.mem.set_f64(0x0101_1588, 0.5);
        store_vec(&mut e, NEGATIVE_HALVES, [-0.5; 4]);
        vector_double(&mut e, &log, SET_ROTATED_DIR, 2);
        let source = e.mem.alloc(0x60);
        store_vec(&mut e, source, [1.0, 2.0, 3.0, 4.0]);
        store_vec(&mut e, source + 0x10, [5.0, 6.0, 7.0, 8.0]);
        e.mem.set_f32(source + 0x20, 1.5);
        e.mem.set_u32(source + 0x24, 0xaaaa);
        e.mem.set_u32(source + 0x28, 0x1111);
        e.mem.set_u32(source + 0x44, 0x2222);
        e.mem.set_f32(source + 0x48, 0.75);
        e.call(0x00e3_7aa0, &args![this, 0u32, source]);
        assert_eq!(load_vec(&e, info), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vec(&e, info + 0x10), [5.0, 6.0, 7.0, 8.0]);
        assert_eq!(e.mem.f32(info + 0x20), 1.5);
        assert_eq!(e.mem.u32(info + 0x24), 0xaaaa);
        assert_eq!(e.mem.u32(info + 0x28), 0x1111);
        assert_eq!(e.mem.u32(info + 0x44), 0x2222);
        assert_eq!(e.mem.f32(info + 0x70), 0.75);
        assert_eq!(load_vec(&e, info + 0x90), [1.0, 0.0, 0.0, 0.0]);
        let direction = load_vec(&e, info + 0xb0);
        close(direction[2], std::f32::consts::FRAC_1_SQRT_2);
        close(direction[3], std::f32::consts::FRAC_1_SQRT_2);
        let rotated = calls_to(&log, SET_ROTATED_DIR);
        assert_eq!(rotated.len(), 1);
        assert_eq!(rotated[0][..2], [info + 0xa0, body + 0xf0]);
        let local = vec_from_bits(&rotated[0][2..]);
        for (lane, expected) in local.into_iter().zip([0.0, 1.0, 0.0, 0.0]) {
            close(lane, expected);
        }
    }

    #[test]
    fn get_axle_params_clears_the_accumulator_of_a_wheel_without_ground() {
        let (mut e, log) = setup();
        let (this, data, _, _, _) = wheel_fixture(&mut e, &log);
        e.mem.set_i32(data + 0x9c, 1);
        let grounds = e.mem.alloc(8);
        let accumulators = e.mem.alloc(0x80);
        e.mem.write(accumulators, &[0xff; 0x80]);
        let bases = e.mem.alloc(0x80);
        let axles = e.mem.alloc(0x60);
        store_vec(&mut e, axles + 0x10, [3.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, axles + 0x20, [0.0, 0.0, 2.0, 0.0]);
        e.call(
            0x00e3_7d60,
            &args![this, 0u32, grounds, accumulators, bases, axles],
        );
        let first = load_vec(&e, axles + 0x10);
        close(first[0], 1.0);
        let second = load_vec(&e, axles + 0x20);
        close(second[2], 1.0);
        assert_eq!(e.mem.u32(axles + 0x30), accumulators);
        assert_eq!(e.mem.u32(axles + 0x34), accumulators);
        assert_eq!(e.mem.u8(accumulators), 1);
        assert_eq!(e.mem.f32(accumulators + 8), 0.0);
        for offset in (0x10..0x80).step_by(0x10) {
            assert_eq!(load_vec(&e, accumulators + offset), [0.0; 4]);
        }
    }

    #[test]
    fn get_axle_params_hands_a_landed_ground_to_the_vehicle() {
        let (mut e, log) = setup();
        let (this, data, _, _, _) = wheel_fixture(&mut e, &log);
        install_vtable(&mut e, &log, this, &[0x3c]);
        e.mem.set_i32(data + 0x9c, 1);
        let ground = e.mem.alloc(0x100);
        e.mem.set_u8(ground + 0xe8, 5);
        let grounds = e.mem.alloc(8);
        e.mem.set_u32(grounds, ground);
        let accumulators = e.mem.alloc(0x80);
        let bases = e.mem.alloc(0x80);
        let axles = e.mem.alloc(0x60);
        e.call(
            0x00e3_7d60,
            &args![this, 0u32, grounds, accumulators, bases, axles],
        );
        assert_eq!(
            slot_calls(&log, 0x3c),
            vec![vec![this, ground, accumulators]]
        );
        assert_eq!(e.mem.u32(axles + 0x34), accumulators);
    }

    #[test]
    fn get_axle_params_builds_the_base_once_for_a_shared_ground() {
        let (mut e, log) = setup();
        let (this, data, _, _, _) = wheel_fixture(&mut e, &log);
        e.mem.set_i32(data + 0x9c, 2);
        e.mem.set_f32(data + 0x88, 2.0);
        let ground = e.mem.alloc(0x1c0);
        let motion = ground + 0xe0;
        store_vec(&mut e, motion + 0x60, [5.0, 6.0, 7.0, 2.0]);
        store_vec(&mut e, motion + 0x50, [1.0, 2.0, 3.0, 0.0]);
        store_vec(&mut e, motion + 0xa0, [1.0, 2.0, 3.0, 0.0]);
        let grounds = e.mem.alloc(8);
        e.mem.set_u32(grounds, ground);
        e.mem.set_u32(grounds + 4, ground);
        let accumulators = e.mem.alloc(0x100);
        store_vec(&mut e, accumulators + 0x50, [1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, accumulators + 0x60, [0.0, 1.0, 0.0, 0.0]);
        store_vec(&mut e, accumulators + 0x70, [0.0, 0.0, 1.0, 0.0]);
        let bases = e.mem.alloc(0x100);
        let axles = e.mem.alloc(0xc0);
        let sink = Rc::clone(&log);
        e.register_double(BUILD_ACCUMULATORS, move |e, a| {
            let mut logged = a.to_vec();
            logged.push(e.mem.u32(a[1]));
            sink.borrow_mut().push((BUILD_ACCUMULATORS, logged));
            Ret::default()
        });
        log_callees(&mut e, &log, &[BUILD_AXLE_BASIS]);
        e.call(
            0x00e3_7d60,
            &args![this, 0x1234u32, grounds, accumulators, bases, axles],
        );
        let built = calls_to(&log, BUILD_ACCUMULATORS);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][2..], [1, 0, accumulators, motion]);
        assert_eq!(
            calls_to(&log, BUILD_AXLE_BASIS),
            vec![vec![bases, accumulators]]
        );
        for axle in [axles, axles + 0x60] {
            assert_eq!(e.mem.u32(axle + 0x30), accumulators);
            assert_eq!(e.mem.u32(axle + 0x34), bases);
        }
        assert_eq!(load_vec(&e, bases + 0x10), [8.0, 8.0, 8.0, 4.0]);
        assert_eq!(load_vec(&e, bases + 0x20), [2.0, 4.0, 6.0, 0.0]);
    }

    #[test]
    fn fn_00e38080_passes_the_step_floats_to_the_solver_call() {
        let (mut e, log) = setup();
        let (this, data) = vehicle(&mut e, 0);
        let step_info = e.mem.alloc(0x10);
        e.mem.set_f32(step_info + 8, 1.5);
        e.mem.set_f32(step_info + 0xc, 2.5);
        let sink = Rc::clone(&log);
        e.register_double(0x00e5_9a40, move |e, a| {
            let mut logged = a.to_vec();
            logged.extend([e.mem.u32(a[0]), e.mem.u32(a[0] + 4)]);
            sink.borrow_mut().push((0x00e5_9a40, logged));
            Ret::default()
        });
        e.call(0x00e3_8080, &args![this, step_info, 0x55u32]);
        let calls = calls_to(&log, 0x00e5_9a40);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][1..4], [data + 0xa4, 0x55, this + 0x54]);
        assert_eq!(calls[0][4..], [1.5f32.to_bits(), 2.5f32.to_bits()]);
    }

    #[test]
    fn fn_00e380c0_combines_the_columns_and_calls_the_motion() {
        let (mut e, log) = setup();
        let (this, _) = vehicle(&mut e, 0);
        let body = e.mem.u32(this + 0x18);
        install_vtable(&mut e, &log, body + 0xe0, &[0x44, 0x40]);
        vector_double(&mut e, &log, SLOT_BASE + 0x44, 1);
        store_vec(&mut e, body + 0xf0, [1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, body + 0x100, [0.0, 1.0, 0.0, 0.0]);
        store_vec(&mut e, body + 0x110, [0.0, 0.0, 1.0, 0.0]);
        let vector = e.mem.alloc(0x40);
        store_vec(&mut e, vector + 0x20, [1.0, 2.0, 3.0, 0.0]);
        e.call(0x00e3_80c0, &args![this, vector]);
        let mut expected = vec![body + 0xe0];
        expected.extend([1.0f32, 2.0, 3.0, 0.0].map(f32::to_bits));
        assert_eq!(slot_calls(&log, 0x44), vec![expected]);
        assert_eq!(
            slot_calls(&log, 0x40),
            vec![vec![body + 0xe0, vector + 0x10]]
        );
    }

    #[test]
    fn fn_00e38140_clamps_the_velocities_to_the_limit() {
        let (mut e, log) = setup();
        let (this, data) = vehicle(&mut e, 0);
        e.mem.set_i32(data + 0x9c, 1);
        e.mem.set_f64(0x0102_0758, 10.0);
        store_bits(&mut e, SIGN_MASK, [0x8000_0000; 4]);
        let ground = e.mem.alloc(0x200);
        install_vtable(&mut e, &log, ground + 0xe0, &[0x44, 0x40]);
        vector_double(&mut e, &log, SLOT_BASE + 0x44, 1);
        vector_double(&mut e, &log, SLOT_BASE + 0x40, 1);
        let bodies = e.mem.alloc(8);
        e.mem.set_u32(bodies, ground);
        let accumulator = e.mem.alloc(0x80);
        store_vec(&mut e, accumulator + 0x50, [1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, accumulator + 0x60, [0.0, 1.0, 0.0, 0.0]);
        store_vec(&mut e, accumulator + 0x70, [0.0, 0.0, 1.0, 0.0]);
        store_vec(&mut e, accumulator + 0x20, [1.0, 2.0, 3.0, 0.0]);
        store_vec(&mut e, accumulator + 0x10, [100.0, -100.0, 1.0, 0.0]);
        let records = e.mem.alloc(0x60);
        e.mem.set_u32(records + 0x30, accumulator);
        e.call(0x00e3_8140, &args![this, 0.5f32, records, bodies]);
        let linear = slot_calls(&log, 0x44);
        assert_eq!(linear.len(), 1);
        assert_eq!(linear[0][0], ground + 0xe0);
        assert_eq!(linear[0][1..4], [1.0f32, 2.0, 3.0].map(f32::to_bits));
        let angular = slot_calls(&log, 0x40);
        assert_eq!(angular.len(), 1);
        assert_eq!(angular[0][1..], [5.0f32, -5.0, 1.0, 0.0].map(f32::to_bits));
    }

    #[test]
    fn apply_results_to_wheel_info_updates_the_wheel_slip() {
        let (mut e, log) = setup();
        let (this, _, body, info, entry) = wheel_fixture(&mut e, &log);
        vector_writer(&mut e, &log, SET_ROTATED_DIR, [1.0, 0.0, 0.0, 0.0]);
        for (k, value) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
            e.mem.set_f32(this + 0x54 + 4 * k as u32, value);
        }
        store_vec(&mut e, body + 0x1b0, [2.0, 0.0, 0.0, 0.0]);
        e.mem.set_f32(entry, 1.5);
        e.mem.set_f32(info + 0xc4, 0.25);
        let axle_grounds = e.mem.alloc(8);
        let axle_records = e.mem.alloc(0x60);
        e.call(
            0x00e3_8310,
            &args![this, 0.5f32, axle_grounds, axle_records],
        );
        assert_eq!(e.mem.f32(info + 0xc8), 0.0);
        assert_eq!(e.mem.f32(info + 0xcc), 4.0);
        assert_eq!(e.mem.f32(info + 0xd4), 2.0);
        assert_eq!(e.mem.f32(info + 0xd0), 1.0);
        assert_eq!(e.mem.f32(info + 0xc0), 2.0);
        assert_eq!(e.mem.f32(info + 0xc4), 1.25);
        let flags = e.mem.u32(this + 0xa0);
        e.mem.set_u8(flags, 1);
        e.mem.set_f32(info + 0xc0, 7.0);
        e.call(
            0x00e3_8310,
            &args![this, 0.5f32, axle_grounds, axle_records],
        );
        assert_eq!(e.mem.f32(info + 0xc0), 0.0);
        assert_eq!(e.mem.f32(info + 0xc4), 1.25);
    }

    #[test]
    fn apply_results_to_wheel_info_subtracts_the_ground_velocity() {
        let (mut e, log) = setup();
        let (this, _, body, info, entry) = wheel_fixture(&mut e, &log);
        vector_writer(&mut e, &log, SET_ROTATED_DIR, [1.0, 0.0, 0.0, 0.0]);
        for (k, value) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
            e.mem.set_f32(this + 0x54 + 4 * k as u32, value);
        }
        store_vec(&mut e, body + 0x1b0, [2.0, 0.0, 0.0, 0.0]);
        e.mem.set_f32(entry, 1.5);
        let ground = e.mem.alloc(0x100);
        e.mem.set_u8(ground + 0xe8, 5);
        e.mem.set_u32(info + 0x24, ground);
        let axle_grounds = e.mem.alloc(8);
        e.mem.set_u32(axle_grounds, ground);
        let accumulator = e.mem.alloc(0x40);
        store_vec(&mut e, accumulator + 0x10, [0.5, 0.0, 0.0, 0.0]);
        let axle_records = e.mem.alloc(0x60);
        e.mem.set_u32(axle_records + 0x30, accumulator);
        e.call(
            0x00e3_8310,
            &args![this, 1.0f32, axle_grounds, axle_records],
        );
        assert_eq!(e.mem.f32(info + 0xc8), 3.0);
        let expected = (2.5f64 / 1.5f64) as f32;
        assert_eq!(e.mem.f32(info + 0xc0), expected);
        assert_eq!(e.mem.f32(info + 0xc4), expected);
    }

    #[test]
    fn calc_current_position_and_rotation_builds_the_wheel_pose() {
        let (mut e, log) = setup();
        let (this, _) = vehicle(&mut e, 1);
        let info = e.mem.alloc(0xe0);
        e.mem.set_u32(this + 0x48, info);
        log_callees(&mut e, &log, &[0x00cb_2450, 0x00c6_6320]);
        e.mem.set_f32(info + 0xc4, 0.5);
        e.mem.set_f32(info + 0x70, 2.0);
        store_vec(&mut e, info + 0x80, [1.0, 1.0, 1.0, 0.0]);
        let frame = e.mem.alloc(0x180);
        store_vec(&mut e, frame + 0xf0, [1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, frame + 0x100, [0.0, 1.0, 0.0, 0.0]);
        store_vec(&mut e, frame + 0x110, [0.0, 0.0, 1.0, 0.0]);
        store_vec(&mut e, frame + 0x120, [10.0, 20.0, 30.0, 1.0]);
        let entries = e.mem.alloc(0x10);
        let array = e.mem.alloc(0x30);
        e.mem.set_u32(entries + 8, array);
        store_vec(&mut e, array, [1.0, 2.0, 3.0, 0.0]);
        let position = e.mem.alloc(0x10);
        let rotation = e.mem.alloc(0x10);
        e.call(
            0x00e3_84e0,
            &args![this, frame, entries, 0u32, position, rotation],
        );
        let axis_angle = calls_to(&log, 0x00cb_2450);
        let multiply = calls_to(&log, 0x00c6_6320);
        assert_eq!(axis_angle.len(), 1);
        assert_eq!(axis_angle[0][1..], [info + 0x90, (-0.5f32).to_bits()]);
        assert_eq!(multiply.len(), 2);
        assert_eq!(multiply[0][1..], [frame + 0x160, info + 0xb0]);
        assert_eq!(
            multiply[1],
            vec![rotation, multiply[0][0], axis_angle[0][0]]
        );
        assert_eq!(load_vec(&e, position), [13.0, 24.0, 35.0, 1.0]);
        e.mem.set_f32(info + 0x70, -1.0);
        e.call(
            0x00e3_84e0,
            &args![this, frame, entries, 0u32, position, rotation],
        );
        assert_eq!(load_vec(&e, position), [11.0, 22.0, 33.0, 1.0]);
    }

    #[test]
    fn simulate_vehicle_runs_the_step_in_order() {
        let (mut e, log) = setup();
        let (this, _) = vehicle(&mut e, 0);
        let body = e.mem.u32(this + 0x18);
        install_vtable(&mut e, &log, body + 0xe0, &[0x5c, 0x60, 0x44, 0x40]);
        let observer = object_with_slots(&mut e, &log, &[0xc]);
        e.mem.set_u32(this + 0x44, observer);
        log_callees(
            &mut e,
            &log,
            &[
                ENTITY_ACTIVATE,
                BUILD_ACCUMULATORS,
                BUILD_AXLE_BASIS,
                SET_ROTATED_DIR,
                0x00e5_9a40,
            ],
        );
        let step_info = e.mem.alloc(0x10);
        e.mem.set_f32(step_info + 8, 0.125);
        let vectors = e.mem.alloc(0x20);
        let forces_ref = e.mem.alloc(8);
        let inputs_ref = e.mem.alloc(8);
        e.call(
            0x00e3_85e0,
            &args![this, step_info, vectors, forces_ref, inputs_ref],
        );
        let order: Vec<u32> = log.borrow().iter().map(|(a, _)| *a).collect();
        assert_eq!(
            order,
            vec![
                ENTITY_ACTIVATE,
                SLOT_BASE + 0x5c,
                ENTITY_ACTIVATE,
                SLOT_BASE + 0x60,
                SLOT_BASE + 0xc,
                BUILD_ACCUMULATORS,
                BUILD_AXLE_BASIS,
                SET_ROTATED_DIR,
                0x00e5_9a40,
                SLOT_BASE + 0x44,
                SLOT_BASE + 0x40,
            ]
        );
        let dt = 0.125f32.to_bits();
        assert_eq!(slot_calls(&log, 0x5c), vec![vec![body + 0xe0, dt, vectors]]);
        assert_eq!(
            slot_calls(&log, 0x60),
            vec![vec![body + 0xe0, dt, vectors + 0x10]]
        );
        assert_eq!(slot_calls(&log, 0xc), vec![vec![observer, dt, this]]);
    }

    #[test]
    fn init_rebuilds_the_inertia_and_sizes_the_wheel_arrays() {
        let (mut e, log) = setup();
        let (this, data) = vehicle(&mut e, 2);
        let body = e.mem.u32(this + 0x18);
        let (_, allocator) = memory(&mut e, &log);
        array_reserve(&mut e, &log);
        install_vtable(&mut e, &log, data, &[0xc]);
        let wheel_collide = object_with_slots(&mut e, &log, &[0xc]);
        e.mem.set_u32(this + 0x3c, wheel_collide);
        e.mem.set_u32(this + 0x34, 0x4000);
        store_bits(&mut e, ABS_MASK, [0x7fff_ffff; 4]);
        store_vec(&mut e, data + 0x30, [-1.0, 2.0, -3.0, 0.0]);
        for offset in [0x74, 0x78, 0x7c] {
            e.mem.set_f32(data + offset, 1.0);
        }
        e.register(MOTION_GET_MASS, |_, _| Ret {
            st0: 2.0,
            ..Ret::default()
        });
        log_callees(&mut e, &log, &[BODY_SET_MASS]);
        let sink = Rc::clone(&log);
        e.register_double(BODY_SET_INERTIA, move |e, a| {
            let mut logged = a.to_vec();
            logged.extend((0..12).map(|k| e.mem.u32(a[1] + 4 * k)));
            sink.borrow_mut().push((BODY_SET_INERTIA, logged));
            Ret::default()
        });
        for offset in [0xac, 0xb4, 0xb8, 0xbc, 0xd0] {
            e.mem.set_f32(this + offset, 5.0);
        }
        e.mem.write(this + 0xcc, &[9, 9, 9]);
        e.call(0x00e3_87e0, &args![this]);
        assert_eq!(
            slot_calls(&log, 0xc),
            vec![vec![data, 0x4008, body], vec![wheel_collide, this]]
        );
        assert_eq!(
            calls_to(&log, BODY_SET_MASS),
            vec![vec![body, 2.0f32.to_bits()]]
        );
        let inertia = calls_to(&log, BODY_SET_INERTIA);
        assert_eq!(inertia.len(), 1);
        let rows = [
            2.0f32, 0.0, 0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 0.0, 6.0, 0.0,
        ];
        assert_eq!(inertia[0][2..], rows.map(f32::to_bits));
        assert_eq!(
            calls_to(&log, ARRAY_RESERVE),
            vec![
                vec![allocator, this + 0x48, 2, 0xe0],
                vec![allocator, this + 0xa0, 2, 1],
                vec![allocator, this + 0xc0, 2, 4],
            ]
        );
        assert_eq!(e.mem.u32(this + 0x4c), 2);
        assert_eq!(e.mem.u32(this + 0xa4), 2);
        assert_eq!(e.mem.u32(this + 0xc4), 2);
        let records = e.mem.u32(this + 0x48);
        assert_eq!(e.mem.u32(records + 0x28), 0xffff_ffff);
        assert_eq!(e.mem.u32(records + 0xe0 + 0x28), 0xffff_ffff);
        for offset in [0xac, 0xb4, 0xb8, 0xbc, 0xd0] {
            assert_eq!(e.mem.f32(this + offset), 0.0);
        }
        assert_eq!(e.mem.bytes(this + 0xcc, 3), vec![0, 0, 0]);
        assert_eq!(e.mem.u8(wheel_collide + 8), 1);
    }

    #[test]
    fn fn_00e38a30_round_trips_the_wheel_flags_and_the_float() {
        let (mut e, log) = setup();
        let (this, _) = vehicle(&mut e, 0);
        let (_, allocator) = memory(&mut e, &log);
        array_reserve(&mut e, &log);
        let component = object_with_slots(&mut e, &log, &[0xc]);
        e.mem.set_u32(this + 0x30, component);
        let flags = e.mem.alloc(8);
        e.mem.write(flags, &[5, 6]);
        e.mem.set_u32(this + 0xa0, flags);
        e.mem.set_u32(this + 0xa4, 2);
        e.mem.set_f32(this + 0xac, 1.5);
        let target = e.mem.alloc(0x40);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(SLOT_BASE + 0xc, move |e, a| {
            let bytes = e.mem.u32(a[4] + 0xc);
            sink.borrow_mut()
                .push((a.to_vec(), e.mem.bytes(bytes, 2), e.mem.f32(a[4] + 0x18)));
            e.mem.write(bytes, &[8, 9]);
            e.mem.set_f32(a[4] + 0x18, 3.5);
            Ret::default()
        });
        e.call(0x00e3_8a30, &args![this, 0.5f32, 0x66u32, target]);
        let expected = vec![component, 0.5f32.to_bits(), this, 0x66, target];
        assert_eq!(*seen.borrow(), vec![(expected, vec![5, 6], 1.5)]);
        assert_eq!(e.mem.bytes(flags, 2), vec![8, 9]);
        assert_eq!(e.mem.f32(this + 0xac), 3.5);
        assert_eq!(e.mem.u32(target + 0x10), 2);
        assert_eq!(e.mem.u32(target + 4), 2);
        assert_eq!(
            calls_to(&log, ARRAY_RESERVE),
            vec![
                vec![allocator, target + 0xc, 2, 1],
                vec![allocator, target, 2, 4]
            ]
        );
    }

    #[test]
    fn constructor_sets_the_vtable_and_empties_the_arrays() {
        let (mut e, log) = setup();
        log_callees(&mut e, &log, &[UNARY_ACTION_CONSTRUCTOR]);
        let this = e.mem.alloc(0x100);
        e.mem.write(this, &[0xff; 0x100]);
        let got = e.call(0x00e3_8b20, &args![this, 0x1234u32]).u32();
        assert_eq!(got, this);
        assert_eq!(
            calls_to(&log, UNARY_ACTION_CONSTRUCTOR),
            vec![vec![this, 0x1234, 0]]
        );
        assert_eq!(e.mem.u32(this), 0x010d_f34c);
        for array in [0x48, 0xa0, 0xc0] {
            assert_eq!(e.mem.u32(this + array), 0);
            assert_eq!(e.mem.u32(this + array + 4), 0);
            assert_eq!(e.mem.u32(this + array + 8), 0x8000_0000);
        }
        for offset in (0x54..=0x98).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        for offset in (0x1c..=0x44).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u32(this + 0x9c), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 0xac), 0xffff_ffff);
    }

    #[test]
    fn destructor_releases_the_objects_and_frees_the_arrays() {
        let (mut e, log) = setup();
        let (_, allocator) = memory(&mut e, &log);
        log_callees(&mut e, &log, &[REMOVE_REFERENCE, UNARY_ACTION_DESTRUCTOR]);
        let this = e.mem.alloc(0x100);
        let objects = [
            (0x1c, 0x11),
            (0x20, 0x12),
            (0x24, 0),
            (0x28, 0x14),
            (0x2c, 0x15),
            (0x30, 0),
            (0x34, 0x17),
            (0x38, 0x18),
            (0x3c, 0x19),
            (0x44, 0),
            (0x9c, 0x1b),
            (0x40, 0x1c),
        ];
        for (offset, object) in objects {
            e.mem.set_u32(this + offset, object);
        }
        for (array, buffer, size, capacity) in [
            (0xc0, 0x5000, 3, 4),
            (0xa0, 0x6000, 2, 0x8000_0008),
            (0x48, 0x7000, 1, 2),
        ] {
            e.mem.set_u32(this + array, buffer);
            e.mem.set_u32(this + array + 4, size);
            e.mem.set_u32(this + array + 8, capacity);
        }
        e.call(0x00e3_8bf0, &args![this]);
        assert_eq!(e.mem.u32(this), 0x010d_f34c);
        let released: Vec<u32> = calls_to(&log, REMOVE_REFERENCE)
            .into_iter()
            .map(|a| a[0])
            .collect();
        assert_eq!(
            released,
            vec![0x11, 0x12, 0x14, 0x15, 0x17, 0x18, 0x19, 0x1b, 0x1c]
        );
        assert_eq!(
            calls_to(&log, ALLOCATOR_FREE),
            vec![vec![allocator, 0x5000, 16], vec![allocator, 0x7000, 0x1c0]]
        );
        for array in [0xc0, 0xa0, 0x48] {
            assert_eq!(e.mem.u32(this + array), 0);
            assert_eq!(e.mem.u32(this + array + 4), 0);
            assert_eq!(e.mem.u32(this + array + 8), 0x8000_0000);
        }
        assert_eq!(calls_to(&log, UNARY_ACTION_DESTRUCTOR), vec![vec![this]]);
    }

    #[test]
    fn fn_00e38d50_round_trips_the_wheel_floats() {
        let (mut e, log) = setup();
        let (this, _) = vehicle(&mut e, 0);
        let (_, allocator) = memory(&mut e, &log);
        array_reserve(&mut e, &log);
        let component = object_with_slots(&mut e, &log, &[0x0c]);
        e.mem.set_u32(this + 0x24, component);
        let floats = e.mem.alloc(8);
        e.mem.set_f32(floats, 1.5);
        e.mem.set_f32(floats + 4, 2.5);
        e.mem.set_u32(this + 0xc0, floats);
        e.mem.set_u32(this + 0xc4, 2);
        e.mem.set_f32(this + 0xbc, 0.75);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(SLOT_BASE + 0xc, move |e, a| {
            let local = a[4];
            let copy = e.mem.u32(local + 4);
            sink.borrow_mut().push((
                a.to_vec(),
                e.mem.f32(local),
                e.mem.f32(copy),
                e.mem.f32(copy + 4),
            ));
            e.mem.set_f32(local, 9.0);
            e.mem.set_f32(copy, 7.0);
            Ret::default()
        });
        e.call(0x00e3_8d50, &args![this, 0.25f32, 0x66u32]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0[..4], [component, 0.25f32.to_bits(), this, 0x66]);
        assert_eq!((seen[0].1, seen[0].2, seen[0].3), (0.75, 1.5, 2.5));
        assert_eq!(e.mem.f32(this + 0xbc), 9.0);
        assert_eq!(e.mem.f32(floats), 7.0);
        assert_eq!(e.mem.f32(floats + 4), 2.5);
        let reserved = calls_to(&log, ARRAY_RESERVE);
        assert_eq!(reserved.len(), 1);
        assert_eq!(
            (reserved[0][0], reserved[0][2], reserved[0][3]),
            (allocator, 2, 4)
        );
        let freed = calls_to(&log, ALLOCATOR_FREE);
        assert_eq!(freed.len(), 1);
        assert_eq!((freed[0][0], freed[0][2]), (allocator, 8));
    }

    #[test]
    fn fn_00e39400_frees_both_arrays() {
        let (mut e, log) = setup();
        let (_, allocator) = memory(&mut e, &log);
        let this = e.mem.alloc(0x40);
        for (array, buffer, size, capacity) in [(0xc, 0x5000, 5, 7), (0, 0x6000, 3, 3)] {
            e.mem.set_u32(this + array, buffer);
            e.mem.set_u32(this + array + 4, size);
            e.mem.set_u32(this + array + 8, capacity);
        }
        e.call(0x00e3_9400, &args![this]);
        assert_eq!(
            calls_to(&log, ALLOCATOR_FREE),
            vec![vec![allocator, 0x5000, 7], vec![allocator, 0x6000, 12]]
        );
        for array in [0xc, 0] {
            assert_eq!(e.mem.u32(this + array), 0);
            assert_eq!(e.mem.u32(this + array + 4), 0);
            assert_eq!(e.mem.u32(this + array + 8), 0x8000_0000);
        }
    }

    /// Gives the vehicle its seven component objects (`+0x20` .. `+0x38`, each with a logging
    /// slot `0xc`) and returns them in that order.
    fn install_components(e: &mut Engine, log: &Log, this: u32) -> [u32; 7] {
        [0x20, 0x24, 0x28, 0x2c, 0x30, 0x34, 0x38].map(|offset| {
            let component = object_with_slots(e, log, &[0xc]);
            e.mem.set_u32(this + offset, component);
            component
        })
    }

    #[test]
    fn update_components_runs_every_component_and_divides_the_result() {
        let (mut e, log) = setup();
        let (this, _, _, _, entry) = wheel_fixture(&mut e, &log);
        let (router, _) = memory(&mut e, &log);
        array_reserve(&mut e, &log);
        let components = install_components(&mut e, &log, this);
        let top = e.mem.alloc(0x100);
        e.mem.set_f32(top, 1.0);
        e.mem.set_u32(router, top);
        // The object at `this + 0x34` is both a component (slot 0xc) and the entry table owner.
        let tracker = components[5];
        let entries = e.mem.alloc(0x30);
        e.mem.set_u32(tracker + 8, entries);
        store_vec(&mut e, entries + 0x10, [0.0, 0.0, 1.0, 0.0]);
        e.mem.set_f64(0x0101_1588, 0.5);
        store_vec(&mut e, NEGATIVE_HALVES, [-0.5; 4]);
        log_callees(&mut e, &log, &[SET_ROTATED_DIR]);
        let scales = e.mem.alloc(8);
        e.mem.set_f32(scales, 2.0);
        e.mem.set_u32(this + 0xc0, scales);
        e.mem.set_u32(this + 0xc4, 1);
        e.mem.set_u32(this + 0xa4, 1);
        e.mem.set_f32(entry, 2.0);
        let result_component = components[4];
        let sink = Rc::clone(&log);
        let during = Rc::new(RefCell::new(0u32));
        let seen_top = Rc::clone(&during);
        e.register_double(SLOT_BASE + 0xc, move |e, a| {
            sink.borrow_mut().push((SLOT_BASE + 0xc, a.to_vec()));
            if a[0] == result_component {
                let floats = e.mem.u32(a[4]);
                e.mem.set_f32(floats, 3.0);
                *seen_top.borrow_mut() = e.mem.u32(router);
            }
            Ret::default()
        });
        let step_info = e.mem.alloc(0x10);
        e.mem.set_f32(step_info + 8, 0.5);
        let wheel_source = e.mem.alloc(0x60);
        let vectors = e.mem.alloc(0x20);
        let inputs_ref = e.mem.alloc(8);
        e.mem.set_u32(inputs_ref, 0x77);
        let outputs = e.mem.alloc(8);
        let outputs_ref = e.mem.alloc(8);
        e.mem.set_u32(outputs_ref, outputs);
        e.call(
            0x00e3_8e60,
            &args![
                this,
                step_info,
                wheel_source,
                vectors,
                inputs_ref,
                outputs_ref
            ],
        );
        let calls = slot_calls(&log, 0xc);
        let order: Vec<u32> = calls.iter().map(|a| a[0]).collect();
        let [c20, c24, c28, c2c, c30, c34, c38] = components;
        assert_eq!(order, vec![c20, c24, c2c, c28, c30, c34, c38]);
        let dt = 0.5f32.to_bits();
        assert_eq!(calls[5], vec![c34, dt, this, wheel_source, 0x77]);
        assert_eq!(calls[6], vec![c38, dt, this, vectors]);
        assert_eq!(e.mem.f32(outputs), 2.0);
        assert_eq!(e.mem.u32(router), top);
        assert_eq!(*during.borrow(), top + 0x80);
    }

    #[test]
    fn step_vehicle_using_wheel_collide_output_updates_then_simulates() {
        let (mut e, log) = setup();
        let (this, _) = vehicle(&mut e, 0);
        let body = e.mem.u32(this + 0x18);
        memory(&mut e, &log);
        array_reserve(&mut e, &log);
        let components = install_components(&mut e, &log, this);
        install_vtable(&mut e, &log, body + 0xe0, &[0x5c, 0x60, 0x44, 0x40]);
        log_callees(
            &mut e,
            &log,
            &[
                ENTITY_ACTIVATE,
                BUILD_ACCUMULATORS,
                BUILD_AXLE_BASIS,
                SET_ROTATED_DIR,
                0x00e5_9a40,
            ],
        );
        let step_info = e.mem.alloc(0x10);
        e.mem.set_f32(step_info + 8, 0.25);
        let collision = e.mem.alloc(0x60);
        e.call(0x00e3_9030, &args![this, step_info, collision]);
        let calls = slot_calls(&log, 0xc);
        let order: Vec<u32> = calls.iter().map(|a| a[0]).collect();
        let [c20, c24, c28, c2c, c30, c34, c38] = components;
        assert_eq!(order, vec![c20, c24, c2c, c28, c30, c34, c38]);
        let dt = 0.25f32.to_bits();
        assert_eq!(calls[5][..4], [c34, dt, this, collision]);
        let stepped = slot_calls(&log, 0x5c);
        assert_eq!(stepped.len(), 1);
        assert_eq!(stepped[0][..2], [body + 0xe0, dt]);
        assert_eq!(stepped[0][2], calls[6][3]);
        assert!(calls_to(&log, ALLOCATOR_FREE).is_empty());
        let first = log.borrow().iter().position(|(a, _)| *a == ENTITY_ACTIVATE);
        assert_eq!(first, Some(7));
    }

    #[test]
    fn step_vehicle_collides_the_wheels_then_steps_with_the_output() {
        let (mut e, log) = setup();
        let (this, _) = vehicle(&mut e, 0);
        let body = e.mem.u32(this + 0x18);
        let (router, _) = memory(&mut e, &log);
        e.mem.set_u32(router, 0x2000);
        array_reserve(&mut e, &log);
        install_components(&mut e, &log, this);
        let wheel_collide = object_with_slots(&mut e, &log, &[0x10, 0x18]);
        e.mem.set_u32(this + 0x3c, wheel_collide);
        install_vtable(&mut e, &log, body + 0xe0, &[0x5c, 0x60, 0x44, 0x40]);
        log_callees(
            &mut e,
            &log,
            &[
                ENTITY_ACTIVATE,
                BUILD_ACCUMULATORS,
                BUILD_AXLE_BASIS,
                SET_ROTATED_DIR,
                0x00e5_9a40,
            ],
        );
        let step_info = e.mem.alloc(0x10);
        e.mem.set_f32(step_info + 8, 0.25);
        e.call(0x00e3_9190, &args![this, step_info]);
        let dt = 0.25f32.to_bits();
        assert_eq!(slot_calls(&log, 0x18), vec![vec![wheel_collide, this]]);
        assert_eq!(
            slot_calls(&log, 0x10),
            vec![vec![wheel_collide, dt, this, 0x2000]]
        );
        let order: Vec<u32> = log.borrow().iter().map(|(a, _)| *a).collect();
        assert_eq!(order[..2], [SLOT_BASE + 0x18, SLOT_BASE + 0x10]);
        assert_eq!(slot_calls(&log, 0xc).len(), 7);
        assert_eq!(e.mem.u32(router), 0x2000);
        assert!(calls_to(&log, ALLOCATOR_FREE).is_empty());
    }

    #[test]
    fn apply_action_jumps_to_step_vehicle() {
        let (mut e, log) = setup();
        log_callees(&mut e, &log, &[0x00e3_9190]);
        e.call(0x00e3_92f0, &args![0x1000u32, 0x2000u32]);
        assert_eq!(calls_to(&log, 0x00e3_9190), vec![vec![0x1000, 0x2000]]);
    }

    #[test]
    fn fn_00e39480_copies_the_instance_state() {
        let (mut e, log) = setup();
        let (_, allocator) = memory(&mut e, &log);
        array_reserve(&mut e, &log);
        log_callees(&mut e, &log, &[STRING_PTR_ASSIGN, RECORD_ASSIGN]);
        let source = e.mem.alloc(0x100);
        let this = e.mem.alloc(0x100);
        e.mem.set_u16(source + 4, 0x1111);
        e.mem.set_u16(source + 6, 0x2222);
        for (k, offset) in [8u32, 0xc, 0x10].into_iter().enumerate() {
            e.mem.set_u32(source + offset, 0x100 + k as u32);
        }
        for k in 0..12u32 {
            e.mem.set_u32(source + 0x18 + 4 * k, 0x200 + k);
        }
        let records = e.mem.alloc(0x1c0);
        e.mem.set_u32(records + 0xe0, 0xabcd);
        e.mem.set_u32(records + 0xe0 + 0xdc, 0xdcba);
        e.mem.set_u32(source + 0x48, records);
        e.mem.set_u32(source + 0x4c, 2);
        e.mem.set_u32(source + 0x50, 2);
        for k in 0..0x12u32 {
            e.mem.set_u32(source + 0x54 + 4 * k, 0x300 + k);
        }
        e.mem.set_u32(source + 0x9c, 0x9c9c);
        let flags = e.mem.alloc(8);
        e.mem.write(flags, &[7, 8, 9]);
        e.mem.set_u32(source + 0xa0, flags);
        e.mem.set_u32(source + 0xa4, 3);
        for (offset, value) in [
            (0xac, 1.5f32),
            (0xb4, 2.5),
            (0xb8, 3.5),
            (0xbc, 4.5),
            (0xd0, 6.5),
        ] {
            e.mem.set_f32(source + offset, value);
        }
        e.mem.set_u8(source + 0xb0, 4);
        let floats = e.mem.alloc(8);
        e.mem.set_f32(floats, 1.25);
        e.mem.set_f32(floats + 4, 2.25);
        e.mem.set_u32(source + 0xc0, floats);
        e.mem.set_u32(source + 0xc4, 2);
        e.mem.write(source + 0xcc, &[1, 2, 3]);
        let own = e.mem.alloc(0xe0);
        e.mem.set_u32(this + 0x48, own);
        e.mem.set_u32(this + 0x4c, 1);
        e.mem.set_u32(this + 0x50, 1);
        let got = e.call(0x00e3_9480, &args![this, source]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u16(this + 4), 0x1111);
        assert_eq!(e.mem.u16(this + 6), 0x2222);
        assert_eq!(e.mem.u32(this + 0xc), 0x101);
        for k in 0..12u32 {
            assert_eq!(e.mem.u32(this + 0x18 + 4 * k), 0x200 + k);
        }
        assert_eq!(
            calls_to(&log, STRING_PTR_ASSIGN),
            vec![vec![this + 0x14, source + 0x14]]
        );
        assert_eq!(
            calls_to(&log, ARRAY_RESERVE),
            vec![vec![allocator, this + 0x48, 2, 0xe0]]
        );
        let copied = e.mem.u32(this + 0x48);
        assert_eq!(calls_to(&log, RECORD_ASSIGN), vec![vec![copied, records]]);
        assert_eq!(e.mem.u32(copied + 0xe0), 0xabcd);
        assert_eq!(e.mem.u32(copied + 0xe0 + 0xdc), 0xdcba);
        assert_eq!(e.mem.u32(this + 0x4c), 2);
        for k in 0..0x12u32 {
            assert_eq!(e.mem.u32(this + 0x54 + 4 * k), 0x300 + k);
        }
        assert_eq!(e.mem.u32(this + 0x9c), 0x9c9c);
        assert_eq!(e.mem.u32(this + 0xa4), 3);
        assert_eq!(e.mem.u32(this + 0xa8), 3);
        assert_eq!(e.mem.bytes(e.mem.u32(this + 0xa0), 3), vec![7, 8, 9]);
        assert_eq!(e.mem.u32(this + 0xc4), 2);
        assert_eq!(e.mem.u32(this + 0xc8), 2);
        let copied_floats = e.mem.u32(this + 0xc0);
        assert_eq!(e.mem.f32(copied_floats), 1.25);
        assert_eq!(e.mem.f32(copied_floats + 4), 2.25);
        for (offset, value) in [
            (0xac, 1.5f32),
            (0xb4, 2.5),
            (0xb8, 3.5),
            (0xbc, 4.5),
            (0xd0, 6.5),
        ] {
            assert_eq!(e.mem.f32(this + offset), value);
        }
        assert_eq!(e.mem.u8(this + 0xb0), 4);
        assert_eq!(e.mem.bytes(this + 0xcc, 3), vec![1, 2, 3]);
        assert_eq!(
            calls_to(&log, ALLOCATOR_FREE),
            vec![vec![allocator, 0, 0], vec![allocator, 0, 0]]
        );
    }

    #[test]
    fn clone_builds_a_copy_and_references_its_parts() {
        let (mut e, log) = setup();
        let (this, _) = vehicle(&mut e, 0);
        memory(&mut e, &log);
        log_callees(&mut e, &log, &[0x00e3_9480, ADD_REFERENCE]);
        let sink = Rc::clone(&log);
        e.register_double(0x00e3_8b20, move |e, a| {
            sink.borrow_mut().push((0x00e3_8b20, a.to_vec()));
            let offsets = [0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x44, 0x40];
            for (k, offset) in offsets.into_iter().enumerate() {
                e.mem.set_u32(a[0] + offset, 0x100 + k as u32);
            }
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let wheel_collide = object_with_slots(&mut e, &log, &[0x1c]);
        let cloned = e.mem.alloc(0x10);
        slot_double(&mut e, &log, 0x1c, cloned);
        e.mem.set_u32(this + 0x3c, wheel_collide);
        let component = object_with_slots(&mut e, &log, &[0xc]);
        slot_double(&mut e, &log, 0xc, 0x1234);
        e.mem.set_u32(this + 0x9c, component);
        let middle = e.mem.alloc(8);
        e.mem.set_u32(middle, 0x5000);
        let holder = e.mem.alloc(8);
        e.mem.set_u32(holder, middle);
        let copy = e.call(0x00e3_9210, &args![this, holder, 0x77u32]).u32();
        assert_eq!(e.mem.u16(copy + 4), 0xd4);
        assert_eq!(calls_to(&log, 0x00e3_8b20), vec![vec![copy, 0x5000]]);
        assert_eq!(calls_to(&log, 0x00e3_9480), vec![vec![copy, this]]);
        assert_eq!(e.mem.u32(copy + 8), 0);
        assert_eq!(e.mem.u32(copy + 0xc), 0);
        assert_eq!(e.mem.u32(copy + 0x18), 0x5000);
        assert_eq!(e.mem.u32(copy + 0x3c), cloned);
        assert_eq!(e.mem.u8(cloned + 8), 1);
        assert_eq!(e.mem.u32(copy + 0x9c), 0x1234);
        assert_eq!(
            slot_calls(&log, 0x1c),
            vec![vec![wheel_collide, 0x5000, 0x77]]
        );
        let referenced: Vec<u32> = calls_to(&log, ADD_REFERENCE)
            .into_iter()
            .map(|a| a[0])
            .collect();
        assert_eq!(referenced, (0x100..0x109).collect::<Vec<u32>>());
        e.mem.set_u32(this + 0x40, 5);
        e.call(0x00e3_9210, &args![this, holder, 0x77u32]);
        let referenced = calls_to(&log, ADD_REFERENCE);
        assert_eq!(referenced.len(), 19);
        assert_eq!(referenced[18], vec![0x109]);
    }
}
