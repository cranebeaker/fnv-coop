//! `fallout/magic/valuemodifiereffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `ValueModifierEffect` (Xbox PDB, 0x4C bytes) is the `ActiveEffect` that
//! changes an actor value (`eWhichActorValue`, +0x48) of its target. Its
//! own virtual slots: +0x5C `SetActorValue`, +0x60 `GetActorValue`, +0x64
//! `ModifyActorValue` (Xbox PDB order). The target's actor-value owner is
//! the `Actor` part at +0xA4 (slots +0x8 `GetActorValue`, +0xC
//! `GetActorFloatValue`, +0x20 `GetPermanentActorFloatValue`); the Actor
//! slots used (+0x22C, +0x360, +0x39C, +0x3A4, +0x3AC) and the process
//! slots (+0x72C `ModTrackedDamage`, +0x754 `AddRadiationMagicDelta`,
//! +0x758 `RemoveRadiationMagicDelta`) take the argument counts their PC
//! implementations show.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_magic::activeeffect::ActiveEffect;

layout! {
    /// `ValueModifierEffect` (Xbox PDB): an `ActiveEffect` and the actor
    /// value it modifies.
    pub struct ValueModifierEffect: 0x4c {
        /// `eWhichActorValue` (Xbox PDB).
        0x48 eWhichActorValue: u32,
    }
}

/// The player (`PlayerCharacter*`).
const PLAYER: u32 = 0x011d_ea3c;
/// The `double` 0.0 and 1.0.
const ZERO: u32 = 0x0101_2060;
const ONE: u32 = 0x0101_2070;
/// The `float` 2.0 the reduction message is shown with.
const MESSAGE_TIME: u32 = 0x0101_62c0;
/// The active effect's target (`0059bb30`, `ECX` = effect) and the
/// target's actor (`MagicTarget::GetTargetAsActor`, `00822b40`).
const EFFECT_TARGET: u32 = 0x0059_bb30;
const TARGET_AS_ACTOR: u32 = 0x0082_2b40;
/// The active effect's caster (`0045cd60`) and
/// `MagicCaster::GetCasterAsActor` (`00815410`).
const EFFECT_CASTER: u32 = 0x0045_cd60;
const CASTER_AS_ACTOR: u32 = 0x0081_5410;
/// The active effect's magic item (`0044ddc0`).
const EFFECT_ITEM: u32 = 0x0044_ddc0;
/// The word at +0xC (`0084e3a0`).
const WORD_AT_C: u32 = 0x0084_e3a0;
/// `EffectItem`'s setting (`00825c00`), the setting's flag test
/// (`00403e60`), the effect item's actor value (`00403ea0`),
/// `EffectItem::GetDuration` (`004044d0`) and `EffectItem::IsHostile`
/// (`00404710`).
const EFFECT_ITEM_SETTING: u32 = 0x0082_5c00;
const SETTING_HAS_FLAG: u32 = 0x0040_3e60;
const EFFECT_ITEM_ACTOR_VALUE: u32 = 0x0040_3ea0;
const EFFECT_ITEM_DURATION: u32 = 0x0040_44d0;
const EFFECT_ITEM_IS_HOSTILE: u32 = 0x0040_4710;
/// The active effect's tests `00804b60` and `007df210`, and its kind
/// (`0055b980`).
const EFFECT_TEST: u32 = 0x0080_4b60;
const EFFECT_NOT_PLAIN: u32 = 0x007d_f210;
const EFFECT_KIND: u32 = 0x0055_b980;
/// `ActiveEffect::CopyData` and `ActiveEffect::Finalize` (Xbox PDB names).
const ACTIVE_EFFECT_COPY_DATA: u32 = 0x0080_4090;
const ACTIVE_EFFECT_FINALIZE: u32 = 0x0080_4160;
/// `ValueModifierEffect`'s constructor (`0082ba60`).
const VALUE_MODIFIER_EFFECT_CONSTRUCT: u32 = 0x0082_ba60;
/// `operator new` (`00401000`, cdecl: size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `__RTDynamicCast` and the type descriptors.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_ACTIVE_EFFECT: u32 = 0x011a_0d1c;
const TYPE_VALUE_MODIFIER_EFFECT: u32 = 0x011a_0cf8;
const TYPE_MAGIC_ITEM: u32 = 0x0118_3140;
const TYPE_ALCHEMY_ITEM: u32 = 0x0118_30cc;
/// The alchemy item's flag test (`004029d0`, `this`; flag).
const ALCHEMY_HAS_FLAG: u32 = 0x0040_29d0;
/// The stat amount (`0040ebd0`, cdecl: two floats, `ST0`), `_ftol2`
/// (`00ec62c0`) and `MiscStatManager::ModVal` (`004d5e10`, cdecl: stat,
/// amount).
const STAT_AMOUNT: u32 = 0x0040_ebd0;
const FTOL: u32 = 0x00ec_62c0;
const MOD_STAT: u32 = 0x004d_5e10;
/// `Actor::Kill` (`0089d900`: killer, float).
const ACTOR_KILL: u32 = 0x0089_d900;
/// `MiddleHighProcess::GetSavedAcquireObject` (the map's name of
/// `008d8520`; the actor's process), the effect's radiation amount
/// (`00495460`, `ST0`) and the player's update after a radiation change
/// (`00968730`).
const ACTOR_PROCESS: u32 = 0x008d_8520;
const RADIATION_AMOUNT: u32 = 0x0049_5460;
const PLAYER_RADIATION_CHANGED: u32 = 0x0096_8730;
/// The actor's health (`00568ad0`, `ST0`).
const ACTOR_HEALTH: u32 = 0x0056_8ad0;
/// `Actor::GetEssential` (`0087f3d0`).
const ACTOR_GET_ESSENTIAL: u32 = 0x0087_f3d0;
/// `Actor::DifficultyLevelAdjustHealthModifier` (`008808a0`: value,
/// killer; `ST0`).
const DIFFICULTY_ADJUST: u32 = 0x0088_08a0;
/// The setting's test of an actor value in `Finalize` (`007869e0`, `ECX` =
/// setting; actor value).
const SETTING_REVERSES: u32 = 0x0078_69e0;
/// Actor-value class tests (cdecl: actor value).
const VALUE_TEST_A: u32 = 0x005d_1260;
const VALUE_TEST_B: u32 = 0x0066_fd90;
const VALUE_TEST_C: u32 = 0x0047_f060;
/// The log print (`005b5e40`, cdecl: text).
const PRINT: u32 = 0x005b_5e40;
/// The damage event's source (`004537b0`, no arguments) and the event
/// (`0087a8b0`, cdecl, variable: source, 0x116C, actor, double).
const EVENT_SOURCE: u32 = 0x0045_37b0;
const SEND_EVENT: u32 = 0x0087_a8b0;
/// Game-setting strings (`00403df0`, `ECX` = setting), the actor-value
/// name (`ActorValue::GetActorValueName`, `0066e950`), `sprintf`
/// (`00ec623a`) and the HUD message (`007052f0`, cdecl, six words).
const SETTING_STRING: u32 = 0x0040_3df0;
const ACTOR_VALUE_NAME: u32 = 0x0066_e950;
const SPRINTF: u32 = 0x00ec_623a;
const SHOW_MESSAGE: u32 = 0x0070_52f0;
/// The three string settings of the reduction message (the format word,
/// and the two endings; their names are not read here).
const MESSAGE_FIRST: u32 = 0x011d_2ce8;
const MESSAGE_ENDING_A: u32 = 0x011d_21a8;
const MESSAGE_ENDING_B: u32 = 0x011d_4a5c;
/// Strings.
const TEXT_FORMAT: u32 = 0x0107_34e0;
const TEXT_FOOD: u32 = 0x0107_c708;
const TEXT_STIMPAK: u32 = 0x0107_c6f4;
const TEXT_NO_ACTOR_VALUE: u32 = 0x0107_c718;
/// The actor values the code singles out: health (0x10), 7, radiation
/// (0x36).
const HEALTH: u32 = 0x10;
const RADIATION: u32 = 0x36;

const SIGN: u32 = 0x8000_0000;

fn which(e: &Engine, this: Ptr<ActiveEffect>) -> u32 {
    e.get(
        this.cast::<ValueModifierEffect>(),
        ValueModifierEffect::eWhichActorValue,
    )
}

fn magnitude_bits(e: &Engine, this: Ptr<ActiveEffect>) -> u32 {
    e.get(this, ActiveEffect::fMagnitude).to_bits()
}

fn set_magnitude_bits(e: &mut Engine, this: Ptr<ActiveEffect>, bits: u32) {
    e.set(this, ActiveEffect::fMagnitude, f32::from_bits(bits));
}

/// The target's actor: null without a target.
fn target_actor(e: &mut Engine, this: Ptr<ActiveEffect>) -> u32 {
    if e.call(EFFECT_TARGET, &args![this]).u32() == 0 {
        return 0;
    }
    let target = e.call(EFFECT_TARGET, &args![this]).u32();
    e.call(TARGET_AS_ACTOR, &args![target]).u32()
}

/// The caster's actor: null without a caster.
fn caster_actor(e: &mut Engine, this: Ptr<ActiveEffect>) -> u32 {
    if e.call(EFFECT_CASTER, &args![this]).u32() == 0 {
        return 0;
    }
    let caster = e.call(EFFECT_CASTER, &args![this]).u32();
    e.call(CASTER_AS_ACTOR, &args![caster]).u32()
}

fn setting_flag(e: &mut Engine, this: Ptr<ActiveEffect>, flag: u32) -> bool {
    let effect = e.get(this, ActiveEffect::pEffect);
    let setting = e.call(EFFECT_ITEM_SETTING, &args![effect]).u32();
    e.call(SETTING_HAS_FLAG, &args![setting, flag]).bool()
}

/// Kills `target` (`Actor::Kill` with the caster's actor and 0.0) when its
/// health is at most 1.0 (`TEST AH,0x41; JP`: not when above or NaN).
fn kill_when_spent(e: &mut Engine, this: Ptr<ActiveEffect>, target: u32) {
    let health = e
        .vcall(target.wrapping_add(0xa4), 0xc, &args![HEALTH])
        .f64();
    if health <= e.global::<f64>(ONE) {
        let killer = caster_actor(e, this);
        e.call(ACTOR_KILL, &args![target, killer, 0.0f32.to_bits()]);
    }
}

/// The player's health below its permanent value (`float`).
fn player_health_lost(e: &mut Engine) -> u32 {
    let owner = e.global::<u32>(PLAYER).wrapping_add(0xa4);
    let permanent = e.vcall(owner, 0x20, &args![HEALTH]).f64();
    let owner = e.global::<u32>(PLAYER).wrapping_add(0xa4);
    let current = e.vcall(owner, 0xc, &args![HEALTH]).f64();
    x87_store32(x87_sub(permanent, current))
}

/// The alchemy stats `Start` keeps for the player's healing: stat 0x12
/// (item flag 2) and 0x10 (item flag 4), each `_ftol2(0040ebd0(amount,
/// lost))`, `amount` computed per stat by `amount_of`.
fn alchemy_stats(
    e: &mut Engine,
    this: Ptr<ActiveEffect>,
    lost: u32,
    amount_of: fn(&mut Engine, Ptr<ActiveEffect>) -> u32,
) {
    let item = e.call(EFFECT_ITEM, &args![this]).u32();
    let is_alchemy = e.vcall(item, 0x18, &[]).u32() == 7;
    let alchemy = if is_alchemy {
        e.call(
            RT_DYNAMIC_CAST,
            &args![item, 0u32, TYPE_MAGIC_ITEM, TYPE_ALCHEMY_ITEM, 0u32],
        )
        .u32()
    } else {
        0
    };
    for (flag, stat) in [(2u32, 0x12u32), (4, 0x10)] {
        if alchemy != 0 && e.call(ALCHEMY_HAS_FLAG, &args![alchemy, flag]).bool() {
            let amount = amount_of(e, this);
            let value = e.call(STAT_AMOUNT, &args![amount, lost]).f64();
            let whole = e.call(FTOL, &args![value]).u32();
            e.call(MOD_STAT, &args![stat, whole]);
        }
    }
}

/// The radiation step `Start` (process slot +0x754) and `Finish` (+0x758)
/// end with.
fn radiation_step(e: &mut Engine, this: Ptr<ActiveEffect>, target: u32, slot: u32) {
    if which(e, this) != RADIATION || !setting_flag(e, this, 4) {
        return;
    }
    if target != 0 && e.call(ACTOR_PROCESS, &args![target]).u32() != 0 {
        let process = e.call(ACTOR_PROCESS, &args![target]).u32();
        let amount = x87_store32(e.call(RADIATION_AMOUNT, &args![this]).f64());
        e.vcall(process, slot, &args![amount]);
    }
    if target == e.global::<u32>(PLAYER) {
        let player = e.global::<u32>(PLAYER);
        e.call(PLAYER_RADIATION_CHANGED, &args![player]);
    }
}

// Translated from 0082bb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ValueModifierEffect::Copy` (Xbox PDB): a new `ValueModifierEffect`
/// (0x4C bytes) for the same caster, spell and effect item, given this
/// one's data (`CopyData`, slot +0x44; null when the allocation failed).
/// Returns it.
// Translation incomplete: 0082bb30 - SEH exception frame (FS:[0]) that frees the block if the constructor throws is not modelled.
pub fn value_modifier_effect_copy(e: &mut Engine, this: Ptr<ActiveEffect>) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x4cu32]).u32();
    let copy = if block != 0 {
        let caster = e.get(this, ActiveEffect::pCaster);
        let spell = e.get(this, ActiveEffect::pSpell);
        let effect = e.get(this, ActiveEffect::pEffect);
        e.call(
            VALUE_MODIFIER_EFFECT_CONSTRUCT,
            &args![block, caster, spell, effect],
        )
        .u32()
    } else {
        0
    };
    e.vcall(this.addr(), 0x44, &args![copy]);
    copy
}

// Translated from 0082bbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ValueModifierEffect::CopyData` (Xbox PDB): `ActiveEffect::CopyData`,
/// then the actor value, when `other` is a `ValueModifierEffect`.
pub fn value_modifier_effect_copy_data(e: &mut Engine, this: Ptr<ActiveEffect>, other: u32) {
    e.call(ACTIVE_EFFECT_COPY_DATA, &args![this, other]);
    let target = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TYPE_ACTIVE_EFFECT,
                TYPE_VALUE_MODIFIER_EFFECT,
                0u32
            ],
        )
        .u32();
    if target != 0 {
        let value = which(e, this);
        e.mem.set_u32(target.wrapping_add(0x48), value);
    }
}

// Translated from 0082bc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ValueModifierEffect` creation function (cdecl: caster, magic item,
/// effect item): a new 0x4C-byte effect, or null when the allocation
/// failed.
// Translation incomplete: 0082bc30 - SEH exception frame (FS:[0]) that frees the block if the constructor throws is not modelled.
pub fn fn_0082bc30(e: &mut Engine, caster: u32, spell: u32, effect: u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x4cu32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(
        VALUE_MODIFIER_EFFECT_CONSTRUCT,
        &args![block, caster, spell, effect],
    )
    .u32()
}

// Translated from 0082bcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ValueModifierEffect::CheckTarget` (Xbox PDB): true when `target` is an
/// actor whose slot +0x22C (with 0) answers false.
pub fn value_modifier_effect_check_target(
    e: &mut Engine,
    _this: Ptr<ActiveEffect>,
    target: u32,
) -> bool {
    let actor = if target == 0 {
        0
    } else {
        e.call(TARGET_AS_ACTOR, &args![target]).u32()
    };
    actor != 0 && !e.vcall(actor, 0x22c, &args![0u32]).bool()
}

// Translated from 0082bd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ValueModifierEffect::Finalize` (Xbox PDB): when the setting reverses
/// the actor value (`007869e0`) the magnitude changes sign, and for a
/// health effect on an actor whose effect item's +0xC word is set it is
/// adjusted for the difficulty level with the caster's actor; then
/// `ActiveEffect::Finalize`.
pub fn value_modifier_effect_finalize(e: &mut Engine, this: Ptr<ActiveEffect>) {
    let target = target_actor(e, this);
    let value = e.vcall(this.addr(), 0x60, &[]).u32();
    let effect = e.get(this, ActiveEffect::pEffect);
    let setting = e.call(EFFECT_ITEM_SETTING, &args![effect]).u32();
    if e.call(SETTING_REVERSES, &args![setting, value]).bool() {
        let negated = x87_quiet_bits(magnitude_bits(e, this)) ^ SIGN;
        set_magnitude_bits(e, this, negated);
        if target != 0 && e.vcall(this.addr(), 0x60, &[]).u32() == HEALTH {
            let item = e.call(WORD_AT_C, &args![this]).u32();
            if e.call(WORD_AT_C, &args![item]).u32() != 0 {
                let killer = caster_actor(e, this);
                let magnitude = x87_quiet_bits(magnitude_bits(e, this));
                let adjusted = e
                    .call(DIFFICULTY_ADJUST, &args![target, magnitude, killer])
                    .f64();
                set_magnitude_bits(e, this, x87_store32(adjusted));
            }
        }
    }
    e.call(ACTIVE_EFFECT_FINALIZE, &args![this]);
}

// Translated from 0082be10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ValueModifierEffect::ShowAttributeReductionMessage` (Xbox PDB): for a
/// hostile effect on the player's magic target, of an actor value in one
/// of two classes (`005d1260`, `0047f060`), from an item whose type is not
/// 4 and whose setting has flag 4, shows `"<word> <value name>
/// <ending>"` for 2.0 (the ending by setting flag 2 or `00804b60`).
// Translation incomplete: 0082be10 - the /GS stack cookie check (00ec408c) is not modelled.
pub fn value_modifier_effect_show_attribute_reduction_message(
    e: &mut Engine,
    this: Ptr<ActiveEffect>,
) {
    let player = e.global::<u32>(PLAYER);
    let player_target = if player == 0 {
        0
    } else {
        player.wrapping_add(0x94)
    };
    if e.call(EFFECT_TARGET, &args![this]).u32() != player_target {
        return;
    }
    let effect = e.get(this, ActiveEffect::pEffect);
    let value = e.call(EFFECT_ITEM_ACTOR_VALUE, &args![effect]).u32();
    if !e.call(VALUE_TEST_A, &args![value]).bool() {
        let effect = e.get(this, ActiveEffect::pEffect);
        let value = e.call(EFFECT_ITEM_ACTOR_VALUE, &args![effect]).u32();
        if !e.call(VALUE_TEST_C, &args![value]).bool() {
            return;
        }
    }
    let effect = e.get(this, ActiveEffect::pEffect);
    if !e.call(EFFECT_ITEM_IS_HOSTILE, &args![effect]).bool() {
        return;
    }
    let item = e.call(EFFECT_ITEM, &args![this]).u32();
    if e.vcall(item, 0x18, &[]).u32() == 4 {
        return;
    }
    if !setting_flag(e, this, 4) {
        return;
    }
    let first_ending = setting_flag(e, this, 2) || e.call(EFFECT_TEST, &args![this]).bool();
    let ending = if first_ending {
        e.call(SETTING_STRING, &args![MESSAGE_ENDING_A]).u32()
    } else {
        e.call(SETTING_STRING, &args![MESSAGE_ENDING_B]).u32()
    };
    let effect = e.get(this, ActiveEffect::pEffect);
    let value = e.call(EFFECT_ITEM_ACTOR_VALUE, &args![effect]).u32();
    let name = e.call(ACTOR_VALUE_NAME, &args![value]).u32();
    let first = e.call(SETTING_STRING, &args![MESSAGE_FIRST]).u32();
    e.with_stack(200, |e, text| {
        e.call(SPRINTF, &args![text, TEXT_FORMAT, first, name, ending]);
        let time = x87_quiet_bits(e.global::<u32>(MESSAGE_TIME));
        e.call(SHOW_MESSAGE, &args![text, 0u32, 0u32, 0u32, time, 0u32]);
    });
}

// Translated from 0082c000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ValueModifierEffect::Start` (Xbox PDB): an effect with setting flag 2,
/// or without duration, `00804b60` and kinds 4 and 1, applies its
/// magnitude now: limited by `CheckAdjustNoNegativeMagnitude`, counted in
/// the player's healing stats for an alchemy health effect, killing a
/// spent target of a negative actor value 7, then `ModifyActorValue` (slot
/// +0x64). Other effects only count the player's healing stats (duration
/// times magnitude). A radiation effect with setting flag 4 adds its
/// radiation to the target's process and updates the player; then the
/// reduction message.
pub fn value_modifier_effect_start(e: &mut Engine, this: Ptr<ActiveEffect>) {
    let target = target_actor(e, this);
    let now = if setting_flag(e, this, 2) {
        true
    } else {
        let effect = e.get(this, ActiveEffect::pEffect);
        !(e.call(EFFECT_ITEM_DURATION, &args![effect]).u32() != 0
            || e.call(EFFECT_TEST, &args![this]).bool()
            || e.call(EFFECT_KIND, &args![this]).u32() == 4
            || e.call(EFFECT_KIND, &args![this]).u32() == 1)
    };
    let player = e.global::<u32>(PLAYER);
    if now {
        let magnitude = x87_quiet_bits(magnitude_bits(e, this));
        let limited = value_modifier_effect_check_adjust_no_negative_magnitude(
            e,
            this,
            f32::from_bits(magnitude),
        );
        set_magnitude_bits(e, this, x87_store32(limited));
        if which(e, this) == HEALTH && target == e.global::<u32>(PLAYER) {
            let lost = player_health_lost(e);
            alchemy_stats(e, this, lost, |e, this| {
                x87_quiet_bits(magnitude_bits(e, this))
            });
        }
        if which(e, this) == 7 && x87_m32(magnitude_bits(e, this)) < e.global::<f64>(ZERO) {
            kill_when_spent(e, this, target);
        }
        let magnitude = x87_quiet_bits(magnitude_bits(e, this));
        e.vcall(this.addr(), 0x64, &args![target, magnitude, 0xffff_ffffu32]);
    } else if target == player {
        let effect = e.get(this, ActiveEffect::pEffect);
        if e.call(EFFECT_ITEM_DURATION, &args![effect]).i32() > 0 && which(e, this) == HEALTH {
            let lost = player_health_lost(e);
            alchemy_stats(e, this, lost, |e, this| {
                let effect = e.get(this, ActiveEffect::pEffect);
                let duration = e.call(EFFECT_ITEM_DURATION, &args![effect]).i32();
                x87_store32(x87_mul(
                    f64::from(duration),
                    x87_m32(magnitude_bits(e, this)),
                ))
            });
        }
    }
    radiation_step(e, this, target, 0x754);
    value_modifier_effect_show_attribute_reduction_message(e, this);
}

// Translated from 0082c660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ValueModifierEffect::Update` (Xbox PDB): unless the setting has flag 2,
/// an effect that lasts (`00804b60`, a positive duration, or kinds 4 and
/// 1) applies `magnitude * seconds` (`ModifyActorValue`); the player's
/// health effects log `"Using a food?"` or (for type 7 items) `"Using a
/// stimpak?"`; a negative actor value 7 kills a spent target.
pub fn value_modifier_effect_update(e: &mut Engine, this: Ptr<ActiveEffect>, seconds: f32) {
    if setting_flag(e, this, 2) {
        return;
    }
    let target = target_actor(e, this);
    if !e.call(EFFECT_TEST, &args![this]).bool() {
        let duration = x87_m32(e.get(this, ActiveEffect::fDuration).to_bits());
        // `TEST AH,0x41; JZ`: on when above 0.0.
        if !(duration > e.global::<f64>(ZERO)) {
            let kind = e.get(this, ActiveEffect::eSpellType);
            if kind != 4 && e.get(this, ActiveEffect::eSpellType) != 1 {
                return;
            }
        }
    }
    let amount = x87_store32(x87_mul(
        x87_m32(magnitude_bits(e, this)),
        x87_m32(seconds.to_bits()),
    ));
    e.vcall(this.addr(), 0x64, &args![target, amount, 0xffff_ffffu32]);
    if which(e, this) == HEALTH && target == e.global::<u32>(PLAYER) {
        let item = e.call(EFFECT_ITEM, &args![this]).u32();
        if item != 0 && e.vcall(item.wrapping_add(0xc), 0x4, &[]).bool() {
            e.call(PRINT, &args![TEXT_FOOD]);
        } else if e.vcall(item, 0x18, &[]).u32() == 7 {
            e.call(PRINT, &args![TEXT_STIMPAK]);
        }
    }
    if which(e, this) == 7 && x87_m32(magnitude_bits(e, this)) < e.global::<f64>(ZERO) {
        kill_when_spent(e, this, target);
    }
}

// Translated from 0082c810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ValueModifierEffect::Finish` (Xbox PDB): for an effect with setting
/// flag 2, takes back what it gave: the part of a positive magnitude that
/// `CheckAdjustNoNegativeMagnitude` keeps (plus one for health when the
/// target's health would end at most at the magnitude plus one) is
/// restored as damage (slot +0x3AC) when not zero, the magnitude is
/// removed (`ModifyActorValue`), and an actor answering slot +0x22C gets
/// its remaining health (or value 7) damage taken back. A radiation effect
/// with setting flag 4 removes its radiation and updates the player.
pub fn value_modifier_effect_finish(e: &mut Engine, this: Ptr<ActiveEffect>) {
    let target = target_actor(e, this);
    if setting_flag(e, this, 2) {
        let zero = e.global::<f64>(ZERO);
        if target != 0 && x87_m32(magnitude_bits(e, this)) > zero {
            let negated = x87_quiet_bits(magnitude_bits(e, this)) ^ SIGN;
            let kept = value_modifier_effect_check_adjust_no_negative_magnitude(
                e,
                this,
                f32::from_bits(negated),
            );
            let mut restored = x87_store32(x87_add(kept, x87_m32(magnitude_bits(e, this))));
            if e.vcall(this.addr(), 0x60, &[]).u32() == HEALTH {
                let health = e.call(ACTOR_HEALTH, &args![target]).f64();
                let ending = x87_add(health, x87_m32(restored));
                let limit = x87_add(x87_m32(magnitude_bits(e, this)), 1.0);
                // `FCOMPP; TEST AH,1; JNZ`: on when the limit is not below.
                if limit >= ending {
                    restored = x87_store32(x87_add(x87_m32(restored), e.global::<f64>(ONE)));
                }
            }
            // `TEST AH,0x44; JNP`: skipped only when equal to 0.0.
            if x87_m32(restored) != zero {
                let killer = caster_actor(e, this);
                let value = which(e, this);
                e.vcall(target, 0x3ac, &args![value, restored, killer]);
            }
        }
        let negated = x87_quiet_bits(magnitude_bits(e, this)) ^ SIGN;
        e.vcall(this.addr(), 0x64, &args![target, negated, 0xffff_ffffu32]);
        if target != 0
            && e.vcall(target, 0x22c, &args![0u32]).bool()
            && (which(e, this) == HEALTH || which(e, this) == 7)
        {
            let value = e
                .vcall(target.wrapping_add(0xa4), 0x8, &args![HEALTH])
                .i32();
            let value = x87_store32(f64::from(value));
            if x87_m32(value) > zero {
                let negated = x87_quiet_bits(value) ^ SIGN;
                e.vcall(target, 0x3ac, &args![HEALTH, negated, 0u32]);
            }
        }
    }
    radiation_step(e, this, target, 0x758);
}

// Translated from 0082ca50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ValueModifierEffect::ModifyActorValue` (Xbox PDB): changes actor value
/// `value_index` (-1: the effect's own) of `target` by `amount`. Without a
/// value it logs (unless the amount is 0) and stops. Kind-4 plain effects
/// with setting flag 2 on an actor (slot +0x360) go through slot +0x3A4;
/// other flag-2 effects through slot +0x39C; the rest through slot +0x3AC
/// with the hostile caster's actor, after tracking the player's damage on
/// a non-essential actor's process (slot +0x72C) and before sending the
/// health-damage event (`0087a8b0`, 0x116C).
pub fn value_modifier_effect_modify_actor_value(
    e: &mut Engine,
    this: Ptr<ActiveEffect>,
    target: u32,
    amount: f32,
    value_index: i32,
) {
    if target == 0 {
        return;
    }
    let index = if value_index == -1 {
        which(e, this)
    } else {
        value_index as u32
    };
    let zero = e.global::<f64>(ZERO);
    let amount_bits = amount.to_bits();
    if index == 0xffff_ffff {
        if x87_m32(amount_bits) != zero {
            e.call(PRINT, &args![TEXT_NO_ACTOR_VALUE]);
        }
        return;
    }
    let plain = e.get(this, ActiveEffect::eSpellType) == 4
        && !e.call(EFFECT_NOT_PLAIN, &args![this]).bool();
    if e.vcall(target, 0x360, &[]).bool() && plain && setting_flag(e, this, 2) {
        let amount = x87_quiet_bits(amount_bits);
        e.vcall(target, 0x3a4, &args![index, amount, 0u32]);
        return;
    }
    let mut killer = caster_actor(e, this);
    let item = e.call(WORD_AT_C, &args![this]).u32();
    let keep = if e.call(WORD_AT_C, &args![item]).u32() != 0 {
        let item = e.call(WORD_AT_C, &args![this]).u32();
        e.call(EFFECT_ITEM_IS_HOSTILE, &args![item]).bool()
    } else {
        false
    };
    if !keep {
        killer = 0;
    }
    if setting_flag(e, this, 2) {
        let amount = x87_quiet_bits(amount_bits);
        e.vcall(target, 0x39c, &args![index, amount, killer]);
        return;
    }
    if killer != 0
        && x87_m32(amount_bits) < zero
        && index == HEALTH
        && e.vcall(killer, 0x360, &[]).bool()
        && target != killer
        && e.call(ACTOR_PROCESS, &args![target]).u32() != 0
        && !e.call(ACTOR_GET_ESSENTIAL, &args![target]).bool()
    {
        let process = e.call(ACTOR_PROCESS, &args![target]).u32();
        let damage = x87_quiet_bits(amount_bits) ^ SIGN;
        let player = e.global::<u32>(PLAYER);
        e.vcall(process, 0x72c, &args![player, damage]);
    }
    let amount = x87_quiet_bits(amount_bits);
    e.vcall(target, 0x3ac, &args![index, amount, killer]);
    if x87_m32(amount_bits) < zero && index == HEALTH {
        let damage = -x87_load32(amount_bits);
        let source = e.call(EVENT_SOURCE, &[]).u32();
        e.call(SEND_EVENT, &args![source, 0x116cu32, target, damage]);
    }
}

// Translated from 0082ce70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ValueModifierEffect::CheckAdjustNoNegativeMagnitude` (Xbox PDB): for
/// an effect that is not plain (`007df210`) and modifies a value of one of
/// three classes (`005d1260`, `0066fd90`, `0047f060`), a non-positive
/// `amount` is limited so that the target's value does not go below 0;
/// returns the amount in `ST0`.
pub fn value_modifier_effect_check_adjust_no_negative_magnitude(
    e: &mut Engine,
    this: Ptr<ActiveEffect>,
    amount: f32,
) -> f64 {
    let mut kept = x87_quiet_bits(amount.to_bits());
    if e.call(EFFECT_NOT_PLAIN, &args![this]).bool() {
        return x87_load32(kept);
    }
    let in_class = {
        let value = e.vcall(this.addr(), 0x60, &[]).u32();
        e.call(VALUE_TEST_A, &args![value]).bool() || {
            let value = e.vcall(this.addr(), 0x60, &[]).u32();
            e.call(VALUE_TEST_B, &args![value]).bool() || {
                let value = e.vcall(this.addr(), 0x60, &[]).u32();
                e.call(VALUE_TEST_C, &args![value]).bool()
            }
        }
    };
    // `TEST AH,0x41; JP`: on when not above 0.0 (and not NaN).
    if in_class && x87_m32(amount.to_bits()) <= e.global::<f64>(ZERO) {
        let target = target_actor(e, this);
        let value = e.vcall(this.addr(), 0x60, &[]).u32();
        let current = e.vcall(target.wrapping_add(0xa4), 0xc, &args![value]).f64();
        let ending = x87_store32(x87_add(current, x87_m32(kept)));
        if x87_m32(ending) < e.global::<f64>(ZERO) {
            kept = x87_store32(x87_add(-x87_load32(ending), x87_m32(kept)));
        }
    }
    x87_load32(kept)
}

/// The effect's effect setting (`007df1d0`, thiscall on the effect;
/// `statsmenu.cpp` range).
const EFFECT_SETTING_007DF1D0: u32 = 0x007d_f1d0;
/// `ActiveEffect::FinishInitLoadGame` (engine map), thiscall (word), `RET 4`.
const ACTIVE_EFFECT_FINISH_INIT_LOAD_GAME: u32 = 0x0080_5760;
/// The save/load singleton (pointer global) and a byte of it
/// (`008df040`, `highprocess.cpp` range) compared with 0x5f and 0x62.
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
const SAVE_LOAD_BYTE_AT_80: u32 = 0x008d_f040;
/// The target actor's virtual slots `FinishInitLoadGame` uses: +0x360
/// (`AL`), +0x3b4 and +0x39c (actor value, value[, caster]).
const SLOT_ACTOR_360: u32 = 0x360;
const SLOT_ACTOR_3B4: u32 = 0x3b4;
const SLOT_ACTOR_39C: u32 = 0x39c;
/// `ValueModifierEffect`'s own virtual slot +0x60 (`GetActorValue`, Xbox
/// PDB order).
const SLOT_GET_ACTOR_VALUE: u32 = 0x60;

// Translated from 0082c490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ValueModifierEffect::Continue` (engine map; `thiscall`, plain `RET`):
/// for a health effect (`eWhichActorValue` 0x10) on the player with an
/// effect item of positive duration (`EffectItem::GetDuration`), the
/// alchemy stats are kept as `Start` keeps them (stats 0x12 and 0x10 by the
/// alchemy item's flags 2 and 4), with the duration times the magnitude
/// (`FILD`, `FMUL float`, stored as a `float`) against the player's health
/// below its permanent value.
pub fn value_modifier_effect_continue(e: &mut Engine, this: Ptr<ActiveEffect>) {
    let target = target_actor(e, this);
    if target != e.global::<u32>(PLAYER) {
        return;
    }
    let effect = e.get(this, ActiveEffect::pEffect);
    if effect == 0 || e.call(EFFECT_ITEM_DURATION, &args![effect]).i32() <= 0 {
        return;
    }
    if which(e, this) != HEALTH {
        return;
    }
    let lost = player_health_lost(e);
    alchemy_stats(e, this, x87_quiet_bits(lost), |e, this| {
        let effect = e.get(this, ActiveEffect::pEffect);
        let duration = e.call(EFFECT_ITEM_DURATION, &args![effect]).i32();
        let amount = x87_store32(x87_mul(
            f64::from(duration),
            x87_m32(magnitude_bits(e, this)),
        ));
        x87_quiet_bits(amount)
    });
}

// Translated from 0082ccb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ValueModifierEffect::IsCausingDamage` (engine map; `thiscall`, plain
/// `RET`): whether it is a health effect (`eWhichActorValue` 0x10) whose
/// effect setting (`007df1d0`, asked twice) has flag 4 and not flag 2.
pub fn value_modifier_effect_is_causing_damage(e: &mut Engine, this: Ptr<ActiveEffect>) -> bool {
    if which(e, this) != HEALTH {
        return false;
    }
    let setting = e.call(EFFECT_SETTING_007DF1D0, &args![this]).u32();
    if !e.call(SETTING_HAS_FLAG, &args![setting, 4u32]).bool() {
        return false;
    }
    let setting = e.call(EFFECT_SETTING_007DF1D0, &args![this]).u32();
    !e.call(SETTING_HAS_FLAG, &args![setting, 2u32]).bool()
}

// Translated from 0082cd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ValueModifierEffect::FinishInitLoadGame` (engine map; `thiscall`, `RET
/// 4`): `ActiveEffect::FinishInitLoadGame(word)`; then, when the save/load
/// singleton's byte (`008df040`, read twice) is from 0x5f to 0x61, for a
/// target actor whose virtual slot +0x360 answers true, an effect
/// `00804b60` accepts and an effect setting with flag 2, the target's
/// slot +0x3b4 runs with the effect's actor value (slot +0x60) and minus
/// the magnitude (`00495460`, `FCHS`), then slot +0x39c with the actor
/// value, the magnitude and the caster actor (or 0).
pub fn value_modifier_effect_finish_init_load_game(
    e: &mut Engine,
    this: Ptr<ActiveEffect>,
    word: u32,
) {
    e.call(ACTIVE_EFFECT_FINISH_INIT_LOAD_GAME, &args![this, word]);
    let game = e.global::<u32>(SAVE_LOAD_GAME);
    if e.call(SAVE_LOAD_BYTE_AT_80, &args![game]).u8() < 0x5f {
        return;
    }
    let game = e.global::<u32>(SAVE_LOAD_GAME);
    if e.call(SAVE_LOAD_BYTE_AT_80, &args![game]).u8() >= 0x62 {
        return;
    }
    let target = target_actor(e, this);
    if target == 0 || !e.vcall(target, SLOT_ACTOR_360, &args![]).bool() {
        return;
    }
    if !e.call(EFFECT_TEST, &args![this]).bool() || !setting_flag(e, this, 2) {
        return;
    }
    let caster = caster_actor(e, this);
    let magnitude = e.call(RADIATION_AMOUNT, &args![this]).f64();
    let negated = x87_store32(-magnitude);
    let actor_value = e.vcall(this.addr(), SLOT_GET_ACTOR_VALUE, &args![]).u32();
    e.vcall(target, SLOT_ACTOR_3B4, &args![actor_value, negated]);
    let magnitude = x87_store32(e.call(RADIATION_AMOUNT, &args![this]).f64());
    let actor_value = e.vcall(this.addr(), SLOT_GET_ACTOR_VALUE, &args![]).u32();
    e.vcall(
        target,
        SLOT_ACTOR_39C,
        &args![actor_value, magnitude, caster],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0082_c490,
            value_modifier_effect_continue(Ptr<ActiveEffect>)
        ),
        entry!(
            0x0082_ccb0,
            value_modifier_effect_is_causing_damage(Ptr<ActiveEffect>) -> bool
        ),
        entry!(
            0x0082_cd10,
            value_modifier_effect_finish_init_load_game(Ptr<ActiveEffect>, u32)
        ),
        entry!(
            0x0082_bb30,
            value_modifier_effect_copy(Ptr<ActiveEffect>) -> u32
        ),
        entry!(
            0x0082_bbe0,
            value_modifier_effect_copy_data(Ptr<ActiveEffect>, u32)
        ),
        entry!(0x0082_bc30, fn_0082bc30(u32, u32, u32) -> u32),
        entry!(
            0x0082_bcb0,
            value_modifier_effect_check_target(Ptr<ActiveEffect>, u32) -> bool
        ),
        entry!(
            0x0082_bd20,
            value_modifier_effect_finalize(Ptr<ActiveEffect>)
        ),
        entry!(
            0x0082_be10,
            value_modifier_effect_show_attribute_reduction_message(Ptr<ActiveEffect>)
        ),
        entry!(0x0082_c000, value_modifier_effect_start(Ptr<ActiveEffect>)),
        entry!(
            0x0082_c660,
            value_modifier_effect_update(Ptr<ActiveEffect>, f32)
        ),
        entry!(0x0082_c810, value_modifier_effect_finish(Ptr<ActiveEffect>)),
        entry!(
            0x0082_ca50,
            value_modifier_effect_modify_actor_value(Ptr<ActiveEffect>, u32, f32, i32)
        ),
        entry!(
            0x0082_ce70,
            value_modifier_effect_check_adjust_no_negative_magnitude(Ptr<ActiveEffect>, f32) -> f64
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn float(st0: f64) -> Ret {
        Ret {
            st0,
            ..Ret::default()
        }
    }

    fn returns(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| ret(value));
    }

    fn returns_float(e: &mut Engine, addr: u32, value: f64) {
        e.register_double(addr, move |_, _| float(value));
    }

    // Slot doubles of the effect, the target actor and its value owner.
    const EFFECT_SLOT_COPY_DATA: u32 = 0x00c1_0044;
    const EFFECT_SLOT_VALUE: u32 = 0x00c1_0060;
    const EFFECT_SLOT_MODIFY: u32 = 0x00c1_0064;
    const ACTOR_SLOT_22C: u32 = 0x00c4_022c;
    const ACTOR_SLOT_360: u32 = 0x00c4_0360;
    const ACTOR_SLOT_39C: u32 = 0x00c4_039c;
    const ACTOR_SLOT_3A4: u32 = 0x00c4_03a4;
    const ACTOR_SLOT_3AC: u32 = 0x00c4_03ac;
    const OWNER_SLOT_VALUE: u32 = 0x00c6_0008;
    const OWNER_SLOT_FLOAT: u32 = 0x00c6_000c;
    const OWNER_SLOT_PERMANENT: u32 = 0x00c6_0020;

    fn table(e: &mut Engine, object: u32, at: u32, slots: &[(u32, u32)]) {
        let mut words = vec![0u32; 0x200];
        for (offset, target) in slots {
            words[(*offset / 4) as usize] = *target;
        }
        e.put_vtable(at, &words);
        e.mem.set_u32(object, at);
    }

    struct Fixture {
        e: Engine,
        effect: Ptr<ActiveEffect>,
        target: u32,
        flags: Rc<Cell<u32>>,
    }

    /// An effect (actor value at +0x48, slot +0x60 reading it) on a target
    /// actor whose target and caster accessors are the identity; the
    /// setting's flags come from `flags`.
    fn fixture() -> Fixture {
        let mut e = Engine::new();
        for page in [0x0101_2000, 0x0101_6000, 0x011d_e000] {
            e.map(page, 0x1000);
        }
        e.set_global(ZERO, 0.0f64);
        e.set_global(ONE, 1.0f64);
        e.set_global(MESSAGE_TIME, 2.0f32);
        let effect = Ptr::<ActiveEffect>::new(e.mem.alloc(0x4c));
        table(
            &mut e,
            effect.addr(),
            0x00c0_0000,
            &[
                (0x44, EFFECT_SLOT_COPY_DATA),
                (0x60, EFFECT_SLOT_VALUE),
                (0x64, EFFECT_SLOT_MODIFY),
            ],
        );
        e.register(EFFECT_SLOT_VALUE, |e, a| ret(e.mem.u32(a[0] + 0x48)));
        e.register(EFFECT_SLOT_MODIFY, |_, _| Ret::default());
        e.register(EFFECT_SLOT_COPY_DATA, |_, _| Ret::default());
        let target = e.mem.alloc(0x200);
        table(
            &mut e,
            target,
            0x00c3_0000,
            &[
                (0x22c, ACTOR_SLOT_22C),
                (0x360, ACTOR_SLOT_360),
                (0x39c, ACTOR_SLOT_39C),
                (0x3a4, ACTOR_SLOT_3A4),
                (0x3ac, ACTOR_SLOT_3AC),
            ],
        );
        for slot in [ACTOR_SLOT_39C, ACTOR_SLOT_3A4, ACTOR_SLOT_3AC] {
            returns(&mut e, slot, 0);
        }
        returns(&mut e, ACTOR_SLOT_22C, 0);
        returns(&mut e, ACTOR_SLOT_360, 0);
        table(
            &mut e,
            target + 0xa4,
            0x00c5_0000,
            &[
                (0x8, OWNER_SLOT_VALUE),
                (0xc, OWNER_SLOT_FLOAT),
                (0x20, OWNER_SLOT_PERMANENT),
            ],
        );
        e.set(effect, ActiveEffect::pTarget, target);
        e.set(effect, ActiveEffect::pEffect, 0x00c7_0000);
        e.register(EFFECT_TARGET, |e, a| ret(e.mem.u32(a[0] + 0x24)));
        e.register(TARGET_AS_ACTOR, |_, a| ret(a[0]));
        e.register(EFFECT_CASTER, |e, a| ret(e.mem.u32(a[0] + 0x28)));
        e.register(CASTER_AS_ACTOR, |_, a| ret(a[0]));
        returns(&mut e, EFFECT_ITEM_SETTING, 0x00c2_0000);
        let flags = Rc::new(Cell::new(0u32));
        let f = flags.clone();
        e.register_double(SETTING_HAS_FLAG, move |_, a| {
            ret(u32::from(f.get() & a[1] != 0))
        });
        returns(&mut e, EFFECT_NOT_PLAIN, 1);
        returns(&mut e, EFFECT_TEST, 0);
        Fixture {
            e,
            effect,
            target,
            flags,
        }
    }

    fn set_which(e: &mut Engine, effect: Ptr<ActiveEffect>, value: u32) {
        e.mem.set_u32(effect.addr() + 0x48, value);
    }

    #[test]
    fn copy_builds_a_twin() {
        let Fixture { mut e, effect, .. } = fixture();
        returns(&mut e, OPERATOR_NEW, 0x00c8_0000);
        e.register(VALUE_MODIFIER_EFFECT_CONSTRUCT, |_, a| ret(a[0]));
        e.set(effect, ActiveEffect::pCaster, 0x11);
        e.set(effect, ActiveEffect::pSpell, 0x22);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0082_bb30, &args![effect]).u32(), 0x00c8_0000);
        assert_eq!(
            calls_to(&e, VALUE_MODIFIER_EFFECT_CONSTRUCT),
            vec![vec![0x00c8_0000, 0x11, 0x22, 0x00c7_0000]]
        );
        assert_eq!(
            calls_to(&e, EFFECT_SLOT_COPY_DATA),
            vec![vec![effect.addr(), 0x00c8_0000]]
        );
        returns(&mut e, OPERATOR_NEW, 0);
        assert_eq!(value_modifier_effect_copy(&mut e, effect), 0);
        assert_eq!(
            calls_to(&e, EFFECT_SLOT_COPY_DATA)[1],
            vec![effect.addr(), 0]
        );
    }

    #[test]
    fn copy_data_takes_the_actor_value() {
        let Fixture { mut e, effect, .. } = fixture();
        returns(&mut e, ACTIVE_EFFECT_COPY_DATA, 0);
        let other = e.mem.alloc(0x4c);
        returns(&mut e, RT_DYNAMIC_CAST, other);
        set_which(&mut e, effect, 0x2b);
        e.call(0x0082_bbe0, &args![effect, other]);
        assert_eq!(e.mem.u32(other + 0x48), 0x2b);
        returns(&mut e, RT_DYNAMIC_CAST, 0);
        set_which(&mut e, effect, 0x2c);
        value_modifier_effect_copy_data(&mut e, effect, other);
        assert_eq!(e.mem.u32(other + 0x48), 0x2b);
    }

    #[test]
    fn create_or_null() {
        let Fixture { mut e, .. } = fixture();
        returns(&mut e, OPERATOR_NEW, 0x00c8_0000);
        e.register(VALUE_MODIFIER_EFFECT_CONSTRUCT, |_, a| ret(a[0] + 1));
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0082_bc30, &args![1u32, 2u32, 3u32]).u32(),
            0x00c8_0001
        );
        assert_eq!(
            calls_to(&e, VALUE_MODIFIER_EFFECT_CONSTRUCT),
            vec![vec![0x00c8_0000, 1, 2, 3]]
        );
        returns(&mut e, OPERATOR_NEW, 0);
        assert_eq!(fn_0082bc30(&mut e, 1, 2, 3), 0);
    }

    #[test]
    fn check_target_wants_an_actor_answering_false() {
        let Fixture {
            mut e,
            effect,
            target,
            ..
        } = fixture();
        assert!(!e.call(0x0082_bcb0, &args![effect, 0u32]).bool());
        assert!(value_modifier_effect_check_target(&mut e, effect, target));
        returns(&mut e, ACTOR_SLOT_22C, 1);
        assert!(!value_modifier_effect_check_target(&mut e, effect, target));
    }

    #[test]
    fn finalize_reverses_and_adjusts_health() {
        let Fixture {
            mut e,
            effect,
            target,
            ..
        } = fixture();
        returns(&mut e, SETTING_REVERSES, 1);
        returns(&mut e, ACTIVE_EFFECT_FINALIZE, 0);
        e.register(WORD_AT_C, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        let item = e.mem.alloc(0x10);
        e.mem.set_u32(item + 0xc, 1);
        e.set(effect, ActiveEffect::pEffect, item);
        set_which(&mut e, effect, HEALTH);
        e.set(effect, ActiveEffect::fMagnitude, 5.0);
        returns_float(&mut e, DIFFICULTY_ADJUST, 7.5);
        e.call_log = Some(vec![]);
        e.call(0x0082_bd20, &args![effect]);
        assert_eq!(
            calls_to(&e, DIFFICULTY_ADJUST),
            vec![vec![target, (-5.0f32).to_bits(), 0]]
        );
        assert_eq!(e.get(effect, ActiveEffect::fMagnitude), 7.5);
        assert_eq!(calls_to(&e, ACTIVE_EFFECT_FINALIZE).len(), 1);
        // Not reversed: unchanged.
        returns(&mut e, SETTING_REVERSES, 0);
        value_modifier_effect_finalize(&mut e, effect);
        assert_eq!(e.get(effect, ActiveEffect::fMagnitude), 7.5);
    }

    #[test]
    fn reduction_message_for_the_player() {
        let Fixture {
            mut e,
            effect,
            flags,
            ..
        } = fixture();
        let player = e.mem.alloc(0x200);
        e.set_global(PLAYER, player);
        e.set(effect, ActiveEffect::pTarget, player + 0x94);
        returns(&mut e, EFFECT_ITEM_ACTOR_VALUE, 5);
        returns(&mut e, VALUE_TEST_A, 1);
        returns(&mut e, EFFECT_ITEM_IS_HOSTILE, 1);
        let item = e.mem.alloc(8);
        table(&mut e, item, 0x00c9_0000, &[(0x18, 0x00c9_1018)]);
        returns(&mut e, 0x00c9_1018, 2);
        returns(&mut e, EFFECT_ITEM, item);
        e.register(SETTING_STRING, |_, a| ret(a[0] + 1));
        returns(&mut e, ACTOR_VALUE_NAME, 0x00ca_0000);
        returns(&mut e, SPRINTF, 0);
        returns(&mut e, SHOW_MESSAGE, 0);
        flags.set(4);
        e.call_log = Some(vec![]);
        e.call(0x0082_be10, &args![effect]);
        let sprintf = calls_to(&e, SPRINTF);
        assert_eq!(sprintf.len(), 1);
        assert_eq!(
            &sprintf[0][1..],
            &[
                TEXT_FORMAT,
                MESSAGE_FIRST + 1,
                0x00ca_0000,
                MESSAGE_ENDING_B + 1
            ]
        );
        let show = calls_to(&e, SHOW_MESSAGE);
        assert_eq!(&show[0][1..], &[0, 0, 0, 2.0f32.to_bits(), 0]);
        assert_eq!(show[0][0], sprintf[0][0]);
        // With setting flag 2 the other ending.
        flags.set(6);
        e.call_log = Some(vec![]);
        value_modifier_effect_show_attribute_reduction_message(&mut e, effect);
        assert_eq!(calls_to(&e, SPRINTF)[0][4], MESSAGE_ENDING_A + 1);
        // Not on the player: nothing.
        e.set(effect, ActiveEffect::pTarget, 0x1234);
        e.call_log = Some(vec![]);
        value_modifier_effect_show_attribute_reduction_message(&mut e, effect);
        assert!(calls_to(&e, SPRINTF).is_empty());
    }

    #[test]
    fn start_applies_now_and_kills_a_spent_target() {
        let Fixture {
            mut e,
            effect,
            target,
            flags,
        } = fixture();
        flags.set(2);
        set_which(&mut e, effect, 7);
        e.set(effect, ActiveEffect::fMagnitude, -5.0);
        e.set(effect, ActiveEffect::pCaster, 0x99);
        returns_float(&mut e, OWNER_SLOT_FLOAT, 0.5);
        returns(&mut e, ACTOR_KILL, 0);
        e.call_log = Some(vec![]);
        e.call(0x0082_c000, &args![effect]);
        assert_eq!(
            calls_to(&e, ACTOR_KILL),
            vec![vec![target, 0x99, 0.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, EFFECT_SLOT_MODIFY),
            vec![vec![
                effect.addr(),
                target,
                (-5.0f32).to_bits(),
                0xffff_ffff
            ]]
        );
        // Health above 1.0: no kill.
        returns_float(&mut e, OWNER_SLOT_FLOAT, 1.5);
        e.call_log = Some(vec![]);
        value_modifier_effect_start(&mut e, effect);
        assert!(calls_to(&e, ACTOR_KILL).is_empty());
    }

    #[test]
    fn start_counts_the_players_healing_over_time() {
        let Fixture {
            mut e,
            effect,
            target,
            ..
        } = fixture();
        e.set_global(PLAYER, target);
        set_which(&mut e, effect, HEALTH);
        e.set(effect, ActiveEffect::fMagnitude, 3.0);
        returns(&mut e, EFFECT_ITEM_DURATION, 4);
        returns_float(&mut e, OWNER_SLOT_PERMANENT, 100.0);
        returns_float(&mut e, OWNER_SLOT_FLOAT, 60.0);
        let item = e.mem.alloc(8);
        table(&mut e, item, 0x00c9_0000, &[(0x18, 0x00c9_1018)]);
        returns(&mut e, 0x00c9_1018, 7);
        returns(&mut e, EFFECT_ITEM, item);
        returns(&mut e, RT_DYNAMIC_CAST, 0x00cb_0000);
        e.register(ALCHEMY_HAS_FLAG, |_, a| ret(u32::from(a[1] == 2)));
        returns_float(&mut e, STAT_AMOUNT, 6.5);
        e.register(FTOL, |_, a| ret(f64::take(a, &mut 0) as i32 as u32));
        returns(&mut e, MOD_STAT, 0);
        e.call_log = Some(vec![]);
        value_modifier_effect_start(&mut e, effect);
        assert_eq!(
            calls_to(&e, STAT_AMOUNT),
            vec![vec![12.0f32.to_bits(), 40.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, MOD_STAT), vec![vec![0x12, 6]]);
        assert!(calls_to(&e, EFFECT_SLOT_MODIFY).is_empty());
    }

    #[test]
    fn update_applies_magnitude_times_seconds() {
        let Fixture {
            mut e,
            effect,
            target,
            flags,
        } = fixture();
        e.set_global(PLAYER, target);
        set_which(&mut e, effect, HEALTH);
        e.set(effect, ActiveEffect::fMagnitude, 2.0);
        e.set(effect, ActiveEffect::eSpellType, 4);
        let item = e.mem.alloc(0x10);
        table(&mut e, item + 0xc, 0x00cc_0000, &[(0x4, 0x00cc_1004)]);
        returns(&mut e, 0x00cc_1004, 1);
        returns(&mut e, EFFECT_ITEM, item);
        returns(&mut e, PRINT, 0);
        e.call_log = Some(vec![]);
        e.call(0x0082_c660, &args![effect, 0.25f32]);
        assert_eq!(
            calls_to(&e, EFFECT_SLOT_MODIFY),
            vec![vec![effect.addr(), target, 0.5f32.to_bits(), 0xffff_ffff]]
        );
        assert_eq!(calls_to(&e, PRINT), vec![vec![TEXT_FOOD]]);
        // A finished effect of another kind: nothing.
        e.set(effect, ActiveEffect::eSpellType, 2);
        e.call_log = Some(vec![]);
        value_modifier_effect_update(&mut e, effect, 0.25);
        assert!(calls_to(&e, EFFECT_SLOT_MODIFY).is_empty());
        // Setting flag 2: nothing either.
        flags.set(2);
        e.set(effect, ActiveEffect::eSpellType, 4);
        value_modifier_effect_update(&mut e, effect, 0.25);
        assert!(calls_to(&e, EFFECT_SLOT_MODIFY).is_empty());
    }

    #[test]
    fn finish_takes_the_magnitude_back() {
        let Fixture {
            mut e,
            effect,
            target,
            flags,
        } = fixture();
        flags.set(2);
        set_which(&mut e, effect, 7);
        e.set(effect, ActiveEffect::fMagnitude, 10.0);
        returns(&mut e, ACTOR_SLOT_22C, 1);
        returns(&mut e, OWNER_SLOT_VALUE, 3);
        e.call_log = Some(vec![]);
        e.call(0x0082_c810, &args![effect]);
        // CheckAdjust keeps -10 (the effect is not plain): nothing restored.
        assert_eq!(
            calls_to(&e, EFFECT_SLOT_MODIFY),
            vec![vec![
                effect.addr(),
                target,
                (-10.0f32).to_bits(),
                0xffff_ffff
            ]]
        );
        assert_eq!(
            calls_to(&e, ACTOR_SLOT_3AC),
            vec![vec![target, HEALTH, (-3.0f32).to_bits(), 0]]
        );
        // Without setting flag 2: nothing.
        flags.set(0);
        e.call_log = Some(vec![]);
        value_modifier_effect_finish(&mut e, effect);
        assert!(calls_to(&e, EFFECT_SLOT_MODIFY).is_empty());
    }

    #[test]
    fn finish_restores_health_damage() {
        let Fixture {
            mut e,
            effect,
            target,
            flags,
        } = fixture();
        flags.set(2);
        set_which(&mut e, effect, HEALTH);
        e.set(effect, ActiveEffect::fMagnitude, 10.0);
        // A plain effect of a limited class: CheckAdjust gives -4.
        returns(&mut e, EFFECT_NOT_PLAIN, 0);
        returns(&mut e, VALUE_TEST_A, 1);
        returns_float(&mut e, OWNER_SLOT_FLOAT, 4.0);
        returns_float(&mut e, ACTOR_HEALTH, 2.0);
        e.set(effect, ActiveEffect::pCaster, 0x77);
        e.call_log = Some(vec![]);
        value_modifier_effect_finish(&mut e, effect);
        // 6 restored; 2 + 6 <= 11, so 7.
        assert_eq!(
            calls_to(&e, ACTOR_SLOT_3AC),
            vec![vec![target, HEALTH, 7.0f32.to_bits(), 0x77]]
        );
    }

    #[test]
    fn modify_actor_value_routes() {
        let Fixture {
            mut e,
            effect,
            target,
            flags,
        } = fixture();
        returns(&mut e, PRINT, 0);
        e.call_log = Some(vec![]);
        e.call(0x0082_ca50, &args![effect, 0u32, 1.0f32, 5i32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        // No value: logged unless the amount is 0.
        set_which(&mut e, effect, 0xffff_ffff);
        value_modifier_effect_modify_actor_value(&mut e, effect, target, 0.0, -1);
        value_modifier_effect_modify_actor_value(&mut e, effect, target, 2.0, -1);
        assert_eq!(calls_to(&e, PRINT), vec![vec![TEXT_NO_ACTOR_VALUE]]);
        // A plain kind-4 effect with flag 2 on an actor: slot +0x3A4.
        flags.set(2);
        returns(&mut e, EFFECT_NOT_PLAIN, 0);
        e.set(effect, ActiveEffect::eSpellType, 4);
        returns(&mut e, ACTOR_SLOT_360, 1);
        value_modifier_effect_modify_actor_value(&mut e, effect, target, 2.0, 9);
        assert_eq!(
            calls_to(&e, ACTOR_SLOT_3A4),
            vec![vec![target, 9, 2.0f32.to_bits(), 0]]
        );
        // Other flag-2 effects: slot +0x39C, without a hostile caster.
        returns(&mut e, ACTOR_SLOT_360, 0);
        e.register(WORD_AT_C, |_, _| ret(0));
        e.set(effect, ActiveEffect::pCaster, 0x55);
        value_modifier_effect_modify_actor_value(&mut e, effect, target, 2.0, 9);
        assert_eq!(
            calls_to(&e, ACTOR_SLOT_39C),
            vec![vec![target, 9, 2.0f32.to_bits(), 0]]
        );
    }

    #[test]
    fn modify_actor_value_damage_by_a_hostile_actor() {
        let Fixture {
            mut e,
            effect,
            target,
            ..
        } = fixture();
        let killer = e.mem.alloc(0x400);
        table(&mut e, killer, 0x00cd_0000, &[(0x360, 0x00cd_1360)]);
        returns(&mut e, 0x00cd_1360, 1);
        e.set(effect, ActiveEffect::pCaster, killer);
        e.register(WORD_AT_C, |_, _| ret(1));
        returns(&mut e, EFFECT_ITEM_IS_HOSTILE, 1);
        let process = e.mem.alloc(8);
        table(&mut e, process, 0x00ce_0000, &[(0x72c, 0x00ce_172c)]);
        returns(&mut e, 0x00ce_172c, 0);
        returns(&mut e, ACTOR_PROCESS, process);
        returns(&mut e, ACTOR_GET_ESSENTIAL, 0);
        e.set_global(PLAYER, 0x00cf_0000u32);
        returns(&mut e, EVENT_SOURCE, 0x00d0_0000);
        returns(&mut e, SEND_EVENT, 0);
        e.call_log = Some(vec![]);
        value_modifier_effect_modify_actor_value(&mut e, effect, target, -4.0, HEALTH as i32);
        assert_eq!(
            calls_to(&e, 0x00ce_172c),
            vec![vec![process, 0x00cf_0000, 4.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, ACTOR_SLOT_3AC),
            vec![vec![target, HEALTH, (-4.0f32).to_bits(), killer]]
        );
        let mut event = vec![0x00d0_0000, 0x116c, target];
        event.extend(args![4.0f64]);
        assert_eq!(calls_to(&e, SEND_EVENT), vec![event]);
    }

    #[test]
    fn check_adjust_keeps_the_value_from_going_negative() {
        let Fixture { mut e, effect, .. } = fixture();
        // Not plain: unchanged.
        assert_eq!(e.call(0x0082_ce70, &args![effect, -5.0f32]).f64(), -5.0);
        returns(&mut e, EFFECT_NOT_PLAIN, 0);
        returns(&mut e, VALUE_TEST_A, 0);
        returns(&mut e, VALUE_TEST_B, 1);
        returns_float(&mut e, OWNER_SLOT_FLOAT, 3.0);
        // 3 - 5 < 0: limited to -3.
        let limited =
            value_modifier_effect_check_adjust_no_negative_magnitude(&mut e, effect, -5.0);
        assert_eq!(limited, -3.0);
        // A positive amount is not limited.
        let kept = value_modifier_effect_check_adjust_no_negative_magnitude(&mut e, effect, 5.0);
        assert_eq!(kept, 5.0);
        // Outside the classes: unchanged.
        returns(&mut e, VALUE_TEST_B, 0);
        returns(&mut e, VALUE_TEST_C, 0);
        let kept = value_modifier_effect_check_adjust_no_negative_magnitude(&mut e, effect, -5.0);
        assert_eq!(kept, -5.0);
    }
}

#[cfg(test)]
mod n050_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    const SLOTS: u32 = 0x00f9_0000;

    /// An object whose vtable slots `slots` point at `SLOTS + slot`.
    fn with_vtable(e: &mut Engine, size: u32, slots: &[u32]) -> u32 {
        let object = e.mem.alloc(size);
        let vtable = e.mem.alloc(0x400);
        for slot in slots {
            e.mem.set_u32(vtable + slot, SLOTS + slot);
        }
        e.mem.set_u32(object, vtable);
        object
    }

    /// Target and caster: the effect's words +0x24 / +0x28 are the actors
    /// themselves.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        e.register(EFFECT_TARGET, |e, a| e.mem.u32(a[0] + 0x24).into_ret());
        e.register(EFFECT_CASTER, |e, a| e.mem.u32(a[0] + 0x28).into_ret());
        e.register(TARGET_AS_ACTOR, |_, a| a[0].into_ret());
        e.register(CASTER_AS_ACTOR, |_, a| a[0].into_ret());
        e
    }

    #[test]
    fn continue_keeps_the_alchemy_stats_for_the_player() {
        let mut e = engine();
        // The player: its value owner at +0xa4 answers 100 permanent and 70
        // current health.
        let player = e.mem.alloc(0xb0);
        let owner_vtable = e.mem.alloc(0x30);
        e.mem.set_u32(owner_vtable + 0x20, SLOTS + 0x20);
        e.mem.set_u32(owner_vtable + 0xc, SLOTS + 0xc);
        e.mem.set_u32(player + 0xa4, owner_vtable);
        e.register(SLOTS + 0x20, |_, _| Ret {
            st0: 100.0,
            ..Ret::default()
        });
        e.register(SLOTS + 0xc, |_, _| Ret {
            st0: 70.0,
            ..Ret::default()
        });
        e.set_global(PLAYER, player);
        let item = with_vtable(&mut e, 0x10, &[0x18]);
        e.register(SLOTS + 0x18, |_, _| 7u32.into_ret());
        e.register_double(EFFECT_ITEM, move |_, _| item.into_ret());
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(ALCHEMY_HAS_FLAG, |_, _| 1u32.into_ret());
        e.register(EFFECT_ITEM_DURATION, |_, _| 4u32.into_ret());
        e.register(STAT_AMOUNT, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])) + f64::from(f32::from_bits(a[1])),
            ..Ret::default()
        });
        e.register(FTOL, |_, a| {
            (f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32).into_ret()
        });
        e.register(MOD_STAT, |_, _| Ret::default());
        let this: Ptr<ActiveEffect> = Ptr::new(e.mem.alloc(0x4c));
        let this_word = this.addr();
        e.mem.set_u32(this_word + 0x24, player);
        e.mem.set_u32(this_word + 0xc, 0x4000);
        e.mem.set_u32(this_word + 0x48, HEALTH);
        e.set(this, ActiveEffect::fMagnitude, 2.5);
        e.call_log = Some(vec![]);
        e.call(0x0082_c490, &args![this]);
        // amount 4 * 2.5 = 10, lost 30: 40 for both stats.
        assert_eq!(calls(&e, MOD_STAT), vec![vec![0x12, 40], vec![0x10, 40]]);
        assert_eq!(
            calls(&e, STAT_AMOUNT)[0],
            vec![10.0f32.to_bits(), 30.0f32.to_bits()]
        );
        // Not health, no duration, or not the player: nothing.
        for change in 0..3 {
            match change {
                0 => e.mem.set_u32(this_word + 0x48, 7),
                1 => e.register(EFFECT_ITEM_DURATION, |_, _| Ret::default()),
                _ => e.mem.set_u32(this_word + 0x24, 0x7777),
            }
            e.call_log = Some(vec![]);
            value_modifier_effect_continue(&mut e, this);
            assert!(calls(&e, MOD_STAT).is_empty());
        }
    }

    #[test]
    fn causing_damage_is_health_with_flag_4_without_flag_2() {
        let mut e = Engine::new();
        e.register(EFFECT_SETTING_007DF1D0, |e, a| {
            e.mem.u32(a[0] + 0x40).into_ret()
        });
        e.register(SETTING_HAS_FLAG, |_, a| {
            u32::from(a[0] & a[1] != 0).into_ret()
        });
        let this: Ptr<ActiveEffect> = Ptr::new(e.mem.alloc(0x4c));
        let word = this.addr();
        e.mem.set_u32(word + 0x48, HEALTH);
        e.mem.set_u32(word + 0x40, 4);
        assert!(e.call(0x0082_ccb0, &args![this]).bool());
        e.mem.set_u32(word + 0x40, 6);
        assert!(!value_modifier_effect_is_causing_damage(&mut e, this));
        e.mem.set_u32(word + 0x40, 0);
        assert!(!value_modifier_effect_is_causing_damage(&mut e, this));
        e.mem.set_u32(word + 0x40, 4);
        e.mem.set_u32(word + 0x48, 0x11);
        assert!(!value_modifier_effect_is_causing_damage(&mut e, this));
    }

    #[test]
    fn finish_init_load_game_reapplies_for_bytes_5f_to_61() {
        let mut e = engine();
        e.set_global(SAVE_LOAD_GAME, 0x4444u32);
        e.register(ACTIVE_EFFECT_FINISH_INIT_LOAD_GAME, |_, _| Ret::default());
        e.register(SAVE_LOAD_BYTE_AT_80, |e, _| {
            e.mem.u32(0x011d_e800).into_ret()
        });
        e.set_global(0x011d_e800u32, 0x60u32);
        let target = with_vtable(
            &mut e,
            0x10,
            &[SLOT_ACTOR_360, SLOT_ACTOR_3B4, SLOT_ACTOR_39C],
        );
        e.register(SLOTS + SLOT_ACTOR_360, |_, _| 1u32.into_ret());
        e.register(SLOTS + SLOT_ACTOR_3B4, |_, _| Ret::default());
        e.register(SLOTS + SLOT_ACTOR_39C, |_, _| Ret::default());
        e.register(EFFECT_TEST, |_, _| 1u32.into_ret());
        e.register(EFFECT_ITEM_SETTING, |_, a| a[0].into_ret());
        e.register(SETTING_HAS_FLAG, |_, a| {
            u32::from(a[0] & a[1] != 0).into_ret()
        });
        e.register(RADIATION_AMOUNT, |_, _| Ret {
            st0: 3.0,
            ..Ret::default()
        });
        let this = with_vtable(&mut e, 0x4c, &[SLOT_GET_ACTOR_VALUE]);
        e.register(SLOTS + SLOT_GET_ACTOR_VALUE, |_, _| 0x22u32.into_ret());
        e.mem.set_u32(this + 0x24, target);
        e.mem.set_u32(this + 0x28, 0x5150);
        e.mem.set_u32(this + 0xc, 2);
        e.call_log = Some(vec![]);
        e.call(0x0082_cd10, &args![this, 9u32]);
        assert_eq!(
            calls(&e, ACTIVE_EFFECT_FINISH_INIT_LOAD_GAME),
            vec![vec![this, 9]]
        );
        assert_eq!(
            calls(&e, SLOTS + SLOT_ACTOR_3B4),
            vec![vec![target, 0x22, (-3.0f32).to_bits()]]
        );
        assert_eq!(
            calls(&e, SLOTS + SLOT_ACTOR_39C),
            vec![vec![target, 0x22, 3.0f32.to_bits(), 0x5150]]
        );
        // Other bytes, or a setting without flag 2: nothing more.
        let effect: Ptr<ActiveEffect> = Ptr::new(this);
        for (byte, flags) in [(0x5e, 2), (0x62, 2), (0x60, 4)] {
            e.set_global(0x011d_e800u32, byte);
            e.mem.set_u32(this + 0xc, flags);
            e.call_log = Some(vec![]);
            value_modifier_effect_finish_init_load_game(&mut e, effect, 9);
            assert!(calls(&e, SLOTS + SLOT_ACTOR_3B4).is_empty());
        }
    }
}
