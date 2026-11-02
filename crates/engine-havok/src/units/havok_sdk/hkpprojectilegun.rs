//! `Havok SDK/hkpprojectilegun.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

use std::cmp::Ordering;

use super::hkpsphereshape::{
    copy_vec, inverse_length, load_vec, shufps, store_vec, sum3, zip_with,
};

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00e450c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e4_4e70`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e450c0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e4_4e70, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00e450f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e4_5040`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e450f0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e4_5040, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00e450a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x8` (`SUB ECX,0x8`) and jumps to `00e450c0`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00e450a0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00e4_50c0, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

/// `hkRotation::setAxisAngle` (`00cb2e30`, `this` = the rotation, `(axis, angle)`, `RET 8`).
const ROTATION_SET_AXIS_ANGLE: u32 = 0x00cb_2e30;
/// `hkMatrix3::setMul` (`00cd55b0`, `this` = the result, `(first, second)`, `RET 8`).
const MATRIX_SET_MUL: u32 = 0x00cd_55b0;
/// `hkpRigidBody::setTransform` (`00ca8890`, `this` = the body, one word, `RET 4`).
const RIGID_BODY_SET_TRANSFORM: u32 = 0x00ca_8890;
/// `hkReferencedObject::removeReference` (`00c905b0`, `this` = the object).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkpWorld::addEntity` (`00c914d0`, `this` = the world, `(entity, 1)`, `RET 8`).
const WORLD_ADD_ENTITY: u32 = 0x00c9_14d0;
/// `hkpWorld::removeEntity` (`00c91620`, `this` = the world, `(result byte, entity)`, `RET 8`).
const WORLD_REMOVE_ENTITY: u32 = 0x00c9_1620;
/// The entity function at `00c9c540` (unnamed in the engine map, `hkpentity.cpp`): `this` = the
/// entity, one word, `RET 4`.
const ENTITY_FN_C9C540: u32 = 0x00c9_c540;
/// The entity function at `00c9c140` (unnamed in the engine map, `hkpentity.cpp`): `this` = the
/// entity, one word, `RET 4`.
const ENTITY_FN_C9C140: u32 = 0x00c9_c140;
/// `hkpFirstPersonGun::hkpFirstPersonGun` (`00e46660`, `this` only).
const FIRST_PERSON_GUN_CONSTRUCTOR: u32 = 0x00e4_6660;
/// The base copy-style constructor at `00e466a0` (`hkpfirstpersongun.cpp`, `this`, one word,
/// `RET 4`).
const FIRST_PERSON_GUN_COPY: u32 = 0x00e4_66a0;
/// `hkpFirstPersonGun::~hkpFirstPersonGun` (`00e466e0`, `this` only).
const FIRST_PERSON_GUN_DESTRUCTOR: u32 = 0x00e4_66e0;
/// The vtable the gun sets in its constructor and destructor.
const GUN_VTABLE: u32 = 0x010e_bfb0;
/// The vtable a gun-like object has while its parts are torn down.
const GUN_PARTS_VTABLE: u32 = 0x010e_bf94;
/// The vtable of the object's second base while it is torn down.
const GUN_SECOND_BASE_VTABLE: u32 = 0x010e_bf74;
/// The vtable of that second base once the destructor has finished with it.
const SECOND_BASE_FINAL_VTABLE: u32 = 0x0104_dd7c;
/// The base object's vtable.
const BASE_OBJECT_VTABLE: u32 = 0x0102_e388;
/// The `float` the constructor starts the reload time at (`01017870`).
const DEFAULT_RELOAD_TIME: u32 = 0x0101_7870;
/// The `float` below which the sideways turn of a flying projectile is not corrected
/// (`010718c0`).
const TURN_EPSILON: u32 = 0x0107_18c0;
/// The `double` the turn angle is multiplied by (`01072f80`, `-5.0`).
const TURN_SCALE: u32 = 0x0107_2f80;

/// An offset in `flyTrue`'s frame, addressed like the exe's: from the stack pointer once the
/// single register is pushed, which is 4 below the frame's base.
const fn esp(offset: u32) -> u32 {
    offset - 4
}

/// The projectile's four `flyTrue` blocks: the transform being built (three rows and the
/// translation), the axis, the result of the matrix product and the rotation.
const FLY_TRANSFORM: u32 = esp(0x20);
const FLY_AXIS: u32 = esp(0x10);
const FLY_PRODUCT: u32 = esp(0x60);
const FLY_ROTATION: u32 = esp(0x90);
const FLY_SCRATCH: u32 = esp(0xc);

// Translated from 00e44990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGunProjectile::flyTrue` (cdecl: the body and two floats). The body's velocity (`+0x1b0`)
/// is turned toward by correcting the body's rotation, three rows at `+0xf0`, `+0x100` and
/// `+0x110` and the translation at `+0x120`: when the speed is above `threshold`, the cross
/// product of the unit velocity and the first rotation row is formed, and when its length is
/// above the tiny float at `010718c0` the rotation becomes the rotation about that cross
/// product (normalised) by `length * turn_rate * -5.0` (`setAxisAngle`) times the old one
/// (`hkMatrix3::setMul`); the transform (three new rows, the old translation) is stored with
/// `hkpRigidBody::setTransform`.
pub fn hkp_gun_projectile_fly_true(e: &mut Engine, body: Ptr, threshold: f32, turn_rate: f32) {
    let body = body.addr();
    e.with_stack(0xbc, |e, frame| {
        let f = frame.addr();
        let rows = [
            load_vec(e, body + 0x100),
            load_vec(e, body + 0x110),
            load_vec(e, body + 0x120),
        ];
        let velocity = load_vec(e, body + 0x1b0);
        let first_row = load_vec(e, body + 0xf0);
        for (index, row) in rows.into_iter().enumerate() {
            store_vec(e, f + FLY_TRANSFORM + 0x10 * (index as u32 + 1), row);
        }
        let speed = sum3(zip_with(velocity, velocity, |a, b| a * b)).sqrt();
        e.mem.set_f32(f + FLY_SCRATCH, speed);
        store_vec(e, f + FLY_TRANSFORM, first_row);
        let faster = f64::from(x87_quiet(threshold)).partial_cmp(&f64::from(x87_quiet(speed)))
            == Some(Ordering::Less);
        if !faster {
            return;
        }
        let epsilon = x87_quiet(e.global::<f32>(TURN_EPSILON));
        let unit_factor = inverse_length(e, sum3(zip_with(velocity, velocity, |a, b| a * b)));
        let unit = velocity.map(|lane| unit_factor * lane);
        let plus = zip_with(
            shufps(first_row, first_row, 0xd2),
            shufps(unit, unit, 0xc9),
            |a, b| a * b,
        );
        let minus = zip_with(
            shufps(first_row, first_row, 0xc9),
            shufps(unit, unit, 0xd2),
            |a, b| a * b,
        );
        let cross = zip_with(plus, minus, |a, b| a - b);
        let squared = sum3(zip_with(cross, cross, |a, b| a * b));
        let length = squared.sqrt();
        e.mem.set_f32(f + FLY_SCRATCH, length);
        let length = x87_quiet(length);
        let longer = f64::from(length).partial_cmp(&f64::from(epsilon)) == Some(Ordering::Greater);
        if !longer {
            return;
        }
        let scale: f64 = e.global(TURN_SCALE);
        let angle = ((f64::from(length) * f64::from(x87_quiet(turn_rate))) * scale) as f32;
        e.mem.set_f32(f + FLY_SCRATCH, angle);
        let angle = x87_quiet(e.mem.f32(f + FLY_SCRATCH));
        let axis_factor = inverse_length(e, squared);
        store_vec(e, f + FLY_AXIS, cross.map(|lane| axis_factor * lane));
        e.call(
            ROTATION_SET_AXIS_ANGLE,
            &args![f + FLY_ROTATION, f + FLY_AXIS, angle],
        );
        e.call(
            MATRIX_SET_MUL,
            &args![f + FLY_PRODUCT, f + FLY_ROTATION, f + FLY_TRANSFORM],
        );
        for row in 0..3 {
            copy_vec(
                e,
                f + FLY_PRODUCT + 0x10 * row,
                f + FLY_TRANSFORM + 0x10 * row,
            );
        }
        e.call(RIGID_BODY_SET_TRANSFORM, &args![body, f + FLY_TRANSFORM]);
    });
}

// Translated from 00e44bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpProjectileGun::fireGun` (`this` in `ECX`, two stack words that the callee pops): when
/// the float at `+0x28` is not above zero it is set to the float at `+0x24` and the virtual
/// function at byte offset `0x24` is run with the same two words (a tail jump); otherwise
/// nothing happens.
pub fn hkp_projectile_gun_fire_gun(e: &mut Engine, this: Ptr, word_0: u32, word_1: u32) {
    let gun = this.addr();
    let waiting = 0.0 < f64::from(x87_quiet(e.mem.f32(gun + 0x28)));
    if waiting {
        return;
    }
    let reload = x87_quiet_bits(e.mem.u32(gun + 0x24));
    e.mem.set_u32(gun + 0x28, reload);
    e.vcall(gun, 0x24, &args![word_0, word_1]);
}

// Translated from 00e44be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpProjectileGun::stepGun` (`this` in `ECX`, a float and five words that the callee
/// pops): when the float at `+0x28` is above zero the step is taken off it (`f32(+0x28 - step)`),
/// then the virtual function at byte offset `0x28` is called with the step and the five words.
pub fn hkp_projectile_gun_step_gun(
    e: &mut Engine,
    this: Ptr,
    step: f32,
    word_0: u32,
    word_1: u32,
    word_2: u32,
    word_3: u32,
    word_4: u32,
) {
    let gun = this.addr();
    let step = x87_quiet(step);
    if 0.0 < f64::from(x87_quiet(e.mem.f32(gun + 0x28))) {
        let left = f64::from(x87_quiet(e.mem.f32(gun + 0x28))) - f64::from(step);
        e.mem.set_f32(gun + 0x28, left as f32);
    }
    e.vcall(
        gun,
        0x28,
        &args![step, word_0, word_1, word_2, word_3, word_4],
    );
}

// Translated from 00e44c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`this` in `ECX`): when bit 0 of the flag byte at `+0xc` is clear,
/// runs the virtual function at byte offset `0xc` and then sets the bit.
pub fn fn_00e44c70(e: &mut Engine, this: Ptr) {
    let projectile = this.addr();
    if e.mem.u8(projectile + 0xc) & 1 == 0 {
        e.vcall(projectile, 0xc, &args![]);
        let flags = e.mem.u8(projectile + 0xc);
        e.mem.set_u8(projectile + 0xc, flags | 1);
    }
}

// Translated from 00e44c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`this` in `ECX`, one stack word that the callee pops, the
/// world): when bit 2 of the flag byte at `+0xc` is set, the entity at `+0x10` gets the
/// function at `00c9c540` with `this + 8`; then the entity is added to the world
/// (`hkpWorld::addEntity` with `(entity, 1)`).
pub fn fn_00e44c90(e: &mut Engine, this: Ptr, world: Ptr) {
    let projectile = this.addr();
    if e.mem.u8(projectile + 0xc) & 4 != 0 {
        let entity = e.mem.u32(projectile + 0x10);
        e.call(ENTITY_FN_C9C540, &args![entity, projectile + 8]);
    }
    let entity = e.mem.u32(projectile + 0x10);
    e.call(WORLD_ADD_ENTITY, &args![world, entity, 1u32]);
}

// Translated from 00e44cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`this` in `ECX`): when the entity at `+0x10` exists, it is
/// removed from its world (the world is the entity's word at `+8`, `hkpWorld::removeEntity`
/// with a result byte and the entity), and when bit 2 of the flag byte at `+0xc` is set the
/// function at `00c9c140` runs on the entity with `this + 8`.
pub fn fn_00e44cc0(e: &mut Engine, this: Ptr) {
    let projectile = this.addr();
    let entity = e.mem.u32(projectile + 0x10);
    if entity == 0 {
        return;
    }
    e.with_stack(4, |e, scratch| {
        let world = e.mem.u32(entity + 8);
        e.call(
            WORLD_REMOVE_ENTITY,
            &args![world, scratch.addr() + 3, entity],
        );
    });
    if e.mem.u8(projectile + 0xc) & 4 != 0 {
        let entity = e.mem.u32(projectile + 0x10);
        e.call(ENTITY_FN_C9C140, &args![entity, projectile + 8]);
    }
}

// Translated from 00e44cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`this` in `ECX`): when the entity at `+0x10` exists, removes it
/// from its world when it has one (`+8`, `00e44cc0`), releases it and clears the field.
pub fn fn_00e44cf0(e: &mut Engine, this: Ptr) {
    let projectile = this.addr();
    let entity = e.mem.u32(projectile + 0x10);
    if entity == 0 {
        return;
    }
    if e.mem.u32(entity + 8) != 0 {
        fn_00e44cc0(e, this);
    }
    let entity = e.mem.u32(projectile + 0x10);
    e.call(REMOVE_REFERENCE, &args![entity]);
    e.mem.set_u32(projectile + 0x10, 0);
}

// Translated from 00e44d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`this` in `ECX`, one float that the callee pops): unless bit 0 of
/// the flag byte at `+0xc` is set, adds the float to the one at `+0x14` and runs the virtual
/// function at byte offset `0x14` with it.
pub fn fn_00e44d20(e: &mut Engine, this: Ptr, step: f32) {
    let projectile = this.addr();
    if e.mem.u8(projectile + 0xc) & 1 != 0 {
        return;
    }
    let age = f64::from(x87_quiet(e.mem.f32(projectile + 0x14)));
    let step = x87_quiet(step);
    e.mem
        .set_f32(projectile + 0x14, (age + f64::from(step)) as f32);
    e.vcall(projectile, 0x14, &args![step]);
}

/// The projectiles of a gun: the pointer array at `+0x2c` and its count at `+0x30`.
fn projectile_at(e: &Engine, gun: u32, index: u32) -> u32 {
    let array = e.mem.u32(gun + 0x2c);
    e.mem.u32(array + 4 * index)
}

// Translated from 00e44da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`this` in `ECX`, one float that the callee pops): every
/// projectile whose float at `+0x14` is above the argument gets `00e44c70`.
pub fn fn_00e44da0(e: &mut Engine, this: Ptr, limit: f32) {
    let gun = this.addr();
    let count = e.mem.i32(gun + 0x30);
    for index in 0..count {
        let projectile = projectile_at(e, gun, index as u32);
        let age = f64::from(x87_quiet(e.mem.f32(projectile + 0x14)));
        if f64::from(x87_quiet(limit)) < age {
            fn_00e44c70(e, Ptr::new(projectile));
        }
    }
}

// Translated from 00e44de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`this` in `ECX`, one float that the callee pops): runs
/// `00e44d20` with the float on every projectile.
pub fn fn_00e44de0(e: &mut Engine, this: Ptr, step: f32) {
    let gun = this.addr();
    let count = e.mem.i32(gun + 0x30);
    for index in 0..count {
        let projectile = projectile_at(e, gun, index as u32);
        fn_00e44d20(e, Ptr::new(projectile), x87_quiet(step));
    }
}

// Translated from 00e44e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`this` in `ECX`): clears bit 1 of the flag byte at `+0xc` of
/// every projectile.
pub fn fn_00e44e10(e: &mut Engine, this: Ptr) {
    let gun = this.addr();
    let count = e.mem.i32(gun + 0x30);
    for index in 0..count {
        let projectile = projectile_at(e, gun, index as u32);
        let flags = e.mem.u8(projectile + 0xc);
        e.mem.set_u8(projectile + 0xc, flags & 0xfd);
    }
}

// Translated from 00e44e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`this` in `ECX`): a destructor body. The vtables are the
/// intermediate ones while `00e44cf0` runs, then the second base's and the base object's.
pub fn fn_00e44e70(e: &mut Engine, this: Ptr) {
    let object = this.addr();
    e.mem.set_u32(object, GUN_PARTS_VTABLE);
    e.mem.set_u32(object + 8, GUN_SECOND_BASE_VTABLE);
    fn_00e44cf0(e, this);
    e.mem.set_u32(object + 8, SECOND_BASE_FINAL_VTABLE);
    e.mem.set_u32(object, BASE_OBJECT_VTABLE);
}

// Translated from 00e44ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a thunk, a jump to `00e44cf0` with the same `this`.
pub fn fn_00e44ea0(e: &mut Engine, this: Ptr) {
    fn_00e44cf0(e, this);
}

// Translated from 00e44eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpProjectileGun::clearProjectiles` (`this` in `ECX`): every projectile gets `00e44c70`
/// and is released, then the count at `+0x30` is zero.
pub fn hkp_projectile_gun_clear_projectiles(e: &mut Engine, this: Ptr) {
    let gun = this.addr();
    let count = e.mem.i32(gun + 0x30);
    for index in 0..count {
        let projectile = projectile_at(e, gun, index as u32);
        fn_00e44c70(e, Ptr::new(projectile));
        e.call(REMOVE_REFERENCE, &args![projectile]);
    }
    e.mem.set_u32(gun + 0x30, 0);
}

// Translated from 00e44ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`this` in `ECX`): every projectile whose flag byte (`+0xc`) has
/// bit 0 set is released and removed from the array by moving the last one into its place (and
/// looking at that one next).
pub fn fn_00e44ef0(e: &mut Engine, this: Ptr) {
    let gun = this.addr();
    let mut count = e.mem.i32(gun + 0x30);
    let mut index: i32 = 0;
    while index < count {
        let projectile = projectile_at(e, gun, index as u32);
        if e.mem.u8(projectile + 0xc) & 1 != 0 {
            e.call(REMOVE_REFERENCE, &args![projectile]);
            let last = e.mem.i32(gun + 0x30).wrapping_sub(1);
            e.mem.set_i32(gun + 0x30, last);
            if last != index {
                let array = e.mem.u32(gun + 0x2c);
                let moved = e.mem.u32(array + 4 * last as u32);
                e.mem.set_u32(array + 4 * index as u32, moved);
            }
            count -= 1;
            index -= 1;
        }
        index += 1;
    }
}

// Translated from 00e44f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpProjectileGun::onUpdate` (`this` in `ECX`, a float and five words that the callee
/// pops; only the float is used): ages every projectile by the float (`00e44de0`), clears bit 1
/// of every flag byte (`00e44e10`) and drops the finished projectiles (`00e44ef0`).
pub fn hkp_projectile_gun_on_update(
    e: &mut Engine,
    this: Ptr,
    step: f32,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
) {
    fn_00e44de0(e, this, x87_quiet(step));
    fn_00e44e10(e, this);
    fn_00e44ef0(e, this);
}

// Translated from 00e44f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpProjectileGun::reset` (`this` in `ECX`, one unused word that the callee pops): clears
/// the projectiles.
pub fn hkp_projectile_gun_reset(e: &mut Engine, this: Ptr, _unused_0: u32) {
    hkp_projectile_gun_clear_projectiles(e, this);
}

// Translated from 00e44fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`this` in `ECX`, two words that the callee pops): a constructor.
/// Runs the base constructor (`00e46660`), then sets the reload time (`+0x24`) to the float at
/// `01017870`, the countdown (`+0x28`) to zero, the vtable, `+0x20` to 5, the projectile array
/// (`+0x2c`, count `+0x30`, capacity word `+0x34` = `0x80000000`, the "owns nothing" mark) to
/// empty and the two words at `+0x38` and `+0x3c`. Returns `this`.
pub fn fn_00e44fc0(e: &mut Engine, this: Ptr, word_0: u32, word_1: u32) -> Ptr {
    let gun = this.addr();
    e.call(FIRST_PERSON_GUN_CONSTRUCTOR, &args![this]);
    let reload = x87_quiet_bits(e.mem.u32(DEFAULT_RELOAD_TIME));
    e.mem.set_u32(gun + 0x24, reload);
    e.mem.set_f32(gun + 0x28, 0.0);
    e.mem.set_u32(gun, GUN_VTABLE);
    e.mem.set_u32(gun + 0x20, 5);
    e.mem.set_u32(gun + 0x2c, 0);
    e.mem.set_u32(gun + 0x30, 0);
    e.mem.set_u32(gun + 0x34, 0x8000_0000);
    e.mem.set_u32(gun + 0x38, word_0);
    e.mem.set_u32(gun + 0x3c, word_1);
    this
}

// Translated from 00e45010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`this` in `ECX`, one word that the callee pops): runs the base
/// function at `00e466a0` with the word, then sets the vtable and an empty projectile array
/// (as `00e44fc0`). Returns `this`.
pub fn fn_00e45010(e: &mut Engine, this: Ptr, word: u32) -> Ptr {
    let gun = this.addr();
    e.call(FIRST_PERSON_GUN_COPY, &args![this, word]);
    e.mem.set_u32(gun, GUN_VTABLE);
    e.mem.set_u32(gun + 0x2c, 0);
    e.mem.set_u32(gun + 0x30, 0);
    e.mem.set_u32(gun + 0x34, 0x8000_0000);
    this
}

// Translated from 00e45040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpProjectileGun::~hkpProjectileGun` (`this` in `ECX`): sets the vtable, clears the
/// projectiles, releases the array's storage through the memory router's allocator (slot
/// `0x10`, `(data, (capacity & 0x3fffffff) * 4)`) unless the capacity word's top bit marks it
/// as not owned, empties the array and runs the base destructor (`00e466e0`, a tail jump).
pub fn hkp_projectile_gun_destructor(e: &mut Engine, this: Ptr) {
    let gun = this.addr();
    e.mem.set_u32(gun, GUN_VTABLE);
    hkp_projectile_gun_clear_projectiles(e, this);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(gun + 0x34);
    e.mem.set_u32(gun + 0x30, 0);
    if capacity & 0x8000_0000 == 0 {
        let data = e.mem.u32(gun + 0x2c);
        e.vcall(allocator, 0x10, &args![data, (capacity & 0x3fff_ffff) * 4]);
    }
    e.mem.set_u32(gun + 0x2c, 0);
    e.mem.set_u32(gun + 0x34, 0x8000_0000);
    e.call(FIRST_PERSON_GUN_DESTRUCTOR, &args![this]);
}

// Translated from 00e45090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk (unnamed in the engine map): moves `this` back by `8` and jumps to the virtual
/// function at byte offset `0x10` of that object's vtable, with the same stack word (the
/// function the vtable at `010ebf94` holds there, `00e450b0`, takes one word).
pub fn fn_00e45090(e: &mut Engine, this: Ptr, word: u32) {
    e.vcall(this.addr().wrapping_sub(8), 0x10, &args![word]);
}

// Translated from 00e450b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`this` in `ECX`, one unused word that the callee pops): sets bit
/// 1 of the flag byte at `+0xc`.
pub fn fn_00e450b0(e: &mut Engine, this: Ptr, _unused_0: u32) {
    let flags = e.mem.u8(this.addr() + 0xc);
    e.mem.set_u8(this.addr() + 0xc, flags | 2);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e4_4990, hkp_gun_projectile_fly_true(Ptr, f32, f32)),
        entry!(0x00e4_4bc0, hkp_projectile_gun_fire_gun(Ptr, u32, u32)),
        entry!(
            0x00e4_4be0,
            hkp_projectile_gun_step_gun(Ptr, f32, u32, u32, u32, u32, u32)
        ),
        entry!(0x00e4_4c70, fn_00e44c70(Ptr)),
        entry!(0x00e4_4c90, fn_00e44c90(Ptr, Ptr)),
        entry!(0x00e4_4cc0, fn_00e44cc0(Ptr)),
        entry!(0x00e4_4cf0, fn_00e44cf0(Ptr)),
        entry!(0x00e4_4d20, fn_00e44d20(Ptr, f32)),
        entry!(0x00e4_4da0, fn_00e44da0(Ptr, f32)),
        entry!(0x00e4_4de0, fn_00e44de0(Ptr, f32)),
        entry!(0x00e4_4e10, fn_00e44e10(Ptr)),
        entry!(0x00e4_4e70, fn_00e44e70(Ptr)),
        entry!(0x00e4_4ea0, fn_00e44ea0(Ptr)),
        entry!(0x00e4_4eb0, hkp_projectile_gun_clear_projectiles(Ptr)),
        entry!(0x00e4_4ef0, fn_00e44ef0(Ptr)),
        entry!(
            0x00e4_4f30,
            hkp_projectile_gun_on_update(Ptr, f32, u32, u32, u32, u32, u32)
        ),
        entry!(0x00e4_4f50, hkp_projectile_gun_reset(Ptr, u32)),
        entry!(0x00e4_4fc0, fn_00e44fc0(Ptr, u32, u32) -> Ptr),
        entry!(0x00e4_5010, fn_00e45010(Ptr, u32) -> Ptr),
        entry!(0x00e4_5040, hkp_projectile_gun_destructor(Ptr)),
        entry!(0x00e4_5090, fn_00e45090(Ptr, u32)),
        entry!(0x00e4_50b0, fn_00e450b0(Ptr, u32)),
        entry!(0x00e4_50c0, deleting_destructor_00e450c0(Ptr, u8) -> Ptr),
        entry!(0x00e4_50f0, deleting_destructor_00e450f0(Ptr, u8) -> Ptr),
        entry!(0x00e4_50a0, fn_00e450a0(Ptr, u32) -> u32),
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
    fn deleting_destructor_00e450c0_frees_only_when_bit_zero_is_set() {
        check(0x00e4_50c0, 0x00e4_4e70);
    }

    #[test]
    fn deleting_destructor_00e450f0_frees_only_when_bit_zero_is_set() {
        check(0x00e4_50f0, 0x00e4_5040);
    }
}

#[cfg(test)]
mod a07_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and one flags word, with a double
    /// over `target` that returns `0xdeadbeef`; checks that the double ran
    /// once, with `this` moved back by `back` and the flags passed on, and
    /// returns the thunk's `EAX`.
    fn check(thunk: u32, target: u32, back: u32) -> u32 {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - back, 0x1]);
        result.u32()
    }

    #[test]
    fn thunk_00e450a0() {
        assert_eq!(check(0x00e4_50a0, 0x00e4_50c0, 0x8), 0xdead_beef);
    }
}

#[cfg(test)]
mod a05_projectile_gun_tests {
    use super::super::hkpsphereshape::{Vec4, HALF_F32, THREE_F32, ZERO_F32};
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const GUN_TABLE: u32 = 0x0900_0300;
    const PROJECTILE_TABLE: u32 = 0x0900_0340;
    const ALLOCATOR_TABLE: u32 = 0x0900_0380;
    const SECOND_BASE_TABLE: u32 = 0x0900_03c0;
    /// The gun's virtual function at byte offset `0x24`.
    const SLOT_FIRE: u32 = 0x0900_0100;
    /// The gun's virtual function at byte offset `0x28`.
    const SLOT_STEP: u32 = 0x0900_0104;
    /// The projectile's virtual function at byte offset `0xc`.
    const SLOT_FIRST: u32 = 0x0900_0108;
    /// The projectile's virtual function at byte offset `0x14`.
    const SLOT_AGE: u32 = 0x0900_010c;
    /// The allocator's virtual function at byte offset `0x10`.
    const SLOT_FREE: u32 = 0x0900_0110;
    /// The second base's virtual function at byte offset `0x10`.
    const SLOT_SECOND: u32 = 0x0900_0114;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// An engine whose callees do nothing, whose vtables point at logged no-ops, with a memory
    /// router whose allocator is returned.
    fn world() -> (Engine, u32) {
        let mut e = Engine::new();
        for page in [
            0x0101_1000,
            0x0101_6000,
            0x0101_7000,
            0x0107_1000,
            0x0107_2000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.set_f32(ZERO_F32, 0.0);
        e.mem.set_f32(THREE_F32, 3.0);
        e.mem.set_f32(HALF_F32, 0.5);
        e.mem.set_f32(DEFAULT_RELOAD_TIME, 0.3);
        e.mem.set_f32(TURN_EPSILON, 1.0e-5);
        e.mem.set_f64(TURN_SCALE, -5.0);
        e.put_vtable(
            GUN_TABLE,
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, SLOT_FIRE, SLOT_STEP],
        );
        e.put_vtable(PROJECTILE_TABLE, &[0, 0, 0, SLOT_FIRST, 0, SLOT_AGE]);
        e.put_vtable(ALLOCATOR_TABLE, &[0, 0, 0, 0, SLOT_FREE]);
        e.put_vtable(SECOND_BASE_TABLE, &[0, 0, 0, 0, SLOT_SECOND]);
        for address in [
            SLOT_FIRE,
            SLOT_STEP,
            SLOT_FIRST,
            SLOT_AGE,
            SLOT_FREE,
            SLOT_SECOND,
            REMOVE_REFERENCE,
            WORLD_ADD_ENTITY,
            WORLD_REMOVE_ENTITY,
            ENTITY_FN_C9C540,
            ENTITY_FN_C9C140,
            FIRST_PERSON_GUN_CONSTRUCTOR,
            FIRST_PERSON_GUN_COPY,
            FIRST_PERSON_GUN_DESTRUCTOR,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, ALLOCATOR_TABLE);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        (e, allocator)
    }

    /// A gun with a projectile array whose entries have the flag bytes and ages given.
    fn gun_with(e: &mut Engine, projectiles: &[(u8, f32)]) -> (u32, Vec<u32>) {
        let gun = e.mem.alloc(0x60);
        e.mem.set_u32(gun, GUN_TABLE);
        let array = e.mem.alloc(4 * projectiles.len().max(1) as u32);
        let mut list = vec![];
        for (index, (flags, age)) in projectiles.iter().enumerate() {
            let projectile = e.mem.alloc(0x30);
            e.mem.set_u32(projectile, PROJECTILE_TABLE);
            e.mem.set_u8(projectile + 0xc, *flags);
            e.mem.set_f32(projectile + 0x14, *age);
            e.mem.set_u32(array + 4 * index as u32, projectile);
            list.push(projectile);
        }
        e.mem.set_u32(gun + 0x2c, array);
        e.mem.set_u32(gun + 0x30, projectiles.len() as u32);
        (gun, list)
    }

    #[test]
    fn fire_gun_reloads_unless_the_countdown_is_positive() {
        for (countdown, fires) in [(1.0, false), (0.0, true), (-1.0, true), (f32::NAN, true)] {
            let (mut e, _) = world();
            let (gun, _) = gun_with(&mut e, &[]);
            e.mem.set_f32(gun + 0x24, 2.5);
            e.mem.set_f32(gun + 0x28, countdown);
            e.call(0x00e4_4bc0, &args![gun, 7u32, 8u32]);
            let calls = calls_to(&e, SLOT_FIRE);
            if fires {
                assert_eq!(calls, vec![vec![gun, 7, 8]]);
                assert_eq!(e.mem.f32(gun + 0x28), 2.5);
            } else {
                assert!(calls.is_empty());
                assert_eq!(e.mem.f32(gun + 0x28), 1.0);
            }
        }
    }

    #[test]
    fn step_gun_counts_down_only_a_positive_countdown_and_always_calls_on() {
        for (countdown, expected) in [(3.0, 2.5), (0.0, 0.0), (-1.0, -1.0)] {
            let (mut e, _) = world();
            let (gun, _) = gun_with(&mut e, &[]);
            e.mem.set_f32(gun + 0x28, countdown);
            e.call(
                0x00e4_4be0,
                &args![gun, 0.5f32, 1u32, 2u32, 3u32, 4u32, 5u32],
            );
            assert_eq!(e.mem.f32(gun + 0x28), expected);
            assert_eq!(
                calls_to(&e, SLOT_STEP),
                vec![vec![gun, 0.5f32.to_bits(), 1, 2, 3, 4, 5]]
            );
        }
    }

    #[test]
    fn first_step_runs_once_and_sets_the_flag() {
        let (mut e, _) = world();
        let (_, list) = gun_with(&mut e, &[(0x06, 0.0), (0x01, 0.0)]);
        e.call(0x00e4_4c70, &args![list[0]]);
        e.call(0x00e4_4c70, &args![list[1]]);
        assert_eq!(calls_to(&e, SLOT_FIRST), vec![vec![list[0]]]);
        assert_eq!(e.mem.u8(list[0] + 0xc), 0x07);
        assert_eq!(e.mem.u8(list[1] + 0xc), 0x01);
    }

    #[test]
    fn adding_the_entity_runs_the_extra_function_only_for_flag_four() {
        for (flags, extra) in [(0x04, true), (0x00, false)] {
            let (mut e, _) = world();
            let (_, list) = gun_with(&mut e, &[(flags, 0.0)]);
            e.mem.set_u32(list[0] + 0x10, 0x1234);
            e.call(0x00e4_4c90, &args![list[0], 0x5555u32]);
            let first = calls_to(&e, ENTITY_FN_C9C540);
            if extra {
                assert_eq!(first, vec![vec![0x1234, list[0] + 8]]);
            } else {
                assert!(first.is_empty());
            }
            assert_eq!(
                calls_to(&e, WORLD_ADD_ENTITY),
                vec![vec![0x5555, 0x1234, 1]]
            );
        }
    }

    #[test]
    fn removing_the_entity_needs_an_entity_and_runs_the_extra_function_for_flag_four() {
        let (mut e, _) = world();
        let (_, list) = gun_with(&mut e, &[(0x04, 0.0)]);
        e.call(0x00e4_4cc0, &args![list[0]]);
        assert!(calls_to(&e, WORLD_REMOVE_ENTITY).is_empty());
        let entity = e.mem.alloc(0x20);
        e.mem.set_u32(entity + 8, 0x777);
        e.mem.set_u32(list[0] + 0x10, entity);
        e.call(0x00e4_4cc0, &args![list[0]]);
        let removed = calls_to(&e, WORLD_REMOVE_ENTITY);
        assert_eq!(removed.len(), 1);
        assert_eq!((removed[0][0], removed[0][2]), (0x777, entity));
        assert_eq!(
            calls_to(&e, ENTITY_FN_C9C140),
            vec![vec![entity, list[0] + 8]]
        );
        e.mem.set_u8(list[0] + 0xc, 0);
        e.call(0x00e4_4cc0, &args![list[0]]);
        assert_eq!(calls_to(&e, WORLD_REMOVE_ENTITY).len(), 2);
        assert_eq!(calls_to(&e, ENTITY_FN_C9C140).len(), 1);
    }

    #[test]
    fn releasing_the_entity_removes_it_from_a_world_it_is_in() {
        let (mut e, _) = world();
        let (_, list) = gun_with(&mut e, &[(0x00, 0.0)]);
        e.call(0x00e4_4cf0, &args![list[0]]);
        assert!(calls_to(&e, REMOVE_REFERENCE).is_empty());
        let entity = e.mem.alloc(0x20);
        e.mem.set_u32(list[0] + 0x10, entity);
        e.call(0x00e4_4cf0, &args![list[0]]);
        assert!(calls_to(&e, WORLD_REMOVE_ENTITY).is_empty());
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![entity]]);
        assert_eq!(e.mem.u32(list[0] + 0x10), 0);
        e.mem.set_u32(entity + 8, 0x777);
        e.mem.set_u32(list[0] + 0x10, entity);
        e.call(0x00e4_4cf0, &args![list[0]]);
        assert_eq!(calls_to(&e, WORLD_REMOVE_ENTITY).len(), 1);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE).len(), 2);
        // The thunk does the same.
        e.mem.set_u32(list[0] + 0x10, entity);
        e.call(0x00e4_4ea0, &args![list[0]]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE).len(), 3);
    }

    #[test]
    fn ageing_adds_the_step_unless_the_flag_is_set() {
        let (mut e, _) = world();
        let (_, list) = gun_with(&mut e, &[(0x00, 1.0), (0x01, 1.0)]);
        e.call(0x00e4_4d20, &args![list[0], 0.25f32]);
        e.call(0x00e4_4d20, &args![list[1], 0.25f32]);
        assert_eq!(e.mem.f32(list[0] + 0x14), 1.25);
        assert_eq!(e.mem.f32(list[1] + 0x14), 1.0);
        assert_eq!(
            calls_to(&e, SLOT_AGE),
            vec![vec![list[0], 0.25f32.to_bits()]]
        );
    }

    #[test]
    fn projectiles_older_than_the_limit_take_their_first_step() {
        let (mut e, _) = world();
        let (gun, list) = gun_with(&mut e, &[(0, 1.0), (0, 5.0), (0, 3.0), (0, 2.0)]);
        e.call(0x00e4_4da0, &args![gun, 2.0f32]);
        assert_eq!(calls_to(&e, SLOT_FIRST), vec![vec![list[1]], vec![list[2]]]);
    }

    #[test]
    fn every_projectile_ages_and_its_second_flag_is_cleared() {
        let (mut e, _) = world();
        let (gun, list) = gun_with(&mut e, &[(0x02, 1.0), (0x03, 2.0), (0x00, 3.0)]);
        e.call(0x00e4_4de0, &args![gun, 0.5f32]);
        assert_eq!(e.mem.f32(list[0] + 0x14), 1.5);
        assert_eq!(e.mem.f32(list[1] + 0x14), 2.0);
        assert_eq!(e.mem.f32(list[2] + 0x14), 3.5);
        e.call(0x00e4_4e10, &args![gun]);
        assert_eq!(
            list.iter().map(|p| e.mem.u8(p + 0xc)).collect::<Vec<_>>(),
            vec![0, 1, 0]
        );
    }

    #[test]
    fn finished_projectiles_are_released_by_moving_the_last_into_their_place() {
        let (mut e, _) = world();
        let flags = [1, 0, 1, 1, 0];
        let entries: Vec<(u8, f32)> = flags.iter().map(|f| (*f, 0.0)).collect();
        let (gun, list) = gun_with(&mut e, &entries);
        e.call(0x00e4_4ef0, &args![gun]);
        assert_eq!(
            calls_to(&e, REMOVE_REFERENCE),
            vec![vec![list[0]], vec![list[2]], vec![list[3]]]
        );
        assert_eq!(e.mem.u32(gun + 0x30), 2);
        let array = e.mem.u32(gun + 0x2c);
        assert_eq!((e.mem.u32(array), e.mem.u32(array + 4)), (list[4], list[1]));
    }

    #[test]
    fn update_ages_clears_and_drops_the_finished() {
        let (mut e, _) = world();
        let (gun, list) = gun_with(&mut e, &[(0x03, 1.0), (0x02, 1.0)]);
        e.call(
            0x00e4_4f30,
            &args![gun, 0.5f32, 0u32, 0u32, 0u32, 0u32, 0u32],
        );
        assert_eq!(e.mem.f32(list[1] + 0x14), 1.5);
        assert_eq!(e.mem.f32(list[0] + 0x14), 1.0);
        assert_eq!(e.mem.u8(list[1] + 0xc), 0);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![list[0]]]);
        assert_eq!(e.mem.u32(gun + 0x30), 1);
        let array = e.mem.u32(gun + 0x2c);
        assert_eq!(e.mem.u32(array), list[1]);
    }

    #[test]
    fn clearing_and_resetting_step_and_release_every_projectile() {
        for address in [0x00e4_4eb0, 0x00e4_4f50] {
            let (mut e, _) = world();
            let (gun, list) = gun_with(&mut e, &[(0x00, 0.0), (0x01, 0.0)]);
            if address == 0x00e4_4eb0 {
                e.call(address, &args![gun]);
            } else {
                e.call(address, &args![gun, 0u32]);
            }
            assert_eq!(calls_to(&e, SLOT_FIRST), vec![vec![list[0]]]);
            assert_eq!(
                calls_to(&e, REMOVE_REFERENCE),
                vec![vec![list[0]], vec![list[1]]]
            );
            assert_eq!(e.mem.u32(gun + 0x30), 0);
        }
        let (mut e, _) = world();
        let (gun, _) = gun_with(&mut e, &[]);
        e.mem.set_u32(gun + 0x30, 0xffff_fffe);
        e.call(0x00e4_4eb0, &args![gun]);
        assert_eq!(e.mem.u32(gun + 0x30), 0);
    }

    #[test]
    fn the_part_destructor_walks_the_vtables_down_to_the_base_object() {
        let (mut e, _) = world();
        let (gun, _) = gun_with(&mut e, &[]);
        let entity = e.mem.alloc(0x20);
        e.mem.set_u32(gun + 0x10, entity);
        let seen = Rc::new(RefCell::new(vec![]));
        let sink = Rc::clone(&seen);
        e.register_double(REMOVE_REFERENCE, move |e, _| {
            sink.borrow_mut().push((e.mem.u32(gun), e.mem.u32(gun + 8)));
            Ret::default()
        });
        e.call(0x00e4_4e70, &args![gun]);
        assert_eq!(
            *seen.borrow(),
            vec![(GUN_PARTS_VTABLE, GUN_SECOND_BASE_VTABLE)]
        );
        assert_eq!(e.mem.u32(gun), BASE_OBJECT_VTABLE);
        assert_eq!(e.mem.u32(gun + 8), SECOND_BASE_FINAL_VTABLE);
        assert_eq!(e.mem.u32(gun + 0x10), 0);
    }

    #[test]
    fn constructors_set_the_vtable_and_an_empty_array() {
        let (mut e, _) = world();
        let gun = e.mem.alloc(0x60);
        e.mem.set_u32(gun + 0x2c, 0x99);
        assert_eq!(
            e.call(0x00e4_4fc0, &args![gun, 0xaau32, 0xbbu32]).u32(),
            gun
        );
        assert_eq!(calls_to(&e, FIRST_PERSON_GUN_CONSTRUCTOR), vec![vec![gun]]);
        assert_eq!(e.mem.u32(gun), GUN_VTABLE);
        assert_eq!(e.mem.f32(gun + 0x24), 0.3);
        assert_eq!(e.mem.f32(gun + 0x28), 0.0);
        assert_eq!(e.mem.u32(gun + 0x20), 5);
        assert_eq!(e.mem.u32(gun + 0x2c), 0);
        assert_eq!(e.mem.u32(gun + 0x30), 0);
        assert_eq!(e.mem.u32(gun + 0x34), 0x8000_0000);
        assert_eq!((e.mem.u32(gun + 0x38), e.mem.u32(gun + 0x3c)), (0xaa, 0xbb));
        let other = e.mem.alloc(0x60);
        e.mem.set_u32(other + 0x2c, 0x99);
        e.mem.set_u32(other + 0x30, 0x99);
        assert_eq!(e.call(0x00e4_5010, &args![other, 0xccu32]).u32(), other);
        assert_eq!(calls_to(&e, FIRST_PERSON_GUN_COPY), vec![vec![other, 0xcc]]);
        assert_eq!(e.mem.u32(other), GUN_VTABLE);
        assert_eq!(e.mem.u32(other + 0x2c), 0);
        assert_eq!(e.mem.u32(other + 0x30), 0);
        assert_eq!(e.mem.u32(other + 0x34), 0x8000_0000);
    }

    #[test]
    fn the_gun_destructor_frees_an_owned_array_and_runs_the_base_destructor() {
        for (capacity, frees) in [(4u32, true), (0x8000_0004, false)] {
            let (mut e, allocator) = world();
            let (gun, list) = gun_with(&mut e, &[(0x00, 0.0), (0x00, 0.0)]);
            let data = e.mem.u32(gun + 0x2c);
            e.mem.set_u32(gun + 0x34, capacity);
            e.call(0x00e4_5040, &args![gun]);
            let freed = calls_to(&e, SLOT_FREE);
            if frees {
                assert_eq!(freed, vec![vec![allocator, data, 16]]);
            } else {
                assert!(freed.is_empty());
            }
            assert_eq!(
                calls_to(&e, REMOVE_REFERENCE),
                vec![vec![list[0]], vec![list[1]]]
            );
            assert_eq!(e.mem.u32(gun), GUN_VTABLE);
            assert_eq!(e.mem.u32(gun + 0x2c), 0);
            assert_eq!(e.mem.u32(gun + 0x30), 0);
            assert_eq!(e.mem.u32(gun + 0x34), 0x8000_0000);
            assert_eq!(calls_to(&e, FIRST_PERSON_GUN_DESTRUCTOR), vec![vec![gun]]);
        }
    }

    #[test]
    fn the_second_base_thunk_and_the_flag_setter() {
        let (mut e, _) = world();
        let object = e.mem.alloc(0x30);
        e.mem.set_u32(object, SECOND_BASE_TABLE);
        e.call(0x00e4_5090, &args![object + 8, 0x42u32]);
        assert_eq!(calls_to(&e, SLOT_SECOND), vec![vec![object, 0x42]]);
        e.mem.set_u8(object + 0xc, 0x01);
        e.call(0x00e4_50b0, &args![object, 0u32]);
        assert_eq!(e.mem.u8(object + 0xc), 0x03);
    }

    /// A projectile body: the first rotation row at `+0xf0`, the other two rows and the
    /// translation after it, the velocity at `+0x1b0`.
    fn body(e: &mut Engine, velocity: [f32; 4]) -> u32 {
        let body = e.mem.alloc(0x200);
        let rows = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [5.0, 6.0, 7.0, 0.0],
        ];
        for (index, row) in rows.into_iter().enumerate() {
            for (lane, value) in row.into_iter().enumerate() {
                e.mem
                    .set_f32(body + 0xf0 + 0x10 * index as u32 + 4 * lane as u32, value);
            }
        }
        for (lane, value) in velocity.into_iter().enumerate() {
            e.mem.set_f32(body + 0x1b0 + 4 * lane as u32, value);
        }
        body
    }

    type Transforms = Rc<RefCell<Vec<[[f32; 4]; 4]>>>;
    type Turns = Rc<RefCell<Vec<(Vec4, f32)>>>;

    /// Doubles for the three callees of `flyTrue`: `setAxisAngle` logs the axis and angle it is
    /// given, `setMul` writes `10 + row` into the lanes of the product's rows, and
    /// `setTransform` logs the transform it is given.
    fn fly_doubles(e: &mut Engine) -> (Transforms, Turns) {
        let turns = Turns::default();
        let turn_sink = Rc::clone(&turns);
        e.register_double(ROTATION_SET_AXIS_ANGLE, move |e, a| {
            turn_sink
                .borrow_mut()
                .push((load_vec(e, a[1]), f32::from_bits(a[2])));
            Ret::default()
        });
        e.register(MATRIX_SET_MUL, |e, a| {
            for row in 0..3u32 {
                for lane in 0..4u32 {
                    e.mem
                        .set_f32(a[0] + 0x10 * row + 4 * lane, 10.0 + row as f32);
                }
            }
            Ret::default()
        });
        let log = Transforms::default();
        let sink = Rc::clone(&log);
        e.register_double(RIGID_BODY_SET_TRANSFORM, move |e, a| {
            let transform = std::array::from_fn(|row| {
                std::array::from_fn(|lane| e.mem.f32(a[1] + 0x10 * row as u32 + 4 * lane as u32))
            });
            sink.borrow_mut().push(transform);
            Ret::default()
        });
        (log, turns)
    }

    #[test]
    fn fly_true_turns_the_rotation_toward_the_velocity() {
        let (mut e, _) = world();
        let (transforms, turns) = fly_doubles(&mut e);
        let body = body(&mut e, [0.0, 2.0, 0.0, 0.0]);
        e.call(0x00e4_4990, &args![body, 1.0f32, 0.1f32]);
        let rotations = calls_to(&e, ROTATION_SET_AXIS_ANGLE);
        assert_eq!(rotations.len(), 1);
        assert_eq!(turns.borrow().len(), 1);
        let (axis, angle) = turns.borrow()[0];
        for (got, want) in axis.iter().zip([0.0, 0.0, -1.0]) {
            assert!((got - want).abs() < 1e-5, "{axis:?}");
        }
        assert!((angle - -0.5).abs() < 1e-5, "{angle}");
        let products = calls_to(&e, MATRIX_SET_MUL);
        assert_eq!(products.len(), 1);
        assert_eq!(products[0][1], rotations[0][0]);
        assert_eq!(
            *transforms.borrow(),
            vec![[[10.0; 4], [11.0; 4], [12.0; 4], [5.0, 6.0, 7.0, 0.0]]]
        );
        assert_eq!(calls_to(&e, RIGID_BODY_SET_TRANSFORM)[0][0], body);
    }

    #[test]
    fn fly_true_leaves_a_slow_or_aligned_projectile_alone() {
        let (mut e, _) = world();
        let (transforms, turns) = fly_doubles(&mut e);
        let slow = body(&mut e, [0.0, 2.0, 0.0, 0.0]);
        e.call(0x00e4_4990, &args![slow, 2.0f32, 0.1f32]);
        e.call(0x00e4_4990, &args![slow, f32::NAN, 0.1f32]);
        let aligned = body(&mut e, [3.0, 0.0, 0.0, 0.0]);
        e.call(0x00e4_4990, &args![aligned, 1.0f32, 0.1f32]);
        let still = body(&mut e, [0.0; 4]);
        e.call(0x00e4_4990, &args![still, -1.0f32, 0.1f32]);
        assert!(transforms.borrow().is_empty());
        assert!(turns.borrow().is_empty());
    }
}
