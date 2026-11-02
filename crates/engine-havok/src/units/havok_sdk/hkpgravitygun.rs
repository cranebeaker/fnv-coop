//! `Havok SDK/hkpgravitygun.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The gun (an `hkpGravityGun`, Xbox PDB, derived from `hkpFirstPersonGun`) keeps: listeners
//! (pointer `+0x14`, count `+0x18`), the held bodies (pointer `+0x20`, count `+0x24`,
//! capacity and flags `+0x28`), the number of bodies it may hold at once `+0x2c`, the mass
//! limit `+0x30`, the range `+0x34`, the pull impulse `+0x38`, the push impulse `+0x3c`, and
//! two vectors at `+0x40` (origin) and `+0x50` (step) along which held bodies are lined up.
//! A body (`hkpRigidBody`) has its motion at `+0xe0` (vtable slots `0x40` set the linear and
//! `0x44` the angular velocity, `0x4c` applies a linear impulse), its position at `+0x140`,
//! its velocities at `+0x1b0` and `+0x1c0` and `w` of the vector at `+0x1a0` is read as a
//! mass-related float.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpgskmanifoldutil::{add, load_vec, mul, store_vec, sub, sum3};

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkVector4::setTransformedPos` and `setTransformedInversePos` (Xbox PDB; `this` = the
/// output, then `(transform, vector)`).
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
const SET_TRANSFORMED_INVERSE_POS: u32 = 0x00c7_f690;
/// `hkpMotion::getMass` (Xbox PDB; `this` = the motion; the mass in `ST0`).
const GET_MASS: u32 = 0x00c8_6430;
/// `hkpWorld::castRay` (Xbox PDB; `this` = the world, then `(ray input, collector)`).
const CAST_RAY: u32 = 0x00c9_1f70;
/// `hkpEntity::activate` (Xbox PDB; `this` = the entity).
const ACTIVATE: u32 = 0x00c9_c1d0;
/// `00c90510` / `00c905b0` (`this` = the object): add and remove a reference, unnamed in the
/// engine map. `00c90940` (cdecl `(allocator, array, element size)`) grows an `hkArray`.
const ADD_REFERENCE: u32 = 0x00c9_0510;
const RELEASE_REFERENCE: u32 = 0x00c9_05b0;
const ARRAY_GROW: u32 = 0x00c9_0940;
/// `00ca88e0` (`this` = a body, then one float): unnamed in the engine map.
const SET_BODY_FLOAT: u32 = 0x00ca_88e0;
/// `hkpRigidBody::setLinearVelocity` (Xbox PDB; `this` = the body, then the vector).
const SET_LINEAR_VELOCITY: u32 = 0x0056_16d0;
/// `hkpFirstPersonGun::hkpFirstPersonGun` (Xbox PDB), its variant with a flag, and
/// `hkpFirstPersonGun::~hkpFirstPersonGun` (Xbox PDB), each with `this` the gun.
const BASE_CONSTRUCT: u32 = 0x00e4_6660;
const BASE_CONSTRUCT_FLAGGED: u32 = 0x00e4_66a0;
const BASE_DESTRUCT: u32 = 0x00e4_66e0;
/// `hkStringPtr::operator=` (Xbox PDB; `this` = the string, then the text).
const STRING_ASSIGN: u32 = 0x00cb_0c10;

/// The gun's vtable (`010ec090`), its name text (`010ec080`) and the ray hit collector's vtable
/// (`01034ec4`, `addRayHit` in the Xbox PDB).
const VTABLE: u32 = 0x010e_c090;
const NAME_TEXT: u32 = 0x010e_c080;
const RAY_COLLECTOR_VTABLE: u32 = 0x0103_4ec4;
/// Floats and doubles the code reads from the exe.
const ZERO: u32 = 0x0101_1d78;
const DEFAULT_SCORE: u32 = 0x0102_1f44;
const TENTH: u32 = 0x0101_ffa0;
const ONE: u32 = 0x0101_2070;
const HALF_DOUBLE: u32 = 0x0101_1588;
const HUNDRED: u32 = 0x0101_7a40;
const HUNDREDTH: u32 = 0x0101_6408;
const FOUR_TENTHS: u32 = 0x0102_90c0;
const TOLERANCE: u32 = 0x0101_7d00;
const BLEND: u32 = 0x0103_0ff0;
const ONES: u32 = 0x010e_c7f0;
const ABS_MASK: u32 = 0x0101_7d10;

// Translated from 00e45a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGravityGun::pickBody` (Xbox PDB): `this` = the gun, then `(reference, exclude, world)`.
/// Looks through the bodies of the world's two lists (`world + 0x28`, then `world + 0x34`;
/// each a list of objects whose `+0x48` is a list of bodies) and answers the best body or 0.
/// A body counts when its position in the reference's frame (`reference` is a transform,
/// `setTransformedInversePos`) has x of at least 0.1 (a NaN passes) and at most the gun's
/// range, it is not `exclude`, and its score is not above the best so far (initially 0.35,
/// a NaN passes). The score is `(|z| + |y|) / (x + 1)` in x87 doubles, plus 0.1 when the
/// body's mass is above the gun's mass limit. A body the gun already holds is skipped. The
/// last test is a ray from the reference's translation (`reference + 0x30`) to the body's
/// position: the body is taken when the ray hits nothing or hits the body's own object
/// (`body + 0x10`). The ray structures are built in the frame and reused from one cast to
/// the next.
pub fn pick_body(e: &mut Engine, this: Ptr, reference: Ptr, exclude: u32, world: Ptr) -> u32 {
    let (gun, reference, world) = (this.addr(), reference.addr(), world.addr());
    e.with_stack(0x100, |e, frame| {
        let f = frame.addr();
        let mut best_score = f32::from_bits(x87_quiet_bits(e.mem.u32(DEFAULT_SCORE)));
        let mut best = 0u32;
        let mut header = world + 0x28;
        for _ in 0..2 {
            let mut outer = 0i32;
            while outer < e.mem.i32(header + 4) {
                let objects = e.mem.u32(header);
                let object = e
                    .mem
                    .u32(objects.wrapping_add((outer as u32).wrapping_mul(4)));
                let mut inner = 0i32;
                while inner < e.mem.i32(object + 0x4c) {
                    let bodies = e.mem.u32(object + 0x48);
                    let body = e
                        .mem
                        .u32(bodies.wrapping_add((inner as u32).wrapping_mul(4)));
                    if let Some(score) =
                        rate_body(e, f, gun, reference, exclude, world, best_score, body)
                    {
                        best_score = score;
                        best = body;
                    }
                    inner += 1;
                }
                outer += 1;
            }
            header = world + 0x34;
        }
        best
    })
}

/// The test of one body in `pick_body`: its score when it is accepted. `f` is the frame
/// that holds the position (`f + 0x40`), the ray input (`f + 0x50`) and the collector
/// (`f + 0x80`, its answer at `f + 0xe0`).
#[allow(clippy::too_many_arguments)]
fn rate_body(
    e: &mut Engine,
    f: u32,
    gun: u32,
    reference: u32,
    exclude: u32,
    world: u32,
    best_score: f32,
    body: u32,
) -> Option<f32> {
    e.call(
        SET_TRANSFORMED_INVERSE_POS,
        &args![f + 0x40, reference, body + 0x140],
    );
    let x = e.mem.f32(f + 0x40);
    if f64::from(x) < e.mem.f64(TENTH) {
        return None;
    }
    if e.mem.f32(gun + 0x34) < x || body == exclude {
        return None;
    }
    let (y, z) = (e.mem.f32(f + 0x44).abs(), e.mem.f32(f + 0x48).abs());
    let sum = (f64::from(z) + f64::from(y)) as f32;
    let mut score = (f64::from(sum) / (f64::from(x) + e.mem.f64(ONE))) as f32;
    let mass = e.call(GET_MASS, &args![body + 0xe0]).f64() as f32;
    if e.mem.f32(gun + 0x30) < mass {
        score = (f64::from(score) + e.mem.f64(TENTH)) as f32;
    }
    if best_score < score {
        return None;
    }
    let held = e.mem.i32(gun + 0x24);
    for index in 0..held {
        let list = e.mem.u32(gun + 0x20);
        if e.mem.u32(list.wrapping_add((index as u32).wrapping_mul(4))) == body {
            return None;
        }
    }
    e.mem.set_u32(f + 0xa4, 0xffff_ffff);
    e.mem.set_f32(f + 0xa0, 1.0);
    e.mem.set_u32(f + 0xb0, 0xffff_ffff);
    e.mem.set_f32(f + 0x84, 1.0);
    copy_words(e, reference + 0x30, f + 0x50, 4);
    copy_words(e, body + 0x140, f + 0x60, 4);
    e.mem.set_u32(f + 0x80, RAY_COLLECTOR_VTABLE);
    e.mem.set_u32(f + 0xe0, 0);
    e.mem.set_u32(f + 0xd0, 0);
    e.mem.set_u8(f + 0x70, 0);
    e.mem.set_u32(f + 0x74, 0);
    e.call(CAST_RAY, &args![world, f + 0x50, f + 0x80]);
    let hit = e.mem.u32(f + 0xe0);
    if hit == 0 || hit == body + 0x10 {
        Some(score)
    } else {
        None
    }
}

/// A raw copy of `words` dwords.
fn copy_words(e: &mut Engine, from: u32, to: u32, words: u32) {
    for word in 0..words {
        let value = e.mem.u32(from.wrapping_add(4 * word));
        e.mem.set_u32(to.wrapping_add(4 * word), value);
    }
}

/// `CMPLEPS` of `|difference|` (the `ANDPS` mask at `01017d10`) against the tolerance vector
/// and `MOVMSKPS`: true when all four lanes are within the tolerance (a NaN is not).
fn all_within(e: &Engine, difference: [f32; 4], tolerance: f32) -> bool {
    difference.iter().enumerate().all(|(lane, value)| {
        let mask = e.mem.u32(ABS_MASK + 4 * lane as u32);
        f32::from_bits(value.to_bits() & mask) <= tolerance
    })
}

// Translated from 00e45c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00e45c90` (no map name, unconfirmed): `this` = the gun, then `(step, transform,
/// body)`; `step` is a float. For each held body `i`: the target point is `transform` applied to
/// `origin + i * step` (the gun's `+0x40` and `+0x50`), and the velocity that would reach it
/// in `0.4 / step` is `(target - position) * (0.4 / step) + 0.2 * velocity + 0.8 * body
/// velocity` (the `0.8` and `0.2 = 1 - 0.8` come from the exe, vectors at `+0x1b0`). When
/// that differs from the body's velocity by more than 0.001 in any lane the body is
/// activated and its motion's slot `0x40` sets it. The angular velocity (`+0x1c0`), scaled by
/// 0.8, is set through slot `0x44` in the same way when the square of its length is above 1.
pub fn fn_00e45c90(e: &mut Engine, this: Ptr, step: f32, transform: Ptr, body: Ptr) {
    let (gun, transform, body) = (this.addr(), transform.addr(), body.addr());
    if e.mem.i32(gun + 0x24) <= 0 {
        return;
    }
    let inverse = (e.mem.f64(FOUR_TENTHS) / f64::from(step)) as f32;
    let tolerance = e.mem.f32(TOLERANCE);
    e.with_stack(0x80, |e, frame| {
        let f = frame.addr();
        let mut index = 0i32;
        while index < e.mem.i32(gun + 0x24) {
            let origin = load_vec(e, gun + 0x40);
            let spacing = load_vec(e, gun + 0x50);
            let point = add(mul([index as f32; 4], spacing), origin);
            store_vec(e, f + 0x40, point);
            e.call(SET_TRANSFORMED_POS, &args![f + 0x50, transform, f + 0x40]);
            let list = e.mem.u32(gun + 0x20);
            let entity = e.mem.u32(list.wrapping_add((index as u32).wrapping_mul(4)));
            let velocity = load_vec(e, entity + 0x1b0);
            let blend = e.mem.f32(BLEND);
            let ones = load_vec(e, ONES);
            let mixed = add(
                mul(sub(ones, [blend; 4]), velocity),
                mul(load_vec(e, body + 0x1b0), [blend; 4]),
            );
            let toward = sub(load_vec(e, f + 0x50), load_vec(e, entity + 0x140));
            let wanted = add(mul(toward, [inverse; 4]), mixed);
            store_vec(e, f + 0x20, wanted);
            let current = load_vec(e, entity + 0x1b0);
            if !all_within(e, sub(wanted, current), tolerance) {
                e.call(ACTIVATE, &args![entity]);
                e.vcall(entity + 0xe0, 0x40, &args![f + 0x20]);
            }
            let spin = mul([blend; 4], load_vec(e, entity + 0x1c0));
            let squared = sum3(mul(spin, spin));
            store_vec(e, f + 0x60, spin);
            if 1.0 < squared {
                let current = load_vec(e, entity + 0x1c0);
                if !all_within(e, sub(spin, current), tolerance) {
                    e.call(ACTIVATE, &args![entity]);
                    e.vcall(entity + 0xe0, 0x44, &args![f + 0x60]);
                }
            }
            index += 1;
        }
    });
}

// Translated from 00e45e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00e45e60` (no map name, unconfirmed): `this` = the gun, then `index`. Takes the held
/// body at `index` out of the list (the later ones move down), calls `00ca88e0` on it with
/// its `+0x1ac` times 0.01, releases the reference and answers the body.
pub fn fn_00e45e60(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    let gun = this.addr();
    let list = e.mem.u32(gun + 0x20);
    let start = list.wrapping_add((index as u32).wrapping_mul(4));
    let body = e.mem.u32(start);
    let count = e.mem.i32(gun + 0x24).wrapping_sub(1);
    e.mem.set_u32(gun + 0x24, count as u32);
    let bytes = count.wrapping_sub(index).wrapping_mul(4);
    if bytes > 0 {
        let moves = ((bytes - 1) as u32 >> 2) + 1;
        for step in 0..moves {
            let word = e.mem.u32(start.wrapping_add(4 + 4 * step));
            e.mem.set_u32(start.wrapping_add(4 * step), word);
        }
    }
    let scaled = (f64::from(e.mem.f32(body + 0x1ac)) * e.mem.f64(HUNDREDTH)) as f32;
    e.call(SET_BODY_FLOAT, &args![body, x87_quiet(scaled).to_bits()]);
    e.call(RELEASE_REFERENCE, &args![body]);
    body
}

// Translated from 00e45ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGravityGun::gravityGunPush` (Xbox PDB): `this` = the gun, then `(transform, body,
/// world)`. With a held body: lets it go (`fn_00e45e60`, index 0) and gives it the velocity
/// `body velocity + push impulse * transform's first vector`, with 0.5 added to y, through
/// `setLinearVelocity`; its byte `+0x2a` becomes 6. Without one: picks a body
/// (`pickBody(transform, body, world)`) and, if there is one, activates it and applies the
/// pull impulse (`+0x38`) times the vector as a linear impulse (motion slot `0x4c`).
pub fn gravity_gun_push(e: &mut Engine, this: Ptr, transform: Ptr, body: Ptr, world: Ptr) {
    let gun = this.addr();
    if e.mem.u32(gun + 0x24) != 0 {
        let held = fn_00e45e60(e, this, 0);
        let impulse = e.mem.f32(gun + 0x3c);
        let scaled = mul([impulse; 4], load_vec(e, transform.addr()));
        let velocity = load_vec(e, body.addr() + 0x1b0);
        e.with_stack(0x20, |e, frame| {
            let vector = frame.addr();
            store_vec(e, vector, scaled);
            let lifted = (f64::from(e.mem.f32(vector + 4)) + e.mem.f64(HALF_DOUBLE)) as f32;
            e.mem.set_f32(vector + 4, lifted);
            let total = add(velocity, load_vec(e, vector));
            store_vec(e, vector, total);
            e.call(SET_LINEAR_VELOCITY, &args![held, vector]);
        });
        e.mem.set_u8(held + 0x2a, 6);
        return;
    }
    let picked = pick_body(e, this, transform, body.addr(), world);
    if picked != 0 {
        let factor = e.mem.f32(gun + 0x38);
        apply_impulse(e, picked, mul([factor; 4], load_vec(e, transform.addr())));
    }
}

/// Activates `body` and applies the linear impulse `vector` through its motion's slot `0x4c`.
fn apply_impulse(e: &mut Engine, body: u32, vector: [f32; 4]) {
    e.with_stack(0x10, |e, frame| {
        store_vec(e, frame.addr(), vector);
        e.call(ACTIVATE, &args![body]);
        e.vcall(body + 0xe0, 0x4c, &args![frame.addr()]);
    });
}

// Translated from 00e45fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00e45fb0` (no map name, unconfirmed), vtable slot: `this` = the gun and three unused
/// arguments. Lets the first held body go (`fn_00e45e60`, index 0) when there is one.
pub fn fn_00e45fb0(e: &mut Engine, this: Ptr, _a: u32, _b: u32, _c: u32) {
    if e.mem.u32(this.addr() + 0x24) != 0 {
        fn_00e45e60(e, this, 0);
    }
}

// Translated from 00e45fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGravityGun::reset` (Xbox PDB), vtable slot: `this` = the gun and one unused argument.
/// Releases the reference of every held body and empties the list.
pub fn reset(e: &mut Engine, this: Ptr, _unused: u32) {
    let gun = this.addr();
    let mut index = 0i32;
    while index < e.mem.i32(gun + 0x24) {
        let list = e.mem.u32(gun + 0x20);
        let body = e.mem.u32(list.wrapping_add((index as u32).wrapping_mul(4)));
        e.call(RELEASE_REFERENCE, &args![body]);
        index += 1;
    }
    e.mem.set_u32(gun + 0x24, 0);
}

// Translated from 00e46000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGravityGun::takeObject` (Xbox PDB): `this` = the gun, then the body. Adds a reference,
/// grows the held list when it is full (`00c90940(allocator, list, 4)`), appends the body and
/// calls `00ca88e0` on it with its `+0x1ac` times 100.
pub fn take_object(e: &mut Engine, this: Ptr, body: u32) {
    let list = this.addr() + 0x20;
    e.call(ADD_REFERENCE, &args![body]);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(list + 8) & 0x3fff_ffff;
    if e.mem.u32(list + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_GROW, &args![allocator, list, 4u32]);
    }
    let data = e.mem.u32(list);
    let size = e.mem.u32(list + 4);
    e.mem.set_u32(data.wrapping_add(size.wrapping_mul(4)), body);
    let size = e.mem.u32(list + 4).wrapping_add(1);
    e.mem.set_u32(list + 4, size);
    let scaled = (f64::from(e.mem.f32(body + 0x1ac)) * e.mem.f64(HUNDRED)) as f32;
    e.call(SET_BODY_FLOAT, &args![body, x87_quiet(scaled).to_bits()]);
}

// Translated from 00e46080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGravityGun::gravityGunPull` (Xbox PDB): `this` = the gun, then `(transform, body,
/// world)`. Picks a body (`pickBody(transform, body, world)`). If the gun's mass limit times
/// the body's `+0x1ac` is not below 1 and the gun holds fewer than it may, the body is taken
/// (`takeObject`); otherwise it is activated and pushed away with the linear impulse minus
/// the pull impulse (`+0x38`) times the transform's first vector.
pub fn gravity_gun_pull(e: &mut Engine, this: Ptr, transform: Ptr, body: Ptr, world: Ptr) {
    let gun = this.addr();
    let picked = pick_body(e, this, transform, body.addr(), world);
    if picked == 0 {
        return;
    }
    let product = f64::from(e.mem.f32(gun + 0x30)) * f64::from(e.mem.f32(picked + 0x1ac));
    let light = product < e.mem.f64(ONE);
    if !light && e.mem.i32(gun + 0x24) < e.mem.i32(gun + 0x2c) {
        take_object(e, this, picked);
        return;
    }
    let factor = -e.mem.f32(gun + 0x38);
    apply_impulse(e, picked, mul([factor; 4], load_vec(e, transform.addr())));
}

// Translated from 00e46140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGravityGun::stepGun` (Xbox PDB), vtable slot: `this` = the gun, then `(step, world,
/// body, transform, push, pull)`; `push` and `pull` are bytes. With `push`: `gravityGunPush`
/// and then every listener's slot `0x14` is called with `(body + 0x120 vector, 0)`. Without
/// it and with `pull`: when the gun holds as many bodies as it may, `fn_00e45fb0` lets one go
/// and the listeners get `(body + 0x120 vector, 0)`; otherwise `gravityGunPull` runs and the
/// listeners get `(transform + 0x30 vector, body + 0x120 vector)`. At the end
/// `fn_00e45c90(step, transform, body)` moves the held bodies. The vectors are copies.
pub fn step_gun(
    e: &mut Engine,
    this: Ptr,
    step: f32,
    world: Ptr,
    body: Ptr,
    transform: Ptr,
    push: u8,
    pull: u8,
) {
    let gun = this.addr();
    e.with_stack(0x40, |e, frame| {
        let f = frame.addr();
        if push != 0 {
            gravity_gun_push(e, this, transform, body, world);
            copy_words(e, body.addr() + 0x120, f + 0x10, 4);
            notify_listeners(e, gun, f + 0x10, 0);
        } else if pull != 0 {
            if e.mem.u32(gun + 0x24) == e.mem.u32(gun + 0x2c) {
                fn_00e45fb0(e, this, transform.addr(), body.addr(), world.addr());
                copy_words(e, body.addr() + 0x120, f + 0x20, 4);
                notify_listeners(e, gun, f + 0x20, 0);
            } else {
                gravity_gun_pull(e, this, transform, body, world);
                copy_words(e, transform.addr() + 0x30, f + 0x20, 4);
                copy_words(e, body.addr() + 0x120, f + 0x10, 4);
                notify_listeners(e, gun, f + 0x20, f + 0x10);
            }
        }
    });
    fn_00e45c90(e, this, step, transform, body);
}

/// Calls slot `0x14` of every listener with the two arguments.
fn notify_listeners(e: &mut Engine, gun: u32, first: u32, second: u32) {
    let mut index = 0i32;
    while index < e.mem.i32(gun + 0x18) {
        let list = e.mem.u32(gun + 0x14);
        let listener = e.mem.u32(list.wrapping_add((index as u32).wrapping_mul(4)));
        e.vcall(listener, 0x14, &args![first, second]);
        index += 1;
    }
}

// Translated from 00e46260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGravityGun::hkpGravityGun` (Xbox PDB; `this` in ECX): runs the base constructor, sets
/// the vtable, the held list (empty, capacity flags `0x80000000`), the floats `+0x30`,
/// `+0x34`, `+0x38`, `+0x3c` from the exe's constants (the same one for `+0x34` and
/// `+0x3c`), `+0x2c` = 10, the byte `+8` = 3, assigns the name text (`010ec080`) to the
/// string at `+0xc` and sets the vectors at `+0x40` and `+0x50`. Answers `this`.
pub fn hkp_gravity_gun(e: &mut Engine, this: Ptr) -> Ptr {
    let t = this.addr();
    e.call(BASE_CONSTRUCT, &args![t]);
    e.mem.set_u32(t, VTABLE);
    e.mem.set_u32(t + 0x28, 0x8000_0000);
    e.mem.set_u32(t + 0x20, 0);
    e.mem.set_u32(t + 0x24, 0);
    for (source, field) in [
        (0x0102_226c, 0x30),
        (0x0101_b268, 0x34),
        (0x0101_6410, 0x38),
        (0x0101_b268, 0x3c),
    ] {
        let bits = x87_quiet_bits(e.mem.u32(source));
        e.mem.set_u32(t + field, bits);
    }
    e.mem.set_u32(t + 0x2c, 10);
    e.mem.set_u8(t + 8, 3);
    e.call(STRING_ASSIGN, &args![t + 0xc, NAME_TEXT]);
    let zero = e.mem.u32(ZERO);
    for (field, lanes) in [
        (0x40, [0x0101_84f4, 0x0101_8180]),
        (0x50, [ZERO, 0x0101_2068]),
    ] {
        let words = [e.mem.u32(lanes[0]), e.mem.u32(lanes[1]), zero, zero];
        for (lane, word) in words.into_iter().enumerate() {
            e.mem.set_u32(t + field + 4 * lane as u32, word);
        }
    }
    this
}

// Translated from 00e46300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00e46300` (no map name, unconfirmed; `this` in ECX, then a flag): runs the flagged base
/// constructor (`00e466a0(flag)`), sets the vtable, empties the held list and, when the flag
/// is set, the byte `+8` = 3. Answers `this`.
pub fn fn_00e46300(e: &mut Engine, this: Ptr, flag: u32) -> Ptr {
    let t = this.addr();
    e.call(BASE_CONSTRUCT_FLAGGED, &args![t, flag]);
    e.mem.set_u32(t, VTABLE);
    e.mem.set_u32(t + 0x20, 0);
    e.mem.set_u32(t + 0x24, 0);
    e.mem.set_u32(t + 0x28, 0x8000_0000);
    if flag != 0 {
        e.mem.set_u8(t + 8, 3);
    }
    this
}

// Translated from 00e46340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGravityGun::_scalar_deleting_destructor_` (Xbox PDB; `this` in ECX, then `flags`):
/// empties the held list (freeing its block through the memory router's allocator, slot 4,
/// when it is owned), runs the base destructor, and when bit 0 of `flags` is set frees the
/// object with the 16-bit size kept at `+4` (allocator slot 2). Answers `this`.
pub fn deleting_destructor(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    let t = this.addr();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.i32(t + 0x28);
    e.mem.set_u32(t + 0x24, 0);
    if capacity >= 0 {
        let block = e.mem.u32(t + 0x20);
        e.vcall(
            allocator,
            0x10,
            &args![block, (capacity as u32 & 0x3fff_ffff) << 2],
        );
    }
    e.mem.set_u32(t + 0x20, 0);
    e.mem.set_u32(t + 0x28, 0x8000_0000);
    e.call(BASE_DESTRUCT, &args![t]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(t + 4));
        e.vcall(allocator, 8, &args![t, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e4_5a60, pick_body(Ptr, Ptr, u32, Ptr) -> u32),
        entry!(0x00e4_5c90, fn_00e45c90(Ptr, f32, Ptr, Ptr)),
        entry!(0x00e4_5e60, fn_00e45e60(Ptr, i32) -> u32),
        entry!(0x00e4_5ee0, gravity_gun_push(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00e4_5fb0, fn_00e45fb0(Ptr, u32, u32, u32)),
        entry!(0x00e4_5fc0, reset(Ptr, u32)),
        entry!(0x00e4_6000, take_object(Ptr, u32)),
        entry!(0x00e4_6080, gravity_gun_pull(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00e4_6140, step_gun(Ptr, f32, Ptr, Ptr, Ptr, u8, u8)),
        entry!(0x00e4_6260, hkp_gravity_gun(Ptr) -> Ptr),
        entry!(0x00e4_6300, fn_00e46300(Ptr, u32) -> Ptr),
        entry!(0x00e4_6340, deleting_destructor(Ptr, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, logging_object, new_log, Log,
    };
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    const ALLOCATOR: u32 = 0x00f4_0000;
    const MOTION: u32 = 0x00f5_0000;
    const LISTENER: u32 = 0x00f6_0000;
    /// What the vector-taking doubles saw: (address, `this`, the vector).
    type Vectors = Rc<RefCell<Vec<(u32, u32, [f32; 4])>>>;
    /// What a listener heard: its two arguments read as vectors (`None` for a null).
    type Heard = Rc<RefCell<Vec<(Option<[f32; 4]>, Option<[f32; 4]>)>>>;
    /// What the ray cast double saw: the eight floats of the ray and the collector words.
    type Rays = Rc<RefCell<Vec<(Vec<f32>, Vec<u32>)>>>;

    struct Rig {
        e: Engine,
        log: Log,
        vectors: Vectors,
        heard: Heard,
        rays: Rays,
        hit: Rc<Cell<u32>>,
        mass: Rc<Cell<f64>>,
        motion: u32,
        listener: u32,
        allocator: u32,
        router: u32,
    }

    fn read_vec(e: &Engine, address: u32) -> Option<[f32; 4]> {
        (address != 0).then(|| load_vec(e, address))
    }

    fn rig() -> Rig {
        let mut e = engine();
        let log = new_log();
        for (address, value) in [
            (DEFAULT_SCORE, 0.35f32),
            (TOLERANCE, 0.001),
            (BLEND, 0.8),
            (0x0102_226c, 200.0),
            (0x0101_b268, 50.0),
            (0x0101_6410, 100.0),
            (0x0101_84f4, 2.5),
            (0x0101_8180, 0.6),
            (0x0101_2068, 1.0),
        ] {
            e.mem.set_f32(address, value);
        }
        for (address, value) in [
            (TENTH, 0.1f64),
            (ONE, 1.0),
            (HALF_DOUBLE, 0.5),
            (HUNDRED, 100.0),
            (HUNDREDTH, 0.01),
            (FOUR_TENTHS, 0.4),
        ] {
            e.mem.set_f64(address, value);
        }
        for lane in 0..4 {
            e.mem.set_f32(ONES + 4 * lane, 1.0);
            e.mem.set_u32(ABS_MASK + 4 * lane, 0x7fff_ffff);
        }
        for address in [SET_TRANSFORMED_POS, SET_TRANSFORMED_INVERSE_POS] {
            let log = Rc::clone(&log);
            e.register_double(address, move |e, a| {
                log.borrow_mut().push((address, a.to_vec()));
                copy_words(e, a[2], a[0], 4);
                Ret::default()
            });
        }
        let mass = Rc::new(Cell::new(1.0f64));
        let hit = Rc::new(Cell::new(0u32));
        {
            let (log, mass) = (Rc::clone(&log), Rc::clone(&mass));
            e.register_double(GET_MASS, move |_, a| {
                log.borrow_mut().push((GET_MASS, a.to_vec()));
                Ret {
                    st0: mass.get(),
                    ..Ret::default()
                }
            });
        }
        let rays: Rays = Rc::default();
        {
            let (log, rays, hit) = (Rc::clone(&log), Rc::clone(&rays), Rc::clone(&hit));
            e.register_double(CAST_RAY, move |e, a| {
                log.borrow_mut().push((CAST_RAY, a.to_vec()));
                let floats = (0..8).map(|i| e.mem.f32(a[1] + 4 * i)).collect();
                let words = [0u32, 4, 0x20, 0x24, 0x30, 0x50]
                    .iter()
                    .map(|offset| e.mem.u32(a[2] + offset))
                    .collect();
                rays.borrow_mut().push((floats, words));
                e.mem.set_u32(a[2] + 0x60, hit.get());
                Ret::default()
            });
        }
        let vectors: Vectors = Rc::default();
        let motion = e.mem.alloc(0x50);
        for slot in [0x40, 0x44, 0x4c] {
            e.mem.set_u32(motion + slot, MOTION + slot);
        }
        for address in [
            SET_LINEAR_VELOCITY,
            MOTION + 0x40,
            MOTION + 0x44,
            MOTION + 0x4c,
        ] {
            let vectors = Rc::clone(&vectors);
            e.register_double(address, move |e, a| {
                vectors
                    .borrow_mut()
                    .push((address, a[0], load_vec(e, a[1])));
                Ret::default()
            });
        }
        let heard: Heard = Rc::default();
        let listener_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(listener_vtable + 0x14, LISTENER + 0x14);
        {
            let heard = Rc::clone(&heard);
            e.register_double(LISTENER + 0x14, move |e, a| {
                heard
                    .borrow_mut()
                    .push((read_vec(e, a[1]), read_vec(e, a[2])));
                Ret::default()
            });
        }
        let listener = e.mem.alloc(8);
        e.mem.set_u32(listener, listener_vtable);
        for address in [
            ACTIVATE,
            ADD_REFERENCE,
            RELEASE_REFERENCE,
            SET_BODY_FLOAT,
            BASE_CONSTRUCT,
            BASE_CONSTRUCT_FLAGGED,
            BASE_DESTRUCT,
            STRING_ASSIGN,
        ] {
            log_double(&mut e, &log, address, 0);
        }
        {
            let log = Rc::clone(&log);
            e.register_double(ARRAY_GROW, move |e, a| {
                log.borrow_mut().push((ARRAY_GROW, a.to_vec()));
                let (list, old) = (a[1], e.mem.u32(a[1]));
                let size = e.mem.u32(list + 4);
                let block = e.mem.alloc(0x40);
                copy_words(e, old, block, size);
                e.mem.set_u32(list, block);
                e.mem.set_u32(list + 8, 0x10);
                Ret::default()
            });
        }
        let allocator = logging_object(&mut e, &log, ALLOCATOR);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        {
            let log = Rc::clone(&log);
            e.register_double(MEMORY_ROUTER, move |_, a| {
                log.borrow_mut().push((MEMORY_ROUTER, a.to_vec()));
                Ret {
                    eax: router,
                    ..Ret::default()
                }
            });
        }
        Rig {
            e,
            log,
            vectors,
            heard,
            rays,
            hit,
            mass,
            motion,
            listener,
            allocator,
            router,
        }
    }

    /// A gun with the default limits, holding `held`, and two listeners.
    fn gun(r: &mut Rig, held: &[u32]) -> u32 {
        let g = r.e.mem.alloc(0x80);
        r.e.mem.set_f32(g + 0x30, 100.0);
        r.e.mem.set_f32(g + 0x34, 50.0);
        r.e.mem.set_f32(g + 0x38, 10.0);
        r.e.mem.set_f32(g + 0x3c, 20.0);
        r.e.mem.set_u32(g + 0x2c, 10);
        store_vec(&mut r.e, g + 0x40, [2.5, 0.6, 0.0, 0.0]);
        store_vec(&mut r.e, g + 0x50, [0.0, 1.0, 0.0, 0.0]);
        let array = r.e.mem.alloc(0x40);
        for (index, body) in held.iter().enumerate() {
            r.e.mem.set_u32(array + 4 * index as u32, *body);
        }
        r.e.mem.set_u32(g + 0x20, array);
        r.e.mem.set_u32(g + 0x24, held.len() as u32);
        r.e.mem.set_u32(g + 0x28, 8);
        let listeners = r.e.mem.alloc(8);
        r.e.mem.set_u32(listeners, r.listener);
        r.e.mem.set_u32(listeners + 4, r.listener);
        r.e.mem.set_u32(g + 0x14, listeners);
        r.e.mem.set_u32(g + 0x18, 2);
        g
    }

    fn body(r: &mut Rig, position: [f32; 4]) -> u32 {
        let b = r.e.mem.alloc(0x200);
        r.e.mem.set_u32(b + 0xe0, r.motion);
        store_vec(&mut r.e, b + 0x140, position);
        b
    }

    /// A transform whose first vector is `direction` and whose translation is `origin`.
    fn transform(r: &mut Rig, direction: [f32; 4], origin: [f32; 4]) -> u32 {
        let t = r.e.mem.alloc(0x40);
        store_vec(&mut r.e, t, direction);
        store_vec(&mut r.e, t + 0x30, origin);
        t
    }

    /// A world whose two lists hold one object each with the given bodies.
    fn world(r: &mut Rig, first: &[u32], second: &[u32]) -> u32 {
        let w = r.e.mem.alloc(0x40);
        for (header, bodies) in [(w + 0x28, first), (w + 0x34, second)] {
            if bodies.is_empty() {
                continue;
            }
            let array = r.e.mem.alloc(4 * bodies.len() as u32);
            for (index, body) in bodies.iter().enumerate() {
                r.e.mem.set_u32(array + 4 * index as u32, *body);
            }
            let object = r.e.mem.alloc(0x60);
            r.e.mem.set_u32(object + 0x48, array);
            r.e.mem.set_u32(object + 0x4c, bodies.len() as u32);
            let objects = r.e.mem.alloc(4);
            r.e.mem.set_u32(objects, object);
            r.e.mem.set_u32(header, objects);
            r.e.mem.set_u32(header + 4, 1);
        }
        w
    }

    fn pick(r: &mut Rig, g: u32, t: u32, exclude: u32, w: u32) -> u32 {
        pick_body(&mut r.e, Ptr::new(g), Ptr::new(t), exclude, Ptr::new(w))
    }

    fn held(r: &Rig, g: u32) -> Vec<u32> {
        let count = r.e.mem.u32(g + 0x24);
        let data = r.e.mem.u32(g + 0x20);
        (0..count).map(|i| r.e.mem.u32(data + 4 * i)).collect()
    }

    fn vectors_at(r: &Rig, address: u32) -> Vec<(u32, [f32; 4])> {
        r.vectors
            .borrow()
            .iter()
            .filter(|(a, _, _)| *a == address)
            .map(|(_, this, vector)| (*this, *vector))
            .collect()
    }

    #[test]
    fn pick_body_answers_a_body_in_front_after_a_clear_ray() {
        let mut r = rig();
        let b = body(&mut r, [1.0, 0.5, 0.0, 0.0]);
        let player = body(&mut r, [9.0, 9.0, 9.0, 0.0]);
        let t = transform(&mut r, [0.0; 4], [1.0, 2.0, 3.0, 1.0]);
        let g = gun(&mut r, &[]);
        let w = world(&mut r, &[b], &[]);
        assert_eq!(pick(&mut r, g, t, player, w), b);
        let rays = r.rays.borrow();
        assert_eq!(rays.len(), 1);
        assert_eq!(rays[0].0, vec![1.0, 2.0, 3.0, 1.0, 1.0, 0.5, 0.0, 0.0]);
        let one = 1.0f32.to_bits();
        assert_eq!(
            rays[0].1,
            vec![RAY_COLLECTOR_VTABLE, one, one, 0xffff_ffff, 0xffff_ffff, 0]
        );
        assert_eq!(calls(&r.log, CAST_RAY)[0][0], w);
        let inverse = calls(&r.log, SET_TRANSFORMED_INVERSE_POS);
        assert_eq!(inverse.len(), 1);
        assert_eq!(inverse[0][1..], [t, b + 0x140]);
        assert_eq!(calls(&r.log, GET_MASS), vec![vec![b + 0xe0]]);
    }

    #[test]
    fn pick_body_rejects_bodies_out_of_reach_or_scoring_badly() {
        for position in [
            [0.05, 0.0, 0.0, 0.0],
            [60.0, 0.0, 0.0, 0.0],
            [1.0, 0.8, 0.0, 0.0],
            [1.0, 0.0, -0.8, 0.0],
        ] {
            let mut r = rig();
            let b = body(&mut r, position);
            let t = transform(&mut r, [0.0; 4], [0.0; 4]);
            let g = gun(&mut r, &[]);
            let w = world(&mut r, &[b], &[]);
            assert_eq!(pick(&mut r, g, t, 0, w), 0, "{position:?}");
            assert!(r.rays.borrow().is_empty());
        }
    }

    #[test]
    fn pick_body_skips_the_excluded_and_the_held_bodies() {
        let mut r = rig();
        let b = body(&mut r, [1.0, 0.0, 0.0, 0.0]);
        let t = transform(&mut r, [0.0; 4], [0.0; 4]);
        let g = gun(&mut r, &[]);
        let w = world(&mut r, &[b], &[]);
        assert_eq!(pick(&mut r, g, t, b, w), 0);
        let holding = gun(&mut r, &[b]);
        assert_eq!(pick(&mut r, holding, t, 0, w), 0);
        assert!(r.rays.borrow().is_empty());
        assert_eq!(pick(&mut r, g, t, 0, w), b);
    }

    #[test]
    fn pick_body_follows_the_ray_result() {
        let mut r = rig();
        let b = body(&mut r, [1.0, 0.0, 0.0, 0.0]);
        let t = transform(&mut r, [0.0; 4], [0.0; 4]);
        let g = gun(&mut r, &[]);
        let w = world(&mut r, &[b], &[]);
        r.hit.set(0x1234);
        assert_eq!(pick(&mut r, g, t, 0, w), 0);
        r.hit.set(b + 0x10);
        assert_eq!(pick(&mut r, g, t, 0, w), b);
        r.hit.set(0);
        assert_eq!(pick(&mut r, g, t, 0, w), b);
    }

    #[test]
    fn pick_body_keeps_the_lowest_score_and_the_later_of_equals() {
        let mut r = rig();
        let low = body(&mut r, [1.0, -0.2, 0.0, 0.0]);
        let high = body(&mut r, [1.0, 0.3, 0.0, 0.0]);
        let twin = body(&mut r, [1.0, -0.2, 0.0, 0.0]);
        let t = transform(&mut r, [0.0; 4], [0.0; 4]);
        let g = gun(&mut r, &[]);
        let w = world(&mut r, &[high, low], &[]);
        assert_eq!(pick(&mut r, g, t, 0, w), low);
        let w = world(&mut r, &[low, high], &[]);
        assert_eq!(pick(&mut r, g, t, 0, w), low);
        assert_eq!(r.rays.borrow().len(), 3);
        let w = world(&mut r, &[low, twin], &[]);
        assert_eq!(pick(&mut r, g, t, 0, w), twin);
    }

    #[test]
    fn pick_body_adds_a_penalty_for_heavy_bodies_and_reads_the_second_list() {
        let mut r = rig();
        let b = body(&mut r, [1.0, 0.6, 0.0, 0.0]);
        let t = transform(&mut r, [0.0; 4], [0.0; 4]);
        let g = gun(&mut r, &[]);
        let w = world(&mut r, &[], &[b]);
        assert_eq!(pick(&mut r, g, t, 0, w), b);
        r.mass.set(200.0);
        assert_eq!(pick(&mut r, g, t, 0, w), 0);
        r.mass.set(100.0);
        assert_eq!(pick(&mut r, g, t, 0, w), b);
    }

    #[test]
    fn pick_body_lets_a_nan_position_through_and_finds_nothing_in_an_empty_world() {
        let mut r = rig();
        let b = body(&mut r, [f32::NAN, 0.0, 0.0, 0.0]);
        let t = transform(&mut r, [0.0; 4], [0.0; 4]);
        let g = gun(&mut r, &[]);
        let w = world(&mut r, &[b], &[]);
        assert_eq!(pick(&mut r, g, t, 0, w), b);
        let empty = world(&mut r, &[], &[]);
        assert_eq!(pick(&mut r, g, t, 0, empty), 0);
        assert_eq!(r.rays.borrow().len(), 1);
    }

    #[test]
    fn step_targets_move_a_held_body_toward_its_slot() {
        let mut r = rig();
        let b = body(&mut r, [1.0, 1.0, 1.0, 0.0]);
        let c = body(&mut r, [0.0, 3.0, 0.0, 0.0]);
        store_vec(&mut r.e, c + 0x1b0, [1.0, 1.0, 1.0, 0.0]);
        let player = body(&mut r, [0.0; 4]);
        store_vec(&mut r.e, player + 0x1b0, [10.0, 0.0, 0.0, 0.0]);
        let t = transform(&mut r, [0.0; 4], [0.0; 4]);
        let g = gun(&mut r, &[b, c]);
        fn_00e45c90(&mut r.e, Ptr::new(g), 0.5, Ptr::new(t), Ptr::new(player));
        let inverse = (0.4f64 / 0.5f64) as f32;
        let wanted = |target: [f32; 4], position: [f32; 4], own: [f32; 4], other: [f32; 4]| {
            let mut out = [0.0f32; 4];
            for (lane, value) in out.iter_mut().enumerate() {
                let mixed = (1.0f32 - 0.8) * own[lane] + other[lane] * 0.8;
                *value = (target[lane] - position[lane]) * inverse + mixed;
            }
            out
        };
        let first = wanted(
            [2.5, 0.6, 0.0, 0.0],
            [1.0, 1.0, 1.0, 0.0],
            [0.0; 4],
            [10.0, 0.0, 0.0, 0.0],
        );
        let second = wanted(
            [2.5, 1.0 + 0.6, 0.0, 0.0],
            [0.0, 3.0, 0.0, 0.0],
            [1.0, 1.0, 1.0, 0.0],
            [10.0, 0.0, 0.0, 0.0],
        );
        assert_eq!(
            vectors_at(&r, MOTION + 0x40),
            vec![(b + 0xe0, first), (c + 0xe0, second)]
        );
        assert_eq!(calls(&r.log, ACTIVATE), vec![vec![b], vec![c]]);
        assert!(vectors_at(&r, MOTION + 0x44).is_empty());
        let moves = calls(&r.log, SET_TRANSFORMED_POS);
        assert_eq!(moves.len(), 2);
        assert!(moves.iter().all(|call| call[1] == t));
    }

    #[test]
    fn step_targets_leave_a_body_within_the_tolerance_alone() {
        let mut r = rig();
        let b = body(&mut r, [2.5, 0.6, 0.0, 0.0]);
        let player = body(&mut r, [0.0; 4]);
        store_vec(&mut r.e, b + 0x1b0, [1.0, 2.0, 3.0, 0.0]);
        store_vec(&mut r.e, player + 0x1b0, [1.0, 2.0, 3.0, 0.0]);
        let t = transform(&mut r, [0.0; 4], [0.0; 4]);
        let g = gun(&mut r, &[b]);
        fn_00e45c90(&mut r.e, Ptr::new(g), 0.5, Ptr::new(t), Ptr::new(player));
        assert!(r.vectors.borrow().is_empty());
        assert!(calls(&r.log, ACTIVATE).is_empty());
        let empty = gun(&mut r, &[]);
        fn_00e45c90(
            &mut r.e,
            Ptr::new(empty),
            0.5,
            Ptr::new(t),
            Ptr::new(player),
        );
        assert_eq!(calls(&r.log, SET_TRANSFORMED_POS).len(), 1);
    }

    #[test]
    fn step_targets_slow_a_spinning_body_only_above_a_unit_square() {
        let mut r = rig();
        let b = body(&mut r, [2.5, 0.6, 0.0, 0.0]);
        let player = body(&mut r, [0.0; 4]);
        store_vec(&mut r.e, b + 0x1b0, [1.0, 2.0, 3.0, 0.0]);
        store_vec(&mut r.e, player + 0x1b0, [1.0, 2.0, 3.0, 0.0]);
        let t = transform(&mut r, [0.0; 4], [0.0; 4]);
        let g = gun(&mut r, &[b]);
        store_vec(&mut r.e, b + 0x1c0, [0.5, 0.0, 0.0, 0.0]);
        fn_00e45c90(&mut r.e, Ptr::new(g), 0.5, Ptr::new(t), Ptr::new(player));
        assert!(r.vectors.borrow().is_empty());
        store_vec(&mut r.e, b + 0x1c0, [2.0, 0.0, 0.0, 0.0]);
        fn_00e45c90(&mut r.e, Ptr::new(g), 0.5, Ptr::new(t), Ptr::new(player));
        assert_eq!(
            vectors_at(&r, MOTION + 0x44),
            vec![(b + 0xe0, [0.8f32 * 2.0, 0.0, 0.0, 0.0])]
        );
        assert_eq!(calls(&r.log, ACTIVATE), vec![vec![b]]);
    }

    #[test]
    fn release_slot_removes_a_held_body_and_scales_its_float() {
        let mut r = rig();
        let (a, b, c) = (
            body(&mut r, [0.0; 4]),
            body(&mut r, [0.0; 4]),
            body(&mut r, [0.0; 4]),
        );
        r.e.mem.set_f32(b + 0x1ac, 3.0);
        let g = gun(&mut r, &[a, b, c]);
        assert_eq!(fn_00e45e60(&mut r.e, Ptr::new(g), 1), b);
        assert_eq!(held(&r, g), vec![a, c]);
        let scaled = (3.0f64 * 0.01f64) as f32;
        assert_eq!(
            calls(&r.log, SET_BODY_FLOAT),
            vec![vec![b, scaled.to_bits()]]
        );
        assert_eq!(calls(&r.log, RELEASE_REFERENCE), vec![vec![b]]);
        assert_eq!(fn_00e45e60(&mut r.e, Ptr::new(g), 1), c);
        assert_eq!(held(&r, g), vec![a]);
        assert_eq!(fn_00e45e60(&mut r.e, Ptr::new(g), 0), a);
        assert!(held(&r, g).is_empty());
    }

    #[test]
    fn push_throws_the_held_body_with_the_players_velocity() {
        let mut r = rig();
        let a = body(&mut r, [0.0; 4]);
        let player = body(&mut r, [0.0; 4]);
        store_vec(&mut r.e, player + 0x1b0, [1.0, 2.0, 3.0, 0.0]);
        let t = transform(&mut r, [2.0, 3.0, 4.0, 0.0], [0.0; 4]);
        let g = gun(&mut r, &[a]);
        let w = world(&mut r, &[], &[]);
        let (gun_ptr, t_ptr) = (Ptr::new(g), Ptr::new(t));
        gravity_gun_push(&mut r.e, gun_ptr, t_ptr, Ptr::new(player), Ptr::new(w));
        assert_eq!(
            vectors_at(&r, SET_LINEAR_VELOCITY),
            vec![(a, [41.0, 62.5, 83.0, 0.0])]
        );
        assert_eq!(r.e.mem.u8(a + 0x2a), 6);
        assert!(held(&r, g).is_empty());
        assert_eq!(calls(&r.log, RELEASE_REFERENCE), vec![vec![a]]);
    }

    #[test]
    fn push_without_a_held_body_pushes_the_picked_one() {
        let mut r = rig();
        let b = body(&mut r, [1.0, 0.5, 0.0, 0.0]);
        let player = body(&mut r, [9.0; 4]);
        let t = transform(&mut r, [2.0, 3.0, 4.0, 0.0], [0.0; 4]);
        let g = gun(&mut r, &[]);
        let w = world(&mut r, &[b], &[]);
        let (gun_ptr, t_ptr, p_ptr) = (Ptr::new(g), Ptr::new(t), Ptr::new(player));
        gravity_gun_push(&mut r.e, gun_ptr, t_ptr, p_ptr, Ptr::new(w));
        assert_eq!(calls(&r.log, ACTIVATE), vec![vec![b]]);
        assert_eq!(
            vectors_at(&r, MOTION + 0x4c),
            vec![(b + 0xe0, [20.0, 30.0, 40.0, 0.0])]
        );
        let empty = world(&mut r, &[], &[]);
        gravity_gun_push(&mut r.e, gun_ptr, t_ptr, p_ptr, Ptr::new(empty));
        assert_eq!(calls(&r.log, ACTIVATE).len(), 1);
    }

    #[test]
    fn release_first_only_acts_on_a_held_body() {
        let mut r = rig();
        let a = body(&mut r, [0.0; 4]);
        let g = gun(&mut r, &[]);
        fn_00e45fb0(&mut r.e, Ptr::new(g), 0, 0, 0);
        assert!(calls(&r.log, RELEASE_REFERENCE).is_empty());
        let g = gun(&mut r, &[a]);
        fn_00e45fb0(&mut r.e, Ptr::new(g), 0, 0, 0);
        assert_eq!(calls(&r.log, RELEASE_REFERENCE), vec![vec![a]]);
        assert!(held(&r, g).is_empty());
    }

    #[test]
    fn reset_releases_every_held_body() {
        let mut r = rig();
        let (a, b) = (body(&mut r, [0.0; 4]), body(&mut r, [0.0; 4]));
        let g = gun(&mut r, &[a, b]);
        reset(&mut r.e, Ptr::new(g), 0);
        assert_eq!(calls(&r.log, RELEASE_REFERENCE), vec![vec![a], vec![b]]);
        assert!(held(&r, g).is_empty());
        reset(&mut r.e, Ptr::new(g), 0);
        assert_eq!(calls(&r.log, RELEASE_REFERENCE).len(), 2);
    }

    #[test]
    fn take_object_appends_the_body_and_scales_its_float() {
        let mut r = rig();
        let (a, b) = (body(&mut r, [0.0; 4]), body(&mut r, [0.0; 4]));
        r.e.mem.set_f32(b + 0x1ac, 0.5);
        let g = gun(&mut r, &[a]);
        take_object(&mut r.e, Ptr::new(g), b);
        assert_eq!(held(&r, g), vec![a, b]);
        assert_eq!(calls(&r.log, ADD_REFERENCE), vec![vec![b]]);
        assert_eq!(
            calls(&r.log, SET_BODY_FLOAT),
            vec![vec![b, 50.0f32.to_bits()]]
        );
        assert!(calls(&r.log, ARRAY_GROW).is_empty());
    }

    #[test]
    fn take_object_grows_a_full_list_first() {
        let mut r = rig();
        let (a, b) = (body(&mut r, [0.0; 4]), body(&mut r, [0.0; 4]));
        let g = gun(&mut r, &[a]);
        r.e.mem.set_u32(g + 0x28, 0x8000_0001);
        take_object(&mut r.e, Ptr::new(g), b);
        assert_eq!(
            calls(&r.log, ARRAY_GROW),
            vec![vec![r.allocator, g + 0x20, 4]]
        );
        assert_eq!(held(&r, g), vec![a, b]);
        assert!(!calls(&r.log, MEMORY_ROUTER).is_empty());
        assert_eq!(r.e.mem.u32(r.router + 0x10), r.allocator);
    }

    #[test]
    fn pull_takes_a_light_enough_body_and_pushes_back_otherwise() {
        let mut r = rig();
        let b = body(&mut r, [1.0, 0.5, 0.0, 0.0]);
        let player = body(&mut r, [9.0; 4]);
        let t = transform(&mut r, [2.0, 3.0, 4.0, 0.0], [0.0; 4]);
        let g = gun(&mut r, &[]);
        let w = world(&mut r, &[b], &[]);
        let (t_ptr, p_ptr, w_ptr) = (Ptr::new(t), Ptr::new(player), Ptr::new(w));
        r.e.mem.set_f32(b + 0x1ac, 0.5);
        gravity_gun_pull(&mut r.e, Ptr::new(g), t_ptr, p_ptr, w_ptr);
        assert_eq!(held(&r, g), vec![b]);
        assert!(r.vectors.borrow().is_empty());
        let g = gun(&mut r, &[]);
        r.e.mem.set_f32(b + 0x1ac, 0.005);
        gravity_gun_pull(&mut r.e, Ptr::new(g), t_ptr, p_ptr, w_ptr);
        assert!(held(&r, g).is_empty());
        assert_eq!(
            vectors_at(&r, MOTION + 0x4c),
            vec![(b + 0xe0, [-20.0, -30.0, -40.0, 0.0])]
        );
        assert_eq!(calls(&r.log, ACTIVATE), vec![vec![b]]);
    }

    #[test]
    fn pull_pushes_back_when_the_gun_is_full_and_takes_a_nan_weight() {
        let mut r = rig();
        let b = body(&mut r, [1.0, 0.5, 0.0, 0.0]);
        let player = body(&mut r, [9.0; 4]);
        let t = transform(&mut r, [2.0, 3.0, 4.0, 0.0], [0.0; 4]);
        let g = gun(&mut r, &[]);
        let w = world(&mut r, &[b], &[]);
        let (gun_ptr, t_ptr, p_ptr) = (Ptr::new(g), Ptr::new(t), Ptr::new(player));
        r.e.mem.set_f32(b + 0x1ac, 0.5);
        r.e.mem.set_u32(g + 0x2c, 0);
        gravity_gun_pull(&mut r.e, gun_ptr, t_ptr, p_ptr, Ptr::new(w));
        assert!(held(&r, g).is_empty());
        assert_eq!(vectors_at(&r, MOTION + 0x4c).len(), 1);
        r.e.mem.set_u32(g + 0x2c, 10);
        r.e.mem.set_f32(b + 0x1ac, f32::NAN);
        gravity_gun_pull(&mut r.e, gun_ptr, t_ptr, p_ptr, Ptr::new(w));
        assert_eq!(held(&r, g), vec![b]);
        let empty = world(&mut r, &[], &[]);
        gravity_gun_pull(&mut r.e, gun_ptr, t_ptr, p_ptr, Ptr::new(empty));
        assert_eq!(held(&r, g), vec![b]);
    }

    #[test]
    fn step_gun_push_tells_the_listeners_and_ignores_pull() {
        let mut r = rig();
        let a = body(&mut r, [0.0; 4]);
        let player = body(&mut r, [9.0; 4]);
        store_vec(&mut r.e, player + 0x120, [7.0, 8.0, 9.0, 1.0]);
        let t = transform(&mut r, [2.0, 3.0, 4.0, 0.0], [0.0; 4]);
        let g = gun(&mut r, &[a]);
        let w = world(&mut r, &[], &[]);
        let (p_ptr, t_ptr, w_ptr) = (Ptr::new(player), Ptr::new(t), Ptr::new(w));
        step_gun(&mut r.e, Ptr::new(g), 0.5, w_ptr, p_ptr, t_ptr, 1, 1);
        let seen = (Some([7.0, 8.0, 9.0, 1.0]), None);
        assert_eq!(*r.heard.borrow(), vec![seen, seen]);
        assert_eq!(vectors_at(&r, SET_LINEAR_VELOCITY).len(), 1);
        assert!(calls(&r.log, ADD_REFERENCE).is_empty());
    }

    #[test]
    fn step_gun_pull_lets_go_when_full_and_pulls_otherwise() {
        let mut r = rig();
        let a = body(&mut r, [0.0; 4]);
        let player = body(&mut r, [9.0; 4]);
        store_vec(&mut r.e, player + 0x120, [7.0, 8.0, 9.0, 1.0]);
        let t = transform(&mut r, [2.0, 3.0, 4.0, 0.0], [5.0, 6.0, 7.0, 1.0]);
        let g = gun(&mut r, &[a]);
        r.e.mem.set_u32(g + 0x2c, 1);
        let w = world(&mut r, &[], &[]);
        let (gun_ptr, p_ptr, t_ptr) = (Ptr::new(g), Ptr::new(player), Ptr::new(t));
        step_gun(&mut r.e, gun_ptr, 0.5, Ptr::new(w), p_ptr, t_ptr, 0, 1);
        let seen = (Some([7.0, 8.0, 9.0, 1.0]), None);
        assert_eq!(*r.heard.borrow(), vec![seen, seen]);
        assert_eq!(calls(&r.log, RELEASE_REFERENCE), vec![vec![a]]);
        r.heard.borrow_mut().clear();
        let b = body(&mut r, [1.0, 0.5, 0.0, 0.0]);
        r.e.mem.set_f32(b + 0x1ac, 0.5);
        let w = world(&mut r, &[b], &[]);
        step_gun(&mut r.e, gun_ptr, 0.5, Ptr::new(w), p_ptr, t_ptr, 0, 1);
        let seen = (Some([5.0, 6.0, 7.0, 1.0]), Some([7.0, 8.0, 9.0, 1.0]));
        assert_eq!(*r.heard.borrow(), vec![seen, seen]);
        assert_eq!(held(&r, g), vec![b]);
    }

    #[test]
    fn step_gun_without_a_button_only_moves_the_held_bodies() {
        let mut r = rig();
        let a = body(&mut r, [0.0; 4]);
        let player = body(&mut r, [9.0; 4]);
        store_vec(&mut r.e, player + 0x1b0, [10.0, 0.0, 0.0, 0.0]);
        let t = transform(&mut r, [0.0; 4], [0.0; 4]);
        let g = gun(&mut r, &[a]);
        let w = world(&mut r, &[], &[]);
        let bits = 0.5f32.to_bits();
        r.e.call(0x00e4_6140, &args![g, bits, w, player, t, 0u32, 0u32]);
        assert!(r.heard.borrow().is_empty());
        assert_eq!(vectors_at(&r, MOTION + 0x40).len(), 1);
        assert_eq!(calls(&r.log, ACTIVATE), vec![vec![a]]);
    }

    #[test]
    fn registered_pick_body_takes_its_arguments_in_order() {
        let mut r = rig();
        let b = body(&mut r, [1.0, 0.5, 0.0, 0.0]);
        let t = transform(&mut r, [0.0; 4], [0.0; 4]);
        let g = gun(&mut r, &[]);
        let w = world(&mut r, &[b], &[]);
        assert_eq!(r.e.call(0x00e4_5a60, &args![g, t, 0u32, w]).u32(), b);
    }

    #[test]
    fn constructor_sets_the_defaults() {
        let mut r = rig();
        let t = r.e.mem.alloc(0x80);
        r.e.mem.set_u32(0x0102_226c, 0x7f80_0001);
        assert_eq!(hkp_gravity_gun(&mut r.e, Ptr::new(t)).addr(), t);
        assert_eq!(calls(&r.log, BASE_CONSTRUCT), vec![vec![t]]);
        assert_eq!(calls(&r.log, STRING_ASSIGN), vec![vec![t + 0xc, NAME_TEXT]]);
        assert_eq!(r.e.mem.u32(t), VTABLE);
        assert_eq!(r.e.mem.u32(t + 0x28), 0x8000_0000);
        assert_eq!((r.e.mem.u32(t + 0x20), r.e.mem.u32(t + 0x24)), (0, 0));
        assert_eq!(r.e.mem.u32(t + 0x30), 0x7fc0_0001);
        assert_eq!(
            [0x34, 0x38, 0x3c].map(|field| r.e.mem.f32(t + field)),
            [50.0, 100.0, 50.0]
        );
        assert_eq!(r.e.mem.u32(t + 0x2c), 10);
        assert_eq!(r.e.mem.u8(t + 8), 3);
        assert_eq!(load_vec(&r.e, t + 0x40), [2.5, 0.6, 0.0, 0.0]);
        assert_eq!(load_vec(&r.e, t + 0x50), [0.0, 1.0, 0.0, 0.0]);
    }

    #[test]
    fn flagged_constructor_sets_the_mode_byte_only_with_the_flag() {
        let mut r = rig();
        let t = r.e.mem.alloc(0x80);
        r.e.mem.set_u8(t + 8, 0x55);
        r.e.mem.set_u32(t + 0x24, 9);
        assert_eq!(fn_00e46300(&mut r.e, Ptr::new(t), 0).addr(), t);
        assert_eq!(r.e.mem.u8(t + 8), 0x55);
        assert_eq!(r.e.mem.u32(t), VTABLE);
        assert_eq!(r.e.mem.u32(t + 0x24), 0);
        assert_eq!(r.e.mem.u32(t + 0x28), 0x8000_0000);
        assert_eq!(fn_00e46300(&mut r.e, Ptr::new(t), 1).addr(), t);
        assert_eq!(r.e.mem.u8(t + 8), 3);
        assert_eq!(
            calls(&r.log, BASE_CONSTRUCT_FLAGGED),
            vec![vec![t, 0], vec![t, 1]]
        );
    }

    #[test]
    fn deleting_destructor_frees_the_list_and_then_the_object() {
        let mut r = rig();
        let t = r.e.mem.alloc(0x80);
        let block = r.e.mem.alloc(0x10);
        r.e.mem.set_u32(t + 0x20, block);
        r.e.mem.set_u32(t + 0x24, 3);
        r.e.mem.set_u32(t + 0x28, 4);
        r.e.mem.set_u16(t + 4, 0x80);
        assert_eq!(deleting_destructor(&mut r.e, Ptr::new(t), 1).addr(), t);
        let watched = [ALLOCATOR + 8, ALLOCATOR + 0x10, BASE_DESTRUCT];
        let order: Vec<u32> = r
            .log
            .borrow()
            .iter()
            .map(|(address, _)| *address)
            .filter(|address| watched.contains(address))
            .collect();
        assert_eq!(order, vec![ALLOCATOR + 0x10, BASE_DESTRUCT, ALLOCATOR + 8]);
        assert_eq!(
            calls(&r.log, ALLOCATOR + 0x10),
            vec![vec![r.allocator, block, 16]]
        );
        assert_eq!(
            calls(&r.log, ALLOCATOR + 8),
            vec![vec![r.allocator, t, 0x80]]
        );
        assert_eq!(r.e.mem.u32(t + 0x20), 0);
        assert_eq!(r.e.mem.u32(t + 0x24), 0);
        assert_eq!(r.e.mem.u32(t + 0x28), 0x8000_0000);
    }

    #[test]
    fn deleting_destructor_keeps_a_borrowed_list_and_an_object_without_the_flag() {
        let mut r = rig();
        let t = r.e.mem.alloc(0x80);
        r.e.mem.set_u32(t + 0x20, 0x1234);
        r.e.mem.set_u32(t + 0x28, 0x8000_0004);
        deleting_destructor(&mut r.e, Ptr::new(t), 0);
        assert!(calls(&r.log, ALLOCATOR + 0x10).is_empty());
        assert!(calls(&r.log, ALLOCATOR + 8).is_empty());
        assert_eq!(calls(&r.log, BASE_DESTRUCT), vec![vec![t]]);
        assert_eq!(r.e.mem.u32(t + 0x20), 0);
        assert_eq!(r.e.mem.u32(t + 0x28), 0x8000_0000);
    }
}
