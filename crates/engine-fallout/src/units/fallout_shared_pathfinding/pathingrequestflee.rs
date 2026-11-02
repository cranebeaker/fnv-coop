//! `fallout shared/pathfinding/pathingrequestflee.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006e53c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestFlee` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `006dad70` and frees 0xb4 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn pathing_request_flee_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x006d_ad70, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0xb4u32]);
    }
    this
}

// ---- Batch n032: the flee request --------------------------------------------

layout! {
    /// `PathingRequestFlee` (Xbox PDB), 0xB4 bytes: a `PathingRequest`
    /// (0xB0 bytes) and the flee distance.
    pub struct PathingRequestFlee: 0xB4 {
        /// `fMinFleeDistance` (Xbox PDB).
        0xB0 fMinFleeDistance: f32,
    }
}

/// The `PathingRequestFlee` vtable (by its RTTI).
const FLEE_VTABLE: u32 = 0x0106_d300;
/// `PathingRequest::PathingRequest`, its copy constructor `_ov2`,
/// `CopyTo`, `SaveGame` and `LoadGame` (Xbox PDB), in `pathingrequest.cpp`.
const REQUEST_CONSTRUCT: u32 = 0x006e_2420;
const REQUEST_COPY_CONSTRUCT: u32 = 0x006e_26a0;
const REQUEST_COPY_TO: u32 = 0x006e_2750;
const REQUEST_SAVE_GAME: u32 = 0x006e_3230;
const REQUEST_LOAD_GAME: u32 = 0x006e_34f0;
/// The default flee distance (`float`, `0101e704`).
const DEFAULT_FLEE_DISTANCE: u32 = 0x0101_e704;
/// `006d2c20` (`thiscall`): the other request's word at +0xB0 as a float
/// in ST0 (its flee distance).
const GET_FLEE_DISTANCE: u32 = 0x006d_2c20;
/// `BGSSaveGameBuffer` raw save (`00865e50`: data, size, 0) and
/// `BGSLoadGameBuffer` raw load (`00864980`: data, size).
const SAVE_DATA: u32 = 0x0086_5e50;
const LOAD_DATA: u32 = 0x0086_4980;

// Translated from 006e5390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestFlee::PathingRequestFlee` (Xbox PDB): the
/// `PathingRequest` base, the vtable and the default flee distance
/// (through the x87 stack). Returns `this`.
pub fn pathing_request_flee_pathing_request_flee(
    e: &mut Engine,
    this: Ptr<PathingRequestFlee>,
) -> Ptr<PathingRequestFlee> {
    e.call(REQUEST_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), FLEE_VTABLE);
    let distance = x87_quiet_bits(e.mem.u32(DEFAULT_FLEE_DISTANCE));
    e.mem.set_u32(this.addr() + 0xb0, distance);
    this
}

// Translated from 006e5400 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e5400 - the C++ exception frame (FS:[0]) that destroys the base on unwinding is not translated
/// The `PathingRequestFlee` copy constructor (by its body): the
/// `PathingRequest` copy constructor, the vtable, then the other's flee
/// distance (`006d2c20`, stored as a `float`). Returns `this`.
pub fn fn_006e5400(
    e: &mut Engine,
    this: Ptr<PathingRequestFlee>,
    other: Ptr,
) -> Ptr<PathingRequestFlee> {
    e.call(REQUEST_COPY_CONSTRUCT, &args![this, other]);
    e.mem.set_u32(this.addr(), FLEE_VTABLE);
    let distance = e.call(GET_FLEE_DISTANCE, &args![other]).f64();
    e.mem.set_u32(this.addr() + 0xb0, x87_store32(distance));
    this
}

// Translated from 006e5470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestFlee::CopyTo` (Xbox PDB): the `PathingRequest` part,
/// then the flee distance (through the x87 stack).
pub fn pathing_request_flee_copy_to(e: &mut Engine, this: Ptr<PathingRequestFlee>, other: Ptr) {
    e.call(REQUEST_COPY_TO, &args![this, other]);
    let distance = x87_quiet_bits(e.mem.u32(this.addr() + 0xb0));
    e.mem.set_u32(other.addr() + 0xb0, distance);
}

// Translated from 006e54b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestFlee::SaveGame` (Xbox PDB): the `PathingRequest` part,
/// then the flee distance (4 bytes).
pub fn pathing_request_flee_save_game(e: &mut Engine, this: Ptr<PathingRequestFlee>, buffer: u32) {
    e.call(REQUEST_SAVE_GAME, &args![this, buffer]);
    e.call(SAVE_DATA, &args![buffer, this.addr() + 0xb0, 4u32, 0u32]);
}

// Translated from 006e54e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestFlee::LoadGame` (Xbox PDB): the `PathingRequest` part,
/// then the flee distance (4 bytes).
pub fn pathing_request_flee_load_game(e: &mut Engine, this: Ptr<PathingRequestFlee>, buffer: u32) {
    e.call(REQUEST_LOAD_GAME, &args![this, buffer]);
    e.call(LOAD_DATA, &args![buffer, this.addr() + 0xb0, 4u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006e_53c0, pathing_request_flee_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x006e_5390,
            pathing_request_flee_pathing_request_flee(
                Ptr<PathingRequestFlee>,
            ) -> Ptr<PathingRequestFlee>
        ),
        entry!(
            0x006e_5400,
            fn_006e5400(Ptr<PathingRequestFlee>, Ptr) -> Ptr<PathingRequestFlee>
        ),
        entry!(
            0x006e_5470,
            pathing_request_flee_copy_to(Ptr<PathingRequestFlee>, Ptr)
        ),
        entry!(
            0x006e_54b0,
            pathing_request_flee_save_game(Ptr<PathingRequestFlee>, u32)
        ),
        entry!(
            0x006e_54e0,
            pathing_request_flee_load_game(Ptr<PathingRequestFlee>, u32)
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
    fn pathing_request_flee_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x006e_53c0, 0x006d_ad70, 0xb4);
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

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn constructor_sets_the_vtable_and_distance() {
        let mut e = Engine::new();
        stub(&mut e, &[REQUEST_CONSTRUCT]);
        e.map(DEFAULT_FLEE_DISTANCE, 4);
        e.mem.set_u32(DEFAULT_FLEE_DISTANCE, 0x7f80_0001);
        let this = e.mem.alloc(0xb4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e_5390, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, REQUEST_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), FLEE_VTABLE);
        assert_eq!(e.mem.u32(this + 0xb0), 0x7fc0_0001);
    }

    #[test]
    fn copy_constructor_takes_the_other_distance() {
        let mut e = Engine::new();
        stub(&mut e, &[REQUEST_COPY_CONSTRUCT]);
        e.register(GET_FLEE_DISTANCE, |_, _| Ret {
            st0: 1.0 / 3.0,
            ..Ret::default()
        });
        let this = e.mem.alloc(0xb4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e_5400, &args![this, 0x900u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, REQUEST_COPY_CONSTRUCT), vec![vec![this, 0x900]]);
        assert_eq!(calls(&log, GET_FLEE_DISTANCE), vec![vec![0x900]]);
        assert_eq!(e.mem.u32(this), FLEE_VTABLE);
        assert_eq!(e.mem.f32(this + 0xb0), (1.0f64 / 3.0) as f32);
    }

    #[test]
    fn copy_to_save_and_load_carry_the_distance() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                REQUEST_COPY_TO,
                REQUEST_SAVE_GAME,
                REQUEST_LOAD_GAME,
                SAVE_DATA,
                LOAD_DATA,
            ],
        );
        let this = e.mem.alloc(0xb4);
        let other = e.mem.alloc(0xb4);
        e.mem.set_u32(this + 0xb0, 0x7f80_0002);
        e.call_log = Some(vec![]);
        e.call(0x006e_5470, &args![this, other]);
        e.call(0x006e_54b0, &args![this, 0x77u32]);
        e.call(0x006e_54e0, &args![this, 0x77u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(other + 0xb0), 0x7fc0_0002);
        assert_eq!(calls(&log, REQUEST_COPY_TO), vec![vec![this, other]]);
        assert_eq!(calls(&log, REQUEST_SAVE_GAME), vec![vec![this, 0x77]]);
        assert_eq!(calls(&log, REQUEST_LOAD_GAME), vec![vec![this, 0x77]]);
        assert_eq!(calls(&log, SAVE_DATA), vec![vec![0x77, this + 0xb0, 4, 0]]);
        assert_eq!(calls(&log, LOAD_DATA), vec![vec![0x77, this + 0xb0, 4]]);
    }
}
