//! `fallout/ai/combat/combatprocedureattackranged.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 009d1b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// A float getter (unnamed in the map; `thiscall`, no arguments): the float at
/// `this + 0x54`, in ST0.
pub fn fn_009d1b50(e: &mut Engine, this: Ptr) -> f32 {
    f32::from_bits(x87_quiet_bits(e.mem.u32(this.addr().wrapping_add(0x54))))
}

layout! {
    /// `CombatProcedureAttackRanged` (Xbox PDB), 0x48 bytes; the offsets are
    /// the ones the PC code uses.
    pub struct CombatProcedureAttackRanged: 0x48 {
        /// `pCombatController` (Xbox PDB, `CombatProcedure`): `CombatController*`.
        0x04 pCombatController: Ptr,
        /// `eState` (Xbox PDB): 0 idle, 1 firing, 2 cooldown, 3 suppressive
        /// fire (the debug texts).
        0x10 eState: u32,
    }
}

/// The embedded members of `CombatProcedureAttackRanged` (Xbox PDB names):
/// `TargetedPoint` (`NiPoint3`) and the `CombatTimer`s.
const TARGETED_POINT: u32 = 0x14;
const EXPLOSIVE_PROJECTILE_BLOCKED_TIMER: u32 = 0x20;
const FIRING_TIMER: u32 = 0x28;
const COOLDOWN_TIMER: u32 = 0x30;
const FIRE_DELAY_TIMER: u32 = 0x38;
const SUPPRESSIVE_FIRE_TIMER: u32 = 0x40;

/// `CombatProcedure::GetAttacker` / `GetTarget` / `SetLookAtTarget` /
/// `ClearLookAtTarget` / `SaveGame` / `LoadGame` (Xbox PDB).
const GET_ATTACKER: u32 = 0x0099_6de0;
const GET_TARGET: u32 = 0x0099_6e00;
const SET_LOOK_AT_TARGET: u32 = 0x0099_6e60;
const CLEAR_LOOK_AT_TARGET: u32 = 0x0099_6ea0;
const BASE_SAVE_GAME: u32 = 0x0099_6ed0;
const BASE_LOAD_GAME: u32 = 0x0099_6ef0;
/// `009caac0` (one word): stores the failure reason and the status 1.
const FAIL_PROCEDURE: u32 = 0x009c_aac0;
/// Controller getters (`__thiscall` on the `CombatController`): `00586150`
/// the combat state (`+0x9c`), `004fb070` the combat group (`+0x80`),
/// `007031c0` the weapon record (`+0xf0`), `00980070` the combat style
/// (`+0xf4`), `0097ae90` the actor.
const GET_COMBAT_STATE: u32 = 0x0058_6150;
const CONTROLLER_GROUP: u32 = 0x004f_b070;
const CONTROLLER_WEAPON: u32 = 0x0070_31c0;
const CONTROLLER_STYLE: u32 = 0x0098_0070;
const CONTROLLER_ACTOR: u32 = 0x0097_ae90;
/// `CombatController::IsSwitchingWeapons` (Xbox PDB).
const IS_SWITCHING_WEAPONS: u32 = 0x0098_1970;
/// Combat state helpers (`__thiscall` on the `CombatState`): `0097fb20` the
/// byte `+0x71`, `0097ee90` and `00998980` time tests (`AL`), `008b3c10` the
/// byte `+0x74`, `008b3c90` the byte `+0x72`, `0097cd10` stores the byte
/// `+0x72`, `00442a80` stores the word `+0x78`,
/// `CombatState::GetLastSeenTargetFiringLocation` (Xbox PDB).
const STATE_BYTE_71: u32 = 0x0097_fb20;
const STATE_TEST_EE90: u32 = 0x0097_ee90;
const STATE_TEST_8980: u32 = 0x0099_8980;
const STATE_BYTE_74: u32 = 0x008b_3c10;
const STATE_BYTE_72: u32 = 0x008b_3c90;
const STATE_SET_BYTE_72: u32 = 0x0097_cd10;
const STATE_SET_WORD_78: u32 = 0x0044_2a80;
const GET_LAST_SEEN_TARGET_FIRING_LOCATION: u32 = 0x0099_abc0;
/// `CombatGroup::IsTarget` (Xbox PDB).
const GROUP_IS_TARGET: u32 = 0x0098_65b0;
/// Weapon record getters (`__thiscall`): `00525a90` the projectile for the
/// actor (one word), `00524b40` and `00645c80` flag tests, `008f7710` the
/// float `+0x170`, `00645c40` / `00645c60` the floats `+0x174` / `+0x178`,
/// `0051f5f0` the attack group byte `+0x11d`.
const WEAPON_PROJECTILE: u32 = 0x0052_5a90;
const WEAPON_TEST_524B40: u32 = 0x0052_4b40;
const WEAPON_TEST_645C80: u32 = 0x0064_5c80;
const WEAPON_FLOAT_170: u32 = 0x008f_7710;
const WEAPON_FLOAT_174: u32 = 0x0064_5c40;
const WEAPON_FLOAT_178: u32 = 0x0064_5c60;
const WEAPON_ATTACK_GROUP: u32 = 0x0051_f5f0;
/// The style's float `+0x48` (`00639aa0`, `ST0`): the targeting field of
/// view in degrees.
const STYLE_FLOAT_48: u32 = 0x0063_9aa0;
/// `004fd3c0` (`__thiscall` on the projectile): the explosion (`+0x84`).
const PROJECTILE_EXPLOSION: u32 = 0x004f_d3c0;
/// `BGSExplosion::GetRadiusBSUnits` (Xbox PDB, `ST0`).
const EXPLOSION_RADIUS: u32 = 0x0047_7900;
/// `00992720` (`__thiscall` on the combat manager at `011f1958`, four
/// words, `AL`).
const MANAGER_EXPLOSION_CHECK: u32 = 0x0099_2720;
const COMBAT_MANAGER: u32 = 0x011f_1958;
/// `CombatDialogueManager::StartDialogue` (Xbox PDB) on the manager at
/// `011f1708`.
const START_DIALOGUE: u32 = 0x0098_3970;
const DIALOGUE_MANAGER: u32 = 0x011f_1708;
/// Timers (`__thiscall` on a `CombatTimer`): `0097fa50` running, `008d7f40`
/// start (a float), `008d7f80` ran out, `00910b10` clear, `00435e00` elapsed
/// and `006b9130` duration (`ST0`); `00435de0` sets a time stamp; the
/// `CombatTimer::SaveGame` / `LoadGame` (Xbox PDB).
const TIMER_RUNNING: u32 = 0x0097_fa50;
const TIMER_START: u32 = 0x008d_7f40;
const TIMER_EXPIRED: u32 = 0x008d_7f80;
const TIMER_CLEAR: u32 = 0x0091_0b10;
const TIMER_ELAPSED: u32 = 0x0043_5e00;
const TIMER_DURATION: u32 = 0x006b_9130;
const TIMESTAMP_SET: u32 = 0x0043_5de0;
const TIMER_SAVE: u32 = 0x009a_5eb0;
const TIMER_LOAD: u32 = 0x009a_5f20;
/// `006a7f50` (`__thiscall` on a time stamp): its time (`ST0`).
const TIMESTAMP_TIME: u32 = 0x006a_7f50;
/// `00403e20` (`__thiscall` on a setting): the address of its value.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// `00404010` (`__cdecl`, two floats, `ST0`): the first when the second
/// is less, else the second.
const FLOAT_MAX: u32 = 0x0040_4010;
/// `RandomFloat` (engine map name of `00476b70`, `__cdecl`, two floats).
const RANDOM_FLOAT: u32 = 0x0047_6b70;
/// Point helpers: `004390c0` / `00439090` (`__thiscall`, one word, `AL`:
/// equal / not equal), `00439ef0` (difference into the out point),
/// `004a7290` (squared length) and `00457990` (length, `ST0`), `006815c0`
/// (a constructor returning `this`).
const POINT_EQUAL: u32 = 0x0043_90c0;
const POINT_NOT_EQUAL: u32 = 0x0043_9090;
const POINT_DIFFERENCE: u32 = 0x0043_9ef0;
const POINT_LENGTH_SQUARED: u32 = 0x004a_7290;
const POINT_LENGTH: u32 = 0x0045_7990;
const POINT_CONSTRUCT: u32 = 0x0068_15c0;
/// `MiddleHighProcess::GetSavedAcquireObject` (engine map name of
/// `008d8520`) and the slots the code calls on its result.
const GET_SAVED_ACQUIRE_OBJECT: u32 = 0x008d_8520;
const SLOT_TEST_3F8: u32 = 0x3f8;
const SLOT_FIRE_KIND: u32 = 0x3e4;
const SLOT_TEST_434: u32 = 0x434;
const SLOT_TEST_438: u32 = 0x438;
const SLOT_SET_43C: u32 = 0x43c;
const SLOT_SET_6E0: u32 = 0x6e0;
/// Actor helpers: `00894900` (`AL`), `Actor::IsPathing`,
/// `Actor::QueueAttack` (Xbox PDB), and the actor vtable slots
/// `+0x1e4` (the animation) and `+0x1f4` (the position).
const ACTOR_IS_ATTACKING: u32 = 0x0089_4900;
const IS_PATHING: u32 = 0x008b_3bd0;
const QUEUE_ATTACK: u32 = 0x0089_35f0;
const SLOT_ANIMATION: u32 = 0x1e4;
const SLOT_POSITION: u32 = 0x1f4;
/// Slots of the hit reference `CalculateProjectileLOS` returns.
const SLOT_HAS_PROCESS: u32 = 0x100;
const SLOT_TEST_22C: u32 = 0x22c;
/// Animation helpers (on the attacker's animation): `004301b0` and
/// `0070f490` (one word), `00495e40` (one word, `AL`), `00497280` (its
/// sequence map), the map's `GetAt` (`0049c390`, two words), a sequence's
/// slot `+0x10` (one word), `0048f7f0` (its group), `TESAnimGroup::GetTime`
/// (Xbox PDB, one word, `ST0`), `TESAnimGroup::GetType` and `005f25d0`
/// (`__cdecl`, one word).
const ANIM_GROUP_OF: u32 = 0x0043_01b0;
const ANIM_STATE_OF: u32 = 0x0070_f490;
const ANIM_DIRECTION: u32 = 0x0049_5e40;
const ANIM_SEQUENCE_MAP: u32 = 0x0049_7280;
const SEQUENCE_MAP_GET: u32 = 0x0049_c390;
const SLOT_SEQUENCE_ANIMATION: u32 = 0x10;
const SEQUENCE_GROUP: u32 = 0x0048_f7f0;
const ANIM_GROUP_TIME: u32 = 0x005f_3780;
const ANIM_GROUP_TYPE: u32 = 0x005f_2440;
const ANIM_GROUP_TEST: u32 = 0x005f_25d0;
/// `CombatUtilities` (Xbox PDB, `__cdecl`).
const IS_TARGET_WITHIN_FIRING_ARC: u32 = 0x009a_6d70;
const COMPUTE_OPTIMAL_WEAPON_RANGES: u32 = 0x009a_9180;
const GET_PROJECTED_POINT_FOR_TARGET: u32 = 0x009a_8bf0;
const GET_BACK_PROJECTED_ATTACKER_FIRING_LOCATION: u32 = 0x009a_82b0;
const CALCULATE_PROJECTILE_LOS: u32 = 0x009a_6e90;
const COMPUTE_PROJECTILE_TIME: u32 = 0x009a_7e00;
const IS_DANGEROUS_EXPLOSION: u32 = 0x009a_9720;
const GET_ANGLE_TO_PROJECTED_POINT: u32 = 0x009a_88e0;
/// `MobileObject::GetActualHeading` and `ClampAngle` (Xbox PDB names),
/// `004b15e0` (`__cdecl`, two floats) and `00408840` (`__cdecl`, one float).
const GET_ACTUAL_HEADING: u32 = 0x0093_6da0;
const CLAMP_ANGLE: u32 = 0x004b_1480;
const ANGLE_DIFFERENCE: u32 = 0x004b_15e0;
const FLOAT_ABS: u32 = 0x0040_8840;
/// `BGSSaveGameBuffer` / `BGSLoadGameBuffer` byte writers and readers.
const SAVE_BYTES: u32 = 0x0086_5e50;
const LOAD_BYTES: u32 = 0x0086_4980;
/// The debug-text object's print slot (`__cdecl` through the vtable).
const SLOT_PRINT: u32 = 0x14;

/// Settings (named by their static initializers).
const SETTING_AUTOMATIC_WEAPON_BURST_FIRE_TIME: u32 = 0x011c_eef0;
const SETTING_AUTOMATIC_WEAPON_BURST_COOLDOWN_TIME: u32 = 0x011c_f21c;
const SETTING_COMBAT_IRON_SIGHTS_DISTANCE: u32 = 0x011c_ebf8;
const SETTING_COMBAT_FIRING_ARC_STATIONARY_TURN_MULT: u32 = 0x011c_fb84;
const SETTING_EXPLOSIVE_PROJECTILE_BLOCKED_WAIT_TIME: u32 = 0x011c_f8ec;

/// Constants of the exe: the `float`s 2, 15, 2 pi (`0101ff50`) and the
/// largest `float` (`01090e90`); the `double`s 0.5, pi / 180 (`01023128`)
/// and 180 / pi (`0102f248`), and the `double` 0 at `01012060` the compares
/// with 0 read.
const TWO_FLOAT: u32 = 0x0101_62c0;
const FIFTEEN_FLOAT: u32 = 0x0101_e580;
const TWO_PI_FLOAT: u32 = 0x0101_ff50;
const LARGEST_FLOAT: u32 = 0x0109_0e90;
const HALF_DOUBLE: u32 = 0x0101_1588;
const ZERO_DOUBLE: u32 = 0x0101_2060;
const RADIANS_PER_DEGREE_DOUBLE: u32 = 0x0102_3128;
const DEGREES_PER_RADIAN_DOUBLE: u32 = 0x0102_f248;
/// The default (zero) `NiPoint3` at `011f426c`.
const DEFAULT_POINT: u32 = 0x011f_426c;

/// Strings.
const TEXT_MISSING_WEAPON: u32 = 0x0109_0a00;
const TEXT_EXPLOSIVE_BLOCKED_TIMER: u32 = 0x0109_0a38;
const TEXT_OUTSIDE_FOV: u32 = 0x0109_0f18;
const TEXT_DISARMED: u32 = 0x0109_0f38;
const TEXT_ARC_MOVING: u32 = 0x0109_0f44;
const TEXT_ARC_STATIONARY: u32 = 0x0109_0f78;
const TEXT_SUPPRESSIVE_FIRE_TIMER: u32 = 0x0109_0fb0;
const TEXT_SUPPRESSION_LOCATION: u32 = 0x0109_0fd4;
const TEXT_SUPPRESSIVE_FIRE: u32 = 0x0109_1000;
const TEXT_COOLDOWN_TIMER: u32 = 0x0109_1014;
const TEXT_COOLDOWN: u32 = 0x0109_1030;
const TEXT_FIRING_DELAY_TIMER: u32 = 0x0109_103c;
const TEXT_TARGET_LOCATION: u32 = 0x0109_105c;
const TEXT_FIRING_TIMER: u32 = 0x0109_1084;
const TEXT_FIRING: u32 = 0x0109_10a0;
const TEXT_IDLE: u32 = 0x0101_c42c;

fn controller_of(e: &Engine, this: Ptr<CombatProcedureAttackRanged>) -> u32 {
    e.get(this, CombatProcedureAttackRanged::pCombatController)
        .addr()
}

fn combat_state_of(e: &mut Engine, this: Ptr<CombatProcedureAttackRanged>) -> u32 {
    let controller = controller_of(e, this);
    e.call(GET_COMBAT_STATE, &args![controller]).u32()
}

/// The bit pattern of the `float` a setting holds.
fn setting_bits(e: &mut Engine, setting: u32) -> u32 {
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    e.mem.u32(value)
}

fn copy_point(e: &mut Engine, from: u32, to: u32) {
    for i in 0..3 {
        let word = e.mem.u32(from + i * 4);
        e.mem.set_u32(to + i * 4, word);
    }
}

fn point_words(e: &Engine, at: u32) -> [u32; 3] {
    [e.mem.u32(at), e.mem.u32(at + 4), e.mem.u32(at + 8)]
}

// Translated from 009d09a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackRanged::Shutdown` (Xbox PDB): clears the look-at
/// target, the controller's byte `+0x142`, stops firing and sets the
/// controller's byte `+0xc6` to 0xff.
pub fn shutdown(e: &mut Engine, this: Ptr<CombatProcedureAttackRanged>) {
    let _attacker = e.call(GET_ATTACKER, &args![this]).u32();
    e.call(CLEAR_LOOK_AT_TARGET, &args![this]);
    let controller = controller_of(e, this);
    fn_009d0a10(e, controller, 0);
    stop_firing(e, this);
    let controller = controller_of(e, this);
    fn_009d09f0(e, controller, 0xff);
}

// Translated from 009d09f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte at `+0xc6` (`__thiscall`, one word; on the controller).
pub fn fn_009d09f0(e: &mut Engine, this: u32, value: u8) {
    e.mem.set_u8(this + 0xc6, value);
}

// Translated from 009d0a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte at `+0x142` (`__thiscall`, one word; on the controller).
pub fn fn_009d0a10(e: &mut Engine, this: u32, value: u8) {
    e.mem.set_u8(this + 0x142, value);
}

/// The kind the compiler's switch on `direction - 0x33` gives (byte map
/// `009d1acc`, jump table `009d1aac`), `None` for the default case.
fn direction_attack(direction: i8) -> Option<u32> {
    match (i32::from(direction)).wrapping_sub(0x33) as u32 {
        0 => Some(0x26),
        1 => Some(0x2c),
        2 => Some(0x32),
        3 => Some(0x38),
        4 => Some(0x3e),
        5 => Some(0x44),
        0x39 => Some(0x1a),
        _ => None,
    }
}

// Translated from 009d0a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackRanged::Update` (Xbox PDB): fails without a
/// weapon, an animation or a projectile, or with the target outside the
/// style's field of view. It computes the targeted point; with the
/// attacker able to fire (its acquire object's slot `+0x3f8`, or
/// attacking, and not switching weapons) it checks the target's distance
/// against the optimal weapon ranges (too far: not firing; too close:
/// firing anyway). With the combat state's byte `+0x71` and its time test
/// it runs the firing / cooldown cycle of automatic weapons (bursts of
/// `fAutomaticWeaponBurstFireTime` plus the burst animation's length,
/// cooldowns of `fAutomaticWeaponBurstCooldownTime`); otherwise the state
/// machine of the switch table `009d1a9c` (suppressive fire for 2
/// seconds). It sets the controller's iron-sights byte (`+0xc6`) by
/// `fCombatIronSightsDistance` and the firing-arc byte (`+0x142`), looks at
/// the targeted point, and once aimed and the fire delay ran out it checks
/// the projectile's line of sight (a blocked shot ends firing unless the
/// blocker is the target, an enemy, or within the explosion radius) and
/// the explosion's safety (`00992720`), then queues the attack of the
/// animation's direction, starts the fire delay and announces dangerous
/// explosions.
///
/// Locals follow the disassembly's frame (offsets below `EBP`).
pub fn update(e: &mut Engine, this: Ptr<CombatProcedureAttackRanged>) {
    const FRAME: u32 = 0x150;
    let t = this.addr();
    e.with_stack(FRAME, |e, base| {
        let ebp = base.addr() + FRAME;
        let targeted = ebp - 0x34;
        let attacker = e.call(GET_ATTACKER, &args![t]).u32();
        let target = e.call(GET_TARGET, &args![t]).u32();
        let combat_state = combat_state_of(e, this);
        let controller = controller_of(e, this);
        let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
        let controller = controller_of(e, this);
        let weapon = e.call(CONTROLLER_WEAPON, &args![controller]).u32();
        let animation = e.vcall(attacker, SLOT_ANIMATION, &args![]).u32();
        if weapon == 0
            || animation == 0
            || e.call(WEAPON_PROJECTILE, &args![weapon, attacker]).u32() == 0
        {
            e.call(FAIL_PROCEDURE, &args![t, TEXT_MISSING_WEAPON]);
            return;
        }
        let controller = controller_of(e, this);
        let style = e.call(CONTROLLER_STYLE, &args![controller]).u32();
        let field_of_view = x87_store32(e.call(STYLE_FLOAT_48, &args![style]).f64());
        if x87_load32(field_of_view) > e.global::<f64>(ZERO_DOUBLE) && target != 0 {
            let two_pi = x87_quiet_bits(e.global::<u32>(TWO_PI_FLOAT));
            let radians = x87_store32(x87_mul(
                x87_load32(field_of_view),
                e.global::<f64>(RADIANS_PER_DEGREE_DOUBLE),
            ));
            let position = e.vcall(target, SLOT_POSITION, &args![]).u32();
            if !e
                .call(
                    IS_TARGET_WITHIN_FIRING_ARC,
                    &args![attacker, position, radians, two_pi],
                )
                .bool()
            {
                e.call(FAIL_PROCEDURE, &args![t, TEXT_OUTSIDE_FOV]);
                return;
            }
        }
        let projectile = e.call(WEAPON_PROJECTILE, &args![weapon, attacker]).u32();
        let automatic = e.call(WEAPON_TEST_524B40, &args![weapon]).bool();
        let bursts = automatic || e.call(WEAPON_TEST_645C80, &args![weapon]).bool();
        let mut too_close = false;
        calculate_targeted_point(e, this, Ptr::new(targeted), 0);
        let object = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![attacker]).u32();
        let mut can_fire = e.vcall(object, SLOT_TEST_3F8, &args![]).bool()
            || e.call(ACTOR_IS_ATTACKING, &args![attacker]).bool();
        let controller = controller_of(e, this);
        if e.call(IS_SWITCHING_WEAPONS, &args![controller]).bool() {
            can_fire = false;
        }
        if e.get(this, CombatProcedureAttackRanged::eState) != 0 && !can_fire {
            stop_firing(e, this);
            e.set(this, CombatProcedureAttackRanged::eState, 0);
        }
        if can_fire {
            let minimum = ebp - 0x84;
            let optimal = ebp - 0x80;
            let maximum = ebp - 0x7c;
            let point = ebp - 0x90;
            let difference = ebp - 0x74;
            for local in [minimum, optimal, maximum] {
                e.mem.set_u32(local, 0.0f32.to_bits());
            }
            let controller = controller_of(e, this);
            e.call(
                COMPUTE_OPTIMAL_WEAPON_RANGES,
                &args![controller, weapon, minimum, optimal, maximum],
            );
            copy_point(e, targeted, point);
            if target != 0 {
                let position = e.vcall(target, SLOT_POSITION, &args![]).u32();
                let z = x87_quiet_bits(e.mem.u32(position + 8));
                e.mem.set_u32(point + 8, z);
            }
            let position = e.vcall(attacker, SLOT_POSITION, &args![]).u32();
            e.call(POINT_DIFFERENCE, &args![point, difference, position]);
            let squared = x87_store32(e.call(POINT_LENGTH_SQUARED, &args![difference]).f64());
            let far = e.mem.u32(maximum);
            if x87_load32(squared) > x87_mul(x87_load32(far), x87_m32(far)) {
                can_fire = false;
            }
            let near = e.mem.u32(minimum);
            if x87_load32(squared) < x87_mul(x87_load32(near), x87_m32(near)) {
                too_close = true;
            }
        }
        if e.call(STATE_BYTE_71, &args![combat_state]).bool() {
            copy_point(e, targeted, t + TARGETED_POINT);
        }
        if e.call(STATE_BYTE_71, &args![combat_state]).bool()
            && e.call(STATE_TEST_EE90, &args![combat_state]).bool()
            && can_fire
        {
            match e.get(this, CombatProcedureAttackRanged::eState) {
                1 => {
                    if bursts {
                        if !e.call(TIMER_RUNNING, &args![t + FIRING_TIMER]).bool() {
                            let stamp = fn_009d1b70(e, combat_state, ebp - 0xfc);
                            let last = e.call(TIMESTAMP_TIME, &args![stamp]).f64();
                            let never = -x87_load32(e.global::<u32>(LARGEST_FLOAT));
                            // FUCOMPP; TEST AH,0x44; JNP: only when the time
                            // differs from -FLT_MAX (or is unordered).
                            if never != last {
                                let mut burst = x87_quiet_bits(setting_bits(
                                    e,
                                    SETTING_AUTOMATIC_WEAPON_BURST_FIRE_TIME,
                                ));
                                let group = e.call(ANIM_GROUP_OF, &args![animation, 4u32]).u16();
                                let kind = e.call(ANIM_GROUP_TYPE, &args![u32::from(group)]).u32();
                                if kind == 0x50
                                    || e.call(ANIM_GROUP_TYPE, &args![u32::from(group)]).u32()
                                        == 0x56
                                {
                                    let out = ebp - 0x9c;
                                    let map = e.call(ANIM_SEQUENCE_MAP, &args![animation]).u32();
                                    if e.call(SEQUENCE_MAP_GET, &args![map, u32::from(group), out])
                                        .bool()
                                    {
                                        let sequence = e.mem.u32(out);
                                        let inner = e
                                            .vcall(
                                                sequence,
                                                SLOT_SEQUENCE_ANIMATION,
                                                &args![u32::MAX],
                                            )
                                            .u32();
                                        let anim_group =
                                            e.call(SEQUENCE_GROUP, &args![inner]).u32();
                                        let start = x87_store32(
                                            e.call(ANIM_GROUP_TIME, &args![anim_group, 0u32]).f64(),
                                        );
                                        let end = x87_store32(
                                            e.call(ANIM_GROUP_TIME, &args![anim_group, 1u32]).f64(),
                                        );
                                        burst = x87_store32(x87_add(
                                            x87_sub(x87_load32(end), x87_m32(start)),
                                            x87_m32(burst),
                                        ));
                                    }
                                }
                                e.call(TIMER_START, &args![t + FIRING_TIMER, burst]);
                            }
                        }
                        if e.call(TIMER_RUNNING, &args![t + FIRING_TIMER]).bool()
                            && e.call(TIMER_EXPIRED, &args![t + FIRING_TIMER]).bool()
                        {
                            stop_firing(e, this);
                            let cooldown =
                                setting_bits(e, SETTING_AUTOMATIC_WEAPON_BURST_COOLDOWN_TIME);
                            e.call(
                                TIMER_START,
                                &args![t + COOLDOWN_TIMER, x87_quiet_bits(cooldown)],
                            );
                            e.set(this, CombatProcedureAttackRanged::eState, 2);
                        }
                    }
                }
                2 => {
                    if e.call(TIMER_EXPIRED, &args![t + COOLDOWN_TIMER]).bool() {
                        fn_009d1b90(e, combat_state);
                        fn_009d1bc0(e, this, automatic);
                        e.set(this, CombatProcedureAttackRanged::eState, 1);
                    }
                }
                _ => {
                    e.call(TIMER_CLEAR, &args![t + SUPPRESSIVE_FIRE_TIMER]);
                    fn_009d1bc0(e, this, automatic);
                    e.set(this, CombatProcedureAttackRanged::eState, 1);
                }
            }
        } else {
            // The compiler's switch on the state (table `009d1a9c`).
            match e.get(this, CombatProcedureAttackRanged::eState) {
                0 => {
                    if can_fire
                        && !e
                            .call(TIMER_EXPIRED, &args![t + SUPPRESSIVE_FIRE_TIMER])
                            .bool()
                    {
                        fn_009d1bc0(e, this, automatic);
                        e.set(this, CombatProcedureAttackRanged::eState, 3);
                    }
                }
                1 => {
                    fn_009d1bc0(e, this, automatic);
                    e.set(this, CombatProcedureAttackRanged::eState, 3);
                    let two = x87_quiet_bits(e.global::<u32>(TWO_FLOAT));
                    e.call(TIMER_START, &args![t + SUPPRESSIVE_FIRE_TIMER, two]);
                }
                2 => {
                    if e.call(TIMER_EXPIRED, &args![t + COOLDOWN_TIMER]).bool() {
                        fn_009d1bc0(e, this, automatic);
                        e.set(this, CombatProcedureAttackRanged::eState, 3);
                        let two = x87_quiet_bits(e.global::<u32>(TWO_FLOAT));
                        e.call(TIMER_START, &args![t + SUPPRESSIVE_FIRE_TIMER, two]);
                    }
                }
                3 => {
                    if e.call(TIMER_EXPIRED, &args![t + SUPPRESSIVE_FIRE_TIMER])
                        .bool()
                    {
                        stop_firing(e, this);
                        e.set(this, CombatProcedureAttackRanged::eState, 0);
                    }
                }
                _ => {}
            }
        }
        if target != 0 && e.call(STATE_TEST_EE90, &args![combat_state]).bool() {
            let manager = e.global::<u32>(DIALOGUE_MANAGER);
            let controller = controller_of(e, this);
            e.call(
                START_DIALOGUE,
                &args![manager, controller, 2u32, 0u32, 0u32],
            );
        }
        let mut iron_sights = false;
        if target != 0 {
            let distance = setting_bits(e, SETTING_COMBAT_IRON_SIGHTS_DISTANCE);
            let scale = e.call(WEAPON_FLOAT_170, &args![weapon]).f64();
            let limit = x87_store32(x87_mul(scale, x87_m32(distance)));
            let difference = ebp - 0xb4;
            let target_position = e.vcall(target, SLOT_POSITION, &args![]).u32();
            let attacker_position = e.vcall(attacker, SLOT_POSITION, &args![]).u32();
            e.call(
                POINT_DIFFERENCE,
                &args![attacker_position, difference, target_position],
            );
            let squared = e.call(POINT_LENGTH_SQUARED, &args![difference]).f64();
            // FLD limit; FMUL limit; FCOMPP; TEST AH,5; JP: within when the
            // squared limit is less.
            if x87_mul(x87_load32(limit), x87_m32(limit)) < squared {
                iron_sights = true;
            }
        }
        let controller = controller_of(e, this);
        fn_009d09f0(e, controller, u8::from(iron_sights));
        if f64::from(fn_009d1b10(e, weapon)) > e.global::<f64>(ZERO_DOUBLE) {
            let mut arc = x87_store32(x87_mul(
                f64::from(fn_009d1b10(e, weapon)),
                e.global::<f64>(RADIANS_PER_DEGREE_DOUBLE),
            ));
            if !e.call(IS_PATHING, &args![attacker]).bool() {
                let multiplier = setting_bits(e, SETTING_COMBAT_FIRING_ARC_STATIONARY_TURN_MULT);
                arc = x87_store32(x87_mul(x87_load32(arc), x87_m32(multiplier)));
            }
            let within = e
                .call(POINT_NOT_EQUAL, &args![t + TARGETED_POINT, DEFAULT_POINT])
                .bool()
                && {
                    let largest = x87_quiet_bits(e.global::<u32>(LARGEST_FLOAT));
                    e.call(
                        IS_TARGET_WITHIN_FIRING_ARC,
                        &args![attacker, t + TARGETED_POINT, arc, largest],
                    )
                    .bool()
                };
            let controller = controller_of(e, this);
            fn_009d0a10(e, controller, if within { 4 } else { 0 });
        }
        if e.get(this, CombatProcedureAttackRanged::eState) == 0 {
            let look = target != 0
                && !e.call(STATE_BYTE_74, &args![combat_state]).bool()
                && (e.call(STATE_BYTE_71, &args![combat_state]).bool()
                    || (e.call(STATE_TEST_8980, &args![combat_state]).bool()
                        && e.call(STATE_TEST_EE90, &args![combat_state]).bool()));
            if look {
                if e.call(POINT_NOT_EQUAL, &args![t + TARGETED_POINT, DEFAULT_POINT])
                    .bool()
                    || e.call(
                        GET_LAST_SEEN_TARGET_FIRING_LOCATION,
                        &args![combat_state, t + TARGETED_POINT],
                    )
                    .bool()
                {
                    e.call(SET_LOOK_AT_TARGET, &args![t, t + TARGETED_POINT, 2u32]);
                } else {
                    e.call(CLEAR_LOOK_AT_TARGET, &args![t]);
                }
            } else {
                e.call(CLEAR_LOOK_AT_TARGET, &args![t]);
            }
            return;
        }
        if e.call(POINT_EQUAL, &args![t + TARGETED_POINT, DEFAULT_POINT])
            .bool()
        {
            copy_point(e, targeted, t + TARGETED_POINT);
        }
        if e.call(POINT_NOT_EQUAL, &args![t + TARGETED_POINT, DEFAULT_POINT])
            .bool()
        {
            e.call(SET_LOOK_AT_TARGET, &args![t, t + TARGETED_POINT, 4u32]);
        }
        if e.get(this, CombatProcedureAttackRanged::eState) == 2 {
            return;
        }
        if !aim_at_target(e, this) && !too_close {
            return;
        }
        if !e.call(TIMER_EXPIRED, &args![t + FIRE_DELAY_TIMER]).bool() {
            return;
        }
        let mut fire = false;
        let mut ready = true;
        if automatic {
            let mut animation_ok = true;
            let object = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![attacker]).u32();
            ready = e.vcall(object, SLOT_TEST_438, &args![attacker]).bool();
            let group = e.call(ANIM_GROUP_OF, &args![animation, 4u32]).u16();
            if e.call(ANIM_GROUP_TEST, &args![u32::from(group)]).bool() {
                let state = e.call(ANIM_STATE_OF, &args![animation, 4u32]).i32();
                if !(1..2).contains(&state) {
                    animation_ok = false;
                }
            }
            if animation_ok && ready {
                fire = true;
            }
        } else {
            let object = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![attacker]).u32();
            let kind = e.vcall(object, SLOT_FIRE_KIND, &args![]).i32();
            if kind == -1
                || (kind == 4 && e.call(ANIM_STATE_OF, &args![animation, 4u32]).u32() == 3)
            {
                fire = true;
            }
        }
        if !fire {
            return;
        }
        let hit_point = ebp - 0x14;
        let hit_reference = ebp - 0x3c;
        let fraction = ebp - 0x64;
        let firing = ebp - 0x54;
        e.call(POINT_CONSTRUCT, &args![hit_point]);
        e.mem.set_u32(hit_reference, 0);
        e.mem.set_u32(ebp - 0x38, 0);
        e.mem.set_u32(fraction, 0.0f32.to_bits());
        e.call(
            GET_BACK_PROJECTED_ATTACKER_FIRING_LOCATION,
            &args![firing, attacker],
        );
        let [fx, fy, fz] = point_words(e, firing);
        let [tx, ty, tz] = point_words(e, targeted);
        let hit = e
            .call(
                CALCULATE_PROJECTILE_LOS,
                &args![
                    attacker,
                    projectile,
                    0.0f32.to_bits(),
                    fx,
                    fy,
                    fz,
                    tx,
                    ty,
                    tz,
                    hit_point,
                    hit_reference,
                    fraction
                ],
            )
            .bool();
        if hit {
            let mut clear = false;
            let blocker = e.mem.u32(hit_reference);
            match e.get(this, CombatProcedureAttackRanged::eState) {
                1 => {
                    if blocker != 0 && e.vcall(blocker, SLOT_HAS_PROCESS, &args![]).bool() {
                        e.mem.set_u32(ebp - 0x38, blocker);
                        let target = e.call(GET_TARGET, &args![t]).u32();
                        if blocker == target
                            || e.call(GROUP_IS_TARGET, &args![group, blocker]).bool()
                        {
                            clear = true;
                        }
                    }
                    if !clear {
                        let explosion = e.call(PROJECTILE_EXPLOSION, &args![projectile]).u32();
                        if explosion != 0 {
                            let difference = ebp - 0xd8;
                            e.call(POINT_DIFFERENCE, &args![hit_point, difference, targeted]);
                            let distance =
                                x87_store32(e.call(POINT_LENGTH, &args![difference]).f64());
                            let radius = e.call(EXPLOSION_RADIUS, &args![explosion]).f64();
                            // FLD distance; FCOMPP; TEST AH,0x41; JP: clear
                            // when less or equal.
                            if x87_load32(distance) <= radius {
                                clear = true;
                            }
                        }
                    }
                }
                3 => {
                    let blocker_has_process =
                        blocker != 0 && e.vcall(blocker, SLOT_HAS_PROCESS, &args![]).bool();
                    if !blocker_has_process {
                        if x87_load32(e.mem.u32(fraction)) > e.global::<f64>(HALF_DOUBLE) {
                            clear = true;
                        }
                    } else if blocker != 0
                        && e.vcall(blocker, SLOT_HAS_PROCESS, &args![]).bool()
                        && !e.vcall(blocker, SLOT_TEST_22C, &args![0u32]).bool()
                        && e.call(GROUP_IS_TARGET, &args![group, blocker]).bool()
                    {
                        clear = true;
                    }
                    if !clear {
                        e.set(this, CombatProcedureAttackRanged::eState, 0);
                        stop_firing(e, this);
                        e.call(TIMER_CLEAR, &args![t + SUPPRESSIVE_FIRE_TIMER]);
                        return;
                    }
                }
                _ => {}
            }
            if clear {
                let state = combat_state_of(e, this);
                e.call(STATE_SET_BYTE_72, &args![state, 0u32]);
            } else {
                let state = combat_state_of(e, this);
                if !e.call(STATE_BYTE_72, &args![state]).bool() {
                    let state = combat_state_of(e, this);
                    e.call(STATE_SET_BYTE_72, &args![state, 1u32]);
                }
                return;
            }
        }
        let explosion = e.call(PROJECTILE_EXPLOSION, &args![projectile]).u32();
        if explosion != 0 {
            let aim = ebp - 0xe8;
            if hit {
                copy_point(e, hit_point, aim);
            } else {
                copy_point(e, t + TARGETED_POINT, aim);
            }
            let [fx, fy, fz] = point_words(e, firing);
            let [ax, ay, az] = point_words(e, aim);
            let time = x87_store32(
                e.call(
                    COMPUTE_PROJECTILE_TIME,
                    &args![projectile, fx, fy, fz, ax, ay, az, 0u32, 0.0f32.to_bits()],
                )
                .f64(),
            );
            let manager = e.global::<u32>(COMBAT_MANAGER);
            if e.call(
                MANAGER_EXPLOSION_CHECK,
                &args![manager, attacker, explosion, aim, time],
            )
            .bool()
            {
                let state = combat_state_of(e, this);
                e.call(STATE_SET_WORD_78, &args![state, 0u32]);
                e.call(TIMER_CLEAR, &args![t + EXPLOSIVE_PROJECTILE_BLOCKED_TIMER]);
            } else {
                fire = false;
                if !e
                    .call(
                        TIMER_RUNNING,
                        &args![t + EXPLOSIVE_PROJECTILE_BLOCKED_TIMER],
                    )
                    .bool()
                {
                    let wait = setting_bits(e, SETTING_EXPLOSIVE_PROJECTILE_BLOCKED_WAIT_TIME);
                    e.call(
                        TIMER_START,
                        &args![t + EXPLOSIVE_PROJECTILE_BLOCKED_TIMER, x87_quiet_bits(wait)],
                    );
                } else if e
                    .call(
                        TIMER_EXPIRED,
                        &args![t + EXPLOSIVE_PROJECTILE_BLOCKED_TIMER],
                    )
                    .bool()
                {
                    let state = combat_state_of(e, this);
                    e.call(STATE_SET_WORD_78, &args![state, explosion]);
                }
            }
        }
        if !fire {
            return;
        }
        let direction = e.call(ANIM_DIRECTION, &args![animation, 0u32]).u8() as i8;
        let attack = match direction_attack(direction) {
            Some(attack) => attack,
            None => {
                if weapon != 0 && e.call(WEAPON_ATTACK_GROUP, &args![weapon]).u32() != 0xff {
                    e.call(WEAPON_ATTACK_GROUP, &args![weapon]).u32()
                } else {
                    0x20
                }
            }
        };
        let object = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![attacker]).u32();
        e.vcall(object, SLOT_SET_43C, &args![u32::from(ready)]);
        if e.call(QUEUE_ATTACK, &args![attacker, attack]).bool() && !automatic {
            let low_scale = e.call(WEAPON_FLOAT_174, &args![weapon]).f64();
            let controller = controller_of(e, this);
            let style = e.call(CONTROLLER_STYLE, &args![controller]).u32();
            let low = x87_store32(x87_mul(f64::from(fn_009d1b30(e, style)), low_scale));
            let high_scale = e.call(WEAPON_FLOAT_178, &args![weapon]).f64();
            let controller = controller_of(e, this);
            let style = e.call(CONTROLLER_STYLE, &args![controller]).u32();
            let high = x87_store32(x87_mul(
                f64::from(fn_009d1b50(e, Ptr::new(style))),
                high_scale,
            ));
            let delay = e.call(RANDOM_FLOAT, &args![low, high]).f64();
            e.call(
                TIMER_START,
                &args![t + FIRE_DELAY_TIMER, x87_store32(delay)],
            );
        }
        if explosion != 0 && e.call(IS_DANGEROUS_EXPLOSION, &args![explosion]).bool() {
            let manager = e.global::<u32>(DIALOGUE_MANAGER);
            let controller = controller_of(e, this);
            e.call(
                START_DIALOGUE,
                &args![manager, controller, 2u32, 4u32, 1u32],
            );
        }
    });
}

// Translated from 009d1b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at `+0x158` (`__thiscall`, `ST0`; on the weapon record, the
/// firing arc in degrees `Update` and `AimAtTarget` use).
pub fn fn_009d1b10(e: &mut Engine, this: u32) -> f32 {
    f32::from_bits(x87_quiet_bits(e.mem.u32(this.wrapping_add(0x158))))
}

// Translated from 009d1b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at `+0x50` (`__thiscall`, `ST0`; on the combat style).
pub fn fn_009d1b30(e: &mut Engine, this: u32) -> f32 {
    f32::from_bits(x87_quiet_bits(e.mem.u32(this.wrapping_add(0x50))))
}

// Translated from 009d1b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the word at `+0xa4` (a time stamp of the combat state) to `out`
/// and returns `out` (one word).
pub fn fn_009d1b70(e: &mut Engine, this: u32, out: u32) -> u32 {
    let word = e.mem.u32(this + 0xa4);
    e.mem.set_u32(out, word);
    out
}

// Translated from 009d1b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the time stamp at `+0xa4` to `-FLT_MAX` (through a local time stamp
/// and `00435de0`).
pub fn fn_009d1b90(e: &mut Engine, this: u32) {
    let never = x87_store32(-x87_load32(e.global::<u32>(LARGEST_FLOAT)));
    e.with_stack(4, |e, stamp| {
        e.call(TIMESTAMP_SET, &args![stamp, never]);
        let value = e.mem.u32(stamp.addr());
        e.mem.set_u32(this + 0xa4, value);
    });
}

// Translated from 009d1bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts firing (unnamed in the map; one word): the actor's acquire object
/// gets `automatic` (slot `+0x6e0`) and 1 (slot `+0x434`); the firing timer
/// is cleared.
pub fn fn_009d1bc0(e: &mut Engine, this: Ptr<CombatProcedureAttackRanged>, automatic: bool) {
    let controller = controller_of(e, this);
    let actor = e.call(CONTROLLER_ACTOR, &args![controller]).u32();
    let object = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![actor]).u32();
    e.vcall(object, SLOT_SET_6E0, &args![u32::from(automatic)]);
    let object = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![actor]).u32();
    e.vcall(object, SLOT_TEST_434, &args![1u32]);
    e.call(TIMER_CLEAR, &args![this.addr() + FIRING_TIMER]);
}

// Translated from 009d1c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackRanged::StopFiring` (Xbox PDB): the acquire
/// object's slots `+0x6e0` and `+0x434` with 0; the firing timer is
/// cleared.
pub fn stop_firing(e: &mut Engine, this: Ptr<CombatProcedureAttackRanged>) {
    let controller = controller_of(e, this);
    let actor = e.call(CONTROLLER_ACTOR, &args![controller]).u32();
    let object = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![actor]).u32();
    e.vcall(object, SLOT_SET_6E0, &args![0u32]);
    let object = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![actor]).u32();
    e.vcall(object, SLOT_TEST_434, &args![0u32]);
    e.call(TIMER_CLEAR, &args![this.addr() + FIRING_TIMER]);
}

// Translated from 009d1c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackRanged::CalculateTargetedPoint` (Xbox PDB): the
/// point to aim at, into `out` (returned): the default point without a
/// target, else `CombatUtilities::GetProjectedPointForTarget` for the
/// weapon's projectile, passing `argument` through.
pub fn calculate_targeted_point(
    e: &mut Engine,
    this: Ptr<CombatProcedureAttackRanged>,
    out: Ptr,
    argument: u32,
) -> Ptr {
    let t = this.addr();
    let target = e.call(GET_TARGET, &args![t]).u32();
    if target == 0 {
        copy_point(e, DEFAULT_POINT, out.addr());
        return out;
    }
    let controller = controller_of(e, this);
    let weapon = e.call(CONTROLLER_WEAPON, &args![controller]).u32();
    let attacker = e.call(GET_ATTACKER, &args![t]).u32();
    let projectile = e.call(WEAPON_PROJECTILE, &args![weapon, attacker]).u32();
    let attacker = e.call(GET_ATTACKER, &args![t]).u32();
    e.with_stack(0xc, |e, point| {
        e.call(
            GET_PROJECTED_POINT_FOR_TARGET,
            &args![
                point,
                attacker,
                target,
                argument,
                projectile,
                0.0f32.to_bits(),
                0u32
            ],
        );
        copy_point(e, point.addr(), out.addr());
    });
    out
}

// Translated from 009d1d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackRanged::GetTargetedPoint` (Xbox PDB): while firing
/// or cooling down (states 1, 2) the targeted point is computed again and
/// kept; in suppressive fire (3) the kept point is used; it is copied to
/// `out` and the result is true. Idle (or another state): false.
pub fn get_targeted_point(
    e: &mut Engine,
    this: Ptr<CombatProcedureAttackRanged>,
    out: u32,
    argument: u32,
) -> bool {
    let t = this.addr();
    let state = e.get(this, CombatProcedureAttackRanged::eState) as i32;
    if state <= 0 {
        return false;
    }
    if state <= 2 {
        e.with_stack(0xc, |e, point| {
            let computed = calculate_targeted_point(e, this, point, argument);
            copy_point(e, computed.addr(), t + TARGETED_POINT);
        });
        copy_point(e, t + TARGETED_POINT, out);
        return true;
    }
    if state == 3 {
        copy_point(e, t + TARGETED_POINT, out);
        return true;
    }
    false
}

// Translated from 009d1dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackRanged::Disarmed` (Xbox PDB): fails with
/// "Disarmed". The stack word is not read.
pub fn disarmed(e: &mut Engine, this: Ptr<CombatProcedureAttackRanged>, _unused_1: u32) {
    e.call(FAIL_PROCEDURE, &args![this, TEXT_DISARMED]);
}

// Translated from 009d1de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackRanged::AimAtTarget` (Xbox PDB): false while the
/// targeted point is the default point; otherwise whether it is within the
/// weapon's firing arc (at least 15 degrees, in radians, the same for both
/// limits).
pub fn aim_at_target(e: &mut Engine, this: Ptr<CombatProcedureAttackRanged>) -> bool {
    let t = this.addr();
    if e.call(POINT_EQUAL, &args![t + TARGETED_POINT, DEFAULT_POINT])
        .bool()
    {
        return false;
    }
    let attacker = e.call(GET_ATTACKER, &args![t]).u32();
    let controller = controller_of(e, this);
    let weapon = e.call(CONTROLLER_WEAPON, &args![controller]).u32();
    let fifteen = x87_quiet_bits(e.global::<u32>(FIFTEEN_FLOAT));
    let arc = fn_009d1b10(e, weapon).to_bits();
    let degrees = e.call(FLOAT_MAX, &args![arc, fifteen]).f64();
    let radians = x87_store32(x87_mul(degrees, e.global::<f64>(RADIANS_PER_DEGREE_DOUBLE)));
    e.call(
        IS_TARGET_WITHIN_FIRING_ARC,
        &args![attacker, t + TARGETED_POINT, radians, radians],
    )
    .bool()
}

/// A timer line: the format with the timer's elapsed time and duration as
/// doubles (the duration is asked for first, as the exe pushes it first).
fn print_timer(e: &mut Engine, text: u32, first: u32, second: u32, format: u32, timer: u32) {
    let duration = e.call(TIMER_DURATION, &args![timer]).f64();
    let elapsed = e.call(TIMER_ELAPSED, &args![timer]).f64();
    e.vcall(
        text,
        SLOT_PRINT,
        &args![first, second, format, elapsed, duration],
    );
}

/// A point line: the format with the three floats of the point as doubles.
fn print_point(e: &mut Engine, text: u32, first: u32, second: u32, format: u32, point: u32) {
    let z = x87_load32(e.mem.u32(point + 8));
    let y = x87_load32(e.mem.u32(point + 4));
    let x = x87_load32(e.mem.u32(point));
    e.vcall(text, SLOT_PRINT, &args![first, second, format, x, y, z]);
}

// Translated from 009d1e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackRanged::DisplayDebugText` (Xbox PDB): the state
/// (switch table `009d22f8`) with its timers and the targeted point, the
/// explosive-blocked timer while it runs, and the weapon's firing arc
/// against the angle to the targeted point (degrees, "Moving" while
/// pathing, else times `fCombatFiringArcStationaryTurnMult`).
pub fn display_debug_text(
    e: &mut Engine,
    this: Ptr<CombatProcedureAttackRanged>,
    text: u32,
    first: u32,
    second: u32,
) {
    let t = this.addr();
    let attacker = e.call(GET_ATTACKER, &args![t]).u32();
    let print = |e: &mut Engine, line: u32| {
        e.vcall(text, SLOT_PRINT, &args![first, second, line]);
    };
    match e.get(this, CombatProcedureAttackRanged::eState) {
        0 => print(e, TEXT_IDLE),
        1 => {
            print(e, TEXT_FIRING);
            if e.call(TIMER_RUNNING, &args![t + FIRING_TIMER]).bool() {
                print_timer(e, text, first, second, TEXT_FIRING_TIMER, t + FIRING_TIMER);
            }
            if e.call(GET_TARGET, &args![t]).u32() != 0 {
                print_point(
                    e,
                    text,
                    first,
                    second,
                    TEXT_TARGET_LOCATION,
                    t + TARGETED_POINT,
                );
            }
            if !e.call(TIMER_EXPIRED, &args![t + FIRE_DELAY_TIMER]).bool() {
                print_timer(
                    e,
                    text,
                    first,
                    second,
                    TEXT_FIRING_DELAY_TIMER,
                    t + FIRE_DELAY_TIMER,
                );
            }
        }
        2 => {
            print(e, TEXT_COOLDOWN);
            print_timer(
                e,
                text,
                first,
                second,
                TEXT_COOLDOWN_TIMER,
                t + COOLDOWN_TIMER,
            );
        }
        3 => {
            print(e, TEXT_SUPPRESSIVE_FIRE);
            print_point(
                e,
                text,
                first,
                second,
                TEXT_SUPPRESSION_LOCATION,
                t + TARGETED_POINT,
            );
            print_timer(
                e,
                text,
                first,
                second,
                TEXT_SUPPRESSIVE_FIRE_TIMER,
                t + SUPPRESSIVE_FIRE_TIMER,
            );
            if !e.call(TIMER_EXPIRED, &args![t + FIRE_DELAY_TIMER]).bool() {
                print_timer(
                    e,
                    text,
                    first,
                    second,
                    TEXT_FIRING_DELAY_TIMER,
                    t + FIRE_DELAY_TIMER,
                );
            }
        }
        _ => {}
    }
    if e.call(
        TIMER_RUNNING,
        &args![t + EXPLOSIVE_PROJECTILE_BLOCKED_TIMER],
    )
    .bool()
    {
        print_timer(
            e,
            text,
            first,
            second,
            TEXT_EXPLOSIVE_BLOCKED_TIMER,
            t + EXPLOSIVE_PROJECTILE_BLOCKED_TIMER,
        );
    }
    let controller = controller_of(e, this);
    let weapon = e.call(CONTROLLER_WEAPON, &args![controller]).u32();
    if !e
        .call(POINT_NOT_EQUAL, &args![t + TARGETED_POINT, DEFAULT_POINT])
        .bool()
        || weapon == 0
        || !(f64::from(fn_009d1b10(e, weapon)) > e.global::<f64>(ZERO_DOUBLE))
    {
        return;
    }
    e.with_stack(0x10, |e, angles| {
        let [x, y, z] = point_words(e, t + TARGETED_POINT);
        e.call(
            GET_ANGLE_TO_PROJECTED_POINT,
            &args![angles, attacker, x, y, z],
        );
        let heading = x87_store32(e.call(GET_ACTUAL_HEADING, &args![attacker]).f64());
        let angle_at = angles.addr() + 8;
        let angle = e.mem.u32(angle_at);
        let clamped = x87_store32(e.call(CLAMP_ANGLE, &args![angle]).f64());
        e.mem.set_u32(angle_at, clamped);
        // 009d2224: the local at angles + 0xc is set to 0.0; 009d223b: the cdecl
        // `004b15e0` takes three words (it writes the direction through the third, that
        // local's address pushed at 009d222c); ADD ESP,8 pops two and the third slot is
        // reused for the next call's argument.
        e.mem.set_u32(angles.addr() + 0xc, 0.0f32.to_bits());
        let difference = e
            .call(
                ANGLE_DIFFERENCE,
                &args![heading, clamped, angles.addr() + 0xc],
            )
            .f64();
        let off = x87_store32(e.call(FLOAT_ABS, &args![x87_store32(difference)]).f64());
        let mut arc = fn_009d1b10(e, weapon).to_bits();
        let format = if e.call(IS_PATHING, &args![attacker]).bool() {
            TEXT_ARC_MOVING
        } else {
            let multiplier = setting_bits(e, SETTING_COMBAT_FIRING_ARC_STATIONARY_TURN_MULT);
            arc = x87_store32(x87_mul(x87_load32(arc), x87_m32(multiplier)));
            TEXT_ARC_STATIONARY
        };
        let degrees = x87_mul(
            x87_add(x87_load32(off), x87_load32(off)),
            e.global::<f64>(DEGREES_PER_RADIAN_DOUBLE),
        );
        e.vcall(
            text,
            SLOT_PRINT,
            &args![first, second, format, degrees, x87_load32(arc)],
        );
    });
}

// Translated from 009d2310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackRanged::SaveGame` (Xbox PDB): the base class, the
/// state, the targeted point and the five timers (blocked, cooldown,
/// suppressive fire, firing, fire delay).
pub fn save_game(e: &mut Engine, this: Ptr<CombatProcedureAttackRanged>, buffer: u32) {
    let t = this.addr();
    e.call(BASE_SAVE_GAME, &args![t, buffer]);
    e.call(SAVE_BYTES, &args![buffer, t + 0x10, 4u32, 0u32]);
    e.call(SAVE_BYTES, &args![buffer, t + TARGETED_POINT, 0xcu32, 0u32]);
    for timer in [
        EXPLOSIVE_PROJECTILE_BLOCKED_TIMER,
        COOLDOWN_TIMER,
        SUPPRESSIVE_FIRE_TIMER,
        FIRING_TIMER,
        FIRE_DELAY_TIMER,
    ] {
        e.call(TIMER_SAVE, &args![t + timer, buffer]);
    }
}

// Translated from 009d23a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackRanged::LoadGame` (Xbox PDB): the mirror of
/// `SaveGame`; the fire delay timer only in saves of version 10 or later
/// (the buffer's version is its virtual function at slot 0).
pub fn load_game(e: &mut Engine, this: Ptr<CombatProcedureAttackRanged>, buffer: u32) {
    let t = this.addr();
    e.call(BASE_LOAD_GAME, &args![t, buffer]);
    e.call(LOAD_BYTES, &args![buffer, t + 0x10, 4u32]);
    e.call(LOAD_BYTES, &args![buffer, t + TARGETED_POINT, 0xcu32]);
    for timer in [
        EXPLOSIVE_PROJECTILE_BLOCKED_TIMER,
        COOLDOWN_TIMER,
        SUPPRESSIVE_FIRE_TIMER,
        FIRING_TIMER,
    ] {
        e.call(TIMER_LOAD, &args![t + timer, buffer]);
    }
    if e.vcall(buffer, 0, &args![]).u8() >= 0xa {
        e.call(TIMER_LOAD, &args![t + FIRE_DELAY_TIMER, buffer]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009d1b50, fn_009d1b50(Ptr) -> f32),
        entry!(0x009d_09a0, shutdown(Ptr<CombatProcedureAttackRanged>)),
        entry!(0x009d_09f0, fn_009d09f0(u32, u8)),
        entry!(0x009d_0a10, fn_009d0a10(u32, u8)),
        entry!(0x009d_0a30, update(Ptr<CombatProcedureAttackRanged>)),
        entry!(0x009d_1b10, fn_009d1b10(u32) -> f32),
        entry!(0x009d_1b30, fn_009d1b30(u32) -> f32),
        entry!(0x009d_1b70, fn_009d1b70(u32, u32) -> u32),
        entry!(0x009d_1b90, fn_009d1b90(u32)),
        entry!(
            0x009d_1bc0,
            fn_009d1bc0(Ptr<CombatProcedureAttackRanged>, bool)
        ),
        entry!(0x009d_1c30, stop_firing(Ptr<CombatProcedureAttackRanged>)),
        entry!(
            0x009d_1c90,
            calculate_targeted_point(Ptr<CombatProcedureAttackRanged>, Ptr, u32) -> Ptr
        ),
        entry!(
            0x009d_1d30,
            get_targeted_point(Ptr<CombatProcedureAttackRanged>, u32, u32) -> bool
        ),
        entry!(0x009d_1dc0, disarmed(Ptr<CombatProcedureAttackRanged>, u32)),
        entry!(
            0x009d_1de0,
            aim_at_target(Ptr<CombatProcedureAttackRanged>) -> bool
        ),
        entry!(
            0x009d_1e80,
            display_debug_text(Ptr<CombatProcedureAttackRanged>, u32, u32, u32)
        ),
        entry!(
            0x009d_2310,
            save_game(Ptr<CombatProcedureAttackRanged>, u32)
        ),
        entry!(
            0x009d_23a0,
            load_game(Ptr<CombatProcedureAttackRanged>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_009d1b50_reads_the_float_at_0x54() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x58);
        e.mem.set_f32(this + 0x54, 3.25);
        assert_eq!(e.call(0x009d1b50, &args![this]).f64(), 3.25);
    }

    use std::cell::RefCell;
    use std::collections::{HashMap, VecDeque};
    use std::rc::Rc;

    type Log = Vec<(u32, Vec<u32>)>;

    /// Every fake object's vtable: slot `n` holds `SLOT_BASE + n`, a double.
    const FAKE_VTABLE: u32 = 0x7e00_0000;
    const SLOT_BASE: u32 = 0x7e10_0000;
    const SLOTS: u32 = 0x800;
    /// Where the setting double puts each setting's value.
    const SETTING_CELLS: u32 = 0x7e20_0000;
    const SETTING_BASE: u32 = 0x011c_0000;

    fn ret(value: u32) -> Ret {
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

    /// What the doubles answer: a queue that is used up first, then a
    /// fixed answer, then 0.
    #[derive(Default)]
    struct Script {
        queues: HashMap<u32, VecDeque<Ret>>,
        fixed: HashMap<u32, Ret>,
    }

    struct Rig {
        e: Engine,
        script: Rc<RefCell<Script>>,
    }

    impl Rig {
        fn new() -> Rig {
            let mut e = Engine::new();
            e.map(FAKE_VTABLE, SLOTS);
            for slot in (0..SLOTS).step_by(4) {
                e.mem.set_u32(FAKE_VTABLE + slot, SLOT_BASE + slot);
            }
            e.map(SETTING_CELLS, 0x4_0000);
            exe_data(&mut e);
            let mut rig = Rig {
                e,
                script: Rc::default(),
            };
            for &addr in CALLEES {
                rig.double(addr);
            }
            for slot in (0..SLOTS).step_by(4) {
                rig.double(SLOT_BASE + slot);
            }
            rig.e.register(SETTING_VALUE, |_, a| {
                ret(SETTING_CELLS + (a[0] - SETTING_BASE))
            });
            rig
        }

        fn double(&mut self, addr: u32) {
            let script = Rc::clone(&self.script);
            self.e.register_double(addr, move |_, _| {
                let mut script = script.borrow_mut();
                if let Some(answer) = script.queues.get_mut(&addr).and_then(VecDeque::pop_front) {
                    return answer;
                }
                script.fixed.get(&addr).copied().unwrap_or_default()
            });
        }

        fn answer(&self, addr: u32, value: u32) {
            self.script.borrow_mut().fixed.insert(addr, ret(value));
        }

        fn answer_float(&self, addr: u32, value: f64) {
            self.script.borrow_mut().fixed.insert(addr, float(value));
        }

        /// The answer of the fake vtable's slot `slot`.
        fn slot(&self, slot: u32, value: u32) {
            self.answer(SLOT_BASE + slot, value);
        }

        fn setting(&mut self, setting: u32, value: f32) {
            self.e
                .mem
                .set_f32(SETTING_CELLS + (setting - SETTING_BASE), value);
        }

        /// A zeroed block whose first word is the fake vtable.
        fn object(&mut self, size: u32) -> u32 {
            let block = self.e.mem.alloc(size);
            self.e.mem.set_u32(block, FAKE_VTABLE);
            block
        }

        /// An actor: a fake object whose `ActorValueOwner` (`+0xa4`) has the
        /// fake vtable too.
        fn actor(&mut self) -> u32 {
            let actor = self.object(0x200);
            self.e.mem.set_u32(actor + 0xa4, FAKE_VTABLE);
            actor
        }

        fn run(&mut self, addr: u32, words: &[u32]) -> (Ret, Log) {
            self.e.call_log = Some(vec![]);
            let result = self.e.call(addr, words);
            (result, self.e.call_log.take().unwrap())
        }
    }

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn slot_calls(log: &Log, slot: u32) -> Vec<Vec<u32>> {
        calls_to(log, SLOT_BASE + slot)
    }

    fn bits(value: f32) -> u32 {
        value.to_bits()
    }

    /// The `float` and `double` constants of the exe the translations read
    /// (their bit patterns, read from the exe with the Ghidra server).
    const EXE_FLOATS: [(u32, u32); 22] = [
        (0x0101_2054, 0xbf80_0000),
        (0x0101_3ea4, 0x3c23_d70a),
        (0x0101_622c, 0x3e80_0000),
        (0x0101_6248, 0x3f00_0000),
        (0x0101_62c0, 0x4000_0000),
        (0x0101_712c, 0x40a0_0000),
        (0x0101_7718, 0x4040_0000),
        (0x0101_7b78, 0x4120_0000),
        (0x0101_e2bc, 0x3dcc_cccd),
        (0x0101_e340, 0x4200_0000),
        (0x0101_e57c, 0x4180_0000),
        (0x0101_e580, 0x4170_0000),
        (0x0101_e6ec, 0x4280_0000),
        (0x0101_e704, 0x4300_0000),
        (0x0101_ff50, 0x40c9_0fdb),
        (0x0102_2958, 0x461c_4000),
        (0x0103_2adc, 0x4380_0000),
        (0x0104_e0e8, 0x41c0_0000),
        (0x0105_0b98, 0x43c0_0000),
        (0x0105_36b8, 0x4340_0000),
        (0x0109_0e90, 0x7f7f_ffff),
        (0x0109_14d0, 0x7f7f_ffff),
    ];
    const EXE_DOUBLES: [(u32, u64); 22] = [
        (0x0101_1588, 0x3fe0_0000_0000_0000),
        (0x0101_1590, 0x4000_0000_0000_0000),
        (0x0101_2060, 0x0000_0000_0000_0000),
        (0x0101_2070, 0x3ff0_0000_0000_0000),
        (0x0101_7a10, 0x40b0_0000_0000_0000),
        (0x0101_7a40, 0x4059_0000_0000_0000),
        (0x0101_db80, 0x4010_0000_0000_0000),
        (0x0101_de30, 0x3fe8_0000_0000_0000),
        (0x0101_ecb8, 0x4090_0000_0000_0000),
        (0x0102_0758, 0x4024_0000_0000_0000),
        (0x0102_0998, 0x4014_0000_0000_0000),
        (0x0102_1798, 0x3ff4_0000_0000_0000),
        (0x0102_3128, 0x3f91_df46_a000_0000),
        (0x0102_31d8, 0x4070_0000_0000_0000),
        (0x0102_f248, 0x404c_a5dc_0000_0000),
        (0x0104_f2f0, 0x4039_0000_0000_0000),
        (0x0108_cb08, 0x4122_0000_0000_0000),
        (0x0108_d6e8, 0x4002_0000_0000_0000),
        (0x0108_e058, 0x40f0_0000_0000_0000),
        (0x0109_0d68, 0x3fe5_1eb8_6000_0000),
        (0x0109_1138, 0x4110_0000_0000_0000),
        (0x0109_1b28, 0x4083_8800_0000_0000),
    ];

    /// Maps the exe data the translations read (constants, globals) and
    /// writes the constants' values; the globals start out zeroed.
    fn exe_data(e: &mut Engine) {
        for page in [
            0x011a_5000u32,
            0x011a_8000,
            0x011d_e000,
            0x011f_1000,
            0x011f_4000,
        ] {
            e.map(page, 0x1000);
        }
        for (addr, value) in EXE_FLOATS {
            e.map(addr & !0xfff, 0x1000);
            e.mem.set_u32(addr, value);
        }
        for (addr, value) in EXE_DOUBLES {
            e.map(addr & !0xfff, 0x1000);
            e.mem.set_u64(addr, value);
        }
        // The empty string at `01011584` and the strings the code copies.
        e.mem.set_u8(0x0101_1584, 0);
    }

    /// Every function outside the file that the new translations call by
    /// address.
    const CALLEES: &[u32] = &[
        GET_ATTACKER,
        GET_TARGET,
        SET_LOOK_AT_TARGET,
        CLEAR_LOOK_AT_TARGET,
        BASE_SAVE_GAME,
        BASE_LOAD_GAME,
        FAIL_PROCEDURE,
        GET_COMBAT_STATE,
        CONTROLLER_GROUP,
        CONTROLLER_WEAPON,
        CONTROLLER_STYLE,
        CONTROLLER_ACTOR,
        IS_SWITCHING_WEAPONS,
        STATE_BYTE_71,
        STATE_TEST_EE90,
        STATE_TEST_8980,
        STATE_BYTE_74,
        STATE_BYTE_72,
        STATE_SET_BYTE_72,
        STATE_SET_WORD_78,
        GET_LAST_SEEN_TARGET_FIRING_LOCATION,
        GROUP_IS_TARGET,
        WEAPON_PROJECTILE,
        WEAPON_TEST_524B40,
        WEAPON_TEST_645C80,
        WEAPON_FLOAT_170,
        WEAPON_FLOAT_174,
        WEAPON_FLOAT_178,
        WEAPON_ATTACK_GROUP,
        STYLE_FLOAT_48,
        PROJECTILE_EXPLOSION,
        EXPLOSION_RADIUS,
        MANAGER_EXPLOSION_CHECK,
        START_DIALOGUE,
        TIMER_RUNNING,
        TIMER_START,
        TIMER_EXPIRED,
        TIMER_CLEAR,
        TIMER_ELAPSED,
        TIMER_DURATION,
        TIMESTAMP_SET,
        TIMER_SAVE,
        TIMER_LOAD,
        TIMESTAMP_TIME,
        FLOAT_MAX,
        RANDOM_FLOAT,
        POINT_EQUAL,
        POINT_NOT_EQUAL,
        POINT_DIFFERENCE,
        POINT_LENGTH_SQUARED,
        POINT_LENGTH,
        POINT_CONSTRUCT,
        GET_SAVED_ACQUIRE_OBJECT,
        ACTOR_IS_ATTACKING,
        IS_PATHING,
        QUEUE_ATTACK,
        ANIM_GROUP_OF,
        ANIM_STATE_OF,
        ANIM_DIRECTION,
        ANIM_SEQUENCE_MAP,
        SEQUENCE_MAP_GET,
        SEQUENCE_GROUP,
        ANIM_GROUP_TIME,
        ANIM_GROUP_TYPE,
        ANIM_GROUP_TEST,
        IS_TARGET_WITHIN_FIRING_ARC,
        COMPUTE_OPTIMAL_WEAPON_RANGES,
        GET_PROJECTED_POINT_FOR_TARGET,
        GET_BACK_PROJECTED_ATTACKER_FIRING_LOCATION,
        CALCULATE_PROJECTILE_LOS,
        COMPUTE_PROJECTILE_TIME,
        IS_DANGEROUS_EXPLOSION,
        GET_ANGLE_TO_PROJECTED_POINT,
        GET_ACTUAL_HEADING,
        CLAMP_ANGLE,
        ANGLE_DIFFERENCE,
        FLOAT_ABS,
        SAVE_BYTES,
        LOAD_BYTES,
    ];

    struct World {
        rig: Rig,
        this: Ptr<CombatProcedureAttackRanged>,
        t: u32,
        controller: u32,
        attacker: u32,
        target: u32,
        weapon: u32,
        style: u32,
        animation: u32,
        object: u32,
        position: u32,
        combat_state: u32,
    }

    /// A ranged procedure with its controller, actors, weapon record,
    /// style, animation, acquire object, position point and combat state.
    fn world() -> World {
        let mut rig = Rig::new();
        let t = rig.e.mem.alloc(0x48);
        let this = Ptr::<CombatProcedureAttackRanged>::new(t);
        let controller = rig.object(0x200);
        rig.e.set(
            this,
            CombatProcedureAttackRanged::pCombatController,
            Ptr::new(controller),
        );
        let attacker = rig.actor();
        let target = rig.actor();
        let weapon = rig.e.mem.alloc(0x200);
        let style = rig.e.mem.alloc(0x100);
        let animation = rig.object(0x40);
        let object = rig.object(0x40);
        let position = rig.e.mem.alloc(0xc);
        let combat_state = rig.e.mem.alloc(0x100);
        rig.answer(GET_ATTACKER, attacker);
        rig.answer(GET_TARGET, target);
        rig.answer(CONTROLLER_WEAPON, weapon);
        rig.answer(CONTROLLER_STYLE, style);
        rig.answer(CONTROLLER_ACTOR, attacker);
        rig.answer(GET_SAVED_ACQUIRE_OBJECT, object);
        rig.answer(GET_COMBAT_STATE, combat_state);
        rig.answer(WEAPON_PROJECTILE, 0x6000);
        rig.slot(SLOT_ANIMATION, animation);
        rig.slot(SLOT_POSITION, position);
        World {
            rig,
            this,
            t,
            controller,
            attacker,
            target,
            weapon,
            style,
            animation,
            object,
            position,
            combat_state,
        }
    }

    #[test]
    fn shutdown_stops_firing_and_clears_the_controller_bytes() {
        let mut w = world();
        w.rig.e.mem.set_u8(w.controller + 0x142, 4);
        let (_, log) = w.rig.run(0x009d_09a0, &args![w.t]);
        assert_eq!(calls_to(&log, CLEAR_LOOK_AT_TARGET), vec![vec![w.t]]);
        assert_eq!(w.rig.e.mem.u8(w.controller + 0x142), 0);
        assert_eq!(w.rig.e.mem.u8(w.controller + 0xc6), 0xff);
        assert_eq!(slot_calls(&log, SLOT_SET_6E0), vec![vec![w.object, 0]]);
        assert_eq!(calls_to(&log, TIMER_CLEAR), vec![vec![w.t + 0x28]]);
    }

    #[test]
    fn fn_009d09f0_and_fn_009d0a10_store_controller_bytes() {
        let mut w = world();
        w.rig.run(0x009d_09f0, &args![w.controller, 1u32]);
        w.rig.run(0x009d_0a10, &args![w.controller, 4u32]);
        assert_eq!(w.rig.e.mem.u8(w.controller + 0xc6), 1);
        assert_eq!(w.rig.e.mem.u8(w.controller + 0x142), 4);
    }

    #[test]
    fn update_fails_without_a_weapon_or_outside_the_field_of_view() {
        let mut w = world();
        w.rig.answer(CONTROLLER_WEAPON, 0);
        let (_, log) = w.rig.run(0x009d_0a30, &args![w.t]);
        assert_eq!(
            calls_to(&log, FAIL_PROCEDURE),
            vec![vec![w.t, TEXT_MISSING_WEAPON]]
        );
        assert!(calls_to(&log, WEAPON_PROJECTILE).is_empty());

        let mut w = world();
        w.rig.answer_float(STYLE_FLOAT_48, 90.0);
        let (_, log) = w.rig.run(0x009d_0a30, &args![w.t]);
        let arc = calls_to(&log, IS_TARGET_WITHIN_FIRING_ARC);
        assert_eq!(arc.len(), 1);
        let radians = x87_store32(x87_mul(90.0, f64::from_bits(0x3f91_df46_a000_0000)));
        assert_eq!(arc[0], vec![w.attacker, w.position, radians, 0x40c9_0fdb]);
        assert_eq!(
            calls_to(&log, FAIL_PROCEDURE),
            vec![vec![w.t, TEXT_OUTSIDE_FOV]]
        );
    }

    #[test]
    fn update_runs_the_burst_cycle() {
        // Cooling down with the cooldown timer run out: firing again.
        let mut w = world();
        w.rig.slot(SLOT_TEST_3F8, 1);
        w.rig.answer(STATE_BYTE_71, 1);
        w.rig.answer(STATE_TEST_EE90, 1);
        w.rig.answer(WEAPON_TEST_524B40, 1);
        w.rig.answer(TIMER_EXPIRED, 1);
        w.rig.answer(TIMESTAMP_SET, 0);
        w.rig.e.set(w.this, CombatProcedureAttackRanged::eState, 2);
        let manager = 0x6100;
        w.rig.e.mem.set_u32(DIALOGUE_MANAGER, manager);
        let (_, log) = w.rig.run(0x009d_0a30, &args![w.t]);
        assert_eq!(w.rig.e.get(w.this, CombatProcedureAttackRanged::eState), 1);
        assert_eq!(slot_calls(&log, SLOT_SET_6E0), vec![vec![w.object, 1]]);
        let set = calls_to(&log, TIMESTAMP_SET);
        assert_eq!(set[0][1], (-f32::MAX).to_bits());
        assert_eq!(
            calls_to(&log, COMPUTE_OPTIMAL_WEAPON_RANGES)[0][..2],
            [w.controller, w.weapon]
        );
        assert_eq!(
            calls_to(&log, START_DIALOGUE),
            vec![vec![manager, w.controller, 2, 0, 0]]
        );
        // Not aimed: nothing fired.
        assert!(calls_to(&log, CALCULATE_PROJECTILE_LOS).is_empty());

        // Firing a burst whose time ran out: cooling down.
        let mut w = world();
        w.rig.slot(SLOT_TEST_3F8, 1);
        w.rig.answer(STATE_BYTE_71, 1);
        w.rig.answer(STATE_TEST_EE90, 1);
        w.rig.answer(WEAPON_TEST_645C80, 1);
        w.rig.answer(TIMER_RUNNING, 1);
        w.rig.answer(TIMER_EXPIRED, 1);
        w.rig
            .setting(SETTING_AUTOMATIC_WEAPON_BURST_COOLDOWN_TIME, 1.25);
        w.rig.e.set(w.this, CombatProcedureAttackRanged::eState, 1);
        let (_, log) = w.rig.run(0x009d_0a30, &args![w.t]);
        assert_eq!(w.rig.e.get(w.this, CombatProcedureAttackRanged::eState), 2);
        assert_eq!(
            calls_to(&log, TIMER_START),
            vec![vec![w.t + 0x30, bits(1.25)]]
        );
    }

    #[test]
    fn update_fires_once_aimed() {
        let mut w = world();
        let t = w.t;
        let delay = t + 0x38;
        w.rig.slot(SLOT_TEST_3F8, 1);
        w.rig.e.set(w.this, CombatProcedureAttackRanged::eState, 3);
        w.rig
            .e
            .register_double(TIMER_EXPIRED, move |_, a| ret(u32::from(a[0] == delay)));
        w.rig.answer(IS_TARGET_WITHIN_FIRING_ARC, 1);
        w.rig.slot(SLOT_FIRE_KIND, u32::MAX);
        w.rig.answer(ANIM_DIRECTION, 0x34);
        w.rig.answer(QUEUE_ATTACK, 1);
        w.rig.answer_float(WEAPON_FLOAT_174, 2.0);
        w.rig.answer_float(WEAPON_FLOAT_178, 3.0);
        w.rig.e.mem.set_f32(w.style + 0x50, 0.5);
        w.rig.e.mem.set_f32(w.style + 0x54, 1.5);
        w.rig.answer_float(RANDOM_FLOAT, 2.5);
        let (_, log) = w.rig.run(0x009d_0a30, &args![t]);
        let los = calls_to(&log, CALCULATE_PROJECTILE_LOS);
        assert_eq!(los.len(), 1);
        assert_eq!(los[0][..3], [w.attacker, 0x6000, 0]);
        assert_eq!(calls_to(&log, QUEUE_ATTACK), vec![vec![w.attacker, 0x2c]]);
        assert_eq!(calls_to(&log, ANIM_DIRECTION), vec![vec![w.animation, 0]]);
        assert_eq!(slot_calls(&log, SLOT_SET_43C), vec![vec![w.object, 1]]);
        assert_eq!(
            calls_to(&log, RANDOM_FLOAT),
            vec![vec![bits(1.0), bits(4.5)]]
        );
        assert_eq!(calls_to(&log, TIMER_START), vec![vec![delay, bits(2.5)]]);
        assert_eq!(w.rig.e.get(w.this, CombatProcedureAttackRanged::eState), 3);

        // Suppressive fire blocked by something that is not an enemy: back
        // to idle.
        let mut w = world();
        let t = w.t;
        let delay = t + 0x38;
        w.rig.slot(SLOT_TEST_3F8, 1);
        w.rig.e.set(w.this, CombatProcedureAttackRanged::eState, 3);
        w.rig
            .e
            .register_double(TIMER_EXPIRED, move |_, a| ret(u32::from(a[0] == delay)));
        w.rig.answer(IS_TARGET_WITHIN_FIRING_ARC, 1);
        w.rig.slot(SLOT_FIRE_KIND, u32::MAX);
        w.rig.answer(CALCULATE_PROJECTILE_LOS, 1);
        let (_, log) = w.rig.run(0x009d_0a30, &args![t]);
        assert_eq!(w.rig.e.get(w.this, CombatProcedureAttackRanged::eState), 0);
        assert!(calls_to(&log, TIMER_CLEAR).contains(&vec![t + 0x40]));
        assert!(calls_to(&log, QUEUE_ATTACK).is_empty());
    }

    #[test]
    fn fn_009d1b10_and_fn_009d1b30_read_floats() {
        let mut w = world();
        w.rig.e.mem.set_f32(w.weapon + 0x158, 30.0);
        w.rig.e.mem.set_f32(w.style + 0x50, 0.75);
        assert_eq!(w.rig.run(0x009d_1b10, &args![w.weapon]).0.f32(), 30.0);
        assert_eq!(w.rig.run(0x009d_1b30, &args![w.style]).0.f32(), 0.75);
    }

    #[test]
    fn fn_009d1b70_and_fn_009d1b90_handle_the_time_stamp() {
        let mut w = world();
        let out = w.rig.e.mem.alloc(4);
        w.rig.e.mem.set_u32(w.combat_state + 0xa4, 0x1234);
        assert_eq!(
            w.rig.run(0x009d_1b70, &args![w.combat_state, out]).0.u32(),
            out
        );
        assert_eq!(w.rig.e.mem.u32(out), 0x1234);
        w.rig.e.register(TIMESTAMP_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        w.rig.run(0x009d_1b90, &args![w.combat_state]);
        assert_eq!(
            w.rig.e.mem.u32(w.combat_state + 0xa4),
            (-f32::MAX).to_bits()
        );
    }

    #[test]
    fn fn_009d1bc0_and_stop_firing_set_the_acquire_object() {
        let mut w = world();
        let (_, log) = w.rig.run(0x009d_1bc0, &args![w.t, 1u32]);
        assert_eq!(calls_to(&log, CONTROLLER_ACTOR), vec![vec![w.controller]]);
        assert_eq!(slot_calls(&log, SLOT_SET_6E0), vec![vec![w.object, 1]]);
        assert_eq!(slot_calls(&log, SLOT_TEST_434), vec![vec![w.object, 1]]);
        assert_eq!(calls_to(&log, TIMER_CLEAR), vec![vec![w.t + 0x28]]);
        let (_, log) = w.rig.run(0x009d_1c30, &args![w.t]);
        assert_eq!(slot_calls(&log, SLOT_SET_6E0), vec![vec![w.object, 0]]);
        assert_eq!(slot_calls(&log, SLOT_TEST_434), vec![vec![w.object, 0]]);
    }

    #[test]
    fn calculate_targeted_point_projects_the_target() {
        let mut w = world();
        let out = w.rig.e.mem.alloc(0xc);
        w.rig.e.register(GET_PROJECTED_POINT_FOR_TARGET, |e, a| {
            e.mem.set_f32(a[0], 1.0);
            e.mem.set_f32(a[0] + 4, 2.0);
            e.mem.set_f32(a[0] + 8, 3.0);
            Ret::default()
        });
        let (result, log) = w.rig.run(0x009d_1c90, &args![w.t, out, 7u32]);
        assert_eq!(result.u32(), out);
        let call = calls_to(&log, GET_PROJECTED_POINT_FOR_TARGET);
        assert_eq!(call[0][1..], [w.attacker, w.target, 7, 0x6000, 0, 0]);
        assert_eq!(w.rig.e.mem.f32(out + 8), 3.0);
        assert_eq!(
            calls_to(&log, WEAPON_PROJECTILE),
            vec![vec![w.weapon, w.attacker]]
        );

        w.rig.answer(GET_TARGET, 0);
        w.rig.e.mem.set_f32(DEFAULT_POINT + 4, 9.0);
        let (_, log) = w.rig.run(0x009d_1c90, &args![w.t, out, 7u32]);
        assert!(calls_to(&log, GET_PROJECTED_POINT_FOR_TARGET).is_empty());
        assert_eq!(w.rig.e.mem.f32(out + 4), 9.0);
    }

    #[test]
    fn get_targeted_point_by_state() {
        let mut w = world();
        let out = w.rig.e.mem.alloc(0xc);
        w.rig.e.register(GET_PROJECTED_POINT_FOR_TARGET, |e, a| {
            e.mem.set_f32(a[0], 4.0);
            Ret::default()
        });
        assert!(!w.rig.run(0x009d_1d30, &args![w.t, out, 0u32]).0.bool());
        w.rig.e.set(w.this, CombatProcedureAttackRanged::eState, 1);
        assert!(w.rig.run(0x009d_1d30, &args![w.t, out, 0u32]).0.bool());
        assert_eq!(w.rig.e.mem.f32(w.t + 0x14), 4.0);
        assert_eq!(w.rig.e.mem.f32(out), 4.0);
        w.rig.e.set(w.this, CombatProcedureAttackRanged::eState, 3);
        w.rig.e.mem.set_f32(w.t + 0x14, 6.0);
        let (result, log) = w.rig.run(0x009d_1d30, &args![w.t, out, 0u32]);
        assert!(result.bool());
        assert!(calls_to(&log, GET_TARGET).is_empty());
        assert_eq!(w.rig.e.mem.f32(out), 6.0);
        w.rig.e.set(w.this, CombatProcedureAttackRanged::eState, 4);
        assert!(!w.rig.run(0x009d_1d30, &args![w.t, out, 0u32]).0.bool());
    }

    #[test]
    fn disarmed_fails_the_procedure() {
        let mut w = world();
        let (_, log) = w.rig.run(0x009d_1dc0, &args![w.t, 0u32]);
        assert_eq!(
            calls_to(&log, FAIL_PROCEDURE),
            vec![vec![w.t, TEXT_DISARMED]]
        );
    }

    #[test]
    fn aim_at_target_checks_the_firing_arc() {
        let mut w = world();
        w.rig.answer(POINT_EQUAL, 1);
        let (result, log) = w.rig.run(0x009d_1de0, &args![w.t]);
        assert!(!result.bool());
        assert!(calls_to(&log, IS_TARGET_WITHIN_FIRING_ARC).is_empty());
        w.rig.answer(POINT_EQUAL, 0);
        w.rig.e.mem.set_f32(w.weapon + 0x158, 10.0);
        w.rig.answer_float(FLOAT_MAX, 15.0);
        w.rig.answer(IS_TARGET_WITHIN_FIRING_ARC, 1);
        let (result, log) = w.rig.run(0x009d_1de0, &args![w.t]);
        assert!(result.bool());
        assert_eq!(
            calls_to(&log, FLOAT_MAX),
            vec![vec![bits(10.0), bits(15.0)]]
        );
        let radians = x87_store32(x87_mul(15.0, f64::from_bits(0x3f91_df46_a000_0000)));
        assert_eq!(
            calls_to(&log, IS_TARGET_WITHIN_FIRING_ARC),
            vec![vec![w.attacker, w.t + 0x14, radians, radians]]
        );
    }

    #[test]
    fn display_debug_text_prints_the_state() {
        let mut w = world();
        let text = w.rig.object(8);
        w.rig.e.set(w.this, CombatProcedureAttackRanged::eState, 1);
        w.rig.answer(TIMER_RUNNING, 1);
        w.rig.answer_float(TIMER_DURATION, 2.0);
        w.rig.answer_float(TIMER_ELAPSED, 0.5);
        w.rig.e.mem.set_f32(w.t + 0x14, 1.0);
        w.rig.e.mem.set_f32(w.t + 0x18, 2.0);
        w.rig.e.mem.set_f32(w.t + 0x1c, 3.0);
        let (_, log) = w.rig.run(0x009d_1e80, &args![w.t, text, 5u32, 6u32]);
        let mut timer = vec![text, 5, 6, TEXT_FIRING_TIMER];
        timer.extend(args![0.5f64, 2.0f64]);
        let mut location = vec![text, 5, 6, TEXT_TARGET_LOCATION];
        location.extend(args![1.0f64, 2.0f64, 3.0f64]);
        let mut delay = vec![text, 5, 6, TEXT_FIRING_DELAY_TIMER];
        delay.extend(args![0.5f64, 2.0f64]);
        let mut blocked = vec![text, 5, 6, TEXT_EXPLOSIVE_BLOCKED_TIMER];
        blocked.extend(args![0.5f64, 2.0f64]);
        assert_eq!(
            slot_calls(&log, SLOT_PRINT),
            vec![
                vec![text, 5, 6, TEXT_FIRING],
                timer,
                location,
                delay,
                blocked
            ]
        );

        // Idle with a weapon arc: the angle line (stationary).
        let mut w = world();
        let text = w.rig.object(8);
        w.rig.answer(POINT_NOT_EQUAL, 1);
        w.rig.answer(TIMER_EXPIRED, 1);
        w.rig.e.mem.set_f32(w.weapon + 0x158, 10.0);
        w.rig
            .setting(SETTING_COMBAT_FIRING_ARC_STATIONARY_TURN_MULT, 2.0);
        w.rig.answer_float(FLOAT_ABS, 0.5);
        let (_, log) = w.rig.run(0x009d_1e80, &args![w.t, text, 5u32, 6u32]);
        let degrees = x87_mul(x87_add(0.5, 0.5), f64::from_bits(0x404c_a5dc_0000_0000));
        let mut arc = vec![text, 5, 6, TEXT_ARC_STATIONARY];
        arc.extend(args![degrees, 20.0f64]);
        assert_eq!(
            slot_calls(&log, SLOT_PRINT),
            vec![vec![text, 5, 6, TEXT_IDLE], arc]
        );
    }

    #[test]
    fn save_and_load_game_cover_the_timers() {
        let mut w = world();
        let t = w.t;
        let (_, log) = w.rig.run(0x009d_2310, &args![t, 0x40u32]);
        let expected: Log = vec![
            (0x009d_2310, vec![t, 0x40]),
            (BASE_SAVE_GAME, vec![t, 0x40]),
            (SAVE_BYTES, vec![0x40, t + 0x10, 4, 0]),
            (SAVE_BYTES, vec![0x40, t + 0x14, 0xc, 0]),
            (TIMER_SAVE, vec![t + 0x20, 0x40]),
            (TIMER_SAVE, vec![t + 0x30, 0x40]),
            (TIMER_SAVE, vec![t + 0x40, 0x40]),
            (TIMER_SAVE, vec![t + 0x28, 0x40]),
            (TIMER_SAVE, vec![t + 0x38, 0x40]),
        ];
        assert_eq!(log, expected);
        let buffer = w.rig.object(8);
        w.rig.slot(0, 9);
        let (_, log) = w.rig.run(0x009d_23a0, &args![t, buffer]);
        assert_eq!(
            calls_to(&log, TIMER_LOAD),
            vec![
                vec![t + 0x20, buffer],
                vec![t + 0x30, buffer],
                vec![t + 0x40, buffer],
                vec![t + 0x28, buffer]
            ]
        );
        w.rig.slot(0, 10);
        let (_, log) = w.rig.run(0x009d_23a0, &args![t, buffer]);
        assert_eq!(calls_to(&log, TIMER_LOAD).len(), 5);
        assert_eq!(
            calls_to(&log, LOAD_BYTES),
            vec![vec![buffer, t + 0x10, 4], vec![buffer, t + 0x14, 0xc]]
        );
    }
}
