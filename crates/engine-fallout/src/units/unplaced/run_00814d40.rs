//! `(unplaced)/run_00814d40` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004534f0(this, 0)` (`fallout shared/tes.cpp`): the call `00815300` makes
/// twice, with a zero stack word.
const CLEAR_STEP: u32 = 0x0045_34f0;
/// The vtable `00815300` stores.
const VTABLE: u32 = 0x0107_ac5c;

// Translated from 00815300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no unit; on the player construction path):
/// stores the vtable `0107ac5c`, calls `004534f0(this, 0)` twice, zeroes the
/// words at +8 and +4 and returns `this`. The class is not confirmed from
/// the exe.
pub fn fn_00815300(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(CLEAR_STEP, &args![this, 0u32]);
    e.call(CLEAR_STEP, &args![this, 0u32]);
    e.mem.set_u32(this.addr() + 8, 0);
    e.mem.set_u32(this.addr() + 4, 0);
    this
}

/// `MobileObject::FinishInitLoadGame` (`00932520`, engine map; `thiscall`, two words).
const MOBILE_OBJECT_FINISH_INIT_LOAD_GAME: u32 = 0x0093_2520;
/// `MobileObject::GetCharController` (`009306d0`, engine map; `thiscall`).
const MOBILE_OBJECT_GET_CHAR_CONTROLLER: u32 = 0x0093_06d0;
/// `009185e0` (`thiscall`; `fallout/ai/middlehighprocess.cpp`): an object whose vtable slot
/// +0x2C gives another reference.
const PROCESS_009185E0: u32 = 0x0091_85e0;
/// `00931ed0` (`thiscall`, an out word) and `0043b4f0` (`thiscall`, an out word): a filter
/// source; `004a3a20` (`thiscall`) the filter word of it.
const FILTER_SOURCE_00931ED0: u32 = 0x0093_1ed0;
const FILTER_SOURCE_0043B4F0: u32 = 0x0043_b4f0;
const FILTER_WORD_004A3A20: u32 = 0x004a_3a20;
/// `FindFirstCollisionObject` (`004b5260`, engine map; `cdecl`, one word) and `006fa820`
/// (`thiscall`): the collision object of a node.
const FIND_FIRST_COLLISION_OBJECT: u32 = 0x004b_5260;
const COLLISION_006FA820: u32 = 0x006f_a820;
/// `005dbe10` (no arguments): the default filter word.
const DEFAULT_FILTER_005DBE10: u32 = 0x005d_be10;
/// `0070c440` (`thiscall`, an out word), `004a39f0` (`thiscall` on a filter word, one
/// word), `0059ce80` (`thiscall` on a filter word, one word) and
/// `bhkCharacterController::SetCollisionFilter` (`00810ff0`, engine map; `thiscall`, one
/// word); `00810f90` (`thiscall` on a filter word, one word).
const FILTER_GET_0070C440: u32 = 0x0070_c440;
const FILTER_SET_LAYER_004A39F0: u32 = 0x004a_39f0;
const FILTER_SET_0059CE80: u32 = 0x0059_ce80;
const SET_COLLISION_FILTER: u32 = 0x0081_0ff0;
const FILTER_SET_00810F90: u32 = 0x0081_0f90;
/// The player (`PlayerCharacter *`) global.
const PLAYER: u32 = 0x011d_ea3c;
/// `00403e20` (`thiscall` on a float setting): the address of its value; the setting at
/// `011d1758`; `0080fae0` (`cdecl`, one float).
const FLOAT_SETTING_VALUE: u32 = 0x0040_3e20;
const SETTING_011D1758: u32 = 0x011d_1758;
const CALL_0080FAE0: u32 = 0x0080_fae0;
/// `00559450` (`thiscall`): the word at `this`.
const FIRST_WORD: u32 = 0x0055_9450;
/// `00440460` and `004f0160` (`thiscall`, one word each) on the object held at +0xB4.
const SET_00440460: u32 = 0x0044_0460;
const SET_004F0160: u32 = 0x004f_0160;
/// `0045bb80` (`thiscall`), `00439ef0` (`thiscall`: a vector out, a point) and `00457990`
/// (`thiscall` on a vector: its length, in ST0).
const OBJECT_0045BB80: u32 = 0x0045_bb80;
const DIRECTION_00439EF0: u32 = 0x0043_9ef0;
const VECTOR_LENGTH_00457990: u32 = 0x0045_7990;
/// `00bc28b0` (`thiscall`: three floats and a byte) on the object held at +0xA8, and
/// `00ee8990` (`thiscall`, `bool`) on it.
const SHOW_00BC28B0: u32 = 0x00bc_28b0;
const CHECK_00EE8990: u32 = 0x00ee_8990;
/// `006815c0` (`thiscall`): the matrix local's constructor here.
const MATRIX_CONSTRUCT_006815C0: u32 = 0x0068_15c0;
/// `atan2` (`004b5550`, LIBCMT; `cdecl`, two floats, the result in ST0).
const ATAN2: u32 = 0x004b_5550;
/// `NiMatrix3::FromEulerAnglesZYX` (`00a59780`, engine map; `thiscall`, three floats).
const MATRIX_FROM_EULER_ANGLES_ZYX: u32 = 0x00a5_9780;
/// `0043fa80` (`thiscall`, one word): sets the rotation of the object held at +0xB4.
const SET_ROTATION_0043FA80: u32 = 0x0043_fa80;
/// `008133d0` (`thiscall`, one word).
const CALL_008133D0: u32 = 0x0081_33d0;
/// `MagicTarget::GetMagicTargetByNumericID` (`00825550`, engine map; `cdecl`, one word).
const MAGIC_TARGET_BY_NUMERIC_ID: u32 = 0x0082_5550;
/// The game's `operator delete` (`00401030`, `cdecl`).
const OPERATOR_DELETE: u32 = 0x0040_1030;

/// The filter word of `node`'s collision object (`004b5260`, `006fa820`, `0043b4f0` with
/// `out`, `004a3a20`), or the default (`005dbe10`) without one.
fn node_filter(e: &mut Engine, node: u32, out: u32) -> u32 {
    let collision = e.call(FIND_FIRST_COLLISION_OBJECT, &args![node]).u32();
    let object = if collision != 0 {
        e.call(COLLISION_006FA820, &args![collision]).u32()
    } else {
        0
    };
    if object != 0 {
        let source = e.call(FILTER_SOURCE_0043B4F0, &args![object, out]).u32();
        e.call(FILTER_WORD_004A3A20, &args![source]).u32()
    } else {
        e.call(DEFAULT_FILTER_005DBE10, &[]).u32()
    }
}

// Translated from 00814d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words; a projectile's load-game finish, by its
/// first call): `MobileObject::FinishInitLoadGame(first, second)`; with a character
/// controller its filter takes layer 7 and the filter word of the shooter (the reference
/// `009185e0`'s slot +0x2C gives: its own controller's when 3D-loaded, else its root node's
/// collision object's) or of the projectile's own root node, through
/// `SetCollisionFilter`. A shooter other than the player gets `0080fae0(setting
/// 011d1758)`. With the pending object at +0xB0: with an object held at +0xB4 it is aimed
/// there (`00440460`, `004f0160` with +0xC); with one held at +0xBC its direction gives the
/// length shown (`00bc28b0` on +0xA8 with +0x1C, the length, +0x88 and whether +0xAC is
/// set) and the rotation (`FromEulerAnglesZYX(atan2(x, y), 0, -atan2(z, |xy|))`,
/// `0043fa80`); otherwise the length from the point vtable +0x1F4 gives, and when
/// `00ee8990` answers true or +0xAC is 2, `008133d0(2)` and filter bit 1 (`00810f90`).
/// Then +0xB0 is cleared, each magic target its +0x20 list names (a `u16` count, then
/// ids) is passed to vtable +0x308 (with 0), and the object is freed.
pub fn fn_00814d40(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    let at = this.addr();
    e.call(
        MOBILE_OBJECT_FINISH_INIT_LOAD_GAME,
        &args![at, first, second],
    );
    e.with_stack(0xcc, |e, frame| {
        // The frame's locals at their offsets from EBP.
        let ebp = frame.addr() + 0xcc;
        let controller = e.call(MOBILE_OBJECT_GET_CHAR_CONTROLLER, &args![at]).u32();
        if controller != 0 {
            let shooter_reference = e.call(PROCESS_009185E0, &args![at]).u32() != 0 && {
                let process = e.call(PROCESS_009185E0, &args![at]).u32();
                e.vcall(process, 0x2c, &[]).u32() != 0
            };
            let filter = if shooter_reference {
                let process = e.call(PROCESS_009185E0, &args![at]).u32();
                let shooter = e.vcall(process, 0x2c, &[]).u32();
                let loaded = e.vcall(shooter, 0x100, &[]).bool();
                // NEG/SBB/AND: the shooter when 3D-loaded, else 0.
                let mobile = if loaded { shooter } else { 0 };
                if mobile != 0 {
                    let source = e
                        .call(FILTER_SOURCE_00931ED0, &args![mobile, ebp - 0x90])
                        .u32();
                    e.call(FILTER_WORD_004A3A20, &args![source]).u32()
                } else {
                    let node = e.vcall(shooter, 0x1d0, &[]).u32();
                    node_filter(e, node, ebp - 0x94)
                }
            } else {
                let node = e.vcall(at, 0x1d0, &[]).u32();
                node_filter(e, node, ebp - 0x98)
            };
            e.call(FILTER_GET_0070C440, &args![controller, ebp - 0x10]);
            e.call(FILTER_SET_LAYER_004A39F0, &args![ebp - 0x10, 7u32]);
            e.call(FILTER_SET_0059CE80, &args![ebp - 0x10, filter]);
            let word = e.mem.u32(ebp - 0x10);
            e.call(SET_COLLISION_FILTER, &args![controller, word]);
        }
        let shooter = if e.call(PROCESS_009185E0, &args![at]).u32() != 0 {
            let process = e.call(PROCESS_009185E0, &args![at]).u32();
            e.vcall(process, 0x2c, &[]).u32()
        } else {
            0
        };
        if shooter != e.mem.u32(PLAYER) {
            let value = e.call(FLOAT_SETTING_VALUE, &args![SETTING_011D1758]).u32();
            let value = x87_quiet_bits(e.mem.u32(value));
            e.call(CALL_0080FAE0, &args![value]);
        }
        let pending = e.mem.u32(at + 0xb0);
        if pending == 0 {
            return;
        }
        if e.call(FIRST_WORD, &args![at + 0xb4]).u32() != 0 {
            let target = e.call(FIRST_WORD, &args![at + 0xb4]).u32();
            e.call(SET_00440460, &args![target, pending]);
            let target = e.call(FIRST_WORD, &args![at + 0xb4]).u32();
            e.call(SET_004F0160, &args![target, pending + 0xc]);
            let hit = u32::from(e.mem.u32(at + 0xac) != 0);
            if e.call(FIRST_WORD, &args![at + 0xbc]).u32() != 0 {
                let direction = ebp - 0x64;
                let held = e.call(FIRST_WORD, &args![at + 0xbc]).u32();
                let object = e.call(OBJECT_0045BB80, &args![held]).u32();
                e.call(DIRECTION_00439EF0, &args![object, direction, pending]);
                let scale = x87_quiet_bits(e.mem.u32(at + 0x88));
                let length = e.call(VECTOR_LENGTH_00457990, &args![direction]).f64();
                let length = x87_store32(length);
                let speed = x87_quiet_bits(e.mem.u32(pending + 0x1c));
                let shown = e.call(FIRST_WORD, &args![at + 0xa8]).u32();
                e.call(SHOW_00BC28B0, &args![shown, speed, length, scale, hit]);
                let matrix = ebp - 0x58;
                e.call(MATRIX_CONSTRUCT_006815C0, &args![matrix]);
                let z = x87_quiet_bits(e.mem.u32(direction + 8));
                e.mem.set_u32(ebp - 0x68, z);
                e.mem.set_u32(direction + 8, 0.0f32.to_bits());
                let flat = e.call(VECTOR_LENGTH_00457990, &args![direction]).f64();
                e.mem.set_u32(ebp - 0x34, x87_store32(flat));
                let flat = x87_quiet_bits(e.mem.u32(ebp - 0x34));
                let z = x87_quiet_bits(e.mem.u32(ebp - 0x68));
                let pitch = e.call(ATAN2, &args![z, flat]).f64();
                // FCHS, then FSTP float.
                let pitch = x87_store32(-pitch);
                let y = x87_quiet_bits(e.mem.u32(direction + 4));
                let x = x87_quiet_bits(e.mem.u32(direction));
                let heading = e.call(ATAN2, &args![x, y]).f64();
                let heading = x87_store32(heading);
                e.call(
                    MATRIX_FROM_EULER_ANGLES_ZYX,
                    &args![matrix, heading, 0.0f32.to_bits(), pitch],
                );
                let target = e.call(FIRST_WORD, &args![at + 0xb4]).u32();
                e.call(SET_ROTATION_0043FA80, &args![target, matrix]);
            } else {
                let point = e.vcall(at, 0x1f4, &[]).u32();
                let copy = ebp - 0x78;
                for i in 0..3 {
                    let word = e.mem.u32(point + 4 * i);
                    e.mem.set_u32(copy + 4 * i, word);
                }
                let vector = e
                    .call(DIRECTION_00439EF0, &args![pending, ebp - 0xa4, copy])
                    .u32();
                let length = e.call(VECTOR_LENGTH_00457990, &args![vector]).f64();
                e.mem.set_u32(ebp - 0x6c, x87_store32(length));
                let scale = x87_quiet_bits(e.mem.u32(at + 0x88));
                let length = x87_quiet_bits(e.mem.u32(ebp - 0x6c));
                let speed = x87_quiet_bits(e.mem.u32(pending + 0x1c));
                let shown = e.call(FIRST_WORD, &args![at + 0xa8]).u32();
                e.call(SHOW_00BC28B0, &args![shown, speed, length, scale, hit]);
                let shown = e.call(FIRST_WORD, &args![at + 0xa8]).u32();
                if e.call(CHECK_00EE8990, &args![shown]).bool() || e.mem.u32(at + 0xac) == 2 {
                    e.call(CALL_008133D0, &args![at, 2u32]);
                    let controller = e.call(MOBILE_OBJECT_GET_CHAR_CONTROLLER, &args![at]).u32();
                    e.call(FILTER_GET_0070C440, &args![controller, ebp - 0x7c]);
                    e.call(FILTER_SET_00810F90, &args![ebp - 0x7c, 1u32]);
                    let word = e.mem.u32(ebp - 0x7c);
                    let controller = e.call(MOBILE_OBJECT_GET_CHAR_CONTROLLER, &args![at]).u32();
                    e.call(SET_COLLISION_FILTER, &args![controller, word]);
                }
            }
        }
        e.mem.set_u32(at + 0xb0, 0);
        if e.mem.u32(pending + 0x20) != 0 {
            let count = u32::from(e.mem.u16(e.mem.u32(pending + 0x20)));
            let mut i = 0u32;
            while i < count {
                let list = e.mem.u32(pending + 0x20);
                let id = e.mem.u32(list.wrapping_add(i.wrapping_mul(4)) + 4);
                let target = e.call(MAGIC_TARGET_BY_NUMERIC_ID, &args![id]).u32();
                if target != 0 {
                    e.vcall(at, 0x308, &args![target, 0u32]);
                }
                i += 1;
            }
        }
        e.call(OPERATOR_DELETE, &args![pending]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00815300, fn_00815300(Ptr) -> Ptr),
        entry!(0x0081_4d40, fn_00814d40(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_vtable_steps_twice_and_clears_the_words() {
        let mut e = Engine::new();
        e.register(CLEAR_STEP, |_, _| Ret::default());
        let this = e.mem.alloc(16);
        e.mem.set_u32(this + 4, 5);
        e.mem.set_u32(this + 8, 6);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00815300, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (CLEAR_STEP, vec![this, 0]));
        assert_eq!(log[2], (CLEAR_STEP, vec![this, 0]));
        assert_eq!(log.len(), 3);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!((e.mem.u32(this + 4), e.mem.u32(this + 8)), (0, 0));
    }
}

#[cfg(test)]
mod tests_n077 {
    use super::*;

    type Log = [(u32, Vec<u32>)];

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn bits(value: f32) -> u32 {
        value.to_bits()
    }

    /// An object whose vtable slots answer the given words.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x400);
        for &(slot, value) in slots {
            let target = vtable + slot;
            e.mem.set_u32(vtable + slot, target);
            e.register_double(target, move |_, _| value.into_ret());
        }
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        object
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for address in [
            MOBILE_OBJECT_FINISH_INIT_LOAD_GAME,
            FILTER_GET_0070C440,
            FILTER_SET_LAYER_004A39F0,
            FILTER_SET_0059CE80,
            SET_COLLISION_FILTER,
            FILTER_SET_00810F90,
            CALL_0080FAE0,
            SET_00440460,
            SET_004F0160,
            SHOW_00BC28B0,
            CHECK_00EE8990,
            MATRIX_CONSTRUCT_006815C0,
            MATRIX_FROM_EULER_ANGLES_ZYX,
            SET_ROTATION_0043FA80,
            CALL_008133D0,
            MAGIC_TARGET_BY_NUMERIC_ID,
            OPERATOR_DELETE,
            FIND_FIRST_COLLISION_OBJECT,
            PROCESS_009185E0,
            MOBILE_OBJECT_GET_CHAR_CONTROLLER,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(DEFAULT_FILTER_005DBE10, |_, _| 0xdef0u32.into_ret());
        e.register(FILTER_SOURCE_00931ED0, |_, _| 0x9310u32.into_ret());
        e.register(FILTER_WORD_004A3A20, |_, a| (a[0] + 1).into_ret());
        e.register(FLOAT_SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        e.register(VECTOR_LENGTH_00457990, |e, a| {
            let v = [e.mem.f32(a[0]), e.mem.f32(a[0] + 4), e.mem.f32(a[0] + 8)];
            Ret {
                st0: f64::from(v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt(),
                ..Ret::default()
            }
        });
        e.register(ATAN2, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])) + 2.0 * f64::from(f32::from_bits(a[1])),
            ..Ret::default()
        });
        e.map(PLAYER & !0xfff, 0x1000);
        e.map(SETTING_011D1758 & !0xfff, 0x1000);
        e.mem.set_f32(SETTING_011D1758 + 4, 1.5);
        e
    }

    /// The projectile (with a vtable slot +0x308 and +0x1D0) and its pending object, which
    /// names the magic targets 11 and 22.
    fn projectile(e: &mut Engine) -> (u32, u32) {
        let this = object(e, &[(0x308, 0), (0x1d0, 0x1d00)]);
        let pending = e.mem.alloc(0x30);
        let list = e.mem.alloc(0xc);
        e.mem.set_u16(list, 2);
        e.mem.set_u32(list + 4, 11);
        e.mem.set_u32(list + 8, 22);
        e.mem.set_u32(pending + 0x20, list);
        e.mem.set_f32(pending + 0x1c, 2.0);
        e.mem.set_u32(this + 0xb0, pending);
        e.register(MAGIC_TARGET_BY_NUMERIC_ID, |_, a| {
            if a[0] == 22 {
                0x2222u32.into_ret()
            } else {
                Ret::default()
            }
        });
        (this, pending)
    }

    #[test]
    fn filters_by_the_shooter_and_hands_the_targets_on() {
        let mut e = engine();
        let shooter = object(&mut e, &[(0x100, 1)]);
        let process = object(&mut e, &[(0x2c, shooter)]);
        e.register_double(PROCESS_009185E0, move |_, _| process.into_ret());
        e.register(MOBILE_OBJECT_GET_CHAR_CONTROLLER, |_, _| {
            0xc0c0u32.into_ret()
        });
        e.mem.set_u32(PLAYER, shooter);
        let (this, pending) = projectile(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0081_4d40, &args![this, 5u32, 6u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MOBILE_OBJECT_FINISH_INIT_LOAD_GAME),
            vec![vec![this, 5, 6]]
        );
        let local = calls_to(&log, FILTER_GET_0070C440)[0][1];
        assert_eq!(
            calls_to(&log, FILTER_GET_0070C440),
            vec![vec![0xc0c0, local]]
        );
        assert_eq!(
            calls_to(&log, FILTER_SET_LAYER_004A39F0),
            vec![vec![local, 7]]
        );
        assert_eq!(
            calls_to(&log, FILTER_SET_0059CE80),
            vec![vec![local, 0x9311]]
        );
        assert_eq!(calls_to(&log, SET_COLLISION_FILTER), vec![vec![0xc0c0, 0]]);
        assert!(calls_to(&log, CALL_0080FAE0).is_empty());
        let vtable = e.mem.u32(this);
        let slot = e.mem.u32(vtable + 0x308);
        assert_eq!(calls_to(&log, slot), vec![vec![this, 0x2222, 0]]);
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![pending]]);
        assert_eq!(e.mem.u32(this + 0xb0), 0);
        assert!(calls_to(&log, SET_00440460).is_empty());
    }

    #[test]
    fn aims_at_the_held_object_and_turns_toward_it() {
        let mut e = engine();
        e.mem.set_u32(PLAYER, 1);
        let (this, pending) = projectile(&mut e);
        e.mem.set_u32(this + 0xb4, 0x7400);
        e.mem.set_u32(this + 0xbc, 0x7bc0);
        e.mem.set_u32(this + 0xa8, 0x7a80);
        e.mem.set_u32(this + 0xac, 1);
        e.mem.set_f32(this + 0x88, 0.5);
        e.register(OBJECT_0045BB80, |_, _| 0x45bbu32.into_ret());
        e.register(DIRECTION_00439EF0, |e, a| {
            e.mem.set_f32(a[1], 3.0);
            e.mem.set_f32(a[1] + 4, 4.0);
            e.mem.set_f32(a[1] + 8, 12.0);
            a[1].into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x0081_4d40, &args![this, 0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, CALL_0080FAE0), vec![vec![bits(1.5)]]);
        assert_eq!(calls_to(&log, SET_00440460), vec![vec![0x7400, pending]]);
        assert_eq!(
            calls_to(&log, SET_004F0160),
            vec![vec![0x7400, pending + 0xc]]
        );
        let direction = calls_to(&log, DIRECTION_00439EF0)[0].clone();
        assert_eq!((direction[0], direction[2]), (0x45bb, pending));
        assert_eq!(
            calls_to(&log, SHOW_00BC28B0),
            vec![vec![0x7a80, bits(2.0), bits(13.0), bits(0.5), 1]]
        );
        assert_eq!(
            calls_to(&log, ATAN2),
            vec![vec![bits(12.0), bits(5.0)], vec![bits(3.0), bits(4.0)]]
        );
        let matrix = calls_to(&log, MATRIX_CONSTRUCT_006815C0)[0][0];
        assert_eq!(
            calls_to(&log, MATRIX_FROM_EULER_ANGLES_ZYX),
            vec![vec![matrix, bits(11.0), 0, bits(-22.0)]]
        );
        assert_eq!(
            calls_to(&log, SET_ROTATION_0043FA80),
            vec![vec![0x7400, matrix]]
        );
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![pending]]);
    }

    #[test]
    fn without_a_held_aim_shows_the_distance_and_sets_the_filter_bit() {
        let mut e = engine();
        e.register(MOBILE_OBJECT_GET_CHAR_CONTROLLER, |_, _| {
            0xc0c0u32.into_ret()
        });
        let (this, pending) = projectile(&mut e);
        let point = e.mem.alloc(0xc);
        e.mem.set_f32(point, 7.0);
        let vtable = e.mem.u32(this);
        e.mem.set_u32(vtable + 0x1f4, vtable + 0x1f4);
        e.register_double(vtable + 0x1f4, move |_, _| point.into_ret());
        e.mem.set_u32(this + 0xb4, 0x7400);
        e.mem.set_u32(this + 0xa8, 0x7a80);
        e.mem.set_u32(this + 0xac, 2);
        e.register(DIRECTION_00439EF0, |e, a| {
            assert_eq!(e.mem.f32(a[2]), 7.0);
            e.mem.set_f32(a[1], 0.0);
            e.mem.set_f32(a[1] + 4, 6.0);
            e.mem.set_f32(a[1] + 8, 8.0);
            a[1].into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x0081_4d40, &args![this, 0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        // The projectile's own node has no collision object: the default filter.
        assert_eq!(
            calls_to(&log, FIND_FIRST_COLLISION_OBJECT),
            vec![vec![0x1d00]]
        );
        let local = calls_to(&log, FILTER_SET_LAYER_004A39F0)[0][0];
        assert_eq!(
            calls_to(&log, FILTER_SET_0059CE80),
            vec![vec![local, 0xdef0]]
        );
        assert_eq!(
            calls_to(&log, SHOW_00BC28B0),
            vec![vec![0x7a80, bits(2.0), bits(10.0), 0, 1]]
        );
        assert_eq!(calls_to(&log, CALL_008133D0), vec![vec![this, 2]]);
        let bit = calls_to(&log, FILTER_SET_00810F90);
        assert_eq!(bit.len(), 1);
        assert_eq!(bit[0][1], 1);
        assert_eq!(calls_to(&log, SET_COLLISION_FILTER).len(), 2);
        assert!(calls_to(&log, ATAN2).is_empty());
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![pending]]);
    }
}
