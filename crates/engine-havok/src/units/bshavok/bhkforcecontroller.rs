//! `BSHavok/bhkforcecontroller.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkForceController` is a 0x70-byte, 16-byte aligned timed controller: `+8` holds its
//! flag word, `+0xC..=+0x20` its float timing fields, `+0x2c` the controlled target, `+0x40` a
//! four-float force vector and `+0x50` the time left.
//!
//! The exception-unwinding frames (`FS:[0]` registrations) and `/GS` cookie checks of the
//! functions marked `Translation incomplete` are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhkentity::{aligned_allocate, enter_scope, MEMORY_MANAGER};

/// Memory manager `Free` (`00aa4060`, thiscall on `011f6238`, one pointer argument).
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
/// Base class destructor body (`00a6d290`) and constructor (`00a6d220`), and the base
/// `CopyMembers` (`00a6cfe0`, `(this, clone, context)`).
const BASE_DESTRUCT: u32 = 0x00a6_d290;
const BASE_CONSTRUCT: u32 = 0x00a6_d220;
const BASE_COPY_MEMBERS: u32 = 0x00a6_cfe0;
/// Dynamic cast (`00653290`, one `NiRTTI` argument) to the `NiRTTI` globals at `01267e64`
/// (`ApplyForce`) and `012043f8` (the force-adding routines).
const IS_KIND_OF: u32 = 0x0065_3290;
const APPLY_TARGET_RTTI: u32 = 0x0126_7e64;
const FORCE_TARGET_RTTI: u32 = 0x0120_43f8;
/// The `NiRTTI` global of the controller class, `NiObjectNET::GetController`-style lookup
/// (`00a5c570`, thiscall on the node with that `NiRTTI`) and the attach (`00a5c480`,
/// thiscall on the node with the controller).
const CONTROLLER_RTTI: u32 = 0x0126_83f4;
const FIND_CONTROLLER: u32 = 0x00a5_c570;
const REMOVE_CONTROLLER: u32 = 0x00a5_c480;
/// What `ApplyForce` hands the target (`00631100`, thiscall on the target with
/// `(time, address of a force vector)`).
const APPLY_TO_TARGET: u32 = 0x0063_1100;
/// The target's magnitude getter used by the force-adding routines (`0062ab20`, answers a
/// float in `ST0`).
const GET_MAGNITUDE: u32 = 0x0062_ab20;
/// This unit's own `ApplyForce`.
const APPLY_FORCE: u32 = 0x00cb_9290;
/// `RemoveForce` itself, for the recursion.
const REMOVE_FORCE: u32 = 0x00cb_9650;
/// Vtable of the controller, the default force vector and the float/double constants:
/// the limit scale of `ApplyForce`, the starting time of `Update`, its reset value, the two
/// factors of the force-adding routines, and the values they store.
const CONTROLLER_VTABLE: u32 = 0x010c_8a2c;
const DEFAULT_VECTOR: u32 = 0x0126_7e30;
const FORCE_SCALE: u32 = 0x0101_6410;
const INITIAL_TIME: u32 = 0x010c_8ae4;
const THRESHOLD: u32 = 0x0102_41b0;
const MAGNITUDE_FACTOR: u32 = 0x011b_1d3c;
const SPEED_FACTOR: u32 = 0x0101_de30;
const STORED_LIMIT: u32 = 0x0101_df44;
const STORED_STEP: u32 = 0x0101_5f5c;

layout! {
    /// `bhkForceController` (Xbox PDB name of the class), 0x70 bytes; the fields this unit
    /// uses.
    pub struct BhkForceController: 0x70 {
        /// Flag word.
        0x08 flags: u16,
        /// Limit read by `Update` (compared with the result of virtual slot `0xA4`).
        0x18 limit: f32,
        /// Delay `Update` subtracts from the time.
        0x20 delay: f32,
        /// The controlled target.
        0x2c target: Ptr,
        /// Force vector (four floats).
        0x40 force: [f32; 4],
        /// Time left.
        0x50 time_left: f32,
    }
}

// Translated from 00cb9260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkForceController::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body
/// (`00a6d290`) and, when bit 0 of `flags` is set and `this` is not null, frees the block
/// that starts `[this - 1]` bytes before it (the 16-byte alignment distance) through the
/// memory manager. Answers `this`.
pub fn bhk_force_controller_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(BASE_DESTRUCT, &args![this]);
    if flags & 1 != 0 && this.addr() != 0 {
        let distance = u32::from(e.mem.u8(this.addr().wrapping_sub(1)));
        let block = this.addr().wrapping_sub(distance);
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, block]);
    }
    this
}

// Translated from 00cb9290 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb9290 - the /GS cookie check is not modeled.
/// `bhkForceController::ApplyForce` (Xbox PDB): casts the target's field `+0x1C` to the
/// `NiRTTI` at `01267e64` and, only when the result's float at `+0x18` is below 1.0 (a NaN
/// is not) and the result's `+0x10` and that object's `+8` exist, calls `00631100` on the
/// latter with the time left (`+0x50`, through the x87 stack) and the address of the force
/// vector (`+0x40`) multiplied lane by lane by the constant at `01016410`.
pub fn bhk_force_controller_apply_force(e: &mut Engine, this: Ptr) {
    let target = e
        .get(
            this.cast::<BhkForceController>(),
            BhkForceController::target,
        )
        .addr();
    if target == 0 {
        return;
    }
    let inner = e.mem.u32(target + 0x1c);
    if inner == 0 {
        return;
    }
    let cast = e.call(IS_KIND_OF, &args![inner, APPLY_TARGET_RTTI]).u32();
    if cast == 0 {
        return;
    }
    let limit = x87_quiet(e.mem.f32(cast + 0x18));
    let below_one = 1.0 > limit;
    if !below_one {
        return;
    }
    let middle = e.mem.u32(cast + 0x10);
    if middle == 0 {
        return;
    }
    let receiver = e.mem.u32(middle + 8);
    if receiver == 0 {
        return;
    }
    let scale = e.mem.f32(FORCE_SCALE);
    let time = x87_quiet(e.get(
        this.cast::<BhkForceController>(),
        BhkForceController::time_left,
    ));
    e.with_stack(0x10, |e, product| {
        for lane in 0..4 {
            let component = e.mem.f32(this.addr() + 0x40 + 4 * lane);
            e.mem.set_f32(product.addr() + 4 * lane, scale * component);
        }
        e.call(APPLY_TO_TARGET, &args![receiver, time, product]);
    });
}

/// Creates a controller object: a 16-byte aligned 0x70-byte block, the base constructor,
/// the controller vtable, the default force vector at `+0x40` and `0.0` at `+0x50`.
fn create_controller(e: &mut Engine) -> u32 {
    let controller = aligned_allocate(e, 0x70);
    e.call(BASE_CONSTRUCT, &args![controller]);
    e.mem.set_u32(controller, CONTROLLER_VTABLE);
    for lane in 0..4 {
        let word = e.mem.u32(DEFAULT_VECTOR + 4 * lane);
        e.mem.set_u32(controller + 0x40 + 4 * lane, word);
    }
    e.mem.set_u32(controller + 0x50, 0);
    controller
}

/// The force vector the force-adding routines store: from the target's field `+0x1C` cast to
/// the `NiRTTI` at `012043f8`, whose `+0x10` object gives a magnitude (`0062ab20`, through the
/// x87 stack, scaled by the float at `011b1d3c`) and, through its `+8` object's float at
/// `+0x194` times the double at `0101de30`, a speed; with `scaled_vector` computing the
/// lane value from the magnitude `m`, the speed `s` and the input lane `l`. Without such a
/// chain the input vector itself.
fn force_vector(
    e: &mut Engine,
    node: u32,
    input: [f32; 4],
    scaled_vector: fn(f32, f32, f32) -> f32,
) -> [f32; 4] {
    let cast = if node == 0 {
        0
    } else {
        e.call(IS_KIND_OF, &args![node, FORCE_TARGET_RTTI]).u32()
    };
    if cast == 0 {
        return input;
    }
    let holder = e.mem.u32(cast + 0x10);
    if holder == 0 {
        return input;
    }
    let magnitude = x87_quiet(e.call(GET_MAGNITUDE, &args![holder]).f32());
    let factor = x87_quiet(e.mem.f32(MAGNITUDE_FACTOR));
    let source = e.mem.u32(holder + 8);
    let scaled = (f64::from(factor) * f64::from(magnitude)) as f32;
    let speed_source = x87_quiet(e.mem.f32(source + 0x194));
    let speed = (f64::from(speed_source) * e.mem.f64(SPEED_FACTOR)) as f32;
    let mut out = [0.0; 4];
    for (lane, value) in input.iter().enumerate() {
        out[lane] = scaled_vector(scaled, speed, *value);
    }
    out
}

/// Stores the controller's fields the force-adding routines share: `0.0` at `+0x14`, `limit`
/// at `+0x18`, the flag word with bits 1 and 3 cleared and bits 0 and 2 set, `0.0` at
/// `+0x10`, `1.0` at `+0xC`; then the force vector, and virtual slot `0x8C` with the
/// constant at `01015f5c`.
fn finish_controller(e: &mut Engine, controller: u32, limit: f32, force: [f32; 4]) {
    e.mem.set_f32(controller + 0x14, 0.0);
    e.mem.set_f32(controller + 0x18, limit);
    let flags = e.mem.u16(controller + 8);
    e.mem.set_u16(controller + 8, (flags & 0xfff5) | 5);
    e.mem.set_f32(controller + 0x10, 0.0);
    e.mem.set_f32(controller + 0xc, 1.0);
    for (lane, value) in force.iter().enumerate() {
        e.mem.set_f32(controller + 0x40 + 4 * lane as u32, *value);
    }
    let step = x87_quiet(e.mem.f32(STORED_STEP));
    e.vcall(controller, 0x8c, &args![step]);
}

// Translated from 00cb9320 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb9320 - the FS:[0] exception-unwinding frame is not modeled.
/// Adds a force to a node (no PDB name; cdecl, the exe's first force-adding routine): `node`
/// is used for the lookup (`00a5c570` with the controller `NiRTTI`) unless `target` is null,
/// when `node` is used; if the node has no controller yet, one is created and attached
/// through its virtual slot `0x98`. The force vector is the 16 bytes at `vector`, replaced
/// (when the node's `+0x1C` casts and its `+0x10` object exists) by the lane-wise
/// `t = m * l`, `t * s + t` of the magnitude `m` and speed `s` described at
/// `force_vector`. The controller's limit (`+0x18`) is the constant at `0101df44`.
pub fn fn_00cb9320(e: &mut Engine, node: u32, vector: u32, target: u32) {
    let target = if target == 0 { node } else { target };
    let mut controller = e
        .call(FIND_CONTROLLER, &args![target, CONTROLLER_RTTI])
        .u32();
    if controller == 0 {
        controller = create_controller(e);
        e.vcall(controller, 0x98, &args![target]);
    }
    let cast_node = e.mem.u32(target + 0x1c);
    let mut input = [0.0; 4];
    for (lane, value) in input.iter_mut().enumerate() {
        *value = e.mem.f32(vector + 4 * lane as u32);
    }
    let force = force_vector(e, cast_node, input, |m, s, l| {
        let t = m * l;
        s * t + t
    });
    let limit = x87_quiet(e.mem.f32(STORED_LIMIT));
    finish_controller(e, controller, limit, force);
}

// Translated from 00cb94a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb94a0 - the FS:[0] exception-unwinding frame is not modeled.
/// Adds a force to a node (no PDB name; cdecl, the exe's second force-adding routine), with
/// the allocation-scope word at 0x10: like `00cb9320` for one node, but the lane value is
/// `s * l + m * l` and the controller's limit (`+0x18`) is the float argument `limit`
/// (through the x87 stack).
pub fn fn_00cb94a0(e: &mut Engine, node: u32, vector: u32, limit: f32) {
    let (word, saved) = enter_scope(e);
    let mut controller = e.call(FIND_CONTROLLER, &args![node, CONTROLLER_RTTI]).u32();
    if controller == 0 {
        controller = create_controller(e);
        e.vcall(controller, 0x98, &args![node]);
    }
    let cast_node = e.mem.u32(node + 0x1c);
    let mut input = [0.0; 4];
    for (lane, value) in input.iter_mut().enumerate() {
        *value = e.mem.f32(vector + 4 * lane as u32);
    }
    let force = force_vector(e, cast_node, input, |m, s, l| s * l + m * l);
    finish_controller(e, controller, x87_quiet(limit), force);
    e.mem.set_u32(word, saved);
}

// Translated from 00cb9650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkForceController::RemoveForce` (Xbox PDB; cdecl, one node argument): for a non-null
/// `node`, looks up its controller with the controller `NiRTTI` (`00a5c570`) and, if there
/// is one, removes it (`00a5c480`); then asks the node's virtual slot `0xC` for a second
/// object and, when that exists, calls itself on each of its `u16` count (`+0xA6`, read
/// again after every call) entries of the array at `+0xA0`.
pub fn bhk_force_controller_remove_force(e: &mut Engine, node: u32) {
    if node == 0 {
        return;
    }
    let controller = e.call(FIND_CONTROLLER, &args![node, CONTROLLER_RTTI]).u32();
    if controller != 0 {
        e.call(REMOVE_CONTROLLER, &args![node, controller]);
    }
    let parent = e.vcall(node, 0xc, &args![]).u32();
    if parent == 0 {
        return;
    }
    let mut index = 0u32;
    if e.mem.u16(parent + 0xa6) != 0 {
        loop {
            let children = e.mem.u32(parent + 0xa0);
            let child = e.mem.u32(children.wrapping_add(index * 4));
            e.call(REMOVE_FORCE, &args![child]);
            let count = u32::from(e.mem.u16(parent + 0xa6));
            index += 1;
            if index >= count {
                break;
            }
        }
    }
}

// Translated from 00cb96c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb96c0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkForceController::CreateClone` (Xbox PDB): creates a controller (16-byte aligned
/// 0x70 bytes, base constructor, vtable, default vector, `0.0` time), copies `this` into it
/// through the base `CopyMembers` (`00a6cfe0`, `(this, clone, context)`), then copies the
/// force vector (`+0x40`, 16 bytes) and the time left (`+0x50`, through the x87 stack).
/// Answers the clone.
pub fn bhk_force_controller_create_clone(e: &mut Engine, this: Ptr, context: u32) -> u32 {
    let clone = create_controller(e);
    e.call(BASE_COPY_MEMBERS, &args![this, clone, context]);
    for lane in 0..4 {
        let word = e.mem.u32(this.addr() + 0x40 + 4 * lane);
        e.mem.set_u32(clone + 0x40 + 4 * lane, word);
    }
    let time = x87_quiet_bits(e.mem.u32(this.addr() + 0x50));
    e.mem.set_u32(clone + 0x50, time);
    clone
}

// Translated from 00cb9770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkForceController::Update` (Xbox PDB): sets the time left (`+0x50`) to the float at
/// `010c8ae4`; when flag bit 3 (`+8`) and the target (`+0x2c`) are set: unless the delay
/// (`+0x20`) equals the double at `010241b0`, stores `time - delay` as the time left, and
/// back to the constant when that is below zero. Then asks virtual slot `0xA4` with the
/// time, runs `ApplyForce`, and clears flag bit 3 when the answer equals the limit
/// (`+0x18`). `time` points at the float.
pub fn bhk_force_controller_update(e: &mut Engine, this: Ptr, time: u32) {
    let object = this.addr();
    let initial = x87_quiet(e.mem.f32(INITIAL_TIME));
    e.mem.set_f32(object + 0x50, initial);
    if (e.mem.u8(object + 8) >> 3) & 1 == 0 || e.mem.u32(object + 0x2c) == 0 {
        return;
    }
    let delay = x87_quiet(e.mem.f32(object + 0x20));
    if f64::from(delay) != e.mem.f64(THRESHOLD) {
        let now = x87_quiet(e.mem.f32(time));
        let delay = x87_quiet(e.mem.f32(object + 0x20));
        let left = x87_quiet((f64::from(now) - f64::from(delay)) as f32);
        e.mem.set_f32(object + 0x50, left);
        if 0.0 > left {
            e.mem.set_f32(object + 0x50, initial);
        }
    }
    let now = x87_quiet(e.mem.f32(time));
    let answer = x87_quiet(e.vcall(object, 0xa4, &args![now]).f32());
    e.call(APPLY_FORCE, &args![this]);
    let limit = x87_quiet(e.mem.f32(object + 0x18));
    if limit == answer {
        let flags = e.mem.u16(object + 8);
        e.mem.set_u16(object + 8, flags & 0xfff7);
    }
}

// Translated from 00cb9250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkForceController::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_83f4.
pub fn bhk_force_controller_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_83f4
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cb9250, bhk_force_controller_get_r_t_t_i() -> u32),
        entry!(
            0x00cb_9260,
            bhk_force_controller_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00cb_9290, bhk_force_controller_apply_force(Ptr)),
        entry!(0x00cb_9320, fn_00cb9320(u32, u32, u32)),
        entry!(0x00cb_94a0, fn_00cb94a0(u32, u32, f32)),
        entry!(0x00cb_9650, bhk_force_controller_remove_force(u32)),
        entry!(
            0x00cb_96c0,
            bhk_force_controller_create_clone(Ptr, u32) -> u32
        ),
        entry!(0x00cb_9770, bhk_force_controller_update(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::super::bhkentity::testing::*;
    use super::super::bhkentity::SCOPE_WORD;
    use super::*;

    /// Maps and fills a float constant.
    fn constant(e: &mut Engine, address: u32, value: f32) {
        e.map(address, 8);
        e.mem.set_f32(address, value);
    }

    #[test]
    fn deleting_destructor_frees_the_aligned_block_start() {
        let mut e = logging_engine();
        e.register(BASE_DESTRUCT, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_FREE, |_, _| Ret::default());
        let block = e.mem.alloc(0x90);
        let this = block + 6;
        e.mem.set_u8(this - 1, 6);
        assert_eq!(e.call(0x00cb_9260, &args![this, 0u32]).u32(), this);
        assert!(calls_to(&e, MEMORY_MANAGER_FREE).is_empty());
        e.call(0x00cb_9260, &args![this, 1u32]);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER, block]]
        );
        assert_eq!(calls_to(&e, BASE_DESTRUCT), vec![vec![this]; 2]);
        // A null object is never freed.
        e.call(0x00cb_9260, &args![0u32, 1u32]);
        assert_eq!(calls_to(&e, MEMORY_MANAGER_FREE).len(), 1);
    }

    /// A controller `this` whose target chain leads to a receiver, with the cast result
    /// holding `limit`.
    fn apply_scene(e: &mut Engine, limit: f32) -> (u32, u32) {
        let this = e.mem.alloc(0x70);
        let target = e.mem.alloc(0x40);
        let inner = e.mem.alloc(0x10);
        let cast = e.mem.alloc(0x30);
        let middle = e.mem.alloc(0x20);
        let receiver = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x2c, target);
        e.mem.set_u32(target + 0x1c, inner);
        e.mem.set_u32(cast + 0x10, middle);
        e.mem.set_f32(cast + 0x18, limit);
        e.mem.set_u32(middle + 8, receiver);
        e.mem.set_u32(0x0900_1000, cast);
        e.mem.set_f32(this + 0x50, 7.0);
        for lane in 0..4 {
            e.mem.set_f32(this + 0x40 + 4 * lane, 1.0 + lane as f32);
        }
        (this, receiver)
    }

    #[test]
    fn apply_force_scales_the_vector_for_a_limit_below_one() {
        let mut e = logging_engine();
        e.map(0x0900_1000, 4);
        e.register(IS_KIND_OF, |e, _| Ret {
            eax: e.mem.u32(0x0900_1000),
            ..Ret::default()
        });
        e.register(APPLY_TO_TARGET, |e, a| {
            for lane in 0..4 {
                let word = e.mem.u32(a[2] + 4 * lane);
                e.mem.set_u32(0x0900_2000 + 4 * lane, word);
            }
            Ret::default()
        });
        e.map(0x0900_2000, 0x10);
        constant(&mut e, FORCE_SCALE, 2.0);
        let (this, receiver) = apply_scene(&mut e, 0.5);
        e.call(0x00cb_9290, &args![this]);
        let calls = calls_to(&e, APPLY_TO_TARGET);
        assert_eq!(calls.len(), 1);
        assert_eq!((calls[0][0], calls[0][1]), (receiver, 7.0f32.to_bits()));
        for lane in 0..4 {
            assert_eq!(e.mem.f32(0x0900_2000 + 4 * lane), 2.0 * (1.0 + lane as f32));
        }
        let inner = e.mem.u32(e.mem.u32(this + 0x2c) + 0x1c);
        assert_eq!(
            calls_to(&e, IS_KIND_OF),
            vec![vec![inner, APPLY_TARGET_RTTI]]
        );
    }

    #[test]
    fn apply_force_does_nothing_for_a_limit_of_one_or_a_nan() {
        let mut e = logging_engine();
        e.map(0x0900_1000, 4);
        e.register(IS_KIND_OF, |e, _| Ret {
            eax: e.mem.u32(0x0900_1000),
            ..Ret::default()
        });
        e.register(APPLY_TO_TARGET, |_, _| Ret::default());
        constant(&mut e, FORCE_SCALE, 2.0);
        let (this, _) = apply_scene(&mut e, 1.0);
        e.call(0x00cb_9290, &args![this]);
        let cast = e.mem.u32(0x0900_1000);
        e.mem.set_u32(cast + 0x18, 0x7fc0_0000);
        e.call(0x00cb_9290, &args![this]);
        // A missing target or receiver ends it quietly too.
        e.mem.set_f32(cast + 0x18, 0.0);
        let middle = e.mem.u32(cast + 0x10);
        e.mem.set_u32(middle + 8, 0);
        e.call(0x00cb_9290, &args![this]);
        e.mem.set_u32(this + 0x2c, 0);
        e.call(0x00cb_9290, &args![this]);
        assert!(calls_to(&e, APPLY_TO_TARGET).is_empty());
    }

    /// Doubles shared by the two force-adding routines: allocation, constructor, lookup,
    /// cast (answering the object at `0x0900_1000`), magnitude and the vtable slots.
    fn add_force_doubles(e: &mut Engine) {
        allocator_doubles(e);
        e.register(BASE_CONSTRUCT, |_, _| Ret::default());
        e.register(IS_KIND_OF, |e, _| Ret {
            eax: e.mem.u32(0x0900_1000),
            ..Ret::default()
        });
        e.register(GET_MAGNITUDE, |_, _| Ret {
            st0: 4.0,
            ..Ret::default()
        });
        e.register(0x0900_0098, |_, _| Ret::default());
        e.register(0x0900_008c, |_, _| Ret::default());
        e.map(CONTROLLER_VTABLE, 0x100);
        e.mem.set_u32(CONTROLLER_VTABLE + 0x98, 0x0900_0098);
        e.mem.set_u32(CONTROLLER_VTABLE + 0x8c, 0x0900_008c);
        e.map(0x0900_1000, 4);
        e.map(DEFAULT_VECTOR, 0x10);
        constant(e, MAGNITUDE_FACTOR, 0.5);
        constant(e, STORED_LIMIT, 0.25);
        constant(e, STORED_STEP, 3.0);
        e.map(SPEED_FACTOR, 8);
        e.mem.set_f64(SPEED_FACTOR, 0.75);
    }

    /// A controller object whose vtable has the attach (`0x98`) and finish (`0x8c`) doubles.
    fn controller(e: &mut Engine) -> u32 {
        let controller = e.mem.alloc(0x70);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(controller, vtable);
        e.mem.set_u32(vtable + 0x98, 0x0900_0098);
        e.mem.set_u32(vtable + 0x8c, 0x0900_008c);
        controller
    }

    /// A node whose `+0x1C` leads to a cast object with the magnitude holder and source.
    fn force_node(e: &mut Engine) -> u32 {
        let node = e.mem.alloc(0x40);
        let inner = e.mem.alloc(0x10);
        let cast = e.mem.alloc(0x30);
        let holder = e.mem.alloc(0x20);
        let source = e.mem.alloc(0x200);
        e.mem.set_u32(node + 0x1c, inner);
        e.mem.set_u32(cast + 0x10, holder);
        e.mem.set_u32(holder + 8, source);
        e.mem.set_f32(source + 0x194, 2.0);
        e.mem.set_u32(0x0900_1000, cast);
        node
    }

    #[test]
    fn first_add_force_creates_a_controller_and_stores_the_scaled_vector() {
        let mut e = logging_engine();
        add_force_doubles(&mut e);
        e.register(FIND_CONTROLLER, |_, _| Ret::default());
        let node = force_node(&mut e);
        let vector = e.mem.alloc(0x20);
        for lane in 0..4 {
            e.mem.set_f32(vector + 4 * lane, 1.0 + lane as f32);
        }
        e.call(0x00cb_9320, &args![node, vector, 0u32]);
        assert_eq!(
            calls_to(&e, FIND_CONTROLLER),
            vec![vec![node, CONTROLLER_RTTI]]
        );
        let created = calls_to(&e, BASE_CONSTRUCT);
        assert_eq!(created.len(), 1);
        let controller = created[0][0];
        assert_eq!(controller & 0xf, 0);
        assert_eq!(e.mem.u32(controller), CONTROLLER_VTABLE);
        assert_eq!(calls_to(&e, GET_MAGNITUDE).len(), 1);
        // m = 0.5 * 4 = 2, s = 2 * 0.75 = 1.5, t = m * l, lane = s * t + t.
        for lane in 0..4 {
            let l = 1.0 + lane as f32;
            let t = 2.0 * l;
            assert_eq!(e.mem.f32(controller + 0x40 + 4 * lane), 1.5 * t + t);
        }
        assert_eq!(e.mem.f32(controller + 0x18), 0.25);
        assert_eq!(e.mem.f32(controller + 0x14), 0.0);
        assert_eq!(e.mem.f32(controller + 0xc), 1.0);
        assert_eq!(e.mem.u16(controller + 8), 5);
        let attach = calls_to(&e, 0x0900_0098);
        assert_eq!(attach, vec![vec![controller, node]]);
        assert_eq!(
            calls_to(&e, 0x0900_008c),
            vec![vec![controller, 3.0f32.to_bits()]]
        );
    }

    #[test]
    fn first_add_force_reuses_a_controller_and_the_second_target() {
        let mut e = logging_engine();
        add_force_doubles(&mut e);
        let existing = controller(&mut e);
        e.mem.set_u16(existing + 8, 0xffff);
        e.register(FIND_CONTROLLER, |e, _| Ret {
            eax: e.mem.u32(0x0900_3000),
            ..Ret::default()
        });
        e.map(0x0900_3000, 4);
        e.mem.set_u32(0x0900_3000, existing);
        e.mem.set_u32(0x0900_1000, 0);
        let node = e.mem.alloc(0x40);
        let other = e.mem.alloc(0x40);
        let vector = e.mem.alloc(0x20);
        for lane in 0..4 {
            e.mem.set_f32(vector + 4 * lane, 1.5 + lane as f32);
        }
        e.call(0x00cb_9320, &args![node, vector, other]);
        // The lookup used the second node; nothing was created or attached.
        assert_eq!(calls_to(&e, FIND_CONTROLLER)[0][0], other);
        assert!(calls_to(&e, BASE_CONSTRUCT).is_empty());
        assert!(calls_to(&e, 0x0900_0098).is_empty());
        // The cast failed, so the input vector is stored unchanged.
        assert_eq!(e.mem.f32(existing + 0x40), 1.5);
        assert_eq!(e.mem.f32(existing + 0x4c), 4.5);
        assert_eq!(e.mem.u16(existing + 8), 0xfff5 | 5);
    }

    #[test]
    fn second_add_force_uses_the_other_lane_formula_and_the_limit_argument() {
        let mut e = logging_engine();
        add_force_doubles(&mut e);
        e.register(FIND_CONTROLLER, |_, _| Ret::default());
        let node = force_node(&mut e);
        let vector = e.mem.alloc(0x20);
        for lane in 0..4 {
            e.mem.set_f32(vector + 4 * lane, 1.0 + lane as f32);
        }
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 9);
        e.call(0x00cb_94a0, &args![node, vector, 0.125f32]);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 9);
        let controller = calls_to(&e, BASE_CONSTRUCT)[0][0];
        for lane in 0..4 {
            let l = 1.0 + lane as f32;
            assert_eq!(e.mem.f32(controller + 0x40 + 4 * lane), 1.5 * l + 2.0 * l);
        }
        assert_eq!(e.mem.f32(controller + 0x18), 0.125);
        assert_eq!(calls_to(&e, 0x0900_0098), vec![vec![controller, node]]);
    }

    #[test]
    fn remove_force_removes_the_controller_and_recurses_into_the_children() {
        let mut e = logging_engine();
        e.register(FIND_CONTROLLER, |_, a| Ret {
            eax: if a[0] == 0x4000 { 0x9999 } else { 0 },
            ..Ret::default()
        });
        e.register(REMOVE_CONTROLLER, |_, _| Ret::default());
        e.register(0x0900_000c, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x30),
            ..Ret::default()
        });
        let make_node = |e: &mut Engine, parent: u32| {
            let node = e.mem.alloc(0x40);
            let vtable = e.mem.alloc(0x40);
            e.mem.set_u32(node, vtable);
            e.mem.set_u32(vtable + 0xc, 0x0900_000c);
            e.mem.set_u32(node + 0x30, parent);
            node
        };
        let child_a = make_node(&mut e, 0);
        let child_b = make_node(&mut e, 0);
        let parent = e.mem.alloc(0x100);
        let children = e.mem.alloc(8);
        e.mem.set_u32(children, child_a);
        e.mem.set_u32(children + 4, child_b);
        e.mem.set_u32(parent + 0xa0, children);
        e.mem.set_u16(parent + 0xa6, 2);
        let root = make_node(&mut e, parent);
        e.call(0x00cb_9650, &args![root]);
        let looked_up: Vec<u32> = calls_to(&e, FIND_CONTROLLER).iter().map(|c| c[0]).collect();
        assert_eq!(looked_up, vec![root, child_a, child_b]);
        e.call(0x00cb_9650, &args![0u32]);
        assert_eq!(calls_to(&e, FIND_CONTROLLER).len(), 3);
        // A node with a controller has it removed.
        let attached = make_node(&mut e, 0);
        e.register(FIND_CONTROLLER, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.call(0x00cb_9650, &args![attached]);
        assert_eq!(
            calls_to(&e, REMOVE_CONTROLLER),
            vec![vec![attached, attached + 1]]
        );
    }

    #[test]
    fn create_clone_copies_the_vector_and_time() {
        let mut e = logging_engine();
        allocator_doubles(&mut e);
        e.register(BASE_CONSTRUCT, |_, _| Ret::default());
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        e.map(DEFAULT_VECTOR, 0x10);
        let this = e.mem.alloc(0x70);
        for lane in 0..4 {
            e.mem.set_f32(this + 0x40 + 4 * lane, 1.0 + lane as f32);
        }
        e.mem.set_u32(this + 0x50, 0x7f80_0001);
        let clone = e.call(0x00cb_96c0, &args![this, 0x77u32]).u32();
        assert_eq!(clone & 0xf, 0);
        assert_eq!(e.mem.u32(clone), CONTROLLER_VTABLE);
        assert_eq!(
            calls_to(&e, BASE_COPY_MEMBERS),
            vec![vec![this, clone, 0x77]]
        );
        assert_eq!(e.mem.f32(clone + 0x40), 1.0);
        assert_eq!(e.mem.f32(clone + 0x4c), 4.0);
        assert_eq!(e.mem.u32(clone + 0x50), 0x7fc0_0001);
        assert_eq!(calls_to(&e, BASE_CONSTRUCT), vec![vec![clone]]);
    }

    /// An `Update` scene: flags with bit 3, a target, delay, limit, and slot `0xA4` answering
    /// the word at `0x0900_5000` as a float.
    fn update_scene(e: &mut Engine, delay: f32, limit: f32, answer: f32) -> (u32, u32) {
        e.map(0x0900_5000, 4);
        e.mem.set_f32(0x0900_5000, answer);
        e.register(0x0900_00a4, |e, _| Ret {
            st0: f64::from(e.mem.f32(0x0900_5000)),
            ..Ret::default()
        });
        e.register(APPLY_FORCE, |_, _| Ret::default());
        constant(e, INITIAL_TIME, 5.0);
        e.map(THRESHOLD, 8);
        e.mem.set_f64(THRESHOLD, -3.4028234663852886e38);
        let this = e.mem.alloc(0x70);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(vtable + 0xa4, 0x0900_00a4);
        e.mem.set_u16(this + 8, 0x0f);
        e.mem.set_u32(this + 0x2c, 0x1234);
        e.mem.set_f32(this + 0x20, delay);
        e.mem.set_f32(this + 0x18, limit);
        let time = e.mem.alloc(8);
        e.mem.set_f32(time, 2.0);
        (this, time)
    }

    #[test]
    fn update_subtracts_the_delay_and_clears_bit_3_when_the_limit_is_reached() {
        let mut e = logging_engine();
        let (this, time) = update_scene(&mut e, 0.5, 9.0, 9.0);
        e.call(0x00cb_9770, &args![this, time]);
        assert_eq!(e.mem.f32(this + 0x50), 1.5);
        assert_eq!(e.mem.u16(this + 8), 0x07);
        assert_eq!(
            calls_to(&e, 0x0900_00a4),
            vec![vec![this, 2.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, APPLY_FORCE), vec![vec![this]]);
    }

    #[test]
    fn update_resets_a_negative_time_and_keeps_bit_3_for_another_answer() {
        let mut e = logging_engine();
        let (this, time) = update_scene(&mut e, 4.0, 9.0, 8.0);
        e.call(0x00cb_9770, &args![this, time]);
        assert_eq!(e.mem.f32(this + 0x50), 5.0);
        assert_eq!(e.mem.u16(this + 8), 0x0f);
        // A NaN answer never equals the limit.
        e.mem.set_u32(0x0900_5000, 0x7fc0_0000);
        e.call(0x00cb_9770, &args![this, time]);
        assert_eq!(e.mem.u16(this + 8), 0x0f);
    }

    #[test]
    fn update_with_the_threshold_delay_skips_the_subtraction() {
        let mut e = logging_engine();
        let (this, time) = update_scene(&mut e, f32::MIN * 1.0, 9.0, 8.0);
        e.call(0x00cb_9770, &args![this, time]);
        assert_eq!(e.mem.f32(this + 0x50), 5.0);
        assert_eq!(calls_to(&e, APPLY_FORCE).len(), 1);
    }

    #[test]
    fn update_without_bit_3_or_target_only_resets_the_time() {
        let mut e = logging_engine();
        let (this, time) = update_scene(&mut e, 0.5, 9.0, 9.0);
        e.mem.set_u16(this + 8, 0x07);
        e.mem.set_f32(this + 0x50, 1.0);
        e.call(0x00cb_9770, &args![this, time]);
        assert_eq!(e.mem.f32(this + 0x50), 5.0);
        e.mem.set_u16(this + 8, 0x0f);
        e.mem.set_u32(this + 0x2c, 0);
        e.call(0x00cb_9770, &args![this, time]);
        assert!(calls_to(&e, APPLY_FORCE).is_empty());
        assert!(calls_to(&e, 0x0900_00a4).is_empty());
    }

    #[test]
    fn bhk_force_controller_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cb_9250, &args![]).u32(), 0x0126_83f4);
    }
}
