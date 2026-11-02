//! `BSHavok/hkorienthingedbodyaction.obj` (Xbox PDB source unit), subsystem `BSHavok`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkOrientHingedBodyAction` (Xbox PDB, 0x60 bytes, vtable `010ce504`) is a Havok unary action.
//! It keeps two 16-byte vectors at `+0x20` and `+0x30` (copied from the constructor's
//! arguments), a third at `+0x40` (zero at construction), two floats at `+0x50` and `+0x54`,
//! the last angle at `+0x58` and an enable byte at `+0x5c`.
//!
//! Floats: x87 `FLD`/`FSTP` copies go through [`x87_quiet`]; the x87 arithmetic is done in
//! `f64` and rounded to `f32` where the exe stores to memory; the SSE code runs in `f32` with
//! the lane order of the exe. `RSQRTSS` is the host CPU's estimate (`rsqrt_estimate`).
//! `applyAction` carries a `/GS` cookie check (`00ec408c`) and `clone` an `FS:[0]` exception
//! frame; neither is modeled, so both are marked `Translation incomplete`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::x87::rsqrt_estimate;

use super::bhkentity::{enter_scope, havok_allocate};

/// The base action constructor (`00d2b260`, `this` plus two words).
const ACTION_CONSTRUCT: u32 = 0x00d2_b260;
/// The class vtable the constructor stores.
const CLASS_VTABLE: u32 = 0x010c_e504;
/// `hkMath::acos` (`00c74a70`, cdecl, one float; the answer is in ST0).
const ACOS: u32 = 0x00c7_4a70;
/// Run on the body before the impulse is applied (`00c9c1d0`, `this` only).
const PREPARE_BODY: u32 = 0x00c9_c1d0;
/// Virtual slot (of the object embedded at body `+0xE0`) that receives the vector.
const APPLY_SLOT: u32 = 0x54;
/// The vector (`010c71e0`) added to the squared fourth lane, the constants 0.0, 3.0 and 0.5
/// (the `RSQRTSS` refinement) and the mask that clears the sign bits (`01017d10`).
const SQUARE_BIAS: u32 = 0x010c_71e0;
const ZERO: u32 = 0x0101_1d78;
const THREE: u32 = 0x0101_7718;
const HALF: u32 = 0x0101_6248;
const SIGN_MASK: u32 = 0x0101_7d10;
/// Size of an `hkOrientHingedBodyAction`.
const ACTION_SIZE: u32 = 0x60;

type Vec4 = [f32; 4];

fn load_vec(e: &Engine, address: u32) -> Vec4 {
    std::array::from_fn(|lane| e.mem.f32(address + 4 * lane as u32))
}

fn store_vec(e: &mut Engine, address: u32, lanes: Vec4) {
    for (lane, value) in lanes.into_iter().enumerate() {
        e.mem.set_f32(address + 4 * lane as u32, value);
    }
}

/// `z + (y + x)`: the `SHUFPS`/`ADDSS` sum of the first three lanes.
fn sum3(lanes: Vec4) -> f32 {
    lanes[2] + (lanes[1] + lanes[0])
}

/// The vector `vector` turned by the quaternion `turn` (x, y, z, w) the way the exe does:
/// `2 * (cross * w + ((w * w + bias) * vector + dot * turn))`, with the cross product lanes
/// `(v2 r1 - v1 r2, v0 r2 - v2 r0, v1 r0 - v0 r1, v3 r3 - v3 r3)` and `dot` the sum of the
/// first three lanes of `vector * turn`.
fn turned(turn: Vec4, vector: Vec4, bias: Vec4) -> Vec4 {
    let w = turn[3];
    let dot = sum3(std::array::from_fn(|lane| vector[lane] * turn[lane]));
    let cross = [
        vector[2] * turn[1] - vector[1] * turn[2],
        vector[0] * turn[2] - vector[2] * turn[0],
        vector[1] * turn[0] - vector[0] * turn[1],
        vector[3] * turn[3] - vector[3] * turn[3],
    ];
    std::array::from_fn(|lane| {
        let half = (w * w + bias[lane]) * vector[lane];
        let sum = half + dot * turn[lane];
        let both = cross[lane] * w + sum;
        both + both
    })
}

/// The cross product lanes `(a2 b1 - a1 b2, a0 b2 - a2 b0, a1 b0 - a0 b1, a3 b3 - a3 b3)`.
fn cross(a: Vec4, b: Vec4) -> Vec4 {
    [
        a[2] * b[1] - a[1] * b[2],
        a[0] * b[2] - a[2] * b[0],
        a[1] * b[0] - a[0] * b[1],
        a[3] * b[3] - a[3] * b[3],
    ]
}

/// The index of the largest of the first three absolute values the way the exe's `FCOM` pairs
/// pick it: lane 1 beats lane 0 only when strictly larger (not for a NaN), and lane 2 beats
/// the winner only when strictly larger.
fn largest_lane(lanes: Vec4) -> usize {
    let strictly_larger = |a: f32, b: f32| f64::from(a) > f64::from(b);
    if strictly_larger(lanes[1], lanes[0]) {
        if strictly_larger(lanes[2], lanes[1]) {
            2
        } else {
            1
        }
    } else if strictly_larger(lanes[2], lanes[0]) {
        2
    } else {
        0
    }
}

fn absolute(e: &Engine, lanes: Vec4) -> Vec4 {
    let mask = e.mem.u32(SIGN_MASK);
    lanes.map(|lane| f32::from_bits(lane.to_bits() & mask))
}

// Translated from 00d2acf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOrientHingedBodyAction::hkOrientHingedBodyAction` (Xbox PDB): takes six words, the first
/// and the last go to the base constructor (`00d2b260`) with `this`; the second and third are
/// the addresses of two 16-byte vectors copied to `+0x20` and `+0x30`; the fourth and fifth
/// are floats copied (through the x87 stack) to `+0x50` and `+0x54`. Stores the vtable
/// 0x010ce504, clears the byte at `+0x5c` and the vector at `+0x40`. Answers `this`.
pub fn hk_orient_hinged_body_action_hk_orient_hinged_body_action(
    e: &mut Engine,
    this: u32,
    first: u32,
    target: u32,
    hinge: u32,
    gain: f32,
    damping: f32,
    last: u32,
) -> u32 {
    e.call(ACTION_CONSTRUCT, &args![this, first, last]);
    e.mem.set_f32(this + 0x50, x87_quiet(gain));
    e.mem.set_f32(this + 0x54, x87_quiet(damping));
    e.mem.set_u32(this, CLASS_VTABLE);
    e.mem.set_u8(this + 0x5c, 0);
    let target_lanes = load_vec(e, target);
    store_vec(e, this + 0x20, target_lanes);
    let hinge_lanes = load_vec(e, hinge);
    store_vec(e, this + 0x30, hinge_lanes);
    for lane in 0..4 {
        e.mem.set_u32(this + 0x40 + 4 * lane, 0);
    }
    this
}

// Translated from 00d2ad50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d2ad50 - the /GS cookie check (00ec408c) is not modeled.
/// `hkOrientHingedBodyAction::applyAction` (Xbox PDB): takes the step information, whose float
/// at `+0xC` is the step. Does nothing when the byte at `+0x5c` is 0. Otherwise, with the
/// body's orientation quaternion at body `+0x160` (the body is the word at `+0x18`), turns the
/// vector at `+0x30` and the vector at `+0x20` by it, normalizes the difference the turned
/// second vector leaves on the vector at `+0x40` (`v = s - (b . s) * b`, scaled by the refined
/// reciprocal length), takes the angle `acos` (`00c74a70`) of its dot with the turned first
/// vector and stores it at `+0x58`, negated when the signs of the largest-magnitude lanes of
/// the cross product and of the turned second vector differ. Then runs `00c9c1d0` on the body
/// and virtual slot 0x54 of the object at body `+0xE0` with the vector
/// `gain * angle * step * b - damping * step * (body vector at +0x1c0)`.
pub fn hk_orient_hinged_body_action_apply_action(e: &mut Engine, this: u32, info: u32) {
    if e.mem.u8(this + 0x5c) == 0 {
        return;
    }
    let body = e.mem.u32(this + 0x18);
    let bias = load_vec(e, SQUARE_BIAS);
    let turn = load_vec(e, body + 0x160);
    let first = load_vec(e, this + 0x30);
    let turned_first = turned(turn, first, bias);
    let second = load_vec(e, this + 0x20);
    let turn_again = load_vec(e, body + 0x160);
    let turned_second = turned(turn_again, second, bias);
    let reference = load_vec(e, this + 0x40);
    let products: Vec4 = std::array::from_fn(|lane| turned_second[lane] * reference[lane]);
    let reach = -x87_quiet(sum3(products));
    let moved: Vec4 = std::array::from_fn(|lane| reach * turned_second[lane] + reference[lane]);
    let squares: Vec4 = moved.map(|lane| lane * lane);
    let sum = sum3(squares);
    let zero: f32 = e.global(ZERO);
    let three: f32 = e.global(THREE);
    let half: f32 = e.global(HALF);
    let estimate = rsqrt_estimate(sum);
    let step = three - (sum * estimate) * estimate;
    let refined = (half * estimate) * step;
    let inverse = if sum == zero { 0.0 } else { refined };
    let direction: Vec4 = moved.map(|lane| inverse * lane);
    let aligned: Vec4 = std::array::from_fn(|lane| direction[lane] * turned_first[lane]);
    let cosine = x87_quiet(sum3(aligned));
    let angle = e.call(ACOS, &args![cosine]).f32();
    let angle = x87_quiet(angle);
    e.mem.set_f32(this + 0x58, angle);
    let axis = cross(direction, turned_first);
    let axis_magnitudes = absolute(e, axis);
    let axis_pick = x87_quiet(axis[largest_lane(axis_magnitudes)]);
    let hinge_magnitudes = absolute(e, turned_second);
    let hinge_pick = x87_quiet(turned_second[largest_lane(hinge_magnitudes)]);
    if (axis_pick < 0.0) != (hinge_pick < 0.0) {
        e.mem.set_f32(this + 0x58, -angle);
    }
    let dt = e.mem.f32(info + 0xc);
    let damping_step = x87_quiet(e.mem.f32(this + 0x54));
    let spin_gain = f64::from(x87_quiet(e.mem.f32(this + 0x50)));
    let velocity = load_vec(e, body + 0x1c0);
    let damping_scale = (f64::from(damping_step) * f64::from(dt)) as f32;
    let damping: Vec4 = velocity.map(|lane| damping_scale * lane);
    let stored_angle = f64::from(e.mem.f32(this + 0x58));
    let impulse_scale = ((spin_gain * stored_angle) * f64::from(dt)) as f32;
    let impulse: Vec4 =
        std::array::from_fn(|lane| impulse_scale * turned_second[lane] - damping[lane]);
    e.call(PREPARE_BODY, &args![body]);
    e.with_stack(0x10, |e, vector| {
        store_vec(e, vector.addr(), impulse);
        e.vcall(body + 0xe0, APPLY_SLOT, &args![vector]);
    });
}

// Translated from 00d2b110 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d2b110 - the FS:[0] exception frame (unwinding that restores the scope word) is not modeled.
/// `hkOrientHingedBodyAction::clone` (Xbox PDB): takes two words. With the allocation-scope
/// word set to 0x10, answers 0 unless the word at `+4` of the first is 2 and the word at `+4`
/// of the second is 0. Otherwise allocates 0x60 bytes from the Havok allocator, builds a new
/// action on it with the constructor on `(the word the first's first word points to,
/// this + 0x20, this + 0x30, the floats at +0x50 and +0x54, the word at this + 0x10)` and
/// copies the vector at `+0x40` to the new one. Answers the new action. The scope word is put
/// back.
pub fn hk_orient_hinged_body_action_clone(
    e: &mut Engine,
    this: u32,
    first: u32,
    second: u32,
) -> u32 {
    let (word, saved) = enter_scope(e);
    let mut result = 0;
    if e.mem.u32(first + 4) == 2 && e.mem.u32(second + 4) == 0 {
        let block = havok_allocate(e, ACTION_SIZE);
        let owner = e.mem.u32(e.mem.u32(first));
        let gain = f32::from_bits(x87_quiet_bits(e.mem.u32(this + 0x50)));
        let damping = f32::from_bits(x87_quiet_bits(e.mem.u32(this + 0x54)));
        let world = e.mem.u32(this + 0x10);
        result = hk_orient_hinged_body_action_hk_orient_hinged_body_action(
            e,
            block,
            owner,
            this + 0x20,
            this + 0x30,
            gain,
            damping,
            world,
        );
        let reference = load_vec(e, this + 0x40);
        store_vec(e, result + 0x40, reference);
    }
    e.mem.set_u32(word, saved);
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d2acf0,
            hk_orient_hinged_body_action_hk_orient_hinged_body_action(
                u32, u32, u32, u32, f32, f32, u32
            ) -> u32
        ),
        entry!(
            0x00d2ad50,
            hk_orient_hinged_body_action_apply_action(u32, u32)
        ),
        entry!(
            0x00d2b110,
            hk_orient_hinged_body_action_clone(u32, u32, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bshavok::bhkentity::testing::{allocator_doubles, calls_to, logging_engine};
    use crate::units::bshavok::bhkentity::SCOPE_WORD;

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// The double standing for the slot of the object embedded in the body.
    const SLOT_DOUBLE: u32 = 0x0900_0054;

    fn vector(e: &mut Engine, lanes: Vec4) -> u32 {
        let address = e.mem.alloc(0x10);
        store_vec(e, address, lanes);
        address
    }

    #[test]
    fn construction_copies_the_vectors_and_floats() {
        let mut e = logging_engine();
        e.register(ACTION_CONSTRUCT, nothing);
        let this = e.mem.alloc(0x60);
        let target = vector(&mut e, [1.0, 2.0, 3.0, 4.0]);
        let hinge = vector(&mut e, [5.0, 6.0, 7.0, 8.0]);
        e.mem.set_u8(this + 0x5c, 9);
        e.mem.set_u32(this + 0x44, 0x1234);
        let result = e.call(
            0x00d2_acf0,
            &args![this, 0x11u32, target, hinge, 2.5f32, 0.75f32, 0x22u32],
        );
        assert_eq!(result.u32(), this);
        assert_eq!(calls_to(&e, ACTION_CONSTRUCT), [[this, 0x11, 0x22]]);
        assert_eq!(e.mem.u32(this), CLASS_VTABLE);
        assert_eq!(load_vec(&e, this + 0x20), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vec(&e, this + 0x30), [5.0, 6.0, 7.0, 8.0]);
        assert_eq!(load_vec(&e, this + 0x40), [0.0; 4]);
        assert_eq!(e.mem.f32(this + 0x50), 2.5);
        assert_eq!(e.mem.f32(this + 0x54), 0.75);
        assert_eq!(e.mem.u8(this + 0x5c), 0);
    }

    #[test]
    fn construction_quiets_signalling_floats() {
        let mut e = logging_engine();
        e.register(ACTION_CONSTRUCT, nothing);
        let this = e.mem.alloc(0x60);
        let target = vector(&mut e, [0.0; 4]);
        let hinge = vector(&mut e, [0.0; 4]);
        let signalling = f32::from_bits(0x7f80_0001);
        e.call(
            0x00d2_acf0,
            &args![this, 0u32, target, hinge, signalling, 1.0f32, 0u32],
        );
        assert_eq!(e.mem.u32(this + 0x50), 0x7fc0_0001);
    }

    /// An action on a body with the identity orientation, a hinge along z, a target along x and
    /// the vector at `+0x40` set to `reference`; the doubles record the vector that reaches
    /// the body's slot at `record` (4 lanes).
    fn setup(reference: Vec4) -> (Engine, u32, u32, u32) {
        let mut e = logging_engine();
        for (address, bits) in [
            (SQUARE_BIAS, (-0.5f32).to_bits()),
            (SQUARE_BIAS + 4, (-0.5f32).to_bits()),
            (SQUARE_BIAS + 8, (-0.5f32).to_bits()),
            (SQUARE_BIAS + 12, (-0.5f32).to_bits()),
            (ZERO, 0),
            (THREE, 3.0f32.to_bits()),
            (HALF, 0.5f32.to_bits()),
            (SIGN_MASK, 0x7fff_ffff),
        ] {
            e.map(address, 4);
            e.mem.set_u32(address, bits);
        }
        e.register(PREPARE_BODY, nothing);
        e.register(ACOS, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])).acos(),
            ..Ret::default()
        });
        let record = e.mem.alloc(0x10);
        e.register_double(SLOT_DOUBLE, move |e, a| {
            for lane in 0..4 {
                let value = e.mem.u32(a[1] + 4 * lane);
                e.mem.set_u32(record + 4 * lane, value);
            }
            Ret::default()
        });
        let body = e.mem.alloc(0x200);
        let slot_vtable = e.mem.alloc(0x100);
        e.mem.set_u32(slot_vtable + APPLY_SLOT, SLOT_DOUBLE);
        e.mem.set_u32(body + 0xe0, slot_vtable);
        store_vec(&mut e, body + 0x160, [0.0, 0.0, 0.0, 1.0]);
        store_vec(&mut e, body + 0x1c0, [1.0, 2.0, 3.0, 4.0]);
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x18, body);
        store_vec(&mut e, this + 0x20, [0.0, 0.0, 1.0, 0.0]);
        store_vec(&mut e, this + 0x30, [1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, this + 0x40, reference);
        e.mem.set_f32(this + 0x50, 2.0);
        e.mem.set_f32(this + 0x54, 3.0);
        e.mem.set_u8(this + 0x5c, 1);
        (e, this, body, record)
    }

    fn step_info(e: &mut Engine, step: f32) -> u32 {
        let info = e.mem.alloc(0x10);
        e.mem.set_f32(info + 0xc, step);
        info
    }

    #[test]
    fn a_disabled_action_does_nothing() {
        let (mut e, this, _, _) = setup([0.0, 1.0, 0.0, 0.0]);
        e.mem.set_u8(this + 0x5c, 0);
        let info = step_info(&mut e, 0.5);
        e.call(0x00d2_ad50, &args![this, info]);
        assert!(calls_to(&e, PREPARE_BODY).is_empty());
        assert!(calls_to(&e, ACOS).is_empty());
        assert_eq!(e.mem.f32(this + 0x58), 0.0);
    }

    #[test]
    fn a_quarter_turn_pushes_along_the_hinge_and_against_the_spin() {
        let (mut e, this, body, record) = setup([0.0, 1.0, 0.0, 0.0]);
        let info = step_info(&mut e, 0.5);
        e.call(0x00d2_ad50, &args![this, info]);
        let angle = e.mem.f32(this + 0x58);
        assert!(
            (angle - std::f32::consts::FRAC_PI_2).abs() < 1e-3,
            "{angle}"
        );
        assert_eq!(calls_to(&e, PREPARE_BODY), [[body]]);
        let impulse = load_vec(&e, record);
        // 2 * angle * 0.5 along the hinge (z) minus 3 * 0.5 times the body vector.
        assert!((impulse[0] + 1.5).abs() < 1e-3, "{impulse:?}");
        assert!((impulse[1] + 3.0).abs() < 1e-3);
        assert!((impulse[2] - (angle - 4.5)).abs() < 1e-3);
        assert!((impulse[3] + 6.0).abs() < 1e-3);
    }

    #[test]
    fn the_opposite_turn_negates_the_angle() {
        let (mut e, this, _, record) = setup([0.0, -1.0, 0.0, 0.0]);
        let info = step_info(&mut e, 0.5);
        e.call(0x00d2_ad50, &args![this, info]);
        let angle = e.mem.f32(this + 0x58);
        assert!(
            (angle + std::f32::consts::FRAC_PI_2).abs() < 1e-3,
            "{angle}"
        );
        let impulse = load_vec(&e, record);
        assert!((impulse[2] - (angle - 4.5)).abs() < 1e-3, "{impulse:?}");
    }

    #[test]
    fn the_largest_lane_prefers_the_earlier_one_on_ties() {
        assert_eq!(largest_lane([1.0, 1.0, 1.0, 9.0]), 0);
        assert_eq!(largest_lane([1.0, 2.0, 2.0, 0.0]), 1);
        assert_eq!(largest_lane([1.0, 2.0, 3.0, 0.0]), 2);
        assert_eq!(largest_lane([3.0, 2.0, 3.0, 0.0]), 0);
        assert_eq!(largest_lane([3.0, 2.0, 4.0, 0.0]), 2);
        assert_eq!(largest_lane([f32::NAN, 2.0, 1.0, 0.0]), 0);
        assert_eq!(largest_lane([1.0, f32::NAN, 5.0, 0.0]), 2);
    }

    #[test]
    fn the_clone_builds_a_copy_only_for_the_expected_words() {
        let mut e = logging_engine();
        allocator_doubles(&mut e);
        e.register(ACTION_CONSTRUCT, nothing);
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 0x77);
        let this = e.mem.alloc(0x60);
        store_vec(&mut e, this + 0x20, [1.0, 2.0, 3.0, 4.0]);
        store_vec(&mut e, this + 0x30, [5.0, 6.0, 7.0, 8.0]);
        store_vec(&mut e, this + 0x40, [9.0, 10.0, 11.0, 12.0]);
        e.mem.set_f32(this + 0x50, 1.5);
        e.mem.set_f32(this + 0x54, 2.5);
        e.mem.set_u32(this + 0x10, 0x4444);
        let owner = e.mem.alloc(8);
        let inner = e.mem.alloc(8);
        e.mem.set_u32(owner, inner);
        e.mem.set_u32(inner, 0x3333);
        e.mem.set_u32(owner + 4, 2);
        let other = e.mem.alloc(8);
        let copy = e.call(0x00d2_b110, &args![this, owner, other]).u32();
        assert_ne!(copy, 0);
        assert_eq!(e.mem.u32(word), 0x77);
        assert_eq!(load_vec(&e, copy + 0x40), [9.0, 10.0, 11.0, 12.0]);
        assert_eq!(load_vec(&e, copy + 0x20), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vec(&e, copy + 0x30), [5.0, 6.0, 7.0, 8.0]);
        assert_eq!(e.mem.f32(copy + 0x50), 1.5);
        assert_eq!(e.mem.f32(copy + 0x54), 2.5);
        assert_eq!(calls_to(&e, ACTION_CONSTRUCT), [[copy, 0x3333, 0x4444]]);
        // A first word other than 2, or a second that is not 0, makes no copy.
        e.mem.set_u32(owner + 4, 3);
        assert_eq!(e.call(0x00d2_b110, &args![this, owner, other]).u32(), 0);
        e.mem.set_u32(owner + 4, 2);
        e.mem.set_u32(other + 4, 1);
        assert_eq!(e.call(0x00d2_b110, &args![this, owner, other]).u32(), 0);
        assert_eq!(e.mem.u32(word), 0x77);
    }
}
