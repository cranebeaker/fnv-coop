//! `fallout shared/havok/teshavokutilities.cpp` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Thiscall getter (`00403e20`): the address of the value word at `this + 4`
/// (a zeroed scratch word when `this` is null). The objects passed here sit
/// in the 011cxxxx settings range.
const VALUE_POINTER_GETTER: u32 = 0x0040_3e20;
/// Collision object of a scene node (`0043b610`, cdecl, one word).
const NODE_COLLISION_OBJECT: u32 = 0x0043_b610;
/// Rigid body of a collision object (`006fa820`, thiscall).
const COLLISION_RIGID_BODY: u32 = 0x006f_a820;
/// Child count of a node (`0043b480`, thiscall).
const CHILD_COUNT: u32 = 0x0043_b480;
/// Child at an index (`0043b4a0`, thiscall, one word).
const CHILD_AT: u32 = 0x0043_b4a0;
/// Vector constructor (`006815c0`, thiscall).
const VECTOR_CONSTRUCTOR: u32 = 0x0068_15c0;
/// Owner getter used on collision objects and Havok bodies (`0044ddc0`).
const OWNER_GETTER: u32 = 0x0044_ddc0;
/// Transform constructor (`00476a80`, thiscall).
const TRANSFORM_CONSTRUCTOR: u32 = 0x0047_6a80;
/// Scaled vector `(out, factor, vector)` (`004a3760`, cdecl).
const SCALED_VECTOR: u32 = 0x004a_3760;
/// Vector sum `this.(out, other)` (`00439e90`, thiscall).
const VECTOR_SUM: u32 = 0x0043_9e90;

/// Per-index multipliers read by `AddExplosionImpulse` (`0119acc8`).
const IMPULSE_MULTIPLIERS: u32 = 0x0119_acc8;
/// The float 1.5 the exe keeps at `01016088`.
const ONE_AND_A_HALF: u32 = 0x0101_6088;
/// The float 1000.0 the exe keeps at `01013974`.
const ONE_THOUSAND: u32 = 0x0101_3974;
/// The float 0.55 the exe keeps at `0104e2b4`.
const FIFTY_FIVE_HUNDREDTHS: u32 = 0x0104_e2b4;
/// The float 0.45 the exe keeps at `0104e2b8`.
const FORTY_FIVE_HUNDREDTHS: u32 = 0x0104_e2b8;
/// Default point copied before a collision is found (`011f426c`).
const DEFAULT_POINT_011F426C: u32 = 0x011f_426c;
/// Table the first argument of `0043b300` points to (`012683c8`).
const BODY_TABLE_012683C8: u32 = 0x0126_83c8;

/// Reads the float `00403e20` points to for `object`.
fn value_at(e: &mut Engine, object: u32) -> f32 {
    let pointer = e.call(VALUE_POINTER_GETTER, &args![object]).u32();
    f32::from_bits(e.mem.u32(pointer))
}

/// Copies the float `00403e20` points to for `object` (FLD/FSTP).
fn copy_value_at(e: &mut Engine, object: u32) -> u32 {
    let pointer = e.call(VALUE_POINTER_GETTER, &args![object]).u32();
    x87_quiet_bits(e.mem.u32(pointer))
}

fn copy_words(e: &mut Engine, destination: u32, source: u32, words: u32) {
    for index in 0..words {
        let word = e.mem.u32(source + 4 * index);
        e.mem.set_u32(destination + 4 * index, word);
    }
}

/// The rigid body of a node's collision object, or 0.
fn node_rigid_body(e: &mut Engine, node: Ptr) -> u32 {
    let collision = e.call(NODE_COLLISION_OBJECT, &args![node]).u32();
    if collision == 0 {
        return 0;
    }
    e.call(COLLISION_RIGID_BODY, &args![collision]).u32()
}

/// Number of children of `container` (`0043b480`).
fn child_count(e: &mut Engine, container: u32) -> u32 {
    e.call(CHILD_COUNT, &args![container]).u32()
}

/// Child `index` of `container` (`0043b4a0`).
fn child_at(e: &mut Engine, container: u32, index: u32) -> Ptr {
    Ptr::new(e.call(CHILD_AT, &args![container, index]).u32())
}

// Translated from 0062b4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESHavokUtilities::FindCollidableRef` (Xbox PDB): the reference owning
/// a collidable's scene object (`bhkUtilFunctions::GetNiAVObject`, then
/// `TESObjectREFR::FindReferenceFor3D`, engine map names), or null.
pub fn tes_havok_utilities_find_collidable_ref(e: &mut Engine, collidable: Ptr) -> Ptr {
    if collidable.is_null() {
        return Ptr::new(0);
    }
    let object = e.call(0x00c7_fa90, &args![collidable]).u32();
    if object == 0 {
        return Ptr::new(0);
    }
    Ptr::new(e.call(0x0056_f930, &args![object]).u32())
}

// Translated from 0062b520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scales `value` for `body`: a factor chosen by the code `0043b4d0` reads
/// (code 8 or 0x1d: `011d0574`, 10: `011d0d50`, 0xe: `011d1068`, 0x14:
/// `011d0f30`, else `011d0d2c`); with `limited`, times `v / limit` when the
/// body's `004b5400` value `v` is below the limit at `011d1284`; then times
/// `011d070c`. A null body returns `value` unchanged.
pub fn fn_0062b520(e: &mut Engine, value: f32, body: Ptr, limited: bool) -> f32 {
    if body.is_null() {
        return f32::from_bits(x87_quiet_bits(value.to_bits()));
    }
    let mut factor = 1.0f32;
    let code = e.with_stack(4, |e, temporary| {
        e.call(0x0043_b4f0, &args![body, temporary]);
        e.call(0x0043_b4d0, &args![temporary]).u32()
    });
    let setting = match code.wrapping_sub(8) {
        0 | 0x15 => 0x011d_0574,
        2 => 0x011d_0d50,
        6 => 0x011d_1068,
        0xc => 0x011d_0f30,
        _ => 0x011d_0d2c,
    };
    factor = (factor as f64 * value_at(e, setting) as f64) as f32;
    if limited {
        let amount = e.call(0x004b_5400, &args![body]).f32();
        let limit = value_at(e, 0x011d_1284);
        if amount < limit {
            let limit = value_at(e, 0x011d_1284);
            factor = (amount as f64 / limit as f64 * factor as f64) as f32;
        }
    }
    factor = (factor as f64 * value_at(e, 0x011d_070c) as f64) as f32;
    (factor as f64 * value as f64) as f32
}

// Translation incomplete: 0062b660 - /GS cookie check (00ec408c) on the aligned frame not translated.
// Translated from 0062b660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESHavokUtilities::AddExplosionImpulse` (Xbox PDB): for a node with a
/// rigid body, scales `force` by the body's multiplier (`0119acc8`
/// indexed by `006240b0`; 1.5 for the matching pairs of the source's word
/// +0x10 and that index), builds the impulse velocity and sets it on the
/// Havok body (`005616d0`); then recurses into the node's children.
/// open: the stack cookie check is not modelled.
pub fn tes_havok_utilities_add_explosion_impulse(
    e: &mut Engine,
    node: Ptr,
    origin: Ptr,
    force: f32,
    source: Ptr,
) {
    if node.is_null() {
        return;
    }
    let body = node_rigid_body(e, node);
    if body != 0 {
        e.with_stack(0x60, |e, frame| {
            let at = |offset: u32| frame.addr() + 0x60 - offset;
            let temporary = at(0x10);
            let velocity = at(0x30);
            let impulse = at(0x50);
            let layer = e.call(0x0043_b4f0, &args![body, temporary]).u32();
            let index = e.call(0x0062_40b0, &args![layer]).u32();
            let mut multiplier = x87_quiet_bits(
                e.mem
                    .u32(IMPULSE_MULTIPLIERS.wrapping_add(index.wrapping_mul(4))),
            );
            if !source.is_null() {
                let kind = e.mem.u32(source.addr() + 0x10) as i32;
                if kind != -1 {
                    let boosted = (matches!(kind, 1 | 2) && index == 1)
                        || (matches!(kind, 3 | 4) && matches!(index, 5..=7))
                        || (matches!(kind, 5 | 6) && matches!(index, 0xb..=0xd))
                        || (matches!(kind, 7..=9) && matches!(index, 8..=10))
                        || (matches!(kind, 10..=12) && matches!(index, 0xe..=0x10));
                    if boosted {
                        multiplier = x87_quiet_bits(e.global::<u32>(ONE_AND_A_HALF));
                    }
                }
            }
            let scaled = (force as f64 * f32::from_bits(multiplier) as f64) as f32;
            e.call(VECTOR_CONSTRUCTOR, &args![velocity]);
            e.vcall(body, 0xf0, &args![velocity]);
            e.call(0x0096_1190, &args![velocity, origin]);
            e.call(0x0062_aea0, &args![velocity]);
            let havok_body = e.call(0x0062_0b80, &args![body]).u32();
            e.call(
                0x005d_bf20,
                &args![impulse, x87_quiet_bits(scaled.to_bits())],
            );
            e.call(0x0062_7920, &args![velocity, impulse]);
            let mass = e.call(0x0056_0dc0, &args![havok_body]).u32();
            e.call(0x0055_3f70, &args![velocity, mass]);
            e.call(0x0056_16d0, &args![havok_body, velocity]);
        });
    }
    let children = e.vcall(node.addr(), 0xc, &[]).u32();
    if children != 0 {
        let mut index = 0;
        while index < child_count(e, children) {
            let child = child_at(e, children, index);
            let force = f32::from_bits(x87_quiet_bits(force.to_bits()));
            tes_havok_utilities_add_explosion_impulse(e, child, origin, force, source);
            index += 1;
        }
    }
}

// Translation incomplete: 0062b8d0 - /GS cookie check (00ec408c) on the aligned frame not translated.
// Translated from 0062b8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESHavokUtilities::AddVelocity` (Xbox PDB): converts `velocity` into a
/// stack vector (`004a3e00`) and applies it through `0062b930`.
/// open: the stack cookie check is not modelled.
pub fn tes_havok_utilities_add_velocity(e: &mut Engine, node: Ptr, velocity: Ptr, keep: bool) {
    e.with_stack(0x10, |e, vector| {
        e.call(VECTOR_CONSTRUCTOR, &args![vector]);
        e.call(0x004a_3e00, &args![vector, velocity]);
        fn_0062b930(e, node, vector, keep);
    });
}

/// Builds `(x, y, z, 0)` from the three components `00560d30` returns for
/// `source` (read in the order 2, 1, 0) into `vector` (`004b4d50`).
fn set_vector_from(e: &mut Engine, vector: u32, source: Ptr) {
    let mut components = [0u32; 3];
    for component in [2u32, 1, 0] {
        let pointer = e.call(0x0056_0d30, &args![source, component]).u32();
        components[component as usize] = x87_quiet_bits(e.mem.u32(pointer));
    }
    e.call(
        0x004b_4d50,
        &args![
            vector,
            components[0],
            components[1],
            components[2],
            0.0f32.to_bits()
        ],
    );
}

// Translation incomplete: 0062b930 - /GS cookie check (00ec408c) on the aligned frame not translated.
// Translated from 0062b930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `vector` as linear velocity to every rigid body under `node` whose
/// `00517630` word passes `00c8cce0`; without `keep` the vector first goes
/// through `00553f70` with the body's `00560d80` value. Recurses into the
/// children. open: the stack cookie check is not modelled.
pub fn fn_0062b930(e: &mut Engine, node: Ptr, vector: Ptr, keep: bool) {
    if node.is_null() {
        return;
    }
    let body = node_rigid_body(e, node);
    if body != 0 {
        e.with_stack(0x20, |e, frame| {
            let handle = frame.addr() + 0x10;
            let velocity = frame.addr();
            let word = e.call(0x0051_7630, &args![body]).u32();
            e.mem.set_u32(handle, word);
            if e.call(0x00c8_cce0, &args![handle]).bool() {
                e.call(VECTOR_CONSTRUCTOR, &args![velocity]);
                set_vector_from(e, velocity, vector);
                if !keep {
                    let value = e.call(0x0056_0d80, &args![body]).u32();
                    e.call(0x0055_3f70, &args![velocity, value]);
                }
                e.call(0x0056_1690, &args![body, velocity]);
            }
        });
    }
    let children = e.vcall(node.addr(), 0xc, &[]).u32();
    if children != 0 {
        let mut index = 0;
        while index < child_count(e, children) {
            let child = child_at(e, children, index);
            fn_0062b930(e, child, vector, keep);
            index += 1;
        }
    }
}

// Translation incomplete: 0062ba80 - /GS cookie check (00ec408c) on the aligned frame not translated.
// Translated from 0062ba80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Converts `vector` into a stack vector (`004a3e00`) and applies it
/// through `0062bae0`. open: the stack cookie check is not modelled.
pub fn fn_0062ba80(e: &mut Engine, node: Ptr, vector: Ptr, keep: bool) {
    e.with_stack(0x10, |e, converted| {
        e.call(VECTOR_CONSTRUCTOR, &args![converted]);
        e.call(0x004a_3e00, &args![converted, vector]);
        fn_0062bae0(e, node, converted, keep);
    });
}

// Translation incomplete: 0062bae0 - /GS cookie check (00ec408c) on the aligned frame not translated.
// Translated from 0062bae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies `vector` to every rigid body under `node` through `005617c0`;
/// without `keep` it first goes through `00553f70` with the body's
/// `00560e10` value. Recurses into the children.
/// open: the stack cookie check is not modelled.
pub fn fn_0062bae0(e: &mut Engine, node: Ptr, vector: Ptr, keep: bool) {
    if node.is_null() {
        return;
    }
    let body = node_rigid_body(e, node);
    if body != 0 {
        e.with_stack(0x10, |e, velocity| {
            e.call(VECTOR_CONSTRUCTOR, &args![velocity]);
            set_vector_from(e, velocity.addr(), vector);
            if !keep {
                let value = e.call(0x0056_0e10, &args![body]).u32();
                e.call(0x0055_3f70, &args![velocity, value]);
            }
            e.call(0x0056_17c0, &args![body, velocity]);
        });
    }
    let children = e.vcall(node.addr(), 0xc, &[]).u32();
    if children != 0 {
        let mut index = 0;
        while index < child_count(e, children) {
            let child = child_at(e, children, index);
            fn_0062bae0(e, child, vector, keep);
            index += 1;
        }
    }
}

// Translated from 0062bc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESHavokUtilities::RemoveHavokFromSceneGraph` (Xbox PDB): clears the
/// collision object of `node` (when `006838b0` finds one) and of every
/// descendant. The children are counted and read on `node` itself once the
/// virtual at +0xc returns non-zero.
pub fn tes_havok_utilities_remove_havok_from_scene_graph(e: &mut Engine, node: Ptr) {
    if node.is_null() {
        return;
    }
    if e.call(0x0068_38b0, &args![node]).u32() != 0 {
        ni_av_object_set_collision_object(e, node, Ptr::new(0));
    }
    if e.vcall(node.addr(), 0xc, &[]).u32() != 0 {
        let mut index = 0;
        while index < child_count(e, node.addr()) {
            let child = child_at(e, node.addr(), index);
            tes_havok_utilities_remove_havok_from_scene_graph(e, child);
            index += 1;
        }
    }
}

// Translated from 0062bc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::SetCollisionObject` (Xbox PDB): stores the pointer at +0x1c
/// (`0066b0d0`); when the stored object's owner (`0044ddc0`) is not `this`,
/// tells it its owner through its virtual at +0x8c.
pub fn ni_av_object_set_collision_object(e: &mut Engine, this: Ptr, collision: Ptr) {
    let slot = this.addr() + 0x1c;
    e.call(0x0066_b0d0, &args![slot, collision]);
    if e.call(0x0055_9450, &args![slot]).u32() != 0 {
        let object = e.call(0x0055_9450, &args![slot]).u32();
        if e.call(OWNER_GETTER, &args![object]).u32() != this.addr() {
            let object = e.call(0x0055_9450, &args![slot]).u32();
            e.vcall(object, 0x8c, &args![this]);
        }
    }
}

// Translated from 0062bd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESHavokUtilities::InspectHavokObjects` (Xbox PDB): for each rigid body
/// under `node`, adds a count from its Havok object by that object's kind
/// (`0043b230`): kind 0x17 and kind 10 (through `0062be70`) add to
/// `*second`, kind 6 adds to `*first`. The children are read on `node`.
pub fn tes_havok_utilities_inspect_havok_objects(
    e: &mut Engine,
    node: Ptr,
    first: Ptr,
    second: Ptr,
) {
    if node.is_null() {
        return;
    }
    let body = node_rigid_body(e, node);
    if body != 0 {
        let holder = e.call(0x004a_e6a0, &args![body]).u32();
        let object = if holder != 0 {
            e.call(0x0062_0b80, &args![holder]).u32()
        } else {
            0
        };
        if object != 0 {
            if e.call(0x0043_b230, &args![object]).u32() == 0x17 {
                let count = e.vcall(object + 0x10, 4, &[]).u32();
                let total = count.wrapping_add(e.mem.u32(second.addr()));
                e.mem.set_u32(second.addr(), total);
            } else if e.call(0x0043_b230, &args![object]).u32() == 6 {
                let count = e.vcall(object, 0x2c, &[]).u32();
                let total = count.wrapping_add(e.mem.u32(first.addr()));
                e.mem.set_u32(first.addr(), total);
            } else if e.call(0x0043_b230, &args![object]).u32() == 10 {
                let inner = fn_0062be70(e, Ptr::new(object));
                if inner != 0 {
                    let count = e.vcall(inner + 0x10, 4, &[]).u32();
                    let total = count.wrapping_add(e.mem.u32(second.addr()));
                    e.mem.set_u32(second.addr(), total);
                }
            }
        }
    }
    if e.vcall(node.addr(), 0xc, &[]).u32() != 0 {
        let mut index = 0;
        while index < child_count(e, node.addr()) {
            let child = child_at(e, node.addr(), index);
            tes_havok_utilities_inspect_havok_objects(e, child, first, second);
            index += 1;
        }
    }
}

// Translated from 0062be70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns `00726070` of the member at `this + 0x30`.
pub fn fn_0062be70(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x0072_6070, &args![this.addr() + 0x30]).u32()
}

// Translated from 0062be90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Piecewise scale: `scale * (b * slope + offset)` when `b` is above a
/// threshold, else 0. `scale` and threshold come from three bands of `a`
/// (limits `011cfb38`, `011ce714`, `011cfa88`); below the first limit the
/// scale is 0 (threshold 1000.0). Slope `011ce984`, offset `011cf2fc`.
pub fn fn_0062be90(e: &mut Engine, a: f32, b: f32) -> f32 {
    let initial_threshold = f32::from_bits(x87_quiet_bits(e.global::<u32>(ONE_THOUSAND)));
    // Each band test is FCOMPP with the limit in ST0: taken only when the
    // limit is strictly above `a` (not on equality or NaN).
    let (scale_bits, threshold) = if value_at(e, 0x011c_fb38) > a {
        (0.0f32.to_bits(), initial_threshold)
    } else if value_at(e, 0x011c_e714) > a {
        let scale = copy_value_at(e, 0x011c_f600);
        (scale, f32::from_bits(copy_value_at(e, 0x011c_fd64)))
    } else if value_at(e, 0x011c_fa88) > a {
        let scale = copy_value_at(e, 0x011c_f92c);
        (scale, f32::from_bits(copy_value_at(e, 0x011c_e154)))
    } else {
        let scale = copy_value_at(e, 0x011c_e978);
        (scale, f32::from_bits(copy_value_at(e, 0x011c_e7a4)))
    };
    let scale = f32::from_bits(scale_bits);
    // TEST AH,5 / JP: computed only when the threshold is below `b`
    // (ordered); equality and NaN give 0.
    let level = if threshold < b {
        let offset = value_at(e, 0x011c_f2fc);
        let slope = value_at(e, 0x011c_e984);
        (b as f64 * slope as f64 + offset as f64) as f32
    } else {
        0.0
    };
    (scale as f64 * level as f64) as f32
}

// Translation incomplete: 0062bfb0 - /GS cookie check (00ec408c) on the aligned frame not translated.
// Translated from 0062bfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESHavokUtilities::GetLocalPointWithinCollision` (Xbox PDB): picks a
/// random point between 45% and 55% of the first collision's local box
/// (`00476b70` per axis) and, unless the body passes neither the
/// `012683c8` test nor belongs to another owner than `node`, moves it
/// through the body's transform into `node`'s inverted world transform.
/// Writes the point (default `011f426c`) to `out` and returns `out`.
/// open: the stack cookie check is not modelled.
pub fn tes_havok_utilities_get_local_point_within_collision(
    e: &mut Engine,
    out: Ptr,
    node: Ptr,
) -> Ptr {
    e.with_stack(0x1a0, |e, frame| {
        let at = |offset: u32| frame.addr() + 0x1a0 - offset;
        let point = at(0x10);
        copy_words(e, point, DEFAULT_POINT_011F426C, 3);
        let collision = e.call(0x004b_5260, &args![node]).u32();
        let body = if collision != 0 {
            e.call(COLLISION_RIGID_BODY, &args![collision]).u32()
        } else {
            0
        };
        if body != 0 {
            let aabb = at(0x40);
            e.call(0x0043_7c10, &args![aabb]);
            e.call(0x00c8_d390, &args![body, aabb]);
            let low = at(0x4c);
            e.call(VECTOR_CONSTRUCTOR, &args![low]);
            e.call(0x0045_8620, &args![low, aabb]);
            let high = at(0x58);
            e.call(VECTOR_CONSTRUCTOR, &args![high]);
            e.call(0x0045_8620, &args![high, at(0x30)]);
            let near_low = x87_quiet_bits(e.global::<u32>(FORTY_FIVE_HUNDREDTHS));
            let first = e.call(SCALED_VECTOR, &args![at(0x64), near_low, low]).u32();
            let near_high = x87_quiet_bits(e.global::<u32>(FIFTY_FIVE_HUNDREDTHS));
            let second = e
                .call(SCALED_VECTOR, &args![at(0x7c), near_high, high])
                .u32();
            let lower = at(0x70);
            e.call(VECTOR_SUM, &args![second, lower, first]);
            let far_low = x87_quiet_bits(e.global::<u32>(FIFTY_FIVE_HUNDREDTHS));
            let third = e.call(SCALED_VECTOR, &args![at(0x88), far_low, low]).u32();
            let far_high = x87_quiet_bits(e.global::<u32>(FORTY_FIVE_HUNDREDTHS));
            let fourth = e
                .call(SCALED_VECTOR, &args![at(0xa0), far_high, high])
                .u32();
            let upper = at(0x94);
            e.call(VECTOR_SUM, &args![fourth, upper, third]);
            let mut axes = [0u32; 3];
            for axis in [2u32, 1, 0] {
                let from_upper = x87_quiet_bits(e.mem.u32(upper + 4 * axis));
                let from_lower = x87_quiet_bits(e.mem.u32(lower + 4 * axis));
                let value = e.call(0x0047_6b70, &args![from_upper, from_lower]).f32();
                axes[axis as usize] = x87_quiet_bits(value.to_bits());
            }
            let random = e
                .call(0x0041_6870, &args![at(0xac), axes[0], axes[1], axes[2]])
                .u32();
            copy_words(e, point, random, 3);
            let transform_point = e
                .call(0x0043_b300, &args![BODY_TABLE_012683C8, body])
                .bool()
                || e.call(OWNER_GETTER, &args![collision]).u32() != node.addr();
            if transform_point {
                let body_transform = at(0xf0);
                e.call(0x0056_ded0, &args![body_transform]);
                e.call(0x00c8_d280, &args![body, body_transform]);
                let local = at(0x124);
                e.call(TRANSFORM_CONSTRUCTOR, &args![local]);
                e.call(0x0062_4ad0, &args![local, body_transform]);
                e.mem.set_u32(at(0xf4), 1.0f32.to_bits());
                let inverse = at(0x158);
                e.call(TRANSFORM_CONSTRUCTOR, &args![inverse]);
                let world = e.call(0x0046_1130, &args![node]).u32();
                e.call(0x004b_4880, &args![world, inverse]);
                let combined =
                    fn_0062c250(e, Ptr::new(inverse), Ptr::new(at(0x198)), Ptr::new(local));
                let moved = e
                    .call(0x0052_4c40, &args![combined, at(0x164), point])
                    .u32();
                copy_words(e, point, moved, 3);
            }
        }
        copy_words(e, out.addr(), point, 3);
    });
    out
}

// Translated from 0062c250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Transform product `this * other` into `out` (13 words): rotation
/// `0043f8d0`, translation `this.t + this.s * (this.R * other.t)`
/// (`004b4500`, `004a3760`, `00439e90`), scale `this.s * other.s`.
pub fn fn_0062c250(e: &mut Engine, this: Ptr, out: Ptr, other: Ptr) -> Ptr {
    e.with_stack(0x80, |e, frame| {
        let at = |offset: u32| frame.addr() + 0x80 - offset;
        let product = at(0x34);
        e.call(TRANSFORM_CONSTRUCTOR, &args![product]);
        let own_scale = f32::from_bits(e.mem.u32(this.addr() + 0x30));
        let other_scale = f32::from_bits(e.mem.u32(other.addr() + 0x30));
        let scale = (own_scale as f64 * other_scale as f64) as f32;
        e.mem.set_u32(product + 0x30, scale.to_bits());
        let rotation = e.call(0x0043_f8d0, &args![this, at(0x58), other]).u32();
        copy_words(e, product, rotation, 9);
        let rotated = e
            .call(0x004b_4500, &args![this, at(0x64), other.addr() + 0x24])
            .u32();
        let factor = x87_quiet_bits(e.mem.u32(this.addr() + 0x30));
        let scaled = e
            .call(SCALED_VECTOR, &args![at(0x70), factor, rotated])
            .u32();
        let moved = e
            .call(VECTOR_SUM, &args![this.addr() + 0x24, at(0x7c), scaled])
            .u32();
        copy_words(e, product + 0x24, moved, 3);
        copy_words(e, out.addr(), product, 13);
    });
    out
}

// Translated from 0062c300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sums the `004b5400` value of every rigid body under `node` (in float).
pub fn fn_0062c300(e: &mut Engine, node: Ptr) -> f32 {
    let mut total = 0.0f32;
    if node.is_null() {
        return total;
    }
    let body = node_rigid_body(e, node);
    if body != 0 {
        let value = e.call(0x004b_5400, &args![body]).f64();
        total = (value + total as f64) as f32;
    }
    let children = e.vcall(node.addr(), 0xc, &[]).u32();
    if children != 0 && child_count(e, children) != 0 {
        let mut index = 0;
        while index < child_count(e, children) {
            let child = child_at(e, children, index);
            let value = fn_0062c300(e, child);
            total = (value as f64 + total as f64) as f32;
            index += 1;
        }
    }
    total
}

// Translated from 0062c3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESHavokUtilities::Should3DRagDoll` (Xbox PDB): true when `reference`
/// passes its virtual +0x100 and either +0x22c (with 0) or +0x230, else
/// when `BSXFlags::Find` (engine map name of `00624b70`) on `node` with
/// 0x48 returns 0x48.
pub fn tes_havok_utilities_should_3d_rag_doll(e: &mut Engine, reference: Ptr, node: Ptr) -> bool {
    if !reference.is_null()
        && e.vcall(reference.addr(), 0x100, &[]).bool()
        && (e.vcall(reference.addr(), 0x22c, &args![0u32]).bool()
            || e.vcall(reference.addr(), 0x230, &[]).bool())
    {
        return true;
    }
    e.call(0x0062_4b70, &args![node, 0x48u32]).u32() == 0x48
}

// Translation incomplete: 0062c430 - FS:[0] exception frame (unwind of the body array) not translated.
// Translated from 0062c430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESHavokUtilities::ConsolidateSimIslands` (Xbox PDB): gathers the
/// reference's rigid bodies (ragdoll controller at +0xac, `00c7be20`, else
/// `00c80930` on its virtual +0x1d0) and, in the first body's world, merges
/// the island of every other body of that world into the first one's
/// (`00cb5570`), between two `00483710` calls on the world.
/// open: the exception frame's unwind state is not modelled.
pub fn tes_havok_utilities_consolidate_sim_islands(e: &mut Engine, reference: Ptr) {
    if reference.is_null() {
        return;
    }
    e.with_stack(12, |e, bodies| {
        e.call(0x0062_99a0, &args![bodies]);
        let controller = e.mem.u32(reference.addr() + 0xac);
        if controller != 0 {
            e.call(0x00c7_be20, &args![controller, bodies]);
        } else {
            let root = e.vcall(reference.addr(), 0x1d0, &[]).u32();
            e.call(0x00c8_0930, &args![root, bodies]);
        }
        let body_at = |e: &mut Engine, index: u32| {
            let slot = e.call(0x0062_9ab0, &args![bodies, index]).u32();
            e.mem.u32(slot)
        };
        if e.call(0x0072_6070, &args![bodies]).u32() != 0 {
            let first = body_at(e, 0);
            let world = e.call(OWNER_GETTER, &args![first]).u32();
            if world != 0 {
                e.call(0x0048_3710, &args![world]);
                let mut index = 1i32;
                while index < e.call(0x0072_6070, &args![bodies]).i32() {
                    let first = body_at(e, 0);
                    let first_island = e.call(0x009a_d610, &args![first]).u32();
                    let other = body_at(e, index as u32);
                    let other_island = e.call(0x009a_d610, &args![other]).u32();
                    let other = body_at(e, index as u32);
                    let other_world = e.call(OWNER_GETTER, &args![other]).u32();
                    if world == other_world && first_island != other_island {
                        let other = body_at(e, index as u32);
                        let first = body_at(e, 0);
                        e.call(0x00cb_5570, &args![world, first, other]);
                    }
                    index += 1;
                }
                e.call(0x0048_3710, &args![world]);
            }
        }
        e.call(0x0062_9b40, &args![bodies]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0062b4e0, tes_havok_utilities_find_collidable_ref(Ptr) -> Ptr),
        entry!(0x0062b520, fn_0062b520(f32, Ptr, bool) -> f32),
        entry!(
            0x0062b660,
            tes_havok_utilities_add_explosion_impulse(Ptr, Ptr, f32, Ptr)
        ),
        entry!(0x0062b8d0, tes_havok_utilities_add_velocity(Ptr, Ptr, bool)),
        entry!(0x0062b930, fn_0062b930(Ptr, Ptr, bool)),
        entry!(0x0062ba80, fn_0062ba80(Ptr, Ptr, bool)),
        entry!(0x0062bae0, fn_0062bae0(Ptr, Ptr, bool)),
        entry!(
            0x0062bc10,
            tes_havok_utilities_remove_havok_from_scene_graph(Ptr)
        ),
        entry!(0x0062bc90, ni_av_object_set_collision_object(Ptr, Ptr)),
        entry!(
            0x0062bd00,
            tes_havok_utilities_inspect_havok_objects(Ptr, Ptr, Ptr)
        ),
        entry!(0x0062be70, fn_0062be70(Ptr) -> u32),
        entry!(0x0062be90, fn_0062be90(f32, f32) -> f32),
        entry!(
            0x0062bfb0,
            tes_havok_utilities_get_local_point_within_collision(Ptr, Ptr) -> Ptr
        ),
        entry!(0x0062c250, fn_0062c250(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x0062c300, fn_0062c300(Ptr) -> f32),
        entry!(
            0x0062c3c0,
            tes_havok_utilities_should_3d_rag_doll(Ptr, Ptr) -> bool
        ),
        entry!(0x0062c430, tes_havok_utilities_consolidate_sim_islands(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const VTABLE: u32 = 0x0030_0000;
    const SLOT_CHILDREN: u32 = 0x0030_1000;
    const SLOT_NOOP: u32 = 0x0030_1004;
    const SLOT_WORD_4: u32 = 0x0030_1008;
    const SLOT_WORD_8: u32 = 0x0030_100c;
    const SLOT_WORD_C: u32 = 0x0030_1010;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn noop(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn bits(value: f32) -> u32 {
        value.to_bits()
    }

    /// A shared vtable: slot 0xc returns the object's word +4 (its child
    /// container), every other slot returns 0 unless a test points it
    /// elsewhere. Child count is the container's word +8, child `i` its
    /// word `+0xc + 4 * i`; a node's collision object is its word +0x20 and
    /// a collision object's rigid body is its first word.
    fn scene(e: &mut Engine) {
        e.put_vtable(VTABLE, &[SLOT_NOOP; 0x100]);
        e.mem.set_u32(VTABLE + 0xc, SLOT_CHILDREN);
        e.register(SLOT_CHILDREN, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(SLOT_NOOP, |_, _| Ret::default());
        e.register(SLOT_WORD_4, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(SLOT_WORD_8, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(SLOT_WORD_C, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(CHILD_COUNT, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(CHILD_AT, |e, a| ret(e.mem.u32(a[0] + 0xc + 4 * a[1])));
        e.register(NODE_COLLISION_OBJECT, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        e.register(COLLISION_RIGID_BODY, |e, a| ret(e.mem.u32(a[0])));
    }

    fn object(e: &mut Engine) -> u32 {
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, VTABLE);
        object
    }

    /// A node whose children (if any) are listed on itself, with a rigid
    /// body when `body` is non-zero.
    fn node(e: &mut Engine, body: u32, children: &[u32]) -> Ptr {
        let node = object(e);
        if !children.is_empty() {
            e.mem.set_u32(node + 4, node);
        }
        e.mem.set_u32(node + 8, children.len() as u32);
        for (index, &child) in children.iter().enumerate() {
            e.mem.set_u32(node + 0xc + 4 * index as u32, child);
        }
        if body != 0 {
            let collision = e.mem.alloc(4);
            e.mem.set_u32(collision, body);
            e.mem.set_u32(node + 0x20, collision);
        }
        Ptr::new(node)
    }

    fn values(e: &mut Engine, settings: &[(u32, f32)]) {
        e.register(VALUE_POINTER_GETTER, |_, a| ret(a[0] + 4));
        for &(setting, value) in settings {
            e.map(setting, 8);
            e.mem.set_u32(setting + 4, bits(value));
        }
    }

    #[test]
    fn find_collidable_ref_skips_null_and_missing_object() {
        let mut e = Engine::new();
        e.register(0x00c7_fa90, |_, a| ret(if a[0] == 0x10 { 0x20 } else { 0 }));
        e.register(0x0056_f930, |_, a| ret(a[0] + 1));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_b4e0, &args![0u32]).u32(), 0);
        assert!(calls(&e, 0x00c7_fa90).is_empty());
        assert_eq!(e.call(0x0062_b4e0, &args![0x11u32]).u32(), 0);
        assert_eq!(e.call(0x0062_b4e0, &args![0x10u32]).u32(), 0x21);
    }

    #[test]
    fn body_factor_uses_code_table_and_limit() {
        let mut e = Engine::new();
        values(
            &mut e,
            &[
                (0x011d_0574, 2.0),
                (0x011d_0d2c, 4.0),
                (0x011d_1284, 10.0),
                (0x011d_070c, 3.0),
            ],
        );
        e.register(0x0043_b4f0, |e, a| {
            let code = e.mem.u32(a[0]);
            e.mem.set_u32(a[1], code);
            Ret::default()
        });
        e.register(0x0043_b4d0, |e, a| ret(e.mem.u32(a[0])));
        e.register(0x004b_5400, |_, _| Ret {
            st0: 5.0,
            ..Ret::default()
        });
        let body = e.mem.alloc(4);
        e.mem.set_u32(body, 0x1d);
        assert_eq!(e.call(0x0062_b520, &args![2.0f32, body, true]).f32(), 6.0);
        assert_eq!(e.call(0x0062_b520, &args![2.0f32, body, false]).f32(), 12.0);
        e.mem.set_u32(body, 9);
        assert_eq!(e.call(0x0062_b520, &args![1.0f32, body, false]).f32(), 12.0);
        // Code 7 wraps below 8 and takes the default too.
        e.mem.set_u32(body, 7);
        assert_eq!(e.call(0x0062_b520, &args![1.0f32, body, false]).f32(), 12.0);
        let quiet = fn_0062b520(&mut e, f32::from_bits(0x7f80_0001), Ptr::new(0), false);
        assert_eq!(quiet.to_bits(), 0x7fc0_0001);
    }

    fn impulse_doubles(e: &mut Engine) {
        e.map(IMPULSE_MULTIPLIERS, 0x40);
        e.mem.set_u32(IMPULSE_MULTIPLIERS + 4, bits(2.0));
        e.map(ONE_AND_A_HALF, 4);
        e.set_global(ONE_AND_A_HALF, bits(1.5));
        e.register(0x0043_b4f0, |_, _| ret(0x55));
        e.register(0x0062_40b0, |_, _| ret(1));
        e.register(0x0062_0b80, |_, _| ret(0x777));
        e.register(0x0056_0dc0, |_, _| ret(0x888));
        noop(
            e,
            &[
                VECTOR_CONSTRUCTOR,
                0x0096_1190,
                0x0062_aea0,
                0x005d_bf20,
                0x0062_7920,
                0x0055_3f70,
                0x0056_16d0,
            ],
        );
    }

    #[test]
    fn explosion_impulse_boosts_matching_kind_and_recurses() {
        let mut e = Engine::new();
        scene(&mut e);
        impulse_doubles(&mut e);
        let body = object(&mut e);
        let child = node(&mut e, 0, &[]);
        let root = node(&mut e, body, &[child.addr()]);
        let source = e.mem.alloc(0x14);
        e.mem.set_u32(source + 0x10, 2);
        e.call_log = Some(vec![]);
        e.call(0x0062_b660, &args![root, 0x99u32, 4.0f32, source]);
        let scaled = calls(&e, 0x005d_bf20);
        assert_eq!(scaled.len(), 1);
        assert_eq!(scaled[0][1], bits(6.0));
        assert_eq!(calls(&e, 0x0096_1190)[0][1], 0x99);
        assert_eq!(calls(&e, 0x0055_3f70)[0][1], 0x888);
        assert_eq!(calls(&e, 0x0056_16d0)[0][0], 0x777);
        assert_eq!(calls(&e, NODE_COLLISION_OBJECT).len(), 2);

        e.mem.set_u32(source + 0x10, u32::MAX);
        e.call_log = Some(vec![]);
        e.call(0x0062_b660, &args![root, 0x99u32, 4.0f32, source]);
        assert_eq!(calls(&e, 0x005d_bf20)[0][1], bits(8.0));
        e.mem.set_u32(source + 0x10, 3);
        e.call_log = Some(vec![]);
        e.call(0x0062_b660, &args![root, 0x99u32, 4.0f32, 0u32]);
        assert_eq!(calls(&e, 0x005d_bf20)[0][1], bits(8.0));
    }

    #[test]
    fn add_velocity_converts_then_applies() {
        let mut e = Engine::new();
        noop(&mut e, &[VECTOR_CONSTRUCTOR, 0x004a_3e00]);
        e.call_log = Some(vec![]);
        e.call(0x0062_b8d0, &args![0u32, 0x44u32, true]);
        let convert = calls(&e, 0x004a_3e00);
        assert_eq!(convert[0][1], 0x44);
        assert_eq!(calls(&e, VECTOR_CONSTRUCTOR)[0][0], convert[0][0]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 3);
    }

    fn velocity_doubles(e: &mut Engine) -> u32 {
        let vector = e.mem.alloc(16);
        e.mem.set_u32(vector, bits(1.0));
        e.mem.set_u32(vector + 4, bits(2.0));
        e.mem.set_u32(vector + 8, 0x7f80_0001);
        e.register(0x0056_0d30, |_, a| ret(a[0] + 4 * a[1]));
        e.register(0x0051_7630, |_, _| ret(7));
        e.register(0x00c8_cce0, |e, a| ret((e.mem.u32(a[0]) == 7) as u32));
        e.register(0x0056_0d80, |_, _| ret(0x31));
        e.register(0x0056_0e10, |_, _| ret(0x32));
        noop(
            e,
            &[
                VECTOR_CONSTRUCTOR,
                0x004b_4d50,
                0x0055_3f70,
                0x0056_1690,
                0x0056_17c0,
                0x004a_3e00,
            ],
        );
        vector
    }

    #[test]
    fn linear_velocity_builds_vector_and_optionally_scales() {
        let mut e = Engine::new();
        scene(&mut e);
        let vector = velocity_doubles(&mut e);
        let body = object(&mut e);
        let root = node(&mut e, body, &[]);
        e.call_log = Some(vec![]);
        e.call(0x0062_b930, &args![root, vector, false]);
        let built = calls(&e, 0x004b_4d50);
        assert_eq!(built[0][1..], [bits(1.0), bits(2.0), 0x7fc0_0001, 0]);
        let order: Vec<u32> = calls(&e, 0x0056_0d30).iter().map(|a| a[1]).collect();
        assert_eq!(order, vec![2, 1, 0]);
        assert_eq!(calls(&e, 0x0055_3f70)[0][1], 0x31);
        assert_eq!(calls(&e, 0x0056_1690), vec![vec![body, built[0][0]]]);
        e.call_log = Some(vec![]);
        e.call(0x0062_b930, &args![root, vector, true]);
        assert!(calls(&e, 0x0055_3f70).is_empty());
        e.register(0x0051_7630, |_, _| ret(8));
        e.call_log = Some(vec![]);
        e.call(0x0062_b930, &args![root, vector, false]);
        assert!(calls(&e, 0x0056_1690).is_empty());
    }

    #[test]
    fn converted_velocity_wrapper_applies_through_second_path() {
        let mut e = Engine::new();
        noop(&mut e, &[VECTOR_CONSTRUCTOR, 0x004a_3e00]);
        e.call_log = Some(vec![]);
        e.call(0x0062_ba80, &args![0u32, 0x45u32, false]);
        assert_eq!(calls(&e, 0x004a_3e00)[0][1], 0x45);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 3);
    }

    #[test]
    fn second_velocity_path_has_no_handle_test() {
        let mut e = Engine::new();
        scene(&mut e);
        let vector = velocity_doubles(&mut e);
        let body = object(&mut e);
        let child = node(&mut e, body, &[]);
        let root = node(&mut e, 0, &[child.addr()]);
        e.call_log = Some(vec![]);
        e.call(0x0062_bae0, &args![root, vector, false]);
        assert!(calls(&e, 0x0051_7630).is_empty());
        assert_eq!(calls(&e, 0x0055_3f70)[0][1], 0x32);
        assert_eq!(calls(&e, 0x0056_17c0).len(), 1);
        assert_eq!(calls(&e, 0x0056_17c0)[0][0], body);
    }

    #[test]
    fn remove_havok_iterates_children_of_node_itself() {
        let mut e = Engine::new();
        scene(&mut e);
        let child = node(&mut e, 0, &[]);
        let root = node(&mut e, 0, &[child.addr()]);
        // The virtual's result only gates the loop.
        e.mem.set_u32(root.addr() + 4, 1);
        e.register_double(0x0068_38b0, move |_, a| ret((a[0] == root.addr()) as u32));
        e.register(0x0055_9450, |_, _| ret(0));
        noop(&mut e, &[0x0066_b0d0]);
        e.call_log = Some(vec![]);
        e.call(0x0062_bc10, &args![root]);
        assert_eq!(calls(&e, 0x0066_b0d0), vec![vec![root.addr() + 0x1c, 0]]);
        assert_eq!(calls(&e, CHILD_AT), vec![vec![root.addr(), 0]]);
        assert_eq!(calls(&e, 0x0068_38b0).len(), 2);
    }

    #[test]
    fn set_collision_object_tells_object_its_owner() {
        let mut e = Engine::new();
        scene(&mut e);
        let this = object(&mut e);
        let collision = object(&mut e);
        e.mem.set_u32(VTABLE + 0x8c, 0x0030_1100);
        noop(&mut e, &[0x0066_b0d0, 0x0030_1100]);
        e.register_double(0x0055_9450, move |_, _| ret(collision));
        e.register(OWNER_GETTER, |_, _| ret(0x1234));
        e.call_log = Some(vec![]);
        e.call(0x0062_bc90, &args![this, collision]);
        assert_eq!(calls(&e, 0x0030_1100), vec![vec![collision, this]]);
        e.register_double(OWNER_GETTER, move |_, _| ret(this));
        e.call_log = Some(vec![]);
        e.call(0x0062_bc90, &args![this, collision]);
        assert!(calls(&e, 0x0030_1100).is_empty());
    }

    #[test]
    fn inspect_counts_by_object_kind() {
        let mut e = Engine::new();
        scene(&mut e);
        let havok = object(&mut e);
        e.mem.set_u32(havok + 0x10, VTABLE);
        e.mem.set_u32(havok + 0x14, 5);
        e.mem.set_u32(havok + 8, 9);
        e.mem.set_u32(VTABLE + 4, SLOT_WORD_4);
        e.mem.set_u32(VTABLE + 0x2c, SLOT_WORD_8);
        e.register(0x004a_e6a0, |_, _| ret(0x66));
        e.register_double(0x0062_0b80, move |_, _| ret(havok));
        e.register(0x0043_b230, |e, a| ret(e.mem.u32(a[0] + 0x30)));
        e.register_double(0x0072_6070, move |_, _| ret(havok));
        let body = object(&mut e);
        let child = node(&mut e, body, &[]);
        let root = node(&mut e, 0, &[child.addr()]);
        let counts = e.mem.alloc(8);
        for (kind, first, second) in [(0x17, 0, 5), (6, 9, 5), (10, 9, 10), (3, 9, 10)] {
            e.mem.set_u32(havok + 0x30, kind);
            e.call(0x0062_bd00, &args![root, counts, counts + 4]);
            assert_eq!(e.mem.u32(counts), first);
            assert_eq!(e.mem.u32(counts + 4), second);
        }
    }

    #[test]
    fn member_getter_reads_offset_30() {
        let mut e = Engine::new();
        e.register(0x0072_6070, |_, a| ret(a[0] + 1));
        assert_eq!(e.call(0x0062_be70, &args![0x100u32]).u32(), 0x131);
    }

    #[test]
    fn piecewise_scale_bands_and_threshold() {
        let mut e = Engine::new();
        values(
            &mut e,
            &[
                (0x011c_fb38, 10.0),
                (0x011c_e714, 20.0),
                (0x011c_fa88, 30.0),
                (0x011c_f600, 1.0),
                (0x011c_fd64, 5.0),
                (0x011c_f92c, 2.0),
                (0x011c_e154, 6.0),
                (0x011c_e978, 3.0),
                (0x011c_e7a4, 7.0),
                (0x011c_e984, 0.5),
                (0x011c_f2fc, 1.0),
            ],
        );
        e.map(ONE_THOUSAND, 4);
        e.set_global(ONE_THOUSAND, bits(1000.0));
        let run = |e: &mut Engine, a: f32, b: f32| e.call(0x0062_be90, &args![a, b]).f32();
        assert_eq!(run(&mut e, 5.0, 2000.0), 0.0);
        assert_eq!(run(&mut e, 10.0, 8.0), 5.0);
        assert_eq!(run(&mut e, 20.0, 8.0), 10.0);
        assert_eq!(run(&mut e, 30.0, 8.0), 15.0);
        // Equal to the threshold, or NaN, gives 0.
        assert_eq!(run(&mut e, 30.0, 7.0), 0.0);
        assert_eq!(run(&mut e, 30.0, f32::NAN), 0.0);
        // NaN `a` falls through every band test.
        assert_eq!(run(&mut e, f32::NAN, 8.0), 15.0);
    }

    fn local_point_doubles(e: &mut Engine) {
        e.map(DEFAULT_POINT_011F426C, 12);
        for (index, value) in [0.25f32, 0.5, 0.75].iter().enumerate() {
            e.mem
                .set_u32(DEFAULT_POINT_011F426C + 4 * index as u32, bits(*value));
        }
        e.map(FIFTY_FIVE_HUNDREDTHS, 8);
        e.set_global(FIFTY_FIVE_HUNDREDTHS, bits(0.55));
        e.set_global(FORTY_FIVE_HUNDREDTHS, bits(0.45));
        e.register(SCALED_VECTOR, |_, a| ret(a[0]));
        let mut sums = 0u32;
        e.register_double(VECTOR_SUM, move |e, a| {
            sums += 1;
            let base = if sums == 1 { 1.0f32 } else { 4.0 };
            for index in 0..3 {
                e.mem.set_u32(a[1] + 4 * index, bits(base + index as f32));
            }
            ret(a[1])
        });
        e.register(0x0047_6b70, |_, a| Ret {
            st0: f32::from_bits(a[0]) as f64 * 10.0 + f32::from_bits(a[1]) as f64,
            ..Ret::default()
        });
        e.register(0x0041_6870, |e, a| {
            for index in 0..3 {
                e.mem.set_u32(a[0] + 4 * index, a[1 + index as usize]);
            }
            ret(a[0])
        });
        e.register(0x0043_f8d0, |_, a| ret(a[1]));
        e.register(0x004b_4500, |_, a| ret(a[1]));
        e.register(0x0046_1130, |_, _| ret(0x4600));
        e.register(COLLISION_RIGID_BODY, |e, a| ret(e.mem.u32(a[0])));
        noop(
            e,
            &[
                0x0043_7c10,
                0x00c8_d390,
                VECTOR_CONSTRUCTOR,
                0x0045_8620,
                0x0056_ded0,
                0x00c8_d280,
                TRANSFORM_CONSTRUCTOR,
                0x0062_4ad0,
                0x004b_4880,
            ],
        );
    }

    #[test]
    fn local_point_defaults_randomizes_and_transforms() {
        let mut e = Engine::new();
        local_point_doubles(&mut e);
        let out = e.mem.alloc(12);
        let node = 0x5000u32;
        e.register(0x004b_5260, |_, _| ret(0));
        assert_eq!(e.call(0x0062_bfb0, &args![out, node]).u32(), out);
        assert_eq!(e.mem.u32(out + 8), bits(0.75));

        let collision = e.mem.alloc(4);
        e.mem.set_u32(collision, 0x6000);
        e.register_double(0x004b_5260, move |_, _| ret(collision));
        e.register(0x0043_b300, |_, _| ret(0));
        e.register(OWNER_GETTER, |_, _| ret(0x5000));
        e.call_log = Some(vec![]);
        e.call(0x0062_bfb0, &args![out, node]);
        let random = calls(&e, 0x0047_6b70);
        assert_eq!(random[0], vec![bits(6.0), bits(3.0)]);
        let point: Vec<u32> = (0..3).map(|i| e.mem.u32(out + 4 * i)).collect();
        assert_eq!(point, vec![bits(41.0), bits(52.0), bits(63.0)]);
        let scaled = calls(&e, SCALED_VECTOR);
        let factors: Vec<u32> = scaled.iter().map(|a| a[1]).collect();
        assert_eq!(
            factors,
            vec![bits(0.45), bits(0.55), bits(0.55), bits(0.45)]
        );
        assert!(calls(&e, 0x0052_4c40).is_empty());

        let moved = e.mem.alloc(12);
        e.mem.set_u32(moved, 7);
        e.register(OWNER_GETTER, |_, _| ret(0x5001));
        e.register_double(0x0052_4c40, move |_, _| ret(moved));
        e.call_log = Some(vec![]);
        e.call(0x0062_bfb0, &args![out, node]);
        assert_eq!(e.mem.u32(out), 7);
        assert_eq!(calls(&e, 0x004b_4880)[0][0], 0x4600);
        assert_eq!(calls(&e, 0x0043_b300)[0], vec![BODY_TABLE_012683C8, 0x6000]);
    }

    #[test]
    fn transform_product_copies_rotation_translation_and_scale() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x34);
        let other = e.mem.alloc(0x34);
        let out = e.mem.alloc(0x34);
        e.mem.set_u32(this + 0x30, bits(2.0));
        e.mem.set_u32(other + 0x30, bits(3.0));
        let rotation = e.mem.alloc(36);
        for index in 0..9 {
            e.mem.set_u32(rotation + 4 * index, index + 1);
        }
        let moved = e.mem.alloc(12);
        for index in 0..3 {
            e.mem.set_u32(moved + 4 * index, index + 10);
        }
        noop(&mut e, &[TRANSFORM_CONSTRUCTOR]);
        e.register_double(0x0043_f8d0, move |_, _| ret(rotation));
        e.register(0x004b_4500, |_, a| ret(a[1]));
        e.register(SCALED_VECTOR, |_, a| ret(a[0]));
        e.register_double(VECTOR_SUM, move |_, _| ret(moved));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_c250, &args![this, out, other]).u32(), out);
        let words: Vec<u32> = (0..13).map(|i| e.mem.u32(out + 4 * i)).collect();
        assert_eq!(
            words,
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, bits(6.0)]
        );
        assert_eq!(calls(&e, 0x004b_4500)[0][2], other + 0x24);
        assert_eq!(calls(&e, SCALED_VECTOR)[0][1], bits(2.0));
        assert_eq!(calls(&e, VECTOR_SUM)[0][0], this + 0x24);
    }

    #[test]
    fn body_value_sum_recurses_into_children() {
        let mut e = Engine::new();
        scene(&mut e);
        e.register(0x004b_5400, |_, _| Ret {
            st0: 1.5,
            ..Ret::default()
        });
        let body = object(&mut e);
        let first = node(&mut e, body, &[]);
        let second = node(&mut e, body, &[]);
        let root = node(&mut e, body, &[first.addr(), second.addr()]);
        assert_eq!(e.call(0x0062_c300, &args![root]).f32(), 4.5);
        assert_eq!(e.call(0x0062_c300, &args![0u32]).f32(), 0.0);
    }

    #[test]
    fn ragdoll_check_uses_virtuals_then_flags() {
        let mut e = Engine::new();
        scene(&mut e);
        e.mem.set_u32(VTABLE + 0x100, SLOT_WORD_4);
        e.mem.set_u32(VTABLE + 0x22c, SLOT_WORD_8);
        e.mem.set_u32(VTABLE + 0x230, SLOT_WORD_C);
        e.register(0x0062_4b70, |_, a| ret(a[0]));
        let reference = object(&mut e);
        let run = |e: &mut Engine, words: [u32; 3], node: u32| {
            for (index, word) in words.iter().enumerate() {
                e.mem.set_u32(reference + 4 + 4 * index as u32, *word);
            }
            e.call(0x0062_c3c0, &args![reference, node]).bool()
        };
        assert!(run(&mut e, [1, 0, 1], 0));
        assert!(run(&mut e, [1, 1, 0], 0));
        assert!(!run(&mut e, [1, 0, 0], 0));
        assert!(!run(&mut e, [0, 1, 1], 0x47));
        assert!(run(&mut e, [0, 1, 1], 0x48));
        e.call_log = Some(vec![]);
        assert!(e.call(0x0062_c3c0, &args![0u32, 0x48u32]).bool());
        assert_eq!(calls(&e, 0x0062_4b70), vec![vec![0x48, 0x48]]);
    }

    #[test]
    fn consolidate_merges_islands_of_bodies_in_first_world() {
        let mut e = Engine::new();
        scene(&mut e);
        let reference = e.mem.alloc(0xb0);
        e.mem.set_u32(reference, VTABLE);
        e.mem.set_u32(VTABLE + 0x1d0, SLOT_WORD_4);
        e.mem.set_u32(reference + 4, 0x4444);
        let table = e.mem.alloc(12);
        for (index, body) in [0x100u32, 0x200, 0x300].iter().enumerate() {
            e.mem.set_u32(table + 4 * index as u32, *body);
        }
        e.register_double(0x0062_9ab0, move |_, a| ret(table + 4 * a[1]));
        e.register(0x0072_6070, |_, _| ret(3));
        e.register(OWNER_GETTER, |_, a| {
            ret(if a[0] == 0x300 { 0x9 } else { 0x8 })
        });
        e.register(0x009a_d610, |_, a| ret(a[0] >> 8));
        noop(
            &mut e,
            &[
                0x0062_99a0,
                0x00c8_0930,
                0x00c7_be20,
                0x0048_3710,
                0x00cb_5570,
                0x0062_9b40,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0062_c430, &args![reference]);
        assert_eq!(calls(&e, 0x00c8_0930)[0][0], 0x4444);
        assert_eq!(calls(&e, 0x00cb_5570), vec![vec![0x8, 0x100, 0x200]]);
        assert_eq!(calls(&e, 0x0048_3710), vec![vec![0x8], vec![0x8]]);
        assert_eq!(calls(&e, 0x0062_9b40).len(), 1);

        e.mem.set_u32(reference + 0xac, 0x5555);
        e.call_log = Some(vec![]);
        e.call(0x0062_c430, &args![reference]);
        assert_eq!(calls(&e, 0x00c7_be20)[0][0], 0x5555);
        assert!(calls(&e, 0x00c8_0930).is_empty());
    }
}
