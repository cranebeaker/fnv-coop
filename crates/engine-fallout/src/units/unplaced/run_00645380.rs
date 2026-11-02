//! `(unplaced)/run_00645380` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// The weapon damage-per-second formula (`00645380`) and the weapon field
// it reads (`00645c20`). The engine map gives neither a name or a unit; the
// callers (`008ac9d0` and the combat and interface code) pass an actor's
// value owner, a weapon and its health. All arithmetic is x87: each
// instruction goes through `crate::x87` in the exe's order and operand
// order, `float` locals are stored with `x87_store32` and reloaded quiet
// (`x87_load32`), memory operands of arithmetic use `x87_m32` (float) or
// the plain `f64` (double).

/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `1.0` (`double`).
const ONE: u32 = 0x0101_2070;
/// `10.0` (`double`): the health factor of the condition curve.
const HEALTH_FACTOR: u32 = 0x0102_0758;
/// `100.0` (`double`): percent.
const PERCENT: u32 = 0x0101_7a40;
/// A `double` divisor of the refraction sum (`01011590`).
const REFRACTION_DIVISOR: u32 = 0x0101_1590;
/// The damage factor (`double`) of the mod effect 0xc (`01051678`).
const MOD_DAMAGE_FACTOR: u32 = 0x0105_1678;
/// The upper bound (`float`) of the critical chance clamp (`01016410`).
const CRITICAL_MAX: u32 = 0x0101_6410;
/// The factor (`double`) applied to the clamped critical chance
/// (`01016408`).
const CRITICAL_FACTOR: u32 = 0x0101_6408;
/// The two multipliers (`float`) of the last two flags (`010162c0`,
/// `01050ff8`).
const FLAG_A_FACTOR: u32 = 0x0101_62c0;
const FLAG_B_FACTOR: u32 = 0x0105_0ff8;
/// The player character pointer.
const PLAYER_POINTER: u32 = 0x011d_ea3c;

/// The float settings the formula reads (`00403e20` gives each value's
/// address).
const DAMAGE_SETTING: u32 = 0x011c_e2c0;
const SKILL_BASE_SETTING: u32 = 0x011c_ece8;
const SKILL_SCALE_SETTING: u32 = 0x011c_f350;
const UNARMED_SETTING: u32 = 0x011c_f810;
const PLAYER_RATE_SETTING: u32 = 0x011c_f6c0;
const RATE_SETTING: u32 = 0x011c_fbf0;
const ATTACK_SETTING_A: u32 = 0x011c_eef0;
const ATTACK_SETTING_B: u32 = 0x011c_f21c;
/// The integer setting (`0043d4d0`) the clip size is compared with.
const CLIP_SETTING: u32 = 0x011c_f888;

/// A float setting's value address (`00403e20`, `thiscall`).
const FLOAT_SETTING_VALUE: u32 = 0x0040_3e20;
/// An integer setting's value address (`0043d4d0`, `thiscall`).
const INT_SETTING_VALUE: u32 = 0x0043_d4d0;
/// `_ftol2_sse` (`00ec62c0`): truncates the value in `ST0` (an `f64` here).
const FTOL: u32 = 0x00ec_62c0;

/// The callees, by address (`ST0` results noted).
const WEAPON_SHOTS: u32 = 0x0064_5ca0; // thiscall weapon, ST0
const CONDITION_CURVE: u32 = 0x004b_d510; // cdecl (float, float), ST0
const MOD_EFFECT_VALUE: u32 = 0x004b_d8d0; // thiscall (item, effect, out)
const HAS_MOD_EFFECT: u32 = 0x004b_da70; // thiscall (item, effect), AL
const SHOTS_PER_SECOND: u32 = 0x0064_5dc0; // cdecl (actor, weapon, 0, item), ST0
const CURRENT_AMMO: u32 = 0x0052_5980; // thiscall (weapon, 0)
const AMMO_DATA: u32 = 0x0092_3000; // thiscall ammo
const AMMO_DAMAGE: u32 = 0x0059_a030; // cdecl (0, ammo data, float), ST0
const WEAPON_PROJECTILE: u32 = 0x0052_5a90; // thiscall (weapon, 0)
const PROJECTILE_DATA: u32 = 0x004f_d3c0; // thiscall
const PROJECTILE_DAMAGE: u32 = 0x006a_78f0; // thiscall, ST0
const PROJECTILE_COUNT: u32 = 0x0052_5b20; // thiscall (weapon, flag, 0, actor), AL
const WEAPON_TYPE: u32 = 0x0044_6390; // thiscall weapon
const CLIP_ROUNDS: u32 = 0x004f_e160; // thiscall (weapon, flag)
const WEAPON_SKILL: u32 = 0x008d_85e0; // thiscall weapon
const ACTOR_VALUE: u32 = 0x0066_ef50; // thiscall (owner, actor value), ST0
const HANDLE_ENTRY_POINT: u32 = 0x005e_58f0; // cdecl, five words here
const JAM_MULTIPLIER: u32 = 0x0064_77b0; // cdecl (index), ST0
const RELOAD_TIME: u32 = 0x0064_5ce0; // thiscall weapon, ST0
const ATTACK_TIME: u32 = 0x0064_5cc0; // thiscall weapon, ST0
const WEAPON_TEST_A: u32 = 0x004c_0c30; // thiscall weapon, AL
const WEAPON_TEST_B: u32 = 0x0052_4b40; // thiscall weapon, AL
const WEAPON_TEST_C: u32 = 0x0064_5c80; // thiscall weapon, AL
const REFRACTION_A: u32 = 0x0064_5c60; // thiscall weapon, ST0
const REFRACTION_B: u32 = 0x0064_5c40; // thiscall weapon, ST0
const CRITICAL_CHANCE: u32 = 0x008d_1eb0; // thiscall weapon, ST0
const CLAMP: u32 = 0x0053_30e0; // cdecl (address, low, high)
const HEALTH_FACTOR_CURVE: u32 = 0x0064_6d00; // cdecl (float), ST0

/// The float the setting at `setting` holds, as an `x87_m32` operand.
fn setting_m32(e: &mut Engine, setting: u32) -> f64 {
    let value = e.call(FLOAT_SETTING_VALUE, &args![setting]).u32();
    x87_m32(e.mem.u32(value))
}

/// The float the setting at `setting` holds, loaded with `FLD`.
fn setting_load(e: &mut Engine, setting: u32) -> f64 {
    let value = e.call(FLOAT_SETTING_VALUE, &args![setting]).u32();
    x87_load32(e.mem.u32(value))
}

/// The weapon's projectile data (`004fd3c0` on `00525a90(weapon, 0)`).
fn projectile_data(e: &mut Engine, weapon: u32) -> u32 {
    let projectile = e.call(WEAPON_PROJECTILE, &args![weapon, 0u32]).u32();
    e.call(PROJECTILE_DATA, &args![projectile]).u32()
}

/// Whether the weapon has a projectile with data (the exe's two tests).
fn has_projectile_data(e: &mut Engine, weapon: u32) -> bool {
    if e.call(WEAPON_PROJECTILE, &args![weapon, 0u32]).u32() == 0 {
        return false;
    }
    projectile_data(e, weapon) != 0
}

fn weapon_type(e: &mut Engine, weapon: u32) -> u32 {
    e.call(WEAPON_TYPE, &args![weapon]).u32()
}

// Translated from 00645380 (decompiled, FalloutNV.exe 1.4.0.525)
/// The damage per second of `weapon` in the hands of the actor whose value
/// owner is `owner` (no name in the engine map; `cdecl`, 13 words, the
/// result in `ST0`). 0 unless `health` is above 0 (or NaN).
///
/// With a weapon: the damage is the weapon's base (slot +0x10 of its part
/// at +0x9c, times the setting at `011ce2c0`) plus the item's mod damage
/// (`004bd8d0`), times `01051678` with mod effect 0xc, through the ammo
/// (`0059a030`), plus the projectiles' damage times their count; weapon
/// types 10 and 11 use the projectile's damage times the setting instead
/// (jam index 10), types 3 to 9, 12 and above 13 read the clip size
/// (`004fe160`). The skill (`skill_override`, or the actor's weapon skill
/// value) scales it, entry point 0 adjusts it when `use_perks`. The rate is
/// the clip size over the shots per second plus the reload time (jam
/// multiplier times the clip, `006477b0`/`00645ce0`; not for types 10, 11
/// and 13), plus the attack time (`00645cc0`) when the clip is within the
/// setting at `011cf888` or `ignore_clip`, adjusted for an NPC's refraction
/// and attack settings. The critical part is the critical chance
/// (`crit_chance` times 100 without an owner, actor value 0xe with one)
/// times the weapon's (`008d1eb0`), clamped (`005330e0`) and scaled.
///
/// Without a weapon: the actor's unarmed damage (slot +0x444 for a player
/// character, else from actor values 0x38 and 0x2d and their settings) and
/// a rate from a setting.
///
/// Both then scale the damage by the health factor (`00646d00`), take the
/// weapon's critical damage (`00645c20`, times `crit_damage_mult` when both
/// are non-zero), and the result is
/// `clip * (crit * crit_damage + damage) * (flag_a factor * flag_b factor)
/// / rate` when the rate is not 0, else 0.
pub fn fn_00645380(
    e: &mut Engine,
    owner: u32,
    weapon: u32,
    health: f32,
    use_perks: u8,
    item: u32,
    ignore_clip: u8,
    no_jam_index: u8,
    skill_override: i32,
    crit_chance: f32,
    crit_damage_mult: f32,
    flag_a: u8,
    flag_b: u8,
    ammo: u32,
) -> f32 {
    let zero = e.global::<f64>(ZERO);
    let health_loaded = x87_load32(health.to_bits());
    // The exe answers 0 when `health` is below or equal to 0 (`TEST AH,0x41`);
    // a NaN goes on.
    if health_loaded <= zero {
        return 0.0;
    }
    let mut result = 0f32.to_bits();
    let mut clip_rounds: i32 = 1;
    // The exe also starts the damage at 0.0 and the rate at `FLT_MAX`
    // (`01016970`); every path below overwrites both before reading them.
    let mut damage: u32;
    let mut critical = 0f32.to_bits();
    let mut rate: u32;
    let condition = x87_quiet_bits(health.to_bits());
    let mut mod_flag_clip = 0u8;
    let shots = if weapon != 0 {
        x87_store32(e.call(WEAPON_SHOTS, &args![weapon]).f64())
    } else {
        1f32.to_bits()
    };
    let mut shots = x87_quiet_bits(shots);
    let mut mod_flag_damage = 0u8;
    let scaled = x87_store32(x87_mul(
        x87_load32(condition),
        e.global::<f64>(HEALTH_FACTOR),
    ));
    let curve = e
        .call(
            CONDITION_CURVE,
            &args![x87_quiet_bits(scaled), 1f32.to_bits()],
        )
        .f64();
    let mut jam_index = e.call(FTOL, &args![curve]).i32();
    let mut mod_damage = 0f32.to_bits();
    let ammo = if ammo == weapon { 0 } else { ammo };
    let actor = if owner != 0 {
        owner.wrapping_sub(0xa4)
    } else {
        0
    };
    let is_player_character = e.vcall(actor, 0x21c, &[]).u8();
    if item != 0 {
        mod_damage = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), mod_damage);
            e.call(MOD_EFFECT_VALUE, &args![item, 1u32, slot]);
            e.mem.u32(slot.addr())
        });
        mod_flag_clip = e.call(HAS_MOD_EFFECT, &args![item, 2u32]).u8();
        // The mod effect 8 test runs; its answer is kept in a local nothing
        // reads.
        e.call(HAS_MOD_EFFECT, &args![item, 8u32]);
        shots = x87_store32(
            e.call(SHOTS_PER_SECOND, &args![actor, weapon, 0u32, item])
                .f64(),
        );
        mod_flag_damage = e.call(HAS_MOD_EFFECT, &args![item, 0xcu32]).u8();
    }

    if weapon != 0 {
        let damage_setting = setting_m32(e, DAMAGE_SETTING);
        let base = e.vcall(weapon.wrapping_add(0x9c), 0x10, &[]).u16();
        damage = x87_store32(x87_mul(f64::from(base), damage_setting));
        damage = x87_store32(x87_add(x87_load32(damage), x87_m32(mod_damage)));
        if mod_flag_damage != 0 {
            damage = x87_store32(x87_mul(
                x87_load32(damage),
                e.global::<f64>(MOD_DAMAGE_FACTOR),
            ));
        }
        let ammo = if ammo != 0 {
            ammo
        } else {
            e.call(CURRENT_AMMO, &args![weapon, 0u32]).u32()
        };
        let ammo_data = if ammo != 0 {
            e.call(AMMO_DATA, &args![ammo]).u32()
        } else {
            0
        };
        damage = x87_store32(
            e.call(AMMO_DAMAGE, &args![0u32, ammo_data, x87_quiet_bits(damage)])
                .f64(),
        );
        if has_projectile_data(e, weapon) {
            let data = projectile_data(e, weapon);
            let projectile_damage = e.call(PROJECTILE_DAMAGE, &args![data]).f64();
            let count = e
                .call(
                    PROJECTILE_COUNT,
                    &args![weapon, u32::from(mod_flag_damage), 0u32, actor],
                )
                .u8();
            damage = x87_store32(x87_add(
                x87_mul(f64::from(count), projectile_damage),
                x87_m32(damage),
            ));
        }
        // The code's jump table on the weapon type plus one, as a match.
        match weapon_type(e, weapon) {
            0xffff_ffff | 0 | 1 | 2 | 13 => {}
            10 | 11 => {
                if has_projectile_data(e, weapon) {
                    let damage_setting = setting_m32(e, DAMAGE_SETTING);
                    let data = projectile_data(e, weapon);
                    let projectile_damage = e.call(PROJECTILE_DAMAGE, &args![data]).f64();
                    damage = x87_store32(x87_mul(projectile_damage, damage_setting));
                }
                // The code also sets its copy of `health` to 1.0 here; nothing
                // reads that copy afterwards.
                jam_index = 10;
            }
            _ => {
                clip_rounds = e
                    .call(CLIP_ROUNDS, &args![weapon, u32::from(mod_flag_clip)])
                    .i32();
            }
        }
        if skill_override == -1 {
            let skill = if owner != 0 {
                let actor_value = e.call(WEAPON_SKILL, &args![weapon]).u32();
                let value = e.call(ACTOR_VALUE, &args![owner, actor_value]).f64();
                x87_div(value, e.global::<f64>(PERCENT))
            } else {
                1.0
            };
            let base = setting_m32(e, SKILL_BASE_SETTING);
            let scale = setting_load(e, SKILL_SCALE_SETTING);
            let factor = x87_add(x87_mul(scale, skill), base);
            damage = x87_store32(x87_mul(factor, x87_m32(damage)));
        } else {
            let skill = x87_store32(x87_div(f64::from(skill_override), e.global::<f64>(PERCENT)));
            let base = setting_m32(e, SKILL_BASE_SETTING);
            let scale = setting_load(e, SKILL_SCALE_SETTING);
            let factor = x87_add(x87_mul(scale, x87_m32(skill)), base);
            damage = x87_store32(x87_mul(factor, x87_m32(damage)));
        }
        if use_perks != 0 {
            damage = e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), damage);
                e.call(HANDLE_ENTRY_POINT, &args![0u32, actor, weapon, actor, slot]);
                e.mem.u32(slot.addr())
            });
        }
        rate = x87_quiet_bits(shots);
        if weapon_type(e, weapon) != 10
            && weapon_type(e, weapon) != 11
            && weapon_type(e, weapon) != 13
        {
            let index = if no_jam_index != 0 { 0 } else { jam_index };
            let rounds = f64::from(clip_rounds);
            let jam = e.call(JAM_MULTIPLIER, &args![index]).f64();
            let jammed = x87_mul(jam, rounds);
            let reload = e.call(RELOAD_TIME, &args![weapon]).f64();
            rate = x87_store32(x87_add(x87_mul(reload, jammed), x87_m32(rate)));
        }
        rate = x87_store32(x87_div(f64::from(clip_rounds), x87_m32(rate)));
        let clip_limit_address = e.call(INT_SETTING_VALUE, &args![CLIP_SETTING]).u32();
        let clip_limit = e.mem.i32(clip_limit_address);
        if (clip_rounds <= clip_limit || ignore_clip != 0)
            && weapon_type(e, weapon) != 1
            && weapon_type(e, weapon) != 2
            && weapon_type(e, weapon) != 0
            && weapon_type(e, weapon) != 10
            && weapon_type(e, weapon) != 13
            && weapon_type(e, weapon) != 11
        {
            let attack = e.call(ATTACK_TIME, &args![weapon]).f64();
            rate = x87_store32(x87_add(attack, x87_m32(rate)));
        }
        let is_npc = actor != e.global::<u32>(PLAYER_POINTER);
        if is_npc && e.call(WEAPON_TEST_A, &args![weapon]).bool() {
            if !e.call(WEAPON_TEST_B, &args![weapon]).bool() {
                let first = e.call(REFRACTION_A, &args![weapon]).f64();
                let second = e.call(REFRACTION_B, &args![weapon]).f64();
                let mean = x87_store32(x87_div(
                    x87_add(second, first),
                    e.global::<f64>(REFRACTION_DIVISOR),
                ));
                let period = x87_store32(x87_div(1.0, x87_load32(shots)));
                if x87_load32(period) < x87_load32(mean) {
                    let extra = x87_store32(x87_sub(x87_load32(mean), x87_m32(period)));
                    rate = x87_store32(x87_add(
                        x87_mul(f64::from(clip_rounds), x87_m32(extra)),
                        x87_m32(rate),
                    ));
                }
            }
            let adjusted = e.call(WEAPON_TEST_B, &args![weapon]).bool()
                || e.call(WEAPON_TEST_C, &args![weapon]).bool();
            if adjusted {
                let first = setting_load(e, ATTACK_SETTING_A);
                let second = setting_m32(e, ATTACK_SETTING_B);
                let sum = x87_add(first, second);
                let divisor = setting_load(e, ATTACK_SETTING_B);
                rate = x87_store32(x87_mul(x87_div(sum, divisor), x87_m32(rate)));
            }
        }
        let mut chance = 0f32.to_bits();
        let crit_loaded = x87_load32(crit_chance.to_bits());
        if crit_loaded != zero {
            chance = x87_store32(x87_mul(crit_loaded, e.global::<f64>(PERCENT)));
        }
        let chance = if owner != 0 {
            x87_store32(e.vcall(owner, 0xc, &args![0xeu32]).f64())
        } else {
            x87_quiet_bits(chance)
        };
        let scaled_chance = x87_mul(x87_load32(chance), e.global::<f64>(ONE));
        let weapon_chance = e.call(CRITICAL_CHANCE, &args![weapon]).f64();
        critical = x87_store32(x87_mul(weapon_chance, scaled_chance));
        let high = x87_quiet_bits(e.global::<u32>(CRITICAL_MAX));
        critical = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), critical);
            e.call(CLAMP, &args![slot, 0f32.to_bits(), high]);
            e.mem.u32(slot.addr())
        });
        critical = x87_store32(x87_mul(
            x87_load32(critical),
            e.global::<f64>(CRITICAL_FACTOR),
        ));
        if weapon != 0 && e.call(WEAPON_TEST_B, &args![weapon]).bool() && x87_load32(shots) != zero
        {
            critical = x87_store32(x87_div(x87_load32(critical), x87_m32(shots)));
        }
    } else {
        if is_player_character != 0 {
            let unarmed = e.vcall(actor, 0x444, &[]).i32();
            damage = x87_store32(f64::from(unarmed));
        } else {
            let value = if owner != 0 {
                x87_store32(e.vcall(owner, 0xc, &args![0x38u32]).f64())
            } else {
                0f32.to_bits()
            };
            let unarmed = setting_load(e, UNARMED_SETTING);
            damage = x87_store32(x87_mul(unarmed, x87_m32(value)));
            let skill = if owner != 0 {
                let value = e.call(ACTOR_VALUE, &args![owner, 0x2du32]).f64();
                x87_div(value, e.global::<f64>(PERCENT))
            } else {
                0.0
            };
            let scale = setting_load(e, SKILL_SCALE_SETTING);
            damage = x87_store32(x87_add(x87_mul(scale, skill), x87_m32(damage)));
        }
        let rate_setting = if is_player_character != 0 {
            PLAYER_RATE_SETTING
        } else {
            RATE_SETTING
        };
        let value = e.call(FLOAT_SETTING_VALUE, &args![rate_setting]).u32();
        rate = x87_quiet_bits(e.mem.u32(value));
    }

    let factor = x87_store32(
        e.call(
            HEALTH_FACTOR_CURVE,
            &args![x87_quiet_bits(health.to_bits())],
        )
        .f64(),
    );
    damage = x87_store32(x87_mul(x87_load32(damage), x87_m32(factor)));
    let critical_damage = if weapon != 0 {
        let value = fn_00645c20(e, Ptr::new(weapon));
        x87_store32(f64::from(value))
    } else {
        0f32.to_bits()
    };
    let mut critical_damage = x87_quiet_bits(critical_damage);
    if x87_load32(crit_damage_mult.to_bits()) != zero && x87_load32(critical_damage) != zero {
        critical_damage = x87_store32(x87_mul(
            x87_load32(critical_damage),
            x87_m32(crit_damage_mult.to_bits()),
        ));
    }
    let factor_a = if flag_a != 0 {
        x87_quiet_bits(e.global::<u32>(FLAG_A_FACTOR))
    } else {
        1f32.to_bits()
    };
    let factor_b = if flag_b != 0 {
        x87_quiet_bits(e.global::<u32>(FLAG_B_FACTOR))
    } else {
        1f32.to_bits()
    };
    let sum = x87_add(
        x87_mul(x87_load32(critical), x87_m32(critical_damage)),
        x87_m32(damage),
    );
    let flags = x87_mul(x87_load32(factor_a), x87_m32(factor_b));
    let total = x87_store32(x87_mul(f64::from(clip_rounds), x87_mul(sum, flags)));
    if x87_load32(rate) != zero {
        result = x87_store32(x87_div(x87_load32(total), x87_m32(rate)));
    }
    f32::from_bits(x87_quiet_bits(result))
}

// Translated from 00645c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 16-bit word at +0x1c0 of `this` (no name in the engine map; the
/// damage formula reads it from the weapon as its critical damage).
pub fn fn_00645c20(e: &mut Engine, this: Ptr) -> u16 {
    e.mem.u16(this.addr().wrapping_add(0x1c0))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0064_5380,
            fn_00645380(u32, u32, f32, u8, u32, u8, u8, i32, f32, f32, u8, u8, u32) -> f32
        ),
        entry!(0x0064_5c20, fn_00645c20(Ptr) -> u16),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn float(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// The constants and settings at their addresses (the setting doubles
    /// answer the setting's own address as its value's), and an actor
    /// whose value owner (+0xa4) answers slot +0xc with 5 for value 0xe and
    /// 10 for 0x38.
    fn engine(player_character: bool) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0101_0000, 0x2_0000);
        e.map(0x0105_0000, 0x2000);
        e.map(0x011c_e000, 0x2000);
        e.map(0x011d_e000, 0x1000);
        e.set_global(ZERO, 0.0f64);
        e.set_global(ONE, 1.0f64);
        e.set_global(HEALTH_FACTOR, 10.0f64);
        e.set_global(PERCENT, 100.0f64);
        e.set_global(CRITICAL_MAX, 100.0f32);
        e.set_global(CRITICAL_FACTOR, 0.01f64);
        e.set_global(DAMAGE_SETTING, 1.0f32);
        e.set_global(SKILL_BASE_SETTING, 0.5f32);
        e.set_global(SKILL_SCALE_SETTING, 1.0f32);
        e.set_global(UNARMED_SETTING, 2.0f32);
        e.set_global(RATE_SETTING, 0.5f32);
        e.set_global(PLAYER_RATE_SETTING, 0.25f32);
        e.set_global(CLIP_SETTING, 5u32);
        e.register(FLOAT_SETTING_VALUE, |_, a| word(a[0]));
        e.register(INT_SETTING_VALUE, |_, a| word(a[0]));
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            word(value as i32 as u32)
        });
        e.register(CONDITION_CURVE, |_, _| float(3.7));
        e.register(HEALTH_FACTOR_CURVE, |_, _| float(1.0));
        let actor = e.mem.alloc(0x100);
        e.put_vtable(0x0300_0000, &[0; 0x112]);
        e.mem.set_u32(0x0300_0000 + 0x21c, 0x0300_1000);
        e.mem.set_u32(0x0300_0000 + 0x444, 0x0300_2000);
        e.mem.set_u32(actor, 0x0300_0000);
        e.register_double(0x0300_1000, move |_, _| word(u32::from(player_character)));
        e.register(0x0300_2000, |_, _| word(7));
        e.put_vtable(0x0300_4000, &[0, 0, 0, 0x0300_3000]);
        e.mem.set_u32(actor + 0xa4, 0x0300_4000);
        e.register(0x0300_3000, |_, a| {
            float(if a[1] == 0xe { 5.0 } else { 10.0 })
        });
        e.register(ACTOR_VALUE, |_, _| float(50.0));
        (e, actor)
    }

    fn call(e: &mut Engine, owner: u32, weapon: u32, health: f32, skill: i32) -> f64 {
        e.call(
            0x0064_5380,
            &args![owner, weapon, health, 0u8, 0u32, 0u8, 0u8, skill, 0f32, 0f32, 0u8, 0u8, 0u32],
        )
        .f64()
    }

    #[test]
    fn no_health_gives_zero() {
        let (mut e, actor) = engine(false);
        e.call_log = Some(vec![]);
        assert_eq!(call(&mut e, actor + 0xa4, 0, 0.0, -1), 0.0);
        assert_eq!(call(&mut e, actor + 0xa4, 0, -1.0, -1), 0.0);
        assert_eq!(calls_to(&e, CONDITION_CURVE).len(), 0);
    }

    #[test]
    fn unarmed_npc_damage_per_second() {
        let (mut e, actor) = engine(false);
        // Damage 2 * 10 + 1 * 0.5, rate 0.5: 41.
        assert_eq!(call(&mut e, actor + 0xa4, 0, 1.0, -1), 41.0);
    }

    #[test]
    fn unarmed_player_character_damage_per_second() {
        let (mut e, actor) = engine(true);
        // Damage 7 (slot +0x444), rate 0.25: 28.
        assert_eq!(call(&mut e, actor + 0xa4, 0, 1.0, -1), 28.0);
    }

    fn weapon(e: &mut Engine) -> u32 {
        let weapon = e.mem.alloc(0x200);
        e.put_vtable(0x0310_0000, &[0, 0, 0, 0, 0x0310_1000]);
        e.mem.set_u32(weapon + 0x9c, 0x0310_0000);
        e.register(0x0310_1000, |_, _| word(0xffff_0014));
        e.mem.set_u16(weapon + 0x1c0, 30);
        e.register(WEAPON_SHOTS, |_, _| float(2.0));
        e.register(CURRENT_AMMO, |_, _| word(0));
        e.register(AMMO_DAMAGE, |_, a| float(f64::from(f32::from_bits(a[2]))));
        e.register(WEAPON_PROJECTILE, |_, _| word(0));
        e.register(WEAPON_TYPE, |_, _| word(5));
        e.register(CLIP_ROUNDS, |_, _| word(10));
        e.register(JAM_MULTIPLIER, |_, _| float(0.1));
        e.register(RELOAD_TIME, |_, _| float(2.0));
        e.register(ATTACK_TIME, |_, _| float(1.0));
        e.register(WEAPON_TEST_A, |_, _| word(0));
        e.register(WEAPON_TEST_B, |_, _| word(0));
        e.register(CRITICAL_CHANCE, |_, _| float(1.0));
        e.register(CLAMP, |e, a| {
            let value = e
                .mem
                .f32(a[0])
                .clamp(f32::from_bits(a[1]), f32::from_bits(a[2]));
            e.mem.set_f32(a[0], value);
            Ret::default()
        });
        weapon
    }

    #[test]
    fn weapon_damage_per_second() {
        let (mut e, actor) = engine(false);
        let weapon = weapon(&mut e);
        e.call_log = Some(vec![]);
        // Damage 20 (skill factor 1), critical 0.05 * 30, clip 10, rate
        // 10 / (2 + 2 * 0.1 * 10) = 2.5: 10 * 21.5 / 2.5 = 86.
        assert_eq!(call(&mut e, actor + 0xa4, weapon, 1.0, 50), 86.0);
        assert_eq!(calls_to(&e, JAM_MULTIPLIER), vec![vec![3]]);
        assert_eq!(calls_to(&e, CLIP_ROUNDS), vec![vec![weapon, 0]]);
        assert_eq!(
            calls_to(&e, CONDITION_CURVE),
            vec![vec![10f32.to_bits(), 1f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, 0x0300_3000), vec![vec![actor + 0xa4, 0xe]]);
        assert_eq!(calls_to(&e, CLAMP)[0][1..], [0, 100f32.to_bits()]);
        // Clip 10 is above the setting (5): no attack time.
        assert!(calls_to(&e, ATTACK_TIME).is_empty());
    }

    #[test]
    fn weapon_types_10_and_11_use_the_projectile_and_no_reload() {
        let (mut e, actor) = engine(false);
        let weapon = weapon(&mut e);
        e.register(WEAPON_TYPE, |_, _| word(10));
        e.register(WEAPON_PROJECTILE, |_, _| word(0x0800_0000));
        e.register(PROJECTILE_DATA, |_, _| word(0x0800_0010));
        e.register(PROJECTILE_DAMAGE, |_, _| float(4.0));
        e.register(PROJECTILE_COUNT, |_, _| word(0xff02));
        e.call_log = Some(vec![]);
        // Damage 4 * 1 (setting), skill factor 1; clip 1; rate 1 / 2 = 0.5;
        // critical 0.05 * 30 = 1.5: (1.5 + 4) / 0.5 = 11.
        let result = call(&mut e, actor + 0xa4, weapon, 1.0, 50);
        assert_eq!(result, 11.0);
        assert!(calls_to(&e, CLIP_ROUNDS).is_empty());
        assert!(calls_to(&e, JAM_MULTIPLIER).is_empty());
        assert_eq!(calls_to(&e, PROJECTILE_COUNT)[0], vec![weapon, 0, 0, actor]);
    }

    #[test]
    fn small_clips_add_the_attack_time() {
        let (mut e, actor) = engine(false);
        let weapon = weapon(&mut e);
        e.register(CLIP_ROUNDS, |_, _| word(4));
        e.call_log = Some(vec![]);
        // rate 4 / (2 + 2 * 0.1 * 4) = 1.4285715 + 1 attack time.
        let result = call(&mut e, actor + 0xa4, weapon, 1.0, 50);
        assert_eq!(calls_to(&e, ATTACK_TIME), vec![vec![weapon]]);
        let rate = x87_store32(x87_add(
            1.0,
            x87_m32(x87_store32(x87_div(4.0, 2.8f32 as f64))),
        ));
        let total = x87_store32(4.0 * (0.05f32 as f64 * 30.0 + 20.0));
        let expected = x87_store32(x87_div(f64::from(f32::from_bits(total)), x87_m32(rate)));
        assert_eq!((result as f32).to_bits(), expected);
    }

    #[test]
    fn fn_00645c20_reads_the_word_at_0x1c0() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1c4);
        e.mem.set_u32(this + 0x1c0, 0xaaaa_1234);
        assert_eq!(e.call(0x0064_5c20, &args![this]).u16(), 0x1234);
    }
}
