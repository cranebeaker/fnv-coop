//! `fallout shared/pathfinding/pathingrequestsafestraightline.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006e5fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestSafeStraightLine` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `006d3b20` and frees 0xbc bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn pathing_request_safe_straight_line_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x006d_3b20, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0xbcu32]);
    }
    this
}

// ---- Batch n032: the safe straight-line request -------------------------------

layout! {
    /// `PathingRequestSafeStraightLine` (Xbox PDB), 0xBC bytes: a
    /// `PathingRequestClosePoint` (0xB8 bytes) and the target.
    pub struct PathingRequestSafeStraightLine: 0xBC {
        /// `pTarget` (Xbox PDB): `TESObjectREFR*` (saved as its form id).
        0xB8 pTarget: u32,
    }
}

/// The `PathingRequestSafeStraightLine` vtable (by its RTTI).
const SAFE_STRAIGHT_LINE_VTABLE: u32 = 0x0106_d414;
/// `PathingRequestClosePoint::PathingRequestClosePoint` (Xbox PDB) and its
/// copy constructor (`006e3fb0`, `RET 4`).
const CLOSE_POINT_CONSTRUCT: u32 = 0x006e_3f30;
const CLOSE_POINT_COPY_CONSTRUCT: u32 = 0x006e_3fb0;
/// `PathingRequest::CopyTo` (Xbox PDB).
const REQUEST_COPY_TO: u32 = 0x006e_2750;
/// `PathingRequestOptimalLocation::SaveGame` / `LoadGame` (engine map; the
/// bodies that save and load +0xB0 and +0xB4 after the `PathingRequest`).
const DISTANCES_SAVE_GAME: u32 = 0x006e_5f00;
const DISTANCES_LOAD_GAME: u32 = 0x006e_5f50;
/// `004036b0` (`thiscall`): the other request's word at +0xB8.
const GET_TARGET: u32 = 0x0040_36b0;
/// `BGSSaveGameBuffer::SaveFormID_ov2` (engine map), `thiscall`, `RET 8`:
/// (form, 0); `BGSLoadGameBuffer::LoadFormID_ov2` (engine map), `RET 4`.
const SAVE_FORM: u32 = 0x0086_5df0;
const LOAD_FORM_ID: u32 = 0x0086_48e0;
/// `004839c0` (`cdecl`): the form of a form id; `__RTDynamicCast`
/// (`00ec43fb`, `cdecl`: object, 0, source type, target type, 0) with the
/// type descriptors of `TESForm` (`01183028`) and `TESObjectREFR`
/// (`011841cc`).
const LOOKUP_FORM: u32 = 0x0048_39c0;
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;

// Translated from 006e5fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestSafeStraightLine::PathingRequestSafeStraightLine` (Xbox
/// PDB): the `PathingRequestClosePoint` base, the vtable, no target.
/// Returns `this`.
pub fn pathing_request_safe_straight_line_pathing_request_safe_straight_line(
    e: &mut Engine,
    this: Ptr<PathingRequestSafeStraightLine>,
) -> Ptr<PathingRequestSafeStraightLine> {
    e.call(CLOSE_POINT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), SAFE_STRAIGHT_LINE_VTABLE);
    e.set(this, PathingRequestSafeStraightLine::pTarget, 0);
    this
}

// Translated from 006e6010 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e6010 - the C++ exception frame (FS:[0]) that destroys the base on unwinding is not translated
/// The `PathingRequestSafeStraightLine` copy constructor (by its body):
/// the `PathingRequestClosePoint` copy constructor, the vtable, then the
/// other's target (`004036b0`). Returns `this`.
pub fn fn_006e6010(
    e: &mut Engine,
    this: Ptr<PathingRequestSafeStraightLine>,
    other: Ptr,
) -> Ptr<PathingRequestSafeStraightLine> {
    e.call(CLOSE_POINT_COPY_CONSTRUCT, &args![this, other]);
    e.mem.set_u32(this.addr(), SAFE_STRAIGHT_LINE_VTABLE);
    let target = e.call(GET_TARGET, &args![other]).u32();
    e.set(this, PathingRequestSafeStraightLine::pTarget, target);
    this
}

// Translated from 006e6080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestSafeStraightLine::CopyTo` (Xbox PDB): the
/// `PathingRequest` part (`PathingRequest::CopyTo`, not the close point's),
/// then [`fn_006e60c0`] on the other with this target (which does not
/// store it).
pub fn pathing_request_safe_straight_line_copy_to(
    e: &mut Engine,
    this: Ptr<PathingRequestSafeStraightLine>,
    other: Ptr,
) {
    e.call(REQUEST_COPY_TO, &args![this, other]);
    let target = e.get(this, PathingRequestSafeStraightLine::pTarget);
    fn_006e60c0(e, other.cast(), target);
}

// Translated from 006e60c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`PathingRequestSafeStraightLine`, `RET 4`): the target setter
/// as compiled: it copies `pTarget` (+0xB8) onto itself and never reads
/// its argument.
pub fn fn_006e60c0(e: &mut Engine, this: Ptr<PathingRequestSafeStraightLine>, _unused_1: u32) {
    let target = e.get(this, PathingRequestSafeStraightLine::pTarget);
    e.set(this, PathingRequestSafeStraightLine::pTarget, target);
}

// Translated from 006e60e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestSafeStraightLine::SaveGame` (Xbox PDB): the distances'
/// save (`006e5f00`), then the target as a form (`SaveFormID_ov2` with 0).
pub fn pathing_request_safe_straight_line_save_game(
    e: &mut Engine,
    this: Ptr<PathingRequestSafeStraightLine>,
    buffer: u32,
) {
    e.call(DISTANCES_SAVE_GAME, &args![this, buffer]);
    let target = e.get(this, PathingRequestSafeStraightLine::pTarget);
    e.call(SAVE_FORM, &args![buffer, target, 0u32]);
}

// Translated from 006e6110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestSafeStraightLine::LoadGame` (Xbox PDB): the distances'
/// load (`006e5f50`), then the target's form id into `pTarget`
/// (`LoadFormID_ov2`; [`pathing_request_safe_straight_line_init_load_game`]
/// turns it into the reference).
pub fn pathing_request_safe_straight_line_load_game(
    e: &mut Engine,
    this: Ptr<PathingRequestSafeStraightLine>,
    buffer: u32,
) {
    e.call(DISTANCES_LOAD_GAME, &args![this, buffer]);
    e.call(LOAD_FORM_ID, &args![buffer, this.addr() + 0xb8]);
}

// Translated from 006e6140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestSafeStraightLine::InitLoadGame` (Xbox PDB): a loaded
/// target form id becomes the form (`004839c0`) cast to `TESObjectREFR`
/// (`__RTDynamicCast`); 0 stays 0. The buffer argument is not read.
pub fn pathing_request_safe_straight_line_init_load_game(
    e: &mut Engine,
    this: Ptr<PathingRequestSafeStraightLine>,
    _unused_1: u32,
) {
    let form_id = e.get(this, PathingRequestSafeStraightLine::pTarget);
    let target = if form_id != 0 {
        let form = e.call(LOOKUP_FORM, &args![form_id]).u32();
        e.call(
            DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_REFR, 0u32],
        )
        .u32()
    } else {
        0
    };
    e.set(this, PathingRequestSafeStraightLine::pTarget, target);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006e_5fd0, pathing_request_safe_straight_line_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x006e_5fa0,
            pathing_request_safe_straight_line_pathing_request_safe_straight_line(
                Ptr<PathingRequestSafeStraightLine>,
            )
                -> Ptr<
                PathingRequestSafeStraightLine,
            >
        ),
        entry!(
            0x006e_6010,
            fn_006e6010(
                Ptr<PathingRequestSafeStraightLine>,
                Ptr,
            ) -> Ptr<PathingRequestSafeStraightLine>
        ),
        entry!(
            0x006e_6080,
            pathing_request_safe_straight_line_copy_to(Ptr<PathingRequestSafeStraightLine>, Ptr)
        ),
        entry!(
            0x006e_60c0,
            fn_006e60c0(Ptr<PathingRequestSafeStraightLine>, u32)
        ),
        entry!(
            0x006e_60e0,
            pathing_request_safe_straight_line_save_game(Ptr<PathingRequestSafeStraightLine>, u32)
        ),
        entry!(
            0x006e_6110,
            pathing_request_safe_straight_line_load_game(Ptr<PathingRequestSafeStraightLine>, u32)
        ),
        entry!(
            0x006e_6140,
            pathing_request_safe_straight_line_init_load_game(
                Ptr<PathingRequestSafeStraightLine>,
                u32,
            )
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
    fn pathing_request_safe_straight_line_scalar_deleting_destructor_frees_only_with_flag_bit_zero()
    {
        check_deleting_destructor(0x006e_5fd0, 0x006d_3b20, 0xbc);
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
    fn constructors_set_the_vtable_and_target() {
        let mut e = Engine::new();
        stub(&mut e, &[CLOSE_POINT_CONSTRUCT, CLOSE_POINT_COPY_CONSTRUCT]);
        e.register(GET_TARGET, |_, _| 0x4321u32.into_ret());
        let this = e.mem.alloc(0xbc);
        e.mem.set_u32(this + 0xb8, 9);
        assert_eq!(e.call(0x006e_5fa0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), SAFE_STRAIGHT_LINE_VTABLE);
        assert_eq!(e.mem.u32(this + 0xb8), 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e_6010, &args![this, 0x900u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            order(&log),
            vec![
                (CLOSE_POINT_COPY_CONSTRUCT, vec![this, 0x900]),
                (GET_TARGET, vec![0x900]),
            ]
        );
        assert_eq!(e.mem.u32(this + 0xb8), 0x4321);
    }

    #[test]
    fn copy_to_leaves_the_other_target() {
        let mut e = Engine::new();
        stub(&mut e, &[REQUEST_COPY_TO]);
        let this = e.mem.alloc(0xbc);
        let other = e.mem.alloc(0xbc);
        e.mem.set_u32(this + 0xb8, 0x1111);
        e.mem.set_u32(other + 0xb8, 0x2222);
        e.call_log = Some(vec![]);
        e.call(0x006e_6080, &args![this, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(order(&log), vec![(REQUEST_COPY_TO, vec![this, other])]);
        assert_eq!(e.mem.u32(other + 0xb8), 0x2222);
        e.call(0x006e_60c0, &args![other, 0x3333u32]);
        assert_eq!(e.mem.u32(other + 0xb8), 0x2222);
    }

    #[test]
    fn save_and_load_the_target_form() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                DISTANCES_SAVE_GAME,
                DISTANCES_LOAD_GAME,
                SAVE_FORM,
                LOAD_FORM_ID,
            ],
        );
        let this = e.mem.alloc(0xbc);
        e.mem.set_u32(this + 0xb8, 0x5555);
        e.call_log = Some(vec![]);
        e.call(0x006e_60e0, &args![this, 0x77u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            order(&log),
            vec![
                (DISTANCES_SAVE_GAME, vec![this, 0x77]),
                (SAVE_FORM, vec![0x77, 0x5555, 0]),
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x006e_6110, &args![this, 0x77u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            order(&log),
            vec![
                (DISTANCES_LOAD_GAME, vec![this, 0x77]),
                (LOAD_FORM_ID, vec![0x77, this + 0xb8]),
            ]
        );
    }

    #[test]
    fn init_load_game_resolves_the_target() {
        let mut e = Engine::new();
        e.register(LOOKUP_FORM, |_, a| (a[0] + 0x10).into_ret());
        e.register(DYNAMIC_CAST, |_, a| (a[0] + 1).into_ret());
        let this = e.mem.alloc(0xbc);
        e.mem.set_u32(this + 0xb8, 0x5555);
        e.call_log = Some(vec![]);
        e.call(0x006e_6140, &args![this, 0x77u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            order(&log),
            vec![
                (LOOKUP_FORM, vec![0x5555]),
                (
                    DYNAMIC_CAST,
                    vec![0x5565, 0, TYPE_TES_FORM, TYPE_TES_OBJECT_REFR, 0]
                ),
            ]
        );
        assert_eq!(e.mem.u32(this + 0xb8), 0x5566);
        // No form id: stays 0, nothing looked up.
        e.mem.set_u32(this + 0xb8, 0);
        e.call_log = Some(vec![]);
        e.call(0x006e_6140, &args![this, 0x77u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.mem.u32(this + 0xb8), 0);
    }
}
