//! `fallout shared/actionpoints.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Action points: the attack action an attack animation maps to, the cost
//! of each action (a table of 22 game settings at `0119baf8`), and the
//! settings `fActionPointsRunAndGunMult` and `fActionPointsRestoreRate`.

#[allow(unused_imports)]
use crate::prelude::*;

/// Returns the address of a float setting's value (`Setting` +4), or of a
/// zeroed scratch float (`01202800`) when the setting pointer is null
/// (`00403e20`, fastcall on the setting).
const SETTING_FLOAT_ADDRESS: u32 = 0x0040_3e20;
/// Returns a setting's value word (`Setting` +4), or 0 when the setting
/// pointer is null (`00403df0`, fastcall on the setting).
const SETTING_VALUE_WORD: u32 = 0x0040_3df0;
/// The cost settings, one `Setting*` per action, 22 entries (`0119baf8`).
const ACTION_COST_SETTINGS: u32 = 0x0119_baf8;
/// A second table of 22 `Setting*` per action (`0119bb50`), read as words.
const ACTION_WORD_SETTINGS: u32 = 0x0119_bb50;
/// Number of entries in both tables.
const ACTION_COUNT: i32 = 0x16;
/// `fActionPointsRunAndGunMult` (its initializer `00f83410` passes that
/// name, string `0106732c`).
const RUN_AND_GUN_MULT: u32 = 0x011d_5f88;
/// `fActionPointsRestoreRate` (its initializer `00f834a0` passes that name,
/// string `01067384`).
const RESTORE_RATE: u32 = 0x011d_5ff4;
/// Returns the signed byte at +0xF4 of the weapon (`00446390`, fastcall).
const WEAPON_BYTE_F4: u32 = 0x0044_6390;
/// Returns the float at +0x138 of the weapon in ST0 (`00821660`, fastcall).
const WEAPON_FLOAT_138: u32 = 0x0082_1660;
/// `BGSEntryPoint::HandleEntryPoint` (`005e58f0`, cdecl).
const HANDLE_ENTRY_POINT: u32 = 0x005e_58f0;
/// The entry point number `0066dce0` passes.
const ENTRY_POINT_NUMBER: u32 = 0x28;
/// The actor `0066dce0` passes to the entry point (`011dea3c`, read as a
/// word; other units name it the player character pointer).
const ENTRY_POINT_ACTOR: u32 = 0x011d_ea3c;
/// The form passed to the entry point when the weapon is null (`011ca278`,
/// read as a word).
const DEFAULT_WEAPON: u32 = 0x011c_a278;

/// A float copied through the x87 stack (`FLD`/`FSTP`): a signalling NaN
/// comes out quiet.
fn quiet(value: f32) -> f32 {
    if value.is_nan() {
        f32::from_bits(value.to_bits() | 0x0040_0000)
    } else {
        value
    }
}

/// `FLD a; FMUL b; FSTP float`: the product rounded once to a float (the
/// exact product of two floats fits a double). NaNs follow the x87 rules:
/// a NaN operand comes out quiet; of two NaNs, a quiet one wins over a
/// signalling one, then the larger significand, then the positive one
/// (the shared x87 choice, #199). `a` is loaded (`FLD`, quiet), `b` is the
/// memory operand.
fn x87_mul(a: f32, b: f32) -> f32 {
    crate::x87::x87_mul(f64::from(x87_quiet(a)), x87_m32(b.to_bits())) as f32
}

/// The float a setting holds, through `00403e20`.
fn setting_float(e: &mut Engine, setting: u32) -> f32 {
    let value = e.call(SETTING_FLOAT_ADDRESS, &args![setting]).u32();
    e.mem.f32(value)
}

// Translated from 0066dba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActionPoints::GetAttackAction` (Xbox PDB): the action an attack
/// animation number maps to (jump table `0066dc30`): 0, 1, 2 to
/// themselves, 3-4 to 3, 5-7 to 4, 8-11 to 5-8, 13 to 0x13; negative
/// numbers, 12 and 14 and above give 0x16.
pub fn action_points_get_attack_action(_e: &mut Engine, animation: i32) -> i32 {
    match animation {
        0 => 0,
        1 => 1,
        2 => 2,
        3 | 4 => 3,
        5..=7 => 4,
        8 => 5,
        9 => 6,
        10 => 7,
        11 => 8,
        13 => 0x13,
        _ => 0x16,
    }
}

// Translated from 0066dc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The action-point cost of `action`: the float of its setting in the
/// table at `0119baf8`, or 0 when `action` is outside 0..22.
pub fn fn_0066dc70(e: &mut Engine, action: i32) -> f32 {
    let mut cost = 0.0f32;
    if (0..ACTION_COUNT).contains(&action) {
        let setting = e.mem.u32(ACTION_COST_SETTINGS + action as u32 * 4);
        cost = quiet(setting_float(e, setting));
    }
    cost
}

// Translated from 0066dca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The cost of `action` (`0066dc70`), multiplied by
/// `fActionPointsRunAndGunMult` when `unscaled` is false.
pub fn fn_0066dca0(e: &mut Engine, action: i32, unscaled: bool) -> f32 {
    let cost = quiet(fn_0066dc70(e, action));
    if unscaled {
        return cost;
    }
    let mult = setting_float(e, RUN_AND_GUN_MULT);
    x87_mul(cost, mult)
}

// Translated from 0066dce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActionPoints::GetWeaponAttackActionPoints` (Xbox PDB): the action-point
/// cost of an attack with `weapon`. The action comes from the weapon's
/// signed byte at +0xF4 (`00446390`; 0 without a weapon) through
/// `GetAttackAction`. When the weapon has bit 3 of the word at +0x12C
/// (`0066dd80`), the cost is its float at +0x138 (`00821660`), otherwise the
/// action's cost (`0066dc70`). Entry point 0x28 then adjusts the cost
/// (`BGSEntryPoint::HandleEntryPoint` with the actor at `011dea3c`, the
/// weapon or the form at `011ca278` without one, and the cost's address).
pub fn action_points_get_weapon_attack_action_points(e: &mut Engine, weapon: Ptr) -> f32 {
    let animation = if weapon.is_null() {
        0
    } else {
        e.call(WEAPON_BYTE_F4, &args![weapon]).i32()
    };
    let action = action_points_get_attack_action(e, animation);
    e.with_stack(4, |e, cost| {
        let value = if !weapon.is_null() && fn_0066dd80(e, weapon) {
            quiet(e.call(WEAPON_FLOAT_138, &args![weapon]).f32())
        } else {
            quiet(fn_0066dc70(e, action))
        };
        e.mem.set_f32(cost.addr(), value);
        let form = if weapon.is_null() {
            e.global::<u32>(DEFAULT_WEAPON)
        } else {
            weapon.addr()
        };
        let actor: u32 = e.global(ENTRY_POINT_ACTOR);
        e.call(
            HANDLE_ENTRY_POINT,
            &args![ENTRY_POINT_NUMBER, actor, form, cost],
        );
        quiet(e.mem.f32(cost.addr()))
    })
}

// Translated from 0066dd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 3 of the word at +0x12C of `weapon` is set.
pub fn fn_0066dd80(e: &mut Engine, weapon: Ptr) -> bool {
    e.mem.u32(weapon.addr() + 0x12c) & 8 != 0
}

// Translated from 0066dda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The value of `fActionPointsRestoreRate`.
pub fn fn_0066dda0(e: &mut Engine) -> f32 {
    quiet(setting_float(e, RESTORE_RATE))
}

// Translated from 0066ddc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The value word of `action`'s setting in the table at `0119bb50`
/// (`00403df0`; its result is left in EAX, which callers use). The index is
/// not range-checked.
pub fn fn_0066ddc0(e: &mut Engine, action: i32) -> u32 {
    let setting = e
        .mem
        .u32(ACTION_WORD_SETTINGS.wrapping_add((action as u32).wrapping_mul(4)));
    e.call(SETTING_VALUE_WORD, &args![setting]).u32()
}

// Translated from 0066dde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActionPoints::IsAttackAction` (Xbox PDB): true for actions 0-8, 0x10,
/// 0x11 and 0x13-0x15 (byte table `0066de14`).
pub fn action_points_is_attack_action(_e: &mut Engine, action: u32) -> bool {
    matches!(action, 0..=8 | 0x10 | 0x11 | 0x13..=0x15)
}

// Translated from 0066de30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActionPoints::IsMeleeAttackAction` (Xbox PDB): true for actions 0-2,
/// 0x10, 0x11, 0x14 and 0x15 (byte table `0066de64`).
pub fn action_points_is_melee_attack_action(_e: &mut Engine, action: u32) -> bool {
    matches!(action, 0..=2 | 0x10 | 0x11 | 0x14 | 0x15)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0066dba0, action_points_get_attack_action(i32) -> i32),
        entry!(0x0066dc70, fn_0066dc70(i32) -> f32),
        entry!(0x0066dca0, fn_0066dca0(i32, bool) -> f32),
        entry!(
            0x0066dce0,
            action_points_get_weapon_attack_action_points(Ptr) -> f32
        ),
        entry!(0x0066dd80, fn_0066dd80(Ptr) -> bool),
        entry!(0x0066dda0, fn_0066dda0() -> f32),
        entry!(0x0066ddc0, fn_0066ddc0(i32) -> u32),
        entry!(0x0066dde0, action_points_is_attack_action(u32) -> bool),
        entry!(0x0066de30, action_points_is_melee_attack_action(u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Settings: each holds its value at +4, as `00403e20`/`00403df0` read.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(SETTING_FLOAT_ADDRESS, |_, a| Ret {
            eax: a[0] + 4,
            ..Ret::default()
        });
        e.register(SETTING_VALUE_WORD, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        for page in [0x0119_b000, 0x011d_5000, 0x011d_e000, 0x011c_a000] {
            e.map(page, 0x1000);
        }
        e
    }

    fn put_cost(e: &mut Engine, action: u32, cost: f32) -> u32 {
        let setting = e.mem.alloc(0xc);
        e.mem.set_f32(setting + 4, cost);
        e.mem.set_u32(ACTION_COST_SETTINGS + action * 4, setting);
        setting
    }

    #[test]
    fn attack_action_follows_the_jump_table() {
        let mut e = engine();
        let expected = [0, 1, 2, 3, 3, 4, 4, 4, 5, 6, 7, 8, 0x16, 0x13, 0x16];
        for (animation, want) in expected.iter().enumerate() {
            assert_eq!(
                action_points_get_attack_action(&mut e, animation as i32),
                *want
            );
        }
        assert_eq!(action_points_get_attack_action(&mut e, -1), 0x16);
    }

    #[test]
    fn cost_reads_the_table_in_range_only() {
        let mut e = engine();
        put_cost(&mut e, 3, 25.0);
        assert_eq!(fn_0066dc70(&mut e, 3), 25.0);
        assert_eq!(fn_0066dc70(&mut e, -1), 0.0);
        assert_eq!(fn_0066dc70(&mut e, 0x16), 0.0);
        put_cost(&mut e, 4, f32::from_bits(0x7f80_0001));
        assert_eq!(fn_0066dc70(&mut e, 4).to_bits(), 0x7fc0_0001);
    }

    #[test]
    fn scaled_cost_multiplies_by_run_and_gun() {
        let mut e = engine();
        put_cost(&mut e, 2, 20.0);
        e.mem.set_f32(RUN_AND_GUN_MULT + 4, 1.5);
        assert_eq!(fn_0066dca0(&mut e, 2, false), 30.0);
        assert_eq!(fn_0066dca0(&mut e, 2, true), 20.0);
        assert_eq!(
            x87_mul(f32::from_bits(0x7fc0_0001), f32::from_bits(0x7f80_0005)).to_bits(),
            0x7fc0_0001
        );
        assert_eq!(
            x87_mul(2.0, f32::from_bits(0xff80_0003)).to_bits(),
            0xffc0_0003
        );
    }

    #[test]
    fn weapon_cost_uses_override_or_table_then_entry_point() {
        let mut e = engine();
        put_cost(&mut e, 4, 30.0);
        e.set_global(ENTRY_POINT_ACTOR, 0x0bad_0001u32);
        e.set_global(DEFAULT_WEAPON, 0x0bad_0002u32);
        e.register(WEAPON_BYTE_F4, |e, a| Ret {
            eax: e.mem.i8(a[0] + 0xf4) as i32 as u32,
            ..Ret::default()
        });
        e.register(WEAPON_FLOAT_138, |e, a| Ret {
            st0: e.mem.f32(a[0] + 0x138) as f64,
            ..Ret::default()
        });
        // The entry point doubles the cost.
        e.register(HANDLE_ENTRY_POINT, |e, a| {
            let cost = e.mem.f32(a[3]);
            e.mem.set_f32(a[3], cost * 2.0);
            Ret::default()
        });
        let weapon = e.mem.alloc(0x140);
        e.mem.set_u8(weapon + 0xf4, 6);
        e.mem.set_f32(weapon + 0x138, 7.0);

        e.call_log = Some(vec![]);
        let weapon = Ptr::new(weapon);
        assert_eq!(
            action_points_get_weapon_attack_action_points(&mut e, weapon),
            60.0
        );
        let log = e.call_log.take().unwrap();
        let entry: Vec<_> = log.iter().filter(|c| c.0 == HANDLE_ENTRY_POINT).collect();
        assert_eq!(entry[0].1[..3], [0x28, 0x0bad_0001, weapon.addr()]);

        // Bit 3 at +0x12C: the weapon's own cost.
        e.mem.set_u32(weapon.addr() + 0x12c, 8);
        assert_eq!(
            action_points_get_weapon_attack_action_points(&mut e, weapon),
            14.0
        );

        // No weapon: action 0's cost and the default form.
        put_cost(&mut e, 0, 5.0);
        e.call_log = Some(vec![]);
        assert_eq!(
            action_points_get_weapon_attack_action_points(&mut e, Ptr::new(0)),
            10.0
        );
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|c| c.0 == WEAPON_BYTE_F4));
        let entry: Vec<_> = log.iter().filter(|c| c.0 == HANDLE_ENTRY_POINT).collect();
        assert_eq!(entry[0].1[2], 0x0bad_0002);
    }

    #[test]
    fn flag_bit_three() {
        let mut e = engine();
        let weapon = e.mem.alloc(0x140);
        assert!(!fn_0066dd80(&mut e, Ptr::new(weapon)));
        e.mem.set_u32(weapon + 0x12c, 0xf7);
        assert!(!fn_0066dd80(&mut e, Ptr::new(weapon)));
        e.mem.set_u32(weapon + 0x12c, 8);
        assert!(fn_0066dd80(&mut e, Ptr::new(weapon)));
    }

    #[test]
    fn restore_rate_reads_its_setting() {
        let mut e = engine();
        e.mem.set_f32(RESTORE_RATE + 4, 4.0);
        assert_eq!(fn_0066dda0(&mut e), 4.0);
    }

    #[test]
    fn word_setting_by_action() {
        let mut e = engine();
        let setting = e.mem.alloc(0xc);
        e.mem.set_u32(setting + 4, 77);
        e.mem.set_u32(ACTION_WORD_SETTINGS + 9 * 4, setting);
        assert_eq!(fn_0066ddc0(&mut e, 9), 77);
    }

    #[test]
    fn attack_and_melee_actions() {
        let mut e = engine();
        let attack: Vec<u32> = (0..0x20)
            .filter(|&a| action_points_is_attack_action(&mut e, a))
            .collect();
        assert_eq!(
            attack,
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 0x10, 0x11, 0x13, 0x14, 0x15]
        );
        let melee: Vec<u32> = (0..0x20)
            .filter(|&a| action_points_is_melee_attack_action(&mut e, a))
            .collect();
        assert_eq!(melee, [0, 1, 2, 0x10, 0x11, 0x14, 0x15]);
    }
}
