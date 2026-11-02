//! `BSHavok/bhkcharacterlistener.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkCharacterListener` is the `hkpCharacterProxyListener` a character
//! proxy reports its contacts to. It sits at +0x410 of its
//! `bhkCharacterProxy` (`contactPointAddedCallback` passes `this - 0x410`
//! to `bhkCharacterProxy::SetLinearVelocity`). Its vtable is `010c4488`
//! (RTTI `.?AVbhkCharacterListener@@`): slot 0 `00c6cb70`, 2 `00c6c840`,
//! 3 `00c6c870`, 4 `00c6ca30`; the base `hkpCharacterProxyListener`'s is
//! `010c446c`.
//!
//! SSE code (`SUBPS`, `MULPS`, `ADDSS`) computes lane by lane in `f32`;
//! x87 code compares the way its `FNSTSW`/`TEST AH` pairs read the flags
//! (NaN included) and rounds at each `float` store; a float copied through
//! `FLD`/`FSTP` goes through [`x87_quiet`]. `RSQRTSS` is the host CPU's
//! estimate (`rsqrt_estimate`), as elsewhere in the crate. The `/GS` cookie
//! checks of the functions marked `Translation incomplete` are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::x87::rsqrt_estimate;

layout! {
    /// `bhkCharacterListener` (Xbox PDB), 0x70 bytes (offsets checked
    /// against the constructor `00c6c7a0`).
    pub struct BhkCharacterListener: 0x70 {
        /// `iFlags` (Xbox PDB).
        0x04 iFlags: u32,
        // +0x10 `hkUp` (Xbox PDB): `hkVector4`, four floats.
        /// `fMaxSlope` (Xbox PDB).
        0x20 fMaxSlope: f32,
        /// `eSupportMaterial` (Xbox PDB).
        0x24 eSupportMaterial: u32,
        /// `eSupportLayer` (Xbox PDB).
        0x28 eSupportLayer: u32,
        // +0x30 `hkSupportVelocity`, +0x40 `hkSupportNorm` (Xbox PDB):
        // `hkVector4`s.
        /// `SupportHeight` (Xbox PDB): the first of four floats (+0x50 to
        /// +0x5C); the smallest positive normal float (`0102efc4`) marks an
        /// unset one.
        0x50 SupportHeight: f32,
        /// `fStepHeight` (Xbox PDB).
        0x60 fStepHeight: f32,
        /// `fCastDepth` (Xbox PDB).
        0x64 fCastDepth: f32,
        /// `bAllSurfacesStopped` (Xbox PDB).
        0x68 bAllSurfacesStopped: bool,
        /// `bStickyCollisions` (Xbox PDB).
        0x69 bStickyCollisions: bool,
        /// `iNumCollisions` (Xbox PDB).
        0x6C iNumCollisions: u32,
    }
}

/// `bhkCharacterListener`'s vtable.
const VTABLE: u32 = 0x010c_4488;
/// `hkpCharacterProxyListener`'s vtable.
const BASE_VTABLE: u32 = 0x010c_446c;
/// The game heap singleton and its `Deallocate` (`00aa4060`, thiscall
/// `(block)`).
const HEAP_SINGLETON: u32 = 0x011f_6238;
const HEAP_FREE: u32 = 0x00aa_4060;
/// The smallest positive normal float (`0x00800000`), the unset support
/// height.
const UNSET_HEIGHT: u32 = 0x0102_efc4;
/// `0.5` (`double`).
const HALF_DOUBLE: u32 = 0x0101_1588;
/// A global `hkVector4` the velocity is compared with (zero in the exe's
/// data).
const REFERENCE_VELOCITY: u32 = 0x0126_7e30;
/// The four-lane absolute-value mask (`0x7FFFFFFF`).
const ABS_MASK: u32 = 0x0101_7d10;
/// `0.01` (`float`).
const VELOCITY_TOLERANCE: u32 = 0x0101_3ea4;
/// `0.0`, `3.0` and `0.5` (`float`): the `RSQRTSS` refinement.
const ZERO: u32 = 0x0101_1d78;
const THREE: u32 = 0x0101_7718;
const HALF: u32 = 0x0101_6248;
/// A global `NiPoint3` the code passes to clear a velocity.
const UNSET_POINT: u32 = 0x011f_426c;
/// `hkpCharacterProxy::getLinearVelocity` (Xbox PDB): `this + 0x10`.
const GET_LINEAR_VELOCITY: u32 = 0x00ca_c950;
/// `hkpCharacterProxy`'s velocity setter (`00cac960`, `(hkVector4*)`).
const SET_LINEAR_VELOCITY: u32 = 0x00ca_c960;
/// `bhkCharacterProxy::SetLinearVelocity` (Xbox PDB).
const PROXY_SET_LINEAR_VELOCITY: u32 = 0x0092_6980;
/// Two CRT routines of the constructor (`LIBCMT`, unnamed in the map):
/// each takes its argument in ST0 (here a leading `f64`) and returns ST0.
const CRT_00EC9F70: u32 = 0x00ec_9f70;
const CRT_00ECA0A0: u32 = 0x00ec_a0a0;

/// Four `float` lanes at `at`.
fn lanes(e: &Engine, at: u32) -> [f32; 4] {
    [0, 1, 2, 3].map(|i| e.mem.f32(at + 4 * i))
}

fn set_lanes(e: &mut Engine, at: u32, value: [f32; 4]) {
    for (i, lane) in value.into_iter().enumerate() {
        e.mem.set_f32(at + 4 * i as u32, lane);
    }
}

/// `x*x + y*y + z*z` the way the SSE code sums it (`MULPS`, then
/// `z + (y + x)` with `ADDSS`).
fn length_squared(v: [f32; 4]) -> f32 {
    let squares = v.map(|lane| lane * lane);
    squares[2] + (squares[1] + squares[0])
}

// Translated from 00c6c770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCharacterProxyListener::`scalar deleting destructor'` (Xbox PDB):
/// puts back the base vtable and frees the object (`Deallocate`) when bit 0
/// of `flags` is set; returns `this`.
pub fn hkp_character_proxy_listener_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BhkCharacterListener>,
    flags: u8,
) -> Ptr<BhkCharacterListener> {
    e.mem.set_u32(this.addr(), BASE_VTABLE);
    if flags & 1 != 0 {
        e.call(HEAP_FREE, &args![HEAP_SINGLETON, this]);
    }
    this
}

// Translated from 00c6c7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `bhkCharacterListener` constructor: copies `up` (an `hkVector4`)
/// into `hkUp`, sets `fMaxSlope` to `00ec9f70(slope)`, `fStepHeight` to
/// `step_height`, `eSupportMaterial` to 0x24, the support heights to unset,
/// clears the rest, and sets `fCastDepth` to
/// `(step_height / 00eca0a0(slope)) * fMaxSlope`. Returns `this`.
pub fn bhk_character_listener_ctor(
    e: &mut Engine,
    this: Ptr<BhkCharacterListener>,
    up: u32,
    slope: f32,
    step_height: f32,
) -> Ptr<BhkCharacterListener> {
    let t = this.addr();
    e.mem.set_u32(t, VTABLE);
    let up = lanes(e, up);
    set_lanes(e, t + 0x10, up);
    let max_slope = e.call(CRT_00EC9F70, &args![x87_quiet(slope) as f64]).f64() as f32;
    e.set(this, BhkCharacterListener::fMaxSlope, x87_quiet(max_slope));
    e.set(this, BhkCharacterListener::iFlags, 0);
    let step_height = x87_quiet(step_height);
    e.set(this, BhkCharacterListener::fStepHeight, step_height);
    e.set(this, BhkCharacterListener::eSupportMaterial, 0x24);
    e.set(this, BhkCharacterListener::eSupportLayer, 0);
    set_lanes(e, t + 0x30, [0.0; 4]);
    let unset = x87_quiet(e.global::<f32>(UNSET_HEIGHT));
    for slot in [0x5c, 0x58, 0x54, 0x50] {
        e.mem.set_f32(t + slot, unset);
    }
    e.set(this, BhkCharacterListener::bAllSurfacesStopped, false);
    e.set(this, BhkCharacterListener::iNumCollisions, 0);
    e.set(this, BhkCharacterListener::bStickyCollisions, false);
    let divisor = e.call(CRT_00ECA0A0, &args![x87_quiet(slope) as f64]).f64() as f32;
    let cast_depth = (step_height as f64 / divisor as f64) * max_slope as f64;
    e.set(this, BhkCharacterListener::fCastDepth, cast_depth as f32);
    this
}

// Translated from 00c6c840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterListener::contactPointAddedCallback` (Xbox PDB): for a
/// contact whose object (`event + 0x48`) has 0x1E in the low 7 bits of its
/// word +0x1C, counts it in `iNumCollisions` and, with
/// `bStickyCollisions`, stops the proxy (`SetLinearVelocity` with the unset
/// point on `this - 0x410`).
pub fn bhk_character_listener_contact_point_added_callback(
    e: &mut Engine,
    this: Ptr<BhkCharacterListener>,
    _unused_1: u32,
    event: u32,
) {
    let object = e.mem.u32(event + 0x48);
    if e.mem.u32(object + 0x1c) & 0x7f != 0x1e {
        return;
    }
    let count = e.get(this, BhkCharacterListener::iNumCollisions);
    e.set(
        this,
        BhkCharacterListener::iNumCollisions,
        count.wrapping_add(1),
    );
    if e.get(this, BhkCharacterListener::bStickyCollisions) {
        let proxy = this.addr().wrapping_sub(0x410);
        e.call(PROXY_SET_LINEAR_VELOCITY, &args![proxy, UNSET_POINT]);
    }
}

// Translated from 00c6c870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterListener::contactPointRemovedCallback` (Xbox PDB): for the
/// same kind of contact, takes it off `iNumCollisions` (not below 0).
pub fn bhk_character_listener_contact_point_removed_callback(
    e: &mut Engine,
    this: Ptr<BhkCharacterListener>,
    _unused_1: u32,
    event: u32,
) {
    let object = e.mem.u32(event + 0x48);
    if e.mem.u32(object + 0x1c) & 0x7f != 0x1e {
        return;
    }
    let count = e.get(this, BhkCharacterListener::iNumCollisions);
    if count != 0 {
        e.set(this, BhkCharacterListener::iNumCollisions, count - 1);
    }
}

/// `00c6c890`'s update of one support height: the new height `height` if
/// the slot is unset (exactly the unset value), otherwise the mean of the
/// two (`(height + slot)` stored as a `float`, times `0.5`).
fn merge_support_height(e: &mut Engine, slot: u32, height: f32, unset: f32, half: f64) {
    let current = e.mem.f32(slot);
    // `FCOM`, `TEST AH,0x44` / `JP`: only an exact match (no NaN) is unset.
    let value = if unset == current {
        x87_quiet(height)
    } else {
        let sum = (height as f64 + current as f64) as f32;
        (sum as f64 * half) as f32
    };
    e.mem.set_f32(slot, value);
}

// Translated from 00c6c890 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c6c890 - the /GS cookie check 00ec408c is not modeled
/// Records the height `point.z` in the support heights: of the four
/// points at `corners` (four `hkVector4`), the horizontal distance to
/// `point` picks slot 0 (corner 1 farther than corner 0) or 1, and slot 2
/// (corner 3 farther than corner 2) or 3; each slot takes the height
/// (`merge_support_height`).
pub fn fn_00c6c890(e: &mut Engine, this: Ptr<BhkCharacterListener>, corners: u32, point: u32) {
    let target = lanes(e, point);
    let differences = [0u32, 1, 2, 3].map(|i| {
        let corner = lanes(e, corners + 0x10 * i);
        let mut difference = [0, 1, 2, 3].map(|lane| target[lane] - corner[lane]);
        difference[2] = 0.0;
        difference
    });
    let unset = e.global::<f32>(UNSET_HEIGHT);
    let half: f64 = e.global(HALF_DOUBLE);
    let height = target[2];
    let base = this.addr() + 0x50;

    let first = length_squared(differences[0]);
    let second = length_squared(differences[1]);
    // `TEST AH,0x41` / `JNZ`: slot 0 only when `second > first` (no NaN).
    let slot = if second > first { 0 } else { 1 };
    merge_support_height(e, base + 4 * slot, height, unset, half);

    let third = length_squared(differences[2]);
    let fourth = length_squared(differences[3]);
    let slot = if fourth > third { 2 } else { 3 };
    merge_support_height(e, base + 4 * slot, height, unset, half);
}

// Translated from 00c6ca30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c6ca30 - the /GS cookie check 00ec408c is not modeled
/// `bhkCharacterListener::characterInteractionCallback` (Xbox PDB): unless
/// `proxy`'s velocity is within 0.01 of the reference vector in x, y and z,
/// pushes `other` along the contact normal (`contact + 0x10`, z cleared,
/// normalized with the refined `RSQRTSS`): `other`'s velocity gains
/// `normal * scale * velocity` lane by lane (velocity z cleared), set
/// through `00cac960`. `this` (`ECX`) is not read.
pub fn bhk_character_listener_character_interaction_callback(
    e: &mut Engine,
    _unused_0: u32,
    proxy: u32,
    other: u32,
    contact: u32,
) {
    let at = e.call(GET_LINEAR_VELOCITY, &args![proxy]).u32();
    let mut velocity = lanes(e, at);
    let reference = lanes(e, REFERENCE_VELOCITY);
    let mask = lanes(e, ABS_MASK);
    let tolerance: f32 = e.global(VELOCITY_TOLERANCE);
    let mut close = 0u32;
    for (lane, ((v, r), m)) in velocity.iter().zip(reference).zip(mask).enumerate() {
        let difference = (v - r).to_bits() & m.to_bits();
        // `CMPLEPS`: false for a NaN.
        if f32::from_bits(difference) <= tolerance {
            close |= 1 << lane;
        }
    }
    if close & 7 == 7 {
        return;
    }
    let mut normal = lanes(e, contact + 0x10);
    normal[2] = 0.0;
    velocity[2] = 0.0;
    let sum = length_squared(normal);
    let estimate = rsqrt_estimate(sum);
    let zero: f32 = e.global(ZERO);
    let three: f32 = e.global(THREE);
    let half: f32 = e.global(HALF);
    let refined = (half * estimate) * (three - (sum * estimate) * estimate);
    // `CMPEQSS` / `ANDNPS`: 0 when the sum is 0.
    let scale = if sum == zero { 0.0 } else { refined };
    let push = [0, 1, 2, 3].map(|lane| (scale * normal[lane]) * velocity[lane]);
    let other_at = e.call(GET_LINEAR_VELOCITY, &args![other]).u32();
    let current = lanes(e, other_at);
    e.with_stack(0x10, |e, local| {
        let result = [0, 1, 2, 3].map(|lane| current[lane] + push[lane]);
        set_lanes(e, local.addr(), result);
        e.call(SET_LINEAR_VELOCITY, &args![other, local]);
    });
}

// Translated from 00c6cb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterListener::`scalar deleting destructor'` (Xbox PDB): puts
/// back the base vtable (`hkpCharacterProxyListener`'s) and, when bit 0 of
/// `flags` is set, frees the block the object was aligned in (`this` minus
/// the byte at `this - 1`). Returns `this`.
pub fn bhk_character_listener_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BhkCharacterListener>,
    flags: u8,
) -> Ptr<BhkCharacterListener> {
    e.mem.set_u32(this.addr(), BASE_VTABLE);
    if flags & 1 != 0 {
        let padding = e.mem.u8(this.addr().wrapping_sub(1)) as u32;
        let block = this.addr().wrapping_sub(padding);
        e.call(HEAP_FREE, &args![HEAP_SINGLETON, block]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c6c770,
            hkp_character_proxy_listener_scalar_deleting_destructor(
                Ptr<BhkCharacterListener>,
                u8,
            )
                -> Ptr<BhkCharacterListener>
        ),
        entry!(
            0x00c6c7a0,
            bhk_character_listener_ctor(
                Ptr<BhkCharacterListener>,
                u32,
                f32,
                f32,
            ) -> Ptr<BhkCharacterListener>
        ),
        entry!(
            0x00c6c840,
            bhk_character_listener_contact_point_added_callback(
                Ptr<BhkCharacterListener>,
                u32,
                u32,
            )
        ),
        entry!(
            0x00c6c870,
            bhk_character_listener_contact_point_removed_callback(
                Ptr<BhkCharacterListener>,
                u32,
                u32,
            )
        ),
        entry!(0x00c6c890, fn_00c6c890(Ptr<BhkCharacterListener>, u32, u32)),
        entry!(
            0x00c6ca30,
            bhk_character_listener_character_interaction_callback(u32, u32, u32, u32)
        ),
        entry!(
            0x00c6cb70,
            bhk_character_listener_scalar_deleting_destructor(
                Ptr<BhkCharacterListener>,
                u8,
            )
                -> Ptr<BhkCharacterListener>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCRATCH: u32 = 0x0600_0000;
    const UNSET: u32 = 0x0080_0000;

    fn engine() -> (Engine, Ptr<BhkCharacterListener>) {
        let mut e = Engine::new();
        for a in [
            HEAP_FREE,
            GET_LINEAR_VELOCITY,
            SET_LINEAR_VELOCITY,
            PROXY_SET_LINEAR_VELOCITY,
            CRT_00EC9F70,
            CRT_00ECA0A0,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        for page in [
            0x0102_e000,
            0x0101_1000,
            0x0126_7000,
            0x0101_7000,
            0x0101_3000,
            0x0101_6000,
        ] {
            e.map(page, 0x1000);
        }
        e.map(SCRATCH, 0x1000);
        e.mem.set_u32(UNSET_HEIGHT, UNSET);
        e.set_global(HALF_DOUBLE, 0.5f64);
        for i in 0..4 {
            e.mem.set_u32(ABS_MASK + 4 * i, 0x7fff_ffff);
        }
        e.set_global(VELOCITY_TOLERANCE, 0.01f32);
        e.set_global(THREE, 3.0f32);
        e.set_global(HALF, 0.5f32);
        let listener = e.new_object::<BhkCharacterListener>();
        (e, listener)
    }

    fn logged(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn heights(e: &Engine, listener: Ptr<BhkCharacterListener>) -> [u32; 4] {
        [0u32, 1, 2, 3].map(|i| e.mem.u32(listener.addr() + 0x50 + 4 * i))
    }

    #[test]
    fn proxy_listener_deleting_destructor_frees_with_bit_0() {
        let (mut e, listener) = engine();
        e.call_log = Some(vec![]);
        let r = e.call(0x00c6_c770, &args![listener, 0u32]).u32();
        assert_eq!(r, listener.addr());
        assert_eq!(e.mem.u32(listener.addr()), BASE_VTABLE);
        assert!(logged(&e, HEAP_FREE).is_empty());
        e.call(0x00c6_c770, &args![listener, 0x101u32]);
        assert_eq!(
            logged(&e, HEAP_FREE),
            vec![vec![HEAP_SINGLETON, listener.addr()]]
        );
    }

    #[test]
    fn listener_deleting_destructor_frees_the_aligned_block() {
        let (mut e, _) = engine();
        let listener = Ptr::<BhkCharacterListener>::new(SCRATCH + 0x110);
        e.mem.set_u8(SCRATCH + 0x10f, 0xc);
        e.call_log = Some(vec![]);
        e.call(0x00c6_cb70, &args![listener, 0u32]);
        assert!(logged(&e, HEAP_FREE).is_empty());
        let r = e.call(0x00c6_cb70, &args![listener, 1u32]).u32();
        assert_eq!(r, listener.addr());
        assert_eq!(e.mem.u32(listener.addr()), BASE_VTABLE);
        assert_eq!(
            logged(&e, HEAP_FREE),
            vec![vec![HEAP_SINGLETON, SCRATCH + 0x104]]
        );
    }

    #[test]
    fn constructor_sets_the_slope_and_cast_depth() {
        let (mut e, listener) = engine();
        for (i, v) in [0.0f32, 0.0, 1.0, 0.0].into_iter().enumerate() {
            e.mem.set_f32(SCRATCH + 4 * i as u32, v);
        }
        e.register(CRT_00EC9F70, |_, a| {
            (f64::from_bits(a[0] as u64 | (a[1] as u64) << 32) * 0.5).into_ret()
        });
        e.register(CRT_00ECA0A0, |_, _| 4.0f64.into_ret());
        e.mem.set_u32(listener.addr() + 0x6c, 9);
        let r = e
            .call(0x00c6_c7a0, &args![listener, SCRATCH, 0.5f32, 10.0f32])
            .u32();
        assert_eq!(r, listener.addr());
        assert_eq!(e.mem.u32(listener.addr()), VTABLE);
        assert_eq!(e.mem.f32(listener.addr() + 0x18), 1.0);
        assert_eq!(e.get(listener, BhkCharacterListener::fMaxSlope), 0.25);
        assert_eq!(e.get(listener, BhkCharacterListener::fStepHeight), 10.0);
        assert_eq!(
            e.get(listener, BhkCharacterListener::eSupportMaterial),
            0x24
        );
        assert_eq!(heights(&e, listener), [UNSET; 4]);
        assert_eq!(e.get(listener, BhkCharacterListener::iNumCollisions), 0);
        assert_eq!(
            e.get(listener, BhkCharacterListener::fCastDepth),
            (10.0 / 4.0) * 0.25
        );
    }

    #[test]
    fn contact_added_counts_the_layer_and_stops_a_sticky_proxy() {
        let (mut e, listener) = engine();
        e.mem.set_u32(SCRATCH + 0x48, SCRATCH + 0x200);
        e.mem.set_u32(SCRATCH + 0x21c, 0x9e);
        e.call_log = Some(vec![]);
        e.call(0x00c6_c840, &args![listener, 0u32, SCRATCH]);
        assert_eq!(e.get(listener, BhkCharacterListener::iNumCollisions), 1);
        assert!(logged(&e, PROXY_SET_LINEAR_VELOCITY).is_empty());
        e.set(listener, BhkCharacterListener::bStickyCollisions, true);
        e.call(0x00c6_c840, &args![listener, 0u32, SCRATCH]);
        assert_eq!(e.get(listener, BhkCharacterListener::iNumCollisions), 2);
        assert_eq!(
            logged(&e, PROXY_SET_LINEAR_VELOCITY),
            vec![vec![listener.addr() - 0x410, UNSET_POINT]]
        );
        e.mem.set_u32(SCRATCH + 0x21c, 0x1f);
        e.call(0x00c6_c840, &args![listener, 0u32, SCRATCH]);
        assert_eq!(e.get(listener, BhkCharacterListener::iNumCollisions), 2);
    }

    #[test]
    fn contact_removed_counts_down_to_zero() {
        let (mut e, listener) = engine();
        e.mem.set_u32(SCRATCH + 0x48, SCRATCH + 0x200);
        e.mem.set_u32(SCRATCH + 0x21c, 0x1e);
        e.set(listener, BhkCharacterListener::iNumCollisions, 1);
        e.call(0x00c6_c870, &args![listener, 0u32, SCRATCH]);
        assert_eq!(e.get(listener, BhkCharacterListener::iNumCollisions), 0);
        e.call(0x00c6_c870, &args![listener, 0u32, SCRATCH]);
        assert_eq!(e.get(listener, BhkCharacterListener::iNumCollisions), 0);
    }

    #[test]
    fn support_heights_take_the_nearer_corners() {
        let (mut e, listener) = engine();
        for i in 0..4 {
            e.mem.set_u32(listener.addr() + 0x50 + 4 * i, UNSET);
        }
        e.mem.set_f32(listener.addr() + 0x5c, 3.0);
        // Corners: 0 at (0,0), 1 at (10,0), 2 at (10,10), 3 at (0,1).
        let corners = [[0.0f32, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 1.0]];
        for (i, c) in corners.into_iter().enumerate() {
            e.mem.set_f32(SCRATCH + 0x10 * i as u32, c[0]);
            e.mem.set_f32(SCRATCH + 0x10 * i as u32 + 4, c[1]);
            // A z that would win without the cleared lane.
            e.mem
                .set_f32(SCRATCH + 0x10 * i as u32 + 8, 1000.0 * i as f32);
        }
        e.mem.set_f32(SCRATCH + 0x100, 1.0);
        e.mem.set_f32(SCRATCH + 0x108, 7.0);
        e.call(0x00c6_c890, &args![listener, SCRATCH, SCRATCH + 0x100]);
        // Corner 1 is farther than corner 0: slot 0 takes 7. Corner 3 is
        // nearer than corner 2: slot 3 averages 7 and 3.
        assert_eq!(
            heights(&e, listener),
            [7.0f32.to_bits(), UNSET, UNSET, 5.0f32.to_bits()]
        );
    }

    #[test]
    fn interaction_pushes_the_other_proxy_along_the_normal() {
        let (mut e, _) = engine();
        let proxy = SCRATCH + 0x100;
        let other = SCRATCH + 0x200;
        let contact = SCRATCH + 0x300;
        let velocity = [2.0f32, 3.0, 4.0, 5.0];
        let normal = [3.0f32, 4.0, 9.0, 2.0];
        for i in 0..4u32 {
            e.mem.set_f32(proxy + 0x10 + 4 * i, velocity[i as usize]);
            e.mem.set_f32(other + 0x10 + 4 * i, 1.0);
            e.mem.set_f32(contact + 0x10 + 4 * i, normal[i as usize]);
        }
        e.register(GET_LINEAR_VELOCITY, |_, a| (a[0] + 0x10).into_ret());
        e.register(SET_LINEAR_VELOCITY, |e, a| {
            for i in 0..4 {
                let v = e.mem.f32(a[1] + 4 * i);
                e.mem.set_f32(SCRATCH + 0x400 + 4 * i, v);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c6_ca30, &args![0u32, proxy, other, contact]);
        let sum = 4.0f32 * 4.0 + 3.0 * 3.0;
        let r = rsqrt_estimate(sum);
        let scale = (0.5 * r) * (3.0 - (sum * r) * r);
        let expected = [
            1.0 + (scale * 3.0) * 2.0,
            1.0 + (scale * 4.0) * 3.0,
            1.0 + (scale * 0.0) * 0.0,
            1.0 + (scale * 2.0) * 5.0,
        ];
        assert_eq!(logged(&e, SET_LINEAR_VELOCITY)[0][0], other);
        assert_eq!(
            [0u32, 1, 2, 3].map(|i| e.mem.f32(SCRATCH + 0x400 + 4 * i)),
            expected
        );
    }

    #[test]
    fn interaction_does_nothing_for_a_resting_proxy() {
        let (mut e, _) = engine();
        let proxy = SCRATCH + 0x100;
        // Only the w lane differs from the reference vector.
        e.mem.set_f32(proxy + 0x10, 0.005);
        e.mem.set_f32(proxy + 0x1c, 9.0);
        e.register(GET_LINEAR_VELOCITY, |_, a| (a[0] + 0x10).into_ret());
        e.call_log = Some(vec![]);
        e.call(
            0x00c6_ca30,
            &args![0u32, proxy, SCRATCH + 0x200, SCRATCH + 0x300],
        );
        assert!(logged(&e, SET_LINEAR_VELOCITY).is_empty());
        // A NaN lane is not close.
        e.mem.set_f32(proxy + 0x14, f32::NAN);
        e.call(
            0x00c6_ca30,
            &args![0u32, proxy, SCRATCH + 0x200, SCRATCH + 0x300],
        );
        assert_eq!(logged(&e, SET_LINEAR_VELOCITY).len(), 1);
    }
}
