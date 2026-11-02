//! `Havok SDK/hkpballgun.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpBallGun` (the Xbox PDB has no record of the Havok classes, so the field
//! names below describe what the code does and are unconfirmed): the gun's
//! listener array is at `+0x14` (data) / `+0x18` (count), the sphere radius at
//! `+0x20`, the muzzle speed at `+0x24`, the bullet mass at `+0x28`, the bullet
//! `+0x9c` value at `+0x2c`, the most bullets kept at `+0x30`, the muzzle
//! offset vector at `+0x40` and the queue of fired bodies at `+0x50`. The
//! queue (`hkQueue<hkpRigidBody *>`, `+0x00` data, `+0x04` capacity, `+0x08`
//! head, `+0x0C` tail, `+0x10` count) is a ring buffer.

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkpsphereshape::{copy_vec, load_vec, store_vec, Vec4};

/// The memory router (`00c85750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkpBallGun`'s vtable (`010ec174`).
const BALL_GUN_VTABLE: u32 = 0x010e_c174;
/// The string `"BallGun"` the constructor hands the string pointer at `+0x0C`.
const BALL_GUN_NAME: u32 = 0x010e_c198;
/// Constants the constructor copies (`FLD`/`FSTP`) into the gun's settings.
const CONSTANT_RADIUS: u32 = 0x0101_df44;
const CONSTANT_SPEED: u32 = 0x0104_ef80;
const CONSTANT_MASS: u32 = 0x0101_b268;
/// Constants `fireGun` copies into the body description.
const CONSTANT_BODY_A: u32 = 0x0101_3ea4;
const CONSTANT_BODY_B: u32 = 0x0102_31a8;
/// `200.0f` (`MOVSS`): how far ahead of the muzzle the target point lies.
const TARGET_DISTANCE: u32 = 0x0102_226c;

/// `hkpFirstPersonGun` constructor (`00e46660`, no arguments) and the
/// flagged one (`00e466a0`, one word); its destructor (`00e466e0`).
const BASE_CONSTRUCT: u32 = 0x00e4_6660;
const BASE_CONSTRUCT_FLAGGED: u32 = 0x00e4_66a0;
const BASE_DESTRUCT: u32 = 0x00e4_66e0;
/// `hkStringPtr::operator=` (`00cb0c10`, one word).
const STRING_ASSIGN: u32 = 0x00cb_0c10;
/// The queue constructor (`00e46e80`, returns the queue) and its
/// `setCapacity` (`00e45460`, one word).
const QUEUE_CONSTRUCT: u32 = 0x00e4_6e80;
const QUEUE_SET_CAPACITY: u32 = 0x00e4_5460;
/// `hkpWorld::removeEntity` (`00c91620`: world, out pointer, entity) and
/// `addEntity` (`00c914d0`: world, entity, flag).
const REMOVE_ENTITY: u32 = 0x00c9_1620;
const ADD_ENTITY: u32 = 0x00c9_14d0;
/// The reference-count release (`00c905b0`, `this` only).
const RELEASE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkpRigidBodyCinfo` constructor (`00cf5ea0`), `hkpSphereShape`
/// constructor (`00cd8160`, radius), `setShapeVolumeMassProperties`
/// (`00cf07a0`, cdecl: shape, mass, description), `hkpRigidBody`
/// constructor (`00ca8c90`, description).
const BODY_INFO_CONSTRUCT: u32 = 0x00cf_5ea0;
const SPHERE_CONSTRUCT: u32 = 0x00cd_8160;
const SET_SHAPE_MASS_PROPERTIES: u32 = 0x00cf_07a0;
const RIGID_BODY_CONSTRUCT: u32 = 0x00ca_8c90;
/// `hkpFirstPersonGun::sweepSphere` (`00e464e0`, cdecl: world, from, radius,
/// to, hit) and `calcVelocityToTarget` (`00e463d0`, cdecl: start, target,
/// gravity, speed, velocity).
const SWEEP_SPHERE: u32 = 0x00e4_64e0;
const CALC_VELOCITY: u32 = 0x00e4_63d0;

const LISTENERS: u32 = 0x14;
const LISTENER_COUNT: u32 = 0x18;
const SPHERE_RADIUS: u32 = 0x20;
const MUZZLE_SPEED: u32 = 0x24;
const BULLET_MASS: u32 = 0x28;
const BULLET_VALUE: u32 = 0x2c;
const MAX_BULLETS: u32 = 0x30;
const MUZZLE_OFFSET: u32 = 0x40;
const QUEUE: u32 = 0x50;

/// An `FLD dword` / `FSTP dword` copy of the float at `addr`: the bit pattern, quieted.
fn copy_float_bits(e: &Engine, addr: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(addr))
}

/// Allocates `size` bytes through the memory router's allocator (slot 4).
fn allocate(e: &mut Engine, size: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.vcall(allocator, 4, &args![size]).u32()
}

/// Frees `block` of `size` bytes through the memory router's allocator (slot 8).
fn deallocate(e: &mut Engine, block: u32, size: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.vcall(allocator, 8, &args![block, size]);
}

/// Takes the front body out of the ring buffer (`data[head]`, head advances
/// and wraps at the capacity, count drops by one).
fn queue_pop_front(e: &mut Engine, queue: u32) -> u32 {
    let head = e.mem.u32(queue + 8);
    let data = e.mem.u32(queue);
    let body = e.mem.u32(data + head * 4);
    let head = head.wrapping_add(1);
    e.mem.set_u32(queue + 8, head);
    if head == e.mem.u32(queue + 4) {
        e.mem.set_u32(queue + 8, 0);
    }
    let count = e.mem.u32(queue + 0x10).wrapping_sub(1);
    e.mem.set_u32(queue + 0x10, count);
    body
}

/// Calls the listeners' slot `slot` (one word, `body`) from the last one to
/// the first; the array and count are read again at each step.
fn notify_listeners_backwards(e: &mut Engine, this: u32, slot: u32, body: u32) {
    let mut index = e.mem.i32(this + LISTENER_COUNT).wrapping_sub(1);
    while index >= 0 {
        let array = e.mem.u32(this + LISTENERS);
        let listener = e.mem.u32(array + (index as u32).wrapping_mul(4));
        e.vcall(listener, slot, &args![body]);
        index = index.wrapping_sub(1);
    }
}

/// Removes the body from the world `world` when it is in one (`removeEntity`
/// with a one-byte result slot at `out`), then drops the queue's reference.
fn remove_and_release(e: &mut Engine, world: u32, body: u32, out: u32) {
    e.call(REMOVE_ENTITY, &args![world, out, body]);
    e.call(RELEASE_REFERENCE, &args![body]);
}

// Translated from 00e46ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBallGun::reset` (engine map): takes every fired body out of the queue
/// (oldest first), tells the listeners (slot `0x10`, last listener first),
/// removes it from `world` when the body's world (`+8`) is that world, and
/// drops the queue's reference.
pub fn hkp_ball_gun_reset(e: &mut Engine, this: Ptr, world: u32) {
    let this = this.addr();
    // The exe passes the address of its own argument slot as the result slot.
    e.with_stack(4, |e, out| loop {
        let queue = e.mem.u32(this + QUEUE);
        if e.mem.u32(queue + 0x10) == 0 {
            break;
        }
        let body = queue_pop_front(e, queue);
        notify_listeners_backwards(e, this, 0x10, body);
        if e.mem.u32(body + 8) == world {
            e.call(REMOVE_ENTITY, &args![world, out, body]);
        }
        e.call(RELEASE_REFERENCE, &args![body]);
    });
}

// Translated from 00e46f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBallGun` constructor for a loaded object (no engine-map name): runs
/// the flagged `hkpFirstPersonGun` constructor with `flag`, sets the vtable
/// and, when `flag` is not zero, sets the byte at `+8` to 1 and makes the
/// queue (0x14 bytes, `00e46e80`) with the capacity at `+0x30` (a failed
/// allocation leaves the pointer 0 and still calls `setCapacity` on it).
/// Returns `this`.
pub fn fn_00e46f20(e: &mut Engine, this: Ptr, flag: u32) -> Ptr {
    let gun = this.addr();
    e.call(BASE_CONSTRUCT_FLAGGED, &args![this, flag]);
    e.mem.set_u32(gun, BALL_GUN_VTABLE);
    if flag != 0 {
        e.mem.set_u8(gun + 8, 1);
        let block = allocate(e, 0x14);
        let queue = if block != 0 {
            e.call(QUEUE_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        e.mem.set_u32(gun + QUEUE, queue);
        let capacity = e.mem.u32(gun + MAX_BULLETS);
        e.call(QUEUE_SET_CAPACITY, &args![queue, capacity]);
    }
    this
}

// Translated from 00e46f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBallGun::hkpBallGun` (engine map): the base constructor, the radius,
/// speed and mass settings copied from the constants at `0101df44`,
/// `0104ef80` and `0101b268` (the mass twice, at `+0x28` and `+0x2c`), the
/// vtable, `max_bullets` at `+0x30`, the 16 bytes at `+0x40` cleared, the
/// byte at `+8` set to 1, the name `"BallGun"` given to the string pointer at
/// `+0x0C` and the queue made with capacity `max_bullets`. Returns `this`.
pub fn hkp_ball_gun_construct(e: &mut Engine, this: Ptr, max_bullets: u32) -> Ptr {
    let gun = this.addr();
    e.call(BASE_CONSTRUCT, &args![this]);
    let radius = copy_float_bits(e, CONSTANT_RADIUS);
    e.mem.set_u32(gun + SPHERE_RADIUS, radius);
    let speed = copy_float_bits(e, CONSTANT_SPEED);
    e.mem.set_u32(gun + MUZZLE_SPEED, speed);
    let mass = copy_float_bits(e, CONSTANT_MASS);
    e.mem.set_u32(gun + BULLET_MASS, mass);
    e.mem.set_u32(gun, BALL_GUN_VTABLE);
    e.mem.set_u32(gun + BULLET_VALUE, mass);
    e.mem.set_u32(gun + MAX_BULLETS, max_bullets);
    store_vec(e, gun + MUZZLE_OFFSET, [0.0; 4]);
    e.mem.set_u8(gun + 8, 1);
    e.call(STRING_ASSIGN, &args![gun + 0xc, BALL_GUN_NAME]);
    let block = allocate(e, 0x14);
    let queue = if block != 0 {
        e.call(QUEUE_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.mem.set_u32(gun + QUEUE, queue);
    e.call(QUEUE_SET_CAPACITY, &args![queue, max_bullets]);
    this
}

// Translated from 00e47020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBallGun::~hkpBallGun` (engine map): sets the vtable, empties the queue
/// the way `reset` does but removing each body from the world it is in
/// (`+8`), frees the queue's storage (`00e47390`) and the queue block (0x14
/// bytes) and runs the base destructor (`00e466e0`, a tail call).
pub fn hkp_ball_gun_destruct(e: &mut Engine, this: Ptr) {
    let gun = this.addr();
    e.mem.set_u32(gun, BALL_GUN_VTABLE);
    e.with_stack(4, |e, out| loop {
        let queue = e.mem.u32(gun + QUEUE);
        if e.mem.u32(queue + 0x10) == 0 {
            break;
        }
        let body = queue_pop_front(e, queue);
        notify_listeners_backwards(e, gun, 0x10, body);
        let world = e.mem.u32(body + 8);
        if world != 0 {
            e.call(REMOVE_ENTITY, &args![world, out, body]);
        }
        e.call(RELEASE_REFERENCE, &args![body]);
    });
    let queue = e.mem.u32(gun + QUEUE);
    if queue != 0 {
        queue_release_storage(e, Ptr::new(queue));
        deallocate(e, queue, 0x14);
    }
    e.call(BASE_DESTRUCT, &args![this]);
}

// Translated from 00e470d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBallGun::fireGun` (engine map): `world` and the gun's transform `matrix`
/// (four vectors; the first is the direction, the fourth the position).
/// Builds a sphere body (`radius` at `+0x20`, mass `2 * [+0x28]`), its
/// position the muzzle offset plus `position + direction`, aims it at the
/// first thing a sphere sweep from there to `200 * direction + ...` hits (or
/// straight along the direction at the muzzle speed when the sweep returns
/// nonzero), tells the listeners (slot `0x14`: the position and the hit),
/// makes the body, stores `[+0x2c]` at its `+0x9c`, tells the listeners (slot
/// `0x0C`), adds it to `world` and pushes it on the queue (growing it to 8 or
/// twice its capacity when full). When the queue then holds more than
/// `[+0x30]` bodies the oldest is taken out, announced (slot `0x10`) and
/// removed from `world`.
pub fn hkp_ball_gun_fire_gun(e: &mut Engine, this: Ptr, world: u32, matrix: Ptr) {
    let gun = this.addr();
    e.with_stack(0x160, |e, frame| {
        let f = frame.addr();
        // Frame (offsets from the aligned frame base): `+0x14` from, `+0x24`
        // direction, `+0x34` position, `+0x44` to, `+0x54` hit copy,
        // `+0x64` body description (0xc0 bytes), `+0x124` hit.
        let from = f + 0x14;
        let direction_slot = f + 0x24;
        let position_slot = f + 0x34;
        let to = f + 0x44;
        let hit_copy = f + 0x54;
        let info = f + 0x64;
        let hit = f + 0x124;

        let direction = load_vec(e, matrix.addr());
        let position = load_vec(e, matrix.addr() + 0x30);
        let distance: f32 = e.global(TARGET_DISTANCE);
        let splat = [distance; 4];
        store_vec(e, position_slot, position);
        let sum: Vec4 = std::array::from_fn(|lane| position[lane] + direction[lane]);
        let far: Vec4 = std::array::from_fn(|lane| splat[lane] * direction[lane] + sum[lane]);
        store_vec(e, direction_slot, direction);
        store_vec(e, from, sum);
        store_vec(e, to, far);

        e.call(BODY_INFO_CONSTRUCT, &args![info]);
        let sphere_block = allocate(e, 0x20);
        e.mem.set_u16(sphere_block + 4, 0x20);
        let radius = copy_float_bits(e, gun + SPHERE_RADIUS);
        let shape = e.call(SPHERE_CONSTRUCT, &args![sphere_block, radius]).u32();
        let mass_bits = copy_float_bits(e, gun + BULLET_MASS);
        e.mem.set_u32(info + 0x90, mass_bits);
        let constant_a = copy_float_bits(e, CONSTANT_BODY_A);
        e.mem.set_u32(info + 0xb0, constant_a);
        e.mem.set_u32(info + 4, shape);
        let radius_default = copy_float_bits(e, CONSTANT_RADIUS);
        e.mem.set_u32(info + 0xa4, radius_default);
        e.mem.set_u8(info + 0xb7, 6);
        e.mem.set_u8(info + 0xb4, 2);
        e.mem.set_f32(info + 0xa0, 1.0);
        // `FLD mass; FADD ST0,ST0; FSTP` then `FLD; FSTP` into the argument slot.
        let mass = f64::from(f32::from_bits(mass_bits));
        let double_mass = x87_quiet((mass + mass) as f32);
        e.mem.set_f32(f + 0x10, double_mass);
        let double_mass = x87_quiet(e.mem.f32(f + 0x10));
        e.call(SET_SHAPE_MASS_PROPERTIES, &args![shape, double_mass, info]);
        let mass_bits = copy_float_bits(e, gun + BULLET_MASS);
        e.mem.set_u32(info + 0x90, mass_bits);
        let muzzle = load_vec(e, gun + MUZZLE_OFFSET);
        e.mem.set_f32(info + 0x94, 0.0);
        let constant_b = copy_float_bits(e, CONSTANT_BODY_B);
        e.mem.set_u32(info + 0x98, constant_b);
        let radius = copy_float_bits(e, gun + SPHERE_RADIUS);
        let start: Vec4 = std::array::from_fn(|lane| muzzle[lane] + sum[lane]);
        store_vec(e, info + 0x10, start);
        let missed = e
            .call(SWEEP_SPHERE, &args![world, from, radius, to, hit])
            .u32();
        if missed == 0 {
            let speed = copy_float_bits(e, gun + MUZZLE_SPEED);
            e.call(
                CALC_VELOCITY,
                &args![info + 0x10, hit, world + 0x10, speed, info + 0x30],
            );
        } else {
            let speed = e.mem.f32(gun + MUZZLE_SPEED);
            let velocity: Vec4 = std::array::from_fn(|lane| speed * direction[lane]);
            store_vec(e, info + 0x30, velocity);
        }
        copy_vec(e, hit, hit_copy);
        let mut index = 0;
        while index < e.mem.i32(gun + LISTENER_COUNT) {
            let array = e.mem.u32(gun + LISTENERS);
            let listener = e.mem.u32(array + (index as u32) * 4);
            e.vcall(listener, 0x14, &args![position_slot, hit_copy]);
            index += 1;
        }

        let body_block = allocate(e, 0x220);
        e.mem.set_u16(body_block + 4, 0x220);
        let body = e.call(RIGID_BODY_CONSTRUCT, &args![body_block, info]).u32();
        let shape = e.mem.u32(info + 4);
        e.call(RELEASE_REFERENCE, &args![shape]);
        let bullet_value = copy_float_bits(e, gun + BULLET_VALUE);
        e.mem.set_u32(body + 0x9c, bullet_value);
        let mut index = 0;
        while index < e.mem.i32(gun + LISTENER_COUNT) {
            let array = e.mem.u32(gun + LISTENERS);
            let listener = e.mem.u32(array + (index as u32) * 4);
            e.vcall(listener, 0xc, &args![body]);
            index += 1;
        }
        e.call(ADD_ENTITY, &args![world, body, 1u32]);

        let queue = e.mem.u32(gun + QUEUE);
        let capacity = e.mem.i32(queue + 4);
        if e.mem.i32(queue + 0x10) >= capacity {
            let new_capacity = if capacity == 0 {
                8
            } else {
                capacity.wrapping_add(capacity)
            };
            e.call(QUEUE_SET_CAPACITY, &args![queue, new_capacity]);
        }
        if e.mem.u32(queue + 0xc) == e.mem.u32(queue + 4) {
            e.mem.set_u32(queue + 0xc, 0);
        }
        let tail = e.mem.u32(queue + 0xc);
        let data = e.mem.u32(queue);
        e.mem.set_u32(data + tail * 4, body);
        e.mem.set_u32(queue + 0xc, tail.wrapping_add(1));
        let count = e.mem.u32(queue + 0x10);
        e.mem.set_u32(queue + 0x10, count.wrapping_add(1));

        let queue = e.mem.u32(gun + QUEUE);
        if e.mem.i32(queue + 0x10) > e.mem.i32(gun + MAX_BULLETS) {
            let oldest = queue_pop_front(e, queue);
            notify_listeners_backwards(e, gun, 0x10, oldest);
            if e.mem.u32(oldest + 8) != 0 {
                remove_and_release(e, world, oldest, f + 0x13);
            } else {
                e.call(RELEASE_REFERENCE, &args![oldest]);
            }
        }
    });
}

// Translated from 00e47390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees a queue's storage (no engine-map name; `this` is the queue): when
/// the capacity at `+4` is not zero, hands the data pointer at `+0` and
/// `capacity * 4` bytes to the allocator's free (slot 8).
pub fn queue_release_storage(e: &mut Engine, this: Ptr) {
    let queue = this.addr();
    let capacity = e.mem.u32(queue + 4);
    if capacity != 0 {
        let data = e.mem.u32(queue);
        deallocate(e, data, capacity.wrapping_mul(4));
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e4_6ea0, hkp_ball_gun_reset(Ptr, u32)),
        entry!(0x00e4_6f20, fn_00e46f20(Ptr, u32) -> Ptr),
        entry!(0x00e4_6f90, hkp_ball_gun_construct(Ptr, u32) -> Ptr),
        entry!(0x00e4_7020, hkp_ball_gun_destruct(Ptr)),
        entry!(0x00e4_70d0, hkp_ball_gun_fire_gun(Ptr, u32, Ptr)),
        entry!(0x00e4_7390, queue_release_storage(Ptr)),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const ALLOC_DOUBLE: u32 = 0x0900_0001;
    const FREE_DOUBLE: u32 = 0x0900_0002;
    const LISTENER_ADDED: u32 = 0x0900_0003;
    const LISTENER_REMOVED: u32 = 0x0900_0004;
    const LISTENER_FIRED: u32 = 0x0900_0005;
    const LISTENER_VTABLE: u32 = 0x0100_0100;
    const SWEEP_HIT: [f32; 4] = [5.0, 6.0, 7.0, 8.0];
    const VELOCITY: [f32; 4] = [9.0, 8.0, 7.0, 6.0];

    fn bits(v: Vec4) -> [u32; 4] {
        v.map(f32::to_bits)
    }

    /// An engine with the exe's data range mapped, an allocator whose slots 4 and 8
    /// are doubles, and a log every double appends `(address, arguments)` to.
    fn setup() -> (Engine, Log) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOC_DOUBLE);
        e.mem.set_u32(vtable + 8, FREE_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(ALLOC_DOUBLE, move |e, a| {
            sink.borrow_mut().push((ALLOC_DOUBLE, a[1..].to_vec()));
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        log_callees(
            &mut e,
            &log,
            &[FREE_DOUBLE, LISTENER_ADDED, LISTENER_REMOVED],
        );
        let sink = Rc::clone(&log);
        e.register_double(LISTENER_FIRED, move |e, a| {
            let mut words = vec![a[0]];
            words.extend(bits(load_vec(e, a[1])));
            words.extend(bits(load_vec(e, a[2])));
            sink.borrow_mut().push((LISTENER_FIRED, words));
            Ret::default()
        });
        e.put_vtable(
            LISTENER_VTABLE,
            &[0, 0, 0, LISTENER_ADDED, LISTENER_REMOVED, LISTENER_FIRED],
        );
        (e, log)
    }

    /// Doubles over callees that log their arguments and return 0.
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

    /// The (pointer, size) pairs freed through the allocator double (its `this` dropped).
    fn frees(log: &Log) -> Vec<Vec<u32>> {
        calls_to(log, FREE_DOUBLE)
            .into_iter()
            .map(|call| call[1..].to_vec())
            .collect()
    }

    /// A gun with one listener and a queue of capacity `capacity` whose
    /// ring starts at slot `head` and holds `bodies`. Returns the gun, the
    /// queue and the listener.
    fn gun_with_queue(e: &mut Engine, capacity: u32, head: u32, bodies: &[u32]) -> (u32, u32, u32) {
        let gun = e.mem.alloc(0x60);
        let listeners = e.mem.alloc(8);
        let listener = e.mem.alloc(8);
        e.mem.set_u32(listener, LISTENER_VTABLE);
        e.mem.set_u32(listeners, listener);
        e.mem.set_u32(gun + LISTENERS, listeners);
        e.mem.set_u32(gun + LISTENER_COUNT, 1);
        let queue = e.mem.alloc(0x14);
        let data = e.mem.alloc(4 * capacity);
        e.mem.set_u32(queue, data);
        e.mem.set_u32(queue + 4, capacity);
        e.mem.set_u32(queue + 8, head);
        e.mem
            .set_u32(queue + 0xc, (head + bodies.len() as u32) % capacity);
        e.mem.set_u32(queue + 0x10, bodies.len() as u32);
        for (i, body) in bodies.iter().enumerate() {
            e.mem
                .set_u32(data + 4 * ((head + i as u32) % capacity), *body);
        }
        e.mem.set_u32(gun + QUEUE, queue);
        (gun, queue, listener)
    }

    fn body_in_world(e: &mut Engine, world: u32) -> u32 {
        let body = e.mem.alloc(0x20);
        e.mem.set_u32(body + 8, world);
        body
    }

    #[test]
    fn reset_empties_the_queue_and_removes_only_bodies_of_that_world() {
        let (mut e, log) = setup();
        log_callees(&mut e, &log, &[REMOVE_ENTITY, RELEASE_REFERENCE]);
        let first = body_in_world(&mut e, 0x5000);
        let second = body_in_world(&mut e, 0x6000);
        let (gun, queue, listener) = gun_with_queue(&mut e, 4, 1, &[first, second]);
        e.call(0x00e4_6ea0, &args![gun, 0x5000u32]);
        assert_eq!(e.mem.u32(queue + 0x10), 0);
        assert_eq!(e.mem.u32(queue + 8), 3);
        assert_eq!(
            calls_to(&log, LISTENER_REMOVED),
            vec![vec![listener, first], vec![listener, second]]
        );
        let removed = calls_to(&log, REMOVE_ENTITY);
        assert_eq!(removed.len(), 1);
        assert_eq!((removed[0][0], removed[0][2]), (0x5000, first));
        assert_eq!(
            calls_to(&log, RELEASE_REFERENCE),
            vec![vec![first], vec![second]]
        );
    }

    #[test]
    fn reset_wraps_the_head_at_the_capacity() {
        let (mut e, log) = setup();
        log_callees(&mut e, &log, &[REMOVE_ENTITY, RELEASE_REFERENCE]);
        let body = body_in_world(&mut e, 0);
        let (gun, queue, _) = gun_with_queue(&mut e, 4, 3, &[body]);
        e.call(0x00e4_6ea0, &args![gun, 0x5000u32]);
        assert_eq!(e.mem.u32(queue + 8), 0);
        assert!(calls_to(&log, REMOVE_ENTITY).is_empty());
        assert_eq!(calls_to(&log, RELEASE_REFERENCE), vec![vec![body]]);
    }

    #[test]
    fn destructor_removes_each_body_from_its_own_world_and_frees_the_queue() {
        let (mut e, log) = setup();
        log_callees(
            &mut e,
            &log,
            &[REMOVE_ENTITY, RELEASE_REFERENCE, BASE_DESTRUCT],
        );
        let in_world = body_in_world(&mut e, 0x7000);
        let loose = body_in_world(&mut e, 0);
        let (gun, queue, _) = gun_with_queue(&mut e, 4, 1, &[in_world, loose]);
        let data = e.mem.u32(queue);
        e.call(0x00e4_7020, &args![gun]);
        assert_eq!(e.mem.u32(gun), BALL_GUN_VTABLE);
        let removed = calls_to(&log, REMOVE_ENTITY);
        assert_eq!(removed.len(), 1);
        assert_eq!((removed[0][0], removed[0][2]), (0x7000, in_world));
        assert_eq!(
            calls_to(&log, RELEASE_REFERENCE),
            vec![vec![in_world], vec![loose]]
        );
        assert_eq!(frees(&log), vec![vec![data, 16], vec![queue, 0x14]]);
        assert_eq!(calls_to(&log, BASE_DESTRUCT), vec![vec![gun]]);
    }

    #[test]
    fn destructor_of_an_empty_queue_frees_only_the_queue_block() {
        let (mut e, log) = setup();
        log_callees(&mut e, &log, &[BASE_DESTRUCT]);
        let gun = e.mem.alloc(0x60);
        let queue = e.mem.alloc(0x14);
        e.mem.set_u32(gun + QUEUE, queue);
        e.call(0x00e4_7020, &args![gun]);
        assert_eq!(frees(&log), vec![vec![queue, 0x14]]);
        assert_eq!(calls_to(&log, BASE_DESTRUCT), vec![vec![gun]]);
    }

    #[test]
    fn queue_release_storage_frees_capacity_times_four_bytes() {
        let (mut e, log) = setup();
        let queue = e.mem.alloc(0x14);
        e.call(0x00e4_7390, &args![queue]);
        assert!(frees(&log).is_empty());
        e.mem.set_u32(queue, 0x1234);
        e.mem.set_u32(queue + 4, 5);
        e.call(0x00e4_7390, &args![queue]);
        assert_eq!(frees(&log), vec![vec![0x1234, 20]]);
    }

    #[test]
    fn constructor_copies_the_constants_and_makes_the_queue() {
        let (mut e, log) = setup();
        log_callees(
            &mut e,
            &log,
            &[BASE_CONSTRUCT, STRING_ASSIGN, QUEUE_SET_CAPACITY],
        );
        e.register(QUEUE_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.mem.set_u32(CONSTANT_RADIUS, 0x3f00_0000);
        e.mem.set_u32(CONSTANT_SPEED, 0x7f80_0001);
        e.mem.set_u32(CONSTANT_MASS, 0x4120_0000);
        let gun = e.mem.alloc(0x60);
        e.mem.set_u32(gun + MUZZLE_OFFSET + 8, 0xdead_beef);
        let got = e.call(0x00e4_6f90, &args![gun, 7u32]).u32();
        assert_eq!(got, gun);
        assert_eq!(e.mem.u32(gun), BALL_GUN_VTABLE);
        assert_eq!(e.mem.u8(gun + 8), 1);
        assert_eq!(e.mem.u32(gun + SPHERE_RADIUS), 0x3f00_0000);
        assert_eq!(e.mem.u32(gun + MUZZLE_SPEED), 0x7fc0_0001);
        assert_eq!(e.mem.u32(gun + BULLET_MASS), 0x4120_0000);
        assert_eq!(e.mem.u32(gun + BULLET_VALUE), 0x4120_0000);
        assert_eq!(e.mem.u32(gun + MAX_BULLETS), 7);
        assert_eq!(load_vec(&e, gun + MUZZLE_OFFSET), [0.0; 4]);
        assert_eq!(
            calls_to(&log, STRING_ASSIGN),
            vec![vec![gun + 0xc, BALL_GUN_NAME]]
        );
        let queue = e.mem.u32(gun + QUEUE);
        assert_ne!(queue, 0);
        assert_eq!(calls_to(&log, ALLOC_DOUBLE), vec![vec![0x14]]);
        assert_eq!(calls_to(&log, QUEUE_SET_CAPACITY), vec![vec![queue, 7]]);
    }

    #[test]
    fn constructor_with_a_failed_allocation_still_sets_the_capacity_on_null() {
        let (mut e, log) = setup();
        log_callees(
            &mut e,
            &log,
            &[BASE_CONSTRUCT, STRING_ASSIGN, QUEUE_SET_CAPACITY],
        );
        e.register(ALLOC_DOUBLE, |_, _| Ret::default());
        let gun = e.mem.alloc(0x60);
        e.call(0x00e4_6f90, &args![gun, 3u32]);
        assert_eq!(e.mem.u32(gun + QUEUE), 0);
        assert_eq!(calls_to(&log, QUEUE_SET_CAPACITY), vec![vec![0, 3]]);
    }

    #[test]
    fn flagged_constructor_makes_the_queue_only_when_the_flag_is_set() {
        let (mut e, log) = setup();
        log_callees(&mut e, &log, &[BASE_CONSTRUCT_FLAGGED, QUEUE_SET_CAPACITY]);
        e.register(QUEUE_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let gun = e.mem.alloc(0x60);
        e.mem.set_u32(gun + MAX_BULLETS, 9);
        assert_eq!(e.call(0x00e4_6f20, &args![gun, 0u32]).u32(), gun);
        assert_eq!(e.mem.u32(gun), BALL_GUN_VTABLE);
        assert_eq!(e.mem.u8(gun + 8), 0);
        assert!(calls_to(&log, QUEUE_SET_CAPACITY).is_empty());
        assert_eq!(e.call(0x00e4_6f20, &args![gun, 1u32]).u32(), gun);
        assert_eq!(e.mem.u8(gun + 8), 1);
        let queue = e.mem.u32(gun + QUEUE);
        assert_ne!(queue, 0);
        assert_eq!(calls_to(&log, QUEUE_SET_CAPACITY), vec![vec![queue, 9]]);
        assert_eq!(
            calls_to(&log, BASE_CONSTRUCT_FLAGGED),
            vec![vec![gun, 0], vec![gun, 1]]
        );
    }

    /// What `fireGun` hands the body constructor, read when it runs.
    #[derive(Default)]
    struct Built {
        shape: u32,
        position: [u32; 4],
        velocity: [u32; 4],
        words: Vec<u32>,
        velocity_slot: u32,
    }

    struct Fire {
        e: Engine,
        log: Log,
        built: Rc<RefCell<Built>>,
        gun: u32,
        queue: u32,
        matrix: u32,
        world: u32,
    }

    /// A gun (radius 2, speed 10, mass 3, limit `max`) whose queue has
    /// capacity 2 and holds `bodies`; the sweep returns `sweep_result`.
    fn fire_setup(bodies: &[u32], max: u32, sweep_result: u32) -> Fire {
        let (mut e, log) = setup();
        log_callees(
            &mut e,
            &log,
            &[
                BODY_INFO_CONSTRUCT,
                SET_SHAPE_MASS_PROPERTIES,
                REMOVE_ENTITY,
                ADD_ENTITY,
                RELEASE_REFERENCE,
                QUEUE_SET_CAPACITY,
            ],
        );
        e.set_global(TARGET_DISTANCE, 200.0f32);
        e.set_global(CONSTANT_RADIUS, 0.5f32);
        e.set_global(CONSTANT_BODY_A, 0.25f32);
        e.set_global(CONSTANT_BODY_B, 0.75f32);
        let built = Rc::new(RefCell::new(Built::default()));
        let sink = Rc::clone(&log);
        e.register_double(SPHERE_CONSTRUCT, move |e, a| {
            sink.borrow_mut().push((SPHERE_CONSTRUCT, a.to_vec()));
            assert_eq!(e.mem.u16(a[0] + 4), 0x20);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let sink = Rc::clone(&log);
        e.register_double(SWEEP_SPHERE, move |e, a| {
            let mut words = a.to_vec();
            words.extend(bits(load_vec(e, a[1])));
            words.extend(bits(load_vec(e, a[3])));
            sink.borrow_mut().push((SWEEP_SPHERE, words));
            store_vec(e, a[4], SWEEP_HIT);
            Ret {
                eax: sweep_result,
                ..Ret::default()
            }
        });
        let sink = Rc::clone(&log);
        let seen = Rc::clone(&built);
        e.register_double(CALC_VELOCITY, move |e, a| {
            let mut words = a.to_vec();
            words.extend(bits(load_vec(e, a[0])));
            words.extend(bits(load_vec(e, a[1])));
            sink.borrow_mut().push((CALC_VELOCITY, words));
            seen.borrow_mut().velocity_slot = a[4];
            store_vec(e, a[4], VELOCITY);
            Ret::default()
        });
        let seen = Rc::clone(&built);
        e.register_double(RIGID_BODY_CONSTRUCT, move |e, a| {
            let info = a[1];
            let mut built = seen.borrow_mut();
            built.shape = e.mem.u32(info + 4);
            built.position = bits(load_vec(e, info + 0x10));
            built.velocity = bits(load_vec(e, info + 0x30));
            built.words = [0x90, 0x94, 0x98, 0xa0, 0xa4, 0xb0]
                .map(|offset| e.mem.u32(info + offset))
                .to_vec();
            built
                .words
                .extend([e.mem.u8(info + 0xb4), e.mem.u8(info + 0xb7)].map(u32::from));
            assert_eq!(e.mem.u16(a[0] + 4), 0x220);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let (gun, queue, _) = gun_with_queue(&mut e, 2, 0, bodies);
        e.mem.set_f32(gun + SPHERE_RADIUS, 2.0);
        e.mem.set_f32(gun + MUZZLE_SPEED, 10.0);
        e.mem.set_f32(gun + BULLET_MASS, 3.0);
        e.mem.set_u32(gun + BULLET_VALUE, 0x4000_0001);
        e.mem.set_u32(gun + MAX_BULLETS, max);
        store_vec(&mut e, gun + MUZZLE_OFFSET, [1.0, 2.0, 3.0, 0.0]);
        let matrix = e.mem.alloc(0x40);
        store_vec(&mut e, matrix, [1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, matrix + 0x30, [10.0, 20.0, 30.0, 0.0]);
        let world = e.mem.alloc(0x40);
        Fire {
            e,
            log,
            built,
            gun,
            queue,
            matrix,
            world,
        }
    }

    #[test]
    fn fire_gun_aims_straight_when_the_sweep_returns_nonzero() {
        let mut f = fire_setup(&[], 4, 1);
        f.e.call(0x00e4_70d0, &args![f.gun, f.world, f.matrix]);
        let sweep = calls_to(&f.log, SWEEP_SPHERE);
        assert_eq!(sweep.len(), 1);
        assert_eq!(sweep[0][0], f.world);
        assert_eq!(sweep[0][2], 2.0f32.to_bits());
        assert_eq!(sweep[0][5..9], bits([11.0, 20.0, 30.0, 0.0]));
        assert_eq!(sweep[0][9..13], bits([211.0, 20.0, 30.0, 0.0]));
        assert!(calls_to(&f.log, CALC_VELOCITY).is_empty());
        let built = f.built.borrow();
        assert_eq!(built.shape, calls_to(&f.log, SPHERE_CONSTRUCT)[0][0]);
        assert_eq!(calls_to(&f.log, SPHERE_CONSTRUCT)[0][1], 2.0f32.to_bits());
        assert_eq!(built.position, bits([12.0, 22.0, 33.0, 0.0]));
        assert_eq!(built.velocity, bits([10.0, 0.0, 0.0, 0.0]));
        assert_eq!(
            built.words,
            vec![
                3.0f32.to_bits(),
                0.0f32.to_bits(),
                0.75f32.to_bits(),
                1.0f32.to_bits(),
                0.5f32.to_bits(),
                0.25f32.to_bits(),
                2,
                6
            ]
        );
        let mass_call = calls_to(&f.log, SET_SHAPE_MASS_PROPERTIES);
        assert_eq!(mass_call[0][..2], [built.shape, 6.0f32.to_bits()]);
        assert_eq!(calls_to(&f.log, RELEASE_REFERENCE), vec![vec![built.shape]]);
        let fired = calls_to(&f.log, LISTENER_FIRED);
        assert_eq!(fired.len(), 1);
        assert_eq!(fired[0][1..5], bits([10.0, 20.0, 30.0, 0.0]));
        assert_eq!(fired[0][5..9], bits(SWEEP_HIT));
        let added = calls_to(&f.log, LISTENER_ADDED);
        assert_eq!(added.len(), 1);
        let body = added[0][1];
        assert_eq!(f.e.mem.u32(body + 0x9c), 0x4000_0001);
        assert_eq!(calls_to(&f.log, ADD_ENTITY), vec![vec![f.world, body, 1]]);
        assert_eq!(f.e.mem.u32(f.queue + 0x10), 1);
        assert_eq!(f.e.mem.u32(f.e.mem.u32(f.queue)), body);
        assert_eq!(f.e.mem.u32(f.queue + 0xc), 1);
        assert!(calls_to(&f.log, QUEUE_SET_CAPACITY).is_empty());
        assert!(calls_to(&f.log, REMOVE_ENTITY).is_empty());
    }

    #[test]
    fn fire_gun_computes_the_velocity_when_the_sweep_returns_zero() {
        let mut f = fire_setup(&[], 4, 0);
        f.e.call(0x00e4_70d0, &args![f.gun, f.world, f.matrix]);
        let velocity = calls_to(&f.log, CALC_VELOCITY);
        assert_eq!(velocity.len(), 1);
        assert_eq!(velocity[0][2], f.world + 0x10);
        assert_eq!(velocity[0][3], 10.0f32.to_bits());
        assert_eq!(velocity[0][5..9], bits([12.0, 22.0, 33.0, 0.0]));
        assert_eq!(velocity[0][9..13], bits(SWEEP_HIT));
        let built = f.built.borrow();
        assert_eq!(velocity[0][4], built.velocity_slot);
        assert_eq!(built.velocity, bits(VELOCITY));
    }

    #[test]
    fn fire_gun_drops_the_oldest_body_beyond_the_limit() {
        let mut probe = fire_setup(&[], 1, 1);
        let oldest = body_in_world(&mut probe.e, 0x7777);
        let queue = probe.queue;
        let data = probe.e.mem.u32(queue);
        probe.e.mem.set_u32(data, oldest);
        probe.e.mem.set_u32(queue + 0xc, 1);
        probe.e.mem.set_u32(queue + 0x10, 1);
        probe
            .e
            .call(0x00e4_70d0, &args![probe.gun, probe.world, probe.matrix]);
        // Count 1 < capacity 2: no growth; count 2 > limit 1 pops the oldest.
        assert!(calls_to(&probe.log, QUEUE_SET_CAPACITY).is_empty());
        assert_eq!(probe.e.mem.u32(queue + 0x10), 1);
        assert_eq!(probe.e.mem.u32(queue + 8), 1);
        let removed = calls_to(&probe.log, REMOVE_ENTITY);
        assert_eq!(removed.len(), 1);
        assert_eq!((removed[0][0], removed[0][2]), (probe.world, oldest));
        let announced = calls_to(&probe.log, LISTENER_REMOVED);
        assert_eq!(announced.len(), 1);
        assert_eq!(announced[0][1], oldest);
        assert_eq!(
            calls_to(&probe.log, RELEASE_REFERENCE).last(),
            Some(&vec![oldest])
        );
    }

    #[test]
    fn fire_gun_pops_a_body_without_a_world_without_removing_it() {
        let mut f = fire_setup(&[], 0, 1);
        f.e.call(0x00e4_70d0, &args![f.gun, f.world, f.matrix]);
        // Limit 0: the new body itself is popped; it has no world (+8 is 0).
        assert!(calls_to(&f.log, REMOVE_ENTITY).is_empty());
        assert_eq!(calls_to(&f.log, RELEASE_REFERENCE).len(), 2);
        assert_eq!(f.e.mem.u32(f.queue + 0x10), 0);
    }

    #[test]
    fn fire_gun_grows_a_full_queue_to_double_and_an_empty_one_to_eight() {
        for (bodies, capacity, expected) in [(&[0x11u32, 0x22][..], 2u32, 4u32), (&[][..], 0, 8)] {
            let mut f = fire_setup(bodies, 4, 1);
            let queue = f.queue;
            if capacity == 0 {
                f.e.mem.set_u32(queue + 4, 0);
            }
            let sink = Rc::clone(&f.log);
            f.e.register_double(QUEUE_SET_CAPACITY, move |e, a| {
                sink.borrow_mut().push((QUEUE_SET_CAPACITY, a.to_vec()));
                let data = e.mem.alloc(4 * a[1]);
                e.mem.set_u32(a[0], data);
                e.mem.set_u32(a[0] + 4, a[1]);
                Ret::default()
            });
            f.e.call(0x00e4_70d0, &args![f.gun, f.world, f.matrix]);
            assert_eq!(
                calls_to(&f.log, QUEUE_SET_CAPACITY),
                vec![vec![queue, expected]]
            );
            assert_eq!(f.e.mem.u32(queue + 4), expected);
            assert_eq!(f.e.mem.u32(queue + 0x10), bodies.len() as u32 + 1);
        }
    }
}
