//! `fallout shared/pathfinding/pathingrequesthide.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006e55a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestHide` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `006e55e0` and frees 0xdc bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn pathing_request_hide_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x006e_55e0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0xdcu32]);
    }
    this
}

// ---- Batch n032: the hide request --------------------------------------------

layout! {
    /// `PathingRequestHide` (Xbox PDB), 0xDC bytes: a `PathingRequest`
    /// (0xB0 bytes), the threat's `PathingLocation` (+0xB0, 0x28 bytes)
    /// and the hide distance.
    pub struct PathingRequestHide: 0xDC {
        /// `fMinHideDistance` (Xbox PDB).
        0xD8 fMinHideDistance: f32,
    }
}

/// `PathingRequestHide::ThreatLocation` (Xbox PDB), a `PathingLocation`.
const THREAT_LOCATION: u32 = 0xb0;
/// The `PathingRequestHide` vtable (by its RTTI).
const HIDE_VTABLE: u32 = 0x0106_d32c;
/// `PathingRequest::PathingRequest`, its copy constructor `_ov2`, its
/// destructor body (`006e2620`), `CopyTo`, `SaveGame` and `LoadGame`
/// (Xbox PDB), in `pathingrequest.cpp`.
const REQUEST_CONSTRUCT: u32 = 0x006e_2420;
const REQUEST_COPY_CONSTRUCT: u32 = 0x006e_26a0;
const REQUEST_DESTRUCT: u32 = 0x006e_2620;
const REQUEST_COPY_TO: u32 = 0x006e_2750;
const REQUEST_SAVE_GAME: u32 = 0x006e_3230;
const REQUEST_LOAD_GAME: u32 = 0x006e_34f0;
/// `PathingLocation::PathingLocation` (engine map), its destructor
/// (`004ff7e0`) and `PathingLocation::operator=` (engine map, `RET 4`).
const LOCATION_CONSTRUCT: u32 = 0x006d_cbb0;
const LOCATION_DESTRUCT: u32 = 0x004f_f7e0;
const LOCATION_ASSIGN: u32 = 0x006d_cce0;
/// `006d2c00` (`thiscall`): sets bit 1 (`2`) of a location's `uiFlags`
/// (+0x26).
const LOCATION_SET_FLAG: u32 = 0x006d_2c00;
/// The default hide distance (`float`, `0101e704`).
const DEFAULT_HIDE_DISTANCE: u32 = 0x0101_e704;
/// `00984230` (`thiscall`): the other request's threat location
/// (`this + 0xB0`); `00885bd0`: its hide distance (+0xD8) in ST0.
const GET_THREAT_LOCATION: u32 = 0x0098_4230;
const GET_HIDE_DISTANCE: u32 = 0x0088_5bd0;
/// `BGSSaveGameBuffer` raw save (`00865e50`: data, size, 0) and
/// `BGSLoadGameBuffer` raw load (`00864980`: data, size).
const SAVE_DATA: u32 = 0x0086_5e50;
const LOAD_DATA: u32 = 0x0086_4980;

// Translated from 006e5510 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e5510 - the C++ exception frame (FS:[0]) that destroys the constructed parts on unwinding is not translated
/// `PathingRequestHide::PathingRequestHide` (Xbox PDB): the
/// `PathingRequest` base, the vtable, the threat location, the default
/// hide distance (through the x87 stack), and flag `2` on the
/// destination (+0x34, `006d2c00`). Returns `this`.
pub fn pathing_request_hide_pathing_request_hide(
    e: &mut Engine,
    this: Ptr<PathingRequestHide>,
) -> Ptr<PathingRequestHide> {
    e.call(REQUEST_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), HIDE_VTABLE);
    e.call(LOCATION_CONSTRUCT, &args![this.addr() + THREAT_LOCATION]);
    let distance = x87_quiet_bits(e.mem.u32(DEFAULT_HIDE_DISTANCE));
    e.mem.set_u32(this.addr() + 0xd8, distance);
    e.call(LOCATION_SET_FLAG, &args![this.addr() + 0x34]);
    this
}

// Translated from 006e55e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e55e0 - the C++ exception frame (FS:[0]) that runs the base destructor on unwinding is not translated
/// The `PathingRequestHide` destructor body (its deleting destructor's
/// callee): the threat location (`004ff7e0`), then the `PathingRequest`
/// destructor body (`006e2620`). It sets no vtable of its own.
pub fn fn_006e55e0(e: &mut Engine, this: Ptr<PathingRequestHide>) {
    e.call(LOCATION_DESTRUCT, &args![this.addr() + THREAT_LOCATION]);
    e.call(REQUEST_DESTRUCT, &args![this]);
}

// Translated from 006e5640 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e5640 - the C++ exception frame (FS:[0]) that destroys the constructed parts on unwinding is not translated
/// `PathingRequestHide::PathingRequestHide_ov2` (Xbox PDB), the copy
/// constructor: the `PathingRequest` copy constructor, the vtable, the
/// threat location built then assigned from the other's (`00984230`), the
/// other's hide distance (`00885bd0`, stored as a `float`) and flag `2`
/// on the destination. Returns `this`.
pub fn pathing_request_hide_pathing_request_hide_ov2(
    e: &mut Engine,
    this: Ptr<PathingRequestHide>,
    other: Ptr,
) -> Ptr<PathingRequestHide> {
    let threat = this.addr() + THREAT_LOCATION;
    e.call(REQUEST_COPY_CONSTRUCT, &args![this, other]);
    e.mem.set_u32(this.addr(), HIDE_VTABLE);
    e.call(LOCATION_CONSTRUCT, &args![threat]);
    let location = e.call(GET_THREAT_LOCATION, &args![other]).u32();
    e.call(LOCATION_ASSIGN, &args![threat, location]);
    let distance = e.call(GET_HIDE_DISTANCE, &args![other]).f64();
    e.mem.set_u32(this.addr() + 0xd8, x87_store32(distance));
    e.call(LOCATION_SET_FLAG, &args![this.addr() + 0x34]);
    this
}

// Translated from 006e56f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestHide::CopyTo` (Xbox PDB): the `PathingRequest` part, the
/// threat location (`PathingLocation::operator=`) and the hide distance
/// (through the x87 stack).
pub fn pathing_request_hide_copy_to(e: &mut Engine, this: Ptr<PathingRequestHide>, other: Ptr) {
    e.call(REQUEST_COPY_TO, &args![this, other]);
    e.call(
        LOCATION_ASSIGN,
        &args![
            other.addr() + THREAT_LOCATION,
            this.addr() + THREAT_LOCATION
        ],
    );
    let distance = x87_quiet_bits(e.mem.u32(this.addr() + 0xd8));
    e.mem.set_u32(other.addr() + 0xd8, distance);
}

// Translated from 006e5740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestHide::SaveGame` (Xbox PDB): the `PathingRequest` part,
/// the threat location (its vtable slot 0) and the hide distance.
pub fn pathing_request_hide_save_game(e: &mut Engine, this: Ptr<PathingRequestHide>, buffer: u32) {
    e.call(REQUEST_SAVE_GAME, &args![this, buffer]);
    e.vcall(this.addr() + THREAT_LOCATION, 0, &args![buffer]);
    e.call(SAVE_DATA, &args![buffer, this.addr() + 0xd8, 4u32, 0u32]);
}

// Translated from 006e5790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestHide::LoadGame` (Xbox PDB): the `PathingRequest` part,
/// the threat location (its vtable slot +4) and the hide distance.
pub fn pathing_request_hide_load_game(e: &mut Engine, this: Ptr<PathingRequestHide>, buffer: u32) {
    e.call(REQUEST_LOAD_GAME, &args![this, buffer]);
    e.vcall(this.addr() + THREAT_LOCATION, 4, &args![buffer]);
    e.call(LOAD_DATA, &args![buffer, this.addr() + 0xd8, 4u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006e_55a0, pathing_request_hide_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x006e_5510,
            pathing_request_hide_pathing_request_hide(
                Ptr<PathingRequestHide>,
            ) -> Ptr<PathingRequestHide>
        ),
        entry!(0x006e_55e0, fn_006e55e0(Ptr<PathingRequestHide>)),
        entry!(
            0x006e_5640,
            pathing_request_hide_pathing_request_hide_ov2(
                Ptr<PathingRequestHide>,
                Ptr,
            ) -> Ptr<PathingRequestHide>
        ),
        entry!(
            0x006e_56f0,
            pathing_request_hide_copy_to(Ptr<PathingRequestHide>, Ptr)
        ),
        entry!(
            0x006e_5740,
            pathing_request_hide_save_game(Ptr<PathingRequestHide>, u32)
        ),
        entry!(
            0x006e_5790,
            pathing_request_hide_load_game(Ptr<PathingRequestHide>, u32)
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
    fn pathing_request_hide_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x006e_55a0, 0x006e_55e0, 0xdc);
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
    fn constructor_builds_the_threat_and_flags_the_destination() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[REQUEST_CONSTRUCT, LOCATION_CONSTRUCT, LOCATION_SET_FLAG],
        );
        e.map(DEFAULT_HIDE_DISTANCE, 4);
        e.mem.set_f32(DEFAULT_HIDE_DISTANCE, 128.0);
        let this = e.mem.alloc(0xdc);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e_5510, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            order(&log),
            vec![
                (REQUEST_CONSTRUCT, vec![this]),
                (LOCATION_CONSTRUCT, vec![this + 0xb0]),
                (LOCATION_SET_FLAG, vec![this + 0x34]),
            ]
        );
        assert_eq!(e.mem.u32(this), HIDE_VTABLE);
        assert_eq!(e.mem.f32(this + 0xd8), 128.0);
    }

    #[test]
    fn destructor_body_destroys_the_threat_then_the_base() {
        let mut e = Engine::new();
        stub(&mut e, &[LOCATION_DESTRUCT, REQUEST_DESTRUCT]);
        let this = e.mem.alloc(0xdc);
        e.mem.set_u32(this, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x006e_55e0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            order(&log),
            vec![
                (LOCATION_DESTRUCT, vec![this + 0xb0]),
                (REQUEST_DESTRUCT, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this), 0x1234);
    }

    #[test]
    fn copy_constructor_copies_the_threat_and_distance() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                REQUEST_COPY_CONSTRUCT,
                LOCATION_CONSTRUCT,
                LOCATION_ASSIGN,
                LOCATION_SET_FLAG,
            ],
        );
        e.register(GET_THREAT_LOCATION, |_, a| (a[0] + 0xb0).into_ret());
        e.register(GET_HIDE_DISTANCE, |_, _| Ret {
            st0: 0.1,
            ..Ret::default()
        });
        let this = e.mem.alloc(0xdc);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e_5640, &args![this, 0x900u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            order(&log),
            vec![
                (REQUEST_COPY_CONSTRUCT, vec![this, 0x900]),
                (LOCATION_CONSTRUCT, vec![this + 0xb0]),
                (GET_THREAT_LOCATION, vec![0x900]),
                (LOCATION_ASSIGN, vec![this + 0xb0, 0x9b0]),
                (GET_HIDE_DISTANCE, vec![0x900]),
                (LOCATION_SET_FLAG, vec![this + 0x34]),
            ]
        );
        assert_eq!(e.mem.u32(this), HIDE_VTABLE);
        assert_eq!(e.mem.f32(this + 0xd8), 0.1f32);
    }

    #[test]
    fn copy_to_save_and_load() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                REQUEST_COPY_TO,
                LOCATION_ASSIGN,
                REQUEST_SAVE_GAME,
                REQUEST_LOAD_GAME,
                SAVE_DATA,
                LOAD_DATA,
                0x5100,
                0x5104,
            ],
        );
        e.put_vtable(0x5000, &[0x5100, 0x5104]);
        let this = e.mem.alloc(0xdc);
        let other = e.mem.alloc(0xdc);
        e.mem.set_u32(this + 0xb0, 0x5000);
        e.mem.set_u32(this + 0xd8, 0xff80_0001);
        e.call_log = Some(vec![]);
        e.call(0x006e_56f0, &args![this, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            order(&log),
            vec![
                (REQUEST_COPY_TO, vec![this, other]),
                (LOCATION_ASSIGN, vec![other + 0xb0, this + 0xb0]),
            ]
        );
        assert_eq!(e.mem.u32(other + 0xd8), 0xffc0_0001);
        e.call_log = Some(vec![]);
        e.call(0x006e_5740, &args![this, 0x77u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            order(&log),
            vec![
                (REQUEST_SAVE_GAME, vec![this, 0x77]),
                (0x5100, vec![this + 0xb0, 0x77]),
                (SAVE_DATA, vec![0x77, this + 0xd8, 4, 0]),
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x006e_5790, &args![this, 0x77u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            order(&log),
            vec![
                (REQUEST_LOAD_GAME, vec![this, 0x77]),
                (0x5104, vec![this + 0xb0, 0x77]),
                (LOAD_DATA, vec![0x77, this + 0xd8, 4]),
            ]
        );
    }
}
