//! `Havok SDK/hkpmountedballgun.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkpsphereshape::{copy_vec, load_vec, store_vec, Vec4};

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00e473c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e4_7020`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e473c0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e4_7020, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// The vtable of `hkpMountedBallGun` (`010ec004`).
const MOUNTED_BALL_GUN_VTABLE: u32 = 0x010e_c004;
/// The string `"MountedBallGun"` the constructor hands the string pointer at `+0x0C`.
const MOUNTED_BALL_GUN_NAME: u32 = 0x010e_c028;
/// `hkpBallGun`'s constructor (`00e46f90`, `this`, the bullet limit).
const BALL_GUN_CONSTRUCT: u32 = 0x00e4_6f90;
/// `hkStringPtr::operator=` (`00cb0c10`, one word).
const STRING_ASSIGN: u32 = 0x00cb_0c10;
/// The constants 0.0 and 100.0 the constructor puts in the vector at `+0x60`.
const VECTOR_FIRST_CONSTANT: u32 = 0x0101_1d78;
const VECTOR_SECOND_CONSTANT: u32 = 0x0101_6410;
/// `200.0f`: how far ahead of the muzzle the target point lies.
const TARGET_DISTANCE: u32 = 0x0102_226c;
/// Constants `fireGun` copies into the body description (`FLD` / `FSTP`).
const CONSTANT_RADIUS: u32 = 0x0101_df44;
const CONSTANT_BODY_A: u32 = 0x0101_3ea4;
const CONSTANT_BODY_B: u32 = 0x0102_31a8;
/// The radius of the sphere sweep (`0.05f`).
const SWEEP_RADIUS: u32 = 0x0101_81bc;
/// `hkpRigidBodyCinfo` constructor (`00cf5ea0`), `hkpSphereShape` constructor (`00cd8160`,
/// radius), `setShapeVolumeMassProperties` (`00cf07a0`, cdecl: shape, mass, description) and
/// `hkpRigidBody` constructor (`00ca8c90`, description).
const BODY_INFO_CONSTRUCT: u32 = 0x00cf_5ea0;
const SPHERE_CONSTRUCT: u32 = 0x00cd_8160;
const SET_SHAPE_MASS_PROPERTIES: u32 = 0x00cf_07a0;
const RIGID_BODY_CONSTRUCT: u32 = 0x00ca_8c90;
/// `hkpFirstPersonGun::sweepSphere` (`00e464e0`, cdecl: world, from, radius, to, hit) and
/// `calcVelocityToTarget` (`00e463d0`, cdecl: start, target, gravity, speed, velocity).
const SWEEP_SPHERE: u32 = 0x00e4_64e0;
const CALC_VELOCITY: u32 = 0x00e4_63d0;
/// `hkpWorld::addEntity` (`00c914d0`: world, entity, flag) and `removeEntity` (`00c91620`:
/// world, out pointer, entity); the reference-count release (`00c905b0`, `this` only).
const ADD_ENTITY: u32 = 0x00c9_14d0;
const REMOVE_ENTITY: u32 = 0x00c9_1620;
const RELEASE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkString::memCpy` (`00cb1260`: destination, source, size).
const MEMORY_COPY: u32 = 0x00cb_1260;
/// The queue's `setCapacity` (`00e45460`, one word).
const QUEUE_SET_CAPACITY: u32 = 0x00e4_5460;

const LISTENERS: u32 = 0x14;
const LISTENER_COUNT: u32 = 0x18;
const SPHERE_RADIUS: u32 = 0x20;
const MUZZLE_SPEED: u32 = 0x24;
const BULLET_MASS: u32 = 0x28;
const BULLET_VALUE: u32 = 0x2c;
const MAX_BULLETS: u32 = 0x30;
const QUEUE: u32 = 0x50;
/// The vector at `+0x60` of the mounted gun (copied to `+0x10` of the body description and
/// the first word of the objects' `0x14` call).
const MUZZLE_VECTOR: u32 = 0x60;

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

/// Calls virtual `slot` of each object of the gun's array (`+0x14` data, `+0x18` count, both
/// read again at each step) from the first to the last with `arguments`.
fn call_objects_forwards(e: &mut Engine, gun: u32, slot: u32, arguments: &[u32]) {
    let mut index = 0;
    while index < e.mem.i32(gun + LISTENER_COUNT) {
        let array = e.mem.u32(gun + LISTENERS);
        let object = e.mem.u32(array + (index as u32).wrapping_mul(4));
        e.vcall(object, slot, arguments);
        index += 1;
    }
}

// Translated from 00e45150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMountedBallGun::hkpMountedBallGun` (Xbox PDB): runs the `hkpBallGun` constructor with
/// `max_bullets`, sets the vtable `010ec004`, the byte at +8 to 4, gives the string pointer at
/// +0xC the name `"MountedBallGun"` and stores the vector `(0.0, 100.0, 0.0, 0.0)` (the two
/// constants at `01011d78` and `01016410`, bit for bit) at +0x60. Returns `this`.
pub fn hkp_mounted_ball_gun_hkp_mounted_ball_gun(
    e: &mut Engine,
    this: Ptr,
    max_bullets: u32,
) -> Ptr {
    let gun = this.addr();
    e.call(BALL_GUN_CONSTRUCT, &args![this, max_bullets]);
    e.mem.set_u32(gun, MOUNTED_BALL_GUN_VTABLE);
    e.mem.set_u8(gun + 8, 4);
    e.call(STRING_ASSIGN, &args![gun + 0xc, MOUNTED_BALL_GUN_NAME]);
    let zero = e.mem.u32(VECTOR_FIRST_CONSTANT);
    let hundred = e.mem.u32(VECTOR_SECOND_CONSTANT);
    e.mem.set_u32(gun + MUZZLE_VECTOR, zero);
    e.mem.set_u32(gun + MUZZLE_VECTOR + 4, hundred);
    e.mem.set_u32(gun + MUZZLE_VECTOR + 8, zero);
    e.mem.set_u32(gun + MUZZLE_VECTOR + 12, zero);
    this
}

// Translated from 00e451b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMountedBallGun::fireGun` (Xbox PDB): `world` and the gun's transform `matrix` (four
/// vectors; the first is the direction, the fourth the position). Builds a sphere body
/// (`radius` at `+0x20`, mass `2 * [+0x28]`) described with the vector at `+0x60` of the gun,
/// aims it at the first thing a sphere sweep (radius `0.05`) from `position + direction` to
/// `200 * direction + ...` hits (or straight along the direction at the muzzle speed when the
/// sweep returns nonzero), tells the objects of the array at `+0x14` (virtual `0x14`: the
/// vector at `+0x60` and the hit), makes the body, stores `[+0x2c]` at its `+0x9c`, tells the
/// objects (virtual `0x0C`), adds it to `world` and pushes it on the queue at `+0x50`
/// (growing it to 8 or twice its capacity when full). When the queue then holds more than
/// `[+0x30]` bodies the oldest is taken out, announced (virtual `0x10`, last object first),
/// removed from the world it is in (`+8`) and released.
pub fn hkp_mounted_ball_gun_fire_gun(e: &mut Engine, this: Ptr, world: u32, matrix: Ptr) {
    let gun = this.addr();
    e.with_stack(0x150, |e, frame| {
        // Frame (offsets from the aligned frame base): `+0x1b` the result slot of
        // `removeEntity`, `+0x1c` the doubled mass, `+0x20` from, `+0x30` hit copy, `+0x40`
        // to, `+0x50` direction, `+0x60` body description (0xb8 bytes), `+0x120` hit.
        let f = frame.addr();
        let (from, hit_copy, to, direction_slot, info, hit) =
            (f + 0x20, f + 0x30, f + 0x40, f + 0x50, f + 0x60, f + 0x120);
        let direction = load_vec(e, matrix.addr());
        let position = load_vec(e, matrix.addr() + 0x30);
        let distance: f32 = e.global(TARGET_DISTANCE);
        let sum: Vec4 = std::array::from_fn(|lane| position[lane] + direction[lane]);
        let far: Vec4 = std::array::from_fn(|lane| distance * direction[lane] + sum[lane]);
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
        let radius_default = copy_float_bits(e, CONSTANT_RADIUS);
        e.mem.set_u32(info + 0xa4, radius_default);
        e.mem.set_u32(info + 4, shape);
        e.mem.set_f32(info + 0xa0, 1.0);
        e.mem.set_u8(info + 0xb7, 6);
        let muzzle = load_vec(e, gun + MUZZLE_VECTOR);
        store_vec(e, info + 0x10, muzzle);
        // `FLD mass; FADD ST0,ST0; FSTP` then `FLD; FSTP` into the argument slot.
        let mass = f64::from(f32::from_bits(mass_bits));
        e.mem.set_f32(f + 0x1c, x87_quiet((mass + mass) as f32));
        e.mem.set_u8(info + 0xb4, 2);
        let double_mass = x87_quiet(e.mem.f32(f + 0x1c));
        e.call(SET_SHAPE_MASS_PROPERTIES, &args![shape, double_mass, info]);
        let mass_bits = copy_float_bits(e, gun + BULLET_MASS);
        e.mem.set_u32(info + 0x90, mass_bits);
        e.mem.set_f32(info + 0x94, 0.0);
        let constant_b = copy_float_bits(e, CONSTANT_BODY_B);
        e.mem.set_u32(info + 0x98, constant_b);
        let sweep_radius = copy_float_bits(e, SWEEP_RADIUS);
        let missed = e
            .call(SWEEP_SPHERE, &args![world, from, sweep_radius, to, hit])
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
        call_objects_forwards(e, gun, 0x14, &[gun + MUZZLE_VECTOR, hit_copy]);

        let body_block = allocate(e, 0x220);
        e.mem.set_u16(body_block + 4, 0x220);
        let body = e.call(RIGID_BODY_CONSTRUCT, &args![body_block, info]).u32();
        let shape = e.mem.u32(info + 4);
        e.call(RELEASE_REFERENCE, &args![shape]);
        let bullet_value = copy_float_bits(e, gun + BULLET_VALUE);
        e.mem.set_u32(body + 0x9c, bullet_value);
        call_objects_forwards(e, gun, 0xc, &[body]);
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
        e.mem.set_u32(data.wrapping_add(tail.wrapping_mul(4)), body);
        e.mem.set_u32(queue + 0xc, tail.wrapping_add(1));
        let count = e.mem.u32(queue + 0x10);
        e.mem.set_u32(queue + 0x10, count.wrapping_add(1));

        let queue = e.mem.u32(gun + QUEUE);
        if e.mem.i32(queue + 0x10) > e.mem.i32(gun + MAX_BULLETS) {
            let head = e.mem.u32(queue + 8);
            let data = e.mem.u32(queue);
            let oldest = e.mem.u32(data.wrapping_add(head.wrapping_mul(4)));
            let head = head.wrapping_add(1);
            e.mem.set_u32(queue + 8, head);
            if head == e.mem.u32(queue + 4) {
                e.mem.set_u32(queue + 8, 0);
            }
            let remaining = e.mem.u32(queue + 0x10).wrapping_sub(1);
            e.mem.set_u32(queue + 0x10, remaining);
            let mut index = e.mem.i32(gun + LISTENER_COUNT).wrapping_sub(1);
            while index >= 0 {
                let array = e.mem.u32(gun + LISTENERS);
                let object = e
                    .mem
                    .u32(array.wrapping_add((index as u32).wrapping_mul(4)));
                e.vcall(object, 0x10, &args![oldest]);
                index = index.wrapping_sub(1);
            }
            let oldest_world = e.mem.u32(oldest + 8);
            if oldest_world != 0 {
                e.call(REMOVE_ENTITY, &args![oldest_world, f + 0x1b, oldest]);
            }
            e.call(RELEASE_REFERENCE, &args![oldest]);
        }
    });
}

// Translated from 00e45460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkQueue<hkpRigidBody_P>::setCapacity` (Xbox PDB): the queue is `this` (data at +0,
/// capacity at +4, head at +8, tail at +0xC, count at +0x10). Does nothing when the capacity
/// is already at least `capacity`. Otherwise the new capacity is `capacity`, or twice the old
/// one when that is not smaller; a block of four bytes per element is allocated (allocator
/// slot 4). With old storage present the `count` elements are copied to the start of the new
/// block (`hkString::memCpy`: in one piece when the tail is above the head, else the part from
/// the head to the end and then the part up to the tail), the head is set to 0 and the tail to
/// the count (also when there are none). The old storage, when the old capacity is not zero,
/// is freed (slot 8, `capacity * 4` bytes); the new block and capacity are stored.
pub fn hk_queue_hkp_rigid_body_p_set_capacity(e: &mut Engine, this: Ptr, capacity: i32) {
    let queue = this.addr();
    let current = e.mem.i32(queue + 4);
    if current >= capacity {
        return;
    }
    let doubled = current.wrapping_add(current);
    let capacity = if doubled < capacity {
        capacity
    } else {
        doubled
    };
    let block = allocate(e, (capacity as u32).wrapping_mul(4));
    let old = e.mem.u32(queue);
    if old != 0 {
        let count = e.mem.u32(queue + 0x10);
        if count != 0 {
            let tail = e.mem.i32(queue + 0xc);
            let head = e.mem.i32(queue + 8);
            let from = old.wrapping_add((head as u32).wrapping_mul(4));
            if tail > head {
                e.call(MEMORY_COPY, &args![block, from, count.wrapping_mul(4)]);
            } else {
                let first = (e.mem.i32(queue + 4).wrapping_sub(head) as u32).wrapping_mul(4);
                e.call(MEMORY_COPY, &args![block, from, first]);
                let start = e.mem.u32(queue);
                let second = (tail as u32).wrapping_mul(4);
                e.call(
                    MEMORY_COPY,
                    &args![first.wrapping_add(block), start, second],
                );
            }
        }
        e.mem.set_u32(queue + 8, 0);
        let count = e.mem.u32(queue + 0x10);
        e.mem.set_u32(queue + 0xc, count);
    }
    let old_capacity = e.mem.u32(queue + 4);
    if old_capacity != 0 {
        let data = e.mem.u32(queue);
        deallocate(e, data, old_capacity.wrapping_mul(4));
    }
    e.mem.set_u32(queue, block);
    e.mem.set_u32(queue + 4, capacity as u32);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e4_73c0, deleting_destructor_00e473c0(Ptr, u8) -> Ptr),
        entry!(
            0x00e4_5150,
            hkp_mounted_ball_gun_hkp_mounted_ball_gun(Ptr, u32) -> Ptr
        ),
        entry!(0x00e4_51b0, hkp_mounted_ball_gun_fire_gun(Ptr, u32, Ptr)),
        entry!(
            0x00e4_5460,
            hk_queue_hkp_rigid_body_p_set_capacity(Ptr, i32)
        ),
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
    fn deleting_destructor_00e473c0_frees_only_when_bit_zero_is_set() {
        check(0x00e4_73c0, 0x00e4_7020);
    }

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const ALLOC_DOUBLE: u32 = 0x0900_0012;
    const LISTENER_ADDED: u32 = 0x0900_0013;
    const LISTENER_REMOVED: u32 = 0x0900_0014;
    const LISTENER_FIRED: u32 = 0x0900_0015;
    const SWEEP_HIT: [f32; 4] = [5.0, 6.0, 7.0, 8.0];
    const VELOCITY: [f32; 4] = [9.0, 8.0, 7.0, 6.0];

    fn bits(vector: Vec4) -> [u32; 4] {
        vector.map(f32::to_bits)
    }

    /// An engine with the exe data range mapped, an allocator (slots 4 and 8 are doubles that
    /// log) and a log every double appends `(address, arguments)` to.
    fn logging_engine() -> (Engine, Log) {
        let mut e = engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOC_DOUBLE);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
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
        log_callees(&mut e, &log, &[FREE_SLOT_DOUBLE]);
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

    #[test]
    fn constructor_names_the_gun_and_sets_the_vector() {
        let (mut e, log) = logging_engine();
        log_callees(&mut e, &log, &[BALL_GUN_CONSTRUCT, STRING_ASSIGN]);
        e.mem.set_f32(VECTOR_FIRST_CONSTANT, 0.0);
        e.mem.set_f32(VECTOR_SECOND_CONSTANT, 100.0);
        let gun = e.mem.alloc(0x70);
        for offset in (0..0x70).step_by(4) {
            e.mem.set_u32(gun + offset, 0xdead_beef);
        }
        assert_eq!(e.call(0x00e4_5150, &args![gun, 7u32]).u32(), gun);
        assert_eq!(calls_to(&log, BALL_GUN_CONSTRUCT), vec![vec![gun, 7]]);
        assert_eq!(
            calls_to(&log, STRING_ASSIGN),
            vec![vec![gun + 0xc, MOUNTED_BALL_GUN_NAME]]
        );
        assert_eq!(e.mem.u32(gun), MOUNTED_BALL_GUN_VTABLE);
        assert_eq!(e.mem.u8(gun + 8), 4);
        assert_eq!(e.mem.u8(gun + 9), 0xbe);
        assert_eq!(load_vec(&e, gun + 0x60), [0.0, 100.0, 0.0, 0.0]);
        assert_eq!(e.mem.u32(gun + 0x5c), 0xdead_beef);
    }

    /// What `fireGun` hands the body constructor, read when it runs.
    #[derive(Default)]
    struct Built {
        shape: u32,
        position: [u32; 4],
        velocity: [u32; 4],
        words: Vec<u32>,
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

    /// A gun (radius 2, speed 10, mass 3, limit `max`) with one listener whose queue has
    /// capacity 2 and holds `bodies`; the sweep returns `sweep_result`.
    fn fire_setup(bodies: &[u32], max: u32, sweep_result: u32) -> Fire {
        let (mut e, log) = logging_engine();
        log_callees(
            &mut e,
            &log,
            &[
                BODY_INFO_CONSTRUCT,
                REMOVE_ENTITY,
                ADD_ENTITY,
                RELEASE_REFERENCE,
                QUEUE_SET_CAPACITY,
                LISTENER_ADDED,
                LISTENER_REMOVED,
            ],
        );
        e.set_global(TARGET_DISTANCE, 200.0f32);
        e.set_global(CONSTANT_RADIUS, 0.5f32);
        e.set_global(CONSTANT_BODY_A, 0.25f32);
        e.set_global(CONSTANT_BODY_B, 0.75f32);
        e.set_global(SWEEP_RADIUS, 0.125f32);
        let built = Rc::new(RefCell::new(Built::default()));
        let sink = Rc::clone(&log);
        e.register_double(SET_SHAPE_MASS_PROPERTIES, move |e, a| {
            sink.borrow_mut()
                .push((SET_SHAPE_MASS_PROPERTIES, a.to_vec()));
            // The real function leaves its own values in these words.
            e.mem.set_u32(a[2] + 0x90, 0x1111_1111);
            e.mem.set_u32(a[2] + 0x94, 0x2222_2222);
            e.mem.set_u32(a[2] + 0x98, 0x3333_3333);
            Ret::default()
        });
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
        e.register_double(CALC_VELOCITY, move |e, a| {
            let mut words = a.to_vec();
            words.extend(bits(load_vec(e, a[0])));
            words.extend(bits(load_vec(e, a[1])));
            sink.borrow_mut().push((CALC_VELOCITY, words));
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
        let sink = Rc::clone(&log);
        e.register_double(LISTENER_FIRED, move |e, a| {
            let mut words = vec![a[0], a[1]];
            words.extend(bits(load_vec(e, a[2])));
            sink.borrow_mut().push((LISTENER_FIRED, words));
            Ret::default()
        });
        let gun = e.mem.alloc(0x70);
        let listener_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(listener_vtable + 0xc, LISTENER_ADDED);
        e.mem.set_u32(listener_vtable + 0x10, LISTENER_REMOVED);
        e.mem.set_u32(listener_vtable + 0x14, LISTENER_FIRED);
        let listener = e.mem.alloc(8);
        e.mem.set_u32(listener, listener_vtable);
        let listeners = e.mem.alloc(8);
        e.mem.set_u32(listeners, listener);
        e.mem.set_u32(gun + LISTENERS, listeners);
        e.mem.set_u32(gun + LISTENER_COUNT, 1);
        let queue = e.mem.alloc(0x14);
        let data = e.mem.alloc(8);
        e.mem.set_u32(queue, data);
        e.mem.set_u32(queue + 4, 2);
        e.mem.set_u32(queue + 0xc, bodies.len() as u32 % 2);
        e.mem.set_u32(queue + 0x10, bodies.len() as u32);
        for (index, body) in bodies.iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, *body);
        }
        e.mem.set_u32(gun + QUEUE, queue);
        e.mem.set_f32(gun + SPHERE_RADIUS, 2.0);
        e.mem.set_f32(gun + MUZZLE_SPEED, 10.0);
        e.mem.set_f32(gun + BULLET_MASS, 3.0);
        e.mem.set_u32(gun + BULLET_VALUE, 0x4000_0001);
        e.mem.set_u32(gun + MAX_BULLETS, max);
        store_vec(&mut e, gun + MUZZLE_VECTOR, [1.0, 2.0, 3.0, 4.0]);
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
        f.e.call(0x00e4_51b0, &args![f.gun, f.world, f.matrix]);
        let sweep = calls_to(&f.log, SWEEP_SPHERE);
        assert_eq!(sweep.len(), 1);
        assert_eq!(sweep[0][0], f.world);
        assert_eq!(sweep[0][2], 0.125f32.to_bits());
        assert_eq!(sweep[0][5..9], bits([11.0, 20.0, 30.0, 0.0]));
        assert_eq!(sweep[0][9..13], bits([211.0, 20.0, 30.0, 0.0]));
        assert!(calls_to(&f.log, CALC_VELOCITY).is_empty());
        let built = f.built.borrow();
        assert_eq!(built.shape, calls_to(&f.log, SPHERE_CONSTRUCT)[0][0]);
        assert_eq!(calls_to(&f.log, SPHERE_CONSTRUCT)[0][1], 2.0f32.to_bits());
        assert_eq!(built.position, bits([1.0, 2.0, 3.0, 4.0]));
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
        assert_eq!(fired[0][1], f.gun + 0x60);
        assert_eq!(fired[0][2..6], bits(SWEEP_HIT));
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
        f.e.call(0x00e4_51b0, &args![f.gun, f.world, f.matrix]);
        let velocity = calls_to(&f.log, CALC_VELOCITY);
        assert_eq!(velocity.len(), 1);
        assert_eq!(velocity[0][2], f.world + 0x10);
        assert_eq!(velocity[0][3], 10.0f32.to_bits());
        assert_eq!(velocity[0][5..9], bits([1.0, 2.0, 3.0, 4.0]));
        assert_eq!(velocity[0][9..13], bits(SWEEP_HIT));
        assert_eq!(f.built.borrow().velocity, bits(VELOCITY));
    }

    #[test]
    fn fire_gun_drops_the_oldest_body_beyond_the_limit() {
        let mut f = fire_setup(&[], 1, 1);
        let oldest = f.e.mem.alloc(0x20);
        f.e.mem.set_u32(oldest + 8, 0x7777);
        let data = f.e.mem.u32(f.queue);
        f.e.mem.set_u32(data, oldest);
        f.e.mem.set_u32(f.queue + 0xc, 1);
        f.e.mem.set_u32(f.queue + 0x10, 1);
        f.e.call(0x00e4_51b0, &args![f.gun, f.world, f.matrix]);
        // Count 1 < capacity 2: no growth; count 2 > limit 1 pops the oldest.
        assert!(calls_to(&f.log, QUEUE_SET_CAPACITY).is_empty());
        assert_eq!(f.e.mem.u32(f.queue + 0x10), 1);
        assert_eq!(f.e.mem.u32(f.queue + 8), 1);
        let removed = calls_to(&f.log, REMOVE_ENTITY);
        assert_eq!(removed.len(), 1);
        assert_eq!((removed[0][0], removed[0][2]), (0x7777, oldest));
        let announced = calls_to(&f.log, LISTENER_REMOVED);
        assert_eq!(announced.len(), 1);
        assert_eq!(announced[0][1], oldest);
        assert_eq!(
            calls_to(&f.log, RELEASE_REFERENCE).last(),
            Some(&vec![oldest])
        );
    }

    #[test]
    fn fire_gun_pops_a_body_without_a_world_without_removing_it() {
        let mut f = fire_setup(&[], 0, 1);
        f.e.call(0x00e4_51b0, &args![f.gun, f.world, f.matrix]);
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
            f.e.call(0x00e4_51b0, &args![f.gun, f.world, f.matrix]);
            assert_eq!(
                calls_to(&f.log, QUEUE_SET_CAPACITY),
                vec![vec![queue, expected]]
            );
            assert_eq!(f.e.mem.u32(queue + 4), expected);
            assert_eq!(f.e.mem.u32(queue + 0x10), bodies.len() as u32 + 1);
        }
    }

    /// A queue with capacity `capacity` (ring starting at `head`) holding `values`, and an
    /// engine whose allocator and copy are doubles.
    fn queue_setup(capacity: u32, head: u32, values: &[u32]) -> (Engine, Log, u32) {
        let (mut e, log) = logging_engine();
        e.register_double(MEMORY_COPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        let queue = e.mem.alloc(0x14);
        if capacity != 0 {
            let data = e.mem.alloc(4 * capacity);
            e.mem.set_u32(queue, data);
            for (index, value) in values.iter().enumerate() {
                let slot = (head + index as u32) % capacity;
                e.mem.set_u32(data + 4 * slot, *value);
            }
            e.mem
                .set_u32(queue + 0xc, (head + values.len() as u32) % capacity);
        }
        e.mem.set_u32(queue + 4, capacity);
        e.mem.set_u32(queue + 8, head);
        e.mem.set_u32(queue + 0x10, values.len() as u32);
        (e, log, queue)
    }

    fn queue_words(e: &Engine, queue: u32, count: u32) -> Vec<u32> {
        let data = e.mem.u32(queue);
        (0..count)
            .map(|index| e.mem.u32(data + 4 * index))
            .collect()
    }

    #[test]
    fn set_capacity_does_nothing_when_the_queue_is_big_enough() {
        let (mut e, log, queue) = queue_setup(4, 1, &[1, 2]);
        e.call(0x00e4_5460, &args![queue, 4i32]);
        e.call(0x00e4_5460, &args![queue, 3i32]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(queue + 4), 4);
    }

    #[test]
    fn set_capacity_copies_a_contiguous_range_to_the_start() {
        let (mut e, log, queue) = queue_setup(4, 1, &[10, 20]);
        let old = e.mem.u32(queue);
        e.call(0x00e4_5460, &args![queue, 8i32]);
        assert_eq!(e.mem.u32(queue + 4), 8);
        assert_eq!(queue_words(&e, queue, 2), [10, 20]);
        assert_eq!(e.mem.u32(queue + 8), 0);
        assert_eq!(e.mem.u32(queue + 0xc), 2);
        assert_eq!(calls_to(&log, ALLOC_DOUBLE), vec![vec![32]]);
        let freed = calls_to(&log, FREE_SLOT_DOUBLE);
        assert_eq!(freed.len(), 1);
        assert_eq!(freed[0][1..], [old, 16]);
    }

    #[test]
    fn set_capacity_copies_a_wrapped_range_in_two_pieces() {
        let (mut e, _, queue) = queue_setup(4, 3, &[1, 2, 3]);
        e.call(0x00e4_5460, &args![queue, 5i32]);
        // The doubled capacity 8 is not smaller than 5.
        assert_eq!(e.mem.u32(queue + 4), 8);
        assert_eq!(queue_words(&e, queue, 3), [1, 2, 3]);
        assert_eq!(e.mem.u32(queue + 8), 0);
        assert_eq!(e.mem.u32(queue + 0xc), 3);
    }

    #[test]
    fn set_capacity_of_a_full_queue_copies_the_whole_ring() {
        let (mut e, _, queue) = queue_setup(2, 1, &[7, 8]);
        e.call(0x00e4_5460, &args![queue, 9i32]);
        assert_eq!(e.mem.u32(queue + 4), 9);
        assert_eq!(queue_words(&e, queue, 2), [7, 8]);
        assert_eq!(e.mem.u32(queue + 0xc), 2);
    }

    #[test]
    fn set_capacity_of_an_empty_queue_resets_the_head_and_tail() {
        let (mut e, log, queue) = queue_setup(4, 2, &[]);
        e.mem.set_u32(queue + 0xc, 2);
        e.call(0x00e4_5460, &args![queue, 6i32]);
        assert_eq!(e.mem.u32(queue + 4), 8);
        assert_eq!(e.mem.u32(queue + 8), 0);
        assert_eq!(e.mem.u32(queue + 0xc), 0);
        assert_eq!(calls_to(&log, FREE_SLOT_DOUBLE).len(), 1);
    }

    #[test]
    fn set_capacity_without_old_storage_frees_nothing() {
        let (mut e, log, queue) = queue_setup(0, 0, &[]);
        e.call(0x00e4_5460, &args![queue, 8i32]);
        assert_eq!(e.mem.u32(queue + 4), 8);
        assert_ne!(e.mem.u32(queue), 0);
        assert!(calls_to(&log, FREE_SLOT_DOUBLE).is_empty());
        assert_eq!(calls_to(&log, ALLOC_DOUBLE), vec![vec![32]]);
    }
}
