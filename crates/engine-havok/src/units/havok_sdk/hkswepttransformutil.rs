//! `Havok SDK/hkswepttransformutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

//!
//! The functions here work on a `hkMotionState` (the transform at `+0`: three rotation columns
//! and the translation at `+0x30`, then the `hkSweptTransform`): the centre of mass at time 0
//! at `+0x40` (its `w` at `+0x4c` is that time) and at time 1 at `+0x50` (`w` at `+0x5c` is the
//! inverse time step), the rotation quaternions at `+0x60` and `+0x70`, the centre of mass in
//! local space at `+0x80` and a velocity vector at `+0x90`. The `lerp` functions take the
//! swept transform alone (the same record seen from `+0x40`: its fields are at `+0`, `+0x10`,
//! `+0x20`, `+0x30`, `+0x40`).
//!
//! The code mixes SSE (`f32`, four lanes, in the order of the instructions) with a short x87
//! sequence for the inverse-length refinement; x87 values are computed in `f64` and rounded
//! at the `FSTP` that stores a `float` (the helpers of the prelude). `RSQRTSS` is the host
//! CPU's estimate (`rsqrt_estimate`), refined once the way the code does it, so results can
//! differ from the exe's CPU in the last bit or two.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::x87::rsqrt_estimate;

/// `hkRotation::set(const hkQuaternion&)` (`00cb2d90`, thiscall on the rotation, one stack
/// word: the quaternion), `hkQuaternion::set(const hkRotation&)` (`00cb26e0`, thiscall on the
/// quaternion, one stack word: the rotation) and `hkVector4::setRotatedDir` (`00c7f700`,
/// thiscall on the result, two stack words `(rotation, vector)`).
const ROTATION_SET: u32 = 0x00cb_2d90;
const QUATERNION_SET: u32 = 0x00cb_26e0;
const SET_ROTATED_DIR: u32 = 0x00c7_f700;
/// The code's constants, read from the exe's data: the doubles `0.125`, `0.75`, `0.5`, `1.5`
/// and `1.0`, the floats `0.5`, `0.0`, `3.0` and `1.1920929e-7`, four floats of `1.0`, and
/// the lane mask `(-1, -1, -1, 0)` (as words).
const EIGHTH_F64: u32 = 0x0109_24a8;
const THREE_QUARTERS_F64: u32 = 0x0101_de30;
const HALF_F64: u32 = 0x0101_1588;
const THREE_HALVES_F64: u32 = 0x0101_6ff0;
const ONE_F64: u32 = 0x0101_2070;
const HALF_F32: u32 = 0x0101_6248;
const ZERO_F32: u32 = 0x0101_1d78;
const THREE_F32: u32 = 0x0101_7718;
const EPSILON_F32: u32 = 0x0104_da4c;
const ONES_F32X4: u32 = 0x010e_c7f0;
const XYZ_MASK: u32 = 0x010c_bfc0;

/// One SSE register: four `f32` lanes.
type Lanes = [f32; 4];

fn load_lanes(e: &Engine, address: u32) -> Lanes {
    std::array::from_fn(|lane| e.mem.f32(address + 4 * lane as u32))
}

fn store_lanes(e: &mut Engine, address: u32, value: Lanes) {
    for (lane, word) in value.iter().enumerate() {
        e.mem.set_f32(address + 4 * lane as u32, *word);
    }
}

fn splat(value: f32) -> Lanes {
    [value; 4]
}

fn lanes_add(a: Lanes, b: Lanes) -> Lanes {
    std::array::from_fn(|lane| a[lane] + b[lane])
}

fn lanes_sub(a: Lanes, b: Lanes) -> Lanes {
    std::array::from_fn(|lane| a[lane] - b[lane])
}

fn lanes_mul(a: Lanes, b: Lanes) -> Lanes {
    std::array::from_fn(|lane| a[lane] * b[lane])
}

/// `(y * c1 + x * c0) + z * c2` lane by lane: a vector rotated by the three columns, in the
/// order of the code's `SHUFPS`/`MULPS`/`ADDPS` sequence.
fn rotate(v: Lanes, c0: Lanes, c1: Lanes, c2: Lanes) -> Lanes {
    let mixed = lanes_add(lanes_mul(splat(v[1]), c1), lanes_mul(splat(v[0]), c0));
    lanes_add(mixed, lanes_mul(splat(v[2]), c2))
}

/// The squared length the code sums with two shuffle-and-add steps: with `s = v * v` it is
/// `(s1 + s3) + (s0 + s2)`, the same in every lane.
fn square_sum(v: Lanes) -> f32 {
    let s = lanes_mul(v, v);
    let first = [s[2] + s[0], s[3] + s[1]];
    first[1] + first[0]
}

/// The inverse-length refinement of the x87 sequence: `a = float(0.75 - 0.125 * L)`, then
/// `b = float((1.5 - ((0.5 * L) * a) * a) * a)`, with `L` the squared length. Returns `b`.
fn refinement(e: &Engine, length_squared: f32) -> f32 {
    let eighth: f64 = e.global(EIGHTH_F64);
    let three_quarters: f64 = e.global(THREE_QUARTERS_F64);
    let half: f64 = e.global(HALF_F64);
    let three_halves: f64 = e.global(THREE_HALVES_F64);
    let l = x87_load32(length_squared.to_bits());
    let a_bits = x87_store32(x87_sub(three_quarters, x87_mul(eighth, l)));
    let a = x87_load32(a_bits);
    let m = x87_mul(x87_mul(x87_mul(l, half), a), a);
    f32::from_bits(x87_store32(x87_mul(x87_sub(three_halves, m), a)))
}

/// `v` scaled to unit length the way the code does: `L = square_sum(v)`, `r = RSQRTSS(L)`,
/// `scale = (0.5 * r) * (3.0 - (L * r) * r)`, forced to 0 when `L` equals the float at
/// `01011d78` (`CMPEQSS`/`ANDNPS`; false for a NaN), then `scale * v`.
fn normalize(e: &Engine, v: Lanes) -> Lanes {
    let zero: f32 = e.global(ZERO_F32);
    let three: f32 = e.global(THREE_F32);
    let half: f32 = e.global(HALF_F32);
    let length_squared = square_sum(v);
    let r = rsqrt_estimate(length_squared);
    let mut scale = (half * r) * (three - (length_squared * r) * r);
    if length_squared == zero {
        scale = 0.0;
    }
    std::array::from_fn(|lane| scale * v[lane])
}

/// The vector `v` of `motion state + offset` rotated by the transform's columns (the rotation
/// at `transform`), the centre-of-mass offset the code adds or subtracts.
fn rotated_by_columns(e: &Engine, transform: u32, v: Lanes) -> Lanes {
    rotate(
        v,
        load_lanes(e, transform),
        load_lanes(e, transform + 0x10),
        load_lanes(e, transform + 0x20),
    )
}

// Translated from 00cf1480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSweptTransformUtil::setTimeInformation` (map name). cdecl, three
/// words: stores the two `float`s at `+0x4c` and `+0x5c` of `target`. Each
/// goes through an `FLD`/`FSTP` pair, which quiets a signalling NaN.
pub fn hk_swept_transform_util_set_time_information(
    e: &mut Engine,
    first: f32,
    second: f32,
    target: Ptr,
) {
    e.mem.set_f32(target.addr() + 0x4c, x87_quiet(first));
    e.mem.set_f32(target.addr() + 0x5c, x87_quiet(second));
}

// Translated from 00cf14a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSweptTransformUtil::setCentreOfRotationLocal` (map name), cdecl `(centre, motion
/// state)`: the difference between the new centre of mass in local space and the one stored
/// at `+0x80` (the new one is stored there) is rotated by the motion state's rotation
/// (`hkVector4::setRotatedDir`, `00c7f700`), masked with `(-1, -1, -1, 0)` (`ANDPS`, the
/// mask at `010cbfc0`) and added to both centres of mass (`+0x40` and `+0x50`, all four
/// lanes).
pub fn hk_swept_transform_util_set_centre_of_rotation_local(
    e: &mut Engine,
    centre: Ptr,
    motion_state: Ptr,
) {
    let state = motion_state.addr();
    let new_centre = load_lanes(e, centre.addr());
    let old_centre = load_lanes(e, state + 0x80);
    let delta = lanes_sub(new_centre, old_centre);
    e.with_stack(0x20, |e, vectors| {
        let delta_address = vectors.addr();
        let rotated_address = vectors.addr() + 0x10;
        store_lanes(e, delta_address, delta);
        store_lanes(e, state + 0x80, new_centre);
        e.call(
            SET_ROTATED_DIR,
            &args![rotated_address, motion_state, delta_address],
        );
        let rotated = load_lanes(e, rotated_address);
        let masked: Lanes = std::array::from_fn(|lane| {
            f32::from_bits(e.mem.u32(XYZ_MASK + 4 * lane as u32) & rotated[lane].to_bits())
        });
        let first = lanes_add(load_lanes(e, state + 0x40), masked);
        store_lanes(e, state + 0x40, first);
        let second = lanes_add(load_lanes(e, state + 0x50), masked);
        store_lanes(e, state + 0x50, second);
    });
}

/// What `lerp2`, `lerp2Ha` and `00cf1920` do once the interpolation factor `t` is known (as
/// the float whose bits are `t_bits`, which the exe keeps in a stack slot): the rotation
/// quaternions at `swept + 0x20` (`a`) and `+0x30` (`b`) are summed, the sum is scaled by the
/// x87 refinement of its length (`refinement`) and blended with a factor `u` (`2t` when `t` is
/// below 0.5, then `(1 - u) * a + u * scaled`, otherwise `2t - 1` and `(1 - u) * scaled + u *
/// b`; the comparison is the code's `FCOM` + `TEST AH,5` + `JP`: the second form also for
/// equality and for a NaN), normalized (`normalize`) and turned into the rotation of `out`
/// (`hkRotation::set`, `00cb2d90`). The translation of `out` (`+0x30`) is the linear blend
/// `(1 - t) * c0 + t * c1` of the two centres of mass (`swept + 0` and `+ 0x10`), minus the
/// local centre of mass (`swept + 0x40`) rotated by the new rotation.
fn interpolate(e: &mut Engine, swept: u32, t_bits: u32, out: u32) {
    let ones = load_lanes(e, ONES_F32X4);
    let rotation0 = load_lanes(e, swept + 0x20);
    let rotation1 = load_lanes(e, swept + 0x30);
    let sum = lanes_add(rotation1, rotation0);
    let scale = refinement(e, square_sum(sum));
    let scaled = lanes_mul(splat(scale), sum);
    let t = x87_load32(t_bits);
    let half = x87_load32(e.global::<f32>(HALF_F32).to_bits());
    let doubled = x87_add(t, t);
    let blended = if t < half {
        let u = f32::from_bits(x87_store32(doubled));
        let first = lanes_mul(lanes_sub(ones, splat(u)), rotation0);
        let second = lanes_mul(splat(u), scaled);
        lanes_add(first, second)
    } else {
        let one: f64 = e.global(ONE_F64);
        let u = f32::from_bits(x87_store32(x87_sub(doubled, one)));
        let first = lanes_mul(lanes_sub(ones, splat(u)), scaled);
        let second = lanes_mul(splat(u), rotation1);
        lanes_add(first, second)
    };
    let rotation = normalize(e, blended);
    e.with_stack(0x10, |e, quaternion| {
        store_lanes(e, quaternion.addr(), rotation);
        e.call(ROTATION_SET, &args![out, quaternion]);
    });
    let weight = splat(f32::from_bits(t_bits));
    let first = lanes_mul(lanes_sub(ones, weight), load_lanes(e, swept));
    let second = lanes_mul(load_lanes(e, swept + 0x10), weight);
    store_lanes(e, out + 0x30, lanes_add(first, second));
    let centre = load_lanes(e, swept + 0x40);
    let rotated = rotated_by_columns(e, out, centre);
    let translation = load_lanes(e, out + 0x30);
    store_lanes(e, out + 0x30, lanes_sub(translation, rotated));
}

// Translated from 00cf1510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSweptTransformUtil::lerp2` (map name), cdecl `(swept transform, time, result
/// transform)`: the transform at `time`. The factor is `float((time - t0) * invDeltaTime)`
/// with `t0` the word at `+0xc` and `invDeltaTime` the one at `+0x1c` (x87, one rounding at
/// the store), then `interpolate`.
pub fn hk_swept_transform_util_lerp2(e: &mut Engine, swept: Ptr, time: f32, out: Ptr) {
    let start = e.mem.u32(swept.addr() + 0xc);
    let inverse_step = e.mem.u32(swept.addr() + 0x1c);
    let t = x87_mul(
        x87_sub(x87_load32(time.to_bits()), x87_m32(start)),
        x87_m32(inverse_step),
    );
    interpolate(e, swept.addr(), x87_store32(t), out.addr());
}

// Translated from 00cf1710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSweptTransformUtil::lerp2Ha` (map name), cdecl `(swept transform, time, extra, result
/// transform)`: as `lerp2` but the factor is `((time - t0) + extra) * invDeltaTime` with the
/// float stored (rounded) after the subtraction and again after the addition, then
/// `interpolate`.
pub fn hk_swept_transform_util_lerp2_ha(
    e: &mut Engine,
    swept: Ptr,
    time: f32,
    extra: f32,
    out: Ptr,
) {
    let start = e.mem.u32(swept.addr() + 0xc);
    let inverse_step = e.mem.u32(swept.addr() + 0x1c);
    let elapsed = x87_store32(x87_sub(x87_load32(time.to_bits()), x87_m32(start)));
    let shifted = x87_store32(x87_add(x87_load32(elapsed), x87_m32(extra.to_bits())));
    let t = x87_mul(x87_load32(shifted), x87_m32(inverse_step));
    interpolate(e, swept.addr(), x87_store32(t), out.addr());
}

// Translated from 00cf1920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`lerp2` for a factor already known), cdecl `(swept transform,
/// factor, result transform)`: `interpolate` with the factor argument as the interpolation
/// factor.
pub fn fn_00cf1920(e: &mut Engine, swept: Ptr, factor: f32, out: Ptr) {
    interpolate(e, swept.addr(), factor.to_bits(), out.addr());
}

// Translated from 00cf1b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSweptTransformUtil::backStepMotionState` (map name), cdecl `(time, motion state)`:
/// moves the motion state's swept transform back to `time`.
///
/// The factor is `t = float((time - m[0x4c]) * m[0x5c])` (x87), replaced by the float
/// epsilon (`0104da4c`) unless it is above it (the code's `FCOM` + `TEST AH,0x41` +
/// `JNZ`: a NaN gives the epsilon). The two rotation quaternions (`+0x60`, `+0x70`) are
/// summed and the sum scaled as in `interpolate`; the second quaternion (`+0x70`) becomes
/// the blend of the first with that sum (`u = 2t` below 0.5, otherwise `2t - 1` with the old
/// second quaternion) normalized. The word at `+0x5c` becomes `float(old / t)` (x87 divide,
/// the old word quieted by an `FLD`/`FSTP`); the second centre of mass (`+0x50`) becomes
/// `(1 - t) * c0 + t * c1` (before that division is stored); the vector at `+0x90` is
/// multiplied by `t`; the rotation of the state is rebuilt from the quaternion (`00cb2d90`)
/// and the translation is the second centre of mass minus the local centre of mass (`+0x80`)
/// rotated by it.
pub fn hk_swept_transform_util_back_step_motion_state(
    e: &mut Engine,
    time: f32,
    motion_state: Ptr,
) {
    let state = motion_state.addr();
    let start = e.mem.u32(state + 0x4c);
    let inverse_step = e.mem.u32(state + 0x5c);
    let raw = x87_mul(
        x87_sub(x87_load32(time.to_bits()), x87_m32(start)),
        x87_m32(inverse_step),
    );
    let raw_stored = x87_load32(x87_store32(raw));
    let epsilon = x87_load32(e.global::<f32>(EPSILON_F32).to_bits());
    let chosen = if raw_stored > epsilon {
        raw_stored
    } else {
        epsilon
    };
    let t_bits = x87_store32(chosen);
    let t = x87_load32(t_bits);

    let ones = load_lanes(e, ONES_F32X4);
    let rotation0 = load_lanes(e, state + 0x60);
    let rotation1 = load_lanes(e, state + 0x70);
    let sum = lanes_add(rotation1, rotation0);
    let scale = refinement(e, square_sum(sum));
    let scaled = lanes_mul(splat(scale), sum);
    let half = x87_load32(e.global::<f32>(HALF_F32).to_bits());
    let doubled = x87_add(t, t);
    let blended = if t < half {
        let u = f32::from_bits(x87_store32(doubled));
        let first = lanes_mul(lanes_sub(ones, splat(u)), rotation0);
        let second = lanes_mul(scaled, splat(u));
        lanes_add(first, second)
    } else {
        let one: f64 = e.global(ONE_F64);
        let u = f32::from_bits(x87_store32(x87_sub(doubled, one)));
        let first = lanes_mul(lanes_sub(ones, splat(u)), scaled);
        let second = lanes_mul(splat(u), rotation1);
        lanes_add(first, second)
    };
    let rotation = normalize(e, blended);
    store_lanes(e, state + 0x70, rotation);

    let old_inverse_step = x87_quiet_bits(e.mem.u32(state + 0x5c));
    let divided = x87_div(x87_m32(old_inverse_step), t);
    let weight = splat(f32::from_bits(t_bits));
    let centre0 = load_lanes(e, state + 0x40);
    let centre1 = load_lanes(e, state + 0x50);
    let blended_centre = lanes_add(
        lanes_mul(lanes_sub(ones, weight), centre0),
        lanes_mul(centre1, weight),
    );
    store_lanes(e, state + 0x50, blended_centre);
    e.mem.set_u32(state + 0x5c, x87_store32(divided));
    let velocity = load_lanes(e, state + 0x90);
    store_lanes(e, state + 0x90, lanes_mul(weight, velocity));
    e.call(ROTATION_SET, &args![motion_state, state + 0x70]);
    let centre_in_object_space = load_lanes(e, state + 0x80);
    let rotated = rotated_by_columns(e, state, centre_in_object_space);
    let centre = load_lanes(e, state + 0x50);
    store_lanes(e, state + 0x30, lanes_sub(centre, rotated));
}

/// The common end of the `warpTo` functions: both centres of mass become `centre`, the
/// local centre of mass rotated by the columns at `transform` plus `translation`; the time
/// words keep their `FLD`/`FSTP` round trip (a signalling NaN gets quieted) at `+0x4c` and
/// the inverse time step at `+0x5c` becomes 0.
fn set_centres_of_mass(e: &mut Engine, state: u32, transform: u32, translation: Lanes) {
    let time = x87_quiet_bits(e.mem.u32(state + 0x4c));
    let local = load_lanes(e, state + 0x80);
    let rotated = rotated_by_columns(e, transform, local);
    let centre = lanes_add(rotated, translation);
    store_lanes(e, state + 0x40, centre);
    store_lanes(e, state + 0x50, centre);
    e.mem.set_u32(state + 0x4c, time);
    e.mem.set_f32(state + 0x5c, 0.0);
}

/// [`set_centres_of_mass`] with the SSE NaN rules: `(y * c1 + x * c0) + z * c2`, each `MULPS`
/// with the splatted lane as the destination, then `ADDPS` of the translation.
fn set_centres_of_mass_sse(e: &mut Engine, state: u32, transform: u32, translation: Lanes) {
    use crate::sse::{addps, mulps};
    let time = x87_quiet_bits(e.mem.u32(state + 0x4c));
    let v = load_lanes(e, state + 0x80);
    let c0 = load_lanes(e, transform);
    let c1 = load_lanes(e, transform + 0x10);
    let c2 = load_lanes(e, transform + 0x20);
    let mixed = addps(mulps(splat(v[1]), c1), mulps(splat(v[0]), c0));
    let rotated = addps(mixed, mulps(splat(v[2]), c2));
    let centre = addps(rotated, translation);
    store_lanes(e, state + 0x40, centre);
    store_lanes(e, state + 0x50, centre);
    e.mem.set_u32(state + 0x4c, time);
    e.mem.set_f32(state + 0x5c, 0.0);
}

// Translated from 00cf1f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSweptTransformUtil::warpTo` (map name), cdecl `(position, rotation quaternion, motion
/// state)`: clears the vector at `+0x90`, stores the quaternion as both swept rotations
/// (`+0x60`, `+0x70`) and rebuilds the state's rotation from it (`00cb2d90`), stores
/// the position as the translation (`+0x30`), then both centres of mass become the local
/// centre of mass rotated by the new rotation plus the position (`+0x40`, `+0x50`, with
/// the time word at `+0x4c` quieted and kept and the word at `+0x5c` set to 0).
pub fn hk_swept_transform_util_warp_to(
    e: &mut Engine,
    position: Ptr,
    rotation: Ptr,
    motion_state: Ptr,
) {
    let state = motion_state.addr();
    store_lanes(e, state + 0x90, splat(0.0));
    let quaternion = load_lanes(e, rotation.addr());
    store_lanes(e, state + 0x60, quaternion);
    let quaternion = load_lanes(e, rotation.addr());
    store_lanes(e, state + 0x70, quaternion);
    e.call(ROTATION_SET, &args![motion_state, rotation]);
    let position_lanes = load_lanes(e, position.addr());
    store_lanes(e, state + 0x30, position_lanes);
    let position_lanes = load_lanes(e, position.addr());
    set_centres_of_mass(e, state, state, position_lanes);
}

// Translated from 00cf2020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSweptTransformUtil::warpTo` overload 2 (map name), cdecl `(transform, motion state)`:
/// clears the vector at `+0x90`, makes the quaternion of the transform's rotation
/// (`hkQuaternion::set`, `00cb26e0`), copies the transform's four columns into the state,
/// stores the quaternion as both swept rotations, and sets both centres of mass to the local
/// centre of mass rotated by the transform's columns plus its translation (the time
/// word at `+0x4c` quieted and kept, the word at `+0x5c` set to 0).
pub fn hk_swept_transform_util_warp_to_ov2(e: &mut Engine, transform: Ptr, motion_state: Ptr) {
    let state = motion_state.addr();
    let source = transform.addr();
    store_lanes(e, state + 0x90, splat(0.0));
    let quaternion = e.with_stack(0x10, |e, quaternion| {
        e.call(QUATERNION_SET, &args![quaternion, transform]);
        load_lanes(e, quaternion.addr())
    });
    for column in 0..4u32 {
        let lanes = load_lanes(e, source + 0x10 * column);
        store_lanes(e, state + 0x10 * column, lanes);
    }
    store_lanes(e, state + 0x60, quaternion);
    store_lanes(e, state + 0x70, quaternion);
    let translation = load_lanes(e, source + 0x30);
    set_centres_of_mass(e, state, source, translation);
}

// Translated from 00cf20d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSweptTransformUtil::warpToPosition` (map name), cdecl `(position, motion state)`:
/// clears the vector at `+0x90`, stores the position as the translation (`+0x30`), sets
/// both centres of mass to the local centre of mass rotated by the state's rotation plus the
/// position (the time word at `+0x4c` quieted and kept, the word at `+0x5c` 0), and copies
/// the second swept rotation (`+0x70`) over the first (`+0x60`).
pub fn hk_swept_transform_util_warp_to_position(e: &mut Engine, position: Ptr, motion_state: Ptr) {
    let state = motion_state.addr();
    store_lanes(e, state + 0x90, splat(0.0));
    let position_lanes = load_lanes(e, position.addr());
    store_lanes(e, state + 0x30, position_lanes);
    let position_lanes = load_lanes(e, position.addr());
    set_centres_of_mass_sse(e, state, state, position_lanes);
    let rotation = load_lanes(e, state + 0x70);
    store_lanes(e, state + 0x60, rotation);
}

// Translated from 00cf2160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSweptTransformUtil::warpToRotation` (map name), cdecl `(rotation quaternion, motion
/// state)`: `warpTo(state + 0x30, rotation, state)`: warps to the rotation at the state's own
/// translation.
pub fn hk_swept_transform_util_warp_to_rotation(e: &mut Engine, rotation: Ptr, motion_state: Ptr) {
    hk_swept_transform_util_warp_to(
        e,
        Ptr::new(motion_state.addr().wrapping_add(0x30)),
        rotation,
        motion_state,
    );
}

// Translated from 00cf1d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSweptTransformUtil::freezeMotionState` (Xbox PDB name from the engine map): cdecl, two stack
/// words (nothing popped): the float `time` and `state`, a motion state (transform: three columns
/// at +0, +0x10, +0x20 and the translation at +0x30; swept transform: centres of mass at +0x40
/// and +0x50 whose fourth lanes at +0x4c and +0x5c are the start time and the inverse interval,
/// rotations at +0x60 and +0x70, the local centre at +0x80). The x87 steps compute in extended
/// precision and every store to memory is rounded to a float, as the exe does.
///
/// `T` is `time`, or the start time at +0x4c when that is not below `time` (an unordered compare
/// counts as not below). The blend factor is `t = (T - start) * inverse interval`, rounded to a
/// float. The sum `s` of the two rotations (`+0x70` plus `+0x60`, lane by lane) is scaled by
/// `g = L * (c_d - ((len2 * c_c) * L) * L)` with `L = c_b - c_a * len2` (each rounded to a float
/// where the exe stores it; `len2` the squared length of `s`; the doubles `c_a` at `010924a8`,
/// `c_b` at `0101de30`, `c_c` at `01011588` and `c_d` at `01016ff0`). With `u = 2 * t`: when `t` is
/// below the float `0.5` at `01016248` the rotation becomes `(1 - u) * (+0x60) + (g * s) * u`,
/// otherwise `(1 - (u - 1.0)) * (g * s) + (u - 1.0) * (the old +0x70)` (the double 1.0 at
/// `01012070`), the vector of four 1.0 being `010ec7f0`. It is normalised with the `RSQRTSS`
/// refinement of `inverse_length` and stored at both +0x60 and +0x70. The centre at +0x40
/// becomes `(1 - t) * (+0x40) + t * (+0x50)`, with the start time `T` stored at +0x4c; the
/// centre at +0x50 becomes that same vector with 0.0 at +0x5c. Then `00cb2d90` rebuilds the three
/// columns of the transform from the quaternion at +0x70, and the translation at +0x30 is the
/// centre at +0x50 minus the three columns weighted by the first three lanes of +0x80
/// (`(y * column1 + x * column0) + z * column2`).
pub fn hk_swept_transform_util_freeze_motion_state(e: &mut Engine, time: f32, state: u32) {
    use crate::units::havok_sdk::hkpsphereshape::{
        inverse_length, load_vec, shufps, store_vec, Vec4,
    };

    // The start time in the float at +0x4c decides what `T` is.
    let start_bits = e.mem.u32(state + 0x4c);
    let start = x87_load32(x87_quiet_bits(start_bits));
    let requested = x87_load32(time.to_bits());
    let time_value = if start < requested {
        requested
    } else {
        x87_load32(x87_store32(start))
    };
    let elapsed = x87_sub(time_value, x87_m32(e.mem.u32(state + 0x4c)));
    let factor_bits = x87_store32(x87_mul(elapsed, x87_m32(e.mem.u32(state + 0x5c))));

    // The two rotations added, and the squared length of the sum.
    let old_rotation = load_vec(e, state + 0x70);
    let first_rotation = load_vec(e, state + 0x60);
    let sum: Vec4 = std::array::from_fn(|lane| old_rotation[lane] + first_rotation[lane]);
    let squares: Vec4 = std::array::from_fn(|lane| sum[lane] * sum[lane]);
    let swapped = shufps(squares, squares, 0x4e);
    let pairs: Vec4 = std::array::from_fn(|lane| swapped[lane] + squares[lane]);
    let swapped = shufps(pairs, pairs, 0xb1);
    let length_squared = swapped[0] + pairs[0];
    store_vec(e, state + 0x70, sum);

    // The scale of the sum (x87 steps with float stores in between).
    let constant_a: f64 = e.global(0x0109_24a8);
    let constant_b: f64 = e.global(0x0101_de30);
    let constant_c: f64 = e.global(0x0101_1588);
    let constant_d: f64 = e.global(0x0101_6ff0);
    let length_loaded = x87_load32(length_squared.to_bits());
    let guess_bits = x87_store32(x87_sub(constant_b, x87_mul(constant_a, length_loaded)));
    let half_length = x87_mul(length_loaded, constant_c);
    let guess = x87_load32(guess_bits);
    let cubed = x87_mul(x87_mul(half_length, guess), guess);
    let scale_bits = x87_store32(x87_mul(x87_sub(constant_d, cubed), guess));
    let scale = f32::from_bits(scale_bits);
    let scaled: Vec4 = std::array::from_fn(|lane| scale * sum[lane]);

    // The blend of the rotation.
    let half = x87_load32(e.global::<u32>(0x0101_6248));
    let factor = x87_load32(factor_bits);
    store_vec(e, state + 0x70, scaled);
    let doubled = x87_add(factor, factor);
    let ones = load_vec(e, 0x010e_c7f0);
    let blended: Vec4 = if factor < half {
        let weight = f32::from_bits(x87_store32(doubled));
        std::array::from_fn(|lane| {
            (ones[lane] - weight) * first_rotation[lane] + scaled[lane] * weight
        })
    } else {
        let one: f64 = e.global(0x0101_2070);
        let weight = f32::from_bits(x87_store32(x87_sub(doubled, one)));
        std::array::from_fn(|lane| {
            (ones[lane] - weight) * scaled[lane] + weight * old_rotation[lane]
        })
    };
    store_vec(e, state + 0x70, blended);

    // Normalise it and keep it as both rotations.
    let rotation = load_vec(e, state + 0x70);
    let squares: Vec4 = std::array::from_fn(|lane| rotation[lane] * rotation[lane]);
    let swapped = shufps(squares, squares, 0x4e);
    let pairs: Vec4 = std::array::from_fn(|lane| swapped[lane] + squares[lane]);
    let swapped = shufps(pairs, pairs, 0xb1);
    let inverse = inverse_length(e, swapped[0] + pairs[0]);
    let normalised: Vec4 = std::array::from_fn(|lane| inverse * rotation[lane]);
    store_vec(e, state + 0x70, normalised);
    let centre_start = load_vec(e, state + 0x40);
    store_vec(e, state + 0x60, normalised);

    // The centres of mass.
    let weight = f32::from_bits(factor_bits);
    let centre_end = load_vec(e, state + 0x50);
    let centre: Vec4 = std::array::from_fn(|lane| {
        (ones[lane] - weight) * centre_start[lane] + weight * centre_end[lane]
    });
    store_vec(e, state + 0x40, centre);
    e.mem.set_u32(state + 0x4c, x87_store32(time_value));
    store_vec(e, state + 0x50, centre);
    e.mem.set_u32(state + 0x5c, 0);

    // The transform: columns from the quaternion, then the translation.
    e.call(0x00cb_2d90, &args![state, state + 0x70]);
    let local = load_vec(e, state + 0x80);
    let column0 = load_vec(e, state);
    let column1 = load_vec(e, state + 0x10);
    let column2 = load_vec(e, state + 0x20);
    let end = load_vec(e, state + 0x50);
    let moved: Vec4 = std::array::from_fn(|lane| {
        end[lane]
            - ((local[1] * column1[lane] + local[0] * column0[lane]) + local[2] * column2[lane])
    });
    store_vec(e, state + 0x30, moved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cf1480,
            hk_swept_transform_util_set_time_information(f32, f32, Ptr)
        ),
        entry!(
            0x00cf_14a0,
            hk_swept_transform_util_set_centre_of_rotation_local(Ptr, Ptr)
        ),
        entry!(0x00cf_1510, hk_swept_transform_util_lerp2(Ptr, f32, Ptr)),
        entry!(
            0x00cf_1710,
            hk_swept_transform_util_lerp2_ha(Ptr, f32, f32, Ptr)
        ),
        entry!(0x00cf_1920, fn_00cf1920(Ptr, f32, Ptr)),
        entry!(
            0x00cf_1b10,
            hk_swept_transform_util_back_step_motion_state(f32, Ptr)
        ),
        entry!(0x00cf_1f80, hk_swept_transform_util_warp_to(Ptr, Ptr, Ptr)),
        entry!(0x00cf_2020, hk_swept_transform_util_warp_to_ov2(Ptr, Ptr)),
        entry!(
            0x00cf_20d0,
            hk_swept_transform_util_warp_to_position(Ptr, Ptr)
        ),
        entry!(
            0x00cf_2160,
            hk_swept_transform_util_warp_to_rotation(Ptr, Ptr)
        ),
        entry!(
            0x00cf_1d50,
            hk_swept_transform_util_freeze_motion_state(f32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hk_swept_transform_util_set_time_information_stores_both_floats_quieted() {
        let mut e = Engine::new();
        let target = e.mem.alloc(0x80);
        e.call(
            0x00cf_1480,
            &args![1.5f32, f32::from_bits(0x7f80_0001), Ptr::<()>::new(target)],
        );
        assert_eq!(e.mem.f32(target + 0x4c), 1.5);
        assert_eq!(e.mem.u32(target + 0x5c), 0x7fc0_0001);
    }
}

#[cfg(test)]
mod tests_swept {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Quaternions = Rc<RefCell<Vec<Lanes>>>;

    /// An engine with the exe's constants in memory.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(EIGHTH_F64, 0.125f64);
        e.set_global(THREE_QUARTERS_F64, 0.75f64);
        e.set_global(HALF_F64, 0.5f64);
        e.set_global(THREE_HALVES_F64, 1.5f64);
        e.set_global(ONE_F64, 1.0f64);
        e.set_global(HALF_F32, 0.5f32);
        e.set_global(ZERO_F32, 0.0f32);
        e.set_global(THREE_F32, 3.0f32);
        e.set_global(EPSILON_F32, 1.192_092_9e-7f32);
        for lane in 0..4 {
            e.set_global(ONES_F32X4 + 4 * lane, 1.0f32);
            e.set_global(XYZ_MASK + 4 * lane, if lane < 3 { u32::MAX } else { 0 });
        }
        e
    }

    /// `hkRotation::set` double: gives the rotation these three columns and logs the
    /// quaternion it was asked to convert.
    fn install_rotation_set(e: &mut Engine, columns: [Lanes; 3]) -> Quaternions {
        let log: Quaternions = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        e.register_double(ROTATION_SET, move |e, a| {
            seen.borrow_mut().push(load_lanes(e, a[1]));
            for (index, column) in columns.iter().enumerate() {
                store_lanes(e, a[0] + 0x10 * index as u32, *column);
            }
            Ret::default()
        });
        log
    }

    const IDENTITY: [Lanes; 3] = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
    ];

    fn record(e: &mut Engine, size: u32, fields: &[(u32, Lanes)]) -> u32 {
        let at = e.mem.alloc(size);
        for (offset, lanes) in fields {
            store_lanes(e, at + offset, *lanes);
        }
        at
    }

    /// The decompiled arithmetic of the rotation blend in `f64`, with the exact inverse length
    /// where the exe has an estimate: `rotation0`, `rotation1`, the factor `t`; returns the
    /// normalized quaternion (the second blend form uses the scaled sum in its first term).
    fn reference_rotation(r0: Lanes, r1: Lanes, t: f64) -> [f64; 4] {
        let r0 = r0.map(f64::from);
        let r1 = r1.map(f64::from);
        let sum: Vec<f64> = (0..4).map(|i| r1[i] + r0[i]).collect();
        let length_squared: f64 = sum.iter().map(|x| x * x).sum();
        let a = 0.75 - length_squared * 0.125;
        let b = a * (1.5 - a * a * length_squared * 0.5);
        let doubled = t + t;
        let blended: Vec<f64> = if t < 0.5 {
            let u = doubled;
            (0..4).map(|i| (1.0 - u) * r0[i] + u * b * sum[i]).collect()
        } else {
            let u = doubled - 1.0;
            (0..4).map(|i| (1.0 - u) * b * sum[i] + u * r1[i]).collect()
        };
        let length = blended.iter().map(|x| x * x).sum::<f64>().sqrt();
        std::array::from_fn(|i| blended[i] / length)
    }

    fn assert_close(got: Lanes, want: [f64; 4]) {
        for lane in 0..4 {
            assert!(
                (f64::from(got[lane]) - want[lane]).abs() < 1e-5,
                "lane {lane}: {got:?} vs {want:?}"
            );
        }
    }

    /// A swept transform: centres of mass (`w` of the first is the start time, `w` of the
    /// second the inverse time step), two rotations, the local centre of mass.
    fn swept(e: &mut Engine, start: f32, inverse_step: f32) -> u32 {
        record(
            e,
            0x50,
            &[
                (0x00, [1.0, 2.0, 3.0, start]),
                (0x10, [5.0, 6.0, 7.0, inverse_step]),
                (0x20, [0.0, 0.0, 0.0, 1.0]),
                (0x30, [0.0, 0.6, 0.0, 0.8]),
                (0x40, [0.5, 0.25, 1.0, 0.0]),
            ],
        )
    }

    #[test]
    fn lerp2_below_one_half_blends_the_first_rotation_with_the_scaled_sum() {
        let mut e = engine();
        let log = install_rotation_set(&mut e, IDENTITY);
        let this = swept(&mut e, 1.0, 0.5);
        let out = e.mem.alloc(0x40);
        e.call(0x00cf_1510, &args![this, 1.5f32, out]);
        let quaternions = log.borrow();
        assert_eq!(quaternions.len(), 1);
        assert_close(
            quaternions[0],
            reference_rotation([0.0, 0.0, 0.0, 1.0], [0.0, 0.6, 0.0, 0.8], 0.25),
        );
        // t = 0.25: 0.75 * c0 + 0.25 * c1 = (2, 3, 4, 0.875), minus the identity-rotated
        // local centre (0.5, 0.25, 1, 0).
        assert_eq!(load_lanes(&e, out + 0x30), [1.5, 2.75, 3.0, 0.875]);
    }

    #[test]
    fn lerp2_from_one_half_blends_the_scaled_sum_with_the_second_rotation() {
        for (time, t) in [(2.5f32, 0.75f64), (2.0, 0.5)] {
            let mut e = engine();
            let log = install_rotation_set(&mut e, IDENTITY);
            let this = swept(&mut e, 1.0, 0.5);
            let out = e.mem.alloc(0x40);
            e.call(0x00cf_1510, &args![this, time, out]);
            assert_close(
                log.borrow()[0],
                reference_rotation([0.0, 0.0, 0.0, 1.0], [0.0, 0.6, 0.0, 0.8], t),
            );
            let t = t as f32;
            let want = [
                (1.0 - t) * 1.0 + t * 5.0 - 0.5,
                (1.0 - t) * 2.0 + t * 6.0 - 0.25,
                (1.0 - t) * 3.0 + t * 7.0 - 1.0,
                (1.0 - t) * 1.0 + t * 0.5,
            ];
            assert_eq!(load_lanes(&e, out + 0x30), want);
        }
    }

    #[test]
    fn lerp2_rotates_the_centre_in_object_space_by_the_new_rotation() {
        let mut e = engine();
        // 90 degrees about z: x -> y, y -> -x.
        install_rotation_set(
            &mut e,
            [
                [0.0, 1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
            ],
        );
        let this = swept(&mut e, 1.0, 0.5);
        let out = e.mem.alloc(0x40);
        e.call(0x00cf_1510, &args![this, 1.5f32, out]);
        // (2, 3, 4, 0.875) - R * (0.5, 0.25, 1): R * v = 0.25 * (-1, 0, 0) + 0.5 * (0, 1, 0)
        // + 1 * (0, 0, 1) = (-0.25, 0.5, 1).
        assert_eq!(load_lanes(&e, out + 0x30), [2.25, 2.5, 3.0, 0.875]);
    }

    #[test]
    fn lerp2_ha_adds_the_extra_time_to_the_elapsed_time() {
        let mut first = engine();
        let log_a = install_rotation_set(&mut first, IDENTITY);
        let this = swept(&mut first, 1.0, 0.5);
        let out_a = first.mem.alloc(0x40);
        first.call(0x00cf_1710, &args![this, 1.5f32, 0.5f32, out_a]);
        let mut second = engine();
        let log_b = install_rotation_set(&mut second, IDENTITY);
        let that = swept(&mut second, 1.0, 0.5);
        let out_b = second.mem.alloc(0x40);
        second.call(0x00cf_1510, &args![that, 2.0f32, out_b]);
        assert_eq!(*log_a.borrow(), *log_b.borrow());
        assert_eq!(
            load_lanes(&first, out_a + 0x30),
            load_lanes(&second, out_b + 0x30)
        );
    }

    #[test]
    fn fn_00cf1920_uses_the_factor_argument_directly() {
        let mut first = engine();
        let log_a = install_rotation_set(&mut first, IDENTITY);
        let this = swept(&mut first, 1.0, 0.5);
        let out_a = first.mem.alloc(0x40);
        first.call(0x00cf_1920, &args![this, 0.25f32, out_a]);
        let mut second = engine();
        let log_b = install_rotation_set(&mut second, IDENTITY);
        let that = swept(&mut second, 1.0, 0.5);
        let out_b = second.mem.alloc(0x40);
        second.call(0x00cf_1510, &args![that, 1.5f32, out_b]);
        assert_eq!(*log_a.borrow(), *log_b.borrow());
        assert_eq!(
            load_lanes(&first, out_a + 0x30),
            load_lanes(&second, out_b + 0x30)
        );
        // A factor from one half takes the second form (also a NaN factor).
        let mut third = engine();
        let log_c = install_rotation_set(&mut third, IDENTITY);
        let other = swept(&mut third, 1.0, 0.5);
        let out_c = third.mem.alloc(0x40);
        third.call(0x00cf_1920, &args![other, 0.5f32, out_c]);
        assert_close(
            log_c.borrow()[0],
            reference_rotation([0.0, 0.0, 0.0, 1.0], [0.0, 0.6, 0.0, 0.8], 0.5),
        );
        let mut fourth = engine();
        let log_d = install_rotation_set(&mut fourth, IDENTITY);
        let nan = swept(&mut fourth, 1.0, 0.5);
        let out_d = fourth.mem.alloc(0x40);
        fourth.call(0x00cf_1920, &args![nan, f32::NAN, out_d]);
        assert_eq!(log_d.borrow().len(), 1);
    }

    /// A motion state: columns/translation untouched, centres of mass, rotations, local
    /// centre, velocity.
    fn motion_state(e: &mut Engine) -> u32 {
        record(
            e,
            0xa0,
            &[
                (0x40, [1.0, 2.0, 3.0, 1.0]),
                (0x50, [5.0, 6.0, 7.0, 0.5]),
                (0x60, [0.0, 0.0, 0.0, 1.0]),
                (0x70, [0.0, 0.6, 0.0, 0.8]),
                (0x80, [1.0, 1.0, 1.0, 0.0]),
                (0x90, [4.0, 8.0, 12.0, 16.0]),
            ],
        )
    }

    #[test]
    fn back_step_moves_the_swept_transform_back_to_the_time() {
        let mut e = engine();
        let log = install_rotation_set(&mut e, IDENTITY);
        let state = motion_state(&mut e);
        e.call(0x00cf_1b10, &args![1.5f32, state]);
        // t = (1.5 - 1.0) * 0.5 = 0.25.
        let quaternions = log.borrow();
        assert_eq!(quaternions.len(), 1);
        assert_close(
            quaternions[0],
            reference_rotation([0.0, 0.0, 0.0, 1.0], [0.0, 0.6, 0.0, 0.8], 0.25),
        );
        // The second quaternion in the state is the one handed to hkRotation::set.
        assert_eq!(load_lanes(&e, state + 0x70), quaternions[0]);
        // The inverse time step is 0.5 / 0.25.
        assert_eq!(e.mem.f32(state + 0x5c), 2.0);
        // The second centre of mass: 0.75 * c0 + 0.25 * c1 in x, y, z.
        assert_eq!(load_lanes(&e, state + 0x50)[..3], [2.0, 3.0, 4.0]);
        // The velocity is scaled by t.
        assert_eq!(load_lanes(&e, state + 0x90), [1.0, 2.0, 3.0, 4.0]);
        // Translation = new centre of mass (with the new w) minus the rotated local centre.
        assert_eq!(load_lanes(&e, state + 0x30), [1.0, 2.0, 3.0, 2.0]);
    }

    #[test]
    fn back_step_takes_the_second_form_from_one_half() {
        let mut e = engine();
        let log = install_rotation_set(&mut e, IDENTITY);
        let state = motion_state(&mut e);
        // t = (2.5 - 1.0) * 0.5 = 0.75.
        e.call(0x00cf_1b10, &args![2.5f32, state]);
        assert_close(
            log.borrow()[0],
            reference_rotation([0.0, 0.0, 0.0, 1.0], [0.0, 0.6, 0.0, 0.8], 0.75),
        );
        assert_eq!(e.mem.f32(state + 0x5c), 0.5 / 0.75);
        assert_eq!(load_lanes(&e, state + 0x90), [3.0, 6.0, 9.0, 12.0]);
    }

    #[test]
    fn back_step_never_uses_a_factor_at_or_below_the_epsilon() {
        let epsilon = 1.192_092_9e-7f32;
        // Before the start time (negative factor), at the epsilon's own time, and a NaN time.
        for time in [0.5f32, 1.0, f32::NAN] {
            let mut e = engine();
            install_rotation_set(&mut e, IDENTITY);
            let state = motion_state(&mut e);
            e.call(0x00cf_1b10, &args![time, state]);
            assert_eq!(e.mem.f32(state + 0x5c), 0.5 / epsilon);
            let velocity = load_lanes(&e, state + 0x90);
            assert_eq!(velocity[0], 4.0 * epsilon);
        }
    }

    #[test]
    fn warp_to_stores_the_pose_and_puts_both_centres_of_mass_on_it() {
        let mut e = engine();
        let log = install_rotation_set(&mut e, IDENTITY);
        let state = motion_state(&mut e);
        // A signalling NaN in the time word comes out quieted.
        e.mem.set_u32(state + 0x4c, 0x7f80_0001);
        let position = record(&mut e, 0x10, &[(0, [10.0, 20.0, 30.0, 40.0])]);
        let rotation = record(&mut e, 0x10, &[(0, [0.0, 0.0, 0.5, 0.5])]);
        e.call(0x00cf_1f80, &args![position, rotation, state]);
        assert_eq!(*log.borrow(), vec![[0.0, 0.0, 0.5, 0.5]]);
        assert_eq!(load_lanes(&e, state + 0x60), [0.0, 0.0, 0.5, 0.5]);
        assert_eq!(load_lanes(&e, state + 0x70), [0.0, 0.0, 0.5, 0.5]);
        assert_eq!(load_lanes(&e, state + 0x90), [0.0; 4]);
        assert_eq!(load_lanes(&e, state + 0x30), [10.0, 20.0, 30.0, 40.0]);
        // local centre (1, 1, 1, 0) rotated by the identity columns plus the position.
        let centre = [11.0, 21.0, 31.0];
        assert_eq!(load_lanes(&e, state + 0x40)[..3], centre);
        assert_eq!(load_lanes(&e, state + 0x50)[..3], centre);
        assert_eq!(e.mem.u32(state + 0x4c), 0x7fc0_0001);
        assert_eq!(e.mem.f32(state + 0x5c), 0.0);
    }

    #[test]
    fn warp_to_rotation_warps_to_the_states_own_position() {
        let mut e = engine();
        let log = install_rotation_set(&mut e, IDENTITY);
        let state = motion_state(&mut e);
        store_lanes(&mut e, state + 0x30, [7.0, 8.0, 9.0, 1.0]);
        let rotation = record(&mut e, 0x10, &[(0, [0.0, 0.0, 0.0, 1.0])]);
        e.call(0x00cf_2160, &args![rotation, state]);
        assert_eq!(log.borrow().len(), 1);
        assert_eq!(load_lanes(&e, state + 0x30), [7.0, 8.0, 9.0, 1.0]);
        assert_eq!(load_lanes(&e, state + 0x40)[..3], [8.0, 9.0, 10.0]);
    }

    #[test]
    fn warp_to_with_a_transform_copies_it_and_derives_the_quaternion() {
        let mut e = engine();
        // hkQuaternion::set double: the quaternion (0.5, 0.25, 0.125, 1) from any rotation.
        e.register_double(QUATERNION_SET, |e, a| {
            store_lanes(e, a[0], [0.5, 0.25, 0.125, 1.0]);
            Ret::default()
        });
        let state = motion_state(&mut e);
        e.mem.set_u32(state + 0x4c, 0x7f80_0001);
        let transform = record(
            &mut e,
            0x40,
            &[
                (0x00, [0.0, 1.0, 0.0, 0.0]),
                (0x10, [-1.0, 0.0, 0.0, 0.0]),
                (0x20, [0.0, 0.0, 1.0, 0.0]),
                (0x30, [100.0, 200.0, 300.0, 1.0]),
            ],
        );
        e.call(0x00cf_2020, &args![transform, state]);
        for column in 0..4u32 {
            assert_eq!(
                load_lanes(&e, state + 0x10 * column),
                load_lanes(&e, transform + 0x10 * column)
            );
        }
        assert_eq!(load_lanes(&e, state + 0x60), [0.5, 0.25, 0.125, 1.0]);
        assert_eq!(load_lanes(&e, state + 0x70), [0.5, 0.25, 0.125, 1.0]);
        assert_eq!(load_lanes(&e, state + 0x90), [0.0; 4]);
        // (1, 1, 1) rotated by the transform's columns: (-1, 1, 1), plus the translation.
        let centre = [99.0, 201.0, 301.0];
        assert_eq!(load_lanes(&e, state + 0x40)[..3], centre);
        assert_eq!(load_lanes(&e, state + 0x50)[..3], centre);
        assert_eq!(e.mem.u32(state + 0x4c), 0x7fc0_0001);
        assert_eq!(e.mem.f32(state + 0x5c), 0.0);
    }

    #[test]
    fn warp_to_position_keeps_the_rotation_and_copies_the_second_quaternion() {
        let mut e = engine();
        let state = motion_state(&mut e);
        // The state's own rotation columns are a 90 degree turn about z.
        store_lanes(&mut e, state, [0.0, 1.0, 0.0, 0.0]);
        store_lanes(&mut e, state + 0x10, [-1.0, 0.0, 0.0, 0.0]);
        store_lanes(&mut e, state + 0x20, [0.0, 0.0, 1.0, 0.0]);
        e.mem.set_u32(state + 0x4c, 0x7f80_0001);
        let position = record(&mut e, 0x10, &[(0, [10.0, 20.0, 30.0, 1.0])]);
        e.call(0x00cf_20d0, &args![position, state]);
        assert_eq!(load_lanes(&e, state + 0x30), [10.0, 20.0, 30.0, 1.0]);
        assert_eq!(load_lanes(&e, state + 0x90), [0.0; 4]);
        // (1, 1, 1) turned to (-1, 1, 1), plus the position.
        assert_eq!(load_lanes(&e, state + 0x40)[..3], [9.0, 21.0, 31.0]);
        assert_eq!(load_lanes(&e, state + 0x50)[..3], [9.0, 21.0, 31.0]);
        assert_eq!(e.mem.u32(state + 0x4c), 0x7fc0_0001);
        assert_eq!(e.mem.f32(state + 0x5c), 0.0);
        assert_eq!(load_lanes(&e, state + 0x60), [0.0, 0.6, 0.0, 0.8]);
        assert_eq!(load_lanes(&e, state + 0x70), [0.0, 0.6, 0.0, 0.8]);
    }

    #[test]
    fn set_centre_of_rotation_local_moves_both_centres_by_the_rotated_difference() {
        let mut e = engine();
        // setRotatedDir double: twice the vector it is given; it logs what it was given.
        let seen: Rc<RefCell<Vec<(u32, Lanes)>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(SET_ROTATED_DIR, move |e, a| {
            let v = load_lanes(e, a[2]);
            log.borrow_mut().push((a[1], v));
            store_lanes(e, a[0], v.map(|x| x * 2.0));
            Ret::default()
        });
        let state = motion_state(&mut e);
        let centre = record(&mut e, 0x10, &[(0, [3.0, 5.0, 7.0, 9.0])]);
        e.call(0x00cf_14a0, &args![centre, state]);
        // The new centre is stored; the call got the state's rotation and the difference.
        assert_eq!(load_lanes(&e, state + 0x80), [3.0, 5.0, 7.0, 9.0]);
        assert_eq!(*seen.borrow(), vec![(state, [2.0, 4.0, 6.0, 9.0])]);
        // The doubled difference (4, 8, 12, 18) masked to xyz is added to both centres; the
        // fourth lane adds 0.
        assert_eq!(load_lanes(&e, state + 0x40), [5.0, 10.0, 15.0, 1.0]);
        assert_eq!(load_lanes(&e, state + 0x50), [9.0, 14.0, 19.0, 0.5]);
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn lanes(e: &Engine, at: u32) -> [f32; 4] {
        std::array::from_fn(|lane| e.mem.f32(at + 4 * lane as u32))
    }

    fn set_lanes(e: &mut Engine, at: u32, values: [f32; 4]) {
        for (lane, value) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * lane as u32, *value);
        }
    }

    /// The constants the function reads: c_a = 0, c_b = 1, c_c = 0.5, c_d = 1.5 (so the scale is
    /// `1.5 - 0.5 * len2`), the double 1.0, the float 0.5, the vector of ones and the `inverse_length`
    /// constants 0, 3 and 0.5. `cb2d90` is a double that writes the identity columns and notes the
    /// quaternion address it was given.
    fn freeze_engine() -> (Engine, Rc<RefCell<Vec<Vec<u32>>>>) {
        let mut e = Engine::new();
        for page in [
            0x0109_2000,
            0x0101_d000,
            0x0101_1000,
            0x0101_6000,
            0x0101_2000,
            0x010e_c000,
            0x0101_7000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(0x0109_24a8, 0.0f64);
        e.set_global(0x0101_de30, 1.0f64);
        e.set_global(0x0101_1588, 0.5f64);
        e.set_global(0x0101_6ff0, 1.5f64);
        e.set_global(0x0101_2070, 1.0f64);
        e.set_global(0x0101_6248, 0.5f32);
        e.set_global(0x0101_1d78, 0.0f32);
        e.set_global(0x0101_7718, 3.0f32);
        for lane in 0..4u32 {
            e.set_global(0x010e_c7f0 + 4 * lane, 1.0f32);
        }
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00cb_2d90, move |e, a| {
            log.borrow_mut().push(a.to_vec());
            set_lanes(e, a[0], [1.0, 0.0, 0.0, 0.0]);
            set_lanes(e, a[0] + 0x10, [0.0, 1.0, 0.0, 0.0]);
            set_lanes(e, a[0] + 0x20, [0.0, 0.0, 1.0, 0.0]);
            Ret::default()
        });
        (e, seen)
    }

    /// A motion state: start time 1.0 and inverse interval `inverse_interval` in the fourth lanes of
    /// the two centres of mass, the given rotations, the local centre (10, 20, 30).
    fn motion_state(
        e: &mut Engine,
        inverse_interval: f32,
        old_rotation: [f32; 4],
        first_rotation: [f32; 4],
    ) -> u32 {
        let state = e.mem.alloc(0x90);
        set_lanes(e, state + 0x40, [4.0, 0.0, 0.0, 1.0]);
        set_lanes(e, state + 0x50, [8.0, 0.0, 0.0, inverse_interval]);
        set_lanes(e, state + 0x60, first_rotation);
        set_lanes(e, state + 0x70, old_rotation);
        set_lanes(e, state + 0x80, [10.0, 20.0, 30.0, 0.0]);
        state
    }

    fn assert_close(actual: [f32; 4], expected: [f32; 4]) {
        for lane in 0..4 {
            assert!(
                (actual[lane] - expected[lane]).abs() < 1e-5,
                "{actual:?} against {expected:?}"
            );
        }
    }

    /// Factor 0.25 (below 0.5): the rotations (2,0,0,0) and (-1,0,0,0) add to (1,0,0,0), which is
    /// scaled by 1 and blended with weight 0.5 against the first rotation into zero.
    #[test]
    fn hk_swept_transform_util_freeze_motion_state_blends_below_the_half() {
        let (mut e, seen) = freeze_engine();
        let state = motion_state(&mut e, 0.25, [2.0, 0.0, 0.0, 0.0], [-1.0, 0.0, 0.0, 0.0]);
        e.call(0x00cf_1d50, &args![2.0f32, state]);
        // The blended rotation is zero, its inverse length is 0 and so the rotations become zero.
        assert_eq!(lanes(&e, state + 0x60), [0.0; 4]);
        assert_eq!(lanes(&e, state + 0x70), [0.0; 4]);
        // The centre: 0.75 * (4,0,0) + 0.25 * (8,0,0) = (5,0,0); time 2.0 and 0.0 in the fourth lanes.
        assert_eq!(lanes(&e, state + 0x40), [5.0, 0.0, 0.0, 2.0]);
        assert_eq!(lanes(&e, state + 0x50), [5.0, 0.0, 0.0, 0.0]);
        // Identity columns: translation = centre - (10, 20, 30).
        assert_eq!(lanes(&e, state + 0x30), [-5.0, -20.0, -30.0, 0.0]);
        assert_eq!(*seen.borrow(), vec![vec![state, state + 0x70]]);
    }

    /// Factor 0.75: the other blend, (1 - 0.5) * (1,0,0,0) + 0.5 * (2,0,0,0) = (1.5,0,0,0), normalised.
    #[test]
    fn hk_swept_transform_util_freeze_motion_state_blends_from_the_half_up() {
        let (mut e, _) = freeze_engine();
        let state = motion_state(&mut e, 0.75, [2.0, 0.0, 0.0, 0.0], [-1.0, 0.0, 0.0, 0.0]);
        e.call(0x00cf_1d50, &args![2.0f32, state]);
        assert_close(lanes(&e, state + 0x60), [1.0, 0.0, 0.0, 0.0]);
        assert_close(lanes(&e, state + 0x70), [1.0, 0.0, 0.0, 0.0]);
        // 0.25 * (4,0,0) + 0.75 * (8,0,0) = (7,0,0).
        assert_eq!(lanes(&e, state + 0x40), [7.0, 0.0, 0.0, 2.0]);
        assert_eq!(lanes(&e, state + 0x30), [-3.0, -20.0, -30.0, 0.0]);
    }

    /// A requested time below the start time keeps the start time, and the factor becomes 0.
    #[test]
    fn hk_swept_transform_util_freeze_motion_state_keeps_the_later_start_time() {
        let (mut e, _) = freeze_engine();
        let state = motion_state(&mut e, 0.5, [1.0, 0.0, 0.0, 0.0], [0.0; 4]);
        e.call(0x00cf_1d50, &args![0.5f32, state]);
        // T = 1.0 (the start time), t = 0: the centre keeps its start value and the time stays 1.0.
        assert_eq!(lanes(&e, state + 0x40), [4.0, 0.0, 0.0, 1.0]);
        assert_eq!(lanes(&e, state + 0x50), [4.0, 0.0, 0.0, 0.0]);
    }
}
