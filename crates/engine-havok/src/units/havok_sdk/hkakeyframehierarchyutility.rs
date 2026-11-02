//! `Havok SDK/hkakeyframehierarchyutility.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The key-framing code of a ragdoll: `applyKeyFrame` drives every bone of a list towards a
//! target pose with a velocity and acceleration controller. The unnamed helpers are a length
//! limiter (`00d6b330`), a limiter with a ratio answer (`00d6b3c0`), a pass that turns the local
//! transforms of the bones into the transforms the controller aims at (`00d6b510`, optionally
//! blending towards the rotation the parent bone has now) and a rotation difference (`00d6bec0`).
//! Nothing here has an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{
    copy_vec, dot3, load_bits, load_vec, shufps, store_bits, store_vec, zip_with, Vec4,
};

/// The CRT square root (`00ec6040`, argument in ST0, result in ST0).
const X87_SQRT: u32 = 0x00ec_6040;
/// The memory router (`00c85750`): its first field is the stack cursor the scratch memory is
/// taken from.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkQuaternion::setMul` (`00c66320`, `this` = the result, `(a, b)`) and
/// `hkQuaternion::setSlerp` (`00cb2850`, `this` = the result, `(a, b, t)`).
const QUATERNION_SET_MUL: u32 = 0x00c6_6320;
const QUATERNION_SET_SLERP: u32 = 0x00cb_2850;
/// The two routines `applyKeyFrame` ends each bone with (`005616d0` for the position and
/// `00561800` for the rotation; both `this` = the bone's object, `(vector)`).
const BONE_SET_POSITION: u32 = 0x0056_16d0;
const BONE_SET_ROTATION: u32 = 0x0056_1800;
/// This unit's own routines, which the translations call the way the code does.
const LIMIT_LENGTH: u32 = 0x00d6_b330;
const LIMIT_RATIO: u32 = 0x00d6_b3c0;
const BONE_TRANSFORMS: u32 = 0x00d6_b510;
const ROTATION_DIFFERENCE: u32 = 0x00d6_bec0;
/// The SSE constants: four times -0.5 (`010c71e0`), four times 1.0 (`010ec7f0`), the four sign
/// bits (`010c44d0`), the fill `00d6b510` leaves in the third vector of every element
/// (`01267e30`), the tolerance (`01017d00`, a float) and the absolute-value mask (`01017d10`).
const NEG_HALF_LANES: u32 = 0x010c_71e0;
const ONE_LANES: u32 = 0x010e_c7f0;
const SIGN_BITS: u32 = 0x010c_44d0;
const ELEMENT_FILL: u32 = 0x0126_7e30;
const TOLERANCE: u32 = 0x0101_7d00;
const ABS_MASK: u32 = 0x0101_7d10;

pub(crate) fn add4(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x + y)
}

pub(crate) fn sub4(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x - y)
}

pub(crate) fn mul4(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x * y)
}

/// One lane in all four (`SHUFPS v, v, 0`).
pub(crate) fn splat(x: f32) -> Vec4 {
    [x; 4]
}

/// The float at `control + offset`, as an `FLD` / `FSTP` copy leaves it.
fn control_float(e: &Engine, control: u32, offset: u32) -> f32 {
    x87_quiet(e.mem.f32(control.wrapping_add(offset)))
}

/// `SHUFPS v, v, 0xc9` and `0xd2`: the two lane rotations of a cross product.
fn rotate_lanes_c9(v: Vec4) -> Vec4 {
    shufps(v, v, 0xc9)
}

fn rotate_lanes_d2(v: Vec4) -> Vec4 {
    shufps(v, v, 0xd2)
}

/// Half of `q` applied to `v`, the way the code spells it out with `SHUFPS`: `cross * w +
/// ((w * w - 0.5) * v + dot * q)` (the caller doubles it).
pub(crate) fn rotate_half(e: &Engine, q: Vec4, v: Vec4) -> Vec4 {
    let dot = dot3(q, v);
    let cross = sub4(
        mul4(rotate_lanes_c9(q), rotate_lanes_d2(v)),
        mul4(rotate_lanes_d2(q), rotate_lanes_c9(v)),
    );
    let w = splat(q[3]);
    let scaled = mul4(cross, w);
    let half = mul4(add4(mul4(w, w), load_vec(e, NEG_HALF_LANES)), v);
    let along = mul4(splat(dot), q);
    add4(scaled, add4(half, along))
}

/// `q` applied to `v` and moved by `t`: `half + half + t`.
pub(crate) fn rotate_and_move(e: &Engine, q: Vec4, v: Vec4, t: Vec4) -> Vec4 {
    let half = rotate_half(e, q, v);
    add4(add4(half, half), t)
}

/// The squared length of the first three lanes, `z + (y + x)` as the code adds them.
fn length_squared(v: Vec4) -> f32 {
    dot3(v, v)
}

/// `CMPLEPS` of `|target - value|` (the absolute-value mask on the bits) against the tolerance,
/// all four lanes at once: true when every lane is within it (NaN never is).
pub(crate) fn within_tolerance(e: &Engine, target: Vec4, value: Vec4) -> bool {
    let tolerance = e.mem.f32(TOLERANCE);
    let mask = load_bits(e, ABS_MASK);
    sub4(target, value)
        .iter()
        .zip(mask)
        .all(|(lane, mask)| f32::from_bits(lane.to_bits() & mask) <= tolerance)
}

// Translated from 00d6b330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (limits the length of a vector), `ESI` = the vector, `(limit)`. When the limit
/// squared (rounded to `float`) is below the squared length of the first three lanes (ordered
/// compare; equal or unordered leaves the vector alone), every lane is multiplied by
/// `(1 / sqrt(length squared)) * limit`, each step rounded to `float` (the square root is the CRT call
/// `00ec6040`).
pub fn fn_00d6b330(e: &mut Engine, vector: Ptr, limit: f32) {
    let v = load_vec(e, vector.addr());
    let squared = length_squared(v);
    let limit_squared = (f64::from(limit) * f64::from(limit)) as f32;
    if limit_squared < squared {
        let root = e.call(X87_SQRT, &args![f64::from(squared)]).f64() as f32;
        let inverse = (1.0 / f64::from(root)) as f32;
        let factor = (f64::from(inverse) * f64::from(limit)) as f32;
        store_vec(e, vector.addr(), mul4(splat(factor), v));
    }
}

// Translated from 00d6b3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (limits the length of a vector and answers how far it was), `ESI` = the vector,
/// `(inner, outer)`. The answer is `length squared / inner²` (each rounded to `float`). When
/// `inner²` is below the squared length (ordered compare), the vector is scaled by
/// `sqrt(inner² / length squared)` if `outer²` is above the squared length, else by `(inner *
/// outer) / length squared`; otherwise it is left alone.
pub fn fn_00d6b3c0(e: &mut Engine, vector: Ptr, inner: f32, outer: f32) -> f32 {
    let v = load_vec(e, vector.addr());
    let inner_squared = (f64::from(inner) * f64::from(inner)) as f32;
    let squared = length_squared(v);
    let ratio = (f64::from(squared) / f64::from(inner_squared)) as f32;
    if inner_squared < squared {
        let outer_squared = (f64::from(outer) * f64::from(outer)) as f32;
        let factor = if outer_squared > squared {
            let quotient = (f64::from(inner_squared) / f64::from(squared)) as f32;
            e.call(X87_SQRT, &args![f64::from(quotient)]).f64() as f32
        } else {
            ((f64::from(inner) * f64::from(outer)) / f64::from(squared)) as f32
        };
        store_vec(e, vector.addr(), mul4(splat(factor), v));
    }
    ratio
}

// Translated from 00d6b4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaKeyFrameHierarchyUtility::initialize` (Xbox PDB), cdecl `(list, out)`. For each of the
/// `list` count (at `+0`, signed) objects of the pointer array at `+4`, copies four vectors
/// (the object's `+0x140`, `+0x160`, `+0x1b0` and `+0x1c0`) into the 0x40-byte element of `out`
/// (in that order at `+0`, `+0x10`, `+0x20` and `+0x30`).
pub fn hka_key_frame_hierarchy_utility_initialize(e: &mut Engine, list: Ptr, out: Ptr) {
    let list = list.addr();
    let mut element = out.addr();
    let mut i = 0u32;
    while (i as i32) < (e.mem.u32(list) as i32) {
        let object = e
            .mem
            .u32(e.mem.u32(list + 4).wrapping_add(i.wrapping_mul(4)));
        for (from, to) in [(0x140, 0), (0x160, 0x10), (0x1b0, 0x20), (0x1c0, 0x30)] {
            copy_vec(e, object.wrapping_add(from), element.wrapping_add(to));
        }
        i += 1;
        element = element.wrapping_add(0x40);
    }
}

// Translated from 00d6b510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, `ECX` = `cursor`, cdecl `(list, weights, default_parent, out)`: for each bone `i` of
/// `list` (count at `+0`, objects at `+4`, 16-bit parent indices at `+8`, optional bone indices
/// at `+0xc` and optional factors at `+0x10`) turns the local transform at `cursor + 0x30 * i`
/// (translation, rotation, scale) into the 0x60-byte element of `out`: its `+0x30` translation,
/// `+0x40` rotation (`setMul` of the parent's rotation and the local one) and `+0x50` scale
/// (product of the parent's scale and the local one) are always written, with the parent being
/// the element of the parent index (its `+0x30` block), or `default_parent` for a negative one.
/// A bone index of `-1` stops there. Otherwise the weight is the float at `weights + 0x30 * index` (times the
/// factor, when there is one) and, when it is above zero and the parent index is not negative,
/// the transform is blended towards the one the parent bone object's `+0x120`/`+0x160` give
/// (rotation by `setSlerp`, translation linearly). Then the element's `+0`, `+0x10` and `+0x20`
/// vectors are written: the object's `+0x170` vector turned by the rotation, plus the
/// translation, the rotation, and the fill vector `01267e30`.
pub fn fn_00d6b510(
    e: &mut Engine,
    cursor: Ptr,
    list: Ptr,
    weights: Ptr,
    default_parent: Ptr,
    out: Ptr,
) {
    let list = list.addr();
    if (e.mem.u32(list) as i32) <= 0 {
        return;
    }
    // The locals: +0x30 the translation, +0x40 the rotation, +0x50 and +0x60 the parent
    // bone's translation and rotation, +0x80 the blended translation, +0x90 its rotation.
    e.with_stack(0xb0, |e, frame| {
        let f = frame.addr();
        let mut cursor = cursor.addr();
        let mut element = out.addr();
        let mut i = 0u32;
        loop {
            'element: {
                let parent_index = i32::from(
                    e.mem
                        .u16(e.mem.u32(list + 8).wrapping_add(i.wrapping_mul(2)))
                        as i16,
                );
                let parent = if parent_index >= 0 {
                    out.addr()
                        .wrapping_add((parent_index as u32).wrapping_mul(0x60))
                        .wrapping_add(0x30)
                } else {
                    default_parent.addr()
                };
                let moved = rotate_and_move(
                    e,
                    load_vec(e, parent + 0x10),
                    load_vec(e, cursor),
                    load_vec(e, parent),
                );
                store_vec(e, element + 0x30, moved);
                e.call(
                    QUATERNION_SET_MUL,
                    &args![element + 0x40, parent + 0x10, cursor + 0x10],
                );
                let parent_scale = load_vec(e, parent + 0x20);
                let local_scale = load_vec(e, cursor + 0x20);
                copy_vec(e, element + 0x40, f + 0x40);
                copy_vec(e, element + 0x30, f + 0x30);
                store_vec(e, element + 0x50, mul4(parent_scale, local_scale));
                let indices = e.mem.u32(list + 0xc);
                let index = if indices == 0 {
                    0
                } else {
                    let index = e.mem.u32(indices.wrapping_add(i.wrapping_mul(4)));
                    if index == u32::MAX {
                        break 'element;
                    }
                    index
                };
                let mut weight = x87_quiet(
                    e.mem
                        .f32(weights.addr().wrapping_add(index.wrapping_mul(0x30))),
                );
                let factors = e.mem.u32(list + 0x10);
                if factors != 0 {
                    let factor = x87_quiet(e.mem.f32(factors.wrapping_add(i.wrapping_mul(4))));
                    weight = (f64::from(factor) * f64::from(weight)) as f32;
                }
                if 0.0 < weight && parent_index >= 0 {
                    let bones = e.mem.u32(list + 4);
                    let parent_bone = e
                        .mem
                        .u32(bones.wrapping_add((parent_index as u32).wrapping_mul(4)));
                    copy_vec(e, parent_bone + 0x120, f + 0x50);
                    copy_vec(e, parent_bone + 0x160, f + 0x60);
                    let blended = rotate_and_move(
                        e,
                        load_vec(e, f + 0x60),
                        load_vec(e, cursor),
                        load_vec(e, f + 0x50),
                    );
                    store_vec(e, f + 0x80, blended);
                    e.call(
                        QUATERNION_SET_MUL,
                        &args![f + 0x90, f + 0x60, cursor + 0x10],
                    );
                    // The scale the code multiplies out here (`01012068` times the local
                    // scale) goes to a local that nothing reads.
                    e.call(
                        QUATERNION_SET_SLERP,
                        &args![f + 0x40, f + 0x40, f + 0x90, weight],
                    );
                    let own = load_vec(e, f + 0x30);
                    let target = load_vec(e, f + 0x80);
                    let one_minus = sub4(load_vec(e, ONE_LANES), splat(weight));
                    store_vec(
                        e,
                        f + 0x30,
                        add4(mul4(one_minus, own), mul4(splat(weight), target)),
                    );
                }
                let bone = e
                    .mem
                    .u32(e.mem.u32(list + 4).wrapping_add(i.wrapping_mul(4)));
                let rotation = load_vec(e, f + 0x40);
                let position = rotate_and_move(
                    e,
                    rotation,
                    load_vec(e, bone + 0x170),
                    load_vec(e, f + 0x30),
                );
                store_vec(e, element, position);
                store_vec(e, element + 0x10, rotation);
                let fill = load_bits(e, ELEMENT_FILL);
                store_bits(e, element + 0x20, fill);
            }
            i += 1;
            cursor = cursor.wrapping_add(0x30);
            element = element.wrapping_add(0x60);
            if (i as i32) >= (e.mem.u32(list) as i32) {
                break;
            }
        }
    });
}

// Translated from 00d6b8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaKeyFrameHierarchyUtility::applyKeyFrame` (Xbox PDB), cdecl `(time_step, target, list,
/// controls, errors)`. Takes `count * 0x60` bytes (rounded up to 0x80) from the router's stack
/// for the elements `00d6b510` builds from the target's local transforms (`target + 0x30`, with
/// `target` itself as the default parent), then drives every bone of `list` (a bone index of
/// `-1` skipped) towards its element with the 0x30-byte control record `controls[index]`.
/// Reading `inverse` as `1 / time_step`, the record holds (as floats) at `+4` the share left of
/// the target velocity, at `+8`, `+0xc`, `+0x10` and `+0x1c` four gains (each multiplied by the
/// list's factor for the bone when there is one) and at `+0x14`, `+0x18`, `+0x20`, `+0x24`,
/// `+0x28`, `+0x2c` six limits. The state lives in 0x40-byte entries from `target + 0x34`.
/// The position stage blends the bone's `+0x1b0` vector, the speeds taken from the element's
/// position against the bone's `+0x140` and against the state, limited by `00d6b330`; when the
/// fourth gain is not zero a second pass turns the element's rotation onto the bone's `+0x170`
/// vector, limited by `00d6b3c0` (its answer goes into `errors[i]`, when `errors` is not null).
/// The rotation stage does the same with `00d6bec0` differences and the `+0x1c0` vector (its
/// limiter's answer is added to `errors[i]`). The bone object gets `005616d0` (position) or
/// `00561800` (rotation) with the result unless every lane is within the tolerance of its
/// `+0x1b0` / `+0x1c0` vector. The router's stack ends where it began.
pub fn apply_key_frame(
    e: &mut Engine,
    time_step: f32,
    target: Ptr,
    list: Ptr,
    controls: Ptr,
    errors: Ptr,
) {
    let list = list.addr();
    let target = target.addr();
    let count = e.mem.u32(list);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let scratch = e.mem.u32(router);
    let size = count.wrapping_mul(0x60).wrapping_add(0x7f) & 0xffff_ff80;
    e.mem.set_u32(router, size.wrapping_add(scratch));
    let transforms = e.mem.u32(target + 0x30);
    e.call(
        BONE_TRANSFORMS,
        &args![transforms, list, controls, target, scratch],
    );
    let inverse = (1.0 / f64::from(time_step)) as f32;
    if (e.mem.u32(list) as i32) > 0 {
        // The locals the callees are given: +0x50 the rotation stage's result, +0x60 the position
        // stage's, +0x90 and +0xf0/+0x100 the rotation differences, +0xc0, +0xd0 and +0xe0 the
        // vectors the limiters work on.
        e.with_stack(0x140, |e, frame| {
            let f = frame.addr();
            let inv = splat(inverse);
            let mut state = 0u32;
            let mut element = scratch.wrapping_add(0x40);
            let mut i = 0u32;
            loop {
                'bone: {
                    let indices = e.mem.u32(list + 0xc);
                    let index = if indices == 0 {
                        0
                    } else {
                        let index = e.mem.u32(indices.wrapping_add(i.wrapping_mul(4)));
                        if index == u32::MAX {
                            break 'bone;
                        }
                        index
                    };
                    let bone = e
                        .mem
                        .u32(e.mem.u32(list + 4).wrapping_add(i.wrapping_mul(4)));
                    let control = controls.addr().wrapping_add(index.wrapping_mul(0x30));
                    let mut gain_a = control_float(e, control, 8);
                    let mut gain_b = control_float(e, control, 0xc);
                    let mut gain_c = control_float(e, control, 0x10);
                    let mut gain_d = control_float(e, control, 0x1c);
                    let factors = e.mem.u32(list + 0x10);
                    if factors != 0 {
                        let factor = x87_quiet(e.mem.f32(factors.wrapping_add(i.wrapping_mul(4))));
                        let scale = |gain: f32| (f64::from(factor) * f64::from(gain)) as f32;
                        gain_a = scale(gain_a);
                        gain_b = scale(gain_b);
                        gain_c = scale(gain_c);
                        gain_d = scale(gain_d);
                    }
                    let kept = (1.0 - f64::from(control_float(e, control, 4))) as f32;
                    let slot = e.mem.u32(target + 0x34).wrapping_add(state);
                    let bone_position = load_vec(e, bone + 0x140);
                    let old_position = load_vec(e, slot);
                    let position = load_vec(e, element - 0x40);
                    store_vec(e, slot, position);
                    let speed = mul4(sub4(position, bone_position), inv);
                    let slot_speed = mul4(sub4(position, old_position), inv);
                    let old_slot_speed = load_vec(e, slot + 0x20);
                    store_vec(e, slot + 0x20, slot_speed);
                    let goal = load_vec(e, bone + 0x1b0);
                    let acceleration = sub4(slot_speed, old_slot_speed);
                    let stage_one =
                        add4(mul4(splat(gain_a), acceleration), mul4(goal, splat(kept)));
                    let stage_two =
                        add4(mul4(splat(gain_b), sub4(slot_speed, stage_one)), stage_one);
                    let pull = mul4(splat(gain_c), sub4(speed, stage_two));
                    store_vec(e, f + 0xe0, pull);
                    let limit = control_float(e, control, 0x14);
                    e.call(LIMIT_LENGTH, &args![f + 0xe0, limit]);
                    let mut result = add4(load_vec(e, f + 0xe0), stage_two);
                    store_vec(e, f + 0x60, result);
                    if gain_d != 0.0 {
                        let rotation = load_vec(e, element);
                        let ratio_limit = (f64::from(gain_d)
                            * (f64::from(control_float(e, control, 0x28)) * f64::from(inverse)))
                            as f32;
                        let turned = rotate_and_move(
                            e,
                            rotation,
                            load_vec(e, bone + 0x170),
                            load_vec(e, element - 0x10),
                        );
                        let wanted = mul4(sub4(turned, load_vec(e, bone + 0x140)), inv);
                        let push = mul4(sub4(wanted, result), splat(gain_d));
                        store_vec(e, f + 0xc0, push);
                        let inner = control_float(e, control, 0x20);
                        let ratio = e
                            .call(LIMIT_RATIO, &args![f + 0xc0, inner, ratio_limit])
                            .f32();
                        let errors = errors.addr();
                        if errors != 0 {
                            e.mem
                                .set_f32(errors.wrapping_add(i.wrapping_mul(4)), x87_quiet(ratio));
                        }
                        result = add4(load_vec(e, f + 0xc0), load_vec(e, f + 0x60));
                        store_vec(e, f + 0x60, result);
                    }
                    if !within_tolerance(e, load_vec(e, bone + 0x1b0), result) {
                        e.call(BONE_SET_POSITION, &args![bone, f + 0x60]);
                    }
                    let rotation_slot = element - 0x30;
                    e.call(
                        ROTATION_DIFFERENCE,
                        &args![bone + 0x160, rotation_slot, f + 0xf0],
                    );
                    e.call(
                        ROTATION_DIFFERENCE,
                        &args![slot + 0x10, rotation_slot, f + 0x100],
                    );
                    copy_vec(e, rotation_slot, slot + 0x10);
                    let first_difference = load_vec(e, f + 0xf0);
                    let second_difference = load_vec(e, f + 0x100);
                    let old_rotation_speed = load_vec(e, slot + 0x30);
                    let rotation_speed = mul4(second_difference, inv);
                    store_vec(e, slot + 0x30, rotation_speed);
                    let rotation_goal = load_vec(e, bone + 0x1c0);
                    let rotation_acceleration = sub4(rotation_speed, old_rotation_speed);
                    let stage_one = add4(
                        mul4(splat(gain_a), rotation_acceleration),
                        mul4(rotation_goal, splat(kept)),
                    );
                    let remainder = sub4(rotation_speed, stage_one);
                    let stage_two = add4(mul4(splat(gain_b), remainder), stage_one);
                    store_vec(e, f + 0x100, remainder);
                    store_vec(e, f + 0x50, stage_two);
                    let rotation_pull = sub4(mul4(first_difference, inv), stage_two);
                    store_vec(e, f + 0xf0, rotation_pull);
                    let pull = mul4(splat(gain_c), rotation_pull);
                    store_vec(e, f + 0xd0, pull);
                    let limit = control_float(e, control, 0x18);
                    e.call(LIMIT_LENGTH, &args![f + 0xd0, limit]);
                    let mut rotation_result = add4(load_vec(e, f + 0xd0), load_vec(e, f + 0x50));
                    store_vec(e, f + 0x50, rotation_result);
                    if gain_d != 0.0 {
                        e.call(ROTATION_DIFFERENCE, &args![bone + 0x160, element, f + 0x90]);
                        let ratio_limit = ((f64::from(control_float(e, control, 0x2c))
                            * f64::from(inverse))
                            * f64::from(gain_d)) as f32;
                        let wanted = mul4(load_vec(e, f + 0x90), inv);
                        let push = mul4(sub4(wanted, load_vec(e, f + 0x50)), splat(gain_d));
                        store_vec(e, f + 0x90, push);
                        let inner = control_float(e, control, 0x24);
                        let ratio = e
                            .call(LIMIT_RATIO, &args![f + 0x90, inner, ratio_limit])
                            .f32();
                        let errors = errors.addr();
                        if errors != 0 {
                            let at = errors.wrapping_add(i.wrapping_mul(4));
                            let sum = f64::from(x87_quiet(e.mem.f32(at))) + f64::from(ratio);
                            e.mem.set_f32(at, sum as f32);
                        }
                        rotation_result = add4(load_vec(e, f + 0x90), load_vec(e, f + 0x50));
                        store_vec(e, f + 0x50, rotation_result);
                    }
                    if !within_tolerance(e, load_vec(e, bone + 0x1c0), rotation_result) {
                        e.call(BONE_SET_ROTATION, &args![bone, f + 0x50]);
                    }
                }
                i += 1;
                element = element.wrapping_add(0x60);
                state = state.wrapping_add(0x40);
                if (i as i32) >= (e.mem.u32(list) as i32) {
                    break;
                }
            }
        });
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, scratch);
}

// Translated from 00d6bec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, `ECX` = `a`, `(b, out)`: a rotation difference from two quaternion vectors. Writes
/// `out` step by step (each step reloads `a` and `b` from memory): `b.zxyw * a.yzxw - b.yzxw *
/// a.zxyw`, minus `b.w * a`, then `2 * ((a.w * b) + that)`. When the sum of the lane products
/// `a * b` (added pairwise, lane 0 of the result) has its sign bit set, the result is
/// `XORPS`ed with the sign bits `010c44d0`.
pub fn fn_00d6bec0(e: &mut Engine, a: Ptr, b: Ptr, out: Ptr) {
    let (a, b, out) = (a.addr(), b.addr(), out.addr());
    let b_vec = load_vec(e, b);
    let a_vec = load_vec(e, a);
    let cross = crate::sse::subps(
        crate::sse::mulps(rotate_lanes_d2(b_vec), rotate_lanes_c9(a_vec)),
        crate::sse::mulps(rotate_lanes_c9(b_vec), rotate_lanes_d2(a_vec)),
    );
    store_vec(e, out, cross);
    let b_w = splat(load_vec(e, b)[3]);
    let a_vec = load_vec(e, a);
    let cross = crate::sse::subps(cross, crate::sse::mulps(b_w, a_vec));
    store_vec(e, out, cross);
    let a_vec = load_vec(e, a);
    let b_vec = load_vec(e, b);
    let sum = crate::sse::addps(crate::sse::mulps(splat(a_vec[3]), b_vec), cross);
    let doubled = crate::sse::addps(sum, sum);
    store_vec(e, out, doubled);
    let products = crate::sse::mulps(load_vec(e, b), load_vec(e, a));
    let pairs = crate::sse::addps(shufps(products, products, 0x4e), products);
    let total = crate::sse::addps(shufps(pairs, pairs, 0xb1), pairs);
    if total[0].to_bits() >> 31 != 0 {
        let signs = load_bits(e, SIGN_BITS);
        let flipped: [u32; 4] = std::array::from_fn(|lane| doubled[lane].to_bits() ^ signs[lane]);
        store_bits(e, out, flipped);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_b330, fn_00d6b330(Ptr, f32)),
        entry!(0x00d6_b3c0, fn_00d6b3c0(Ptr, f32, f32) -> f32),
        entry!(
            0x00d6_b4b0,
            hka_key_frame_hierarchy_utility_initialize(Ptr, Ptr)
        ),
        entry!(0x00d6_b510, fn_00d6b510(Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00d6_b8e0, apply_key_frame(f32, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00d6_bec0, fn_00d6bec0(Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn entry(address: u32, words: &[u32]) -> (u32, Vec<u32>) {
        (address, words.to_vec())
    }

    /// An engine with the constants in memory (-0.5, 1.0, the sign bits of lanes 0 and 2, a fill
    /// vector, the tolerance 0.001 and the absolute-value mask) and the CRT square root.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.register(X87_SQRT, |_, a| {
            let value = f64::from_bits((u64::from(a[1]) << 32) | u64::from(a[0]));
            Ret {
                st0: value.sqrt(),
                ..Ret::default()
            }
        });
        store_vec(&mut e, NEG_HALF_LANES, [-0.5; 4]);
        store_vec(&mut e, ONE_LANES, [1.0; 4]);
        store_bits(&mut e, SIGN_BITS, [0x8000_0000, 0, 0x8000_0000, 0]);
        store_vec(&mut e, ELEMENT_FILL, [9.0, 8.0, 7.0, 6.0]);
        e.mem.set_f32(TOLERANCE, 0.001);
        store_bits(&mut e, ABS_MASK, [0x7fff_ffff; 4]);
        e
    }

    fn put(e: &mut Engine, at: u32, v: Vec4) {
        store_vec(e, at, v);
    }

    fn get(e: &Engine, at: u32) -> Vec4 {
        load_vec(e, at)
    }

    fn bits(v: Vec4) -> Vec<u32> {
        v.iter().map(|lane| lane.to_bits()).collect()
    }

    #[test]
    fn the_length_limiter_scales_only_longer_vectors() {
        let mut e = engine();
        let at = e.mem.alloc(16);
        let mut run = |v: Vec4, limit: f32| {
            put(&mut e, at, v);
            e.call(0x00d6_b330, &args![at, limit]);
            get(&e, at)
        };
        // Length 5, limit 2.5: 1 / 5 is rounded to float before the multiplication; the last
        // lane is scaled too.
        assert_eq!(run([3.0, 4.0, 0.0, 10.0], 2.5), [1.5, 2.0, 0.0, 5.0]);
        assert_eq!(run([3.0, 4.0, 0.0, 10.0], -2.5), [-1.5, -2.0, 0.0, -5.0]);
        // A limit at or above the length (or not a number) leaves it alone.
        assert_eq!(run([3.0, 4.0, 0.0, 10.0], 5.0), [3.0, 4.0, 0.0, 10.0]);
        assert_eq!(run([3.0, 4.0, 0.0, 10.0], 6.0), [3.0, 4.0, 0.0, 10.0]);
        assert_eq!(
            bits(run([3.0, 4.0, 0.0, 10.0], f32::NAN)),
            bits([3.0, 4.0, 0.0, 10.0])
        );
        assert_eq!(run([0.0; 4], 0.0), [0.0; 4]);
    }

    #[test]
    fn the_ratio_limiter_answers_the_ratio_and_scales_by_one_of_two_factors() {
        let mut e = engine();
        let at = e.mem.alloc(16);
        let mut run = |v: Vec4, inner: f32, outer: f32| {
            put(&mut e, at, v);
            let ratio = e.call(0x00d6_b3c0, &args![at, inner, outer]).f32();
            (ratio, get(&e, at))
        };
        let v = [3.0, 4.0, 0.0, 8.0];
        // The inner limit is above the length: nothing changes.
        assert_eq!(run(v, 10.0, 1.0), (0.25, v));
        // Outer squared (25) not above the squared length (25): (inner * outer) / squared.
        assert_eq!(run(v, 2.5, 5.0), (4.0, [1.5, 2.0, 0.0, 4.0]));
        // Outer squared above it: sqrt(inner squared / squared), here 0.5.
        assert_eq!(run(v, 2.5, 6.0), (4.0, [1.5, 2.0, 0.0, 4.0]));
        // A zero inner limit: the ratio is infinite, the factor (0 * outer) / squared is zero.
        let (ratio, scaled) = run(v, 0.0, 1.0);
        assert_eq!((ratio, scaled), (f32::INFINITY, [0.0; 4]));
        // Not a number: as inner it is unordered (nothing changes), as outer it is not above
        // the squared length, so the factor is not a number.
        let (ratio, same) = run(v, f32::NAN, 1.0);
        assert!(ratio.is_nan());
        assert_eq!(bits(same), bits(v));
        let (ratio, nans) = run(v, 2.5, f32::NAN);
        assert_eq!(ratio, 4.0);
        assert!(nans.iter().all(|lane| lane.is_nan()));
    }

    #[test]
    fn initialize_copies_four_vectors_of_every_object() {
        let mut e = engine();
        let list = e.mem.alloc(8);
        let objects = e.mem.alloc(8);
        e.mem.set_u32(list, 2);
        e.mem.set_u32(list + 4, objects);
        for k in 0..2u32 {
            let object = e.mem.alloc(0x200);
            e.mem.set_u32(objects + 4 * k, object);
            for (n, offset) in [0x140u32, 0x160, 0x1b0, 0x1c0].into_iter().enumerate() {
                let base = (100 * k + 10 * n as u32) as f32;
                put(
                    &mut e,
                    object + offset,
                    [base, base + 1.0, base + 2.0, base + 3.0],
                );
            }
        }
        let out = e.mem.alloc(0x80);
        e.call(0x00d6_b4b0, &args![list, out]);
        for k in 0..2u32 {
            // Offsets 0x140, 0x160, 0x1b0, 0x1c0 land at +0, +0x10, +0x20, +0x30.
            for n in 0..4u32 {
                let base = (100 * k + 10 * n) as f32;
                assert_eq!(
                    get(&e, out + 0x40 * k + 0x10 * n),
                    [base, base + 1.0, base + 2.0, base + 3.0]
                );
            }
        }
        // A count that is zero or negative copies nothing.
        let untouched = e.mem.alloc(0x40);
        for count in [0u32, u32::MAX] {
            e.mem.set_u32(list, count);
            e.call(0x00d6_b4b0, &args![list, untouched]);
            assert_eq!(get(&e, untouched), [0.0; 4]);
        }
    }

    #[test]
    fn the_rotation_difference_follows_the_step_by_step_formula() {
        let mut e = engine();
        let a = e.mem.alloc(16);
        let b = e.mem.alloc(16);
        let out = e.mem.alloc(16);
        let run = |e: &mut Engine, a_vec: Vec4, b_vec: Vec4| {
            put(e, a, a_vec);
            put(e, b, b_vec);
            e.call(0x00d6_bec0, &args![a, b, out]);
            get(e, out)
        };
        // b = (1, 2, 3, 4), a = (5, 6, 7, 8): cross (4, -8, 4, 0), minus 4 * a, then twice
        // (8 * b + that); the lane products sum to a positive number.
        assert_eq!(
            run(&mut e, [5.0, 6.0, 7.0, 8.0], [1.0, 2.0, 3.0, 4.0]),
            [-16.0, -32.0, 0.0, 0.0]
        );
        // With b negated the sum is negative: the sign bits at 010c44d0 (lanes 0 and 2) flip.
        let flipped = run(&mut e, [5.0, 6.0, 7.0, 8.0], [-1.0, -2.0, -3.0, -4.0]);
        assert_eq!(bits(flipped), bits([-16.0, 32.0, -0.0, 0.0]));
        // Two identity quaternions give zero and no flip.
        assert_eq!(
            bits(run(&mut e, [0.0, 0.0, 0.0, 1.0], [0.0, 0.0, 0.0, 1.0])),
            bits([0.0; 4])
        );
    }

    /// The pieces `00d6b510` works on.
    struct Rig {
        list: u32,
        cursor: u32,
        weights: u32,
        parent: u32,
        out: u32,
        bones: Vec<u32>,
        log: Log,
    }

    /// A list of bones with the given 16-bit parent indices (and optional bone indices and
    /// factors), local transforms, a weights table (floats 0x30 bytes apart), a default parent
    /// block and room for the elements; `setMul` and `setSlerp` doubles log their arguments
    /// (and write `a + 2 * b` with `a.w * b.w`, and the linear interpolation from `a` to `b`).
    fn rig(
        e: &mut Engine,
        parents: &[i16],
        indices: Option<&[i32]>,
        factors: Option<&[f32]>,
        weights: &[f32],
    ) -> Rig {
        let count = parents.len() as u32;
        let list = e.mem.alloc(0x20);
        let bones_array = e.mem.alloc(4 * count);
        let parent_array = e.mem.alloc(2 * count + 2);
        e.mem.set_u32(list, count);
        e.mem.set_u32(list + 4, bones_array);
        e.mem.set_u32(list + 8, parent_array);
        let mut bones = Vec::new();
        for (k, parent) in parents.iter().enumerate() {
            let bone = e.mem.alloc(0x200);
            e.mem.set_u32(bones_array + 4 * k as u32, bone);
            e.mem.set_u16(parent_array + 2 * k as u32, *parent as u16);
            bones.push(bone);
        }
        if let Some(indices) = indices {
            let array = e.mem.alloc(4 * count);
            for (k, index) in indices.iter().enumerate() {
                e.mem.set_u32(array + 4 * k as u32, *index as u32);
            }
            e.mem.set_u32(list + 0xc, array);
        }
        if let Some(factors) = factors {
            let array = e.mem.alloc(4 * count);
            for (k, factor) in factors.iter().enumerate() {
                e.mem.set_f32(array + 4 * k as u32, *factor);
            }
            e.mem.set_u32(list + 0x10, array);
        }
        let weights_base = e.mem.alloc(0x30 * weights.len() as u32);
        for (k, weight) in weights.iter().enumerate() {
            e.mem.set_f32(weights_base + 0x30 * k as u32, *weight);
        }
        let cursor = e.mem.alloc(0x30 * count);
        let parent = e.mem.alloc(0x30);
        put(e, parent, [10.0, 0.0, 0.0, 1.0]);
        put(e, parent + 0x10, [0.0, 0.0, 0.0, 1.0]);
        put(e, parent + 0x20, [2.0, 2.0, 2.0, 1.0]);
        let out = e.mem.alloc(0x60 * count + 0x40);
        let log: Log = Rc::default();
        let l = Rc::clone(&log);
        e.register_double(QUATERNION_SET_MUL, move |e, a| {
            l.borrow_mut().push((QUATERNION_SET_MUL, a.to_vec()));
            let (p, q) = (get(e, a[1]), get(e, a[2]));
            put(
                e,
                a[0],
                [
                    p[0] + 2.0 * q[0],
                    p[1] + 2.0 * q[1],
                    p[2] + 2.0 * q[2],
                    p[3] * q[3],
                ],
            );
            Ret::default()
        });
        let l = Rc::clone(&log);
        e.register_double(QUATERNION_SET_SLERP, move |e, a| {
            let (p, q, t) = (get(e, a[1]), get(e, a[2]), f32::from_bits(a[3]));
            let mut seen = a.to_vec();
            seen.extend(bits(p));
            seen.extend(bits(q));
            l.borrow_mut().push((QUATERNION_SET_SLERP, seen));
            put(
                e,
                a[0],
                std::array::from_fn(|lane| p[lane] + t * (q[lane] - p[lane])),
            );
            Ret::default()
        });
        Rig {
            list,
            cursor,
            weights: weights_base,
            parent,
            out,
            bones,
            log,
        }
    }

    fn run_rig(e: &mut Engine, rig: &Rig) {
        e.call(
            0x00d6_b510,
            &args![rig.cursor, rig.list, rig.weights, rig.parent, rig.out],
        );
    }

    /// Two bones, the second the child of the first, and local transforms: bone 0 at (1, 2, 3)
    /// with the identity rotation and scale 1, bone 1 at (0, 0, 5) with rotation (0.5, 0, 0, 1)
    /// and scale 3.
    fn two_bones(e: &mut Engine, weights: &[f32], factors: Option<&[f32]>) -> Rig {
        let rig = rig(e, &[-1, 0], None, factors, weights);
        put(e, rig.cursor, [1.0, 2.0, 3.0, 0.0]);
        put(e, rig.cursor + 0x10, [0.0, 0.0, 0.0, 1.0]);
        put(e, rig.cursor + 0x20, [1.0, 1.0, 1.0, 1.0]);
        put(e, rig.cursor + 0x30, [0.0, 0.0, 5.0, 0.0]);
        put(e, rig.cursor + 0x40, [0.5, 0.0, 0.0, 1.0]);
        put(e, rig.cursor + 0x50, [3.0, 3.0, 3.0, 1.0]);
        put(e, rig.bones[0] + 0x170, [7.0, 8.0, 9.0, 0.0]);
        put(e, rig.bones[1] + 0x170, [0.0, 0.0, 1.0, 0.0]);
        rig
    }

    #[test]
    fn the_elements_chain_through_their_parents() {
        let mut e = engine();
        let rig = two_bones(&mut e, &[0.0], None);
        run_rig(&mut e, &rig);
        let (o0, o1) = (rig.out, rig.out + 0x60);
        // Bone 0 against the default parent: translation (1, 2, 3) + (10, 0, 0, 1), rotation
        // from setMul, scale 2 * 1.
        assert_eq!(get(&e, o0 + 0x30), [11.0, 2.0, 3.0, 1.0]);
        assert_eq!(get(&e, o0 + 0x40), [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(get(&e, o0 + 0x50), [2.0, 2.0, 2.0, 1.0]);
        // The finished vectors: the object's +0x170 vector plus the translation, the rotation
        // and the fill.
        assert_eq!(get(&e, o0), [18.0, 10.0, 12.0, 1.0]);
        assert_eq!(get(&e, o0 + 0x10), [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(get(&e, o0 + 0x20), [9.0, 8.0, 7.0, 6.0]);
        // Bone 1 builds on the +0x30 block of bone 0's element.
        assert_eq!(get(&e, o1 + 0x30), [11.0, 2.0, 8.0, 1.0]);
        assert_eq!(get(&e, o1 + 0x40), [1.0, 0.0, 0.0, 1.0]);
        assert_eq!(get(&e, o1 + 0x50), [6.0, 6.0, 6.0, 1.0]);
        // Rotating (0, 0, 1) by (1, 0, 0, 1) gives half (0, -1, 0.5); doubled and moved.
        assert_eq!(get(&e, o1), [11.0, 0.0, 9.0, 1.0]);
        assert_eq!(get(&e, o1 + 0x10), [1.0, 0.0, 0.0, 1.0]);
        assert_eq!(get(&e, o1 + 0x20), [9.0, 8.0, 7.0, 6.0]);
        assert_eq!(
            rig.log.borrow()[..],
            [
                (
                    QUATERNION_SET_MUL,
                    vec![o0 + 0x40, rig.parent + 0x10, rig.cursor + 0x10]
                ),
                (
                    QUATERNION_SET_MUL,
                    vec![o1 + 0x40, o0 + 0x40, rig.cursor + 0x40]
                ),
            ]
        );
    }

    #[test]
    fn a_bone_index_of_minus_one_only_builds_the_temporary_vectors() {
        let mut e = engine();
        let rig = rig(&mut e, &[-1, -1], Some(&[-1, 1]), None, &[5.0, 0.0]);
        put(&mut e, rig.cursor, [1.0, 2.0, 3.0, 0.0]);
        put(&mut e, rig.cursor + 0x10, [0.0, 0.0, 0.0, 1.0]);
        put(&mut e, rig.cursor + 0x20, [1.0; 4]);
        put(&mut e, rig.cursor + 0x30, [4.0, 5.0, 6.0, 0.0]);
        put(&mut e, rig.cursor + 0x40, [0.0, 0.0, 0.0, 1.0]);
        put(&mut e, rig.cursor + 0x50, [1.0; 4]);
        put(&mut e, rig.bones[1] + 0x170, [1.0, 1.0, 1.0, 0.0]);
        run_rig(&mut e, &rig);
        let (o0, o1) = (rig.out, rig.out + 0x60);
        // Bone 0: the temporaries only (the index stops it before the weight is read).
        assert_eq!(get(&e, o0 + 0x30), [11.0, 2.0, 3.0, 1.0]);
        assert_eq!(get(&e, o0 + 0x40), [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(get(&e, o0 + 0x50), [2.0, 2.0, 2.0, 1.0]);
        for at in [o0, o0 + 0x10, o0 + 0x20] {
            assert_eq!(get(&e, at), [0.0; 4]);
        }
        // Bone 1 uses weights[1] = 0 (no blend), the default parent and everything else.
        assert_eq!(get(&e, o1 + 0x30), [14.0, 5.0, 6.0, 1.0]);
        assert_eq!(get(&e, o1), [15.0, 6.0, 7.0, 1.0]);
        assert_eq!(get(&e, o1 + 0x20), [9.0, 8.0, 7.0, 6.0]);
    }

    #[test]
    fn a_positive_weight_blends_towards_the_parent_bone() {
        let mut e = engine();
        // weights[0] = 1 times the factors (1, 0.5): bone 0 has no parent (nothing blends),
        // bone 1 blends with 0.5.
        let rig = two_bones(&mut e, &[1.0], Some(&[1.0, 0.5]));
        let parent_bone = rig.bones[0];
        put(&mut e, parent_bone + 0x120, [100.0, 200.0, 300.0, 1.0]);
        put(&mut e, parent_bone + 0x160, [0.0, 0.0, 0.0, 3.0]);
        run_rig(&mut e, &rig);
        let (o0, o1) = (rig.out, rig.out + 0x60);
        // Bone 0 is as without the weight.
        assert_eq!(get(&e, o0), [18.0, 10.0, 12.0, 1.0]);
        // The slerp is asked for (frame copy, same, blended rotation, 0.5) and sees the rotation
        // (1, 0, 0, 1) of the element and the blended (1, 0, 0, 3).
        let log = rig.log.borrow();
        let slerp: Vec<_> = log
            .iter()
            .filter(|(address, _)| *address == QUATERNION_SET_SLERP)
            .collect();
        assert_eq!(slerp.len(), 1);
        let call = &slerp[0].1;
        assert_eq!(call[0], call[1]);
        assert_eq!(call[3], 0.5f32.to_bits());
        assert_eq!(call[4..8], bits([1.0, 0.0, 0.0, 1.0])[..]);
        assert_eq!(call[8..12], bits([1.0, 0.0, 0.0, 3.0])[..]);
        // The rotation became (1, 0, 0, 2) and the translation (0.5 * (11, 2, 8, 1) + 0.5 *
        // (100, 200, 385, 1)): (55.5, 101, 196.5, 1).
        // The element keeps its own temporaries; the finished vectors use the blend.
        assert_eq!(get(&e, o1 + 0x30), [11.0, 2.0, 8.0, 1.0]);
        assert_eq!(get(&e, o1 + 0x40), [1.0, 0.0, 0.0, 1.0]);
        assert_eq!(get(&e, o1 + 0x10), [1.0, 0.0, 0.0, 2.0]);
        assert_eq!(get(&e, o1), [55.5, 97.0, 203.5, 1.0]);
    }

    #[test]
    fn an_empty_list_does_nothing() {
        let mut e = engine();
        let rig = rig(&mut e, &[-1], None, None, &[1.0]);
        e.mem.set_u32(rig.list, 0);
        run_rig(&mut e, &rig);
        assert!(rig.log.borrow().is_empty());
        e.mem.set_u32(rig.list, u32::MAX);
        run_rig(&mut e, &rig);
        assert!(rig.log.borrow().is_empty());
    }

    /// The pieces `applyKeyFrame` works on: bones with a goal position (4, 8, 12, 16) at
    /// `+0x1b0`, goal rotation (2, 2, 2, 2) at `+0x1c0` and the identity at `+0x160`, a target
    /// with room for the state, two control records, the router and a log of what the doubles
    /// of `00d6b510` (which writes the same element for every bone: position (1, 2, 3, 4),
    /// rotation slot (0, 0, 0, 1), temporary translation (2, 2, 2, 2), rotation (0, 0, 0, 1)),
    /// `005616d0` and `00561800` saw.
    struct KeyRig {
        list: u32,
        target: u32,
        controls: u32,
        state: u32,
        router: u32,
        arena: u32,
        bones: Vec<u32>,
        log: Log,
    }

    fn key_rig(
        e: &mut Engine,
        count: u32,
        indices: Option<&[i32]>,
        factors: Option<&[f32]>,
    ) -> KeyRig {
        let list = e.mem.alloc(0x20);
        let bones_array = e.mem.alloc(4 * count);
        e.mem.set_u32(list, count);
        e.mem.set_u32(list + 4, bones_array);
        let mut bones = Vec::new();
        for k in 0..count {
            let bone = e.mem.alloc(0x200);
            e.mem.set_u32(bones_array + 4 * k, bone);
            put(e, bone + 0x160, [0.0, 0.0, 0.0, 1.0]);
            put(e, bone + 0x1b0, [4.0, 8.0, 12.0, 16.0]);
            put(e, bone + 0x1c0, [2.0; 4]);
            bones.push(bone);
        }
        if let Some(indices) = indices {
            let array = e.mem.alloc(4 * count);
            for (k, index) in indices.iter().enumerate() {
                e.mem.set_u32(array + 4 * k as u32, *index as u32);
            }
            e.mem.set_u32(list + 0xc, array);
        }
        if let Some(factors) = factors {
            let array = e.mem.alloc(4 * count);
            for (k, factor) in factors.iter().enumerate() {
                e.mem.set_f32(array + 4 * k as u32, *factor);
            }
            e.mem.set_u32(list + 0x10, array);
        }
        let target = e.mem.alloc(0x40);
        let transforms = e.mem.alloc(0x10);
        let state = e.mem.alloc(0x40 * count);
        e.mem.set_u32(target + 0x30, transforms);
        e.mem.set_u32(target + 0x34, state);
        let controls = e.mem.alloc(0x60);
        let router = e.mem.alloc(8);
        let arena = e.mem.alloc(0x400);
        e.mem.set_u32(router, arena);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let log: Log = Rc::default();
        let l = Rc::clone(&log);
        e.register_double(BONE_TRANSFORMS, move |e, a| {
            let cursor = e.mem.u32(router);
            l.borrow_mut()
                .push((BONE_TRANSFORMS, vec![a[0], a[1], a[2], a[3], a[4], cursor]));
            for k in 0..count {
                let base = a[4] + 0x60 * k;
                put(e, base, [1.0, 2.0, 3.0, 4.0]);
                put(e, base + 0x10, [0.0, 0.0, 0.0, 1.0]);
                put(e, base + 0x30, [2.0; 4]);
                put(e, base + 0x40, [0.0, 0.0, 0.0, 1.0]);
            }
            Ret::default()
        });
        for address in [BONE_SET_POSITION, BONE_SET_ROTATION] {
            let l = Rc::clone(&log);
            e.register_double(address, move |e, a| {
                let mut seen = vec![a[0]];
                seen.extend(bits(get(e, a[1])));
                l.borrow_mut().push((address, seen));
                Ret::default()
            });
        }
        KeyRig {
            list,
            target,
            controls,
            state,
            router,
            arena,
            bones,
            log,
        }
    }

    /// A control record: share `kept` (at `+4`), four gains and the limits.
    fn control(e: &mut Engine, at: u32, kept: f32, gain: f32, limit: f32) {
        for (offset, value) in [
            (4, kept),
            (8, gain),
            (0xc, gain),
            (0x10, gain),
            (0x1c, gain),
            (0x14, limit),
            (0x18, limit),
            (0x20, limit),
            (0x24, limit),
            (0x28, 1.0),
            (0x2c, 1.0),
        ] {
            e.mem.set_f32(at + offset, value);
        }
    }

    #[test]
    fn a_bone_already_on_its_goal_is_not_moved() {
        let mut e = engine();
        let rig = key_rig(&mut e, 1, None, None);
        // Gains of zero, nothing of the old state kept (4: 0 gives a share of 1): the stages
        // end on the goal.
        control(&mut e, rig.controls, 0.0, 0.0, 1000.0);
        e.call(
            0x00d6_b8e0,
            &args![0.5f32, rig.target, rig.list, rig.controls, 0u32],
        );
        let transforms = e.mem.u32(rig.target + 0x30);
        assert_eq!(
            rig.log.borrow()[..],
            [entry(
                BONE_TRANSFORMS,
                &[
                    transforms,
                    rig.list,
                    rig.controls,
                    rig.target,
                    rig.arena,
                    rig.arena + 0x80
                ]
            )]
        );
        // The state: position, speed, the rotation slot and the rotation speed.
        assert_eq!(get(&e, rig.state), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(get(&e, rig.state + 0x20), [2.0, 4.0, 6.0, 8.0]);
        assert_eq!(get(&e, rig.state + 0x10), [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(get(&e, rig.state + 0x30), [0.0; 4]);
        // The router's stack ends where it began.
        assert_eq!(e.mem.u32(rig.router), rig.arena);
    }

    #[test]
    fn the_two_stages_move_the_bone_and_report_the_ratios() {
        let mut e = engine();
        // Bone 0 is skipped; bone 1 uses control record 1 (gains of 2, times its factor 0.5).
        let rig = key_rig(&mut e, 2, Some(&[-1, 1]), Some(&[9.0, 0.5]));
        control(&mut e, rig.controls + 0x30, 0.5, 2.0, 1000.0);
        let errors = e.mem.alloc(8);
        e.mem.set_f32(errors, 99.0);
        e.mem.set_f32(errors + 4, 99.0);
        let l = Rc::clone(&rig.log);
        let mut answers = vec![4.0f32, 3.0];
        e.register_double(LIMIT_RATIO, move |e, a| {
            let mut seen = bits(get(e, a[0]));
            seen.extend([a[1], a[2]]);
            l.borrow_mut().push((LIMIT_RATIO, seen));
            Ret {
                st0: f64::from(answers.pop().unwrap()),
                ..Ret::default()
            }
        });
        e.call(
            0x00d6_b8e0,
            &args![0.5f32, rig.target, rig.list, rig.controls, errors],
        );
        let bone = rig.bones[1];
        let limits = [1000.0f32.to_bits(), 2.0f32.to_bits()];
        let transforms = e.mem.u32(rig.target + 0x30);
        assert_eq!(
            rig.log.borrow()[..],
            [
                entry(
                    BONE_TRANSFORMS,
                    &[
                        transforms,
                        rig.list,
                        rig.controls,
                        rig.target,
                        rig.arena,
                        rig.arena + 0x100
                    ]
                ),
                // The position stage pushes (2, 0, -2, -4) with the limits 1000 and
                // 1 * (1 * 2).
                entry(
                    LIMIT_RATIO,
                    &[bits([2.0, 0.0, -2.0, -4.0]), limits.to_vec()].concat()
                ),
                // The moved position is (4, 4, 4, 4), far from the goal (4, 8, 12, 16).
                entry(BONE_SET_POSITION, &[vec![bone], bits([4.0; 4])].concat()),
                entry(LIMIT_RATIO, &[bits([0.0; 4]), limits.to_vec()].concat()),
                entry(BONE_SET_ROTATION, &[vec![bone], bits([0.0; 4])].concat()),
            ]
        );
        // The first answer is stored, the second added to it; bone 0's entry is untouched.
        assert_eq!(e.mem.f32(errors), 99.0);
        assert_eq!(e.mem.f32(errors + 4), 7.0);
        // The state of bone 1 sits 0x40 further; bone 0's state is untouched.
        assert_eq!(get(&e, rig.state), [0.0; 4]);
        assert_eq!(get(&e, rig.state + 0x40), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(get(&e, rig.state + 0x60), [2.0, 4.0, 6.0, 8.0]);
        assert_eq!(get(&e, rig.state + 0x50), [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(e.mem.u32(rig.router), rig.arena);
    }
}
