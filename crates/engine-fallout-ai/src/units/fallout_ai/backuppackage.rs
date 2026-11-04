//! `fallout/ai/backuppackage.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BackUpPackage::pBackUpPoint` (Xbox PDB +0x90; +0x80 on PC, where
/// `TESForm` is 0x10 smaller): an `NiPoint3`.
const BACK_UP_POINT: u32 = 0x80;
/// The zero point the code copies (`011f426c`, three `float`s).
const ZERO_POINT: u32 = 0x011f_426c;
/// `300.0` (`01013dc0`): how far the point lies from the actor, and
/// `200.0` (`0102226c`): the navmesh search radius.
const BACK_UP_DISTANCE: u32 = 0x0101_3dc0;
const NAVMESH_RADIUS: u32 = 0x0102_226c;
/// The point arithmetic (all `thiscall` on a point): difference
/// (`00439ef0(result, other)`), normalise (`004a0c10`), scale
/// (`0045bb20(result, float)`), sum (`00439e90(result, other)`) and the
/// copy `004390c0(other)`.
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
const POINT_NORMALIZE: u32 = 0x004a_0c10;
const POINT_SCALE: u32 = 0x0045_bb20;
const POINT_ADD: u32 = 0x0043_9e90;
const POINT_ASSIGN: u32 = 0x0043_90c0;
/// The actor's cell (`008d6f30`), `TESObjectREFR::GetWorldSpace` (engine
/// map; `00575d70`) and `Pathing::FindPointOnNavMesh` (engine map;
/// `006d7110`, `cdecl(worldspace, cell, point, radius, out)`).
const ACTOR_CELL: u32 = 0x008d_6f30;
const GET_WORLD_SPACE: u32 = 0x0057_5d70;
const FIND_POINT_ON_NAVMESH: u32 = 0x006d_7110;
/// `operator new` (`00401000`, `cdecl(size)`).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// The package calls (engine map names): `TESPackage::SetPackType`
/// (`00670fc0`), `00826b40(1)` and `00826b90(1)`, `PackageLocation`'s
/// constructor (`0067f030`, 0x0C bytes), `SetLocReference` (`0067f3c0`),
/// `SetLocCell` (`0067f410`), `0067f1f0(radius)`, `SetPackageLocation`
/// (`00671d30`), the location's deleting destructor (`00670b30(flags)`),
/// `PackageTarget`'s constructor (`0067ff70`, 0x10 bytes),
/// `SetPackageTarget` (`00672fc0`), the target's deleting destructor
/// (`007b3fa0(flags)`), `00984f60(0x30)`, the package's target (`00671d10`),
/// `PackageTarget::SetTargType` (`006800b0`), `SetTargReference`
/// (`00680110`) and `TESPackage::SetIsCreated` (`00674d70`).
const SET_PACK_TYPE: u32 = 0x0067_0fc0;
const PACKAGE_FLAG_A: u32 = 0x0082_6b40;
const PACKAGE_FLAG_B: u32 = 0x0082_6b90;
const LOCATION_CONSTRUCT: u32 = 0x0067_f030;
const SET_LOC_REFERENCE: u32 = 0x0067_f3c0;
const SET_LOC_CELL: u32 = 0x0067_f410;
const SET_LOC_RADIUS: u32 = 0x0067_f1f0;
const SET_PACKAGE_LOCATION: u32 = 0x0067_1d30;
const LOCATION_DELETE: u32 = 0x0067_0b30;
const TARGET_CONSTRUCT: u32 = 0x0067_ff70;
const SET_PACKAGE_TARGET: u32 = 0x0067_2fc0;
const TARGET_DELETE: u32 = 0x007b_3fa0;
const PACKAGE_SET_WORD: u32 = 0x0098_4f60;
const PACKAGE_TARGET: u32 = 0x0067_1d10;
const SET_TARG_TYPE: u32 = 0x0068_00b0;
const SET_TARG_REFERENCE: u32 = 0x0068_0110;
const SET_IS_CREATED: u32 = 0x0067_4d70;
/// `00425fd0` (`thiscall`, `bool`) on the cell argument: chooses between
/// the two point helpers.
const CELL_TEST: u32 = 0x0042_5fd0;

/// Copies the three words at `from` to `to`.
fn copy_point(e: &mut Engine, from: u32, to: u32) {
    for offset in [0, 4, 8] {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, word);
    }
}

/// The two point helpers' shared body (their frames have the same layout;
/// `at(k)` is the game's `EBP - k`): the zero point, or, with both an
/// `actor` and a `reference`, the point 300 units from the actor away from
/// the reference (their slot 0x1F4 positions), moved onto the navmesh near
/// it (`FindPointOnNavMesh`, radius 200); `assign` adds the final
/// `004390c0`.
fn back_up_point(e: &mut Engine, out: u32, actor: u32, reference: u32, assign: bool) -> u32 {
    e.with_stack(0x50, |e, frame| {
        let at = |k: u32| frame.addr() + 0x50 - k;
        copy_point(e, ZERO_POINT, at(0xc));
        if reference != 0 && actor != 0 {
            let position = e.vcall(reference, 0x1f4, &[]).u32();
            copy_point(e, position, at(0x38));
            let position = e.vcall(actor, 0x1f4, &[]).u32();
            copy_point(e, position, at(0x2c));
            let distance = x87_quiet_bits(e.global::<u32>(BACK_UP_DISTANCE));
            e.mem.set_u32(at(0x20), distance);
            e.call(POINT_SUBTRACT, &args![at(0x2c), at(0x1c), at(0x38)]);
            e.call(POINT_NORMALIZE, &args![at(0x1c)]);
            let distance = x87_quiet_bits(e.mem.u32(at(0x20)));
            let scaled = e
                .call(POINT_SCALE, &args![at(0x1c), at(0x44), distance])
                .u32();
            let sum = e.call(POINT_ADD, &args![at(0x2c), at(0x50), scaled]).u32();
            copy_point(e, sum, at(0x1c));
            copy_point(e, at(0x1c), at(0xc));
            let cell = e.call(ACTOR_CELL, &args![actor]).u32();
            e.mem.set_u32(at(0x10), cell);
            let radius = x87_quiet_bits(e.global::<u32>(NAVMESH_RADIUS));
            let world = e.call(GET_WORLD_SPACE, &args![actor]).u32();
            e.call(
                FIND_POINT_ON_NAVMESH,
                &args![world, cell, at(0x1c), radius, at(0xc)],
            );
            if assign {
                e.call(POINT_ASSIGN, &args![at(0xc), at(0x1c)]);
            }
        }
        copy_point(e, at(0xc), out);
        out
    })
}

// Translated from 009ed0d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009ed0d0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `BackUpPackage::InitializeBackUpPackage` (Xbox PDB; `thiscall(actor,
/// cell, reference)`): package type 0x27, `00826b40(1)`, `00826b90(1)`;
/// with an `actor`: a new `PackageLocation` on the actor (no cell and no
/// reference), else on the reference, else on the cell, radius 0x32; the
/// back-up point at +0x80 is zero, or with a cell the point of
/// [`fn_009ed380`] (when `00425fd0` holds on the cell) or [`fn_009ed4b0`];
/// the location and a new `PackageTarget` are set (each copied, then
/// deleted), `00984f60(0x30)`, the target's type 0 and reference `actor`.
/// Always `SetIsCreated(1)`. A failed allocation leaves a null location or
/// target, which the calls then get.
pub fn back_up_package_initialize_back_up_package(
    e: &mut Engine,
    this: Ptr,
    actor: u32,
    cell: u32,
    reference: u32,
) {
    e.call(SET_PACK_TYPE, &args![this, 0x27u32]);
    e.call(PACKAGE_FLAG_A, &args![this, 1u32]);
    e.call(PACKAGE_FLAG_B, &args![this, 1u32]);
    if actor != 0 {
        let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
        let location = if block != 0 {
            e.call(LOCATION_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        if cell == 0 && reference == 0 {
            e.call(SET_LOC_REFERENCE, &args![location, actor]);
        } else if reference != 0 {
            e.call(SET_LOC_REFERENCE, &args![location, reference]);
        } else if cell != 0 {
            e.call(SET_LOC_CELL, &args![location, cell]);
        }
        e.call(SET_LOC_RADIUS, &args![location, 0x32u32]);
        let point = this.addr() + BACK_UP_POINT;
        copy_point(e, ZERO_POINT, point);
        if cell != 0 {
            let found = e.with_stack(0xc, |e, out| {
                let result = if e.call(CELL_TEST, &args![cell]).bool() {
                    fn_009ed380(e, out.addr(), actor, reference)
                } else {
                    fn_009ed4b0(e, out.addr(), actor, reference)
                };
                [
                    e.mem.u32(result),
                    e.mem.u32(result + 4),
                    e.mem.u32(result + 8),
                ]
            });
            for (i, word) in found.into_iter().enumerate() {
                e.mem.set_u32(point + 4 * i as u32, word);
            }
        }
        e.call(SET_PACKAGE_LOCATION, &args![this, location]);
        if location != 0 {
            e.call(LOCATION_DELETE, &args![location, 1u32]);
        }
        let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
        let target = if block != 0 {
            e.call(TARGET_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        e.call(SET_PACKAGE_TARGET, &args![this, target]);
        if target != 0 {
            e.call(TARGET_DELETE, &args![target, 1u32]);
        }
        e.call(PACKAGE_SET_WORD, &args![this, 0x30u32]);
        let package_target = e.call(PACKAGE_TARGET, &args![this]).u32();
        e.call(SET_TARG_TYPE, &args![package_target, 0u32]);
        let package_target = e.call(PACKAGE_TARGET, &args![this]).u32();
        e.call(SET_TARG_REFERENCE, &args![package_target, actor]);
    }
    e.call(SET_IS_CREATED, &args![this, 1u32]);
}

// Translated from 009ed380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl(out, actor, reference)`): the back-up
/// point (see `back_up_point`), written to `out`, which is returned.
pub fn fn_009ed380(e: &mut Engine, out: u32, actor: u32, reference: u32) -> u32 {
    back_up_point(e, out, actor, reference, false)
}

// Translated from 009ed4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl(out, actor, reference)`): as
/// [`fn_009ed380`], with `004390c0` from the moved point into the result
/// after the navmesh search.
pub fn fn_009ed4b0(e: &mut Engine, out: u32, actor: u32, reference: u32) -> u32 {
    back_up_point(e, out, actor, reference, true)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x009e_d0d0,
            back_up_package_initialize_back_up_package(Ptr, u32, u32, u32)
        ),
        entry!(0x009e_d380, fn_009ed380(u32, u32, u32) -> u32),
        entry!(0x009e_d4b0, fn_009ed4b0(u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const VTABLE: u32 = 0x0200_0000;
    const SLOT_POSITION: u32 = 0x0300_01f4;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn point(e: &Engine, at: u32) -> [f32; 3] {
        [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)]
    }

    /// A reference whose slot 0x1F4 points at its +4 position.
    fn reference(e: &mut Engine, x: f32, y: f32) -> u32 {
        let p = e.mem.alloc(0x20);
        e.mem.set_u32(p, VTABLE);
        e.mem.set_f32(p + 4, x);
        e.mem.set_f32(p + 8, y);
        p
    }

    /// The point code as plain float arithmetic.
    fn engine() -> Engine {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x1f8 / 4];
        slots[0x1f4 / 4] = SLOT_POSITION;
        e.put_vtable(VTABLE, &slots);
        e.map(0x011f_4000, 0x1000);
        e.map(0x0101_3000, 0x1000);
        e.map(0x0102_2000, 0x1000);
        e.set_global(BACK_UP_DISTANCE, 300.0f32);
        e.set_global(NAVMESH_RADIUS, 200.0f32);
        e.register(SLOT_POSITION, |_, a| (a[0] + 4).into_ret());
        fn three(e: &Engine, at: u32) -> [f32; 3] {
            [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)]
        }
        fn put(e: &mut Engine, at: u32, p: [f32; 3]) {
            for (i, v) in p.into_iter().enumerate() {
                e.mem.set_f32(at + 4 * i as u32, v);
            }
        }
        e.register(POINT_SUBTRACT, |e, a| {
            let (p, q) = (three(e, a[0]), three(e, a[2]));
            put(e, a[1], [p[0] - q[0], p[1] - q[1], p[2] - q[2]]);
            a[1].into_ret()
        });
        e.register(POINT_NORMALIZE, |e, a| {
            let p = three(e, a[0]);
            let length = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
            put(e, a[0], [p[0] / length, p[1] / length, p[2] / length]);
            Ret::default()
        });
        e.register(POINT_SCALE, |e, a| {
            let (p, s) = (three(e, a[0]), f32::from_bits(a[2]));
            put(e, a[1], [p[0] * s, p[1] * s, p[2] * s]);
            a[1].into_ret()
        });
        e.register(POINT_ADD, |e, a| {
            let (p, q) = (three(e, a[0]), three(e, a[2]));
            put(e, a[1], [p[0] + q[0], p[1] + q[1], p[2] + q[2]]);
            a[1].into_ret()
        });
        e.register(POINT_ASSIGN, |e, a| {
            let p = three(e, a[1]);
            put(e, a[0], [p[0], p[1], p[2] + 1.0]);
            Ret::default()
        });
        e.register(ACTOR_CELL, |_, _| 0x7700u32.into_ret());
        e.register(GET_WORLD_SPACE, |_, _| 0x7800u32.into_ret());
        e.register(FIND_POINT_ON_NAVMESH, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn the_point_lies_away_from_the_reference() {
        let mut e = engine();
        let actor = reference(&mut e, 100.0, 0.0);
        let other = reference(&mut e, 0.0, 0.0);
        let out = e.mem.alloc(12);
        assert_eq!(e.call(0x009e_d380, &args![out, actor, other]).u32(), out);
        assert_eq!(point(&e, out), [400.0, 0.0, 0.0]);
        let search = calls(&e, FIND_POINT_ON_NAVMESH);
        assert_eq!(search[0][..2], [0x7800, 0x7700]);
        assert_eq!(search[0][3], 200.0f32.to_bits());
        assert!(calls(&e, POINT_ASSIGN).is_empty());
        // 009ed4b0 adds the final copy (here: z + 1).
        e.call(0x009e_d4b0, &args![out, actor, other]);
        assert_eq!(point(&e, out), [400.0, 0.0, 1.0]);
        // Without a reference: the zero point.
        e.mem.set_f32(ZERO_POINT, 7.0);
        e.call(0x009e_d380, &args![out, actor, 0u32]);
        assert_eq!(point(&e, out), [7.0, 0.0, 0.0]);
        e.call(0x009e_d4b0, &args![out, 0u32, other]);
        assert_eq!(point(&e, out), [7.0, 0.0, 0.0]);
    }

    #[test]
    fn initialize_sets_location_point_and_target() {
        let mut e = engine();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(LOCATION_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(TARGET_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(PACKAGE_TARGET, |_, _| 0x9900u32.into_ret());
        e.register(CELL_TEST, |e, a| e.mem.u8(a[0]).into_ret());
        for addr in [
            SET_PACK_TYPE,
            PACKAGE_FLAG_A,
            PACKAGE_FLAG_B,
            SET_LOC_REFERENCE,
            SET_LOC_CELL,
            SET_LOC_RADIUS,
            SET_PACKAGE_LOCATION,
            LOCATION_DELETE,
            SET_PACKAGE_TARGET,
            TARGET_DELETE,
            PACKAGE_SET_WORD,
            SET_TARG_TYPE,
            SET_TARG_REFERENCE,
            SET_IS_CREATED,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x90);
        // No actor: only the type, the flags and SetIsCreated.
        e.call(0x009e_d0d0, &args![this, 0u32, 0u32, 0u32]);
        assert_eq!(calls(&e, SET_PACK_TYPE), vec![vec![this, 0x27]]);
        assert_eq!(calls(&e, SET_IS_CREATED), vec![vec![this, 1]]);
        assert!(calls(&e, OPERATOR_NEW).is_empty());
        // An actor alone: the location is on the actor.
        let actor = reference(&mut e, 100.0, 0.0);
        e.call_log = Some(vec![]);
        e.call(0x009e_d0d0, &args![this, actor, 0u32, 0u32]);
        let location = calls(&e, LOCATION_CONSTRUCT)[0][0];
        assert_eq!(calls(&e, SET_LOC_REFERENCE), vec![vec![location, actor]]);
        assert_eq!(calls(&e, SET_LOC_RADIUS), vec![vec![location, 0x32]]);
        assert_eq!(calls(&e, SET_PACKAGE_LOCATION), vec![vec![this, location]]);
        assert_eq!(calls(&e, LOCATION_DELETE), vec![vec![location, 1]]);
        let target = calls(&e, TARGET_CONSTRUCT)[0][0];
        assert_eq!(calls(&e, SET_PACKAGE_TARGET), vec![vec![this, target]]);
        assert_eq!(calls(&e, PACKAGE_SET_WORD), vec![vec![this, 0x30]]);
        assert_eq!(calls(&e, SET_TARG_TYPE), vec![vec![0x9900, 0]]);
        assert_eq!(calls(&e, SET_TARG_REFERENCE), vec![vec![0x9900, actor]]);
        assert_eq!(point(&e, this + BACK_UP_POINT), [0.0, 0.0, 0.0]);
        // A cell (00425fd0 false) and a reference: the location is on the
        // reference and the point comes from 009ed4b0.
        let cell = e.mem.alloc(4);
        let other = reference(&mut e, 0.0, 0.0);
        e.call_log = Some(vec![]);
        e.call(0x009e_d0d0, &args![this, actor, cell, other]);
        assert_eq!(calls(&e, SET_LOC_REFERENCE)[0][1], other);
        assert_eq!(point(&e, this + BACK_UP_POINT), [400.0, 0.0, 1.0]);
        // A cell alone with 00425fd0 true: the location is on the cell and
        // the point comes from 009ed380 (zero, there is no reference).
        e.mem.set_u8(cell, 1);
        e.call_log = Some(vec![]);
        e.call(0x009e_d0d0, &args![this, actor, cell, 0u32]);
        assert_eq!(calls(&e, SET_LOC_CELL)[0][1], cell);
        assert_eq!(point(&e, this + BACK_UP_POINT), [0.0, 0.0, 0.0]);
        assert!(calls(&e, POINT_ASSIGN).is_empty());
    }
}
