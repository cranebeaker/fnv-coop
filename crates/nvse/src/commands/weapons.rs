//! xNVSE's weapon commands (`Commands_Inventory.cpp` (xNVSE): `GetWeaponValue`, the weapon
//! setters), batches X09 to X11 of ADR-0016's build order (and the X12 weapon setters that share
//! their code).
//!
//! `TESObjectWEAP` (xNVSE `GameForms.h`, whose offsets it checks with `STATIC_ASSERT` at `0x101`,
//! `0x118`, `0x14C`; its components at `+0xA4` ammo and `+0xAC` clip rounds are where the exe's
//! destructor `0051db00` stores their vtables): the fields below. The real game with xNVSE 6.4.9
//! read them on the 9mm pistol, the combat knife, the hunting shotgun and the varmint rifle
//! (`measure/extenders/real-results/batch1.txt`), which the tests' weapon copies.

use super::{
    base_form, console_print, double_words, dynamic_cast, extract_float_and_form,
    extract_int_and_form, form_or_caller, is_console_mode, rtti, Read,
};
use crate::table::{Command, Param, Returns};
use crate::{handled, CommandArgs, EvalArgs};
use engine_core::prelude::*;

/// `TESObjectWEAP` fields (xNVSE).
pub mod field {
    pub const WEAPON_TYPE: u32 = 0xf4;
    pub const ANIM_MULT: u32 = 0xf8;
    pub const REACH: u32 = 0xfc;
    pub const FLAGS1: u32 = 0x100;
    pub const HAND_GRIP: u32 = 0x101;
    pub const AMMO_USE: u32 = 0x102;
    pub const RELOAD_ANIM: u32 = 0x103;
    pub const MIN_SPREAD: u32 = 0x104;
    pub const SPREAD: u32 = 0x108;
    pub const SIGHT_FOV: u32 = 0x110;
    pub const PROJECTILE: u32 = 0x118;
    pub const BASE_VATS_CHANCE: u32 = 0x11c;
    pub const ATTACK_ANIM: u32 = 0x11d;
    pub const NUM_PROJECTILES: u32 = 0x11e;
    pub const MIN_RANGE: u32 = 0x120;
    pub const MAX_RANGE: u32 = 0x124;
    pub const FLAGS2: u32 = 0x12c;
    pub const ANIM_ATTACK_MULT: u32 = 0x130;
    pub const FIRE_RATE: u32 = 0x134;
    pub const AP: u32 = 0x138;
    pub const RUMBLE_LEFT: u32 = 0x13c;
    pub const RUMBLE_RIGHT: u32 = 0x140;
    pub const RUMBLE_DURATION: u32 = 0x144;
    pub const ANIM_SHOTS_PER_SEC: u32 = 0x14c;
    pub const ANIM_RELOAD_TIME: u32 = 0x150;
    pub const ANIM_JAM_TIME: u32 = 0x154;
    pub const AIM_ARC: u32 = 0x158;
    pub const SKILL: u32 = 0x15c;
    pub const RUMBLE_WAVELENGTH: u32 = 0x164;
    pub const LIMB_DAMAGE_MULT: u32 = 0x168;
    pub const RESIST_TYPE: u32 = 0x16c;
    pub const SIGHT_USAGE: u32 = 0x170;
    pub const FIRE_DELAY_MIN: u32 = 0x174;
    pub const FIRE_DELAY_MAX: u32 = 0x178;
    pub const EFFECT_MODS: u32 = 0x180;
    pub const VALUE1_MODS: u32 = 0x18c;
    pub const STR_REQUIRED: u32 = 0x19c;
    pub const REGEN_RATE: u32 = 0x1a4;
    pub const VALUE2_MODS: u32 = 0x1ac;
    pub const SKILL_REQUIREMENT: u32 = 0x1bc;
    pub const CRITICAL_DAMAGE: u32 = 0x1c0;
    pub const CRITICAL_PERCENT: u32 = 0x1c4;
    pub const CRITICAL_EFFECT: u32 = 0x1cc;
    pub const ITEM_MODS: u32 = 0x350;
    pub const IS_LOOPING_RELOAD: u32 = 0x384;
    /// `BGSAmmoForm ammo` (`+0xA4`) and its `ammo` (`+4`).
    pub const AMMO: u32 = 0xa8;
}

/// `eFlag_IsAutomatic`, `eFlag_HasScope` (flags 1), `eFlag_LongBurst` (flags 2).
const FLAG_IS_AUTOMATIC: u8 = 0x2;
const FLAG_HAS_SCOPE: u8 = 0x4;
const FLAG_LONG_BURST: u32 = 0x800;
/// `kHandGripTable` (xNVSE `GameForms.cpp`).
const HAND_GRIPS: [u8; 7] = [0xff, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb];
/// `kAttackAnims` (xNVSE `GameForms.cpp`).
const ATTACK_ANIMS: [u8; 23] = [
    255, 38, 44, 50, 56, 62, 68, 26, 74, 32, 80, 86, 114, 120, 126, 132, 138, 102, 108, 144, 150,
    156, 162,
];
/// `kWeapType_Last`, `eReload_Count`, `eReload_W`, `eAttackAnim_Count`.
const WEAPON_TYPE_LAST: u32 = 14;
const RELOAD_COUNT: u32 = 23;
const RELOAD_W: u32 = 19;
const ATTACK_ANIM_COUNT: u32 = 23;
/// `eActorVal_SkillsStart`, `eActorVal_SkillsEnd` (Barter 32 to Unarmed 45).
const SKILLS: std::ops::RangeInclusive<u32> = 32..=45;

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), GameForms.cpp:
// TESObjectWEAP::AttackAnimation.
/// The index of the stored attack animation in [`ATTACK_ANIMS`] (`Default` 255 is 0), 255 for
/// one not in it. The switch lists the table's values in the table's order.
fn attack_animation(stored: u8) -> u8 {
    ATTACK_ANIMS
        .iter()
        .position(|&a| a == stored)
        .map_or(255, |i| i as u8)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), GameForms.cpp: TESObjectWEAP::HandGrip.
fn hand_grip(stored: u8) -> u8 {
    HAND_GRIPS
        .iter()
        .position(|&g| g == stored)
        .map_or(0, |i| i as u8)
}

/// What `GetWeaponValue` reads (`EWeapValues` (xNVSE)).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum W {
    Type,
    MinSpread,
    Spread,
    Proj,
    SightFov,
    MinRange,
    Range,
    AmmoUse,
    ApCost,
    CritDam,
    CritChance,
    CritEffect,
    FireRate,
    AnimAttackMult,
    RumbleLeft,
    RumbleRight,
    RumbleDuration,
    RumbleWaveLength,
    AnimShotsPerSec,
    AnimReloadTime,
    AnimJamTime,
    Skill,
    ResistType,
    FireDelayMin,
    FireDelayMax,
    AnimMult,
    Reach,
    IsAutomatic,
    HandGrip,
    ReloadAnim,
    VatsChance,
    AttackAnim,
    NumProj,
    AimArc,
    LimbDamageMult,
    SightUsage,
    ReqStr,
    ReqSkill,
    LongBursts,
    Flags1,
    Flags2,
    RegenRate,
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp: GetWeaponValue.
/// Writes the value into the result (a form's id into its low word); a null weapon writes
/// nothing.
fn weapon_value(e: &mut Engine, weapon: u32, which: W, result: u32) {
    use field::*;
    if weapon == 0 {
        return;
    }
    let f = |e: &Engine, at: u32| f64::from(e.mem.f32(weapon + at));
    let b = |e: &Engine, at: u32| f64::from(e.mem.u8(weapon + at));
    let d = |e: &Engine, at: u32| f64::from(e.mem.u32(weapon + at));
    let v = match which {
        W::Type => b(e, WEAPON_TYPE),
        W::MinSpread => f(e, MIN_SPREAD),
        W::Spread => f(e, SPREAD),
        W::Proj | W::CritEffect => {
            let at = if which == W::Proj {
                PROJECTILE
            } else {
                CRITICAL_EFFECT
            };
            let form = e.mem.u32(weapon + at);
            if form != 0 {
                let id = e.mem.u32(form + 0xc);
                e.mem.set_u32(result, id);
            }
            return;
        }
        W::SightFov => f(e, SIGHT_FOV),
        W::MinRange => f(e, MIN_RANGE),
        W::Range => f(e, MAX_RANGE),
        W::AmmoUse => b(e, AMMO_USE),
        W::ApCost => f(e, AP),
        W::CritDam => f64::from(e.mem.u16(weapon + CRITICAL_DAMAGE)),
        W::CritChance => f(e, CRITICAL_PERCENT),
        W::FireRate => f(e, FIRE_RATE),
        W::AnimAttackMult => f(e, ANIM_ATTACK_MULT),
        W::RumbleLeft => f(e, RUMBLE_LEFT),
        W::RumbleRight => f(e, RUMBLE_RIGHT),
        W::RumbleDuration => f(e, RUMBLE_DURATION),
        W::RumbleWaveLength => f(e, RUMBLE_WAVELENGTH),
        W::AnimShotsPerSec => f(e, ANIM_SHOTS_PER_SEC),
        W::AnimReloadTime => f(e, ANIM_RELOAD_TIME),
        W::AnimJamTime => f(e, ANIM_JAM_TIME),
        W::Skill => d(e, SKILL),
        W::ResistType => d(e, RESIST_TYPE),
        W::FireDelayMin => f(e, FIRE_DELAY_MIN),
        W::FireDelayMax => f(e, FIRE_DELAY_MAX),
        W::AnimMult => f(e, ANIM_MULT),
        W::Reach => f(e, REACH),
        W::IsAutomatic => f64::from(u8::from(e.mem.u8(weapon + FLAGS1) & FLAG_IS_AUTOMATIC != 0)),
        W::HandGrip => f64::from(hand_grip(e.mem.u8(weapon + HAND_GRIP))),
        W::ReloadAnim => b(e, RELOAD_ANIM),
        W::VatsChance => b(e, BASE_VATS_CHANCE),
        W::AttackAnim => f64::from(attack_animation(e.mem.u8(weapon + ATTACK_ANIM))),
        W::NumProj => b(e, NUM_PROJECTILES),
        W::AimArc => f(e, AIM_ARC),
        W::LimbDamageMult => f(e, LIMB_DAMAGE_MULT),
        W::SightUsage => f(e, SIGHT_USAGE),
        W::ReqStr => d(e, STR_REQUIRED),
        W::ReqSkill => d(e, SKILL_REQUIREMENT),
        W::LongBursts => f64::from(u8::from(e.mem.u32(weapon + FLAGS2) & FLAG_LONG_BURST != 0)),
        W::Flags1 => b(e, FLAGS1),
        W::Flags2 => d(e, FLAGS2),
        W::RegenRate => f(e, REGEN_RATE),
    };
    e.mem.set_f64(result, v);
}

fn weapon_of(e: &mut Engine, form: u32) -> u32 {
    dynamic_cast(e, form, rtti::TES_FORM, rtti::TES_OBJECT_WEAP)
}

/// The weapon a getter reads: its optional argument's base form, else the calling reference's.
fn getter_weapon(e: &mut Engine, a: &CommandArgs) -> Option<u32> {
    let read = Read::of(e, a);
    if !read.ok {
        return None;
    }
    let form = form_or_caller(e, read.form(0), a.this_obj)?;
    Some(weapon_of(e, form)).filter(|&w| w != 0)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// GetWeaponValue_Execute.
fn value_execute(e: &mut Engine, w: &[u32], which: W) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if let Some(weapon) = getter_weapon(e, &a) {
        weapon_value(e, weapon, which, a.result);
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// GetWeaponValue_Eval.
fn value_eval(e: &mut Engine, w: &[u32], which: W) -> Ret {
    let a = EvalArgs::new(w);
    e.mem.set_f64(a.result, 0.0);
    let form = if a.arg1 != 0 {
        a.arg1
    } else if a.this_obj != 0 {
        base_form(e, a.this_obj)
    } else {
        return handled(true);
    };
    let weapon = weapon_of(e, form);
    weapon_value(e, weapon, which, a.result);
    handled(true)
}

/// A getter's execute and eval, one pair per `EWeapValues` entry (xNVSE's `Cmd_GetWeapon*`).
macro_rules! getters {
    ($($exec:ident, $eval:ident => $which:ident;)*) => {
        $(
            fn $exec(e: &mut Engine, w: &[u32]) -> Ret {
                value_execute(e, w, W::$which)
            }
            fn $eval(e: &mut Engine, w: &[u32]) -> Ret {
                value_eval(e, w, W::$which)
            }
        )*
    };
}

getters! {
    get_type, get_type_eval => Type;
    get_min_spread, get_min_spread_eval => MinSpread;
    get_spread, get_spread_eval => Spread;
    get_projectile, _get_projectile_eval => Proj;
    get_sight_fov, get_sight_fov_eval => SightFov;
    get_min_range, get_min_range_eval => MinRange;
    get_max_range, get_max_range_eval => Range;
    get_ammo_use, get_ammo_use_eval => AmmoUse;
    get_action_points, get_action_points_eval => ApCost;
    get_crit_damage, get_crit_damage_eval => CritDam;
    get_crit_chance, get_crit_chance_eval => CritChance;
    get_crit_effect, _get_crit_effect_eval => CritEffect;
    get_fire_rate, get_fire_rate_eval => FireRate;
    get_anim_attack_mult, get_anim_attack_mult_eval => AnimAttackMult;
    get_rumble_left, get_rumble_left_eval => RumbleLeft;
    get_rumble_right, get_rumble_right_eval => RumbleRight;
    get_rumble_duration, get_rumble_duration_eval => RumbleDuration;
    get_rumble_wavelength, get_rumble_wavelength_eval => RumbleWaveLength;
    get_anim_shots_per_sec, get_anim_shots_per_sec_eval => AnimShotsPerSec;
    get_anim_reload_time, get_anim_reload_time_eval => AnimReloadTime;
    get_anim_jam_time, get_anim_jam_time_eval => AnimJamTime;
    get_skill, get_skill_eval => Skill;
    get_resist_type, get_resist_type_eval => ResistType;
    get_fire_delay_min, get_fire_delay_min_eval => FireDelayMin;
    get_fire_delay_max, get_fire_delay_max_eval => FireDelayMax;
    get_anim_mult, get_anim_mult_eval => AnimMult;
    get_reach, get_reach_eval => Reach;
    get_is_automatic, get_is_automatic_eval => IsAutomatic;
    get_hand_grip, get_hand_grip_eval => HandGrip;
    get_reload_anim, get_reload_anim_eval => ReloadAnim;
    get_vats_chance, get_vats_chance_eval => VatsChance;
    get_attack_animation, get_attack_animation_eval => AttackAnim;
    get_num_projectiles, get_num_projectiles_eval => NumProj;
    get_aim_arc, get_aim_arc_eval => AimArc;
    get_limb_damage_mult, get_limb_damage_mult_eval => LimbDamageMult;
    get_sight_usage, get_sight_usage_eval => SightUsage;
    get_required_strength, get_required_strength_eval => ReqStr;
    get_required_skill, get_required_skill_eval => ReqSkill;
    get_long_bursts, get_long_bursts_eval => LongBursts;
    get_flags1, get_flags1_eval => Flags1;
    get_flags2, get_flags2_eval => Flags2;
    get_regen_rate, get_regen_rate_eval => RegenRate;
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetWeaponHasScope_Eval.
fn has_scope_eval(e: &mut Engine, w: &[u32]) -> Ret {
    let a = EvalArgs::new(w);
    e.mem.set_f64(a.result, 0.0);
    let form = if a.arg1 != 0 {
        a.arg1
    } else if a.this_obj != 0 {
        base_form(e, a.this_obj)
    } else {
        return handled(true);
    };
    let weapon = weapon_of(e, form);
    if weapon != 0 {
        let scope = e.mem.u8(weapon + field::FLAGS1) & FLAG_HAS_SCOPE != 0;
        e.mem.set_f64(a.result, f64::from(u8::from(scope)));
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetWeaponHasScope_Execute.
fn has_scope(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let read = Read::of(e, &a);
    if read.ok {
        if let Some(form) = form_or_caller(e, read.form(0), a.this_obj) {
            return has_scope_eval(e, &[a.this_obj, form, 0, a.result]);
        }
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetWeaponAmmo_Execute.
fn get_ammo(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let read = Read::of(e, &a);
    if !read.ok {
        return handled(true);
    }
    let Some(form) = form_or_caller(e, read.form(0), a.this_obj) else {
        return handled(true);
    };
    let ammo_form = dynamic_cast(e, form, rtti::TES_FORM, rtti::BGS_AMMO_FORM);
    if ammo_form != 0 {
        let ammo = e.mem.u32(ammo_form + 4);
        if ammo != 0 {
            let id = e.mem.u32(ammo + 0xc);
            a.set_form_result(e, id);
        }
    }
    handled(true)
}

/// `Extract_IntAndWeapon` (xNVSE): the integer and the weapon of the optional form.
fn int_and_weapon(e: &mut Engine, a: &CommandArgs) -> (u32, u32) {
    let (value, form) = extract_int_and_form(e, a);
    let weapon = form.map_or(0, |f| weapon_of(e, f));
    (value, weapon)
}

/// `Extract_FloatAndWeapon` (xNVSE).
fn float_and_weapon(e: &mut Engine, a: &CommandArgs) -> (f32, u32) {
    let (value, form) = extract_float_and_form(e, a);
    let weapon = form.map_or(0, |f| weapon_of(e, f));
    (value, weapon)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetWeaponItemMod_Execute.
fn get_item_mod(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let (which, weapon) = int_and_weapon(e, &a);
    // `GetItemMod(UInt8 which)`: 1 to 3, anything else none.
    if weapon != 0 && (1..=3).contains(&(which & 0xff)) {
        let item_mod = e
            .mem
            .u32(weapon + field::ITEM_MODS + 4 * ((which & 0xff) - 1));
        if item_mod != 0 {
            let id = e.mem.u32(item_mod + 0xc);
            a.set_form_result(e, id);
        }
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetWeaponItemModEffect_Execute, Cmd_GetWeaponItemModValue1_Execute,
// Cmd_GetWeaponItemModValue2_Execute.
fn item_mod_value(e: &mut Engine, w: &[u32], at: u32, float: bool, format: &str) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let (which, weapon) = int_and_weapon(e, &a);
    if weapon == 0 || !(1..=3).contains(&which) {
        return handled(true);
    }
    // `which` passes through a `UInt8` parameter (`which -= 1` there).
    let slot = weapon + at + 4 * (which - 1);
    let v = if float {
        f64::from(e.mem.f32(slot))
    } else {
        f64::from(e.mem.u32(slot))
    };
    a.set_result(e, v);
    if is_console_mode(e) {
        if float {
            let [lo, hi] = double_words(v);
            console_print(e, format, &[which, lo, hi]);
        } else {
            console_print(e, format, &[which, e.mem.u32(slot)]);
        }
    }
    handled(true)
}

fn get_item_mod_effect(e: &mut Engine, w: &[u32]) -> Ret {
    item_mod_value(e, w, field::EFFECT_MODS, false, "Item Mod %d Effect: %d")
}

fn get_item_mod_value1(e: &mut Engine, w: &[u32]) -> Ret {
    item_mod_value(e, w, field::VALUE1_MODS, true, "Item Mod %d Value1: %f")
}

fn get_item_mod_value2(e: &mut Engine, w: &[u32]) -> Ret {
    item_mod_value(e, w, field::VALUE2_MODS, true, "Item Mod %d Value2: %f")
}

/// A float setter: `Extract_FloatAndWeapon`, then the field.
fn set_float(e: &mut Engine, w: &[u32], at: u32) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let (value, weapon) = float_and_weapon(e, &a);
    if weapon != 0 {
        e.mem.set_f32(weapon + at, value);
    }
    handled(true)
}

/// An integer setter: `Extract_IntAndWeapon` (or `Extract_IntAndForm` and the cast, the same),
/// a check of the value, then `store`.
fn set_int(
    e: &mut Engine,
    w: &[u32],
    check: fn(u32) -> bool,
    store: fn(&mut Engine, u32, u32),
) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let (value, weapon) = int_and_weapon(e, &a);
    if weapon != 0 && check(value) {
        store(e, weapon, value);
    }
    handled(true)
}

fn any(_: u32) -> bool {
    true
}

macro_rules! float_setters {
    ($($name:ident => $field:ident;)*) => {
        $(
            fn $name(e: &mut Engine, w: &[u32]) -> Ret {
                set_float(e, w, field::$field)
            }
        )*
    };
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp: the float
// setters (Cmd_SetWeaponMinSpread_Execute ... Cmd_SetWeaponRegenRate_Execute).
float_setters! {
    set_min_spread => MIN_SPREAD;
    set_spread => SPREAD;
    set_sight_fov => SIGHT_FOV;
    set_min_range => MIN_RANGE;
    set_max_range => MAX_RANGE;
    set_action_points => AP;
    set_crit_chance => CRITICAL_PERCENT;
    set_fire_rate => FIRE_RATE;
    set_anim_attack_mult => ANIM_ATTACK_MULT;
    set_anim_mult => ANIM_MULT;
    set_reach => REACH;
    set_aim_arc => AIM_ARC;
    set_limb_damage_mult => LIMB_DAMAGE_MULT;
    set_sight_usage => SIGHT_USAGE;
    set_regen_rate => REGEN_RATE;
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp: the integer
// setters, each with its check.
fn set_type(e: &mut Engine, w: &[u32]) -> Ret {
    set_int(
        e,
        w,
        |v| v < WEAPON_TYPE_LAST,
        |e, wp, v| e.mem.set_u8(wp + field::WEAPON_TYPE, v as u8),
    )
}

fn set_ammo_use(e: &mut Engine, w: &[u32]) -> Ret {
    set_int(e, w, any, |e, wp, v| {
        e.mem.set_u8(wp + field::AMMO_USE, v as u8)
    })
}

fn set_crit_damage(e: &mut Engine, w: &[u32]) -> Ret {
    set_int(e, w, any, |e, wp, v| {
        e.mem.set_u16(wp + field::CRITICAL_DAMAGE, v as u16)
    })
}

fn set_is_automatic(e: &mut Engine, w: &[u32]) -> Ret {
    set_int(e, w, any, |e, wp, v| {
        let flags = e.mem.u8(wp + field::FLAGS1);
        let flags = if v == 1 {
            flags | FLAG_IS_AUTOMATIC
        } else {
            flags & !FLAG_IS_AUTOMATIC
        };
        e.mem.set_u8(wp + field::FLAGS1, flags);
    })
}

fn set_hand_grip(e: &mut Engine, w: &[u32]) -> Ret {
    set_int(
        e,
        w,
        |v| v < 7,
        |e, wp, v| {
            // `SetHandGrip(UInt8)`.
            if let Some(&g) = HAND_GRIPS.get((v & 0xff) as usize) {
                e.mem.set_u8(wp + field::HAND_GRIP, g);
            }
        },
    )
}

fn set_reload_anim(e: &mut Engine, w: &[u32]) -> Ret {
    set_int(
        e,
        w,
        |v| v < RELOAD_COUNT,
        |e, wp, v| {
            e.mem.set_u8(wp + field::RELOAD_ANIM, v as u8);
            // `isLoopingReload`, as the exe sets it at 0x51E283.
            e.mem
                .set_u8(wp + field::IS_LOOPING_RELOAD, u8::from(v >= RELOAD_W));
        },
    )
}

fn set_vats_chance(e: &mut Engine, w: &[u32]) -> Ret {
    set_int(
        e,
        w,
        |v| v <= 100,
        |e, wp, v| e.mem.set_u8(wp + field::BASE_VATS_CHANCE, v as u8),
    )
}

fn set_attack_animation(e: &mut Engine, w: &[u32]) -> Ret {
    set_int(
        e,
        w,
        |v| v < ATTACK_ANIM_COUNT,
        |e, wp, v| {
            e.mem
                .set_u8(wp + field::ATTACK_ANIM, ATTACK_ANIMS[v as usize]);
        },
    )
}

fn set_num_projectiles(e: &mut Engine, w: &[u32]) -> Ret {
    set_int(
        e,
        w,
        |v| v <= 255,
        |e, wp, v| e.mem.set_u8(wp + field::NUM_PROJECTILES, v as u8),
    )
}

fn set_required_strength(e: &mut Engine, w: &[u32]) -> Ret {
    set_int(e, w, any, |e, wp, v| {
        e.mem.set_u32(wp + field::STR_REQUIRED, v)
    })
}

fn set_required_skill(e: &mut Engine, w: &[u32]) -> Ret {
    set_int(e, w, any, |e, wp, v| {
        e.mem.set_u32(wp + field::SKILL_REQUIREMENT, v)
    })
}

fn set_skill(e: &mut Engine, w: &[u32]) -> Ret {
    set_int(
        e,
        w,
        |v| SKILLS.contains(&v),
        |e, wp, v| e.mem.set_u32(wp + field::SKILL, v),
    )
}

fn set_resist_type(e: &mut Engine, w: &[u32]) -> Ret {
    set_int(e, w, any, |e, wp, v| {
        e.mem.set_u32(wp + field::RESIST_TYPE, v)
    })
}

fn set_long_bursts(e: &mut Engine, w: &[u32]) -> Ret {
    set_int(e, w, any, |e, wp, v| {
        let flags = e.mem.u32(wp + field::FLAGS2);
        let flags = if v != 0 {
            flags | FLAG_LONG_BURST
        } else {
            flags & !FLAG_LONG_BURST
        };
        e.mem.set_u32(wp + field::FLAGS2, flags);
    })
}

fn set_flags1(e: &mut Engine, w: &[u32]) -> Ret {
    set_int(e, w, any, |e, wp, v| {
        e.mem.set_u8(wp + field::FLAGS1, v as u8)
    })
}

fn set_flags2(e: &mut Engine, w: &[u32]) -> Ret {
    set_int(e, w, any, |e, wp, v| e.mem.set_u32(wp + field::FLAGS2, v))
}

/// `BGSClipRoundsForm::clipRounds` (a byte) and `TESAttackDamageForm::damage` (16 bits), through
/// their components, as `GetWeaponClipRounds` and `GetAttackDamage` read them.
fn set_component(e: &mut Engine, w: &[u32], class: u32, store: fn(&mut Engine, u32, u32)) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let (value, form) = extract_int_and_form(e, &a);
    if let Some(form) = form {
        let component = dynamic_cast(e, form, rtti::TES_FORM, class);
        if component != 0 {
            store(e, component, value);
        }
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_SetWeaponClipRounds_Execute.
fn set_clip_rounds(e: &mut Engine, w: &[u32]) -> Ret {
    set_component(e, w, rtti::BGS_CLIP_ROUNDS_FORM, |e, c, v| {
        e.mem.set_u8(c + 4, v as u8)
    })
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_SetAttackDamage_Execute.
fn set_attack_damage(e: &mut Engine, w: &[u32]) -> Ret {
    set_component(e, w, rtti::TES_ATTACK_DAMAGE_FORM, |e, c, v| {
        e.mem.set_u16(c + 4, v as u16)
    })
}

/// The two forms of `SetWeaponAmmo` and `SetWeaponProjectile` (`ExtractArgsEx(&what, &form)`):
/// the first form and the weapon, `None` to stop.
fn form_and_weapon(e: &mut Engine, a: &CommandArgs) -> Option<(u32, u32)> {
    let read = Read::of(e, a);
    if !read.ok {
        return None;
    }
    let target = form_or_caller(e, read.form(1), a.this_obj)?;
    Some((read.form(0), target))
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_SetWeaponAmmo_Execute.
/// Ammo or a form list of ammo; anything else changes nothing.
fn set_ammo(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if let Some((what, form)) = form_and_weapon(e, &a) {
        let weapon = weapon_of(e, form);
        if weapon != 0 {
            let ammo = dynamic_cast(e, what, rtti::TES_FORM, super::rtti::TES_AMMO);
            let list = dynamic_cast(e, what, rtti::TES_FORM, rtti::BGS_LIST_FORM);
            if ammo != 0 {
                e.mem.set_u32(weapon + field::AMMO, ammo);
            } else if list != 0 {
                e.mem.set_u32(weapon + field::AMMO, list);
            }
        }
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_SetWeaponProjectile_Execute.
fn set_projectile(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if let Some((what, form)) = form_and_weapon(e, &a) {
        let projectile = dynamic_cast(e, what, rtti::TES_FORM, rtti::BGS_PROJECTILE);
        if projectile != 0 {
            let weapon = weapon_of(e, form);
            if weapon != 0 {
                e.mem.set_u32(weapon + field::PROJECTILE, projectile);
            }
        }
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_SetWeaponCritEffect_Execute.
/// The magic item must be a spell (the reader already cast the form to `MagicItem`).
fn set_crit_effect(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if let Some((magic_item, form)) = form_and_weapon(e, &a) {
        let spell = dynamic_cast(e, magic_item, rtti::MAGIC_ITEM, rtti::SPELL_ITEM);
        if spell != 0 {
            let weapon = weapon_of(e, form);
            if weapon != 0 {
                e.mem.set_u32(weapon + field::CRITICAL_EFFECT, spell);
            }
        }
    }
    handled(true)
}

const fn p(name: &'static str, kind: u32, optional: bool) -> Param {
    Param {
        name,
        kind,
        optional,
    }
}
/// `kParams_OneOptionalObjectID`, `kParams_OneInt_OneOptionalObjectID`,
/// `kParams_OneFloat_OneOptionalObjectID`, `kParams_OneMagicItem_OneOptionalObjectID`,
/// `kParams_OneInventoryItem_OneOptionalObjectID` (xNVSE).
const OPT_ITEM: &[Param] = &[p("item", 0x03, true)];
const INT_OPT_ITEM: &[Param] = &[p("path type", 0x01, false), p("item", 0x03, true)];
const FLOAT_OPT_ITEM: &[Param] = &[p("float", 0x02, false), p("item", 0x03, true)];
const MAGIC_OPT_ITEM: &[Param] = &[p("magic item", 0x0b, false), p("item", 0x03, true)];
const INV_ITEM_OPT_TARGET: &[Param] = &[p("inv item", 0x3d, false), p("target item", 0x03, true)];

/// A getter with a condition eval (`DEFINE_GET_INV_COND`).
const fn getter(
    opcode: u32,
    name: &'static str,
    short: &'static str,
    help: &'static str,
    execute: AbiFn,
    eval: AbiFn,
) -> Command {
    Command {
        opcode,
        name,
        short,
        help,
        needs_ref: false,
        params: OPT_ITEM,
        execute,
        eval: Some(eval),
        returns: Returns::Default,
    }
}

/// A command with no eval.
const fn plain(
    opcode: u32,
    name: &'static str,
    short: &'static str,
    help: &'static str,
    params: &'static [Param],
    execute: AbiFn,
    returns: Returns,
) -> Command {
    Command {
        opcode,
        name,
        short,
        help,
        needs_ref: false,
        params,
        execute,
        eval: None,
        returns,
    }
}

const D: Returns = Returns::Default;

pub static COMMANDS: &[Command] = &[
    plain(
        0x140d,
        "GetWeaponAmmo",
        "GetAmmo",
        "returns the ammo of a weapon",
        OPT_ITEM,
        get_ammo,
        Returns::Form,
    ),
    getter(
        0x1410,
        "GetWeaponType",
        "GetWeapType",
        "returns the weapon type",
        get_type,
        get_type_eval,
    ),
    getter(
        0x1411,
        "GetWeaponMinSpread",
        "GetMinSpread",
        "returns the minimum spread of the weapon",
        get_min_spread,
        get_min_spread_eval,
    ),
    getter(
        0x1412,
        "GetWeaponSpread",
        "GetSpread",
        "returns the spread of the weapon",
        get_spread,
        get_spread_eval,
    ),
    plain(
        0x1413,
        "GetWeaponProjectile",
        "GetWeapProj",
        "returns the weapon projectile info",
        OPT_ITEM,
        get_projectile,
        Returns::Form,
    ),
    getter(
        0x1414,
        "GetWeaponSightFOV",
        "GetSightFOV",
        "returns the zoom field of view for the weapon",
        get_sight_fov,
        get_sight_fov_eval,
    ),
    getter(
        0x1415,
        "GetWeaponMinRange",
        "GetMinRange",
        "returns the min range of the weapon",
        get_min_range,
        get_min_range_eval,
    ),
    getter(
        0x1416,
        "GetWeaponMaxRange",
        "GetMaxRange",
        "returns the max range of the weapon",
        get_max_range,
        get_max_range_eval,
    ),
    getter(
        0x1417,
        "GetWeaponAmmoUse",
        "GetAmmoUse",
        "returns the ammo used per shot of the weapon",
        get_ammo_use,
        get_ammo_use_eval,
    ),
    getter(
        0x1418,
        "GetWeaponActionPoints",
        "GetAP",
        "returns the number of action points per shot of the weapon",
        get_action_points,
        get_action_points_eval,
    ),
    getter(
        0x1419,
        "GetWeaponCritDamage",
        "GetCritDam",
        "returns the critical damage of the weapon",
        get_crit_damage,
        get_crit_damage_eval,
    ),
    getter(
        0x141a,
        "GetWeaponCritChance",
        "GetCritPerc",
        "returns the chance of a critical shot for the weapon",
        get_crit_chance,
        get_crit_chance_eval,
    ),
    plain(
        0x141b,
        "GetWeaponCritEffect",
        "GetCritEffect",
        "returns the spell for the critical effect for the weapon",
        OPT_ITEM,
        get_crit_effect,
        Returns::Form,
    ),
    getter(
        0x141c,
        "GetWeaponFireRate",
        "GetFireRate",
        "returns the fire rate of the weapon.",
        get_fire_rate,
        get_fire_rate_eval,
    ),
    getter(
        0x141d,
        "GetWeaponAnimAttackMult",
        "GetAnimAttackMult",
        "returns the animation attack multiplier of the weapon.",
        get_anim_attack_mult,
        get_anim_attack_mult_eval,
    ),
    getter(
        0x141e,
        "GetWeaponRumbleLeftMotor",
        "GetRumbleLeft",
        "returns the rumble left motor of the weapon.",
        get_rumble_left,
        get_rumble_left_eval,
    ),
    getter(
        0x141f,
        "GetWeaponRumbleRightMotor",
        "GetRumbleRight",
        "returns the rumble right motor of the weapon.",
        get_rumble_right,
        get_rumble_right_eval,
    ),
    getter(
        0x1420,
        "GetWeaponRumbleDuration",
        "GetRumbleDuration",
        "returns the rumble duration of the weapon.",
        get_rumble_duration,
        get_rumble_duration_eval,
    ),
    getter(
        0x1421,
        "GetWeaponRumbleWavelength",
        "GetRumbleWavelen",
        "returns the rumble wavelegnth for the weapon.",
        get_rumble_wavelength,
        get_rumble_wavelength_eval,
    ),
    getter(
        0x1422,
        "GetWeaponAnimShotsPerSec",
        "GetAnimShotsPerSec",
        "returns the animation shots per second of the weapon.",
        get_anim_shots_per_sec,
        get_anim_shots_per_sec_eval,
    ),
    getter(
        0x1423,
        "GetWeaponAnimReloadTime",
        "GetAnimReloadTime",
        "retuns the animation reload time for the weapon.",
        get_anim_reload_time,
        get_anim_reload_time_eval,
    ),
    getter(
        0x1424,
        "GetWeaponAnimJamTime",
        "GetAnimJamTime",
        "returns the animation jam time of the weapon.",
        get_anim_jam_time,
        get_anim_jam_time_eval,
    ),
    getter(
        0x1425,
        "GetWeaponSkill",
        "",
        "returns the skill for the weapon.",
        get_skill,
        get_skill_eval,
    ),
    getter(
        0x1426,
        "GetWeaponResistType",
        "GetWeaponResist",
        "returns the resist type for the weapon.",
        get_resist_type,
        get_resist_type_eval,
    ),
    getter(
        0x1427,
        "GetWeaponFireDelayMin",
        "GetFireDelayMin",
        "returns the semi-auto min fire delay for the weapon.",
        get_fire_delay_min,
        get_fire_delay_min_eval,
    ),
    getter(
        0x1428,
        "GetWeaponFireDelayMax",
        "GetFireDelayMax",
        "returns the semi-auto max fire delay for the weapon.",
        get_fire_delay_max,
        get_fire_delay_max_eval,
    ),
    getter(
        0x1429,
        "GetWeaponAnimMult",
        "GetAnimMult",
        "returns the animation multiplier for the weapon",
        get_anim_mult,
        get_anim_mult_eval,
    ),
    getter(
        0x142a,
        "GetWeaponReach",
        "GetReach",
        "returns the reach of the weapon",
        get_reach,
        get_reach_eval,
    ),
    getter(
        0x142b,
        "GetWeaponIsAutomatic",
        "GetIsAutomatic",
        "returns 1 if the weapon is an automatic weapon",
        get_is_automatic,
        get_is_automatic_eval,
    ),
    getter(
        0x142c,
        "GetWeaponHandGrip",
        "GetHandGrip",
        "returns the hand grip of the weapon",
        get_hand_grip,
        get_hand_grip_eval,
    ),
    getter(
        0x142d,
        "GetWeaponReloadAnim",
        "GetReloadAnim",
        "returns the reload animation of the weapon",
        get_reload_anim,
        get_reload_anim_eval,
    ),
    getter(
        0x142e,
        "GetWeaponBaseVATSChance",
        "GetVATSChance",
        "returns the base VATS chance of the weapon",
        get_vats_chance,
        get_vats_chance_eval,
    ),
    getter(
        0x142f,
        "GetWeaponAttackAnimation",
        "GetAttackAnim",
        "returns the attack animation of the weapon",
        get_attack_animation,
        get_attack_animation_eval,
    ),
    getter(
        0x1430,
        "GetWeaponNumProjectiles",
        "GetNumProj",
        "returns the number of projectiles used in a single shot by the weapon.",
        get_num_projectiles,
        get_num_projectiles_eval,
    ),
    getter(
        0x1431,
        "GetWeaponAimArc",
        "GetAimArc",
        "returns the aim arc of the weapon.",
        get_aim_arc,
        get_aim_arc_eval,
    ),
    getter(
        0x1432,
        "GetWeaponLimbDamageMult",
        "GetLimbDamageMult",
        "returns the limb damage multiplier of the weapon.",
        get_limb_damage_mult,
        get_limb_damage_mult_eval,
    ),
    getter(
        0x1433,
        "GetWeaponSightUsage",
        "GetSightUsage",
        "returns the sight usage of the weapon.",
        get_sight_usage,
        get_sight_usage_eval,
    ),
    getter(
        0x1434,
        "GetWeaponHasScope",
        "",
        "returns whether the weapon has a scope or not.",
        has_scope,
        has_scope_eval,
    ),
    plain(
        0x147f,
        "SetWeaponAmmo",
        "SetAmmo",
        "sets the ammo of the weapon",
        INV_ITEM_OPT_TARGET,
        set_ammo,
        D,
    ),
    plain(
        0x1480,
        "SetWeaponClipRounds",
        "SetClipSize",
        "sets the weapon clip size",
        INT_OPT_ITEM,
        set_clip_rounds,
        D,
    ),
    plain(
        0x1481,
        "SetAttackDamage",
        "SetDamage",
        "sets the attack damage of the form",
        INT_OPT_ITEM,
        set_attack_damage,
        D,
    ),
    plain(
        0x1482,
        "SetWeaponType",
        "",
        "sets the weapon type of the weapon.",
        INT_OPT_ITEM,
        set_type,
        D,
    ),
    plain(
        0x1483,
        "SetWeaponMinSpread",
        "SetMinSpread",
        "sets the weapon min spread",
        FLOAT_OPT_ITEM,
        set_min_spread,
        D,
    ),
    plain(
        0x1484,
        "SetWeaponSpread",
        "SetSpread",
        "sets the weapon spread",
        FLOAT_OPT_ITEM,
        set_spread,
        D,
    ),
    plain(
        0x1485,
        "SetWeaponProjectile",
        "SetProjectile",
        "sets the projectile of the weapon",
        INV_ITEM_OPT_TARGET,
        set_projectile,
        D,
    ),
    plain(
        0x1486,
        "SetWeaponSightFOV",
        "SetSightFOV",
        "sets the weapon xoom field of view",
        FLOAT_OPT_ITEM,
        set_sight_fov,
        D,
    ),
    plain(
        0x1487,
        "SetWeaponMinRange",
        "SetMinRange",
        "sets the weapon min range",
        FLOAT_OPT_ITEM,
        set_min_range,
        D,
    ),
    plain(
        0x1488,
        "SetWeaponMaxRange",
        "SetMaxRange",
        "sets the weapon max range.",
        FLOAT_OPT_ITEM,
        set_max_range,
        D,
    ),
    plain(
        0x1489,
        "SetWeaponAmmoUse",
        "SetAmmoUse",
        "sets the weapon ammo use",
        INT_OPT_ITEM,
        set_ammo_use,
        D,
    ),
    plain(
        0x148a,
        "SetWeaponActionPoints",
        "SetAP",
        "sets the weapon number of action pointer per shot",
        FLOAT_OPT_ITEM,
        set_action_points,
        D,
    ),
    plain(
        0x148b,
        "SetWeaponCritDamage",
        "SetCritDam",
        "sets the weapon critical hit damage.",
        INT_OPT_ITEM,
        set_crit_damage,
        D,
    ),
    plain(
        0x148c,
        "SetWeaponCritChance",
        "SetCritPerc",
        "sets the weapon critical hit chance",
        FLOAT_OPT_ITEM,
        set_crit_chance,
        D,
    ),
    plain(
        0x148d,
        "SetWeaponCritEffect",
        "SetCritEffect",
        "sets the weapon critical hit effect",
        MAGIC_OPT_ITEM,
        set_crit_effect,
        D,
    ),
    plain(
        0x148e,
        "SetWeaponAnimAttackMult",
        "SetAnimAttackMult",
        "sets the anim attack multiplier.",
        FLOAT_OPT_ITEM,
        set_anim_attack_mult,
        D,
    ),
    plain(
        0x148f,
        "SetWeaponAnimMult",
        "SetAnimMult",
        "sets the animiation multiple of the weapon.",
        FLOAT_OPT_ITEM,
        set_anim_mult,
        D,
    ),
    plain(
        0x1490,
        "SetWeaponReach",
        "SetReach",
        "sets the reach of the weapon.",
        FLOAT_OPT_ITEM,
        set_reach,
        D,
    ),
    plain(
        0x1491,
        "SetWeaponIsAutomatic",
        "SetIsAutomatic",
        "sets whether the weapon is an automatic weapon or not.",
        INT_OPT_ITEM,
        set_is_automatic,
        D,
    ),
    plain(
        0x1492,
        "SetWeaponHandGrip",
        "SetHandGrip",
        "sets the hand grip of the weapon.",
        INT_OPT_ITEM,
        set_hand_grip,
        D,
    ),
    plain(
        0x1493,
        "SetWeaponReloadAnim",
        "SetReloadAnim",
        "sets the reload animation of the weapon.",
        INT_OPT_ITEM,
        set_reload_anim,
        D,
    ),
    plain(
        0x1494,
        "SetWeaponBaseVATSChance",
        "SetVATSChance",
        "sets the base VATS chance for the weapon.",
        INT_OPT_ITEM,
        set_vats_chance,
        D,
    ),
    plain(
        0x1495,
        "SetWeaponAttackAnimation",
        "SetAttackAnim",
        "sets the attack animation for the weapon.",
        INT_OPT_ITEM,
        set_attack_animation,
        D,
    ),
    plain(
        0x1496,
        "SetWeaponNumProjectiles",
        "SetNumProj",
        "sets the number of projectiles the weapon fires each time it is shot.",
        INT_OPT_ITEM,
        set_num_projectiles,
        D,
    ),
    plain(
        0x1497,
        "SetWeaponAimArc",
        "SetAimArc",
        "sets the aim arc of the weapon.",
        FLOAT_OPT_ITEM,
        set_aim_arc,
        D,
    ),
    plain(
        0x1498,
        "SetWeaponLimbDamageMult",
        "SetLimbDamageMult",
        "sets the limb damage multiplier for the weapon.",
        FLOAT_OPT_ITEM,
        set_limb_damage_mult,
        D,
    ),
    plain(
        0x1499,
        "SetWeaponSightUsage",
        "SetSightUsage",
        "sets the sight usage for the weapon.",
        FLOAT_OPT_ITEM,
        set_sight_usage,
        D,
    ),
    plain(
        0x14ad,
        "GetWeaponItemMod",
        "",
        "returns the specified item mod of the weapon",
        INT_OPT_ITEM,
        get_item_mod,
        Returns::Form,
    ),
    getter(
        0x14bc,
        "GetWeaponRequiredStrength",
        "GetReqStr",
        "returns the required strength for the weapon.",
        get_required_strength,
        get_required_strength_eval,
    ),
    getter(
        0x14bd,
        "GetWeaponRequiredSkill",
        "GetReqSkill",
        "returns the required strength for the weapon.",
        get_required_skill,
        get_required_skill_eval,
    ),
    plain(
        0x14be,
        "SetWeaponRequiredStrength",
        "SetReqStr",
        "sets the required strength for the weapon",
        INT_OPT_ITEM,
        set_required_strength,
        D,
    ),
    plain(
        0x14bf,
        "SetWeaponRequiredSkill",
        "SetReqSkill",
        "sets the required skill for the weapon",
        INT_OPT_ITEM,
        set_required_skill,
        D,
    ),
    plain(
        0x14c0,
        "SetWeaponResistType",
        "SetWeaponResist",
        "sets the weapon clip size",
        INT_OPT_ITEM,
        set_resist_type,
        D,
    ),
    plain(
        0x14c1,
        "SetWeaponSkill",
        "",
        "sets the skill for the weapon",
        INT_OPT_ITEM,
        set_skill,
        D,
    ),
    plain(
        0x14e5,
        "SetWeaponFireRate",
        "SetFireRate",
        "sets the weapon fire rate",
        FLOAT_OPT_ITEM,
        set_fire_rate,
        D,
    ),
    getter(
        0x14e6,
        "GetWeaponLongBursts",
        "GetLongBursts",
        "returns if a weapon uses long bursts",
        get_long_bursts,
        get_long_bursts_eval,
    ),
    plain(
        0x14e7,
        "SetWeaponLongBursts",
        "SetLongBursts",
        "sets if a weapon uses long bursts",
        INT_OPT_ITEM,
        set_long_bursts,
        D,
    ),
    getter(
        0x14e8,
        "GetWeaponFlags1",
        "",
        "returns weapon flags bitfield 1",
        get_flags1,
        get_flags1_eval,
    ),
    getter(
        0x14e9,
        "GetWeaponFlags2",
        "",
        "returns weapon flags bitfield 2",
        get_flags2,
        get_flags2_eval,
    ),
    plain(
        0x14ea,
        "SetWeaponFlags1",
        "",
        "sets weapon flags bitfield 1",
        INT_OPT_ITEM,
        set_flags1,
        D,
    ),
    plain(
        0x14eb,
        "SetWeaponFlags2",
        "",
        "sets weapon flags bitfield 2",
        INT_OPT_ITEM,
        set_flags2,
        D,
    ),
    plain(
        0x14f4,
        "GetWeaponItemModEffect",
        "",
        "returns the specified item mod effect id of the weapon",
        INT_OPT_ITEM,
        get_item_mod_effect,
        D,
    ),
    plain(
        0x14f5,
        "GetWeaponItemModValue1",
        "",
        "returns the specified item mod value1 of the weapon",
        INT_OPT_ITEM,
        get_item_mod_value1,
        D,
    ),
    plain(
        0x14f6,
        "GetWeaponItemModValue2",
        "",
        "returns the specified item mod value2 of the weapon",
        INT_OPT_ITEM,
        get_item_mod_value2,
        D,
    ),
    getter(
        0x1633,
        "GetWeaponRegenRate",
        "",
        "\"\"",
        get_regen_rate,
        get_regen_rate_eval,
    ),
    plain(
        0x1634,
        "SetWeaponRegenRate",
        "",
        "\"\"",
        FLOAT_OPT_ITEM,
        set_regen_rate,
        D,
    ),
];

#[cfg(test)]
pub(crate) mod tests {
    use super::super::test_support::*;
    use super::field::*;
    use super::*;

    pub const NINE_MM: u32 = 0x0200_8000;
    pub const REFERENCE: u32 = 0x0200_7000;

    /// The real 9mm pistol's values (form 000E3778, `b_9mm` in `real-results/batch1.txt`) in a
    /// weapon laid out as `TESObjectWEAP`.
    pub fn nine_mm(e: &mut Engine) {
        let w = NINE_MM;
        e.mem.set_u32(w + 0x7f0, 256);
        e.mem.set_u32(w + 0xc, 0x000e_3778);
        e.mem.set_u8(w + 4, 0x28);
        e.mem.set_u8(w + WEAPON_TYPE, 3);
        e.mem.set_f32(w + MIN_SPREAD, 0.7);
        e.mem.set_f32(w + SIGHT_FOV, 65.0);
        e.mem.set_f32(w + MIN_RANGE, 256.0);
        e.mem.set_f32(w + MAX_RANGE, 768.0);
        e.mem.set_u8(w + AMMO_USE, 1);
        e.mem.set_f32(w + AP, 17.0);
        e.mem.set_u16(w + CRITICAL_DAMAGE, 16);
        e.mem.set_f32(w + CRITICAL_PERCENT, 1.0);
        e.mem.set_f32(w + FIRE_RATE, 1.0);
        e.mem.set_f32(w + ANIM_ATTACK_MULT, 1.25);
        e.mem.set_f32(w + RUMBLE_LEFT, 0.5);
        e.mem.set_f32(w + RUMBLE_DURATION, 0.15);
        e.mem.set_f32(w + RUMBLE_WAVELENGTH, 0.15);
        e.mem.set_f32(w + ANIM_SHOTS_PER_SEC, 3.124_999_5);
        e.mem.set_f32(w + ANIM_RELOAD_TIME, 1.6667);
        e.mem.set_f32(w + ANIM_JAM_TIME, 1.4);
        e.mem.set_u32(w + SKILL, 41);
        e.mem.set_u32(w + RESIST_TYPE, 0xffff_ffff);
        e.mem.set_f32(w + FIRE_DELAY_MAX, 0.3);
        e.mem.set_f32(w + ANIM_MULT, 1.0);
        e.mem.set_u8(w + HAND_GRIP, 0xff);
        e.mem.set_u8(w + RELOAD_ANIM, 11);
        e.mem.set_u8(w + BASE_VATS_CHANCE, 15);
        e.mem.set_u8(w + ATTACK_ANIM, 32);
        e.mem.set_u8(w + NUM_PROJECTILES, 1);
        e.mem.set_f32(w + LIMB_DAMAGE_MULT, 1.0);
        e.mem.set_f32(w + SIGHT_USAGE, 1.0);
        e.mem.set_u32(w + STR_REQUIRED, 2);
        e.mem.set_u8(w + FLAGS1, 4);
        e.mem.set_u32(w + FLAGS2, 8200);
        e.mem.set_u32(w + PROJECTILE, 0x0200_9000);
        e.mem.set_u32(0x0200_9000 + 0xc, 0x0008_f20f);
        e.mem.set_u32(REFERENCE + 0x7f0, 1);
        e.mem.set_u8(REFERENCE + 4, 0x3a);
        e.mem.set_u32(REFERENCE + 0x20, w);
    }

    fn real_scaled(e: &mut Engine, opcode: u32) -> f64 {
        let (ok, v) = call(e, opcode, 0, &[r(1)]);
        assert_eq!(ok, 1);
        v * 1_099_511_627_776.0
    }

    /// Every number getter on the 9mm pistol gives the real game's value; the evals with the
    /// form or the subject agree.
    #[test]
    fn getters_read_the_real_nine_mm_values() {
        let mut e = engine_with_table();
        nine_mm(&mut e);
        set_script_ref(&mut e, 1, NINE_MM);
        let real: &[(u32, f64)] = &[
            (0x1410, 3298534883328.0),
            (0x1411, 769658126336.0),
            (0x1412, 0.0),
            (0x1414, 71468255805440.0),
            (0x1415, 281474976710656.0),
            (0x1416, 844424930131968.0),
            (0x1417, 1099511627776.0),
            (0x1418, 18691697672192.0),
            (0x1419, 17592186044416.0),
            (0x141a, 1099511627776.0),
            (0x141c, 1099511627776.0),
            (0x141d, 1374389534720.0),
            (0x141e, 549755813888.0),
            (0x141f, 0.0),
            (0x1420, 164926750720.0),
            (0x1421, 164926750720.0),
            (0x1422, 3435973312512.0),
            (0x1423, 1832556036096.0),
            (0x1424, 1539316252672.0),
            (0x1425, 45079976738816.0),
            (0x1426, 4722366481770133585920.0),
            (0x1427, 0.0),
            (0x1428, 329853501440.0),
            (0x1429, 1099511627776.0),
            (0x142a, 0.0),
            (0x142b, 0.0),
            (0x142c, 0.0),
            (0x142d, 12094627905536.0),
            (0x142e, 16492674416640.0),
            (0x142f, 9895604649984.0),
            (0x1430, 1099511627776.0),
            (0x1431, 0.0),
            (0x1432, 1099511627776.0),
            (0x1433, 1099511627776.0),
            (0x1434, 1099511627776.0),
            (0x14bc, 2199023255552.0),
            (0x14bd, 0.0),
            (0x14e6, 0.0),
            (0x14e8, 4398046511104.0),
            (0x14e9, 9015995347763200.0),
            (0x1633, 0.0),
        ];
        let mut wrong = Vec::new();
        for (opcode, want) in real {
            let got = real_scaled(&mut e, *opcode);
            let by_eval = eval(&mut e, *opcode, 0, NINE_MM, 0) * 1_099_511_627_776.0;
            let by_subject = eval(&mut e, *opcode, REFERENCE, 0, 0) * 1_099_511_627_776.0;
            if got != *want || by_eval != *want || by_subject != *want {
                wrong.push(format!(
                    "{opcode:04x}: {got} {by_eval} {by_subject}, real {want}"
                ));
            }
        }
        assert!(wrong.is_empty(), "{wrong:#?}");
        // The projectile as a form id (`0008F20F`); the calling reference stands for its base.
        let (_, v) = call(&mut e, 0x1413, REFERENCE, &[]);
        assert_eq!(v.to_bits() as u32, 0x0008_f20f);
    }

    /// Setters write their fields, refuse values out of range, and keep the reload loop flag.
    #[test]
    fn setters_write_and_check() {
        let mut e = engine_with_table();
        nine_mm(&mut e);
        set_script_ref(&mut e, 1, NINE_MM);
        call(&mut e, 0x1483, 0, &[z(1.5), r(1)]);
        assert_eq!(e.mem.f32(NINE_MM + MIN_SPREAD), 1.5);
        call(&mut e, 0x1482, 0, &[n(14), r(1)]);
        assert_eq!(e.mem.u8(NINE_MM + WEAPON_TYPE), 3, "type 14 is refused");
        call(&mut e, 0x1482, REFERENCE, &[n(5)]);
        assert_eq!(
            e.mem.u8(NINE_MM + WEAPON_TYPE),
            5,
            "the calling reference's base"
        );
        call(&mut e, 0x1493, 0, &[n(20), r(1)]);
        assert_eq!(
            (
                e.mem.u8(NINE_MM + RELOAD_ANIM),
                e.mem.u8(NINE_MM + IS_LOOPING_RELOAD)
            ),
            (20, 1)
        );
        call(&mut e, 0x1492, 0, &[n(2), r(1)]);
        assert_eq!(e.mem.u8(NINE_MM + HAND_GRIP), 0xe7);
        assert_eq!(real_scaled(&mut e, 0x142c), 2199023255552.0);
        call(&mut e, 0x1495, 0, &[n(12), r(1)]);
        assert_eq!(e.mem.u8(NINE_MM + ATTACK_ANIM), 114);
        call(&mut e, 0x1491, 0, &[n(1), r(1)]);
        assert_eq!(e.mem.u8(NINE_MM + FLAGS1), 4 | 2);
        call(&mut e, 0x14c1, 0, &[n(31), r(1)]);
        assert_eq!(e.mem.u32(NINE_MM + SKILL), 41, "not a skill");
        call(&mut e, 0x14c1, 0, &[n(45), r(1)]);
        assert_eq!(e.mem.u32(NINE_MM + SKILL), 45);
        call(&mut e, 0x14e7, 0, &[n(1), r(1)]);
        assert_eq!(e.mem.u32(NINE_MM + FLAGS2), 8200 | 0x800);
        call(&mut e, 0x1494, 0, &[n(101), r(1)]);
        assert_eq!(e.mem.u8(NINE_MM + BASE_VATS_CHANCE), 15);
    }

    #[test]
    fn hand_grip_and_attack_animation_tables() {
        assert_eq!(hand_grip(0xff), 0);
        assert_eq!(hand_grip(0xeb), 6);
        assert_eq!(hand_grip(0x10), 0);
        assert_eq!(attack_animation(255), 0);
        assert_eq!(attack_animation(144), 19);
        assert_eq!(attack_animation(1), 255);
    }
}
