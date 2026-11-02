//! `fallout shared/pathfinding/pathingrequestoptimallocation.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006e5d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestOptimalLocation` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `006dad70` and frees 0xc0 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn pathing_request_optimal_location_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x006d_ad70, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0xc0u32]);
    }
    this
}

// Translated from 006e5ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A float setter (unnamed in the map; `thiscall`, `(value)`): `value` stored at
/// `this + 0xB4` (through the x87 stack, so a signalling NaN is stored quiet).
pub fn fn_006e5ee0(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_u32(
        this.addr().wrapping_add(0xb4),
        x87_quiet_bits(value.to_bits()),
    );
}

// ---- Batch n032: the optimal-location request ---------------------------------

layout! {
    /// `PathingRequestOptimalLocation` (Xbox PDB), 0xC0 bytes: a
    /// `PathingRequest` (0xB0 bytes) and four distances.
    pub struct PathingRequestOptimalLocation: 0xC0 {
        /// `fMinDistance` (Xbox PDB).
        0xB0 fMinDistance: f32,
        /// `fMaxDistance` (Xbox PDB).
        0xB4 fMaxDistance: f32,
        /// `fAbsoluteMinDistance` (Xbox PDB).
        0xB8 fAbsoluteMinDistance: f32,
        /// `fAbsoluteMaxDistance` (Xbox PDB).
        0xBC fAbsoluteMaxDistance: f32,
    }
}

/// The `PathingRequestOptimalLocation` vtable (by its RTTI).
const OPTIMAL_LOCATION_VTABLE: u32 = 0x0106_d3b8;
/// `PathingRequest::PathingRequest`, its copy constructor `_ov2`,
/// `CopyTo`, `SaveGame` and `LoadGame` (Xbox PDB), in `pathingrequest.cpp`.
const REQUEST_CONSTRUCT: u32 = 0x006e_2420;
const REQUEST_COPY_CONSTRUCT: u32 = 0x006e_26a0;
const REQUEST_COPY_TO: u32 = 0x006e_2750;
const REQUEST_SAVE_GAME: u32 = 0x006e_3230;
const REQUEST_LOAD_GAME: u32 = 0x006e_34f0;
/// The default minimum (`0101e704`) and maximum (`01018bfc`) distances
/// (`float`s), for both pairs.
const DEFAULT_MIN_DISTANCE: u32 = 0x0101_e704;
const DEFAULT_MAX_DISTANCE: u32 = 0x0101_8bfc;
/// The other request's getters (`thiscall`, a float in ST0): the words at
/// +0xB0 (`006d2c20`), +0xB4 (`006d2c40`), +0xB8 (`0048ee50`) and +0xBC
/// (`0099e040`).
const GET_B0: u32 = 0x006d_2c20;
const GET_B4: u32 = 0x006d_2c40;
const GET_B8: u32 = 0x0048_ee50;
const GET_BC: u32 = 0x0099_e040;
/// The other request's setters outside this unit (`thiscall`, `RET 4`,
/// a float through the x87 stack): +0xB0 (`00507610`), +0xB8
/// (`006e4ef0`) and +0xBC (`006e4320`, the cover request's distance
/// setter).
const SET_B0: u32 = 0x0050_7610;
const SET_B8: u32 = 0x006e_4ef0;
const SET_BC: u32 = 0x006e_4320;
/// `BGSSaveGameBuffer` raw save (`00865e50`: data, size, 0) and
/// `BGSLoadGameBuffer` raw load (`00864980`: data, size).
const SAVE_DATA: u32 = 0x0086_5e50;
const LOAD_DATA: u32 = 0x0086_4980;

// Translated from 006e5d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestOptimalLocation::PathingRequestOptimalLocation` (Xbox
/// PDB): the `PathingRequest` base, the vtable, and the default distances
/// (minimum, maximum, absolute minimum, absolute maximum; through the x87
/// stack). Returns `this`.
pub fn pathing_request_optimal_location_pathing_request_optimal_location(
    e: &mut Engine,
    this: Ptr<PathingRequestOptimalLocation>,
) -> Ptr<PathingRequestOptimalLocation> {
    e.call(REQUEST_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), OPTIMAL_LOCATION_VTABLE);
    for (offset, constant) in [
        (0xb0u32, DEFAULT_MIN_DISTANCE),
        (0xb4, DEFAULT_MAX_DISTANCE),
        (0xb8, DEFAULT_MIN_DISTANCE),
        (0xbc, DEFAULT_MAX_DISTANCE),
    ] {
        let value = x87_quiet_bits(e.mem.u32(constant));
        e.mem.set_u32(this.addr() + offset, value);
    }
    this
}

// Translated from 006e5db0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e5db0 - the C++ exception frame (FS:[0]) that destroys the base on unwinding is not translated
/// The `PathingRequestOptimalLocation` copy constructor (by its body): the
/// `PathingRequest` copy constructor, the vtable, then the other's four
/// distances through its getters (each stored as a `float`). Returns
/// `this`.
pub fn fn_006e5db0(
    e: &mut Engine,
    this: Ptr<PathingRequestOptimalLocation>,
    other: Ptr,
) -> Ptr<PathingRequestOptimalLocation> {
    e.call(REQUEST_COPY_CONSTRUCT, &args![this, other]);
    e.mem.set_u32(this.addr(), OPTIMAL_LOCATION_VTABLE);
    for (offset, getter) in [
        (0xb0u32, GET_B0),
        (0xb4, GET_B4),
        (0xb8, GET_B8),
        (0xbc, GET_BC),
    ] {
        let value = e.call(getter, &args![other]).f64();
        e.mem.set_u32(this.addr() + offset, x87_store32(value));
    }
    this
}

// Translated from 006e5e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestOptimalLocation::CopyTo` (Xbox PDB): the
/// `PathingRequest` part, then the four distances through the other's
/// setters (`00507610`, [`fn_006e5ee0`], `006e4ef0`, `006e4320`; through the
/// x87 stack).
pub fn pathing_request_optimal_location_copy_to(
    e: &mut Engine,
    this: Ptr<PathingRequestOptimalLocation>,
    other: Ptr,
) {
    e.call(REQUEST_COPY_TO, &args![this, other]);
    let value = x87_quiet_bits(e.mem.u32(this.addr() + 0xb0));
    e.call(SET_B0, &args![other, value]);
    let value = f32::from_bits(x87_quiet_bits(e.mem.u32(this.addr() + 0xb4)));
    fn_006e5ee0(e, other, value);
    let value = x87_quiet_bits(e.mem.u32(this.addr() + 0xb8));
    e.call(SET_B8, &args![other, value]);
    let value = x87_quiet_bits(e.mem.u32(this.addr() + 0xbc));
    e.call(SET_BC, &args![other, value]);
}

// Translated from 006e5f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestOptimalLocation::SaveGame` (Xbox PDB): the
/// `PathingRequest` part, then `fMinDistance` and `fMaxDistance` (the
/// absolute distances are not saved).
pub fn pathing_request_optimal_location_save_game(
    e: &mut Engine,
    this: Ptr<PathingRequestOptimalLocation>,
    buffer: u32,
) {
    e.call(REQUEST_SAVE_GAME, &args![this, buffer]);
    e.call(SAVE_DATA, &args![buffer, this.addr() + 0xb0, 4u32, 0u32]);
    e.call(SAVE_DATA, &args![buffer, this.addr() + 0xb4, 4u32, 0u32]);
}

// Translated from 006e5f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestOptimalLocation::LoadGame` (Xbox PDB): the
/// `PathingRequest` part, then `fMinDistance` and `fMaxDistance`.
pub fn pathing_request_optimal_location_load_game(
    e: &mut Engine,
    this: Ptr<PathingRequestOptimalLocation>,
    buffer: u32,
) {
    e.call(REQUEST_LOAD_GAME, &args![this, buffer]);
    e.call(LOAD_DATA, &args![buffer, this.addr() + 0xb0, 4u32]);
    e.call(LOAD_DATA, &args![buffer, this.addr() + 0xb4, 4u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006e_5d70, pathing_request_optimal_location_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006e5ee0, fn_006e5ee0(Ptr, f32)),
        entry!(
            0x006e_5d10,
            pathing_request_optimal_location_pathing_request_optimal_location(
                Ptr<PathingRequestOptimalLocation>,
            ) -> Ptr<
                PathingRequestOptimalLocation,
            >
        ),
        entry!(
            0x006e_5db0,
            fn_006e5db0(
                Ptr<PathingRequestOptimalLocation>,
                Ptr,
            ) -> Ptr<PathingRequestOptimalLocation>
        ),
        entry!(
            0x006e_5e60,
            pathing_request_optimal_location_copy_to(Ptr<PathingRequestOptimalLocation>, Ptr)
        ),
        entry!(
            0x006e_5f00,
            pathing_request_optimal_location_save_game(Ptr<PathingRequestOptimalLocation>, u32)
        ),
        entry!(
            0x006e_5f50,
            pathing_request_optimal_location_load_game(Ptr<PathingRequestOptimalLocation>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_deleting_destructor(address: u32, body: u32, size: u32) {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(size));
        e.register(body, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(address, &args![this, flags]).u32(), this.addr());
        }
        let log = e.call_log.take().unwrap();
        let calls_to = |target: u32| {
            log.iter()
                .filter(|(x, _)| *x == target)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(calls_to(body).len(), 4);
        assert_eq!(
            calls_to(0x00aa_1460),
            vec![(0x00aa_1460, vec![this.addr(), size]); 2]
        );
    }

    #[test]
    fn pathing_request_optimal_location_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x006e_5d70, 0x006d_ad70, 0xc0);
    }

    #[test]
    fn fn_006e5ee0_stores_the_float_at_0xb4() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xb8);
        e.call(0x006e5ee0, &args![this, 12.0f32]);
        assert_eq!(e.mem.f32(this + 0xb4), 12.0);
    }
}

#[cfg(test)]
mod n032_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn stub(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    fn order(log: &Log) -> Vec<(u32, Vec<u32>)> {
        log[1..].to_vec()
    }

    #[test]
    fn constructor_sets_the_default_distances() {
        let mut e = Engine::new();
        stub(&mut e, &[REQUEST_CONSTRUCT]);
        e.map(DEFAULT_MIN_DISTANCE, 4);
        e.mem.set_f32(DEFAULT_MIN_DISTANCE, 128.0);
        e.map(DEFAULT_MAX_DISTANCE, 4);
        e.mem.set_u32(DEFAULT_MAX_DISTANCE, 0x7f80_0001);
        let this = e.mem.alloc(0xc0);
        assert_eq!(e.call(0x006e_5d10, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), OPTIMAL_LOCATION_VTABLE);
        assert_eq!(e.mem.f32(this + 0xb0), 128.0);
        assert_eq!(e.mem.u32(this + 0xb4), 0x7fc0_0001);
        assert_eq!(e.mem.f32(this + 0xb8), 128.0);
        assert_eq!(e.mem.u32(this + 0xbc), 0x7fc0_0001);
    }

    #[test]
    fn copy_constructor_reads_the_four_getters() {
        let mut e = Engine::new();
        stub(&mut e, &[REQUEST_COPY_CONSTRUCT]);
        for (getter, value) in [(GET_B0, 1.0), (GET_B4, 2.0), (GET_B8, 3.0), (GET_BC, 0.1)] {
            e.register_double(getter, move |_, _| Ret {
                st0: value,
                ..Ret::default()
            });
        }
        let this = e.mem.alloc(0xc0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e_5db0, &args![this, 0x900u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            order(&log),
            vec![
                (REQUEST_COPY_CONSTRUCT, vec![this, 0x900]),
                (GET_B0, vec![0x900]),
                (GET_B4, vec![0x900]),
                (GET_B8, vec![0x900]),
                (GET_BC, vec![0x900]),
            ]
        );
        assert_eq!(e.mem.u32(this), OPTIMAL_LOCATION_VTABLE);
        assert_eq!(e.mem.f32(this + 0xb4), 2.0);
        assert_eq!(e.mem.f32(this + 0xbc), 0.1f32);
    }

    #[test]
    fn copy_to_uses_the_setters() {
        let mut e = Engine::new();
        stub(&mut e, &[REQUEST_COPY_TO, SET_B0, SET_B8, SET_BC]);
        let this = e.mem.alloc(0xc0);
        let other = e.mem.alloc(0xc0);
        e.mem.set_f32(this + 0xb0, 1.0);
        e.mem.set_u32(this + 0xb4, 0x7f80_0001);
        e.mem.set_f32(this + 0xb8, 3.0);
        e.mem.set_f32(this + 0xbc, 4.0);
        e.call_log = Some(vec![]);
        e.call(0x006e_5e60, &args![this, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            order(&log),
            vec![
                (REQUEST_COPY_TO, vec![this, other]),
                (SET_B0, vec![other, 1.0f32.to_bits()]),
                (SET_B8, vec![other, 3.0f32.to_bits()]),
                (SET_BC, vec![other, 4.0f32.to_bits()]),
            ]
        );
        assert_eq!(e.mem.u32(other + 0xb4), 0x7fc0_0001);
    }

    #[test]
    fn save_and_load_carry_two_distances() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[REQUEST_SAVE_GAME, REQUEST_LOAD_GAME, SAVE_DATA, LOAD_DATA],
        );
        let this = e.mem.alloc(0xc0);
        e.call_log = Some(vec![]);
        e.call(0x006e_5f00, &args![this, 0x77u32]);
        e.call(0x006e_5f50, &args![this, 0x77u32]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<(u32, Vec<u32>)> = log
            .into_iter()
            .filter(|c| c.0 != 0x006e_5f00 && c.0 != 0x006e_5f50)
            .collect();
        assert_eq!(
            calls,
            vec![
                (REQUEST_SAVE_GAME, vec![this, 0x77]),
                (SAVE_DATA, vec![0x77, this + 0xb0, 4, 0]),
                (SAVE_DATA, vec![0x77, this + 0xb4, 4, 0]),
                (REQUEST_LOAD_GAME, vec![this, 0x77]),
                (LOAD_DATA, vec![0x77, this + 0xb0, 4]),
                (LOAD_DATA, vec![0x77, this + 0xb4, 4]),
            ]
        );
    }
}
