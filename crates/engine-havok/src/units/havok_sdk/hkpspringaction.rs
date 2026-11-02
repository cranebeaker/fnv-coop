//! `Havok SDK/hkpspringaction.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpboxmotion::{mul, sub};
use crate::units::havok_sdk::hkpsphereshape::{copy_vec, load_vec, store_vec, sum3, Vec4};

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d4de00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00cf_4de0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d4de00(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00cf_4de0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00cf4ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBinaryAction::getClassType` (Xbox PDB): takes no arguments and returns the constant 0x0126_9168.
pub fn hkp_binary_action_get_class_type(_e: &mut Engine) -> u32 {
    0x0126_9168
}

/// `hkpBinaryAction::hkpBinaryAction` (`00cf4d80`, `this`, `(entity a, entity b, user data)`).
const BINARY_ACTION_CONSTRUCT: u32 = 0x00cf_4d80;
/// `hkVector4::setTransformedPos` (`00c7f640`) and `setTransformedInversePos` (`00c7f690`):
/// `this` = the result, `(transform, point)`.
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
const SET_TRANSFORMED_INVERSE_POS: u32 = 0x00c7_f690;
/// The velocity of a body at a point (`00ca0c40`, `this` = the body, `(point, out vector)`).
const POINT_VELOCITY: u32 = 0x00ca_0c40;
/// `hkpEntity::activate` (`00c9c1d0`, `this` = the entity).
const ENTITY_ACTIVATE: u32 = 0x00c9_c1d0;
/// The vtable the constructor installs, the `float`s it takes the strength and the damping
/// from, and the length below which `applyAction` does nothing.
const SPRING_ACTION_VTABLE: u32 = 0x010d_08f4;
const DEFAULT_STRENGTH: u32 = 0x0101_3974;
const DEFAULT_DAMPING: u32 = 0x0101_e2bc;
const MINIMUM_LENGTH: u32 = 0x0101_7d00;
/// Bytes of one action.
const ACTION_SIZE: u32 = 0x60;

// Translated from 00d4d340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSpringAction::hkpSpringAction` (Xbox PDB), `(this; entity a, entity b, user data)`: runs
/// the `hkpBinaryAction` constructor (`00cf4d80`), then sets the rest length (`+0x50`) to 1.0,
/// installs the vtable (`010d08f4`), sets the strength (`+0x54`) and the damping (`+0x58`) from
/// the `float`s at `01013974` and `0101e2bc` (copied through the x87, so a signalling NaN comes
/// out quiet) and sets the two bytes `+0x5c` and `+0x5d` (`m_onCompression`, `m_onExtension`)
/// to 1. Returns `this`.
pub fn hkp_spring_action_construct(
    e: &mut Engine,
    this: Ptr,
    entity_a: u32,
    entity_b: u32,
    user_data: u32,
) -> Ptr {
    let base = this.addr();
    e.call(
        BINARY_ACTION_CONSTRUCT,
        &args![this, entity_a, entity_b, user_data],
    );
    e.mem.set_f32(base + 0x50, 1.0);
    e.mem.set_u32(base, SPRING_ACTION_VTABLE);
    let strength = x87_quiet_bits(e.mem.u32(DEFAULT_STRENGTH));
    e.mem.set_u32(base + 0x54, strength);
    let damping = x87_quiet_bits(e.mem.u32(DEFAULT_DAMPING));
    e.mem.set_u32(base + 0x58, damping);
    e.mem.set_u8(base + 0x5c, 1);
    e.mem.set_u8(base + 0x5d, 1);
    this
}

/// The length of `delta`: the square root (`SQRTSS`, exact) of the squared lanes summed as
/// `z + (y + x)`.
fn length_of(delta: Vec4) -> f32 {
    sum3(mul(delta, delta)).sqrt()
}

// Translated from 00d4d390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the pivots from world space points (the map gives the function no name; the pair of
/// this and `00d4d410` suggests `setPositionsInWorldSpace`, unconfirmed), `(this; pivot a,
/// pivot b)`: each point goes into the space of its entity by `hkVector4::
/// setTransformedInversePos` (`00c7f690`) with the entity's transform (`entity + 0xf0`) into
/// the action's pivot (`+0x30` for entity a, `+0x40` for entity b); the rest length (`+0x50`)
/// becomes the distance of the two world points (`SQRTSS` of the squares summed as
/// `z + (y + x)`, copied through the x87).
pub fn fn_00d4d390(e: &mut Engine, this: Ptr, pivot_a: Ptr, pivot_b: Ptr) {
    let base = this.addr();
    let entity_a = e.mem.u32(base + 0x18);
    let entity_b = e.mem.u32(base + 0x1c);
    e.call(
        SET_TRANSFORMED_INVERSE_POS,
        &args![base + 0x30, entity_a + 0xf0, pivot_a],
    );
    e.call(
        SET_TRANSFORMED_INVERSE_POS,
        &args![base + 0x40, entity_b + 0xf0, pivot_b],
    );
    let delta = sub(load_vec(e, pivot_a.addr()), load_vec(e, pivot_b.addr()));
    e.mem.set_f32(base + 0x50, x87_quiet(length_of(delta)));
}

// Translated from 00d4d410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSpringAction::setPositionsInBodySpace` (Xbox PDB), `(this; pivot a, pivot b)`: copies the
/// two pivots (16 bytes each) to `+0x30` and `+0x40`; the rest length (`+0x50`) becomes the
/// distance of the two pivots taken to world space by `hkVector4::setTransformedPos`
/// (`00c7f640`) with the entities' transforms (`entity + 0xf0`), `SQRTSS` of the squares summed
/// as `z + (y + x)`, copied through the x87.
pub fn hkp_spring_action_set_positions_in_body_space(
    e: &mut Engine,
    this: Ptr,
    pivot_a: Ptr,
    pivot_b: Ptr,
) {
    let base = this.addr();
    let entity_b = e.mem.u32(base + 0x1c);
    let entity_a = e.mem.u32(base + 0x18);
    copy_vec(e, pivot_a.addr(), base + 0x30);
    copy_vec(e, pivot_b.addr(), base + 0x40);
    let length = e.with_stack(0x20, |e, frame| {
        let frame = frame.addr();
        e.call(SET_TRANSFORMED_POS, &args![frame, entity_a + 0xf0, pivot_a]);
        e.call(
            SET_TRANSFORMED_POS,
            &args![frame + 0x10, entity_b + 0xf0, pivot_b],
        );
        length_of(sub(load_vec(e, frame), load_vec(e, frame + 0x10)))
    });
    e.mem.set_f32(base + 0x50, x87_quiet(length));
}

// Translated from 00d4d4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSpringAction::clone` (Xbox PDB), `(this; entities, phantoms)` (two `hkArray`s: data
/// pointer at `+0`, size at `+4`): null unless there are two entities and no phantoms. Takes
/// `0x60` bytes from the memory router's allocator (slot 4), stores the size in the 16-bit word
/// at `+4`, constructs the copy (`00d4d340`) from the two entities and this action's user data
/// (`+0x10`), then copies the two pivots (16 bytes each at `+0x30` and `+0x40`), the rest
/// length, strength and damping (`+0x50`, `+0x54`, `+0x58`, each through the x87) and the two
/// bytes `+0x5c` and `+0x5d`. Returns the copy.
pub fn hkp_spring_action_clone(e: &mut Engine, this: Ptr, entities: u32, phantoms: u32) -> Ptr {
    if e.mem.u32(entities + 4) != 2 || e.mem.u32(phantoms + 4) != 0 {
        return Ptr::new(0);
    }
    let base = this.addr();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![ACTION_SIZE]).u32();
    e.mem.set_u16(block + 4, ACTION_SIZE as u16);
    let array = e.mem.u32(entities);
    let (entity_a, entity_b) = (e.mem.u32(array), e.mem.u32(array + 4));
    let user_data = e.mem.u32(base + 0x10);
    let copy = hkp_spring_action_construct(e, Ptr::new(block), entity_a, entity_b, user_data);
    copy_vec(e, base + 0x30, block + 0x30);
    copy_vec(e, base + 0x40, block + 0x40);
    for offset in [0x50, 0x54, 0x58] {
        let value = x87_quiet_bits(e.mem.u32(base + offset));
        e.mem.set_u32(block + offset, value);
    }
    let on_compression = e.mem.u8(base + 0x5c);
    e.mem.set_u8(block + 0x5c, on_compression);
    let on_extension = e.mem.u8(base + 0x5d);
    e.mem.set_u8(block + 0x5d, on_extension);
    copy
}

// Translated from 00d4d530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSpringAction::applyAction` (Xbox PDB), `(this; step info)`. Takes the two pivots (`+0x30`
/// and `+0x40`) to world space (`00c7f640`, with the entities' transforms at `entity + 0xf0`)
/// and the length of their difference `delta` (b less a; `SQRTSS` of the squares summed as
/// `z + (y + x)`). Nothing happens when the length is below the `float` at `01017d00` (an
/// x87 compare: a NaN length goes on), when `m_onCompression` (`+0x5c`) is 0 and the rest
/// length (`+0x50`) is above the length, or when `m_onExtension` (`+0x5d`) is 0 and the rest
/// length is below it. Otherwise the direction is `delta` times the `float` `1 / length`, the
/// point velocities of the entities at the world points come from `00ca0c40`, and the force
/// magnitude is `(length - rest length) * strength + damping * dot` (the x87 does the sums and
/// products in extended precision; `f64` here) where `dot` is `z + (y + x)` of `(velocity b -
/// velocity a) * direction` (a `float`). The magnitude is rounded to a `float`; the force
/// `-magnitude * direction` is stored at `this + 0x20` and applied to entity b (activated
/// first through `00c9c1d0`) by its motion (`entity + 0xe0`, slot `0x58`, `(delta time, force,
/// point)`, the delta time the `float` at `step info + 8`) at its world point; then the force
/// `magnitude * direction` is stored there and applied to entity a the same way.
pub fn hkp_spring_action_apply_action(e: &mut Engine, this: Ptr, step_info: Ptr) {
    let base = this.addr();
    let entity_b = e.mem.u32(base + 0x1c);
    let entity_a = e.mem.u32(base + 0x18);
    e.with_stack(0x40, |e, frame| {
        let (world_a, world_b) = (frame.addr(), frame.addr() + 0x10);
        let (velocity_a, velocity_b) = (frame.addr() + 0x20, frame.addr() + 0x30);
        e.call(
            SET_TRANSFORMED_POS,
            &args![world_a, entity_a + 0xf0, base + 0x30],
        );
        e.call(
            SET_TRANSFORMED_POS,
            &args![world_b, entity_b + 0xf0, base + 0x40],
        );
        let delta = sub(load_vec(e, world_b), load_vec(e, world_a));
        let length = x87_quiet(length_of(delta));
        if length < x87_quiet(e.mem.f32(MINIMUM_LENGTH)) {
            return;
        }
        let rest = x87_quiet(e.mem.f32(base + 0x50));
        let inverse = (1.0 / f64::from(length)) as f32;
        let direction = mul([inverse; 4], delta);
        if e.mem.u8(base + 0x5c) == 0 && rest > length {
            return;
        }
        if e.mem.u8(base + 0x5d) == 0 && rest < length {
            return;
        }
        e.call(POINT_VELOCITY, &args![entity_a, world_a, velocity_a]);
        e.call(POINT_VELOCITY, &args![entity_b, world_b, velocity_b]);
        let relative = sub(load_vec(e, velocity_b), load_vec(e, velocity_a));
        let along = sum3(mul(relative, direction));
        let strength = x87_quiet(e.mem.f32(base + 0x54));
        let damping = x87_quiet(e.mem.f32(base + 0x58));
        let magnitude = ((f64::from(length) - f64::from(rest)) * f64::from(strength)
            + f64::from(damping) * f64::from(along)) as f32;
        let force = base + 0x20;
        for (entity, point, scale) in [
            (entity_b, world_b, -magnitude),
            (entity_a, world_a, magnitude),
        ] {
            store_vec(e, force, mul([scale; 4], direction));
            let step = x87_quiet(e.mem.f32(step_info.addr() + 8));
            e.call(ENTITY_ACTIVATE, &args![entity]);
            e.vcall(entity + 0xe0, 0x58, &args![step, force, point]);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cf4ef0, hkp_binary_action_get_class_type() -> u32),
        entry!(0x00d4_de00, deleting_destructor_00d4de00(Ptr, u8) -> Ptr),
        entry!(0x00d4_d340, hkp_spring_action_construct(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00d4_d390, fn_00d4d390(Ptr, Ptr, Ptr)),
        entry!(
            0x00d4_d410,
            hkp_spring_action_set_positions_in_body_space(Ptr, Ptr, Ptr)
        ),
        entry!(0x00d4_d4a0, hkp_spring_action_clone(Ptr, u32, u32) -> Ptr),
        entry!(0x00d4_d530, hkp_spring_action_apply_action(Ptr, Ptr)),
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
    fn deleting_destructor_00d4de00_frees_only_when_bit_zero_is_set() {
        check(0x00d4_de00, 0x00cf_4de0);
    }

    #[test]
    fn hkp_binary_action_get_class_type_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cf_4ef0, &args![]).u32(), 0x0126_9168);
    }
}

#[cfg(test)]
mod spring_tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, new_log, Log,
    };
    use std::cell::RefCell;
    use std::rc::Rc;

    const ALLOCATE: u32 = 0x0900_0004;
    const APPLY_FORCE: u32 = 0x0900_0058;
    const SIGNALLING_NAN: u32 = 0x7fa0_0001;
    const QUIET_NAN: u32 = 0x7fe0_0001;

    /// A spring action with two entities, whose motions' slot `0x58` is a double.
    struct Spring {
        e: Engine,
        log: Log,
        this: u32,
        entity_a: u32,
        entity_b: u32,
        step: u32,
        applied: Applied,
    }

    /// What the motions' `applyForce` double saw: the argument words, and the force and the
    /// point at the time of the call.
    type Applied = Rc<RefCell<Vec<(Vec<u32>, Vec4, Vec4)>>>;

    fn spring() -> Spring {
        let mut e = engine();
        let log = new_log();
        let vtable = e.mem.alloc(0x80);
        e.mem.set_u32(vtable + 0x58, APPLY_FORCE);
        let entity = |e: &mut Engine| {
            let entity = e.mem.alloc(0x200);
            e.mem.set_u32(entity + 0xe0, vtable);
            entity
        };
        let entity_a = entity(&mut e);
        let entity_b = entity(&mut e);
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x18, entity_a);
        e.mem.set_u32(this + 0x1c, entity_b);
        let step = e.mem.alloc(0x10);
        e.mem.set_f32(step + 8, 0.5);
        e.mem.set_f32(MINIMUM_LENGTH, 0.001);
        log_double(&mut e, &log, ENTITY_ACTIVATE, 0);
        let applied: Applied = Rc::new(RefCell::new(Vec::new()));
        let copy = Rc::clone(&applied);
        e.register_double(APPLY_FORCE, move |e, a| {
            copy.borrow_mut()
                .push((a.to_vec(), load_vec(e, a[2]), load_vec(e, a[3])));
            Ret::default()
        });
        Spring {
            e,
            log,
            this,
            entity_a,
            entity_b,
            step,
            applied,
        }
    }

    /// Makes `00c7f640` (and `00c7f690`) write `world_a` for the transform of entity a and
    /// `world_b` for that of entity b, and the velocity function write the velocities.
    fn world(s: &mut Spring, address: u32, points: [Vec4; 2]) {
        let log = Rc::clone(&s.log);
        let (transform_a, transform_b) = (s.entity_a + 0xf0, s.entity_b + 0xf0);
        s.e.register_double(address, move |e, a| {
            log.borrow_mut().push((address, a.to_vec()));
            let point = if a[1] == transform_a {
                points[0]
            } else {
                assert_eq!(a[1], transform_b);
                points[1]
            };
            store_vec(e, a[0], point);
            Ret::default()
        });
    }

    fn velocities(s: &mut Spring, velocities: [Vec4; 2]) {
        let log = Rc::clone(&s.log);
        let entity_a = s.entity_a;
        s.e.register_double(POINT_VELOCITY, move |e, a| {
            log.borrow_mut().push((POINT_VELOCITY, a.to_vec()));
            let velocity = if a[0] == entity_a {
                velocities[0]
            } else {
                velocities[1]
            };
            store_vec(e, a[2], velocity);
            Ret::default()
        });
    }

    #[test]
    fn construct_sets_the_defaults_00d4d340() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, BINARY_ACTION_CONSTRUCT, 0);
        e.mem.set_u32(DEFAULT_STRENGTH, SIGNALLING_NAN);
        e.set_global(DEFAULT_DAMPING, 2.0f32);
        let this = e.mem.alloc(0x80);
        let got = e
            .call(0x00d4_d340, &args![this, 0x11u32, 0x22u32, 0x33u32])
            .u32();
        assert_eq!(got, this);
        assert_eq!(
            calls(&log, BINARY_ACTION_CONSTRUCT),
            vec![vec![this, 0x11, 0x22, 0x33]]
        );
        assert_eq!(e.mem.u32(this), SPRING_ACTION_VTABLE);
        assert_eq!(e.mem.f32(this + 0x50), 1.0);
        assert_eq!(e.mem.u32(this + 0x54), QUIET_NAN);
        assert_eq!(e.mem.f32(this + 0x58), 2.0);
        assert_eq!((e.mem.u8(this + 0x5c), e.mem.u8(this + 0x5d)), (1, 1));
    }

    #[test]
    fn world_space_pivots_go_into_each_entitys_space_00d4d390() {
        let mut s = spring();
        let pivot_a = s.e.mem.alloc(0x10);
        let pivot_b = s.e.mem.alloc(0x10);
        store_vec(&mut s.e, pivot_a, [1.0, 2.0, 3.0, 0.0]);
        store_vec(&mut s.e, pivot_b, [4.0, 6.0, 3.0, 0.0]);
        world(&mut s, SET_TRANSFORMED_INVERSE_POS, [[0.5; 4], [0.25; 4]]);
        s.e.call(0x00d4_d390, &args![s.this, pivot_a, pivot_b]);
        assert_eq!(
            calls(&s.log, SET_TRANSFORMED_INVERSE_POS),
            vec![
                vec![s.this + 0x30, s.entity_a + 0xf0, pivot_a],
                vec![s.this + 0x40, s.entity_b + 0xf0, pivot_b]
            ]
        );
        assert_eq!(load_vec(&s.e, s.this + 0x30), [0.5; 4]);
        assert_eq!(load_vec(&s.e, s.this + 0x40), [0.25; 4]);
        // The rest length is the distance of the world points.
        assert_eq!(s.e.mem.f32(s.this + 0x50), 5.0);
    }

    #[test]
    fn body_space_pivots_are_kept_and_the_rest_length_is_made_00d4d410() {
        let mut s = spring();
        let pivot_a = s.e.mem.alloc(0x10);
        let pivot_b = s.e.mem.alloc(0x10);
        store_vec(&mut s.e, pivot_a, [0.5, 0.25, 7.0, 1.0]);
        store_vec(&mut s.e, pivot_b, [8.0, 9.0, 10.0, 2.0]);
        world(
            &mut s,
            SET_TRANSFORMED_POS,
            [[1.0, 2.0, 3.0, 1.0], [4.0, 6.0, 3.0, 1.0]],
        );
        s.e.call(0x00d4_d410, &args![s.this, pivot_a, pivot_b]);
        assert_eq!(load_vec(&s.e, s.this + 0x30), [0.5, 0.25, 7.0, 1.0]);
        assert_eq!(load_vec(&s.e, s.this + 0x40), [8.0, 9.0, 10.0, 2.0]);
        let positions = calls(&s.log, SET_TRANSFORMED_POS);
        assert_eq!(positions.len(), 2);
        assert_eq!(positions[0][1..], [s.entity_a + 0xf0, pivot_a]);
        assert_eq!(positions[1][1..], [s.entity_b + 0xf0, pivot_b]);
        assert_eq!(positions[1][0], positions[0][0] + 0x10);
        assert_eq!(s.e.mem.f32(s.this + 0x50), 5.0);
    }

    /// An allocator whose slot 4 hands out `block`; the router answers it.
    fn router(e: &mut Engine, log: &Log, block: u32) {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let memory = e.mem.alloc(0x20);
        e.mem.set_u32(memory + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: memory,
            ..Ret::default()
        });
        log_double(e, log, ALLOCATE, block);
    }

    /// A pair of `hkArray`s: two entities, and a number of phantoms.
    fn arrays(s: &mut Spring, entities: u32, phantoms: u32) -> (u32, u32) {
        let data = s.e.mem.alloc(8);
        s.e.mem.set_u32(data, s.entity_a);
        s.e.mem.set_u32(data + 4, s.entity_b);
        let first = s.e.mem.alloc(8);
        s.e.mem.set_u32(first, data);
        s.e.mem.set_u32(first + 4, entities);
        let second = s.e.mem.alloc(8);
        s.e.mem.set_u32(second + 4, phantoms);
        (first, second)
    }

    #[test]
    fn clone_copies_the_action_00d4d4a0() {
        let mut s = spring();
        let block = s.e.mem.alloc(0x80);
        router(&mut s.e, &s.log, block);
        log_double(&mut s.e, &s.log, BINARY_ACTION_CONSTRUCT, 0);
        s.e.mem.set_u32(DEFAULT_STRENGTH, 0x4040_0000);
        s.e.set_global(DEFAULT_DAMPING, 5.0f32);
        let this = s.this;
        s.e.mem.set_u32(this + 0x10, 0x77);
        store_vec(&mut s.e, this + 0x30, [1.0, 2.0, 3.0, 4.0]);
        store_vec(&mut s.e, this + 0x40, [5.0, 6.0, 7.0, 8.0]);
        s.e.mem.set_f32(this + 0x50, 3.0);
        s.e.mem.set_u32(this + 0x54, SIGNALLING_NAN);
        s.e.mem.set_f32(this + 0x58, 4.0);
        s.e.mem.set_u8(this + 0x5c, 0);
        s.e.mem.set_u8(this + 0x5d, 1);
        let (entities, phantoms) = arrays(&mut s, 2, 0);
        let copy =
            s.e.call(0x00d4_d4a0, &args![this, entities, phantoms])
                .u32();
        assert_eq!(copy, block);
        assert_eq!(calls(&s.log, ALLOCATE).len(), 1);
        assert_eq!(calls(&s.log, ALLOCATE)[0][1], 0x60);
        assert_eq!(s.e.mem.u16(block + 4), 0x60);
        assert_eq!(
            calls(&s.log, BINARY_ACTION_CONSTRUCT),
            vec![vec![block, s.entity_a, s.entity_b, 0x77]]
        );
        assert_eq!(s.e.mem.u32(block), SPRING_ACTION_VTABLE);
        assert_eq!(load_vec(&s.e, block + 0x30), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vec(&s.e, block + 0x40), [5.0, 6.0, 7.0, 8.0]);
        assert_eq!(s.e.mem.f32(block + 0x50), 3.0);
        assert_eq!(s.e.mem.u32(block + 0x54), QUIET_NAN);
        assert_eq!(s.e.mem.f32(block + 0x58), 4.0);
        assert_eq!((s.e.mem.u8(block + 0x5c), s.e.mem.u8(block + 0x5d)), (0, 1));
    }

    #[test]
    fn clone_needs_two_entities_and_no_phantoms_00d4d4a0() {
        for (entities, phantoms) in [(3, 0), (1, 0), (2, 1)] {
            let mut s = spring();
            router(&mut s.e, &s.log, 0x1234);
            let (first, second) = arrays(&mut s, entities, phantoms);
            let this = s.this;
            assert_eq!(s.e.call(0x00d4_d4a0, &args![this, first, second]).u32(), 0);
            assert!(calls(&s.log, ALLOCATE).is_empty());
        }
    }

    /// A spring of strength 10 and damping 2 between world points `(0, 0, 0)` and `world_b`,
    /// with the rest length `rest` and the two flags; the velocities are `(0, 0, 0.5)` and
    /// `(0, 0, 1)`. Applies it.
    fn apply(world_b: Vec4, rest: f32, flags: (u8, u8)) -> Spring {
        let mut s = spring();
        let this = s.this;
        s.e.mem.set_f32(this + 0x50, rest);
        s.e.mem.set_f32(this + 0x54, 10.0);
        s.e.mem.set_f32(this + 0x58, 2.0);
        s.e.mem.set_u8(this + 0x5c, flags.0);
        s.e.mem.set_u8(this + 0x5d, flags.1);
        world(&mut s, SET_TRANSFORMED_POS, [[0.0, 0.0, 0.0, 0.0], world_b]);
        velocities(&mut s, [[0.0, 0.0, 0.5, 0.0], [0.0, 0.0, 1.0, 0.0]]);
        let step = s.step;
        s.e.call(0x00d4_d530, &args![this, step]);
        s
    }

    const FAR: Vec4 = [0.0, 0.0, 2.0, 0.0];

    #[test]
    fn a_stretched_spring_pulls_the_entities_together_00d4d530() {
        let s = apply(FAR, 1.0, (1, 1));
        // Length 2, direction (0, 0, 1); magnitude (2 - 1) * 10 + 2 * 0.5 = 11.
        let positions = calls(&s.log, SET_TRANSFORMED_POS);
        assert_eq!(positions[0][1..], [s.entity_a + 0xf0, s.this + 0x30]);
        assert_eq!(positions[1][1..], [s.entity_b + 0xf0, s.this + 0x40]);
        let velocities = calls(&s.log, POINT_VELOCITY);
        assert_eq!(velocities.len(), 2);
        assert_eq!(velocities[0][0], s.entity_a);
        assert_eq!(velocities[1][0], s.entity_b);
        // Entity b first: the force -11 * direction at its world point, then entity a.
        assert_eq!(
            calls(&s.log, ENTITY_ACTIVATE),
            vec![vec![s.entity_b], vec![s.entity_a]]
        );
        let applied = s.applied.borrow();
        assert_eq!(applied.len(), 2);
        let motion = |entity: u32| entity + 0xe0;
        let step = 0.5f32.to_bits();
        assert_eq!(
            applied[0].0,
            vec![motion(s.entity_b), step, s.this + 0x20, velocities[1][1]]
        );
        assert_eq!(applied[0].1, [0.0, 0.0, -11.0, 0.0]);
        assert_eq!(applied[0].2, FAR);
        assert_eq!(
            applied[1].0,
            vec![motion(s.entity_a), step, s.this + 0x20, velocities[0][1]]
        );
        assert_eq!(applied[1].1, [0.0, 0.0, 11.0, 0.0]);
        assert_eq!(applied[1].2, [0.0; 4]);
    }

    #[test]
    fn a_length_below_the_minimum_does_nothing_00d4d530() {
        let s = apply([0.0, 0.0, 0.0005, 0.0], 1.0, (1, 1));
        assert_eq!(calls(&s.log, SET_TRANSFORMED_POS).len(), 2);
        assert!(calls(&s.log, POINT_VELOCITY).is_empty());
        assert!(calls(&s.log, ENTITY_ACTIVATE).is_empty());
        assert!(s.applied.borrow().is_empty());
    }

    #[test]
    fn compression_is_only_resisted_when_asked_for_00d4d530() {
        // Rest length 3 above the length 2: a compressed spring.
        assert!(apply(FAR, 3.0, (0, 1)).applied.borrow().is_empty());
        assert_eq!(apply(FAR, 3.0, (1, 0)).applied.borrow().len(), 2);
        // The rest length equal to the length is neither.
        assert_eq!(apply(FAR, 2.0, (0, 0)).applied.borrow().len(), 2);
    }

    #[test]
    fn extension_is_only_resisted_when_asked_for_00d4d530() {
        // Rest length 1 below the length 2: an extended spring.
        assert!(apply(FAR, 1.0, (1, 0)).applied.borrow().is_empty());
        assert_eq!(apply(FAR, 1.0, (0, 1)).applied.borrow().len(), 2);
    }

    #[test]
    fn a_nan_length_does_not_stop_the_spring_00d4d530() {
        let s = apply([0.0, 0.0, f32::NAN, 0.0], 1.0, (0, 0));
        assert_eq!(calls(&s.log, POINT_VELOCITY).len(), 2);
        assert_eq!(s.applied.borrow().len(), 2);
    }
}
