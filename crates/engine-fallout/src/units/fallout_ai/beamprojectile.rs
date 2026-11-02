//! `fallout/ai/combat/beamprojectile.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// A test on a reference (`00440d80`, `thiscall`, `bool`).
const REFERENCE_TEST: u32 = 0x0044_0d80;
/// The projectile's age (`00885bd0`, `thiscall`, `float` in ST0; the code
/// compares it with 0 and 5 and with `006ca4e0`'s value).
const PROJECTILE_AGE: u32 = 0x0088_5bd0;
/// `Projectile::CheckExplosion` (engine map; `009c3190`, `thiscall(float)`).
const CHECK_EXPLOSION: u32 = 0x009c_3190;
/// The projectile's source (`00874480`, `thiscall`), compared with the
/// player (`011dea3c`).
const PROJECTILE_SOURCE: u32 = 0x0087_4480;
const PLAYER: u32 = 0x011d_ea3c;
/// `0.0` and `5.0` (`double`s the comparisons read).
const ZERO: u32 = 0x0101_2060;
const FIVE: u32 = 0x0102_0998;
/// `00461130` (`thiscall`) on the source's node: the rotation the code
/// multiplies with this +0x94 (`NiMatrix3::operator*`, engine map,
/// `0043f8d0`, `thiscall(result, other)`; a 0x24-byte result).
const NODE_ROTATE: u32 = 0x0046_1130;
const MATRIX_MULTIPLY: u32 = 0x0043_f8d0;
/// The rotation setter on the node of slot 0x1D0 (`0043fa80`,
/// `thiscall(matrix)`) and `Projectile::SetProjectileOrientation` (engine
/// map; `009c6330`, `thiscall(matrix)`).
const NODE_SET_ROTATE: u32 = 0x0043_fa80;
const SET_ORIENTATION: u32 = 0x009c_6330;
/// `0045bb80` (`thiscall`) on the source's node: the position given to the
/// node of slot 0x1D0 (`00440460`, `thiscall(point)`) and to
/// `TESObjectREFR::SetLocationOnReference` (engine map; `00575830`,
/// `thiscall(point)`).
const NODE_TRANSLATE: u32 = 0x0045_bb80;
const NODE_SET_TRANSLATE: u32 = 0x0044_0460;
const SET_LOCATION: u32 = 0x0057_5830;
/// The pointer at +0x150 (`00559450`, `thiscall`: its object), the value
/// `00644660` (`thiscall`, `float` in ST0) and the setter `0051b840`
/// (`thiscall(float)`) on that object.
const POINTER_GET: u32 = 0x0055_9450;
const BEAM_VALUE: u32 = 0x0064_4660;
const OBJECT_SET_VALUE: u32 = 0x0051_b840;
/// `009bc8f0` (`thiscall`), called when the age passes 5 and at the end.
const PROJECTILE_FINISH: u32 = 0x009b_c8f0;
/// `004181e0` (`thiscall`) and `006ca4e0` (`thiscall`, `float` in ST0): the
/// limit the age is compared with.
const LIMIT_OWNER: u32 = 0x0041_81e0;
const LIMIT_VALUE: u32 = 0x006c_a4e0;
/// `00450f90` (`thiscall(word)`), called with 1 on slot 0x1D0's object.
const NODE_CALL: u32 = 0x0045_0f90;

// Translated from 00979330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BeamProjectile::UpdateProjectile` (Xbox PDB; `thiscall(seconds)`):
/// nothing when the test `00440d80` holds. Otherwise: with an age of exactly
/// 0, `Projectile::CheckExplosion(seconds)`; when the source (`00874480`)
/// has a node (slot 0x20C) and slot 0x1D0 gives a node, the source's node
/// rotation times this +0x94 goes to that node and (for the player's
/// projectile only) to the orientation, and the source node's position to
/// that node and this reference; with an object at +0x150, it gets
/// `00644660`'s value; an age above 5 runs `009bc8f0`; and with the byte at
/// +0x90 set, a limit (`004181e0` then `006ca4e0`) at or below the age calls
/// `00450f90(1)` on slot 0x1D0's object (when there is one) and `009bc8f0`.
/// The age is read again for each test, as the exe does; NaN compares false.
pub fn beam_projectile_update_projectile(e: &mut Engine, this: Ptr, seconds: f32) {
    if e.call(REFERENCE_TEST, &args![this]).bool() {
        return;
    }
    let zero: f64 = e.global(ZERO);
    if e.call(PROJECTILE_AGE, &args![this]).f64() == zero {
        e.call(CHECK_EXPLOSION, &args![this, x87_quiet(seconds)]);
    }
    let source_node = if e.call(PROJECTILE_SOURCE, &args![this]).u32() != 0 {
        let source = e.call(PROJECTILE_SOURCE, &args![this]).u32();
        e.vcall(source, 0x20c, &[]).u32()
    } else {
        0
    };
    let node = e.vcall(this.addr(), 0x1d0, &[]).u32();
    if source_node != 0 && node != 0 {
        let player: u32 = e.global(PLAYER);
        if e.call(PROJECTILE_SOURCE, &args![this]).u32() == player {
            let rotation = e.call(NODE_ROTATE, &args![source_node]).u32();
            let product = e.with_stack(0x24, |e, result| {
                e.call(
                    MATRIX_MULTIPLY,
                    &args![this.addr() + 0x94, result, rotation],
                )
                .u32()
            });
            e.call(NODE_SET_ROTATE, &args![node, product]);
            let rotation = e.call(NODE_ROTATE, &args![source_node]).u32();
            let product = e.with_stack(0x24, |e, result| {
                e.call(
                    MATRIX_MULTIPLY,
                    &args![this.addr() + 0x94, result, rotation],
                )
                .u32()
            });
            e.call(SET_ORIENTATION, &args![this, product]);
        }
        let position = e.call(NODE_TRANSLATE, &args![source_node]).u32();
        e.call(NODE_SET_TRANSLATE, &args![node, position]);
        let position = e.call(NODE_TRANSLATE, &args![source_node]).u32();
        e.call(SET_LOCATION, &args![this, position]);
    }
    if e.call(POINTER_GET, &args![this.addr() + 0x150]).u32() != 0 {
        let value = x87_store32(e.call(BEAM_VALUE, &args![this]).f64());
        let object = e.call(POINTER_GET, &args![this.addr() + 0x150]).u32();
        e.call(OBJECT_SET_VALUE, &args![object, value]);
    }
    let five: f64 = e.global(FIVE);
    if e.call(PROJECTILE_AGE, &args![this]).f64() > five {
        e.call(PROJECTILE_FINISH, &args![this]);
    }
    if e.mem.u8(this.addr() + 0x90) != 0 {
        let age = e.call(PROJECTILE_AGE, &args![this]).f64();
        let owner = e.call(LIMIT_OWNER, &args![this]).u32();
        let limit = e.call(LIMIT_VALUE, &args![owner]).f64();
        if limit <= age {
            if e.vcall(this.addr(), 0x1d0, &[]).u32() != 0 {
                let node = e.vcall(this.addr(), 0x1d0, &[]).u32();
                e.call(NODE_CALL, &args![node, 1u32]);
            }
            e.call(PROJECTILE_FINISH, &args![this]);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x0097_9330,
        beam_projectile_update_projectile(Ptr, f32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    const VTABLE: u32 = 0x0200_0000;
    const SLOT_NODE: u32 = 0x0300_01d0;
    const SLOT_SOURCE_NODE: u32 = 0x0300_020c;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn ret_f64(v: f64) -> Ret {
        Ret {
            st0: v,
            ..Ret::default()
        }
    }

    /// The projectile: +0x04 the test answer, +0x08 the age (`f64`), +0x10
    /// the source, +0x14 slot 0x1D0's node, +0x18 the +0x150 pointer's
    /// object, +0x1C the limit (`f64`); a source keeps its node at +0x14.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x210 / 4];
        slots[0x1d0 / 4] = SLOT_NODE;
        slots[0x20c / 4] = SLOT_SOURCE_NODE;
        e.put_vtable(VTABLE, &slots);
        e.map(0x0101_2000, 0x1000);
        e.map(0x0102_0000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.set_global(ZERO, 0.0f64);
        e.set_global(FIVE, 5.0f64);
        let this = e.mem.alloc(0x200);
        e.mem.set_u32(this, VTABLE);
        e.register(REFERENCE_TEST, |e, a| {
            (e.mem.u32(a[0] + 4) as u8).into_ret()
        });
        e.register(PROJECTILE_AGE, |e, a| ret_f64(e.mem.f64(a[0] + 8)));
        e.register(PROJECTILE_SOURCE, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(SLOT_NODE, |e, a| e.mem.u32(a[0] + 0x14).into_ret());
        e.register(SLOT_SOURCE_NODE, |e, a| e.mem.u32(a[0] + 0x14).into_ret());
        e.register(POINTER_GET, |e, a| {
            e.mem.u32(a[0] - 0x150 + 0x18).into_ret()
        });
        e.register(BEAM_VALUE, |_, _| ret_f64(0.1));
        e.register(LIMIT_OWNER, |_, a| a[0].into_ret());
        e.register(LIMIT_VALUE, |e, a| ret_f64(e.mem.f64(a[0] + 0x1c)));
        e.register(NODE_ROTATE, |_, a| (a[0] + 0x40).into_ret());
        e.register(MATRIX_MULTIPLY, |_, a| a[1].into_ret());
        e.register(NODE_TRANSLATE, |_, a| (a[0] + 0x50).into_ret());
        for addr in [
            CHECK_EXPLOSION,
            NODE_SET_ROTATE,
            SET_ORIENTATION,
            NODE_SET_TRANSLATE,
            SET_LOCATION,
            OBJECT_SET_VALUE,
            PROJECTILE_FINISH,
            NODE_CALL,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn a_tested_projectile_does_nothing() {
        let (mut e, this) = engine();
        e.mem.set_u32(this + 4, 1);
        e.call(0x0097_9330, &args![this, 1.0f32]);
        assert!(calls(&e, PROJECTILE_AGE).is_empty());
    }

    #[test]
    fn a_new_player_beam_follows_its_source() {
        let (mut e, this) = engine();
        let source = e.mem.alloc(0x40);
        e.mem.set_u32(source, VTABLE);
        let source_node = 0x7000u32;
        e.mem.set_u32(source + 0x14, source_node);
        e.mem.set_u32(this + 0x10, source);
        e.mem.set_u32(this + 0x14, 0x7100);
        e.set_global(PLAYER, source);
        let object = 0x7200u32;
        e.mem.set_u32(this + 0x18, object);
        let signalling = f32::from_bits(0x7f80_0001);
        e.call(0x0097_9330, &args![this, signalling]);
        assert_eq!(calls(&e, CHECK_EXPLOSION), vec![vec![this, 0x7fc0_0001]]);
        let rotate = calls(&e, MATRIX_MULTIPLY);
        assert_eq!(rotate.len(), 2);
        assert_eq!(rotate[0][0], this + 0x94);
        assert_eq!(rotate[0][2], source_node + 0x40);
        assert_eq!(calls(&e, NODE_SET_ROTATE), vec![vec![0x7100, rotate[0][1]]]);
        assert_eq!(calls(&e, SET_ORIENTATION), vec![vec![this, rotate[1][1]]]);
        assert_eq!(
            calls(&e, NODE_SET_TRANSLATE),
            vec![vec![0x7100, source_node + 0x50]]
        );
        assert_eq!(
            calls(&e, SET_LOCATION),
            vec![vec![this, source_node + 0x50]]
        );
        assert_eq!(
            calls(&e, OBJECT_SET_VALUE),
            vec![vec![object, 0.1f32.to_bits()]]
        );
        assert!(calls(&e, PROJECTILE_FINISH).is_empty());
    }

    #[test]
    fn another_source_moves_but_does_not_turn() {
        let (mut e, this) = engine();
        let source = e.mem.alloc(0x40);
        e.mem.set_u32(source, VTABLE);
        e.mem.set_u32(source + 0x14, 0x7000);
        e.mem.set_u32(this + 0x10, source);
        e.mem.set_u32(this + 0x14, 0x7100);
        e.mem.set_f64(this + 8, 1.0);
        e.call(0x0097_9330, &args![this, 1.0f32]);
        assert!(calls(&e, CHECK_EXPLOSION).is_empty());
        assert!(calls(&e, MATRIX_MULTIPLY).is_empty());
        assert_eq!(calls(&e, SET_LOCATION).len(), 1);
        // No source node: nothing moves.
        e.call_log = Some(vec![]);
        e.mem.set_u32(source + 0x14, 0);
        e.call(0x0097_9330, &args![this, 1.0f32]);
        assert!(calls(&e, SET_LOCATION).is_empty());
    }

    #[test]
    fn old_or_limited_beams_finish() {
        let (mut e, this) = engine();
        e.mem.set_f64(this + 8, 6.0);
        e.call(0x0097_9330, &args![this, 1.0f32]);
        assert_eq!(calls(&e, PROJECTILE_FINISH).len(), 1);
        // The +0x90 flag with a limit at the age: 00450f90(1) on the node.
        e.call_log = Some(vec![]);
        e.mem.set_f64(this + 8, 2.0);
        e.mem.set_u8(this + 0x90, 1);
        e.mem.set_f64(this + 0x1c, 2.0);
        e.mem.set_u32(this + 0x14, 0x7100);
        e.call(0x0097_9330, &args![this, 1.0f32]);
        assert_eq!(calls(&e, NODE_CALL), vec![vec![0x7100, 1]]);
        assert_eq!(calls(&e, PROJECTILE_FINISH).len(), 1);
        // A limit above the age, or NaN, does nothing.
        e.call_log = Some(vec![]);
        e.mem.set_f64(this + 0x1c, 2.5);
        e.call(0x0097_9330, &args![this, 1.0f32]);
        e.mem.set_f64(this + 0x1c, f64::NAN);
        e.call(0x0097_9330, &args![this, 1.0f32]);
        assert!(calls(&e, PROJECTILE_FINISH).is_empty());
        // Without slot 0x1D0's node only 009bc8f0 runs.
        e.mem.set_f64(this + 0x1c, 1.0);
        e.mem.set_u32(this + 0x14, 0);
        e.call(0x0097_9330, &args![this, 1.0f32]);
        assert!(calls(&e, NODE_CALL).is_empty());
        assert_eq!(calls(&e, PROJECTILE_FINISH).len(), 1);
    }
}
