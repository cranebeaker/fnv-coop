//! `Havok SDK/hkpfirstpersongun.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpFirstPersonGun` is not in the Xbox PDB; the layout is read off the code (descriptive,
//! unconfirmed): a string (`hkStringPtr`) at `+0xC`, a byte at `+0x10` and an array of
//! reference-counted listeners (data `+0x14`, size `+0x18`, capacity word `+0x1c`). SSE code
//! computes lane by lane in `f32` (`SQRTSS` is the exact `f32` square root); x87 code rounds at
//! each `float` store and a `float` copied through `FLD`/`FSTP` goes through `x87_quiet`.

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkpsphereshape::{copy_vec, load_vec, shufps, store_vec, sum3, unpckhps, zip_with};

/// The vtable the constructors install.
const VTABLE: u32 = 0x010e_c0e4;
/// The vtable the destructor ends with.
const BASE_VTABLE: u32 = 0x0102_e388;
/// The string the default constructor gives the `hkStringPtr` at `+0xC`.
const DEFAULT_NAME: u32 = 0x0101_1584;
/// `hkStringPtr` constructor from a string (`00cb0b80`, `this` in `ECX`, the string), from a
/// flag (`00cb0a70`, the flag) and its destructor (`00cb0c00`).
const STRING_FROM_TEXT: u32 = 0x00cb_0b80;
const STRING_FROM_FLAG: u32 = 0x00cb_0a70;
const STRING_DESTRUCTOR: u32 = 0x00cb_0c00;
/// `hkReferencedObject` reference release (`00c905b0`, `this` in `ECX`).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// The memory router (`00c85750`): its `+0x10` is the allocator (slot `4` allocates, slot
/// `0x10` frees storage).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The empty `hkArray` capacity word.
const EMPTY_CAPACITY: u32 = 0x8000_0000;
/// `hkpSphereShape` constructor (`00cd8160`, `this` in `ECX`, the radius), `hkpCollidable`
/// constructor (`00cd2d80`, `this` in `ECX`: shape, transform, a flag byte) and
/// `hkpWorld::linearCast` (`00c92430`, `this` = world: collidable, input, collector, start
/// point collector).
const SPHERE_CONSTRUCTOR: u32 = 0x00cd_8160;
const COLLIDABLE_CONSTRUCTOR: u32 = 0x00cd_2d80;
const LINEAR_CAST: u32 = 0x00c9_2430;
/// The vtable of the collector `sweepSphere` casts with.
const COLLECTOR_VTABLE: u32 = 0x010c_476c;
/// `float` constants: the epsilon (`2^-23`) and the "no hit yet" distance, and the vector
/// `sweepSphere` puts in the normal when nothing was hit.
const EPSILON: u32 = 0x0104_da4c;
const LARGE_DISTANCE: u32 = 0x0104_da48;
const NO_HIT_NORMAL: u32 = 0x0126_7e30;
/// The `double` `calcVelocityToTarget` multiplies the flight time by.
const TIME_FACTOR: u32 = 0x0101_1598;
/// Frame offsets of `sweepSphere`'s locals (the code addresses them from the aligned stack
/// pointer after `SUB ESP, 0x138`).
const FRAME: u32 = 0x138;
const FRAME_TRANSFORM: u32 = 0x08;
const FRAME_INPUT: u32 = 0x48;
const FRAME_COLLECTOR: u32 = 0x68;
const FRAME_HIT_COUNT: u32 = 0x98;
const FRAME_COLLIDABLE: u32 = 0xe8;

// Translated from 00e463b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpFirstPersonGun::getName` (Ghidra name): the word at `+0xC` with bit 0 cleared.
pub fn hkp_first_person_gun_get_name(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0xc) & !1
}

// Translated from 00e463c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpFirstPersonGun::quitGun` (Ghidra name): jumps to the object's own virtual function in
/// slot `0x20`; the caller gets that function's `EAX`.
pub fn hkp_first_person_gun_quit_gun(e: &mut Engine, this: Ptr) -> u32 {
    e.vcall(this.addr(), 0x20, &args![]).u32()
}

// Translated from 00e463d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpFirstPersonGun::calcVelocityToTarget` (Ghidra name), a `cdecl` function of five words:
/// the addresses of the start point, the target point and a vector (gravity), a speed (`float`)
/// and the address of the result. With `d = target - start` and its length `n` (`SQRTSS`): when
/// `n` is not above the epsilon at `0104da4c` the result is `d`. Otherwise the flight time is
/// `n / speed` (a `float`), the correction is that time times the `double` at `01011598`
/// times the vector, scaled down to length `speed` if it is longer, and the result is
/// `d * (speed / n) + correction`.
pub fn hkp_first_person_gun_calc_velocity_to_target(
    e: &mut Engine,
    start: Ptr,
    target: Ptr,
    gravity: Ptr,
    speed: f32,
    out: Ptr,
) {
    let epsilon: f32 = e.global(EPSILON);
    let delta = zip_with(
        load_vec(e, target.addr()),
        load_vec(e, start.addr()),
        |p, q| p - q,
    );
    let length = sum3(zip_with(delta, delta, |p, q| p * q)).sqrt();
    if length.partial_cmp(&epsilon) != Some(std::cmp::Ordering::Greater) {
        store_vec(e, out.addr(), delta);
        return;
    }
    let time = (f64::from(length) / f64::from(speed)) as f32;
    let factor: f64 = e.global(TIME_FACTOR);
    let scale = (f64::from(time) * factor) as f32;
    let gravity = load_vec(e, gravity.addr());
    let mut correction = gravity.map(|lane| scale * lane);
    let correction_length = sum3(zip_with(correction, correction, |p, q| p * q)).sqrt();
    if correction_length.partial_cmp(&speed) == Some(std::cmp::Ordering::Greater) {
        let shrink = (f64::from(speed) / f64::from(correction_length)) as f32;
        correction = correction.map(|lane| shrink * lane);
    }
    let direction_scale = (f64::from(speed) / f64::from(length)) as f32;
    let velocity = zip_with(
        delta.map(|lane| direction_scale * lane),
        correction,
        |p, q| p + q,
    );
    store_vec(e, out.addr(), velocity);
}

// Translated from 00e464e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpFirstPersonGun::sweepSphere` (Ghidra name), a `cdecl` function of five words: a world,
/// the addresses of the start and end points, a radius (`float`) and the address of the result
/// (0x28 bytes). Allocates and constructs a sphere shape of that radius (slot `4` of the
/// memory router's allocator, 0x20 bytes, the size word at `+4` set to 0x20), builds an
/// `hkpCollidable` for it with an identity rotation at the start point, runs
/// `hkpWorld::linearCast` towards the end point (epsilon penetration and tolerance, a collector
/// whose distance word starts at the large constant `0104da48` and hit count at 0) and releases
/// the sphere. With a hit it copies the collector's 32 bytes of contact data to the result,
/// the hit shape key to `+0x20` and, when the hit body's type byte (`+0x18`) is 1, the body
/// plus its signed owner offset (`+0x10`) to `+0x24` (else 0), and returns 0. Without a hit it
/// copies the end point to the result, puts the first three lanes of the vector at `01267e30`
/// in `+0x10` (keeping lane 3), `0xffffffff` in `+0x20` and 0 in `+0x24`, and returns 1.
pub fn hkp_first_person_gun_sweep_sphere(
    e: &mut Engine,
    world: Ptr,
    from: Ptr,
    radius: f32,
    to: Ptr,
    hit: Ptr,
) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let sphere = e.vcall(allocator, 4, &args![0x20u32]).u32();
    e.mem.set_u16(sphere + 4, 0x20);
    e.call(SPHERE_CONSTRUCTOR, &args![sphere, x87_quiet(radius)]);
    e.with_stack(FRAME, |e, frame| {
        let frame = frame.addr();
        let transform = frame + FRAME_TRANSFORM;
        for (column, value) in [(0, 0), (1, 1), (2, 2)] {
            store_vec(e, transform + 0x10 * column, [0.0; 4]);
            e.mem.set_f32(transform + 0x10 * column + 4 * value, 1.0);
        }
        copy_vec(e, from.addr(), transform + 0x30);
        let collidable = frame + FRAME_COLLIDABLE;
        e.call(
            COLLIDABLE_CONSTRUCTOR,
            &args![collidable, sphere, transform, 0u32],
        );
        let large = x87_quiet_bits(e.mem.u32(LARGE_DISTANCE));
        e.mem.set_u32(frame + 0x94, large);
        let epsilon = x87_quiet_bits(e.mem.u32(EPSILON));
        e.mem.set_u32(frame + FRAME_INPUT + 0x10, epsilon);
        e.mem.set_u32(frame + FRAME_INPUT + 0x14, epsilon);
        e.mem.set_u32(frame + FRAME_COLLECTOR + 4, large);
        let input = frame + FRAME_INPUT;
        copy_vec(e, to.addr(), input);
        let collector = frame + FRAME_COLLECTOR;
        e.mem.set_u32(collector, COLLECTOR_VTABLE);
        e.mem.set_u32(frame + FRAME_HIT_COUNT, 0);
        e.call(
            LINEAR_CAST,
            &args![world, collidable, input, collector, 0u32],
        );
        e.call(REMOVE_REFERENCE, &args![sphere]);
        if e.mem.u32(frame + FRAME_HIT_COUNT) != 0 {
            let out = hit.addr();
            copy_vec(e, frame + 0x78, out);
            copy_vec(e, frame + 0x88, out + 0x10);
            let body = e.mem.u32(frame + 0xc0);
            let key = e.mem.u32(frame + 0xc4);
            let owner = if e.mem.u8(body + 0x18) == 1 {
                let offset = i32::from(e.mem.u8(body + 0x10) as i8);
                body.wrapping_add(offset as u32)
            } else {
                0
            };
            e.mem.set_u32(out + 0x24, owner);
            e.mem.set_u32(out + 0x20, key);
            return 0;
        }
        let out = hit.addr();
        e.mem.set_u32(out + 0x24, 0);
        copy_vec(e, to.addr(), out);
        let constant = load_vec(e, NO_HIT_NORMAL);
        let previous = load_vec(e, out + 0x10);
        let normal = shufps(constant, unpckhps(constant, previous), 0xc4);
        store_vec(e, out + 0x10, normal);
        e.mem.set_u32(out + 0x20, 0xffff_ffff);
        1
    })
}

// Translated from 00e46660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpFirstPersonGun::hkpFirstPersonGun` (Ghidra name): `ECX` = object, nothing on the stack.
/// Sets the 16-bit word at `+6` to 1, the vtable `010ec0e4`, the byte at `+8` to 0, builds the
/// `hkStringPtr` at `+0xC` from the string at `01011584`, and sets the byte at `+0x10` to
/// `0x71` and the listener array (`+0x14`, `+0x18`, `+0x1c`) to empty. Returns `this`.
pub fn hkp_first_person_gun_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me, VTABLE);
    e.mem.set_u8(me + 8, 0);
    e.call(STRING_FROM_TEXT, &args![me + 0xc, DEFAULT_NAME]);
    e.mem.set_u8(me + 0x10, 0x71);
    e.mem.set_u32(me + 0x14, 0);
    e.mem.set_u32(me + 0x18, 0);
    e.mem.set_u32(me + 0x1c, EMPTY_CAPACITY);
    this
}

// Translated from 00e466a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor taking a flag (no name in the engine map; `ECX` = object, one stack word,
/// `flag`). Sets the vtable `010ec0e4`, builds the `hkStringPtr` at `+0xC` with `flag`
/// (`00cb0a70`), empties the listener array (`+0x14`, `+0x18`, `+0x1c`) and, when `flag` is
/// not 0, clears the byte at `+8`. Returns `this`.
pub fn fn_00e466a0(e: &mut Engine, this: Ptr, flag: u32) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me, VTABLE);
    e.call(STRING_FROM_FLAG, &args![me + 0xc, flag]);
    e.mem.set_u32(me + 0x14, 0);
    e.mem.set_u32(me + 0x18, 0);
    e.mem.set_u32(me + 0x1c, EMPTY_CAPACITY);
    if flag != 0 {
        e.mem.set_u8(me + 8, 0);
    }
    this
}

// Translated from 00e466e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpFirstPersonGun::~hkpFirstPersonGun` (Ghidra name): `ECX` = object. Sets the vtable
/// `010ec0e4`, releases every listener of the array (data `+0x14`, size `+0x18`; no null
/// check), frees the array's storage through the memory router's allocator (slot `0x10`,
/// capacity times 4 bytes, unless bit 31 of the capacity word `+0x1c` is set), empties the
/// array, runs the `hkStringPtr` destructor on `+0xC` and sets the vtable `0102e388`.
pub fn hkp_first_person_gun_destructor(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    e.mem.set_u32(me, VTABLE);
    let mut index = 0u32;
    if e.mem.i32(me + 0x18) > 0 {
        loop {
            let listener = e.mem.u32(e.mem.u32(me + 0x14) + 4 * index);
            e.call(REMOVE_REFERENCE, &args![listener]);
            index += 1;
            if index as i32 >= e.mem.i32(me + 0x18) {
                break;
            }
        }
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(me + 0x1c);
    e.mem.set_u32(me + 0x18, 0);
    if capacity as i32 >= 0 {
        let data = e.mem.u32(me + 0x14);
        e.vcall(allocator, 0x10, &args![data, (capacity & 0x3fff_ffff) * 4]);
    }
    e.mem.set_u32(me + 0x14, 0);
    e.mem.set_u32(me + 0x1c, EMPTY_CAPACITY);
    e.call(STRING_DESTRUCTOR, &args![me + 0xc]);
    e.mem.set_u32(me, BASE_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e463b0, hkp_first_person_gun_get_name(Ptr) -> u32),
        entry!(0x00e463c0, hkp_first_person_gun_quit_gun(Ptr) -> u32),
        entry!(
            0x00e463d0,
            hkp_first_person_gun_calc_velocity_to_target(Ptr, Ptr, Ptr, f32, Ptr)
        ),
        entry!(
            0x00e464e0,
            hkp_first_person_gun_sweep_sphere(Ptr, Ptr, f32, Ptr, Ptr) -> u32
        ),
        entry!(0x00e46660, hkp_first_person_gun_constructor(Ptr) -> Ptr),
        entry!(0x00e466a0, fn_00e466a0(Ptr, u32) -> Ptr),
        entry!(0x00e466e0, hkp_first_person_gun_destructor(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, logging_object, new_log,
    };
    use crate::units::havok_sdk::hkpsphereshape::Vec4;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn name_is_the_string_pointer_without_its_flag_bit() {
        let mut e = engine();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, 0x1235);
        assert_eq!(e.call(0x00e4_63b0, &args![this]).u32(), 0x1234);
    }

    #[test]
    fn quit_gun_jumps_to_slot_0x20() {
        let mut e = engine();
        let log = new_log();
        let this = logging_object(&mut e, &log, 0x0900_0000);
        e.register_double(0x0900_0020, |_, _| Ret {
            eax: 7,
            ..Ret::default()
        });
        assert_eq!(e.call(0x00e4_63c0, &args![this]).u32(), 7);
    }

    fn velocity(e: &mut Engine, target: Vec4, gravity: Vec4, speed: f32) -> Vec4 {
        e.set_global(EPSILON, f32::from_bits(0x3400_0000));
        e.set_global(TIME_FACTOR, -0.5f64);
        let start = e.mem.alloc(0x10);
        let target_at = e.mem.alloc(0x10);
        store_vec(e, target_at, target);
        let gravity_at = e.mem.alloc(0x10);
        store_vec(e, gravity_at, gravity);
        let out = e.mem.alloc(0x10);
        e.call(
            0x00e4_63d0,
            &args![start, target_at, gravity_at, speed, out],
        );
        load_vec(e, out)
    }

    #[test]
    fn velocity_aims_at_the_target_and_lifts_against_gravity() {
        let mut e = engine();
        // Flight time 10 / 20 = 0.5, the correction -0.25 * (0, 0, -10) is shorter than 20.
        let lofted = velocity(&mut e, [10.0, 0.0, 0.0, 0.0], [0.0, 0.0, -10.0, 0.0], 20.0);
        assert_eq!(lofted, [20.0, 0.0, 2.5, 0.0]);
        // At speed 5 the correction (0, 0, 10) is longer than the speed and shrinks to length 5.
        let slow = velocity(&mut e, [10.0, 0.0, 0.0, 0.0], [0.0, 0.0, -10.0, 0.0], 5.0);
        assert_eq!(slow, [5.0, 0.0, 5.0, 0.0]);
    }

    #[test]
    fn velocity_to_a_target_at_the_start_is_the_offset() {
        let mut e = engine();
        let tiny = velocity(&mut e, [1.0e-8, 0.0, 0.0, 0.0], [0.0, 0.0, -10.0, 0.0], 5.0);
        assert_eq!(tiny, [1.0e-8, 0.0, 0.0, 0.0]);
    }

    /// What the cast saw and what it should leave in the collector.
    #[derive(Default)]
    struct Seen {
        sphere_radius: u32,
        transform: Vec<Vec4>,
        input: Vec4,
        tolerances: (u32, u32),
        collector_words: (u32, u32),
        released: Vec<u32>,
    }

    /// Doubles for the whole sweep; the cast writes `hit` into the collector (`None` = no hit).
    fn install(
        e: &mut Engine,
        sphere: u32,
        hit: Option<(Vec4, Vec4, u32, u32)>,
    ) -> Rc<RefCell<Seen>> {
        let seen = Rc::new(RefCell::new(Seen::default()));
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, 0x0900_0004);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(0x0900_0004, move |_, a| {
            assert_eq!(a[1], 0x20);
            Ret {
                eax: sphere,
                ..Ret::default()
            }
        });
        let sink = Rc::clone(&seen);
        e.register_double(SPHERE_CONSTRUCTOR, move |_, a| {
            assert_eq!(a[0], sphere);
            sink.borrow_mut().sphere_radius = a[1];
            Ret::default()
        });
        let sink = Rc::clone(&seen);
        e.register_double(COLLIDABLE_CONSTRUCTOR, move |e, a| {
            assert_eq!(a[1], sphere);
            assert_eq!(a[3], 0);
            sink.borrow_mut().transform = (0..4).map(|v| load_vec(e, a[2] + 0x10 * v)).collect();
            Ret::default()
        });
        let sink = Rc::clone(&seen);
        e.register_double(LINEAR_CAST, move |e, a| {
            let mut seen = sink.borrow_mut();
            seen.input = load_vec(e, a[2]);
            seen.tolerances = (e.mem.u32(a[2] + 0x10), e.mem.u32(a[2] + 0x14));
            seen.collector_words = (e.mem.u32(a[3]), e.mem.u32(a[3] + 4));
            assert_eq!(e.mem.u32(a[3] + 0x30), 0);
            assert_eq!(a[4], 0);
            if let Some((point, normal, body, key)) = hit {
                store_vec(e, a[3] + 0x10, point);
                store_vec(e, a[3] + 0x20, normal);
                e.mem.set_u32(a[3] + 0x30, 1);
                e.mem.set_u32(a[3] + 0x58, body);
                e.mem.set_u32(a[3] + 0x5c, key);
            }
            Ret::default()
        });
        let sink = Rc::clone(&seen);
        e.register_double(REMOVE_REFERENCE, move |_, a| {
            sink.borrow_mut().released.push(a[0]);
            Ret::default()
        });
        e.set_global(EPSILON, f32::from_bits(0x3400_0000));
        e.set_global(LARGE_DISTANCE, f32::from_bits(0x7f7f_ffee));
        seen
    }

    fn sweep(e: &mut Engine, out: u32) -> u32 {
        let from = e.mem.alloc(0x10);
        store_vec(e, from, [1.0, 2.0, 3.0, 1.0]);
        let to = e.mem.alloc(0x10);
        store_vec(e, to, [4.0, 5.0, 6.0, 1.0]);
        e.call(0x00e4_64e0, &args![0x5000u32, from, 0.25f32, to, out])
            .u32()
    }

    fn check_cast(seen: &Seen) {
        assert_eq!(seen.sphere_radius, 0.25f32.to_bits());
        assert_eq!(
            seen.transform,
            vec![
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [1.0, 2.0, 3.0, 1.0]
            ]
        );
        assert_eq!(seen.input, [4.0, 5.0, 6.0, 1.0]);
        assert_eq!(seen.tolerances, (0x3400_0000, 0x3400_0000));
        assert_eq!(seen.collector_words, (0x010c_476c, 0x7f7f_ffee));
    }

    #[test]
    fn sweep_reports_the_hit_with_the_owner_of_a_body() {
        let mut e = engine();
        let body = e.mem.alloc(0x40) + 0x20;
        e.mem.set_u8(body + 0x18, 1);
        e.mem.set_u8(body + 0x10, (-0x20i8) as u8);
        let sphere = e.mem.alloc(0x20);
        let seen = install(
            &mut e,
            sphere,
            Some(([1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0], body, 0x77)),
        );
        let out = e.mem.alloc(0x30);
        assert_eq!(sweep(&mut e, out), 0);
        check_cast(&seen.borrow());
        assert_eq!(seen.borrow().released, vec![sphere]);
        assert_eq!(load_vec(&e, out), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vec(&e, out + 0x10), [5.0, 6.0, 7.0, 8.0]);
        assert_eq!(e.mem.u32(out + 0x20), 0x77);
        assert_eq!(e.mem.u32(out + 0x24), body - 0x20);
    }

    #[test]
    fn sweep_hit_on_another_kind_of_body_has_no_owner() {
        let mut e = engine();
        let body = e.mem.alloc(0x40);
        e.mem.set_u8(body + 0x18, 2);
        let sphere = e.mem.alloc(0x20);
        install(&mut e, sphere, Some(([1.0; 4], [2.0; 4], body, 0x99)));
        let out = e.mem.alloc(0x30);
        e.mem.set_u32(out + 0x24, 0x1234);
        assert_eq!(sweep(&mut e, out), 0);
        assert_eq!(e.mem.u32(out + 0x20), 0x99);
        assert_eq!(e.mem.u32(out + 0x24), 0);
    }

    #[test]
    fn sweep_without_a_hit_ends_at_the_target() {
        let mut e = engine();
        let sphere = e.mem.alloc(0x20);
        let seen = install(&mut e, sphere, None);
        store_vec(&mut e, NO_HIT_NORMAL, [9.0, 8.0, 7.0, 6.0]);
        let out = e.mem.alloc(0x30);
        store_vec(&mut e, out + 0x10, [1.0, 1.0, 1.0, 5.5]);
        e.mem.set_u32(out + 0x24, 0x1234);
        assert_eq!(sweep(&mut e, out), 1);
        check_cast(&seen.borrow());
        assert_eq!(seen.borrow().released, vec![sphere]);
        assert_eq!(load_vec(&e, out), [4.0, 5.0, 6.0, 1.0]);
        assert_eq!(load_vec(&e, out + 0x10), [9.0, 8.0, 7.0, 5.5]);
        assert_eq!(e.mem.u32(out + 0x20), 0xffff_ffff);
        assert_eq!(e.mem.u32(out + 0x24), 0);
    }

    #[test]
    fn constructor_builds_the_default_name() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, STRING_FROM_TEXT, 0);
        let this = e.mem.alloc(0x30);
        assert_eq!(e.call(0x00e4_6660, &args![this]).u32(), this);
        assert_eq!(
            calls(&log, STRING_FROM_TEXT),
            vec![vec![this + 0xc, 0x0101_1584]]
        );
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), 0x010e_c0e4);
        assert_eq!(e.mem.u8(this + 8), 0);
        assert_eq!(e.mem.u8(this + 0x10), 0x71);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0x8000_0000);
    }

    #[test]
    fn flagged_constructor_clears_the_byte_only_with_a_flag() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, STRING_FROM_FLAG, 0);
        let this = e.mem.alloc(0x30);
        e.mem.set_u8(this + 8, 5);
        assert_eq!(e.call(0x00e4_66a0, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u8(this + 8), 5);
        e.call(0x00e4_66a0, &args![this, 1u32]);
        assert_eq!(e.mem.u8(this + 8), 0);
        assert_eq!(
            calls(&log, STRING_FROM_FLAG),
            vec![vec![this + 0xc, 0], vec![this + 0xc, 1]]
        );
        assert_eq!(e.mem.u32(this), 0x010e_c0e4);
        assert_eq!(e.mem.u32(this + 0x1c), 0x8000_0000);
    }

    #[test]
    fn destructor_releases_the_listeners_and_the_name() {
        let mut e = engine();
        let log = new_log();
        let allocator = logging_object(&mut e, &log, 0x0900_0000);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        log_double(&mut e, &log, REMOVE_REFERENCE, 0);
        log_double(&mut e, &log, STRING_DESTRUCTOR, 0);
        let this = e.mem.alloc(0x30);
        let listeners = e.mem.alloc(8);
        e.mem.set_u32(listeners, 0xa1);
        e.mem.set_u32(listeners + 4, 0xa2);
        e.mem.set_u32(this + 0x14, listeners);
        e.mem.set_u32(this + 0x18, 2);
        e.mem.set_u32(this + 0x1c, 3);
        e.call(0x00e4_66e0, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![
                (REMOVE_REFERENCE, vec![0xa1]),
                (REMOVE_REFERENCE, vec![0xa2]),
                (0x0900_0010, vec![allocator, listeners, 12]),
                (STRING_DESTRUCTOR, vec![this + 0xc]),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0x8000_0000);
        assert_eq!(e.mem.u32(this), 0x0102_e388);
    }
}
