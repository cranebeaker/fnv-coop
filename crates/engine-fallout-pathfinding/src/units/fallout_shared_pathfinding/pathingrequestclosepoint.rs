//! `fallout shared/pathfinding/pathingrequestclosepoint.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `PathingRequestClosePoint` is a `PathingRequest` (0xB0 bytes on PC) with
//! two more floats, the minimum and maximum distance of the point searched.
//! Its vtable is `0x0106d230` (RTTI name `PathingRequestClosePoint`).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of `PathingRequestClosePoint`.
const VTABLE: u32 = 0x0106_d230;
/// `006e2420(this)`: the `PathingRequest` constructor.
const PATHING_REQUEST_CONSTRUCT: u32 = 0x006e_2420;
/// `006e26a0(this, other)`: the `PathingRequest` copy constructor.
const PATHING_REQUEST_COPY_CONSTRUCT: u32 = 0x006e_26a0;
/// `006e2750(this, destination)`: `PathingRequest::CopyTo`.
const PATHING_REQUEST_COPY_TO: u32 = 0x006e_2750;
/// `006dad70(this)`: the `PathingRequest` destructor body.
const PATHING_REQUEST_DESTROY: u32 = 0x006d_ad70;
/// `006d2c20(this)`: returns the float at `+0xB0` of a close-point request
/// (the minimum distance) in ST0.
const GET_MIN_DISTANCE: u32 = 0x006d_2c20;
/// `006d2c40(this)`: returns the float at `+0xB4` (the maximum distance).
const GET_MAX_DISTANCE: u32 = 0x006d_2c40;
/// `00507610(this, float)`: stores the minimum distance into the request.
const SET_MIN_DISTANCE: u32 = 0x0050_7610;
/// `006e5ee0(this, float)`: stores the maximum distance into the request.
const SET_MAX_DISTANCE: u32 = 0x006e_5ee0;
/// `operator delete` with the size (`00aa1460(block, size)`, cdecl).
const SIZED_DELETE: u32 = 0x00aa_1460;
/// The default minimum distance (`128.0f`).
const DEFAULT_MIN_DISTANCE: u32 = 0x0101_e704;
/// The default maximum distance (`2048.0f`).
const DEFAULT_MAX_DISTANCE: u32 = 0x0101_8bfc;

layout! {
    /// `PathingRequestClosePoint` (Xbox PDB), 0xB8 bytes: a `PathingRequest`
    /// (0xB0 bytes) and two floats.
    pub struct PathingRequestClosePoint: 0xB8 {
        /// `fMinDistance` (Xbox PDB).
        0xB0 fMinDistance: f32,
        /// `fMaxDistance` (Xbox PDB).
        0xB4 fMaxDistance: f32,
    }
}

/// A float the exe copies through `FLD` and `FSTP`: a signaling NaN comes
/// out quiet.
fn quiet(value: f32) -> f32 {
    let bits = value.to_bits();
    if bits & 0x7f80_0000 == 0x7f80_0000 && bits & 0x007f_ffff != 0 {
        f32::from_bits(bits | 0x0040_0000)
    } else {
        value
    }
}

// Translated from 006e3f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestClosePoint::PathingRequestClosePoint` (Xbox PDB): runs the
/// `PathingRequest` constructor, stores the vtable and sets the minimum
/// and maximum distance from the two float constants.
pub fn pathing_request_close_point_pathing_request_close_point(
    e: &mut Engine,
    this: Ptr<PathingRequestClosePoint>,
) -> Ptr<PathingRequestClosePoint> {
    e.call(PATHING_REQUEST_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    let minimum = e.global::<f32>(DEFAULT_MIN_DISTANCE);
    e.set(this, PathingRequestClosePoint::fMinDistance, quiet(minimum));
    let maximum = e.global::<f32>(DEFAULT_MAX_DISTANCE);
    e.set(this, PathingRequestClosePoint::fMaxDistance, quiet(maximum));
    this
}

// Translated from 006e3f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestClosePoint::_scalar_deleting_destructor_` (Xbox PDB): runs
/// the `PathingRequest` destructor body and, when bit 0 of `flags` is set,
/// frees the 0xB8-byte object. Returns `this`.
pub fn pathing_request_close_point_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<PathingRequestClosePoint>,
    flags: u32,
) -> Ptr<PathingRequestClosePoint> {
    e.call(PATHING_REQUEST_DESTROY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, PathingRequestClosePoint::SIZE]);
    }
    this
}

// Translated from 006e3fb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e3fb0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Copy constructor of `PathingRequestClosePoint` (no Xbox PDB name): runs
/// the `PathingRequest` copy constructor on `other`, stores the vtable and
/// takes the two distances from `other` through their getters (`006d2c20`,
/// `006d2c40`). The exception-unwinding frame is not translated.
pub fn fn_006e3fb0(
    e: &mut Engine,
    this: Ptr<PathingRequestClosePoint>,
    other: Ptr<PathingRequestClosePoint>,
) -> Ptr<PathingRequestClosePoint> {
    e.call(PATHING_REQUEST_COPY_CONSTRUCT, &args![this, other]);
    e.mem.set_u32(this.addr(), VTABLE);
    let minimum = e.call(GET_MIN_DISTANCE, &args![other]).f32();
    e.set(this, PathingRequestClosePoint::fMinDistance, minimum);
    let maximum = e.call(GET_MAX_DISTANCE, &args![other]).f32();
    e.set(this, PathingRequestClosePoint::fMaxDistance, maximum);
    this
}

// Translated from 006e4030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestClosePoint::CopyTo` (Xbox PDB): runs `PathingRequest::CopyTo`
/// into `destination`, then stores this request's minimum and maximum
/// distance into it through its setters (`00507610`, `006e5ee0`), each
/// passed as the float's bit pattern.
pub fn pathing_request_close_point_copy_to(
    e: &mut Engine,
    this: Ptr<PathingRequestClosePoint>,
    destination: Ptr<PathingRequestClosePoint>,
) {
    e.call(PATHING_REQUEST_COPY_TO, &args![this, destination]);
    let minimum = e.get(this, PathingRequestClosePoint::fMinDistance);
    e.call(SET_MIN_DISTANCE, &args![destination, quiet(minimum)]);
    let maximum = e.get(this, PathingRequestClosePoint::fMaxDistance);
    e.call(SET_MAX_DISTANCE, &args![destination, quiet(maximum)]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x006e3f30,
            pathing_request_close_point_pathing_request_close_point(
                Ptr<PathingRequestClosePoint>,
            ) -> Ptr<
                PathingRequestClosePoint,
            >
        ),
        entry!(
            0x006e3f70,
            pathing_request_close_point_scalar_deleting_destructor(
                Ptr<PathingRequestClosePoint>,
                u32,
            )
                -> Ptr<PathingRequestClosePoint>
        ),
        entry!(
            0x006e3fb0,
            fn_006e3fb0(
                Ptr<PathingRequestClosePoint>,
                Ptr<PathingRequestClosePoint>,
            ) -> Ptr<PathingRequestClosePoint>
        ),
        entry!(
            0x006e4030,
            pathing_request_close_point_copy_to(
                Ptr<PathingRequestClosePoint>,
                Ptr<PathingRequestClosePoint>,
            )
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn silent(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    fn calls(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.clone().unwrap()
    }

    #[test]
    fn constructor_sets_the_vtable_and_the_default_distances() {
        let mut e = Engine::new();
        silent(&mut e, &[PATHING_REQUEST_CONSTRUCT]);
        e.map(DEFAULT_MIN_DISTANCE, 4);
        e.map(DEFAULT_MAX_DISTANCE, 4);
        e.set_global(DEFAULT_MIN_DISTANCE, 128.0f32);
        // A signaling NaN comes out of FLD/FSTP quiet.
        e.set_global(DEFAULT_MAX_DISTANCE, f32::from_bits(0x7f80_0001));
        let this: Ptr<PathingRequestClosePoint> = e.new_object();
        e.call_log = Some(vec![]);
        let result = e
            .call(0x006e3f30, &args![this])
            .ptr::<PathingRequestClosePoint>();
        assert_eq!(result, this);
        assert_eq!(calls(&e)[1], (PATHING_REQUEST_CONSTRUCT, vec![this.addr()]));
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(e.get(this, PathingRequestClosePoint::fMinDistance), 128.0);
        assert_eq!(e.mem.u32(this.addr() + 0xB4), 0x7fc0_0001);
    }

    #[test]
    fn destructor_frees_only_when_asked() {
        let mut e = Engine::new();
        silent(&mut e, &[PATHING_REQUEST_DESTROY, SIZED_DELETE]);
        let this: Ptr<PathingRequestClosePoint> = e.new_object();
        e.call_log = Some(vec![]);
        let result = e
            .call(0x006e3f70, &args![this, 0u32])
            .ptr::<PathingRequestClosePoint>();
        assert_eq!(result, this);
        assert_eq!(calls(&e).len(), 2);
        e.call_log = Some(vec![]);
        e.call(0x006e3f70, &args![this, 3u32]);
        assert_eq!(calls(&e)[2], (SIZED_DELETE, vec![this.addr(), 0xb8]));
    }

    #[test]
    fn copy_constructor_takes_the_distances_from_the_getters() {
        let mut e = Engine::new();
        silent(&mut e, &[PATHING_REQUEST_COPY_CONSTRUCT]);
        e.register(GET_MIN_DISTANCE, |_, _| Ret {
            st0: 10.5,
            ..Ret::default()
        });
        e.register(GET_MAX_DISTANCE, |_, _| Ret {
            st0: 99.0,
            ..Ret::default()
        });
        let this: Ptr<PathingRequestClosePoint> = e.new_object();
        let other: Ptr<PathingRequestClosePoint> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x006e3fb0, &args![this, other]);
        assert_eq!(
            calls(&e)[1],
            (
                PATHING_REQUEST_COPY_CONSTRUCT,
                vec![this.addr(), other.addr()]
            )
        );
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(e.get(this, PathingRequestClosePoint::fMinDistance), 10.5);
        assert_eq!(e.get(this, PathingRequestClosePoint::fMaxDistance), 99.0);
    }

    #[test]
    fn copy_to_passes_the_distances_as_bit_patterns() {
        let mut e = Engine::new();
        silent(
            &mut e,
            &[PATHING_REQUEST_COPY_TO, SET_MIN_DISTANCE, SET_MAX_DISTANCE],
        );
        let this: Ptr<PathingRequestClosePoint> = e.new_object();
        let destination: Ptr<PathingRequestClosePoint> = e.new_object();
        e.set(this, PathingRequestClosePoint::fMinDistance, 2.0);
        e.set(this, PathingRequestClosePoint::fMaxDistance, 4.0);
        e.call_log = Some(vec![]);
        e.call(0x006e4030, &args![this, destination]);
        assert_eq!(
            calls(&e)[1..],
            [
                (
                    PATHING_REQUEST_COPY_TO,
                    vec![this.addr(), destination.addr()]
                ),
                (SET_MIN_DISTANCE, vec![destination.addr(), 2.0f32.to_bits()]),
                (SET_MAX_DISTANCE, vec![destination.addr(), 4.0f32.to_bits()]),
            ]
        );
    }
}
