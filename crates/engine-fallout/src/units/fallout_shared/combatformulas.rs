//! `fallout shared/combatformulas.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The damage, spread, hit-chance, knockdown and combat-style formulas. Notes
//! for the next session:
//!
//! - Every function is `cdecl` (arguments on the stack, `this`-less) unless
//!   it takes `this`: the thiscall ones are the plain field getters and
//!   flag tests (`fn_00644640`, `fn_006450c0`, ...), which take `ECX`.
//! - The float code is x87: intermediate results are computed in `f64`
//!   (the game's x87 control word rounds to 53 bits) and rounded to `f32`
//!   wherever the code stores a `float` (`FSTP dword`). A `FLD`/`FSTP` copy of
//!   a `float` turns a signalling NaN into a quiet one ([`x87_quiet_bits`]).
//!   Comparisons use the status word the exe tests ([`x87_compare`] and
//!   [`parity_even`], which `JP`/`JNP` read), so NaN behaves as in the game.
//! - Game settings are read through `00403e20` (a pointer to the setting's
//!   value), named after the strings their static initializers register
//!   (`SETTING_*`; the integer ones go through `0043d4d0`).
//! - `TESCombatStyle` has no layout in this crate; the combat-style getters
//!   (`COMBAT_STYLE_*`) read its fields at the PC offsets. The names are the
//!   Xbox PDB field names of `TESCombatStyle` (PC offset = Xbox offset - 0x10);
//!   every offset read here matches a PDB field of the right type, but no RTTI
//!   instance was inspected.
//! - Functions `00646910`, `00646f70`, `00644ce0` and `00646060` call
//!   `00645380` (the damage-per-second figure, which belongs to no unit in
//!   the engine map and is not translated yet) by address with 13 words.

#[allow(unused_imports)]
use crate::prelude::*;

// ---- addresses outside this unit ---------------------------------------------------------------

/// `Setting` value getter: `ECX` = the setting object, returns the address of
/// its value (a `float` for the `f...` settings).
const GET_SETTING_VALUE: u32 = 0x0040_3e20;
/// The same for the integer game settings (`i...`).
const GET_INTEGER_SETTING_VALUE: u32 = 0x0043_d4d0;
/// `rand()` wrapper (`00487f50`, no arguments).
const RANDOM: u32 = 0x0048_7f50;
/// `_ftol2_sse` (`00ec62c0`): truncates the `ST0` value; passed as an `f64`.
const FTOL: u32 = 0x00ec_62c0;
/// `__RTDynamicCast(object, 0, fromType, toType, 0)`.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors (`.?AV...@@`) read from the exe: the cast sources and targets.
const TYPE_ACTOR_VALUE_OWNER: u32 = 0x0119_9c1c;
const TYPE_ACTOR: u32 = 0x0118_46d4;
const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;
/// `min(a, b)` / `max(a, b)` on floats (`cdecl`, result in `ST0`).
const FLOAT_MIN: u32 = 0x0040_ebd0;
const FLOAT_MAX: u32 = 0x0040_4010;
/// `RandomFloat(low, high)` (Xbox PDB).
const RANDOM_FLOAT: u32 = 0x0047_6b70;
/// `00476b20(float)`: rounds the float (used by `00647870`).
const ROUND_FLOAT: u32 = 0x0047_6b20;
/// `00408840(float)`: a one-argument float function whose result is the
/// death-force interpolation parameter (`fn_006466e0`).
const FLOAT_FUNCTION_00408840: u32 = 0x0040_8840;
/// `00647ba0(&int, low, high)` and `00647bd0(&uint, low, high)`: clamp the
/// value the pointer addresses into `low ..= high` (signed / unsigned).
const CLAMP_INT: u32 = 0x0064_7ba0;
const CLAMP_UINT: u32 = 0x0064_7bd0;
/// `BGSEntryPoint::HandleEntryPoint` (`cdecl`): entry point id, the perk
/// owner, further form arguments some entry points take, and the address of
/// the `float` it updates.
const HANDLE_ENTRY_POINT: u32 = 0x005e_58f0;
/// `00645380` (damage-per-second figure, 13 words; not in any unit).
const WEAPON_DAMAGE_FIGURE: u32 = 0x0064_5380;
/// The "apply perk entry" float helper of `fn_00646260` and
/// `combat_formulas_calc_weapon_damage_for_display` (`0059a030(entry, form,
/// float)`), `cdecl`.
const APPLY_PERK_ENTRY_FLOAT: u32 = 0x0059_a030;

/// `00401170` (thiscall): the byte at `+4` of a form (its form type).
const FORM_TYPE_BYTE: u32 = 0x0040_1170;
/// The player reference pointer global.
const PLAYER_REFERENCE: u32 = 0x011d_ea3c;
/// Offset of the `ActorValueOwner` sub-object inside an `Actor`.
const ACTOR_VALUE_OWNER: u32 = 0xa4;

// Weapon / item methods outside this unit (thiscall on the weapon form unless noted).
/// `TESObjectWEAP::GetCurrentAmmo(holder)` (Xbox PDB).
const WEAPON_GET_CURRENT_AMMO: u32 = 0x0052_5980;
/// `00525a90(holder)`: the weapon's ammunition-related object (calls `GetCurrentAmmo`).
const WEAPON_GET_AMMO_OBJECT: u32 = 0x0052_5a90;
/// `TESObjectWEAP::GetNumProjectiles(bool, 0, actor)` (Xbox PDB), returns a byte.
const WEAPON_GET_NUM_PROJECTILES: u32 = 0x0052_5b20;
/// Weapon flag test `byte [+0x100] & 2` (set by `fn_00645320`).
const WEAPON_FLAG_TEST: u32 = 0x0052_4b40;
/// `00524b80(byte)`: a float spread value of the weapon.
const WEAPON_GET_SPREAD: u32 = 0x0052_4b80;
/// Weapon `eType` as a signed byte (`+0xf4`).
const WEAPON_GET_TYPE: u32 = 0x0044_6390;
/// `eType` in `3 ..= 13`, `3 ..= 9` and "13-class" tests.
const WEAPON_IS_GUN_TYPE: u32 = 0x004c_0c30;
const WEAPON_IS_GUN_TYPE_3_TO_9: u32 = 0x004c_0c60;
const WEAPON_IS_THROWN_TYPE: u32 = 0x004c_0bf0;
/// The weapon's skill actor value (word at `+0x15c`).
const WEAPON_GET_SKILL: u32 = 0x008d_85e0;
/// The weapon's field at `+0x124` (also `fn_00646f50`).
/// `00663b60`: the word at `+0x19c`; `008d8160`: weapon field used by `00647870`.
const WEAPON_GET_FIELD_19C: u32 = 0x0066_3b60;
const WEAPON_GET_FIELD_8D8160: u32 = 0x008d_8160;
/// Weapon float getters: `008d1eb0` (`00821640` is `FUNCTION_00821640` below).
const FUNCTION_008D1EB0: u32 = 0x008d_1eb0;
/// `TESObjectWEAP::GetModEffectValue(effect, 0)` (Xbox PDB), returns a float.
const WEAPON_GET_MOD_EFFECT_VALUE: u32 = 0x004b_cf60;
/// `ItemChange::HasModEffectActive(effect, &float)` and `_ov2(effect)` (Xbox PDB).
const ITEM_CHANGE_HAS_MOD_EFFECT_ACTIVE: u32 = 0x004b_d8d0;
const ITEM_CHANGE_HAS_MOD_EFFECT_ACTIVE_OV2: u32 = 0x004b_da70;
/// Weapon type-table readers of `00645dc0`.
const WEAPON_GET_ATTACK_ANIM_GROUP: u32 = 0x0051_e2a0;
const WEAPON_GET_ATTACK_ANIM_GROUP_2: u32 = 0x0051_f5f0;
const ANIM_GROUP_LOOKUP: u32 = 0x005f_2370;
const ANIM_GROUP_ID: u32 = 0x005f_2440;
const ANIM_GROUP_ACTION_TIME: u32 = 0x005f_3780;
/// Player animations: `00950a60(0)` on the player, then `00495740` / `00496500`.
const PLAYER_GET_ANIMATIONS: u32 = 0x0095_0a60;
const ANIMATIONS_FIND_GROUP: u32 = 0x0049_5740;
const ANIMATIONS_GET_SEQUENCE: u32 = 0x0049_6500;
/// Clip size of the weapon (`004fe160(byte)`).
const WEAPON_GET_CLIP_ROUNDS: u32 = 0x004f_e160;
/// The weapon's attack-speed getter `+0x124`'s sibling and other helpers.
const WEAPON_GET_PROJECTILE_FLOAT: u32 = 0x0045_cd80;
const AMMO_GET_VALUE_FROM_OBJECT: u32 = 0x004f_d3c0;
const FUNCTION_006479E0: u32 = 0x0064_79e0;
const FUNCTION_00923000: u32 = 0x0092_3000;
const ACTOR_VALUE_GET_BY_INDEX: u32 = 0x0066_ef50;
const ACTOR_VALUE_GET_INT_BY_INDEX: u32 = 0x0066_ef20;
const ACTOR_GET_FLAG_566950: u32 = 0x0056_6950;
const FUNCTION_00525430: u32 = 0x0052_5430;
const FUNCTION_0084D030: u32 = 0x0084_d030;
const FUNCTION_00522C80: u32 = 0x0052_2c80;
const FUNCTION_006B05D0: u32 = 0x006b_05d0;
const FUNCTION_008A7F80: u32 = 0x008a_7f80;
const FUNCTION_008A16B0: u32 = 0x008a_16b0;
const FUNCTION_008D8520: u32 = 0x008d_8520;
const FUNCTION_0044DDC0: u32 = 0x0044_ddc0;
const FUNCTION_00452370: u32 = 0x0045_2370;
const FUNCTION_005DC980: u32 = 0x005d_c980;
const FUNCTION_0057DD70: u32 = 0x0057_dd70;
const FUNCTION_004E4620: u32 = 0x004e_4620;
const FUNCTION_00821640: u32 = 0x0082_1640;

// TESCombatStyle getters outside this unit (folded bodies; each reads one field).
/// `cPowerAttackChance` (`byte [+0x8c]`).
const COMBAT_STYLE_POWER_ATTACK_CHANCE: u32 = 0x0055_24b0;
/// `fPowerAttackRecoilStaggerBonus` (`+0x90`).
const COMBAT_STYLE_POWER_ATTACK_RECOIL_STAGGER_BONUS: u32 = 0x006c_a4e0;
/// `fBlockSkillMult` (`+0xdc`), `fBlockDuringAttackMult` (`+0xe4`),
/// `fBlockNoAttackMult` (`+0xe8`).
const COMBAT_STYLE_BLOCK_SKILL_MULT: u32 = 0x0088_5d70;
const COMBAT_STYLE_BLOCK_DURING_ATTACK_MULT: u32 = 0x009a_5480;
const COMBAT_STYLE_BLOCK_NO_ATTACK_MULT: u32 = 0x004e_45e0;
/// `fAttackHandBonus` (`+0x88`), `fAttackSkillBase` (`+0xf0`),
/// `fAttackSkillMult` (`+0xec`), `fAttackNoAttackMult` (`+0xf8`),
/// `cAttackChance` (`byte [+0x7d]`).
const COMBAT_STYLE_ATTACK_HAND_BONUS: u32 = 0x0051_b8e0;
const COMBAT_STYLE_ATTACK_SKILL_BASE: u32 = 0x004e_4600;
const COMBAT_STYLE_ATTACK_SKILL_MULT: u32 = 0x0096_6a20;
const COMBAT_STYLE_ATTACK_NO_ATTACK_MULT: u32 = 0x004e_4620;
const COMBAT_STYLE_ATTACK_CHANCE: u32 = 0x0079_8420;
/// `fDodgeSpeedMult` (`+0xc0`), `fDodgeSpeedBase` (`+0xbc`),
/// `fDodgeFatigueMult` (`+0xb4`), `fDodgeFatigueBase` (`+0xb8`),
/// `fDodgeNoAttackMult` (`+0xc8`).
const COMBAT_STYLE_DODGE_SPEED_MULT: u32 = 0x006d_6d90;
const COMBAT_STYLE_DODGE_SPEED_BASE: u32 = 0x0099_e040;
const COMBAT_STYLE_DODGE_FATIGUE_MULT: u32 = 0x006d_2c40;
const COMBAT_STYLE_DODGE_FATIGUE_BASE: u32 = 0x0048_ee50;
const COMBAT_STYLE_DODGE_NO_ATTACK_MULT: u32 = 0x009a_1260;
/// `fDodgeBackDuringAttackMult` (`+0xcc`), `fDodgeBackNoAttackMult` (`+0xd0`),
/// `fDodgeForwardWhileAttackingMult` (`+0xd4`),
/// `fDodgeForwardNotAttackingMult` (`+0xd8`).
const COMBAT_STYLE_DODGE_BACK_DURING_ATTACK_MULT: u32 = 0x0045_36e0;
const COMBAT_STYLE_DODGE_BACK_NO_ATTACK_MULT: u32 = 0x0045_3700;
const COMBAT_STYLE_DODGE_FORWARD_WHILE_ATTACKING_MULT: u32 = 0x0097_9260;
const COMBAT_STYLE_DODGE_FORWARD_NOT_ATTACKING_MULT: u32 = 0x0088_5bd0;

// ---- exe constants (data read at run time, as the game does) -----------------------------------

/// `0.0` as a `double`.
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// `1.0` as a `double`.
const ONE_DOUBLE: u32 = 0x0101_2070;
/// `0.5`, `2.0`, `3.0`, `10.0`, `100.0`, `1000.0`, `10000.0`, `1.5`, `0.75`, `48.0` as `double`s.
const HALF_DOUBLE: u32 = 0x0101_1588;
const TWO_DOUBLE: u32 = 0x0101_1590;
const THREE_DOUBLE: u32 = 0x0102_1928;
const TEN_DOUBLE: u32 = 0x0102_0758;
const HUNDRED_DOUBLE: u32 = 0x0101_7a40;
const THOUSAND_DOUBLE: u32 = 0x0101_7b70;
const TEN_THOUSAND_DOUBLE: u32 = 0x0102_f078;
const ONE_AND_A_HALF_DOUBLE: u32 = 0x0101_6ff0;
const THREE_QUARTERS_DOUBLE: u32 = 0x0101_de30;
const FORTY_EIGHT_DOUBLE: u32 = 0x0103_57a0;
/// `0.01f`, `0.05f`, `0.68f`, pi, pi/180 (as `float`s widened to `double`) and `1.3` as `double`s.
const ONE_HUNDREDTH_DOUBLE: u32 = 0x0101_6408;
const FIVE_HUNDREDTHS_DOUBLE: u32 = 0x0103_c7c8;
const CONDITION_CURVE_SCALE_DOUBLE: u32 = 0x0105_1680;
const PI_DOUBLE: u32 = 0x0101_ff40;
const DEGREES_TO_RADIANS_DOUBLE: u32 = 0x0102_3128;
const SPLIT_BEAM_MULT_DOUBLE: u32 = 0x0105_1678;
/// `100.0f` and `0.25f` as `float`s.
const HUNDRED_FLOAT: u32 = 0x0101_6410;
const QUARTER_FLOAT: u32 = 0x0101_622c;
/// The empty string used for the log arguments.
const EMPTY_TEXT: u32 = 0x0101_1584;

// ---- game settings (names from the exe's static initializers) ------------------------------------

const SETTING_KNOCKDOWN_AGIL_MULT: u32 = 0x011c_e064;
const SETTING_VATS_MAX_CHANCE: u32 = 0x011c_e10c;
const SETTING_INTEGER_COMBAT_EXPLODE_PART_CHANCE: u32 = 0x011c_e2fc;
const SETTING_DAMAGE_WEAPON_MULT: u32 = 0x011c_e2c0;
const SETTING_AI_MELEE_WEAPON_MULT: u32 = 0x011c_e448;
const SETTING_WEAP_STRENGTH_REQ_PENALTY: u32 = 0x011c_e534;
const SETTING_CRIPPLED_ARMS_1H_SPREAD_PENALTY: u32 = 0x011c_e60c;
const SETTING_KNOCKBACK_AGIL_BASE: u32 = 0x011c_e660;
const SETTING_DAMAGE_POWER_ATTACK_BONUS: u32 = 0x011c_e678;
const SETTING_VATS_DESTRUCTIBLE_MULT: u32 = 0x011c_e6fc;
const SETTING_KNOCKDOWN_AGIL_BASE: u32 = 0x011c_e78c;
const SETTING_KNOCKDOWN_CHANCE: u32 = 0x011c_e808;
const SETTING_RUNNING_SPREAD_PENALTY: u32 = 0x011c_e830;
const SETTING_BLOCK_SKILL_BASE: u32 = 0x011c_e840;
const SETTING_VATS_SKILL_FACTOR: u32 = 0x011c_e99c;
const SETTING_KNOCKBACK_AGIL_MULT: u32 = 0x011c_ea68;
const SETTING_VATS_GRENADE_TARGET_AREA: u32 = 0x011c_eb20;
const SETTING_MIN_GUN_SPREAD_VALUE: u32 = 0x011c_eb8c;
const SETTING_KNOCKDOWN_CURRENT_HEALTH_THRESHOLD: u32 = 0x011c_ebc8;
const SETTING_AI_MELEE_HAND_MULT: u32 = 0x011c_ec70;
const SETTING_ARROW_GRAVITY_MULT: u32 = 0x011c_ec88;
const SETTING_KNOCKDOWN_DAMAGE_MULT: u32 = 0x011c_ecd0;
const SETTING_DAMAGE_SKILL_BASE: u32 = 0x011c_ece8;
const SETTING_VATS_GRENADE_RANGE_MULT: u32 = 0x011c_ed00;
const SETTING_UNAIMED_SPREAD_PENALTY: u32 = 0x011c_edd0;
const SETTING_VATS_STEALTH_MULT: u32 = 0x011c_ee54;
const SETTING_KNOCKDOWN_DAMAGE_BASE: u32 = 0x011c_eecc;
const SETTING_CRIPPLED_ARMS_2H_SPREAD_PENALTY: u32 = 0x011c_ef38;
const SETTING_VATS_THROWN_WEAPON_RANGE_MULT: u32 = 0x011c_ef80;
const SETTING_CRIPPLED_ARM_2H_SPREAD_PENALTY: u32 = 0x011c_f008;
const SETTING_ARROW_GRAVITY_BASE: u32 = 0x011c_f044;
const SETTING_CRIPPLED_ARM_1H_SPREAD_PENALTY: u32 = 0x011c_f068;
const SETTING_DAMAGE_TO_ARMOR_PERCENTAGE: u32 = 0x011c_f0fc;
const SETTING_DAMAGE_TO_WEAPON_VALUE: u32 = 0x011c_f150;
const SETTING_KNOCKDOWN_BASE_HEALTH_THRESHOLD: u32 = 0x011c_f1a4;
const SETTING_BLOCK_SCORE_NO_SHIELD_MULT: u32 = 0x011c_f258;
const SETTING_AI_DODGE_FAVOR_LEFT_RIGHT_MULT: u32 = 0x011c_f288;
const SETTING_INTEGER_COMBAT_DISMEMBER_PART_CHANCE: u32 = 0x011c_f2ec;
const SETTING_DAMAGE_SKILL_MULT: u32 = 0x011c_f350;
const SETTING_VATS_HIT_CHANCE_MULT: u32 = 0x011c_f504;
const SETTING_VATS_GRENADE_SKILL_FACTOR: u32 = 0x011c_f528;
const SETTING_DEATH_FORCE_DAMAGE_MIN: u32 = 0x011c_f618;
const SETTING_DEATH_FORCE_RANGED_DAMAGE_MAX: u32 = 0x011c_f624;
const SETTING_STANDING_SPREAD_PENALTY: u32 = 0x011c_f648;
const SETTING_WALKING_SPREAD_PENALTY: u32 = 0x011c_f654;
const SETTING_VATS_MELEE_MAX_DISTANCE: u32 = 0x011c_f678;
const SETTING_DEATH_FORCE_FORCE_MIN: u32 = 0x011c_f724;
const SETTING_WOBBLE_TO_SKILL_CONVERSION: u32 = 0x011c_f7f8;
const SETTING_COMBAT_DISTANCE: u32 = 0x011c_f84c;
const SETTING_VATS_GRENADE_CHANCE_MULT: u32 = 0x011c_f8bc;
const SETTING_AI_DODGE_DECISION_BASE: u32 = 0x011c_f8f8;
const SETTING_DEATH_FORCE_RANGED_FORCE_MAX: u32 = 0x011c_f950;
const SETTING_KNOCKBACK_DAMAGE_BASE: u32 = 0x011c_f9bc;
const SETTING_KNOCKBACK_DAMAGE_MULT: u32 = 0x011c_fa1c;
const SETTING_VATS_SCREEN_PERCENT_FACTOR: u32 = 0x011c_fab0;
const SETTING_DAMAGE_ARM_CONDITION_MULT: u32 = 0x011c_faec;
const SETTING_DEATH_FORCE_FORCE_MAX: u32 = 0x011c_fb9c;
const SETTING_VATS_GRENADE_RANGE_MIN: u32 = 0x011c_fba8;
const SETTING_DAMAGE_ARM_CONDITION_BASE: u32 = 0x011c_fcb0;
const SETTING_DEATH_FORCE_RANGED_FORCE_MIN: u32 = 0x011c_fd04;
const SETTING_DEATH_FORCE_RANGED_DAMAGE_MIN: u32 = 0x011c_fd58;
const SETTING_BLOCK_SKILL_MULT: u32 = 0x011c_fd8c;
const SETTING_VATS_SPREAD_MULT: u32 = 0x011c_fe08;
const SETTING_ARROW_WEAK_GRAVITY: u32 = 0x011c_fdf0;
const SETTING_VATS_RANGE_SPREAD_MAX: u32 = 0x011c_fe50;
const SETTING_DEATH_FORCE_DAMAGE_MAX: u32 = 0x011c_fe74;

/// Tables of the ten `fWeaponConditionJam1` .. `10` / `fWeaponConditionReloadJam1` ..
/// `10` setting objects, indexed by condition decile.
const JAM_SETTING_TABLE: u32 = 0x0119_b26c;
const RELOAD_JAM_SETTING_TABLE: u32 = 0x0119_b2bc;
/// Text of the VATS debug log messages (`00646f70`).
const LOG_FORMAT_TWO_TEXTS: u32 = 0x0101_996c;
const LOG_GRENADE_TARGET: u32 = 0x0105_1878;
const LOG_DESTRUCTIBLE_TARGET: u32 = 0x0105_1860;
const LOG_OUT_OF_EXPLOSION_RANGE: u32 = 0x0105_1840;
const LOG_OUT_OF_THROWN_RANGE: u32 = 0x0105_181c;
const LOG_MOD_MAX_RANGE: u32 = 0x0105_17e4;
const LOG_ARM_CONDITION: u32 = 0x0105_17ac;
const LOG_BOUND_SIZE: u32 = 0x0105_1788;
const LOG_SPREAD_FACTOR: u32 = 0x0105_175c;
const LOG_TARGET_AREA: u32 = 0x0105_1720;
const LOG_DESTRUCTIBLE_GRENADE_CHANCE: u32 = 0x0105_16f4;
const LOG_MELEE_ASSUME_100: u32 = 0x0105_16d0;
const LOG_MELEE_OUT_OF_RANGE: u32 = 0x0105_16a8;
const LOG_STEALTH_BONUS: u32 = 0x0105_1688;
/// Offset of the weapon-type table `00645dc0` indexes with the weapon `eType`.
const WEAPON_TYPE_ANIMATION_TABLE: u32 = 0x0118_a838;

// ---- field offsets ---------------------------------------------------------------------------------

/// `TESCombatStyle` (Xbox PDB), PC offsets: `CombatStyleData Data` at +0x58,
/// `AdvancedCombatStyleData AdvancedData` at +0xb4.
const COMBAT_STYLE_DODGE_CHANCE: u32 = 0x58;
const COMBAT_STYLE_DODGE_LEFT_RIGHT_CHANCE: u32 = 0x59;
const COMBAT_STYLE_BLOCK_CHANCE: u32 = 0x7c;
const COMBAT_STYLE_ATTACK_DURING_RECOIL_STAGGER_BONUS: u32 = 0x80;
const COMBAT_STYLE_ATTACK_DURING_UNCONSCIOUS_BONUS: u32 = 0x84;
const COMBAT_STYLE_POWER_ATTACK_UNCONSCIOUS_BONUS: u32 = 0x94;
const COMBAT_STYLE_DODGE_DURING_ATTACK_MULT: u32 = 0xc4;
const COMBAT_STYLE_BLOCK_SKILL_BASE: u32 = 0xe0;
const COMBAT_STYLE_ATTACK_DURING_ATTACK_MULT: u32 = 0xf4;
const COMBAT_STYLE_ATTACK_DURING_BLOCK_MULT: u32 = 0xfc;
const COMBAT_STYLE_POWER_ATTACK_FATIGUE_BASE: u32 = 0x100;
const COMBAT_STYLE_POWER_ATTACK_FATIGUE_MULT: u32 = 0x104;
/// `TESObjectWEAP`: `data` (`OBJ_WEAP`) is at +0xf4 (`eType`), `cFlags` at +0x100.
const WEAPON_TYPE: u32 = 0xf4;
const WEAPON_FLAGS: u32 = 0x100;
const WEAPON_FIELD_124: u32 = 0x124;
const WEAPON_FIELD_12C: u32 = 0x12c;
const WEAPON_FIELD_130: u32 = 0x130;
const WEAPON_FIELD_11C: u32 = 0x11c;

// ---- x87 helpers -----------------------------------------------------------------------------------

/// The high byte of the x87 status word (`FNSTSW AX`, then `AH`) after `FCOM st0, operand`: C0
/// (`0x01`) when `st0 < operand`, C3 (`0x40`) when equal, nothing when `st0 > operand`, and
/// C0 | C2 | C3 (`0x45`) when either is a NaN.
fn x87_compare(st0: f64, operand: f64) -> u8 {
    if st0.is_nan() || operand.is_nan() {
        0x45
    } else if st0 > operand {
        0x00
    } else if st0 < operand {
        0x01
    } else {
        0x40
    }
}

/// The parity flag `TEST AH, mask` leaves (set when the masked byte has an even number of 1 bits),
/// which `JP` / `JNP` test.
fn parity_even(value: u8) -> bool {
    value.count_ones() % 2 == 0
}

/// `FISTP qword` with the truncating rounding mode, low dword: the integer indefinite
/// (`0x8000000000000000`, low dword 0) for a NaN or a value outside the 64-bit range.
fn x87_truncate_to_u32_via_i64(value: f64) -> u32 {
    let truncated = value.trunc();
    if truncated.is_nan()
        || !(-9_223_372_036_854_775_808.0..9_223_372_036_854_775_808.0).contains(&truncated)
    {
        0
    } else {
        truncated as i64 as u32
    }
}

/// `FISTP dword` with the truncating rounding mode: the integer indefinite (`0x80000000`) for a
/// NaN or a value outside the 32-bit range.
fn x87_truncate_to_i32(value: f64) -> i32 {
    let truncated = value.trunc();
    if truncated.is_nan() || !(-2_147_483_648.0..2_147_483_648.0).contains(&truncated) {
        i32::MIN
    } else {
        truncated as i32
    }
}

// ---- small helpers ---------------------------------------------------------------------------------

/// The address a setting getter returns (`ECX` = the setting).
fn setting_pointer(e: &mut Engine, setting: u32) -> u32 {
    e.call(GET_SETTING_VALUE, &args![setting]).u32()
}

/// A float setting's value (`FLD dword [EAX]`).
fn setting(e: &mut Engine, setting: u32) -> f32 {
    let pointer = setting_pointer(e, setting);
    e.mem.f32(pointer)
}

/// An integer game setting's value (`0043d4d0` returns its address).
fn integer_setting(e: &mut Engine, setting: u32) -> i32 {
    let pointer = e.call(GET_INTEGER_SETTING_VALUE, &args![setting]).u32();
    e.mem.i32(pointer)
}

/// `rand()` through the game's wrapper.
fn random(e: &mut Engine) -> u32 {
    e.call(RANDOM, &[]).u32()
}

/// `_ftol2_sse` on a value in `ST0`.
fn float_to_int(e: &mut Engine, value: f64) -> i32 {
    e.call(FTOL, &args![value]).i32()
}

/// `holder ? holder + 0xa4 : 0`: the `ActorValueOwner` sub-object of an `Actor`.
fn actor_value_owner(actor: Ptr) -> Ptr {
    if actor.is_null() {
        Ptr::NULL
    } else {
        Ptr::new(actor.addr().wrapping_add(ACTOR_VALUE_OWNER))
    }
}

/// `__RTDynamicCast(object, 0, from, to, 0)`.
fn dynamic_cast(e: &mut Engine, object: Ptr, from: u32, to: u32) -> Ptr {
    e.call(DYNAMIC_CAST, &args![object, 0u32, from, to, 0u32])
        .ptr()
}

/// The player reference (`[011dea3c]`).
fn player(e: &Engine) -> Ptr {
    e.global(PLAYER_REFERENCE)
}

/// The weapon's `eType` byte, signed (`00446390`).
fn weapon_type(e: &mut Engine, weapon: Ptr) -> i32 {
    e.call(WEAPON_GET_TYPE, &args![weapon]).i32()
}

// Translated from 00525ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatFormulas::GetArmCondition` (engine map name, Xbox PDB): converts the
/// `ActorValueOwner` pointer (0 stays 0, else minus `0xa4`) to the `Actor` and
/// returns what `00525a90(this, actor)` returns. That callee asks `TESObjectWEAP::GetCurrentAmmo`
/// (`00525980`) for the actor's ammunition and returns what `008d80e0` gives for it, or, when
/// there is none, the word at `+0x118` of the weapon; the map's name does not describe this
/// body.
pub fn combat_formulas_get_arm_condition(e: &mut Engine, this: Ptr, owner: Ptr) -> Ptr {
    let actor: Ptr = if owner.is_null() {
        Ptr::NULL
    } else {
        Ptr::new(owner.addr().wrapping_sub(ACTOR_VALUE_OWNER))
    };
    e.call(WEAPON_GET_AMMO_OBJECT, &args![this, actor]).ptr()
}

// Translated from 00644520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatFormulas::GetPowerAttackDamageMultiplier` (Xbox PDB): the value of the
/// `fDamagePowerAttackBonus` game setting. Both stack words are unused.
pub fn combat_formulas_get_power_attack_damage_multiplier(
    e: &mut Engine,
    _unused_0: u32,
    _unused_1: u32,
) -> f32 {
    setting(e, SETTING_DAMAGE_POWER_ATTACK_BONUS)
}

// Translated from 00644540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `value` times `fAIMeleeHandMult` when `hand` is set, else
/// `fAIMeleeWeaponMult`. The word at `+0xc` is unused.
pub fn fn_00644540(e: &mut Engine, value: f32, _unused_1: u32, hand: bool) -> f32 {
    let multiplier = if hand {
        setting(e, SETTING_AI_MELEE_HAND_MULT)
    } else {
        setting(e, SETTING_AI_MELEE_WEAPON_MULT)
    };
    (value as f64 * multiplier as f64) as f32
}

// Translated from 00644580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the combat style's block chance for `skill` (0 to 100):
/// `(cBlockChance + fBlockSkillBase + fBlockSkillMult * skill / 100)` times
/// `fBlockDuringAttackMult` when `attacking`, else `fBlockNoAttackMult`; unless
/// `flag_14` is set it is also scaled by `fBlockScoreNoShieldMult` and by 1.0 when
/// attacking, 0.25 otherwise. `style` is a `TESCombatStyle` (see the module notes).
/// The word at `+0x10` is unused.
pub fn fn_00644580(
    e: &mut Engine,
    style: Ptr,
    skill: i32,
    _unused_2: u32,
    flag_14: bool,
    attacking: bool,
) -> f32 {
    let hundred: f64 = e.global(HUNDRED_DOUBLE);
    let skill_fraction = (skill as f64 / hundred) as f32;
    let block_chance = fn_00644640(e, style);
    let block_chance_double = block_chance as f64;
    let skill_base = fn_00644660(e, style);
    let base_and_chance = skill_base as f64 + block_chance_double;
    let skill_mult = e.call(COMBAT_STYLE_BLOCK_SKILL_MULT, &args![style]).f64();
    let mut score = (skill_mult * skill_fraction as f64 + base_and_chance) as f32;
    let situation_mult = if attacking {
        e.call(COMBAT_STYLE_BLOCK_DURING_ATTACK_MULT, &args![style])
            .f32()
    } else {
        e.call(COMBAT_STYLE_BLOCK_NO_ATTACK_MULT, &args![style])
            .f32()
    };
    score = (score as f64 * situation_mult as f64) as f32;
    let result = if flag_14 {
        score as f64
    } else {
        let attack_factor = if attacking {
            1.0f32
        } else {
            e.global::<f32>(QUARTER_FLOAT)
        };
        let no_shield = setting(e, SETTING_BLOCK_SCORE_NO_SHIELD_MULT);
        score as f64 * no_shield as f64 * attack_factor as f64
    };
    result as f32
}

// Translated from 00644640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCombatStyle::Data.cBlockChance` (Xbox PDB field; `byte [+0x7c]`).
pub fn fn_00644640(e: &mut Engine, this: Ptr) -> i8 {
    e.mem.i8(this.addr() + COMBAT_STYLE_BLOCK_CHANCE)
}

// Translated from 00644660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCombatStyle::AdvancedData.fBlockSkillBase` (Xbox PDB field; `float [+0xe0]`).
pub fn fn_00644660(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + COMBAT_STYLE_BLOCK_SKILL_BASE)
}

// Translated from 00644680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the combat style's attack score for `skill`. Adds, each when
/// its flag is set: `fAttackDuringRecoilStaggerBonus` (`recoil_stagger`),
/// `fAttackDuringUnconsciousBonus` (`unconscious`) and `fAttackHandBonus` (`hand`);
/// plus `cAttackChance` and `fAttackSkillBase + fAttackSkillMult * skill / 100`; times
/// `fAttackDuringAttackMult` (`attacking`) or `fAttackNoAttackMult`, and, when
/// `blocking`, `fAttackDuringBlockMult`. The word at `+0x10` is unused.
#[allow(clippy::too_many_arguments)]
pub fn fn_00644680(
    e: &mut Engine,
    style: Ptr,
    skill: i32,
    _unused_2: u32,
    attacking: bool,
    hand: bool,
    recoil_stagger: bool,
    unconscious: bool,
    blocking: bool,
) -> f32 {
    let hundred: f64 = e.global(HUNDRED_DOUBLE);
    let skill_fraction = (skill as f64 / hundred) as f32;
    let recoil_stagger_bonus = if recoil_stagger {
        x87_quiet(fn_00644790(e, style))
    } else {
        0.0
    };
    let unconscious_bonus = if unconscious {
        x87_quiet(fn_006447b0(e, style))
    } else {
        0.0
    };
    let hand_bonus = if hand {
        x87_quiet(e.call(COMBAT_STYLE_ATTACK_HAND_BONUS, &args![style]).f32())
    } else {
        0.0
    };
    let skill_base = e.call(COMBAT_STYLE_ATTACK_SKILL_BASE, &args![style]).f64();
    let skill_mult = e.call(COMBAT_STYLE_ATTACK_SKILL_MULT, &args![style]).f64();
    let skill_term = (skill_mult * skill_fraction as f64 + skill_base) as f32;
    let attack_chance = e.call(COMBAT_STYLE_ATTACK_CHANCE, &args![style]).u8() as i8;
    let mut score = (((attack_chance as f64 + recoil_stagger_bonus as f64)
        + unconscious_bonus as f64)
        + hand_bonus as f64
        + skill_term as f64) as f32;
    let situation_mult = if attacking {
        fn_006447d0(e, style)
    } else {
        e.call(COMBAT_STYLE_ATTACK_NO_ATTACK_MULT, &args![style])
            .f32()
    };
    score = (score as f64 * x87_quiet(situation_mult) as f64) as f32;
    let blocking_mult = if blocking {
        x87_quiet(fn_006447f0(e, style))
    } else {
        1.0
    };
    (score as f64 * blocking_mult as f64) as f32
}

// Translated from 00644790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCombatStyle::Data.fAttackDuringRecoilStaggerBonus` (Xbox PDB field; `float [+0x80]`).
pub fn fn_00644790(e: &mut Engine, this: Ptr) -> f32 {
    e.mem
        .f32(this.addr() + COMBAT_STYLE_ATTACK_DURING_RECOIL_STAGGER_BONUS)
}

// Translated from 006447b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCombatStyle::Data.fAttackDuringUnconsciousBonus` (Xbox PDB field; `float [+0x84]`).
pub fn fn_006447b0(e: &mut Engine, this: Ptr) -> f32 {
    e.mem
        .f32(this.addr() + COMBAT_STYLE_ATTACK_DURING_UNCONSCIOUS_BONUS)
}

// Translated from 006447d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCombatStyle::AdvancedData.fAttackDuringAttackMult` (Xbox PDB field; `float [+0xf4]`).
pub fn fn_006447d0(e: &mut Engine, this: Ptr) -> f32 {
    e.mem
        .f32(this.addr() + COMBAT_STYLE_ATTACK_DURING_ATTACK_MULT)
}

// Translated from 006447f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCombatStyle::AdvancedData.fAttackDuringBlockMult` (Xbox PDB field; `float [+0xfc]`).
pub fn fn_006447f0(e: &mut Engine, this: Ptr) -> f32 {
    e.mem
        .f32(this.addr() + COMBAT_STYLE_ATTACK_DURING_BLOCK_MULT)
}

// Translated from 00644810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether the combat style makes a power attack. Never when
/// `suppress`; with `cPowerAttackChance` of 0 never, of 100 or more always; otherwise the
/// chance gets `fPowerAttackRecoilStaggerBonus` (`recoil_stagger`),
/// `fPowerAttackUnconsciousBonus` (`unconscious`) and
/// `fPowerAttackFatigueBase + fPowerAttackFatigueMult * (1 - fatigue)` added, and a
/// `rand() % 100` roll must not be above it.
pub fn fn_00644810(
    e: &mut Engine,
    style: Ptr,
    fatigue: f32,
    recoil_stagger: bool,
    unconscious: bool,
    suppress: bool,
) -> bool {
    if suppress {
        return false;
    }
    let power_attack_chance = e.call(COMBAT_STYLE_POWER_ATTACK_CHANCE, &args![style]).u8() as i8;
    let mut chance = power_attack_chance as f32;
    let zero: f64 = e.global(ZERO_DOUBLE);
    if !parity_even(x87_compare(chance as f64, zero) & 0x44) {
        return false;
    }
    let hundred: f64 = e.global(HUNDRED_DOUBLE);
    if x87_compare(chance as f64, hundred) & 0x01 == 0 {
        return true;
    }
    let recoil_stagger_bonus = if recoil_stagger {
        x87_quiet(
            e.call(
                COMBAT_STYLE_POWER_ATTACK_RECOIL_STAGGER_BONUS,
                &args![style],
            )
            .f32(),
        )
    } else {
        0.0
    };
    let unconscious_bonus = if unconscious {
        x87_quiet(fn_00644930(e, style))
    } else {
        0.0
    };
    let fatigue_base = fn_00644950(e, style) as f64;
    let one_minus_fatigue = 1.0 - fatigue as f64;
    let fatigue_mult = fn_00644970(e, style) as f64;
    let fatigue_term = (fatigue_mult * one_minus_fatigue + fatigue_base) as f32;
    chance = (((recoil_stagger_bonus as f64 + unconscious_bonus as f64) + fatigue_term as f64)
        + chance as f64) as f32;
    let roll = (random(e) % 100) as f64 as f32;
    x87_compare(chance as f64, roll as f64) & 0x01 == 0
}

// Translated from 00644930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCombatStyle::Data.fPowerAttackUnconsciousBonus` (Xbox PDB field; `float [+0x94]`).
pub fn fn_00644930(e: &mut Engine, this: Ptr) -> f32 {
    e.mem
        .f32(this.addr() + COMBAT_STYLE_POWER_ATTACK_UNCONSCIOUS_BONUS)
}

// Translated from 00644950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCombatStyle::AdvancedData.fPowerAttackFatigueBase` (Xbox PDB field; `float [+0x100]`).
pub fn fn_00644950(e: &mut Engine, this: Ptr) -> f32 {
    e.mem
        .f32(this.addr() + COMBAT_STYLE_POWER_ATTACK_FATIGUE_BASE)
}

// Translated from 00644970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCombatStyle::AdvancedData.fPowerAttackFatigueMult` (Xbox PDB field; `float [+0x104]`).
pub fn fn_00644970(e: &mut Engine, this: Ptr) -> f32 {
    e.mem
        .f32(this.addr() + COMBAT_STYLE_POWER_ATTACK_FATIGUE_MULT)
}

// Translated from 00644990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the combat style's dodge score for `skill`:
/// `cDodgeChance + (fDodgeSpeedBase + fDodgeSpeedMult * skill) +
/// (fDodgeFatigueBase + fDodgeFatigueMult * (1 - fatigue))`, times
/// `fDodgeDuringAttackMult` (`attacking`) or `fDodgeNoAttackMult`.
pub fn fn_00644990(e: &mut Engine, style: Ptr, skill: i32, fatigue: f32, attacking: bool) -> f32 {
    let skill_double = skill as f64;
    let speed_mult = e.call(COMBAT_STYLE_DODGE_SPEED_MULT, &args![style]).f64();
    let speed_term = speed_mult * skill_double;
    let speed_base = e.call(COMBAT_STYLE_DODGE_SPEED_BASE, &args![style]).f64();
    let speed = (speed_base + speed_term) as f32;
    let one_minus_fatigue = 1.0 - fatigue as f64;
    let fatigue_mult = e.call(COMBAT_STYLE_DODGE_FATIGUE_MULT, &args![style]).f64();
    let fatigue_term = fatigue_mult * one_minus_fatigue;
    let fatigue_base = e.call(COMBAT_STYLE_DODGE_FATIGUE_BASE, &args![style]).f64();
    let fatigue_part = (fatigue_base + fatigue_term) as f32;
    let dodge_chance = fn_00644a30(e, style);
    let mut score = ((dodge_chance as f64 + speed as f64) + fatigue_part as f64) as f32;
    let situation_mult = if attacking {
        fn_00644a50(e, style)
    } else {
        e.call(COMBAT_STYLE_DODGE_NO_ATTACK_MULT, &args![style])
            .f32()
    };
    score = (score as f64 * x87_quiet(situation_mult) as f64) as f32;
    score
}

// Translated from 00644a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCombatStyle::Data.cDodgeChance` (Xbox PDB field; `byte [+0x58]`).
pub fn fn_00644a30(e: &mut Engine, this: Ptr) -> i8 {
    e.mem.i8(this.addr() + COMBAT_STYLE_DODGE_CHANCE)
}

// Translated from 00644a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCombatStyle::AdvancedData.fDodgeDuringAttackMult` (Xbox PDB field; `float [+0xc4]`).
pub fn fn_00644a50(e: &mut Engine, this: Ptr) -> f32 {
    e.mem
        .f32(this.addr() + COMBAT_STYLE_DODGE_DURING_ATTACK_MULT)
}

// Translated from 00644a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a combat style's dodge decision. Returns 0, 1, 2, 4 or 8. It
/// rolls three scores, each `(rand % 5 + 1) * fAIDodgeDecisionBase * m`, where `m` is the
/// style's `fDodgeBackDuringAttackMult` / `fDodgeBackNoAttackMult` (by `flag_10`) for the
/// first, `fDodgeForwardWhileAttackingMult` / `fDodgeForwardNotAttackingMult` (by `flag_0c`)
/// for the second and `fAIDodgeFavorLeftRightMult` (when `flag_1c`, else 1) for the third.
/// `mask_24` bit 1 or `flag_20` zeroes the first score and bit 0 the second. Scores 2 and 1 are
/// answered when the scores and `value_14` / `value_18` order accordingly; otherwise two
/// `rand() % 100` rolls against `cDodgeLeftRightChance` and 48 pick 4 or 8, unless `mask_24`
/// bit 2 / bit 3 forbids it. The meanings of the flags and of the result are not confirmed.
#[allow(clippy::too_many_arguments)]
pub fn fn_00644a70(
    e: &mut Engine,
    style: Ptr,
    flag_0c: bool,
    flag_10: bool,
    value_14: f32,
    value_18: f32,
    flag_1c: bool,
    flag_20: bool,
    mask_24: u16,
) -> u16 {
    let mut result: u16 = 0;
    let first_mult = if flag_10 {
        e.call(COMBAT_STYLE_DODGE_BACK_DURING_ATTACK_MULT, &args![style])
            .f32()
    } else {
        e.call(COMBAT_STYLE_DODGE_BACK_NO_ATTACK_MULT, &args![style])
            .f32()
    };
    let decision_base = setting(e, SETTING_AI_DODGE_DECISION_BASE);
    let scale = decision_base as f64 * first_mult as f64;
    let roll = (random(e) % 5 + 1) as f64;
    let mut first_score = (roll * scale) as f32;

    let second_mult = if flag_0c {
        e.call(
            COMBAT_STYLE_DODGE_FORWARD_WHILE_ATTACKING_MULT,
            &args![style],
        )
        .f32()
    } else {
        e.call(COMBAT_STYLE_DODGE_FORWARD_NOT_ATTACKING_MULT, &args![style])
            .f32()
    };
    let decision_base = setting(e, SETTING_AI_DODGE_DECISION_BASE);
    let scale = decision_base as f64 * second_mult as f64;
    let roll = (random(e) % 5 + 1) as f64;
    let mut second_score = (roll * scale) as f32;

    if mask_24 & 2 != 0 || flag_20 {
        first_score = 0.0;
    }
    if mask_24 & 1 != 0 {
        second_score = 0.0;
    }

    let favor = if flag_1c {
        x87_quiet(setting(e, SETTING_AI_DODGE_FAVOR_LEFT_RIGHT_MULT))
    } else {
        1.0
    };
    let decision_base = setting(e, SETTING_AI_DODGE_DECISION_BASE);
    let scale = decision_base as f64 * favor as f64;
    let roll = (random(e) % 5 + 1) as f64;
    let third_score = (roll * scale) as f32;

    let half: f64 = e.global(HALF_DOUBLE);
    let one_and_a_half: f64 = e.global(ONE_AND_A_HALF_DOUBLE);
    let first = first_score as f64;
    let second = second_score as f64;
    let third = third_score as f64;
    let value_14 = value_14 as f64;
    let value_18 = value_18 as f64;

    // 00644ba6: `FCOMPP second, first`, `TEST AH, 0x41`, `JP 00644bc4`; then `FCOMPP third,
    // first`, `TEST AH, 5`, `JNP 00644bd9` (the "result 2" block).
    let go_to_result_2 = !parity_even(x87_compare(second, first) & 0x41)
        && !parity_even(x87_compare(third, first) & 0x05);
    if go_to_result_2 {
        return 2;
    }
    // 00644bc4: `FCOMPP value_18 * 0.5, value_14`, `TEST AH, 0x41`, `JNZ 00644be7`.
    if x87_compare(value_18 * half, value_14) & 0x41 == 0 {
        return 2;
    }
    // 00644be7: three tests that each jump to the random section (`JP`) unless they pass.
    let random_section = parity_even(x87_compare(first, second) & 0x05)
        || parity_even(x87_compare(third, second) & 0x05)
        || parity_even(x87_compare(value_18 * one_and_a_half, value_14) & 0x05);
    if !random_section {
        return 1;
    }
    // 00644c28: the random section.
    let roll_a = (random(e) % 100) as f64 as f32;
    let dodge_left_right_chance = fn_00644cc0(e, style);
    if parity_even(x87_compare(roll_a as f64, dodge_left_right_chance as f64) & 0x41) {
        return result;
    }
    let roll_b = (random(e) % 100) as f64 as f32;
    let forty_eight: f64 = e.global(FORTY_EIGHT_DOUBLE);
    if !parity_even(x87_compare(roll_b as f64, forty_eight) & 0x41) {
        if mask_24 & 4 == 0 {
            result = 4;
        }
    } else if mask_24 & 8 == 0 {
        result = 8;
    }
    result
}

// Translated from 00644cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCombatStyle::Data.cDodgeLeftRightChance` (Xbox PDB field; `byte [+0x59]`).
pub fn fn_00644cc0(e: &mut Engine, this: Ptr) -> i8 {
    e.mem.i8(this.addr() + COMBAT_STYLE_DODGE_LEFT_RIGHT_CHANCE)
}

// Translated from 00644ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatFormulas::CalcWeaponDamage` (Xbox PDB, `cdecl`, 8 words): the damage of one
/// attack. `owner` is the attacker's `ActorValueOwner` (or null), `weapon` the weapon form
/// (null for fists), `health_ratio` the weapon's condition (0 to 1), `scale` a multiplier,
/// `item_change` the inventory entry (its mod effects add to the damage), `owner_is_actor`
/// makes the ammunition lookup use the attacker. The word at `+0x18` is unused. The
/// weapon-type switch is a jump table in `.text` (not mapped here): types -1 and 0 add the
/// owner's value 0x38, 1, 2 and 13 its value 0x11, every other type nothing.
#[allow(clippy::too_many_arguments)]
pub fn combat_formulas_calc_weapon_damage(
    e: &mut Engine,
    owner: Ptr,
    weapon: Ptr,
    health_ratio: f32,
    scale: f32,
    _unused_4: u32,
    flag_1c: bool,
    item_change: Ptr,
    owner_is_actor: bool,
) -> f32 {
    let (skill, quality) = if weapon.is_null() {
        (0x2d_u32, 1.0f32)
    } else {
        let skill = e.call(WEAPON_GET_SKILL, &args![weapon]).u32();
        // `[weapon + 0x9c]` is a sub-object whose slot 0x10 returns a 16-bit value.
        let quality = e.vcall(weapon.addr().wrapping_add(0x9c), 0x10, &[]).u16();
        (skill, quality as f32)
    };
    let damage_mult = setting(e, SETTING_DAMAGE_WEAPON_MULT);
    let weapon_damage = (damage_mult as f64 * quality as f64) as f32;

    let mut attack_bonus = 0.0f32;
    if !item_change.is_null() {
        attack_bonus = e.with_stack(4, |e, slot| {
            e.mem.set_f32(slot.addr(), 0.0);
            e.call(
                ITEM_CHANGE_HAS_MOD_EFFECT_ACTIVE,
                &args![item_change, 1u32, slot],
            );
            e.mem.f32(slot.addr())
        });
    }
    let bonus_index = if weapon.is_null() {
        Some(0x38_u32)
    } else {
        // The jump table at 0064509c, indexed by `eType + 1` through the byte table at 006450a8.
        match weapon_type(e, weapon).wrapping_add(1) as u32 {
            0 | 1 => Some(0x38),
            2 | 3 | 14 => Some(0x11),
            _ => None,
        }
    };
    if let Some(index) = bonus_index {
        if !owner.is_null() {
            let value = e.vcall(owner.addr(), 0xc, &args![index]).f64();
            attack_bonus = (value + attack_bonus as f64) as f32;
        }
    }

    let mut arm_factor = 1.0f32;
    if !owner.is_null() && (weapon.is_null() || fn_006450c0(e, weapon)) {
        let kind = if weapon.is_null() {
            0
        } else {
            weapon_type(e, weapon)
        };
        let arms = fn_00646880(e, owner, kind);
        let mult = setting(e, SETTING_DAMAGE_ARM_CONDITION_MULT);
        let base = setting(e, SETTING_DAMAGE_ARM_CONDITION_BASE);
        arm_factor = (base as f64 + mult as f64 * arms as f64) as f32;
    }

    let skill_value = if owner.is_null() {
        e.global::<f32>(HUNDRED_FLOAT)
    } else {
        e.call(ACTOR_VALUE_GET_BY_INDEX, &args![owner, skill]).f32()
    };
    let mut skill_factor = arm_factor;
    if !weapon.is_null() && weapon_type(e, weapon) != 0 {
        let skill_base_pointer = setting_pointer(e, SETTING_DAMAGE_SKILL_BASE);
        let skill_mult_pointer = setting_pointer(e, SETTING_DAMAGE_SKILL_MULT);
        let hundredth: f64 = e.global(ONE_HUNDREDTH_DOUBLE);
        let skill_mult = e.mem.f32(skill_mult_pointer);
        let skill_base = e.mem.f32(skill_base_pointer);
        skill_factor = (skill_value as f64 * hundredth * skill_mult as f64 * arm_factor as f64
            + skill_base as f64) as f32;
    }
    let scaled = (skill_factor as f64 * scale as f64) as f32;

    let mut ammo_bonus = 0.0f32;
    if !flag_1c && !weapon.is_null() {
        let holder = if owner_is_actor { owner } else { Ptr::NULL };
        let first = combat_formulas_get_arm_condition(e, weapon, holder);
        if !first.is_null() {
            let second = combat_formulas_get_arm_condition(e, weapon, holder);
            let second_value = e.call(AMMO_GET_VALUE_FROM_OBJECT, &args![second]).u32();
            if second_value != 0 {
                let actor = if owner_is_actor {
                    dynamic_cast(e, owner, TYPE_ACTOR_VALUE_OWNER, TYPE_ACTOR)
                } else {
                    Ptr::NULL
                };
                let mod_active = if item_change.is_null() {
                    0
                } else {
                    e.call(
                        ITEM_CHANGE_HAS_MOD_EFFECT_ACTIVE_OV2,
                        &args![item_change, 0xcu32],
                    )
                    .u8()
                };
                let third = combat_formulas_get_arm_condition(e, weapon, holder);
                let third_value = e.call(AMMO_GET_VALUE_FROM_OBJECT, &args![third]).u32();
                let factor = e.call(FUNCTION_006479E0, &args![third_value]).f64();
                let projectiles = e
                    .call(
                        WEAPON_GET_NUM_PROJECTILES,
                        &args![weapon, mod_active as u32, 0u32, actor],
                    )
                    .u8();
                ammo_bonus = (projectiles as f64 * factor) as f32;
            }
        }
    }

    let condition_mult = fn_00646d00(e, health_ratio);
    let mut perk_mult = 1.0f32;
    let attacker = dynamic_cast(e, owner, TYPE_ACTOR_VALUE_OWNER, TYPE_ACTOR);
    if !attacker.is_null() && e.call(ACTOR_GET_FLAG_566950, &args![attacker]).bool() {
        let player_owner = actor_value_owner_unchecked(player(e));
        let value = e
            .call(ACTOR_VALUE_GET_BY_INDEX, &args![player_owner, 8u32])
            .f32();
        let five_hundredths: f64 = e.global(FIVE_HUNDREDTHS_DOUBLE);
        perk_mult = (value as f64 * five_hundredths + 1.0) as f32;
    }
    let mut weapon_mult = 1.0f32;
    if !weapon.is_null()
        && e.call(WEAPON_FLAG_TEST, &args![weapon]).bool()
        && fn_006450c0(e, weapon)
    {
        let unknown = e.call(FUNCTION_00525430, &args![0x011f_2250u32]).bool();
        if !unknown || attacker != player(e) {
            weapon_mult = e.call(FUNCTION_0084D030, &args![0x011f_6394u32]).f32();
        }
    }

    ((((weapon_damage as f64 + ammo_bonus as f64) * scaled as f64 + attack_bonus as f64)
        * condition_mult as f64)
        * weapon_mult as f64
        * perk_mult as f64) as f32
}

/// `player + 0xa4` without the null test (the exe adds the offset to the global directly).
fn actor_value_owner_unchecked(actor: Ptr) -> Ptr {
    Ptr::new(actor.addr().wrapping_add(ACTOR_VALUE_OWNER))
}

// Translated from 006450c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether the weapon's `eType` (signed byte at +0xf4) is 0 to 2
/// (hand to hand and the melee types).
pub fn fn_006450c0(e: &mut Engine, this: Ptr) -> bool {
    let weapon_type = e.mem.i8(this.addr() + WEAPON_TYPE);
    (0..=2).contains(&weapon_type)
}

// Translated from 006450f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatFormulas::CalcWeaponDamageForDisplay` (Xbox PDB, `cdecl`): the damage shown for
/// `weapon` at `condition`: the damage figure (`00645380`, with the player as holder) and,
/// for a melee weapon with the flag at +0x100 bit 1, `CalcWeaponDamage` with that bit
/// cleared around the call; the ammunition's effect on the figure; a split beam mod
/// (`x 1.3`); then the player's perks (`HandleEntryPoint` 0).
pub fn combat_formulas_calc_weapon_damage_for_display(
    e: &mut Engine,
    weapon: Ptr,
    condition: f32,
    item_change: Ptr,
    ammo: Ptr,
) -> f32 {
    let holder = actor_value_owner(player(e));
    let condition_copy = x87_quiet(condition);
    // The damage figure's own result is overwritten below; only its call matters.
    e.call(
        WEAPON_DAMAGE_FIGURE,
        &args![
            holder,
            weapon,
            condition_copy,
            1u32,
            item_change,
            0u32,
            0u32,
            -1i32,
            0.0f32,
            0.0f32,
            0u32,
            0u32,
            ammo
        ],
    );
    let mut result = if !weapon.is_null()
        && e.call(WEAPON_FLAG_TEST, &args![weapon]).bool()
        && fn_006450c0(e, weapon)
    {
        fn_00645320(e, weapon, false);
        let holder = actor_value_owner(player(e));
        let damage = combat_formulas_calc_weapon_damage(
            e,
            holder,
            weapon,
            condition_copy,
            1.0,
            0,
            false,
            item_change,
            !ammo.is_null(),
        );
        fn_00645320(e, weapon, true);
        damage
    } else {
        let holder = actor_value_owner(player(e));
        combat_formulas_calc_weapon_damage(
            e,
            holder,
            weapon,
            condition_copy,
            1.0,
            0,
            false,
            item_change,
            !ammo.is_null(),
        )
    };

    let mut ammo_form = Ptr::NULL;
    if !weapon.is_null() {
        ammo_form = if ammo.is_null() {
            let holder = player(e);
            e.call(WEAPON_GET_CURRENT_AMMO, &args![weapon, holder])
                .ptr()
        } else {
            ammo
        };
    }
    if ammo_form.is_null() {
        ammo_form = e.call(WEAPON_GET_CURRENT_AMMO, &args![weapon, 0u32]).ptr();
    }
    if !ammo_form.is_null() && !weapon.is_null() && ammo_form != weapon {
        // The exe tests `ammo_form` once more before this call; it is non-null here.
        let effect = e.call(FUNCTION_00923000, &args![ammo_form]).u32();
        result = e
            .call(
                APPLY_PERK_ENTRY_FLOAT,
                &args![0u32, effect, x87_quiet(result)],
            )
            .f32();
    }
    if !item_change.is_null()
        && e.call(
            ITEM_CHANGE_HAS_MOD_EFFECT_ACTIVE_OV2,
            &args![item_change, 0xcu32],
        )
        .bool()
    {
        let split_beam: f64 = e.global(SPLIT_BEAM_MULT_DOUBLE);
        result = (result as f64 * split_beam) as f32;
    }
    let first_player = player(e);
    let second_player = player(e);
    e.with_stack(4, |e, slot| {
        e.mem.set_f32(slot.addr(), result);
        e.call(
            HANDLE_ENTRY_POINT,
            &args![0u32, first_player, weapon, second_player, slot],
        );
        e.mem.f32(slot.addr())
    })
}

// Translated from 00645320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets (`set`) or clears bit 1 of the weapon's flag byte at +0x100
/// (`OBJ_WEAP::cFlags`, Xbox PDB).
pub fn fn_00645320(e: &mut Engine, this: Ptr, set: bool) {
    let flags = e.mem.u8(this.addr() + WEAPON_FLAGS);
    let flags = if set { flags | 0x02 } else { flags & !0x02 };
    e.mem.set_u8(this.addr() + WEAPON_FLAGS, flags);
}

// Translated from 00645d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatFormulas::GetWeaponRateOfFire` (Xbox PDB, `cdecl`): for a melee weapon
/// (`fn_006450c0`) its speed (`+0xf8`); for others `GetWeaponShotsPerSecond` with the owner cast to
/// a `TESObjectREFR`; times the number of projectiles per shot.
pub fn combat_formulas_get_weapon_rate_of_fire(
    e: &mut Engine,
    owner: Ptr,
    weapon: Ptr,
    item_change: Ptr,
) -> f32 {
    let holder: Ptr = if owner.is_null() {
        Ptr::NULL
    } else {
        Ptr::new(owner.addr().wrapping_sub(ACTOR_VALUE_OWNER))
    };
    let rate = if fn_006450c0(e, weapon) {
        e.call(FUNCTION_004E4620, &args![weapon]).f32()
    } else {
        let reference = dynamic_cast(e, owner, TYPE_ACTOR_VALUE_OWNER, TYPE_TES_OBJECT_REFR);
        combat_formulas_get_weapon_shots_per_second(e, reference, weapon, true, item_change)
    };
    let mod_active = if item_change.is_null() {
        0
    } else {
        e.call(
            ITEM_CHANGE_HAS_MOD_EFFECT_ACTIVE_OV2,
            &args![item_change, 0xcu32],
        )
        .u8()
    };
    let projectiles = e
        .call(
            WEAPON_GET_NUM_PROJECTILES,
            &args![weapon, mod_active as u32, 0u32, holder],
        )
        .u8();
    (projectiles as f64 * rate as f64) as f32
}

// Translated from 00645dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatFormulas::GetWeaponShotsPerSecond` (Xbox PDB, `cdecl`): the weapon's shots a second.
/// With `use_animations` and the player's animations it reads the attack animation's `a:`
/// time (`ANIM_GROUP_ACTION_TIME`) for semi-automatic weapons, else the speed times the
/// attack animation's rate; with the reload (`item_change` mods and the clip) folded in.
/// The first word (a `TESObjectREFR` cast by the caller) is not read.
pub fn combat_formulas_get_weapon_shots_per_second(
    e: &mut Engine,
    _unused_0: Ptr,
    weapon: Ptr,
    use_animations: bool,
    item_change: Ptr,
) -> f32 {
    let mut shots_per_second = 0.0f32;
    let mut reload_time = 0.0f32;
    let mut mod_flag_2 = 0u8;
    let mut mod_flag_8 = 0u8;
    let current_player = player(e);
    let animations: Ptr = e
        .call(PLAYER_GET_ANIMATIONS, &args![current_player, 0u32])
        .ptr();
    if !item_change.is_null() {
        mod_flag_2 = e
            .call(
                ITEM_CHANGE_HAS_MOD_EFFECT_ACTIVE_OV2,
                &args![item_change, 2u32],
            )
            .u8();
        mod_flag_8 = e
            .call(
                ITEM_CHANGE_HAS_MOD_EFFECT_ACTIVE_OV2,
                &args![item_change, 8u32],
            )
            .u8();
    }
    if !animations.is_null() && use_animations {
        let mut group = e
            .call(
                WEAPON_GET_ATTACK_ANIM_GROUP,
                &args![weapon, mod_flag_2 as u32],
            )
            .u32();
        if group == 0xff {
            group = 0xb1;
        }
        let weapon_kind = weapon_type(e, weapon);
        let table_entry = e
            .mem
            .u32(WEAPON_TYPE_ANIMATION_TABLE.wrapping_add((weapon_kind as u32).wrapping_mul(4)));
        let lookup = e
            .call(ANIM_GROUP_LOOKUP, &args![0u32, table_entry, group, 0u32])
            .u16();
        let found = e
            .call(
                ANIMATIONS_FIND_GROUP,
                &args![animations, lookup as u32, 0u32],
            )
            .u16();
        let found_id = e.call(ANIM_GROUP_ID, &args![found as u32]).u32();
        if found_id == group {
            let sequence: Ptr = e
                .call(ANIMATIONS_GET_SEQUENCE, &args![animations, found as u32])
                .ptr();
            if !sequence.is_null() {
                let action_time = e.call(ANIM_GROUP_ACTION_TIME, &args![sequence, 1u32]).f64();
                let speed = e.call(FUNCTION_004E4620, &args![weapon]).f64();
                reload_time = (speed * action_time) as f32;
            }
        }
    }
    let is_melee_like = e.call(WEAPON_FLAG_TEST, &args![weapon]).bool();
    if is_melee_like || animations.is_null() {
        let rate = e.call(FUNCTION_00821640, &args![weapon]).f64();
        let attack_speed = tes_object_weap_get_attack_speed(e, weapon, mod_flag_8 != 0);
        shots_per_second = (attack_speed as f64 * rate) as f32;
    } else if !animations.is_null() {
        let mut group = e.call(WEAPON_GET_ATTACK_ANIM_GROUP_2, &args![weapon]).u32();
        if group == 0xff {
            group = 0x20;
        }
        let weapon_kind = weapon_type(e, weapon);
        let table_entry = e
            .mem
            .u32(WEAPON_TYPE_ANIMATION_TABLE.wrapping_add((weapon_kind as u32).wrapping_mul(4)));
        let lookup = e
            .call(ANIM_GROUP_LOOKUP, &args![0u32, table_entry, group, 0u32])
            .u16();
        let found = e
            .call(
                ANIMATIONS_FIND_GROUP,
                &args![animations, lookup as u32, 0u32],
            )
            .u16();
        let found_id = e.call(ANIM_GROUP_ID, &args![found as u32]).u32();
        if found_id == group {
            let sequence: Ptr = e
                .call(ANIMATIONS_GET_SEQUENCE, &args![animations, found as u32])
                .ptr();
            let action_time = e.call(ANIM_GROUP_ACTION_TIME, &args![sequence, 3u32]).f64();
            shots_per_second = (1.0 / action_time) as f32;
        }
    }
    if use_animations {
        let zero: f64 = e.global(ZERO_DOUBLE);
        if x87_compare(reload_time as f64, zero) & 0x41 == 0
            && x87_compare(shots_per_second as f64, zero) & 0x41 == 0
        {
            let clip = if e.call(WEAPON_IS_THROWN_TYPE, &args![weapon]).bool() {
                1
            } else {
                e.call(WEAPON_GET_CLIP_ROUNDS, &args![weapon, mod_flag_2 as u32])
                    .i32()
            };
            let per_clip = (reload_time as f64 / clip as f64) as f32;
            let combined = ((1.0 / shots_per_second as f64) + per_clip as f64) as f32;
            shots_per_second = (1.0 / combined as f64) as f32;
        }
    }
    let speed = e.call(FUNCTION_004E4620, &args![weapon]).f64();
    let scaled = speed * shots_per_second as f64;
    let attack_speed = tes_object_weap_get_attack_speed(e, weapon, mod_flag_8 != 0);
    (attack_speed as f64 * scaled) as f32
}

// Translated from 00646020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectWEAP::GetAttackSpeed` (engine map name): the float at +0x130, plus, when
/// `with_mod`, the weapon's mod effect value 8 (`GetModEffectValue(8, 0)`).
pub fn tes_object_weap_get_attack_speed(e: &mut Engine, this: Ptr, with_mod: bool) -> f32 {
    if with_mod {
        let mod_value = e
            .call(WEAPON_GET_MOD_EFFECT_VALUE, &args![this, 8u32, 0u32])
            .f64();
        let base = e.mem.f32(this.addr() + WEAPON_FIELD_130);
        (base as f64 + mod_value) as f32
    } else {
        e.mem.f32(this.addr() + WEAPON_FIELD_130)
    }
}

// Translated from 00646060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`crates/world/src/dps.rs` calls it `CalcCombatWeaponDPS`; not
/// confirmed here): the damage figure of `weapon` held by `holder` for the combat rating.
/// 0 for a weapon whose flags word (+0x12c) has bit 6 (`fn_00646230`); otherwise the damage
/// figure `00645380` (the holder is the `ActorValueOwner` of `holder` at +0xa4, or +0x100 for a
/// form of type 0x2a / 0x2b), divided by 10 for weapon class 3 and zeroed for class 4
/// (`00522c80`), then divided by 10000 depending on the ammunition's class byte
/// (`006b05d0`) and whether the weapon is a gun type. `ammo` is filled in from the holder
/// when it is null. The role of the words at +0x14, +0x18 and +0x1c is inferred from where
/// they go (ammunition, the inventory entry and an ammunition override of the figure).
// The two ammunition-class branches divide alike but are separate tests in the exe.
#[allow(clippy::if_same_then_else)]
pub fn fn_00646060(
    e: &mut Engine,
    holder: Ptr,
    weapon: Ptr,
    health_ratio: f32,
    ammo: Ptr,
    inventory_entry: Ptr,
    ammo_override: Ptr,
) -> f32 {
    if !weapon.is_null() && fn_00646230(e, weapon) {
        return 0.0;
    }
    let health_copy = x87_quiet(health_ratio);
    let is_actor_form = e.call(FORM_TYPE_BYTE, &args![holder]).u32() == 0x2b
        || e.call(FORM_TYPE_BYTE, &args![holder]).u32() == 0x2a;
    // `00645380`'s holder: +0x100 of an actor-like form, else the `ActorValueOwner` at +0xa4.
    let figure_holder = if is_actor_form {
        if holder.is_null() {
            Ptr::NULL
        } else {
            Ptr::new(holder.addr().wrapping_add(0x100))
        }
    } else {
        actor_value_owner(holder)
    };
    let mut figure = e
        .call(
            WEAPON_DAMAGE_FIGURE,
            &args![
                figure_holder,
                weapon,
                health_copy,
                0u32,
                inventory_entry,
                0u32,
                0u32,
                -1i32,
                0.0f32,
                0.0f32,
                0u32,
                0u32,
                ammo_override
            ],
        )
        .f32();
    let mut ammo = ammo;
    if ammo.is_null() {
        ammo = if is_actor_form {
            e.vcall(holder.addr(), 0x188, &[]).ptr()
        } else {
            e.call(FUNCTION_008A7F80, &args![holder]).ptr()
        };
    }
    let weapon_class = if weapon.is_null() {
        6
    } else {
        e.call(FUNCTION_00522C80, &args![weapon]).u32()
    };
    let ten: f64 = e.global(TEN_DOUBLE);
    if weapon_class == 3 {
        figure = (figure as f64 / ten) as f32;
    } else if weapon_class == 4 {
        figure = 0.0;
    }
    if !ammo.is_null() {
        let ammo_class = e.call(FUNCTION_006B05D0, &args![ammo]).u8() as i8;
        let ten_thousand: f64 = e.global(TEN_THOUSAND_DOUBLE);
        if ammo_class == 1 && !weapon.is_null() && e.call(WEAPON_IS_GUN_TYPE, &args![weapon]).bool()
        {
            figure = (figure as f64 / ten_thousand) as f32;
        } else if ammo_class == 2
            && (weapon.is_null() || !e.call(WEAPON_IS_GUN_TYPE, &args![weapon]).bool())
        {
            figure = (figure as f64 / ten_thousand) as f32;
        }
    }
    figure
}

// Translated from 00646230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether bit 6 (`0x40`) of the word at +0x12c is set.
pub fn fn_00646230(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + WEAPON_FIELD_12C) & 0x40 != 0
}

// Translated from 00646250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: returns the integer in the second word as a float (`FILD`
/// into `ST0`, a `double`-exact value). The first word is unused.
pub fn fn_00646250(_e: &mut Engine, _unused_0: u32, value: i32) -> f64 {
    value as f64
}

// Translated from 00646260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `fDamageToWeaponValue`, passed through the player's perk entry 4
/// (`0059a030`) when the holder's current ammunition object exists.
pub fn fn_00646260(e: &mut Engine, holder: Ptr) -> f32 {
    let mut value = setting(e, SETTING_DAMAGE_TO_WEAPON_VALUE);
    let process: Ptr = e.call(FUNCTION_008D8520, &args![holder]).ptr();
    let slot_result = e.vcall(process.addr(), 0x148, &[]).u32();
    let form: Ptr = e.call(FUNCTION_0044DDC0, &args![slot_result]).ptr();
    if !form.is_null() {
        let first_ammo = e.call(WEAPON_GET_CURRENT_AMMO, &args![form, holder]).u32();
        if first_ammo != 0 {
            let ammo: Ptr = e.call(WEAPON_GET_CURRENT_AMMO, &args![form, holder]).ptr();
            let object = e.call(FUNCTION_00923000, &args![ammo]).u32();
            value = e
                .call(
                    APPLY_PERK_ENTRY_FLOAT,
                    &args![4u32, object, x87_quiet(value)],
                )
                .f32();
        }
    }
    value
}

// Translated from 006462e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `fDamageToArmorPercentage * (first * second)`.
pub fn fn_006462e0(e: &mut Engine, first: f32, second: f32) -> f32 {
    let product = first as f64 * second as f64;
    let percentage = setting(e, SETTING_DAMAGE_TO_ARMOR_PERCENTAGE);
    (percentage as f64 * product) as f32
}

// Translated from 00646310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatFormulas::CalcHandToHandDamage` (Xbox PDB, `cdecl`): stores `CalcWeaponDamage` of bare
/// fists (no weapon, ratio and scale 1) at `damage`; unless `skip_second`, stores half of it
/// at `second_damage`.
pub fn combat_formulas_calc_hand_to_hand_damage(
    e: &mut Engine,
    owner: Ptr,
    skip_second: bool,
    damage: Ptr,
    second_damage: Ptr,
) {
    let value = combat_formulas_calc_weapon_damage(
        e,
        owner,
        Ptr::NULL,
        1.0,
        1.0,
        0,
        false,
        Ptr::NULL,
        false,
    );
    e.mem.set_f32(damage.addr(), value);
    if !skip_second {
        let half: f64 = e.global(HALF_DOUBLE);
        let stored = e.mem.f32(damage.addr());
        e.mem
            .set_f32(second_damage.addr(), (stored as f64 * half) as f32);
    }
}

// Translated from 00646360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatFormulas::CalcArmorRating` (Xbox PDB, `cdecl`): `rating` times the condition factor
/// of `fn_00646d40(condition)`.
pub fn combat_formulas_calc_armor_rating(e: &mut Engine, rating: u16, condition: f32) -> f32 {
    let factor = fn_00646d40(e, condition);
    (rating as f64 * factor as f64) as f32
}

// Translated from 006463a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `fBlockSkillBase + fBlockSkillMult * (float)value`.
pub fn fn_006463a0(e: &mut Engine, value: i32) -> f32 {
    let as_float = value as f32;
    let base_pointer = setting_pointer(e, SETTING_BLOCK_SKILL_BASE);
    let mult_pointer = setting_pointer(e, SETTING_BLOCK_SKILL_MULT);
    let mult = e.mem.f32(mult_pointer);
    let base = e.mem.f32(base_pointer);
    (mult as f64 * as_float as f64 + base as f64) as f32
}

// Translated from 006463e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatFormulas::CalcWeaponReach` (Xbox PDB, `cdecl`): `reach * fCombatDistance`.
pub fn combat_formulas_calc_weapon_reach(e: &mut Engine, reach: f32) -> f32 {
    let distance = setting(e, SETTING_COMBAT_DISTANCE);
    (reach as f64 * distance as f64) as f32
}

// Translated from 00646400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether `amount` of damage knocks the actor `target` down. With
/// `health` (slot 0xc of the object, argument 0x10) above 0 and still above 0 after `amount`:
/// true at once when `amount` as a percent of the slot-4 value reaches above
/// `fKnockdownBaseHealthThreshold`; otherwise, when it is above
/// `fKnockdownCurrentHealthThreshold` percent of the remaining health, a roll of
/// `rand() % 1000` below `1000 * min(chance, fKnockdownChance)` decides, the chance being
/// `(amount * fKnockdownDamageMult + fKnockdownDamageBase) / (agility * 10 *
/// fKnockdownAgilMult + fKnockdownAgilBase)`.
pub fn fn_00646400(e: &mut Engine, target: Ptr, amount: i32) -> bool {
    let zero: f64 = e.global(ZERO_DOUBLE);
    let hundred: f64 = e.global(HUNDRED_DOUBLE);
    let health = e.vcall(target.addr(), 0xc, &args![0x10u32]).f32();
    if x87_compare(health as f64, zero) & 0x41 != 0 {
        return false;
    }
    let remaining = (health as f64 - amount as f64) as f32;
    if x87_compare(remaining as f64, zero) & 0x41 != 0 {
        return false;
    }
    let amount_double = amount as f64;
    let maximum = e.vcall(target.addr(), 0x4, &args![0x10u32]).f64();
    let fraction = (amount_double / maximum) as f32;
    let percent_of_maximum = (fraction as f64 * hundred) as f32;
    let base_threshold = setting(e, SETTING_KNOCKDOWN_BASE_HEALTH_THRESHOLD);
    if !parity_even(x87_compare(base_threshold as f64, percent_of_maximum as f64) & 0x05) {
        return true;
    }
    let fraction = (amount_double / health as f64) as f32;
    let percent_of_health = (fraction as f64 * hundred) as f32;
    let current_threshold = setting(e, SETTING_KNOCKDOWN_CURRENT_HEALTH_THRESHOLD);
    if parity_even(x87_compare(current_threshold as f64, percent_of_health as f64) & 0x05) {
        return false;
    }
    let agility = e
        .call(ACTOR_VALUE_GET_BY_INDEX, &args![target, 0xau32])
        .f64();
    let ten: f64 = e.global(TEN_DOUBLE);
    let agility_scaled = (agility * ten) as f32;
    let agil_base_pointer = setting_pointer(e, SETTING_KNOCKDOWN_AGIL_BASE);
    let agil_mult_pointer = setting_pointer(e, SETTING_KNOCKDOWN_AGIL_MULT);
    let agil_mult = e.mem.f32(agil_mult_pointer);
    let agil_base = e.mem.f32(agil_base_pointer);
    let resistance = (agil_mult as f64 * agility_scaled as f64 + agil_base as f64) as f32;
    let damage_base_pointer = setting_pointer(e, SETTING_KNOCKDOWN_DAMAGE_BASE);
    let damage_mult_pointer = setting_pointer(e, SETTING_KNOCKDOWN_DAMAGE_MULT);
    let damage_mult = e.mem.f32(damage_mult_pointer);
    let damage_base = e.mem.f32(damage_base_pointer);
    let force = (amount_double * damage_mult as f64 + damage_base as f64) as f32;
    let mut chance = (force as f64 / resistance as f64) as f32;
    let chance_limit = setting(e, SETTING_KNOCKDOWN_CHANCE);
    if !parity_even(x87_compare(chance_limit as f64, chance as f64) & 0x41) {
        chance = setting(e, SETTING_KNOCKDOWN_CHANCE);
    }
    let thousand: f64 = e.global(THOUSAND_DOUBLE);
    chance = (chance as f64 * thousand) as f32;
    let roll = random(e) % 1000;
    let threshold = float_to_int(e, chance as f64);
    (roll as i32) < threshold
}

// Translated from 00646580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatFormulas::GetKnockbackSpeed` (Xbox PDB, `cdecl`): `(fKnockbackAgilMult * agility * 10
/// + fKnockbackAgilBase) * (amount * fKnockbackDamageMult + fKnockbackDamageBase)`, the
/// agility being actor value 10 of `target`.
pub fn combat_formulas_get_knockback_speed(e: &mut Engine, target: Ptr, amount: i32) -> f32 {
    let agility = e
        .call(ACTOR_VALUE_GET_BY_INDEX, &args![target, 0xau32])
        .f64();
    let ten: f64 = e.global(TEN_DOUBLE);
    let agility_scaled = (agility * ten) as f32;
    let agil_base_pointer = setting_pointer(e, SETTING_KNOCKBACK_AGIL_BASE);
    let agil_mult_pointer = setting_pointer(e, SETTING_KNOCKBACK_AGIL_MULT);
    let agil_mult = e.mem.f32(agil_mult_pointer);
    let agil_base = e.mem.f32(agil_base_pointer);
    let from_agility = (agil_mult as f64 * agility_scaled as f64 + agil_base as f64) as f32;
    let damage_base_pointer = setting_pointer(e, SETTING_KNOCKBACK_DAMAGE_BASE);
    let damage_mult_pointer = setting_pointer(e, SETTING_KNOCKBACK_DAMAGE_MULT);
    let damage_mult = e.mem.f32(damage_mult_pointer);
    let damage_base = e.mem.f32(damage_base_pointer);
    let from_damage = (amount as f64 * damage_mult as f64 + damage_base as f64) as f32;
    (from_agility as f64 * from_damage as f64) as f32
}

// Translated from 006465f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatFormulas::GetRandomBetween` (Xbox PDB, `cdecl`): `RandomFloat(min, max)` of the two
/// floats, or the minimum when they are equal (unordered values take the random path).
pub fn combat_formulas_get_random_between(e: &mut Engine, first: f32, second: f32) -> f32 {
    let low = e
        .call(FLOAT_MIN, &args![x87_quiet(first), x87_quiet(second)])
        .f32();
    let high = e
        .call(FLOAT_MAX, &args![x87_quiet(first), x87_quiet(second)])
        .f32();
    if !parity_even(x87_compare(high as f64, low as f64) & 0x44) {
        low
    } else {
        e.call(RANDOM_FLOAT, &args![low, high]).f32()
    }
}

// Translated from 00646660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `fArrowWeakGravity * (1 - first) + first * (fArrowGravityBase -
/// second * fArrowGravityMult)`, set to 0 when negative.
pub fn fn_00646660(e: &mut Engine, first: f32, second: f32) -> f32 {
    let second_copy = x87_quiet(second);
    let base_pointer = setting_pointer(e, SETTING_ARROW_GRAVITY_BASE);
    let mult_pointer = setting_pointer(e, SETTING_ARROW_GRAVITY_MULT);
    let mult = e.mem.f32(mult_pointer);
    let base = e.mem.f32(base_pointer);
    let gravity = (base as f64 - second_copy as f64 * mult as f64) as f32;
    let strong = first as f64 * gravity as f64;
    let one_minus_first = 1.0 - first as f64;
    let weak = setting(e, SETTING_ARROW_WEAK_GRAVITY);
    let mut result = (weak as f64 * one_minus_first + strong) as f32;
    let zero: f64 = e.global(ZERO_DOUBLE);
    if !parity_even(x87_compare(result as f64, zero) & 0x05) {
        result = 0.0;
    }
    result
}

// Translated from 006466e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the death force interpolated between the damage and force
/// settings (`ranged` picks the `fDeathForceRanged...` set): the parameter
/// `t = clamp((f(value) - damageMin) / (damageMax - damageMin), 0, 1)` (with `f` = `00408840`)
/// gives `forceMin + (forceMax - forceMin) * t`.
pub fn fn_006466e0(e: &mut Engine, value: f32, ranged: bool) -> f32 {
    let damage_min = if ranged {
        x87_quiet(setting(e, SETTING_DEATH_FORCE_RANGED_DAMAGE_MIN))
    } else {
        x87_quiet(setting(e, SETTING_DEATH_FORCE_DAMAGE_MIN))
    };
    let damage_max = if ranged {
        x87_quiet(setting(e, SETTING_DEATH_FORCE_RANGED_DAMAGE_MAX))
    } else {
        x87_quiet(setting(e, SETTING_DEATH_FORCE_DAMAGE_MAX))
    };
    let force_min = if ranged {
        x87_quiet(setting(e, SETTING_DEATH_FORCE_RANGED_FORCE_MIN))
    } else {
        x87_quiet(setting(e, SETTING_DEATH_FORCE_FORCE_MIN))
    };
    let force_max = if ranged {
        x87_quiet(setting(e, SETTING_DEATH_FORCE_RANGED_FORCE_MAX))
    } else {
        x87_quiet(setting(e, SETTING_DEATH_FORCE_FORCE_MAX))
    };
    let mapped = e
        .call(FLOAT_FUNCTION_00408840, &args![x87_quiet(value)])
        .f32();
    let fraction =
        ((mapped as f64 - damage_min as f64) / (damage_max as f64 - damage_min as f64)) as f32;
    let at_least_zero = e.call(FLOAT_MAX, &args![x87_quiet(fraction), 0.0f32]).f32();
    let parameter = e
        .call(FLOAT_MIN, &args![1.0f32, x87_quiet(at_least_zero)])
        .f32();
    (((force_max as f64 - force_min as f64) * parameter as f64) + force_min as f64) as f32
}

// Translated from 00646800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatFormulas::GetBodyPartCondition` (Xbox PDB, `cdecl`): for actor value `index` in
/// `0x19 ..= 0x1f` (the limb conditions) `min(100, owner value)`, turned into 1.0 / 0.0 (above
/// zero or not) unless `keep_value`; 1.0 for any other index.
pub fn combat_formulas_get_body_part_condition(
    e: &mut Engine,
    owner: Ptr,
    index: i32,
    keep_value: bool,
) -> f32 {
    let mut condition = 1.0f32;
    if (0x19..=0x1f).contains(&index) {
        let value = e.vcall(owner.addr(), 0xc, &args![index]).f32();
        let hundred: f32 = e.global(HUNDRED_FLOAT);
        condition = e.call(FLOAT_MIN, &args![hundred, x87_quiet(value)]).f32();
        if !keep_value {
            let zero: f64 = e.global(ZERO_DOUBLE);
            condition = if x87_compare(condition as f64, zero) & 0x41 != 0 {
                0.0
            } else {
                1.0
            };
        }
    }
    condition
}

// Translated from 00646880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the arm condition for the weapon type `kind` (`eType`): for types
/// 0, 2 and 5 to 9 the mean of limbs `0x1b` and `0x1c`, for every other type (1, 3 and 4, and
/// those past 9, compared as unsigned) the condition of limb `0x1c` (both through
/// `GetBodyPartCondition`, not keeping the value). The exe's `1.0` initial value is never read.
pub fn fn_00646880(e: &mut Engine, owner: Ptr, kind: i32) -> f32 {
    match kind as u32 {
        0 | 2 | 5..=9 => {
            let first = combat_formulas_get_body_part_condition(e, owner, 0x1b, false);
            let second = combat_formulas_get_body_part_condition(e, owner, 0x1c, false);
            let two: f64 = e.global(TWO_DOUBLE);
            ((second as f64 + first as f64) / two) as f32
        }
        _ => combat_formulas_get_body_part_condition(e, owner, 0x1c, false),
    }
}

// Translated from 00646910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the weapon spread of the shooter): the sum of the stance
/// penalties (`fStandingSpreadPenalty` unless `moving_flag_18`, `fWalkingSpreadPenalty` /
/// `fRunningSpreadPenalty` divided by the player's perk factor, `fUnaimedSpreadPenalty` unless
/// `aimed`), the crippled-arm penalties (limbs `0x1b`/`0x1c`), the weapon strength and skill
/// terms (`00647810`, `00647870`), scaled by the skill wobble conversion and not below
/// `fMinGunSpreadValue`. The word at +0x10 is unused; the flags' meanings are inferred from
/// the settings they select.
#[allow(clippy::too_many_arguments)]
pub fn fn_00646910(
    e: &mut Engine,
    actor: Ptr,
    weapon: Ptr,
    _unused_2: u32,
    aimed: bool,
    flag_18: bool,
    walking: bool,
    running: bool,
) -> f32 {
    let mut perk_factor = 1.0f32;
    let mut spread = 0.0f32;
    let current_player = player(e);
    if actor == current_player {
        perk_factor = e.with_stack(4, |e, slot| {
            e.mem.set_f32(slot.addr(), 1.0);
            e.call(
                HANDLE_ENTRY_POINT,
                &args![0x36u32, current_player, weapon, slot],
            );
            e.mem.f32(slot.addr())
        });
        let one: f64 = e.global(ONE_DOUBLE);
        if !parity_even(x87_compare(perk_factor as f64, one) & 0x05) {
            perk_factor = 1.0;
        }
    }
    if !flag_18 {
        let standing = setting(e, SETTING_STANDING_SPREAD_PENALTY);
        spread = (spread as f64 + standing as f64) as f32;
    }
    if walking {
        let walking_penalty = setting(e, SETTING_WALKING_SPREAD_PENALTY);
        spread = (walking_penalty as f64 / perk_factor as f64 + spread as f64) as f32;
    } else if running {
        let running_penalty = setting(e, SETTING_RUNNING_SPREAD_PENALTY);
        spread = (running_penalty as f64 / perk_factor as f64 + spread as f64) as f32;
    }
    if !aimed {
        let unaimed = setting(e, SETTING_UNAIMED_SPREAD_PENALTY);
        spread = (spread as f64 + unaimed as f64) as f32;
    }
    let owner = actor_value_owner(actor);
    let first_arm = combat_formulas_get_body_part_condition(e, owner, 0x1b, false);
    let owner = actor_value_owner(actor);
    let second_arm = combat_formulas_get_body_part_condition(e, owner, 0x1c, false);
    let owner = actor_value_owner(actor);
    // The result for limb 0x19 is stored but never read.
    combat_formulas_get_body_part_condition(e, owner, 0x19, false);
    let zero: f64 = e.global(ZERO_DOUBLE);
    let is_zero = |value: f32| !parity_even(x87_compare(value as f64, zero) & 0x44);
    let owner_pointer = actor.addr().wrapping_add(ACTOR_VALUE_OWNER);
    let owner_state = e.vcall(owner_pointer, 0x8, &args![0x48u32]).u32();
    if owner_state == 0 {
        if is_zero(first_arm) && is_zero(second_arm) {
            let two_handed = fn_00646cb0(e, weapon);
            let penalty = if two_handed {
                setting(e, SETTING_CRIPPLED_ARMS_2H_SPREAD_PENALTY)
            } else {
                setting(e, SETTING_CRIPPLED_ARMS_1H_SPREAD_PENALTY)
            };
            spread = (spread as f64 + penalty as f64) as f32;
        } else if is_zero(first_arm) || is_zero(second_arm) {
            if fn_00646cb0(e, weapon) {
                let penalty = setting(e, SETTING_CRIPPLED_ARM_2H_SPREAD_PENALTY);
                spread = (spread as f64 + penalty as f64) as f32;
            } else if is_zero(second_arm) {
                let penalty = setting(e, SETTING_CRIPPLED_ARM_1H_SPREAD_PENALTY);
                spread = (spread as f64 + penalty as f64) as f32;
            }
        }
    }
    let skill_five = e
        .call(
            ACTOR_VALUE_GET_BY_INDEX,
            &args![Ptr::<()>::new(owner_pointer), 5u32],
        )
        .f64();
    let strength = x87_truncate_to_u32_via_i64(skill_five);
    let perk_value = e.with_stack(4, |e, slot| {
        e.mem.set_f32(slot.addr(), 0.0);
        e.call(HANDLE_ENTRY_POINT, &args![0x35u32, actor, weapon, slot]);
        e.mem.f32(slot.addr())
    });
    let perk_term = x87_truncate_to_u32_via_i64(perk_value as f64);
    let weapon_value = e.call(WEAPON_GET_FIELD_19C, &args![weapon]).u32();
    let strength_penalty = fn_00647810(e, weapon_value, strength, perk_term);
    spread = (spread as f64 + strength_penalty as f64) as f32;
    let skill = if weapon.is_null() {
        0x2d
    } else {
        e.call(WEAPON_GET_SKILL, &args![weapon]).u32()
    };
    let skill_level = e
        .call(
            ACTOR_VALUE_GET_INT_BY_INDEX,
            &args![Ptr::<()>::new(owner_pointer), skill],
        )
        .i32();
    let skill_float = skill_level as f32;
    let skill_truncated = x87_truncate_to_u32_via_i64(skill_float as f64);
    let weapon_skill_requirement = e.call(WEAPON_GET_FIELD_8D8160, &args![weapon]).u32();
    let skill_penalty = fn_00647870(e, weapon_skill_requirement, skill_truncated, 0);
    spread = (spread as f64 + skill_penalty as f64) as f32;
    let wobble = setting(e, SETTING_WOBBLE_TO_SKILL_CONVERSION);
    let hundredth: f64 = e.global(ONE_HUNDREDTH_DOUBLE);
    let skill_scale = (1.0 - skill_float as f64 * wobble as f64 * hundredth) as f32;
    spread = (spread as f64 * skill_scale as f64) as f32;
    let minimum = setting(e, SETTING_MIN_GUN_SPREAD_VALUE);
    if x87_compare(minimum as f64, spread as f64) & 0x41 == 0 {
        spread = setting(e, SETTING_MIN_GUN_SPREAD_VALUE);
    }
    spread
}

// Translated from 00646cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether the weapon's `eType` is 2, 5, 6, 8 or 9 (the cases of a
/// jump table at 00646cf0 indexed by `eType - 2`).
pub fn fn_00646cb0(e: &mut Engine, this: Ptr) -> bool {
    let weapon_type = e.mem.i8(this.addr() + WEAPON_TYPE) as i32;
    matches!(weapon_type, 2 | 5 | 6 | 8 | 9)
}

// Translated from 00646d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: 1.0 for a condition above 0.75, else `1 - (0.75 - condition) *
/// 0.68`.
pub fn fn_00646d00(e: &mut Engine, condition: f32) -> f32 {
    let threshold: f64 = e.global(THREE_QUARTERS_DOUBLE);
    if x87_compare(condition as f64, threshold) & 0x41 != 0 {
        let scale: f64 = e.global(CONDITION_CURVE_SCALE_DOUBLE);
        (1.0 - (threshold - condition as f64) * scale) as f32
    } else {
        1.0
    }
}

// Translated from 00646d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: 1.0 for a condition above 0.5, else `1 - (0.5 - condition)`.
pub fn fn_00646d40(e: &mut Engine, condition: f32) -> f32 {
    let threshold: f64 = e.global(HALF_DOUBLE);
    if x87_compare(condition as f64, threshold) & 0x41 != 0 {
        let one: f64 = e.global(ONE_DOUBLE);
        (1.0 - (threshold - condition as f64) * one) as f32
    } else {
        1.0
    }
}

// Translated from 00646d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the shooter's slot-0xe value divided by the weapon's
/// float at +0x134 (1.0 when that is 0 or the weapon has no flag at +0x100 bit 1), times
/// `008d1eb0` of the weapon when that is not negative.
pub fn fn_00646d80(e: &mut Engine, owner: Ptr, weapon: Ptr) -> f32 {
    if !weapon.is_null() {
        // The weapon's `eType` is read and not used afterwards.
        weapon_type(e, weapon);
    }
    let mut divisor = 1.0f32;
    if !weapon.is_null() && e.call(WEAPON_FLAG_TEST, &args![weapon]).bool() {
        divisor = e.call(FUNCTION_00821640, &args![weapon]).f32();
    }
    let zero: f64 = e.global(ZERO_DOUBLE);
    if !parity_even(x87_compare(divisor as f64, zero) & 0x44) {
        divisor = 1.0;
    }
    let value = if owner.is_null() {
        0.0
    } else {
        e.vcall(owner.addr(), 0xc, &args![0xeu32]).f32()
    };
    let mut result = (value as f64 / divisor as f64) as f32;
    if !weapon.is_null() {
        let factor = e.call(FUNCTION_008D1EB0, &args![weapon]).f64();
        if x87_compare(factor, zero) & 0x01 == 0 {
            let factor = e.call(FUNCTION_008D1EB0, &args![weapon]).f64();
            result = (factor * result as f64) as f32;
        }
    }
    result
}

// Translated from 00646e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: rolls `rand() % 100` against two chances and sets the bytes at
/// `first_result` and `second_result` (cleared first): `second_result` when the roll is below
/// `chance_a` (`base_a + base_c` when `use_direct`, else `(base_c + iCombatExplodePartChance) *
/// base_a * 0.01`), `first_result` when it is below `base_b + iCombatDismemberPartChance`. When
/// `target` has one of them set, `008a16b0(target)` then overrides: 1 = first only, 2 = second
/// only, 3 = neither. The first word is unused.
#[allow(clippy::too_many_arguments)]
pub fn fn_00646e50(
    e: &mut Engine,
    _unused_0: u32,
    target: Ptr,
    base_a: i32,
    base_b: i32,
    base_c: i32,
    use_direct: bool,
    first_result: Ptr,
    second_result: Ptr,
) {
    e.mem.set_u8(first_result.addr(), 0);
    e.mem.set_u8(second_result.addr(), 0);
    let roll = (random(e) % 100) as i32;
    let chance_a = if use_direct {
        base_a.wrapping_add(base_c)
    } else {
        let explode_chance = integer_setting(e, SETTING_INTEGER_COMBAT_EXPLODE_PART_CHANCE);
        let sum = base_c.wrapping_add(explode_chance);
        let hundredth: f64 = e.global(ONE_HUNDREDTH_DOUBLE);
        float_to_int(e, sum as f64 * (base_a as f64 * hundredth))
    };
    let dismember_chance = integer_setting(e, SETTING_INTEGER_COMBAT_DISMEMBER_PART_CHANCE);
    let chance_b = base_b.wrapping_add(dismember_chance);
    if roll < chance_a {
        e.mem.set_u8(second_result.addr(), 1);
    } else if roll < chance_b {
        e.mem.set_u8(first_result.addr(), 1);
    }
    if !target.is_null()
        && (e.mem.u8(second_result.addr()) != 0 || e.mem.u8(first_result.addr()) != 0)
    {
        match e.call(FUNCTION_008A16B0, &args![target]).u32() {
            1 => {
                e.mem.set_u8(second_result.addr(), 0);
                e.mem.set_u8(first_result.addr(), 1);
            }
            2 => {
                e.mem.set_u8(second_result.addr(), 1);
                e.mem.set_u8(first_result.addr(), 0);
            }
            3 => {
                e.mem.set_u8(second_result.addr(), 0);
                e.mem.set_u8(first_result.addr(), 0);
            }
            _ => {}
        }
    }
}

// Translated from 00646f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the weapon's float at +0x124 (the range the VATS chance and the
/// thrown-weapon range use).
pub fn fn_00646f50(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + WEAPON_FIELD_124)
}

// Translated from 00646f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatFormulas::GetVATSChanceToHit` (Xbox PDB, `cdecl`, 12 words): the chance (0 to 0.95
/// as a fraction) that an attack hits the target in VATS.
///
/// `shooter` is the attacker's `ActorValueOwner`, `target` the target reference (its virtual
/// `+0x100` tells actors and `+0x224` grenades apart), `weapon` the weapon form, `distance`
/// the distance between the bodies' edges, `bound_size` the target part's bound,
/// `base_chance` the part's own chance, `spread_bonus` an addition to the weapon's spread.
/// `projectile` and `ammo_object` give the proximity-fused and the base-value factors.
/// `log`, when not null, receives the debug lines of the exe (slot `0x14`, a variable-argument
/// member function that takes `this` on the stack). The words at +0x14 and +0x18 are unused.
///
/// The game's flow: melee and unarmed weapons hit for 100 within `fVATSMeleeMaxDistance`;
/// thrown ones use the range odds and the arm condition; guns use the spread, the target's
/// area and the visible share; destructible and grenade targets scale to a percentage; the
/// stealth bonus applies; the result is capped by `fVATSMaxChance` and divided by 100.
#[allow(clippy::too_many_arguments)]
pub fn combat_formulas_get_vats_chance_to_hit(
    e: &mut Engine,
    shooter: Ptr,
    target: Ptr,
    weapon: Ptr,
    _unused_3: u32,
    _unused_4: u32,
    spread_bonus: f32,
    projectile: Ptr,
    ammo_object: Ptr,
    base_chance: f32,
    distance: f32,
    bound_size: f32,
    log: Ptr,
) -> f32 {
    let zero: f64 = e.global(ZERO_DOUBLE);
    let hundred: f64 = e.global(HUNDRED_DOUBLE);
    let mut chance = 0.0f32;

    // `target` is a non-actor form with flag 0x1000000 (a destructible object).
    let destructible = !target.is_null()
        && e.call(FUNCTION_00452370, &args![target]).bool()
        && !e.vcall(target.addr(), 0x100, &[]).bool();
    let grenade = e.vcall(target.addr(), 0x224, &[]).bool();
    let target_owner: Ptr = if !target.is_null() && e.vcall(target.addr(), 0x100, &[]).bool() {
        Ptr::new(target.addr().wrapping_add(ACTOR_VALUE_OWNER))
    } else {
        Ptr::NULL
    };

    let mut base_value = if !projectile.is_null() {
        e.call(FUNCTION_005DC980, &args![projectile]).u8() as f32
    } else if !ammo_object.is_null() {
        fn_00647730(e, ammo_object, false) as f32
    } else {
        1.0
    };
    if grenade {
        base_value = 1.0;
    }
    if !log.is_null() && (destructible || grenade) {
        let grenade_text = if grenade {
            LOG_GRENADE_TARGET
        } else {
            EMPTY_TEXT
        };
        let destructible_text = if destructible {
            LOG_DESTRUCTIBLE_TARGET
        } else {
            EMPTY_TEXT
        };
        e.vcall(
            log.addr(),
            0x14,
            &args![
                1u32,
                3u32,
                LOG_FORMAT_TWO_TEXTS,
                destructible_text,
                grenade_text
            ],
        );
    }

    let screen_percent = setting(e, SETTING_VATS_SCREEN_PERCENT_FACTOR);
    let visible_factor = e
        .call(FLOAT_MIN, &args![x87_quiet(screen_percent), 1.0f32])
        .f32();
    let visible =
        (visible_factor as f64 * base_chance as f64 + (1.0 - visible_factor as f64)) as f32;

    if !weapon.is_null() && !fn_006450c0(e, weapon) {
        let ammo_object_first = combat_formulas_get_arm_condition(e, weapon, shooter);
        if !ammo_object_first.is_null() {
            let ammo_object_again = combat_formulas_get_arm_condition(e, weapon, shooter);
            let fuse = e
                .call(WEAPON_GET_PROJECTILE_FLOAT, &args![ammo_object_again])
                .f64();
            if parity_even(x87_compare(fuse, zero) & 0x44) {
                if !log.is_null() {
                    e.vcall(
                        log.addr(),
                        0x14,
                        &args![1u32, 3u32, LOG_OUT_OF_EXPLOSION_RANGE],
                    );
                }
                return 0.0;
            }
        }
        if e.call(WEAPON_IS_THROWN_TYPE, &args![weapon]).bool() {
            let range = fn_00646f50(e, weapon);
            let thrown_mult = setting(e, SETTING_VATS_THROWN_WEAPON_RANGE_MULT);
            let max_range = range as f64 * thrown_mult as f64;
            if x87_compare(distance as f64, max_range) & 0x41 == 0 {
                if !log.is_null() {
                    e.vcall(
                        log.addr(),
                        0x14,
                        &args![1u32, 3u32, LOG_OUT_OF_THROWN_RANGE],
                    );
                }
                return 0.0;
            }
            let range_mult = setting(e, SETTING_VATS_GRENADE_RANGE_MULT);
            let mod_max_range = (range as f64 * range_mult as f64) as f32;
            let range_min_pointer = setting_pointer(e, SETTING_VATS_GRENADE_RANGE_MIN);
            let range_min = e.mem.f32(range_min_pointer);
            let range_gap = (mod_max_range as f64 - range_min as f64) as f32;
            let range_min_pointer = setting_pointer(e, SETTING_VATS_GRENADE_RANGE_MIN);
            let range_min = e.mem.f32(range_min_pointer);
            let beyond = (distance as f64 - range_min as f64) as f32;
            let beyond_clamped = e.call(FLOAT_MAX, &args![beyond, 0.0f32]).f64();
            let range_odds = (1.0 - beyond_clamped / range_gap as f64) as f32;
            // The exe tests the weapon again here (it is not null in this branch).
            let weapon_kind = weapon_type(e, weapon);
            let arms = fn_00646880(e, shooter, weapon_kind);
            let arm_condition = if parity_even(x87_compare(arms as f64, zero) & 0x41) {
                1.0f32
            } else {
                let penalty = setting(e, SETTING_CRIPPLED_ARM_1H_SPREAD_PENALTY);
                (1.0 - penalty as f64) as f32
            };
            let weapon_skill = e.call(WEAPON_GET_SKILL, &args![weapon]).u32();
            let skill = e
                .call(ACTOR_VALUE_GET_BY_INDEX, &args![shooter, weapon_skill])
                .f64();
            let grenade_skill_factor = setting(e, SETTING_VATS_GRENADE_SKILL_FACTOR);
            let skill_term = grenade_skill_factor as f64 * skill / hundred;
            let skill_factor = setting(e, SETTING_VATS_SKILL_FACTOR);
            let skill_calc =
                (((1.0 - skill_factor as f64) + skill_term) * arm_condition as f64) as f32;
            let grenade_chance_mult = setting(e, SETTING_VATS_GRENADE_CHANCE_MULT);
            chance = (((range_odds as f64 + skill_calc as f64) * grenade_chance_mult as f64)
                * hundred) as f32;
            if !log.is_null() {
                e.vcall(
                    log.addr(),
                    0x14,
                    &args![
                        1u32,
                        3u32,
                        LOG_MOD_MAX_RANGE,
                        mod_max_range as f64,
                        range_gap as f64,
                        range_odds as f64
                    ],
                );
                e.vcall(
                    log.addr(),
                    0x14,
                    &args![
                        1u32,
                        3u32,
                        LOG_ARM_CONDITION,
                        arm_condition as f64,
                        skill_calc as f64,
                        chance as f64
                    ],
                );
            }
        } else {
            if e.call(WEAPON_IS_GUN_TYPE_3_TO_9, &args![weapon]).bool() && fn_00647790(e, weapon) {
                let range = fn_00646f50(e, weapon);
                if x87_compare(distance as f64, range as f64) & 0x41 == 0 {
                    return 0.0;
                }
            }
            let mut spread_factor = 1.0f32;
            if e.call(WEAPON_IS_GUN_TYPE_3_TO_9, &args![weapon]).bool() {
                let weapon_spread = e.call(WEAPON_GET_SPREAD, &args![weapon, 0u32]).f64();
                spread_factor = (weapon_spread + spread_bonus as f64) as f32;
            }
            let spread_mult = setting(e, SETTING_VATS_SPREAD_MULT);
            let degrees_to_radians: f64 = e.global(DEGREES_TO_RADIANS_DOUBLE);
            let angle = (spread_factor as f64 * spread_mult as f64 * degrees_to_radians) as f32;
            let tangent = e.call(FUNCTION_0057DD70, &args![x87_quiet(angle)]).f64();
            let radius = (tangent * distance as f64) as f32;
            let pi: f64 = e.global(PI_DOUBLE);
            let area = (radius as f64 * radius as f64 * pi) as f32;
            let three: f64 = e.global(THREE_DOUBLE);
            let mut target_area = ((bound_size as f64 / three) * bound_size as f64) as f32;
            if grenade {
                target_area = x87_quiet(setting(e, SETTING_VATS_GRENADE_TARGET_AREA));
            } else {
                let diameter = radius as f64 + radius as f64;
                if x87_compare(bound_size as f64, diameter) & 0x41 == 0 {
                    target_area = ((diameter / bound_size as f64) * target_area as f64) as f32;
                }
                if destructible {
                    let destructible_mult = setting(e, SETTING_VATS_DESTRUCTIBLE_MULT);
                    target_area = (target_area as f64 * destructible_mult as f64) as f32;
                }
            }
            let mut range_spread = 0.0f32;
            if x87_compare(area as f64, zero) & 0x41 == 0 {
                let spread_max = setting(e, SETTING_VATS_RANGE_SPREAD_MAX);
                let ratio = (target_area as f64 / area as f64) as f32;
                range_spread = e
                    .call(FLOAT_MIN, &args![x87_quiet(ratio), x87_quiet(spread_max)])
                    .f32();
            }
            let hit_chance_mult = setting(e, SETTING_VATS_HIT_CHANCE_MULT);
            chance =
                (hit_chance_mult as f64 * visible as f64 * range_spread as f64 * base_value as f64)
                    as f32;
            if !log.is_null() {
                e.vcall(
                    log.addr(),
                    0x14,
                    &args![
                        1u32,
                        3u32,
                        LOG_BOUND_SIZE,
                        bound_size as f64,
                        base_value as f64
                    ],
                );
                e.vcall(
                    log.addr(),
                    0x14,
                    &args![
                        1u32,
                        3u32,
                        LOG_SPREAD_FACTOR,
                        spread_factor as f64,
                        radius as f64,
                        area as f64
                    ],
                );
                e.vcall(
                    log.addr(),
                    0x14,
                    &args![
                        1u32,
                        3u32,
                        LOG_TARGET_AREA,
                        target_area as f64,
                        range_spread as f64,
                        chance as f64
                    ],
                );
            }
        }
        if destructible || grenade {
            chance = (chance as f64 * hundred) as f32;
            let one: f64 = e.global(ONE_DOUBLE);
            if x87_compare(chance as f64, zero) & 0x41 == 0
                && !parity_even(x87_compare(chance as f64, one) & 0x05)
            {
                chance = 1.0;
            }
            if !log.is_null() {
                e.vcall(
                    log.addr(),
                    0x14,
                    &args![1u32, 3u32, LOG_DESTRUCTIBLE_GRENADE_CHANCE, chance as f64],
                );
            }
        }
    } else {
        let melee_distance = setting(e, SETTING_VATS_MELEE_MAX_DISTANCE);
        if x87_compare(melee_distance as f64, distance as f64) & 0x01 == 0 {
            if !log.is_null() {
                e.vcall(log.addr(), 0x14, &args![1u32, 3u32, LOG_MELEE_ASSUME_100]);
            }
            chance = x87_quiet(e.global::<f32>(HUNDRED_FLOAT));
        } else if !log.is_null() {
            e.vcall(log.addr(), 0x14, &args![1u32, 3u32, LOG_MELEE_OUT_OF_RANGE]);
        }
    }

    if !target_owner.is_null() {
        let target_state = e.vcall(target_owner.addr(), 0x8, &args![0x31u32]).i32();
        if target_state > 0 || e.vcall(shooter.addr(), 0x8, &args![0x30u32]).i32() > 0 {
            if !log.is_null() {
                let stealth_mult = setting(e, SETTING_VATS_STEALTH_MULT);
                e.vcall(
                    log.addr(),
                    0x14,
                    &args![
                        1u32,
                        3u32,
                        LOG_STEALTH_BONUS,
                        chance as f64,
                        chance as f64 * stealth_mult as f64
                    ],
                );
            }
            let current_player = player(e);
            let player_perk = e.with_stack(4, |e, slot| {
                e.mem.set_f32(slot.addr(), 0.0);
                e.call(HANDLE_ENTRY_POINT, &args![0x45u32, current_player, slot]);
                e.mem.f32(slot.addr())
            });
            if !parity_even(x87_compare(player_perk as f64, zero) & 0x41) {
                let stealth_mult = setting(e, SETTING_VATS_STEALTH_MULT);
                chance = (chance as f64 * stealth_mult as f64) as f32;
            }
        }
    }
    let max_chance = setting(e, SETTING_VATS_MAX_CHANCE);
    let limited = e
        .call(FLOAT_MIN, &args![x87_quiet(chance), x87_quiet(max_chance)])
        .f32();
    (limited as f64 / hundred) as f32
}

// Translated from 00647730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the byte at +0x11c, plus, when `add_mod`, the weapon's mod effect
/// value 13 truncated (`GetModEffectValue(13, 0)`); the result is `AL` (the sum wraps at 8 bits).
pub fn fn_00647730(e: &mut Engine, this: Ptr, add_mod: bool) -> u8 {
    let base = e.mem.u8(this.addr() + WEAPON_FIELD_11C);
    if add_mod {
        let mod_value = e
            .call(WEAPON_GET_MOD_EFFECT_VALUE, &args![this, 13u32, 0u32])
            .f64();
        (x87_truncate_to_i32(mod_value) as u8).wrapping_add(base)
    } else {
        base
    }
}

// Translated from 00647790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether bit 5 (`0x20`) of the word at +0x12c is set.
pub fn fn_00647790(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + WEAPON_FIELD_12C) & 0x20 != 0
}

// Translated from 006477b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatFormulas::GetWeaponConditionJamMult` (Xbox PDB, `cdecl`): the value of the
/// `fWeaponConditionJam<n>` setting for condition decile `index` (0 to 9; any other index 0.0).
pub fn combat_formulas_get_weapon_condition_jam_mult(e: &mut Engine, index: i32) -> f32 {
    setting_from_table(e, JAM_SETTING_TABLE, index)
}

// Translated from 006477e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the same for the `fWeaponConditionReloadJam<n>` settings.
pub fn fn_006477e0(e: &mut Engine, index: i32) -> f32 {
    setting_from_table(e, RELOAD_JAM_SETTING_TABLE, index)
}

/// The value of the setting `table[index]` for `index` in `0 ..= 9`, else 0.0.
fn setting_from_table(e: &mut Engine, table: u32, index: i32) -> f32 {
    let mut value = 0.0f32;
    if (0..10).contains(&index) {
        let setting_object = e
            .mem
            .u32(table.wrapping_add((index as u32).wrapping_mul(4)));
        value = x87_quiet(setting(e, setting_object));
    }
    value
}

// Translated from 00647810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `fWeapStrengthReqPenalty * clamp(required - strength - bonus, 0,
/// 10)` where the difference is zero when `strength` exceeds `required` (unsigned compare).
pub fn fn_00647810(e: &mut Engine, required: u32, strength: u32, bonus: u32) -> f32 {
    let missing = if strength > required {
        0
    } else {
        required.wrapping_sub(strength).wrapping_sub(bonus) as i32
    };
    let clamped = e.with_stack(4, |e, slot| {
        e.mem.set_i32(slot.addr(), missing);
        e.call(CLAMP_INT, &args![slot, 0u32, 10u32]);
        e.mem.i32(slot.addr())
    });
    let penalty = setting(e, SETTING_WEAP_STRENGTH_REQ_PENALTY);
    (penalty as f64 * clamped as f64) as f32
}

// Translated from 00647870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: like `fn_00647810` for skills: the shortfall `required - skill -
/// bonus` (when `skill <= required`, unsigned) is divided by 10, rounded (`00476b20`) and
/// truncated, clamped to `0 ..= 10`, and multiplied by `fWeapStrengthReqPenalty`.
pub fn fn_00647870(e: &mut Engine, required: u32, skill: u32, bonus: u32) -> f32 {
    let mut steps: u32 = 0;
    if skill <= required {
        let shortfall = required.wrapping_sub(skill).wrapping_sub(bonus);
        let shortfall_float = shortfall as f64 as f32;
        let ten: f64 = e.global(TEN_DOUBLE);
        let tenths = (shortfall_float as f64 / ten) as f32;
        let rounded = e.call(ROUND_FLOAT, &args![x87_quiet(tenths)]).f32();
        steps = x87_truncate_to_u32_via_i64(rounded as f64);
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), steps);
            e.call(CLAMP_UINT, &args![slot, 0u32, 10u32]);
            steps = e.mem.u32(slot.addr());
        });
    }
    let penalty = setting(e, SETTING_WEAP_STRENGTH_REQ_PENALTY);
    (penalty as f64 * steps as f64) as f32
}

// Translated from 00647920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatFormulas::CalcExplosionPower` (Xbox PDB, `cdecl`): `1 - (second / first)^2` when
/// `second <= first` (ordered), else 0.
pub fn combat_formulas_calc_explosion_power(_e: &mut Engine, first: f32, second: f32) -> f32 {
    let mut power = 0.0f32;
    if !parity_even(x87_compare(second as f64, first as f64) & 0x41) {
        power = (second as f64 / first as f64) as f32;
        power = (power as f64 * power as f64) as f32;
        power = (1.0 - power as f64) as f32;
    }
    power
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00525ae0, combat_formulas_get_arm_condition(Ptr, Ptr) -> Ptr),
        entry!(0x00644520, combat_formulas_get_power_attack_damage_multiplier(u32, u32) -> f32),
        entry!(0x00644540, fn_00644540(f32, u32, bool) -> f32),
        entry!(0x00644580, fn_00644580(Ptr, i32, u32, bool, bool) -> f32),
        entry!(0x00644640, fn_00644640(Ptr) -> i8),
        entry!(0x00644660, fn_00644660(Ptr) -> f32),
        entry!(0x00644680, fn_00644680(Ptr, i32, u32, bool, bool, bool, bool, bool) -> f32),
        entry!(0x00644790, fn_00644790(Ptr) -> f32),
        entry!(0x006447b0, fn_006447b0(Ptr) -> f32),
        entry!(0x006447d0, fn_006447d0(Ptr) -> f32),
        entry!(0x006447f0, fn_006447f0(Ptr) -> f32),
        entry!(0x00644810, fn_00644810(Ptr, f32, bool, bool, bool) -> bool),
        entry!(0x00644930, fn_00644930(Ptr) -> f32),
        entry!(0x00644950, fn_00644950(Ptr) -> f32),
        entry!(0x00644970, fn_00644970(Ptr) -> f32),
        entry!(0x00644990, fn_00644990(Ptr, i32, f32, bool) -> f32),
        entry!(0x00644a30, fn_00644a30(Ptr) -> i8),
        entry!(0x00644a50, fn_00644a50(Ptr) -> f32),
        entry!(0x00644a70, fn_00644a70(Ptr, bool, bool, f32, f32, bool, bool, u16) -> u16),
        entry!(0x00644cc0, fn_00644cc0(Ptr) -> i8),
        entry!(0x00644ce0, combat_formulas_calc_weapon_damage(Ptr, Ptr, f32, f32, u32, bool, Ptr, bool) -> f32),
        entry!(0x006450c0, fn_006450c0(Ptr) -> bool),
        entry!(0x006450f0, combat_formulas_calc_weapon_damage_for_display(Ptr, f32, Ptr, Ptr) -> f32),
        entry!(0x00645320, fn_00645320(Ptr, bool)),
        entry!(0x00645d00, combat_formulas_get_weapon_rate_of_fire(Ptr, Ptr, Ptr) -> f32),
        entry!(0x00645dc0, combat_formulas_get_weapon_shots_per_second(Ptr, Ptr, bool, Ptr) -> f32),
        entry!(0x00646020, tes_object_weap_get_attack_speed(Ptr, bool) -> f32),
        entry!(0x00646060, fn_00646060(Ptr, Ptr, f32, Ptr, Ptr, Ptr) -> f32),
        entry!(0x00646230, fn_00646230(Ptr) -> bool),
        entry!(0x00646250, fn_00646250(u32, i32) -> f64),
        entry!(0x00646260, fn_00646260(Ptr) -> f32),
        entry!(0x006462e0, fn_006462e0(f32, f32) -> f32),
        entry!(
            0x00646310,
            combat_formulas_calc_hand_to_hand_damage(Ptr, bool, Ptr, Ptr)
        ),
        entry!(0x00646360, combat_formulas_calc_armor_rating(u16, f32) -> f32),
        entry!(0x006463a0, fn_006463a0(i32) -> f32),
        entry!(0x006463e0, combat_formulas_calc_weapon_reach(f32) -> f32),
        entry!(0x00646400, fn_00646400(Ptr, i32) -> bool),
        entry!(0x00646580, combat_formulas_get_knockback_speed(Ptr, i32) -> f32),
        entry!(0x006465f0, combat_formulas_get_random_between(f32, f32) -> f32),
        entry!(0x00646660, fn_00646660(f32, f32) -> f32),
        entry!(0x006466e0, fn_006466e0(f32, bool) -> f32),
        entry!(0x00646800, combat_formulas_get_body_part_condition(Ptr, i32, bool) -> f32),
        entry!(0x00646880, fn_00646880(Ptr, i32) -> f32),
        entry!(0x00646910, fn_00646910(Ptr, Ptr, u32, bool, bool, bool, bool) -> f32),
        entry!(0x00646cb0, fn_00646cb0(Ptr) -> bool),
        entry!(0x00646d00, fn_00646d00(f32) -> f32),
        entry!(0x00646d40, fn_00646d40(f32) -> f32),
        entry!(0x00646d80, fn_00646d80(Ptr, Ptr) -> f32),
        entry!(
            0x00646e50,
            fn_00646e50(u32, Ptr, i32, i32, i32, bool, Ptr, Ptr)
        ),
        entry!(0x00646f50, fn_00646f50(Ptr) -> f32),
        entry!(0x00646f70, combat_formulas_get_vats_chance_to_hit(Ptr, Ptr, Ptr, u32, u32, f32, Ptr, Ptr, f32, f32, f32, Ptr) -> f32),
        entry!(0x00647730, fn_00647730(Ptr, bool) -> u8),
        entry!(0x00647790, fn_00647790(Ptr) -> bool),
        entry!(0x006477b0, combat_formulas_get_weapon_condition_jam_mult(i32) -> f32),
        entry!(0x006477e0, fn_006477e0(i32) -> f32),
        entry!(0x00647810, fn_00647810(u32, u32, u32) -> f32),
        entry!(0x00647870, fn_00647870(u32, u32, u32) -> f32),
        entry!(0x00647920, combat_formulas_calc_explosion_power(f32, f32) -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A function in the vtables of the fake objects: slots 0x4, 0x8, 0xc, 0x14, ...
    const FAKE_SLOT_4: u32 = 0x00a0_0004;
    const FAKE_SLOT_8: u32 = 0x00a0_0008;
    const FAKE_SLOT_C: u32 = 0x00a0_000c;
    const FAKE_SLOT_10: u32 = 0x00a0_0010;
    const FAKE_SLOT_14: u32 = 0x00a0_0014;
    const FAKE_SLOT_100: u32 = 0x00a0_0100;
    const FAKE_SLOT_148: u32 = 0x00a0_0148;
    const FAKE_SLOT_188: u32 = 0x00a0_0188;
    const FAKE_SLOT_224: u32 = 0x00a0_0224;
    /// Where the fake vtables live.
    const FAKE_TABLE: u32 = 0x00b0_0000;

    /// An engine with the exe's constants the code reads, the setting getters (a setting's
    /// "value pointer" is the setting address itself in these tests: set it with
    /// `set_global`), and the float min / max the exe provides.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_1000,
            0x0101_2000,
            0x0101_6000,
            0x0101_7000,
            0x0101_d000,
            0x0101_f000,
            0x0102_0000,
            0x0102_1000,
            0x0102_3000,
            0x0102_f000,
            0x0103_5000,
            0x0103_c000,
            0x0105_1000,
            0x0118_a000,
            0x0119_b000,
            0x011d_e000,
        ] {
            e.map(page, 0x1000);
        }
        e.map(0x011c_e000, 0x2000);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.set_global(HALF_DOUBLE, 0.5f64);
        e.set_global(TWO_DOUBLE, 2.0f64);
        e.set_global(THREE_DOUBLE, 3.0f64);
        e.set_global(TEN_DOUBLE, 10.0f64);
        e.set_global(HUNDRED_DOUBLE, 100.0f64);
        e.set_global(THOUSAND_DOUBLE, 1000.0f64);
        e.set_global(TEN_THOUSAND_DOUBLE, 10000.0f64);
        e.set_global(ONE_AND_A_HALF_DOUBLE, 1.5f64);
        e.set_global(THREE_QUARTERS_DOUBLE, 0.75f64);
        e.set_global(FORTY_EIGHT_DOUBLE, 48.0f64);
        e.set_global(ONE_HUNDREDTH_DOUBLE, 0.01f32 as f64);
        e.set_global(FIVE_HUNDREDTHS_DOUBLE, 0.05f32 as f64);
        e.set_global(CONDITION_CURVE_SCALE_DOUBLE, 0.68f32 as f64);
        e.set_global(PI_DOUBLE, std::f32::consts::PI as f64);
        e.set_global(
            DEGREES_TO_RADIANS_DOUBLE,
            (std::f32::consts::PI / 180.0) as f64,
        );
        e.set_global(SPLIT_BEAM_MULT_DOUBLE, 1.3f64);
        e.set_global(HUNDRED_FLOAT, 100.0f32);
        e.set_global(QUARTER_FLOAT, 0.25f32);
        e.register(GET_SETTING_VALUE, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(GET_INTEGER_SETTING_VALUE, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        // The exe's `min` / `max` of two floats, with its NaN behaviour.
        e.register(FLOAT_MIN, |_, a| {
            let (first, second) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            let result = if second <= first || first.is_nan() || second.is_nan() {
                second
            } else {
                first
            };
            result.into_ret()
        });
        e.register(FLOAT_MAX, |_, a| {
            let (first, second) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            let result = if second >= first || first.is_nan() || second.is_nan() {
                second
            } else {
                first
            };
            result.into_ret()
        });
        e.register(FTOL, |_, a| {
            let value = f64::take(a, &mut 0);
            Ret {
                eax: value as i32 as u32,
                ..Ret::default()
            }
        });
        e.set_global(PLAYER_REFERENCE, Ptr::<()>::NULL);
        e
    }

    fn returns_f32(e: &mut Engine, addr: u32, value: f32) {
        e.register_double(addr, move |_, _| value.into_ret());
    }

    fn returns_u32(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| value.into_ret());
    }

    /// `rand()` hands out `rolls` in order.
    fn set_rolls(e: &mut Engine, rolls: &[u32]) {
        let mut queue = Vec::from(rolls).into_iter();
        e.register_double(RANDOM, move |_, _| {
            queue
                .next()
                .expect("the code rolled more often than the test queued")
                .into_ret()
        });
    }

    /// `00647ba0` / `00647bd0` clamp the value behind the pointer; `00476b20` rounds.
    fn arithmetic_doubles(e: &mut Engine) {
        e.register(CLAMP_INT, |e, a| {
            let value = e.mem.i32(a[0]).clamp(a[1] as i32, a[2] as i32);
            e.mem.set_i32(a[0], value);
            Ret::default()
        });
        e.register(CLAMP_UINT, |e, a| {
            let value = e.mem.u32(a[0]).clamp(a[1], a[2]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        e.register(ROUND_FLOAT, |_, a| f32::from_bits(a[0]).round().into_ret());
    }

    fn object(e: &mut Engine, size: u32) -> Ptr {
        Ptr::new(e.mem.alloc(size))
    }

    /// An object whose vtable (at `FAKE_TABLE + offset`) points at the fake slot functions.
    fn object_with_vtable(e: &mut Engine, offset: u32) -> Ptr {
        let mut slots = vec![0u32; 0x90];
        slots[1] = FAKE_SLOT_4;
        slots[2] = FAKE_SLOT_8;
        slots[3] = FAKE_SLOT_C;
        slots[4] = FAKE_SLOT_10;
        slots[5] = FAKE_SLOT_14;
        slots[0x100 / 4] = FAKE_SLOT_100;
        slots[0x148 / 4] = FAKE_SLOT_148;
        slots[0x188 / 4] = FAKE_SLOT_188;
        slots[0x224 / 4] = FAKE_SLOT_224;
        e.put_vtable(FAKE_TABLE + offset, &slots);
        let new_object = e.mem.alloc(0x400);
        e.mem.set_u32(new_object, FAKE_TABLE + offset);
        Ptr::new(new_object)
    }

    /// A fake `ActorValueOwner` whose slot 0xc answers `values[(index, value)]`, else 0.
    fn set_owner_values(e: &mut Engine, values: &[(u32, f32)]) {
        let values = values.to_vec();
        e.register_double(FAKE_SLOT_C, move |_, a| {
            values
                .iter()
                .find(|(index, _)| *index == a[1])
                .map_or(0.0, |(_, value)| *value)
                .into_ret()
        });
    }

    fn set_field_f32(e: &mut Engine, object: Ptr, offset: u32, value: f32) {
        e.mem.set_f32(object.addr() + offset, value);
    }

    // ---- 00525ae0 --------------------------------------------------------------------------

    #[test]
    fn get_arm_condition_converts_the_owner_to_the_actor() {
        let mut e = engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(WEAPON_GET_AMMO_OBJECT, move |_, a| {
            log.borrow_mut().push((a[0], a[1]));
            0x77u32.into_ret()
        });
        assert_eq!(e.call(0x0052_5ae0, &args![0x10u32, 0x1a4u32]).u32(), 0x77);
        assert_eq!(e.call(0x0052_5ae0, &args![0x10u32, 0u32]).u32(), 0x77);
        assert_eq!(*seen.borrow(), vec![(0x10, 0x100), (0x10, 0)]);
    }

    // ---- 00644520, 00644540 --------------------------------------------------------------------

    #[test]
    fn power_attack_multiplier_is_the_setting() {
        let mut e = engine();
        e.set_global(SETTING_DAMAGE_POWER_ATTACK_BONUS, 3.0f32);
        assert_eq!(e.call(0x0064_4520, &args![0u32, 0u32]).f32(), 3.0);
    }

    #[test]
    fn fn_00644540_picks_the_hand_or_weapon_multiplier() {
        let mut e = engine();
        e.set_global(SETTING_AI_MELEE_HAND_MULT, 1.5f32);
        e.set_global(SETTING_AI_MELEE_WEAPON_MULT, 3.0f32);
        assert_eq!(e.call(0x0064_4540, &args![2.5f32, 0u32, true]).f32(), 3.75);
        assert_eq!(e.call(0x0064_4540, &args![2.5f32, 0u32, false]).f32(), 7.5);
    }

    // ---- the combat style getters ----------------------------------------------------------------

    fn getter_reads(
        address: u32,
        offset: u32,
        write: impl Fn(&mut Engine, u32),
        read: impl Fn(Ret),
    ) {
        let mut e = engine();
        let style = object(&mut e, 0x120);
        write(&mut e, style.addr() + offset);
        read(e.call(address, &args![style]));
    }

    #[test]
    fn style_float_getters_read_their_fields() {
        let cases = [
            (0x0064_4660, 0xe0),
            (0x0064_4790, 0x80),
            (0x0064_47b0, 0x84),
            (0x0064_47d0, 0xf4),
            (0x0064_47f0, 0xfc),
            (0x0064_4930, 0x94),
            (0x0064_4950, 0x100),
            (0x0064_4970, 0x104),
            (0x0064_4a50, 0xc4),
            (0x0064_6f50, 0x124),
        ];
        for (address, offset) in cases {
            getter_reads(
                address,
                offset,
                |e, at| e.mem.set_f32(at, 12.5),
                |ret| assert_eq!(ret.f32(), 12.5, "{address:08x}"),
            );
        }
    }

    #[test]
    fn style_byte_getters_read_signed_bytes() {
        for (address, offset) in [
            (0x0064_4640, 0x7c),
            (0x0064_4a30, 0x58),
            (0x0064_4cc0, 0x59),
        ] {
            getter_reads(
                address,
                offset,
                |e, at| e.mem.set_u8(at, 0xf6),
                |ret| assert_eq!(ret.i32() as i8, -10, "{address:08x}"),
            );
        }
    }

    #[test]
    fn float_getters_return_quiet_nans_for_signalling_ones() {
        let mut e = engine();
        let style = object(&mut e, 0x120);
        e.mem.set_u32(style.addr() + 0xe0, 0x7fa0_0000);
        let result = e.call(0x0064_4660, &args![style]).f32();
        assert!(result.is_nan());
        assert_eq!(result.to_bits() & 0x0040_0000, 0x0040_0000);
    }

    // ---- 00644580 ---------------------------------------------------------------------------------

    #[test]
    fn block_chance_combines_style_skill_and_situation() {
        let mut e = engine();
        let style = object(&mut e, 0x120);
        e.mem.set_u8(style.addr() + 0x7c, 20);
        set_field_f32(&mut e, style, 0xe0, 10.0);
        returns_f32(&mut e, COMBAT_STYLE_BLOCK_SKILL_MULT, 50.0);
        returns_f32(&mut e, COMBAT_STYLE_BLOCK_DURING_ATTACK_MULT, 2.0);
        returns_f32(&mut e, COMBAT_STYLE_BLOCK_NO_ATTACK_MULT, 4.0);
        e.set_global(SETTING_BLOCK_SCORE_NO_SHIELD_MULT, 0.5f32);
        // base: 20 + 10 + 50 * (50 / 100) = 55
        let call = |e: &mut Engine, flag_14: bool, attacking: bool| {
            e.call(0x0064_4580, &args![style, 50i32, 0u32, flag_14, attacking])
                .f32()
        };
        assert_eq!(call(&mut e, true, true), 110.0);
        assert_eq!(call(&mut e, false, true), 55.0);
        assert_eq!(call(&mut e, false, false), 27.5);
    }

    // ---- 00644680 ---------------------------------------------------------------------------------

    #[test]
    fn attack_score_adds_the_bonuses_and_multiplies() {
        let mut e = engine();
        let style = object(&mut e, 0x120);
        set_field_f32(&mut e, style, 0x80, 1.0);
        set_field_f32(&mut e, style, 0x84, 2.0);
        set_field_f32(&mut e, style, 0xf4, 4.0);
        set_field_f32(&mut e, style, 0xfc, 0.5);
        returns_f32(&mut e, COMBAT_STYLE_ATTACK_HAND_BONUS, 3.0);
        returns_f32(&mut e, COMBAT_STYLE_ATTACK_SKILL_BASE, 5.0);
        returns_f32(&mut e, COMBAT_STYLE_ATTACK_SKILL_MULT, 20.0);
        returns_f32(&mut e, COMBAT_STYLE_ATTACK_NO_ATTACK_MULT, 2.0);
        returns_u32(&mut e, COMBAT_STYLE_ATTACK_CHANCE, 10);
        // skill term: 5 + 20 * 0.5 = 15
        let all = e
            .call(
                0x0064_4680,
                &args![style, 50i32, 0u32, true, true, true, true, true],
            )
            .f32();
        assert_eq!(all, (10.0 + 1.0 + 2.0 + 3.0 + 15.0) * 4.0 * 0.5);
        let none = e
            .call(
                0x0064_4680,
                &args![style, 50i32, 0u32, false, false, false, false, false],
            )
            .f32();
        assert_eq!(none, (10.0 + 15.0) * 2.0);
    }

    // ---- 00644810 ---------------------------------------------------------------------------------

    #[test]
    fn power_attack_decision_follows_the_chance_and_the_roll() {
        let mut e = engine();
        let style = object(&mut e, 0x120);
        set_field_f32(&mut e, style, 0x94, 5.0);
        set_field_f32(&mut e, style, 0x100, 2.0);
        set_field_f32(&mut e, style, 0x104, 4.0);
        returns_f32(&mut e, COMBAT_STYLE_POWER_ATTACK_RECOIL_STAGGER_BONUS, 10.0);
        let call = |e: &mut Engine, chance: u32, rolls: &[u32], suppress: bool| {
            returns_u32(e, COMBAT_STYLE_POWER_ATTACK_CHANCE, chance);
            set_rolls(e, rolls);
            e.call(0x0064_4810, &args![style, 0.5f32, true, true, suppress])
                .bool()
        };
        assert!(!call(&mut e, 50, &[], true), "suppressed");
        assert!(!call(&mut e, 0, &[], false), "chance 0 never");
        assert!(call(&mut e, 100, &[], false), "chance 100 always");
        // 50 + 10 + 5 + (4 * 0.5 + 2) = 69
        assert!(call(&mut e, 50, &[68], false));
        assert!(call(&mut e, 50, &[69], false));
        assert!(!call(&mut e, 50, &[70], false));
    }

    // ---- 00644990 ---------------------------------------------------------------------------------

    #[test]
    fn dodge_score_adds_speed_and_fatigue_terms() {
        let mut e = engine();
        let style = object(&mut e, 0x120);
        e.mem.set_u8(style.addr() + 0x58, 10);
        set_field_f32(&mut e, style, 0xc4, 2.0);
        returns_f32(&mut e, COMBAT_STYLE_DODGE_SPEED_MULT, 2.0);
        returns_f32(&mut e, COMBAT_STYLE_DODGE_SPEED_BASE, 3.0);
        returns_f32(&mut e, COMBAT_STYLE_DODGE_FATIGUE_MULT, 4.0);
        returns_f32(&mut e, COMBAT_STYLE_DODGE_FATIGUE_BASE, 5.0);
        returns_f32(&mut e, COMBAT_STYLE_DODGE_NO_ATTACK_MULT, 0.5);
        // 10 + (3 + 2 * 10) + (5 + 4 * 0.75) = 41
        let attacking = e
            .call(0x0064_4990, &args![style, 10i32, 0.25f32, true])
            .f32();
        assert_eq!(attacking, 82.0);
        let idle = e
            .call(0x0064_4990, &args![style, 10i32, 0.25f32, false])
            .f32();
        assert_eq!(idle, 20.5);
    }

    // ---- 00644a70 ---------------------------------------------------------------------------------

    fn dodge_world() -> (Engine, Ptr) {
        let mut e = engine();
        let style = object(&mut e, 0x120);
        e.mem.set_u8(style.addr() + 0x59, 50);
        e.set_global(SETTING_AI_DODGE_DECISION_BASE, 1.0f32);
        e.set_global(SETTING_AI_DODGE_FAVOR_LEFT_RIGHT_MULT, 1.0f32);
        for address in [
            COMBAT_STYLE_DODGE_BACK_DURING_ATTACK_MULT,
            COMBAT_STYLE_DODGE_BACK_NO_ATTACK_MULT,
            COMBAT_STYLE_DODGE_FORWARD_WHILE_ATTACKING_MULT,
            COMBAT_STYLE_DODGE_FORWARD_NOT_ATTACKING_MULT,
        ] {
            returns_f32(&mut e, address, 1.0);
        }
        (e, style)
    }

    fn dodge(e: &mut Engine, style: Ptr, rolls: &[u32], distances: (f32, f32), mask: u16) -> u16 {
        set_rolls(e, rolls);
        e.call(
            0x0064_4a70,
            &args![
                style,
                true,
                true,
                distances.0,
                distances.1,
                false,
                false,
                mask
            ],
        )
        .u16()
    }

    #[test]
    fn dodge_decision_answers_two_when_the_first_score_is_highest() {
        let (mut e, style) = dodge_world();
        // scores: first 5 (roll 4), second 1, third 1
        assert_eq!(dodge(&mut e, style, &[4, 0, 0], (10.0, 4.0), 0), 2);
    }

    #[test]
    fn dodge_decision_answers_two_when_the_second_distance_is_far_enough() {
        let (mut e, style) = dodge_world();
        // second score above the first, but 20 * 0.5 > 5
        assert_eq!(dodge(&mut e, style, &[0, 4, 0], (5.0, 20.0), 0), 2);
    }

    #[test]
    fn dodge_decision_answers_one_when_the_second_score_wins_within_reach() {
        let (mut e, style) = dodge_world();
        // scores 1, 5, 1; reach 4 * 0.5 < 10 and 4 * 1.5 < 10
        assert_eq!(dodge(&mut e, style, &[0, 4, 0], (10.0, 4.0), 0), 1);
    }

    #[test]
    fn dodge_decision_rolls_for_a_side_when_the_scores_tie() {
        let (mut e, style) = dodge_world();
        // scores 1, 5, 5: the third does not stay under the second -> random section;
        // first roll 10 <= 50, second roll 30 <= 48 -> 4
        assert_eq!(dodge(&mut e, style, &[0, 4, 4, 10, 30], (10.0, 4.0), 0), 4);
        // second roll above 48 -> 8
        assert_eq!(dodge(&mut e, style, &[0, 4, 4, 10, 60], (10.0, 4.0), 0), 8);
        // first roll above the style's chance -> 0
        assert_eq!(dodge(&mut e, style, &[0, 4, 4, 90], (10.0, 4.0), 0), 0);
        // the mask forbids 4 / 8
        assert_eq!(dodge(&mut e, style, &[0, 4, 4, 10, 30], (10.0, 4.0), 4), 0);
        assert_eq!(dodge(&mut e, style, &[0, 4, 4, 10, 60], (10.0, 4.0), 8), 0);
    }

    #[test]
    fn dodge_decision_mask_zeroes_scores() {
        let (mut e, style) = dodge_world();
        // mask bit 1 zeroes the first score, bit 0 the second: scores 0, 0, 5 -> the third
        // is not under the first... random section
        assert_eq!(dodge(&mut e, style, &[4, 4, 4, 10, 30], (10.0, 4.0), 3), 4);
    }

    // ---- 00644ce0 CalcWeaponDamage ----------------------------------------------------------------

    /// Fists: no weapon, an attacker whose slot 0xc answers 5 for value 0x38 and 100 for the
    /// limb conditions.
    fn damage_world() -> (Engine, Ptr) {
        let mut e = engine();
        e.set_global(SETTING_DAMAGE_WEAPON_MULT, 2.0f32);
        e.set_global(SETTING_DAMAGE_ARM_CONDITION_BASE, 0.25f32);
        e.set_global(SETTING_DAMAGE_ARM_CONDITION_MULT, 0.75f32);
        let owner = object_with_vtable(&mut e, 0);
        set_owner_values(
            &mut e,
            &[(0x38, 5.0), (0x11, 7.0), (0x1b, 100.0), (0x1c, 100.0)],
        );
        returns_f32(&mut e, ACTOR_VALUE_GET_BY_INDEX, 80.0);
        returns_u32(&mut e, DYNAMIC_CAST, 0);
        (e, owner)
    }

    #[test]
    fn calc_weapon_damage_for_fists() {
        let (mut e, owner) = damage_world();
        // ((2 * 1) + 0) * (1 * 1.5) + 5
        let damage = e
            .call(
                0x0064_4ce0,
                &args![owner, 0u32, 1.0f32, 1.5f32, 0u32, false, 0u32, false],
            )
            .f32();
        assert_eq!(damage, 8.0);
    }

    #[test]
    fn calc_weapon_damage_with_mods_and_without_an_owner() {
        let mut e = engine();
        e.set_global(SETTING_DAMAGE_WEAPON_MULT, 2.0f32);
        returns_u32(&mut e, DYNAMIC_CAST, 0);
        let item_change = object(&mut e, 0x40);
        e.register(ITEM_CHANGE_HAS_MOD_EFFECT_ACTIVE, |e, a| {
            e.mem.set_f32(a[2], 4.0);
            Ret::default()
        });
        // no weapon, no owner: only the mod bonus 4 is added: ((2 * 1) + 0) * (1 * 1) + 4
        let damage = e
            .call(
                0x0064_4ce0,
                &args![0u32, 0u32, 1.0f32, 1.0f32, 0u32, false, item_change, false],
            )
            .f32();
        assert_eq!(damage, 6.0);
    }

    #[test]
    fn calc_weapon_damage_for_a_gun_with_ammunition() {
        let mut e = engine();
        e.set_global(SETTING_DAMAGE_WEAPON_MULT, 2.0f32);
        e.set_global(SETTING_DAMAGE_SKILL_BASE, 0.25f32);
        e.set_global(SETTING_DAMAGE_SKILL_MULT, 0.5f32);
        returns_u32(&mut e, DYNAMIC_CAST, 0);
        let weapon = object_with_vtable(&mut e, 0x1000);
        // eType 3 (a gun): no melee arm factor, no attack bonus
        e.mem.set_u8(weapon.addr() + 0xf4, 3);
        // the condition quality: the sub-object at +0x9c answers 1 from slot 0x10
        let sub_object = weapon.addr() + 0x9c;
        e.mem.set_u32(sub_object, FAKE_TABLE + 0x1000);
        returns_u32(&mut e, FAKE_SLOT_10, 1);
        returns_u32(&mut e, WEAPON_GET_TYPE, 3);
        returns_u32(&mut e, WEAPON_GET_SKILL, 0x20);
        // the projectile: ammunition object, its value, the factor and the count
        returns_u32(&mut e, WEAPON_GET_AMMO_OBJECT, 0x5000);
        returns_u32(&mut e, AMMO_GET_VALUE_FROM_OBJECT, 3);
        returns_f32(&mut e, FUNCTION_006479E0, 2.5);
        returns_u32(&mut e, WEAPON_GET_NUM_PROJECTILES, 2);
        returns_u32(&mut e, WEAPON_FLAG_TEST, 0);
        // no owner: the skill value is the default 100
        let hundredth = 0.01f32 as f64;
        let skill_factor = (100.0 * hundredth * 0.5 * 1.0 + 0.25) as f32;
        let scaled = (skill_factor as f64 * 1.5) as f32;
        let expected = ((2.0f64 + 2.0 * 2.5) * scaled as f64) as f32;
        let damage = e
            .call(
                0x0064_4ce0,
                &args![0u32, weapon, 1.0f32, 1.5f32, 0u32, false, 0u32, false],
            )
            .f32();
        assert_eq!(damage, expected);
    }

    // ---- 006450c0 and 00646230.. flag tests ---------------------------------------------------------

    #[test]
    fn melee_type_test_covers_zero_to_two() {
        let mut e = engine();
        let weapon = object(&mut e, 0x140);
        for (weapon_type, expected) in
            [(0xffu8, false), (0, true), (1, true), (2, true), (3, false)]
        {
            e.mem.set_u8(weapon.addr() + 0xf4, weapon_type);
            assert_eq!(e.call(0x0064_50c0, &args![weapon]).bool(), expected);
        }
    }

    #[test]
    fn flag_word_tests_read_bits() {
        let mut e = engine();
        let weapon = object(&mut e, 0x140);
        e.mem.set_u32(weapon.addr() + 0x12c, 0x40);
        assert!(e.call(0x0064_6230, &args![weapon]).bool());
        assert!(!e.call(0x0064_7790, &args![weapon]).bool());
        e.mem.set_u32(weapon.addr() + 0x12c, 0x20);
        assert!(!e.call(0x0064_6230, &args![weapon]).bool());
        assert!(e.call(0x0064_7790, &args![weapon]).bool());
    }

    // ---- 006450f0 ---------------------------------------------------------------------------------

    #[test]
    fn display_damage_runs_the_figure_then_the_perks() {
        let (mut e, _) = damage_world();
        let player = object(&mut e, 0x400);
        e.mem.set_u32(player.addr() + 0xa4, FAKE_TABLE);
        e.set_global(PLAYER_REFERENCE, player);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let figure_log = seen.clone();
        e.register_double(WEAPON_DAMAGE_FIGURE, move |_, a| {
            figure_log.borrow_mut().push(a.to_vec());
            3.0f32.into_ret()
        });
        returns_u32(&mut e, WEAPON_GET_CURRENT_AMMO, 0);
        // the perk entry doubles the value behind the pointer
        e.register(HANDLE_ENTRY_POINT, |e, a| {
            let value = e.mem.f32(a[4]);
            e.mem.set_f32(a[4], value * 2.0);
            Ret::default()
        });
        let damage = e.call(0x0064_50f0, &args![0u32, 1.0f32, 0u32, 0u32]).f32();
        // fists: ((2 * 1) + 0) * (1 * 1) + 5 = 7, doubled by the perk
        assert_eq!(damage, 14.0);
        let holder = player.addr() + 0xa4;
        assert_eq!(
            *seen.borrow(),
            vec![vec![
                holder,
                0,
                1.0f32.to_bits(),
                1,
                0,
                0,
                0,
                u32::MAX,
                0,
                0,
                0,
                0,
                0
            ]]
        );
    }

    // ---- 00645320 ---------------------------------------------------------------------------------

    #[test]
    fn weapon_flag_bit_is_set_and_cleared() {
        let mut e = engine();
        let weapon = object(&mut e, 0x140);
        e.mem.set_u8(weapon.addr() + 0x100, 0x05);
        e.call(0x0064_5320, &args![weapon, true]);
        assert_eq!(e.mem.u8(weapon.addr() + 0x100), 0x07);
        e.call(0x0064_5320, &args![weapon, false]);
        assert_eq!(e.mem.u8(weapon.addr() + 0x100), 0x05);
    }

    // ---- 00645d00 and 00645dc0 -----------------------------------------------------------------------

    #[test]
    fn rate_of_fire_of_a_melee_weapon_uses_its_speed() {
        let mut e = engine();
        let weapon = object(&mut e, 0x140);
        e.mem.set_u8(weapon.addr() + 0xf4, 1);
        returns_f32(&mut e, FUNCTION_004E4620, 1.5);
        returns_u32(&mut e, WEAPON_GET_NUM_PROJECTILES, 2);
        let rate = e.call(0x0064_5d00, &args![0u32, weapon, 0u32]).f32();
        assert_eq!(rate, 3.0);
    }

    #[test]
    fn rate_of_fire_of_a_gun_without_animations() {
        let mut e = engine();
        let weapon = object(&mut e, 0x140);
        e.mem.set_u8(weapon.addr() + 0xf4, 3);
        set_field_f32(&mut e, weapon, 0x130, 3.0);
        returns_u32(&mut e, DYNAMIC_CAST, 0x1234);
        returns_u32(&mut e, PLAYER_GET_ANIMATIONS, 0);
        returns_u32(&mut e, WEAPON_FLAG_TEST, 0);
        returns_f32(&mut e, FUNCTION_00821640, 2.0);
        returns_f32(&mut e, FUNCTION_004E4620, 1.5);
        returns_u32(&mut e, WEAPON_GET_NUM_PROJECTILES, 2);
        // shots a second: 3 * 2 = 6; final: 3 * (1.5 * 6) = 27; two projectiles: 54
        let rate = e.call(0x0064_5d00, &args![0u32, weapon, 0u32]).f32();
        assert_eq!(rate, 54.0);
    }

    #[test]
    fn shots_per_second_reads_the_semi_automatic_animation_times() {
        let mut e = engine();
        let weapon = object(&mut e, 0x140);
        e.mem.set_u8(weapon.addr() + 0xf4, 3);
        set_field_f32(&mut e, weapon, 0x130, 1.0);
        let animations = 0x0000_9000u32;
        returns_u32(&mut e, PLAYER_GET_ANIMATIONS, animations);
        returns_u32(&mut e, WEAPON_GET_ATTACK_ANIM_GROUP, 0x10);
        returns_u32(&mut e, WEAPON_GET_ATTACK_ANIM_GROUP_2, 0x10);
        returns_u32(&mut e, WEAPON_GET_TYPE, 3);
        returns_u32(&mut e, ANIM_GROUP_LOOKUP, 0x50);
        returns_u32(&mut e, ANIMATIONS_FIND_GROUP, 0x51);
        returns_u32(&mut e, ANIM_GROUP_ID, 0x10);
        returns_u32(&mut e, ANIMATIONS_GET_SEQUENCE, 0x9100);
        e.register(ANIM_GROUP_ACTION_TIME, |_, a| {
            // action 1 (`reload time`) takes 0.5 s, action 3 (`shot`) 0.25 s
            (if a[1] == 1 { 0.5f32 } else { 0.25f32 }).into_ret()
        });
        returns_f32(&mut e, FUNCTION_004E4620, 2.0);
        returns_u32(&mut e, WEAPON_FLAG_TEST, 0);
        returns_u32(&mut e, WEAPON_IS_THROWN_TYPE, 0);
        returns_u32(&mut e, WEAPON_GET_CLIP_ROUNDS, 10);
        let item_change = object(&mut e, 0x40);
        returns_u32(&mut e, ITEM_CHANGE_HAS_MOD_EFFECT_ACTIVE_OV2, 0);
        // reload 2.0 * 0.5 = 1.0; shots 1 / 0.25 = 4; per clip 1 / 10 = 0.1; period
        // 1 / 4 + 0.1 = 0.35; shots 1 / 0.35; final 1.0 * (2.0 * shots)
        let per_clip = (1.0f64 / 10.0) as f32;
        let combined = ((1.0 / 4.0f64) + per_clip as f64) as f32;
        let shots = (1.0 / combined as f64) as f32;
        let expected = (1.0f64 * (2.0 * shots as f64)) as f32;
        let result = e
            .call(0x0064_5dc0, &args![0u32, weapon, true, item_change])
            .f32();
        assert_eq!(result, expected);
    }

    // ---- 00646020 ---------------------------------------------------------------------------------

    #[test]
    fn attack_speed_adds_the_mod_effect_value_when_asked() {
        let mut e = engine();
        let weapon = object(&mut e, 0x140);
        set_field_f32(&mut e, weapon, 0x130, 1.25);
        returns_f32(&mut e, WEAPON_GET_MOD_EFFECT_VALUE, 0.5);
        assert_eq!(e.call(0x0064_6020, &args![weapon, false]).f32(), 1.25);
        assert_eq!(e.call(0x0064_6020, &args![weapon, true]).f32(), 1.75);
    }

    // ---- 00646060 ---------------------------------------------------------------------------------

    #[test]
    fn combat_rating_scales_by_class_and_ammunition() {
        let mut e = engine();
        let weapon = object(&mut e, 0x140);
        let holder = object_with_vtable(&mut e, 0);
        returns_u32(&mut e, FORM_TYPE_BYTE, 0x2b);
        returns_f32(&mut e, WEAPON_DAMAGE_FIGURE, 100.0);
        returns_u32(&mut e, FUNCTION_00522C80, 3);
        returns_u32(&mut e, FAKE_SLOT_188, 0x6000);
        returns_u32(&mut e, FUNCTION_006B05D0, 1);
        returns_u32(&mut e, WEAPON_IS_GUN_TYPE, 1);
        let args_for = |ammo: u32| args![holder, weapon, 1.0f32, ammo, 0u32, 0u32];
        // class 3: / 10; ammunition class 1 and a gun type: / 10000
        let rating = e.call(0x0064_6060, &args_for(0)).f32();
        assert_eq!(rating, (100.0f64 / 10.0 / 10000.0) as f32);
        // the weapon flagged "not for normal combat" rates 0
        e.mem.set_u32(weapon.addr() + 0x12c, 0x40);
        assert_eq!(e.call(0x0064_6060, &args_for(0)).f32(), 0.0);
    }

    #[test]
    fn combat_rating_of_other_holders_uses_the_owner_sub_object() {
        let mut e = engine();
        let weapon = object(&mut e, 0x140);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(WEAPON_DAMAGE_FIGURE, move |_, a| {
            log.borrow_mut().push(a[0]);
            50.0f32.into_ret()
        });
        returns_u32(&mut e, FORM_TYPE_BYTE, 0x10);
        returns_u32(&mut e, FUNCTION_00522C80, 4);
        returns_u32(&mut e, FUNCTION_008A7F80, 0);
        // class 4 rates 0
        let rating = e.call(
            0x0064_6060,
            &args![0x8000u32, weapon, 1.0f32, 0u32, 0u32, 0u32],
        );
        assert_eq!(rating.f32(), 0.0);
        assert_eq!(*seen.borrow(), vec![0x8000 + 0xa4]);
    }

    // ---- 00646250, 00646260, 006462e0 ----------------------------------------------------------------

    #[test]
    fn fn_00646250_converts_the_second_word() {
        let mut e = engine();
        assert_eq!(e.call(0x0064_6250, &args![0u32, -7i32]).f64(), -7.0);
    }

    #[test]
    fn damage_to_weapon_value_goes_through_the_perk_entry_when_ammunition_exists() {
        let mut e = engine();
        e.set_global(SETTING_DAMAGE_TO_WEAPON_VALUE, 0.5f32);
        let holder = object_with_vtable(&mut e, 0);
        returns_u32(&mut e, FUNCTION_008D8520, holder.addr());
        returns_u32(&mut e, FAKE_SLOT_148, 0x20);
        returns_u32(&mut e, FUNCTION_0044DDC0, 0x30);
        returns_u32(&mut e, FUNCTION_00923000, 0x40);
        returns_u32(&mut e, WEAPON_GET_CURRENT_AMMO, 0x50);
        e.register(APPLY_PERK_ENTRY_FLOAT, |_, a| {
            assert_eq!((a[0], a[1]), (4, 0x40));
            (f32::from_bits(a[2]) * 3.0).into_ret()
        });
        assert_eq!(e.call(0x0064_6260, &args![holder]).f32(), 1.5);
        // without a current ammunition the setting stays
        returns_u32(&mut e, WEAPON_GET_CURRENT_AMMO, 0);
        assert_eq!(e.call(0x0064_6260, &args![holder]).f32(), 0.5);
        // and without the form
        returns_u32(&mut e, FUNCTION_0044DDC0, 0);
        assert_eq!(e.call(0x0064_6260, &args![holder]).f32(), 0.5);
    }

    #[test]
    fn fn_006462e0_scales_the_product() {
        let mut e = engine();
        e.set_global(SETTING_DAMAGE_TO_ARMOR_PERCENTAGE, 0.5f32);
        assert_eq!(e.call(0x0064_62e0, &args![3.0f32, 4.0f32]).f32(), 6.0);
    }

    // ---- 00646310 ---------------------------------------------------------------------------------

    #[test]
    fn hand_to_hand_damage_stores_the_figure_and_half() {
        let (mut e, owner) = damage_world();
        let out = object(&mut e, 0x10);
        let first = out.addr();
        let second = out.addr() + 4;
        e.call(0x0064_6310, &args![owner, false, first, second]);
        // fists at ratio 1 and scale 1: (2 * 1) + 5 = 7, half 3.5
        assert_eq!(e.mem.f32(first), 7.0);
        assert_eq!(e.mem.f32(second), 3.5);
        e.mem.set_f32(second, 99.0);
        e.call(0x0064_6310, &args![owner, true, first, second]);
        assert_eq!(e.mem.f32(second), 99.0);
    }

    // ---- 00646360, 006463a0, 006463e0 ----------------------------------------------------------------

    #[test]
    fn armor_rating_scales_by_the_condition_curve() {
        let mut e = engine();
        assert_eq!(e.call(0x0064_6360, &args![40u16, 0.75f32]).f32(), 40.0);
        // condition 0.25: factor 1 - (0.5 - 0.25) = 0.75
        assert_eq!(e.call(0x0064_6360, &args![40u16, 0.25f32]).f32(), 30.0);
    }

    #[test]
    fn fn_006463a0_is_base_plus_multiplier_times_value() {
        let mut e = engine();
        e.set_global(SETTING_BLOCK_SKILL_BASE, 10.0f32);
        e.set_global(SETTING_BLOCK_SKILL_MULT, 0.5f32);
        assert_eq!(e.call(0x0064_63a0, &args![30i32]).f32(), 25.0);
    }

    #[test]
    fn weapon_reach_scales_by_the_combat_distance() {
        let mut e = engine();
        e.set_global(SETTING_COMBAT_DISTANCE, 128.0f32);
        assert_eq!(e.call(0x0064_63e0, &args![1.5f32]).f32(), 192.0);
    }

    // ---- 00646400 ---------------------------------------------------------------------------------

    fn knockdown_world() -> (Engine, Ptr) {
        let mut e = engine();
        let target = object_with_vtable(&mut e, 0);
        e.register(FAKE_SLOT_C, |_, _| 100.0f32.into_ret());
        e.register(FAKE_SLOT_4, |_, _| 200.0f32.into_ret());
        returns_f32(&mut e, ACTOR_VALUE_GET_BY_INDEX, 5.0);
        e.set_global(SETTING_KNOCKDOWN_BASE_HEALTH_THRESHOLD, 50.0f32);
        e.set_global(SETTING_KNOCKDOWN_CURRENT_HEALTH_THRESHOLD, 20.0f32);
        e.set_global(SETTING_KNOCKDOWN_AGIL_BASE, 2.0f32);
        e.set_global(SETTING_KNOCKDOWN_AGIL_MULT, 1.0f32);
        e.set_global(SETTING_KNOCKDOWN_DAMAGE_BASE, 10.0f32);
        e.set_global(SETTING_KNOCKDOWN_DAMAGE_MULT, 1.0f32);
        e.set_global(SETTING_KNOCKDOWN_CHANCE, 0.5f32);
        (e, target)
    }

    #[test]
    fn knockdown_needs_health_left_and_a_big_enough_blow() {
        let (mut e, target) = knockdown_world();
        // health 100 of 200: 150 is more than the remaining health -> no
        assert!(!e.call(0x0064_6400, &args![target, 100i32]).bool());
        assert!(!e.call(0x0064_6400, &args![target, 150i32]).bool());
        // 120 of 200 is 60 percent, above the base threshold 50 -> knocked down... but 120 > 100
        // health leaves nothing: still no
        assert!(!e.call(0x0064_6400, &args![target, 120i32]).bool());
    }

    #[test]
    fn knockdown_by_a_big_share_of_the_maximum_health() {
        let (mut e, target) = knockdown_world();
        e.register(FAKE_SLOT_C, |_, _| 300.0f32.into_ret());
        // health 300 of 200, amount 120: 60 percent of the maximum > 50 -> true
        assert!(e.call(0x0064_6400, &args![target, 120i32]).bool());
    }

    #[test]
    fn knockdown_rolls_against_the_chance() {
        let (mut e, target) = knockdown_world();
        // amount 40 of 100 health (200 maximum): 20 percent of the maximum (not above 50),
        // 40 percent of the health (above 20): agility 5 * 10 = 50 -> 50 + 2 = 52;
        // force 40 + 10 = 50; chance 50 / 52 limited to 0.5 -> 500
        set_rolls(&mut e, &[499]);
        assert!(e.call(0x0064_6400, &args![target, 40i32]).bool());
        set_rolls(&mut e, &[500]);
        assert!(!e.call(0x0064_6400, &args![target, 40i32]).bool());
    }

    #[test]
    fn knockdown_below_the_current_health_threshold_never_happens() {
        let (mut e, target) = knockdown_world();
        // 10 of 100 health is 10 percent, not above 20
        assert!(!e.call(0x0064_6400, &args![target, 10i32]).bool());
    }

    // ---- 00646580 ---------------------------------------------------------------------------------

    #[test]
    fn knockback_speed_multiplies_the_agility_and_damage_terms() {
        let mut e = engine();
        let target = object(&mut e, 0x40);
        returns_f32(&mut e, ACTOR_VALUE_GET_BY_INDEX, 5.0);
        e.set_global(SETTING_KNOCKBACK_AGIL_BASE, 2.0f32);
        e.set_global(SETTING_KNOCKBACK_AGIL_MULT, 0.5f32);
        e.set_global(SETTING_KNOCKBACK_DAMAGE_BASE, 1.0f32);
        e.set_global(SETTING_KNOCKBACK_DAMAGE_MULT, 0.25f32);
        // (0.5 * 50 + 2) * (40 * 0.25 + 1) = 27 * 11
        assert_eq!(e.call(0x0064_6580, &args![target, 40i32]).f32(), 297.0);
    }

    // ---- 006465f0 ---------------------------------------------------------------------------------

    #[test]
    fn random_between_returns_the_minimum_when_equal() {
        let mut e = engine();
        e.register(RANDOM_FLOAT, |_, a| {
            ((f32::from_bits(a[0]) + f32::from_bits(a[1])) / 2.0).into_ret()
        });
        assert_eq!(e.call(0x0064_65f0, &args![2.0f32, 6.0f32]).f32(), 4.0);
        assert_eq!(e.call(0x0064_65f0, &args![6.0f32, 2.0f32]).f32(), 4.0);
        assert_eq!(e.call(0x0064_65f0, &args![3.0f32, 3.0f32]).f32(), 3.0);
        // a NaN is "not equal": the random function decides
        assert!(e.call(0x0064_65f0, &args![3.0f32, f32::NAN]).f32().is_nan());
    }

    // ---- 00646660 ---------------------------------------------------------------------------------

    #[test]
    fn arrow_gravity_blends_the_weak_and_strong_values() {
        let mut e = engine();
        e.set_global(SETTING_ARROW_GRAVITY_BASE, 3.0f32);
        e.set_global(SETTING_ARROW_GRAVITY_MULT, 0.5f32);
        e.set_global(SETTING_ARROW_WEAK_GRAVITY, 2.0f32);
        // gravity = 3 - 2 * 0.5 = 2; 0.25 * 2 + 0.75 * 2 = 2
        assert_eq!(e.call(0x0064_6660, &args![0.25f32, 2.0f32]).f32(), 2.0);
        // negative results clamp to 0: gravity = 3 - 10 * 0.5 = -2; 1 * -2
        assert_eq!(e.call(0x0064_6660, &args![1.0f32, 10.0f32]).f32(), 0.0);
    }

    // ---- 006466e0 ---------------------------------------------------------------------------------

    #[test]
    fn death_force_interpolates_between_the_settings() {
        let mut e = engine();
        e.set_global(SETTING_DEATH_FORCE_DAMAGE_MIN, 10.0f32);
        e.set_global(SETTING_DEATH_FORCE_DAMAGE_MAX, 30.0f32);
        e.set_global(SETTING_DEATH_FORCE_FORCE_MIN, 100.0f32);
        e.set_global(SETTING_DEATH_FORCE_FORCE_MAX, 200.0f32);
        e.set_global(SETTING_DEATH_FORCE_RANGED_DAMAGE_MIN, 0.0f32);
        e.set_global(SETTING_DEATH_FORCE_RANGED_DAMAGE_MAX, 4.0f32);
        e.set_global(SETTING_DEATH_FORCE_RANGED_FORCE_MIN, 1.0f32);
        e.set_global(SETTING_DEATH_FORCE_RANGED_FORCE_MAX, 3.0f32);
        // the float function of 00408840 is the identity here
        e.register(FLOAT_FUNCTION_00408840, |_, a| {
            f32::from_bits(a[0]).into_ret()
        });
        assert_eq!(e.call(0x0064_66e0, &args![20.0f32, false]).f32(), 150.0);
        assert_eq!(e.call(0x0064_66e0, &args![40.0f32, false]).f32(), 200.0);
        assert_eq!(e.call(0x0064_66e0, &args![0.0f32, false]).f32(), 100.0);
        assert_eq!(e.call(0x0064_66e0, &args![1.0f32, true]).f32(), 1.5);
    }

    // ---- 00646800 and 00646880 --------------------------------------------------------------------

    #[test]
    fn body_part_condition_is_capped_and_boolean() {
        let mut e = engine();
        let owner = object_with_vtable(&mut e, 0);
        set_owner_values(
            &mut e,
            &[(0x19, 50.0), (0x1a, 250.0), (0x1b, 0.0), (0x1c, -3.0)],
        );
        let call = |e: &mut Engine, index: i32, keep: bool| {
            e.call(0x0064_6800, &args![owner, index, keep]).f32()
        };
        assert_eq!(call(&mut e, 0x19, true), 50.0);
        assert_eq!(call(&mut e, 0x1a, true), 100.0);
        assert_eq!(call(&mut e, 0x19, false), 1.0);
        assert_eq!(call(&mut e, 0x1b, false), 0.0);
        assert_eq!(call(&mut e, 0x1c, false), 0.0);
        assert_eq!(call(&mut e, 0x18, true), 1.0, "outside 0x19..=0x1f");
        assert_eq!(call(&mut e, 0x20, false), 1.0);
    }

    #[test]
    fn weapon_types_past_the_table_use_the_one_arm_block() {
        let mut e = engine();
        let owner = object_with_vtable(&mut e, 0);
        // the right arm (0x1c) broken, the left one (0x1b) fine
        set_owner_values(&mut e, &[(0x1b, 80.0), (0x1c, 0.0)]);
        let call = |e: &mut Engine, kind: i32| e.call(0x0064_6880, &args![owner, kind]).f32();
        assert_eq!(call(&mut e, 0), 0.5, "both arms");
        assert_eq!(call(&mut e, 1), 0.0, "one arm: the right one");
        // The exe's `JA` takes every type past 9 (and the negative ones) to the one-arm block.
        assert_eq!(call(&mut e, 10), 0.0);
        assert_eq!(call(&mut e, -1), 0.0);
    }

    #[test]
    fn arm_condition_of_a_weapon_type_uses_one_or_both_arms() {
        let mut e = engine();
        let owner = object_with_vtable(&mut e, 0);
        // left arm (0x1b) broken, right arm (0x1c) fine
        set_owner_values(&mut e, &[(0x1b, 0.0), (0x1c, 80.0)]);
        let call = |e: &mut Engine, kind: i32| e.call(0x0064_6880, &args![owner, kind]).f32();
        assert_eq!(call(&mut e, 1), 1.0, "one-handed melee: right arm only");
        assert_eq!(call(&mut e, 3), 1.0);
        assert_eq!(call(&mut e, 4), 1.0);
        assert_eq!(call(&mut e, 0), 0.5, "mean of both arms");
        assert_eq!(call(&mut e, 5), 0.5);
        assert_eq!(call(&mut e, 10), 1.0, "unknown types");
        assert_eq!(call(&mut e, -1), 1.0);
    }

    // ---- 00646910 ---------------------------------------------------------------------------------

    fn spread_world() -> (Engine, Ptr, Ptr) {
        let mut e = engine();
        let actor = object(&mut e, 0x400);
        // the owner sub-object at +0xa4 has its own vtable
        let owner_table = FAKE_TABLE + 0x2000;
        let slots = {
            let mut slots = vec![0u32; 8];
            slots[2] = FAKE_SLOT_8;
            slots[3] = FAKE_SLOT_C;
            slots
        };
        e.put_vtable(owner_table, &slots);
        e.mem.set_u32(actor.addr() + 0xa4, owner_table);
        let weapon = object(&mut e, 0x200);
        e.mem.set_u8(weapon.addr() + 0xf4, 3);
        e.set_global(SETTING_STANDING_SPREAD_PENALTY, 0.5f32);
        e.set_global(SETTING_WALKING_SPREAD_PENALTY, 1.0f32);
        e.set_global(SETTING_RUNNING_SPREAD_PENALTY, 2.0f32);
        e.set_global(SETTING_UNAIMED_SPREAD_PENALTY, 4.0f32);
        e.set_global(SETTING_CRIPPLED_ARMS_2H_SPREAD_PENALTY, 8.0f32);
        e.set_global(SETTING_CRIPPLED_ARMS_1H_SPREAD_PENALTY, 16.0f32);
        e.set_global(SETTING_CRIPPLED_ARM_2H_SPREAD_PENALTY, 32.0f32);
        e.set_global(SETTING_CRIPPLED_ARM_1H_SPREAD_PENALTY, 64.0f32);
        e.set_global(SETTING_WOBBLE_TO_SKILL_CONVERSION, 0.5f32);
        e.set_global(SETTING_MIN_GUN_SPREAD_VALUE, 0.25f32);
        returns_u32(&mut e, FAKE_SLOT_8, 0);
        set_owner_values(&mut e, &[(0x1b, 100.0), (0x1c, 100.0), (0x19, 100.0)]);
        returns_f32(&mut e, ACTOR_VALUE_GET_BY_INDEX, 0.0);
        returns_u32(&mut e, WEAPON_GET_FIELD_19C, 0);
        returns_u32(&mut e, WEAPON_GET_SKILL, 0x20);
        returns_u32(&mut e, ACTOR_VALUE_GET_INT_BY_INDEX, 0);
        returns_u32(&mut e, WEAPON_GET_FIELD_8D8160, 0);
        arithmetic_doubles(&mut e);
        e.register(HANDLE_ENTRY_POINT, |_, _| Ret::default());
        // 647810 and 647870 run for real: the strength / skill requirement is 0
        (e, actor, weapon)
    }

    #[test]
    fn spread_sums_the_stance_penalties() {
        let (mut e, actor, weapon) = spread_world();
        // stationary and aimed: standing 0.5 only; skill scale 1 - 0 = 1
        let aimed_still = e
            .call(
                0x0064_6910,
                &args![actor, weapon, 0u32, true, false, false, false],
            )
            .f32();
        assert_eq!(aimed_still, 0.5);
        // not aimed, walking: 0.5 + 1 + 4
        let unaimed_walk = e
            .call(
                0x0064_6910,
                &args![actor, weapon, 0u32, false, false, true, false],
            )
            .f32();
        assert_eq!(unaimed_walk, 5.5);
        // running instead of walking: 0.5 + 2 + 4; flag_18 skips the standing term
        let unaimed_run = e
            .call(
                0x0064_6910,
                &args![actor, weapon, 0u32, false, true, false, true],
            )
            .f32();
        assert_eq!(unaimed_run, 6.0);
    }

    #[test]
    fn spread_gets_the_crippled_arm_penalties_and_the_minimum() {
        let (mut e, actor, weapon) = spread_world();
        let spread = |e: &mut Engine, weapon_type: u8| {
            e.mem.set_u8(weapon.addr() + 0xf4, weapon_type);
            e.call(
                0x0064_6910,
                &args![actor, weapon, 0u32, true, true, false, false],
            )
            .f32()
        };
        // both arms crippled: 1-handed type 3 -> 16, 2-handed type 5 -> 8
        set_owner_values(&mut e, &[(0x1b, 0.0), (0x1c, 0.0), (0x19, 100.0)]);
        assert_eq!(spread(&mut e, 3), 16.0);
        assert_eq!(spread(&mut e, 5), 8.0);
        // only the right arm (0x1c): 1H 64, 2H 32; only the left arm: 1H skipped, 2H 32
        set_owner_values(&mut e, &[(0x1b, 100.0), (0x1c, 0.0), (0x19, 100.0)]);
        assert_eq!(spread(&mut e, 3), 64.0);
        assert_eq!(spread(&mut e, 5), 32.0);
        set_owner_values(&mut e, &[(0x1b, 0.0), (0x1c, 100.0), (0x19, 100.0)]);
        // nothing is added for the left arm alone with a one-handed weapon: the minimum applies
        assert_eq!(spread(&mut e, 3), 0.25);
        assert_eq!(spread(&mut e, 5), 32.0);
    }

    #[test]
    fn spread_of_the_player_uses_the_perk_factor_and_skill_terms() {
        let (mut e, actor, weapon) = spread_world();
        e.set_global(PLAYER_REFERENCE, actor);
        // entry 0x36 gives a factor of 2: walking 1 / 2; entry 0x35 gives 3 (weapon-skill term);
        // 647810(0, 0, 3) is 0: the weapon has no strength requirement
        e.register(HANDLE_ENTRY_POINT, |e, a| {
            if a[0] == 0x36 {
                e.mem.set_f32(a[3], 2.0);
            }
            Ret::default()
        });
        // skill 10 through 66ef20: the wobble scale is 1 - 10 * 0.5 * 0.01 = 0.95
        returns_u32(&mut e, ACTOR_VALUE_GET_INT_BY_INDEX, 10);
        let spread = e
            .call(
                0x0064_6910,
                &args![actor, weapon, 0u32, true, true, true, false],
            )
            .f32();
        let scale = (1.0 - 10.0f32 as f64 * 0.5 * (0.01f32 as f64)) as f32;
        assert_eq!(spread, (0.5f32 as f64 * scale as f64) as f32);
    }

    // ---- 00646cb0 ---------------------------------------------------------------------------------

    #[test]
    fn two_handed_types_are_the_jump_table_cases() {
        let mut e = engine();
        let weapon = object(&mut e, 0x140);
        for weapon_type in -1i32..12 {
            e.mem.set_u8(weapon.addr() + 0xf4, weapon_type as u8);
            let expected = matches!(weapon_type, 2 | 5 | 6 | 8 | 9);
            assert_eq!(
                e.call(0x0064_6cb0, &args![weapon]).bool(),
                expected,
                "type {weapon_type}"
            );
        }
    }

    // ---- 00646d00, 00646d40 -------------------------------------------------------------------------

    #[test]
    fn condition_curves() {
        let mut e = engine();
        assert_eq!(e.call(0x0064_6d00, &args![0.9f32]).f32(), 1.0);
        let expected = (1.0 - (0.75 - 0.25f64) * (0.68f32 as f64)) as f32;
        assert_eq!(e.call(0x0064_6d00, &args![0.25f32]).f32(), expected);
        assert_eq!(e.call(0x0064_6d40, &args![0.9f32]).f32(), 1.0);
        assert_eq!(e.call(0x0064_6d40, &args![0.25f32]).f32(), 0.75);
        // a NaN takes the formula path in both
        assert!(e.call(0x0064_6d40, &args![f32::NAN]).f32().is_nan());
    }

    // ---- 00646d80 ---------------------------------------------------------------------------------

    #[test]
    fn fn_00646d80_divides_the_owner_value_by_the_weapon_float() {
        let mut e = engine();
        let owner = object_with_vtable(&mut e, 0);
        let weapon = object(&mut e, 0x140);
        set_owner_values(&mut e, &[(0xe, 12.0)]);
        returns_u32(&mut e, WEAPON_GET_TYPE, 1);
        returns_u32(&mut e, WEAPON_FLAG_TEST, 1);
        returns_f32(&mut e, FUNCTION_00821640, 4.0);
        returns_f32(&mut e, FUNCTION_008D1EB0, 2.0);
        // 12 / 4 = 3, times 2
        assert_eq!(e.call(0x0064_6d80, &args![owner, weapon]).f32(), 6.0);
        // a zero divisor becomes 1; a negative factor is skipped
        returns_f32(&mut e, FUNCTION_00821640, 0.0);
        returns_f32(&mut e, FUNCTION_008D1EB0, -1.0);
        assert_eq!(e.call(0x0064_6d80, &args![owner, weapon]).f32(), 12.0);
        // no weapon, no owner
        assert_eq!(e.call(0x0064_6d80, &args![0u32, 0u32]).f32(), 0.0);
    }

    // ---- 00646e50 ---------------------------------------------------------------------------------

    #[test]
    fn part_chances_roll_against_both_thresholds() {
        let mut e = engine();
        e.set_global(SETTING_INTEGER_COMBAT_EXPLODE_PART_CHANCE, 10i32);
        e.set_global(SETTING_INTEGER_COMBAT_DISMEMBER_PART_CHANCE, 5i32);
        let bytes = e.mem.alloc(8);
        let (first, second): (Ptr, Ptr) = (Ptr::new(bytes), Ptr::new(bytes + 1));
        let run = |e: &mut Engine, roll: u32, direct: bool, target: u32| {
            set_rolls(e, &[roll]);
            e.call(
                0x0064_6e50,
                &args![0u32, target, 40i32, 20i32, 30i32, direct, first, second],
            );
            (e.mem.u8(first.addr()), e.mem.u8(second.addr()))
        };
        // not direct: threshold A = (30 + 10) * (40 * 0.01) = 16 (0.01f widened: 15.99.. -> 15);
        // threshold B = 20 + 5 = 25
        assert_eq!(run(&mut e, 14, false, 0), (0, 1));
        assert_eq!(run(&mut e, 20, false, 0), (1, 0));
        assert_eq!(run(&mut e, 90, false, 0), (0, 0));
        // direct: threshold A = 40 + 30 = 70
        assert_eq!(run(&mut e, 69, true, 0), (0, 1));
        assert_eq!(run(&mut e, 71, true, 0), (0, 0));
        // the target's override: 1 first only, 2 second only, 3 neither
        returns_u32(&mut e, FUNCTION_008A16B0, 1);
        assert_eq!(run(&mut e, 69, true, 0x1000), (1, 0));
        returns_u32(&mut e, FUNCTION_008A16B0, 2);
        assert_eq!(run(&mut e, 69, true, 0x1000), (0, 1));
        returns_u32(&mut e, FUNCTION_008A16B0, 3);
        assert_eq!(run(&mut e, 69, true, 0x1000), (0, 0));
        returns_u32(&mut e, FUNCTION_008A16B0, 9);
        assert_eq!(run(&mut e, 69, true, 0x1000), (0, 1));
    }

    // ---- 00646f70 ---------------------------------------------------------------------------------

    /// A VATS world: a target with a vtable (slot 0x100 "is an actor", slot 0x224 "is a
    /// grenade"), a shooter whose slot 8 answers the stealth query, and a log sink.
    struct Vats {
        e: Engine,
        shooter: Ptr,
        target: Ptr,
        weapon: Ptr,
        log: Ptr,
        lines: Rc<RefCell<Vec<Vec<u32>>>>,
    }

    fn vats() -> Vats {
        let mut e = engine();
        let shooter = object_with_vtable(&mut e, 0);
        let target = object_with_vtable(&mut e, 0x4000);
        let log = object_with_vtable(&mut e, 0x6000);
        e.mem.set_u32(target.addr() + 0xa4, FAKE_TABLE);
        let weapon = object(&mut e, 0x200);
        let lines = Rc::new(RefCell::new(Vec::new()));
        let sink = lines.clone();
        e.register_double(FAKE_SLOT_14, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        returns_u32(&mut e, FAKE_SLOT_100, 1);
        returns_u32(&mut e, FAKE_SLOT_224, 0);
        returns_u32(&mut e, FAKE_SLOT_8, 0);
        returns_u32(&mut e, FUNCTION_00452370, 0);
        returns_u32(&mut e, WEAPON_GET_AMMO_OBJECT, 0);
        returns_u32(&mut e, WEAPON_IS_THROWN_TYPE, 0);
        returns_u32(&mut e, WEAPON_IS_GUN_TYPE_3_TO_9, 1);
        returns_u32(&mut e, WEAPON_GET_TYPE, 3);
        e.set_global(SETTING_VATS_SCREEN_PERCENT_FACTOR, 0.5f32);
        e.set_global(SETTING_VATS_HIT_CHANCE_MULT, 1.0f32);
        e.set_global(SETTING_VATS_RANGE_SPREAD_MAX, 100.0f32);
        e.set_global(SETTING_VATS_SPREAD_MULT, 1.0f32);
        e.set_global(SETTING_VATS_MAX_CHANCE, 95.0f32);
        e.set_global(SETTING_VATS_MELEE_MAX_DISTANCE, 300.0f32);
        e.set_global(SETTING_VATS_STEALTH_MULT, 0.5f32);
        e.set_global(SETTING_VATS_THROWN_WEAPON_RANGE_MULT, 2.0f32);
        e.set_global(SETTING_VATS_GRENADE_RANGE_MULT, 2.0f32);
        e.set_global(SETTING_VATS_GRENADE_RANGE_MIN, 100.0f32);
        e.set_global(SETTING_VATS_GRENADE_SKILL_FACTOR, 0.5f32);
        e.set_global(SETTING_VATS_SKILL_FACTOR, 1.0f32);
        e.set_global(SETTING_VATS_GRENADE_CHANCE_MULT, 1.0f32);
        e.set_global(SETTING_VATS_GRENADE_TARGET_AREA, 8.0f32);
        e.set_global(SETTING_VATS_DESTRUCTIBLE_MULT, 2.0f32);
        e.set_global(SETTING_CRIPPLED_ARM_1H_SPREAD_PENALTY, 0.25f32);
        Vats {
            e,
            shooter,
            target,
            weapon,
            log,
            lines,
        }
    }

    fn vats_call(v: &mut Vats, with_log: bool, distance: f32, bound: f32) -> f32 {
        let log = if with_log { v.log } else { Ptr::NULL };
        v.e.call(
            0x0064_6f70,
            &args![
                v.shooter, v.target, v.weapon, 0u32, 0u32, 0.0f32, 0u32, 0u32, 1.0f32, distance,
                bound, log
            ],
        )
        .f32()
    }

    #[test]
    fn vats_melee_hits_within_the_maximum_distance() {
        let mut v = vats();
        v.e.mem.set_u8(v.weapon.addr() + 0xf4, 1);
        // 100 percent, capped at 95, as a fraction
        assert_eq!(vats_call(&mut v, true, 250.0, 10.0), 0.95);
        assert_eq!(vats_call(&mut v, false, 350.0, 10.0), 0.0);
        let lines = v.lines.borrow();
        // the first call logged "assume 100%": [this, 1, 3, format]
        assert_eq!(lines[0][1..], [1, 3, LOG_MELEE_ASSUME_100]);
    }

    #[test]
    fn vats_melee_out_of_range_logs() {
        let mut v = vats();
        v.e.mem.set_u8(v.weapon.addr() + 0xf4, 1);
        assert_eq!(vats_call(&mut v, true, 350.0, 10.0), 0.0);
        assert_eq!(v.lines.borrow()[0][1..], [1, 3, LOG_MELEE_OUT_OF_RANGE]);
    }

    #[test]
    fn vats_gun_chance_follows_the_spread_and_the_target_area() {
        let mut v = vats();
        v.e.mem.set_u8(v.weapon.addr() + 0xf4, 3);
        returns_f32(&mut v.e, WEAPON_GET_SPREAD, 10.0);
        // tan(x) = x: radius = angle * distance
        v.e.register(FUNCTION_0057DD70, |_, a| f32::from_bits(a[0]).into_ret());
        let distance = 100.0f32;
        let bound = 60.0f32;
        let spread_radians = (10.0f32 as f64 * 1.0 * (std::f32::consts::PI / 180.0) as f64) as f32;
        let radius = (spread_radians as f64 * distance as f64) as f32;
        let area = (radius as f64 * radius as f64 * std::f32::consts::PI as f64) as f32;
        let mut target_area = ((bound as f64 / 3.0) * bound as f64) as f32;
        let diameter = radius as f64 + radius as f64;
        if bound as f64 > diameter {
            target_area = ((diameter / bound as f64) * target_area as f64) as f32;
        }
        let range_spread = (target_area as f64 / area as f64) as f32;
        let range_spread = range_spread.min(100.0);
        // visible share: min(0.5, 1) * 1 + (1 - 0.5) = 1
        let chance = (1.0f64 * 1.0 * range_spread as f64 * 1.0) as f32;
        let expected = (chance.min(95.0) as f64 / 100.0) as f32;
        assert_eq!(vats_call(&mut v, false, distance, bound), expected);
        // with a log sink: the three gun lines are written
        let _ = vats_call(&mut v, true, distance, bound);
        let formats: Vec<u32> = v.lines.borrow().iter().map(|line| line[3]).collect();
        assert_eq!(
            formats,
            vec![LOG_BOUND_SIZE, LOG_SPREAD_FACTOR, LOG_TARGET_AREA]
        );
    }

    #[test]
    fn vats_gun_beyond_its_range_has_no_chance() {
        let mut v = vats();
        v.e.mem.set_u8(v.weapon.addr() + 0xf4, 3);
        v.e.mem.set_u32(v.weapon.addr() + 0x12c, 0x20);
        set_field_f32(&mut v.e, v.weapon, 0x124, 500.0);
        returns_f32(&mut v.e, WEAPON_GET_SPREAD, 1.0);
        v.e.register(FUNCTION_0057DD70, |_, a| f32::from_bits(a[0]).into_ret());
        assert_eq!(vats_call(&mut v, false, 600.0, 10.0), 0.0);
    }

    #[test]
    fn vats_proximity_projectiles_have_no_chance() {
        let mut v = vats();
        v.e.mem.set_u8(v.weapon.addr() + 0xf4, 3);
        returns_u32(&mut v.e, WEAPON_GET_AMMO_OBJECT, 0x5000);
        returns_f32(&mut v.e, WEAPON_GET_PROJECTILE_FLOAT, 5.0);
        assert_eq!(vats_call(&mut v, true, 10.0, 10.0), 0.0);
        assert_eq!(v.lines.borrow()[0][3], LOG_OUT_OF_EXPLOSION_RANGE);
    }

    #[test]
    fn vats_thrown_weapons_use_range_odds_and_skill() {
        let mut v = vats();
        v.e.mem.set_u8(v.weapon.addr() + 0xf4, 11);
        returns_u32(&mut v.e, WEAPON_GET_TYPE, 11);
        returns_u32(&mut v.e, WEAPON_IS_THROWN_TYPE, 1);
        returns_u32(&mut v.e, WEAPON_GET_SKILL, 0x20);
        returns_f32(&mut v.e, ACTOR_VALUE_GET_BY_INDEX, 50.0);
        set_owner_values(&mut v.e, &[(0x1c, 100.0), (0x1b, 100.0)]);
        set_field_f32(&mut v.e, v.weapon, 0x124, 400.0);
        // max range 400 * 2 = 800; distance 700 is inside
        // mod max range = 800, gap = 700, beyond = 600, odds = 1 - 600 / 700
        let gap = 700.0f64;
        let odds = (1.0 - 600.0 / gap) as f32;
        let skill = 0.5f64 * 50.0 / 100.0;
        let skill_calc = (((1.0 - 1.0) + skill) * 1.0) as f32;
        let chance = (((odds as f64 + skill_calc as f64) * 1.0) * 100.0) as f32;
        let expected = (chance.min(95.0) as f64 / 100.0) as f32;
        assert_eq!(vats_call(&mut v, true, 700.0, 10.0), expected);
        let formats: Vec<u32> = v.lines.borrow().iter().map(|line| line[3]).collect();
        assert_eq!(formats, vec![LOG_MOD_MAX_RANGE, LOG_ARM_CONDITION]);
        // beyond the maximum range: nothing
        assert_eq!(vats_call(&mut v, false, 900.0, 10.0), 0.0);
    }

    #[test]
    fn vats_stealth_halves_the_chance_without_the_perk() {
        let mut v = vats();
        v.e.mem.set_u8(v.weapon.addr() + 0xf4, 1);
        // the target (an actor: slot 0x100 true) has the stealth value, the player has no perk
        returns_u32(&mut v.e, FAKE_SLOT_8, 1);
        v.e.register(HANDLE_ENTRY_POINT, |_, _| Ret::default());
        // melee in range: 100 * 0.5 = 50 percent
        assert_eq!(vats_call(&mut v, false, 100.0, 10.0), 0.5);
        // with the perk (value above 0) the bonus is not applied
        v.e.register(HANDLE_ENTRY_POINT, |e, a| {
            e.mem.set_f32(a[2], 1.0);
            Ret::default()
        });
        assert_eq!(vats_call(&mut v, false, 100.0, 10.0), 0.95);
    }

    #[test]
    fn vats_grenade_target_scales_to_a_percentage() {
        let mut v = vats();
        v.e.mem.set_u8(v.weapon.addr() + 0xf4, 3);
        returns_u32(&mut v.e, FAKE_SLOT_224, 1);
        returns_f32(&mut v.e, WEAPON_GET_SPREAD, 10.0);
        v.e.register(FUNCTION_0057DD70, |_, a| f32::from_bits(a[0]).into_ret());
        let chance = vats_call(&mut v, true, 100.0, 60.0);
        // the area is the grenade target area 8; the chance is tiny but positive: raised to 1
        // percent by the minimum, then divided by 100
        assert_eq!(chance, (1.0f64 / 100.0) as f32);
        let lines = v.lines.borrow();
        assert_eq!(lines[0][3], LOG_FORMAT_TWO_TEXTS);
        assert_eq!(lines[0][4..], [EMPTY_TEXT, LOG_GRENADE_TARGET]);
        assert_eq!(lines.last().unwrap()[3], LOG_DESTRUCTIBLE_GRENADE_CHANCE);
    }

    // ---- 00647730 .. 00647920 --------------------------------------------------------------------

    #[test]
    fn fn_00647730_adds_the_mod_value_to_the_byte() {
        let mut e = engine();
        let object = object(&mut e, 0x140);
        e.mem.set_u8(object.addr() + 0x11c, 200);
        returns_f32(&mut e, WEAPON_GET_MOD_EFFECT_VALUE, 70.9);
        assert_eq!(e.call(0x0064_7730, &args![object, false]).u8(), 200);
        assert_eq!(
            e.call(0x0064_7730, &args![object, true]).u8(),
            14,
            "270 wraps"
        );
    }

    #[test]
    fn jam_multipliers_read_the_tables() {
        let mut e = engine();
        for index in 0..10u32 {
            let jam_setting = 0x011c_e000 + 0x100 + 4 * index;
            let reload_setting = 0x011c_e000 + 0x200 + 4 * index;
            e.mem.set_u32(JAM_SETTING_TABLE + 4 * index, jam_setting);
            e.mem
                .set_u32(RELOAD_JAM_SETTING_TABLE + 4 * index, reload_setting);
            e.set_global(jam_setting, (index + 1) as f32 / 10.0);
            e.set_global(reload_setting, 0.5 + index as f32);
        }
        assert_eq!(e.call(0x0064_77b0, &args![0i32]).f32(), 0.1);
        assert_eq!(e.call(0x0064_77b0, &args![9i32]).f32(), 1.0);
        assert_eq!(e.call(0x0064_77b0, &args![10i32]).f32(), 0.0);
        assert_eq!(e.call(0x0064_77b0, &args![-1i32]).f32(), 0.0);
        assert_eq!(e.call(0x0064_77e0, &args![0i32]).f32(), 0.5);
        assert_eq!(e.call(0x0064_77e0, &args![9i32]).f32(), 9.5);
        assert_eq!(e.call(0x0064_77e0, &args![10i32]).f32(), 0.0);
    }

    #[test]
    fn requirement_penalties() {
        let mut e = engine();
        e.set_global(SETTING_WEAP_STRENGTH_REQ_PENALTY, 0.5f32);
        arithmetic_doubles(&mut e);
        // 647810: required 8, strength 5, bonus 1 -> 2 missing points * 0.5
        assert_eq!(e.call(0x0064_7810, &args![8u32, 5u32, 1u32]).f32(), 1.0);
        // strength above the requirement: nothing; a huge shortfall clamps to 10
        assert_eq!(e.call(0x0064_7810, &args![3u32, 5u32, 0u32]).f32(), 0.0);
        assert_eq!(e.call(0x0064_7810, &args![50u32, 5u32, 0u32]).f32(), 5.0);
        // 647870: shortfall 40 - 10 - 0 = 30 -> 3 steps; 100 -> clamps at 10; skill above: 0
        assert_eq!(e.call(0x0064_7870, &args![40u32, 10u32, 0u32]).f32(), 1.5);
        assert_eq!(e.call(0x0064_7870, &args![200u32, 10u32, 0u32]).f32(), 5.0);
        assert_eq!(e.call(0x0064_7870, &args![10u32, 40u32, 0u32]).f32(), 0.0);
    }

    #[test]
    fn explosion_power_falls_off_with_the_squared_ratio() {
        let mut e = engine();
        assert_eq!(e.call(0x0064_7920, &args![10.0f32, 5.0f32]).f32(), 0.75);
        assert_eq!(e.call(0x0064_7920, &args![10.0f32, 10.0f32]).f32(), 0.0);
        assert_eq!(e.call(0x0064_7920, &args![10.0f32, 20.0f32]).f32(), 0.0);
        assert_eq!(e.call(0x0064_7920, &args![f32::NAN, 5.0f32]).f32(), 0.0);
    }

    // ---- registration -------------------------------------------------------------------------------

    #[test]
    fn every_function_of_the_unit_is_registered() {
        let addresses: Vec<u32> = funcs().iter().map(|(address, _)| *address).collect();
        assert_eq!(addresses.len(), 58);
        let mut sorted = addresses.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), 58);
    }
}
