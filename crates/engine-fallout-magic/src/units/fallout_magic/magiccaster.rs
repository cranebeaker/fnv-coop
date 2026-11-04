//! `fallout/magic/magiccaster.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `MagicCaster` (Xbox PDB, 0x0C bytes: vtable, `pLight` +4, `eState` +8)
//! is the casting side of actors and non-actor casters. Its virtual slots
//! (PC vtable `0107ac5c`, the Xbox PDB's order): +0x18 `GetTouchTarget`,
//! +0x24 `SpellCast`, +0x2C `GetCasterStatsObject`, +0x30
//! `GetMagicNode`, +0x34 `GetCurrentSpell`, +0x38 `TargetSelf`, +0x3C
//! `GetMagicEffectivenessModifier`, +0x40 `SetCurrentSpell`, +0x44
//! `GetDesiredTarget`, +0x48 `SetDesiredTarget`, +0x4C
//! `CreateActiveEffect`. The argument counts used here were checked
//! against the PC implementations (`MagicCaster`, `NonActorMagicCaster`,
//! `Actor`, `Character`, `Creature`, `PlayerCharacter`).

#[allow(unused_imports)]
use crate::prelude::*;

/// `MagicCaster`'s vtable (RTTI `.?AVMagicCaster@@`).
const MAGIC_CASTER_VTABLE: u32 = 0x0107_ac5c;
/// The `MagicLight` deleting destructor (`0080fcc0`, `this`; 1).
const MAGIC_LIGHT_DELETE: u32 = 0x0080_fcc0;
/// `__RTDynamicCast` (cdecl: object, 0, source type, target type, 0).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors.
const TYPE_MAGIC_CASTER: u32 = 0x011a_1224;
const TYPE_MAGIC_TARGET: u32 = 0x011a_0ef8;
const TYPE_NON_ACTOR_MAGIC_CASTER: u32 = 0x0118_4638;
const TYPE_MAGIC_ITEM: u32 = 0x0118_3140;
const TYPE_ALCHEMY_ITEM: u32 = 0x0118_30cc;
const TYPE_ENCHANTMENT_ITEM: u32 = 0x0118_3594;
const TYPE_INGREDIENT_ITEM: u32 = 0x0118_39dc;
const TYPE_SPELL_ITEM: u32 = 0x0118_3060;
/// The word getter `0084e3a0` (`ECX` = object): the word at +0xC (a form's
/// id, an active effect's effect item, an effect item's range).
const WORD_AT_C: u32 = 0x0084_e3a0;
/// The default listener position (`NiPoint3`, three words) passed by value.
const DEFAULT_POINT: u32 = 0x011f_426c;
/// `EffectArchetypes::Activate` (cdecl: caster, magic item, effect item,
/// source).
const EFFECT_ARCHETYPES_ACTIVATE: u32 = 0x0040_3ab0;
/// `hkpEntity::activate` (Xbox PDB name of `00c9c1d0`) and the getter of
/// the entity's motion (`009d9f40`).
const HKP_ENTITY_ACTIVATE: u32 = 0x00c9_c1d0;
const HKP_ENTITY_MOTION: u32 = 0x009d_9f40;
/// `MagicItem::GetAVEffect` (`this`; 0): the effect setting of the item's
/// actor-value effect.
const GET_AV_EFFECT: u32 = 0x0040_a300;
/// The size of the effect setting's model (`0048cee0`, `ECX` = its
/// `TESModel` at +0x18).
const MODEL_SIZE: u32 = 0x0048_cee0;
/// `ModelLoader::ReleaseModel` (`ECX` = the loader; model, 0, 1).
const MODEL_LOADER_RELEASE: u32 = 0x0044_5300;
/// The `ModelLoader*` global.
const MODEL_LOADER: u32 = 0x011c_3b3c;
/// The magic node's count (`00453470`) and its stop (`004def90`), `ECX`
/// = the node.
const MAGIC_NODE_COUNT: u32 = 0x0045_3470;
const MAGIC_NODE_STOP: u32 = 0x004d_ef90;
/// The magic node's position (`0045bb80`, `ECX` = node): `NiPoint3*`.
const MAGIC_NODE_POSITION: u32 = 0x0045_bb80;
/// `"Cannot Cast"` and `"Find Targets"` (the state's unused second word).
const TEXT_CANNOT_CAST: u32 = 0x0107_acf0;
const TEXT_FIND_TARGETS: u32 = 0x0107_acfc;
/// `"SpecialIdle_AreaEffect"`.
const TEXT_SPECIAL_IDLE_AREA_EFFECT: u32 = 0x0102_054c;

/// `FindTargets`'s callees.
const CAN_BE_POISON: u32 = 0x0040_5da0;
const HAS_TOUCH_EFFECT: u32 = 0x0040_5fe0;
const HAS_TARGET_EFFECT: u32 = 0x0040_5f30;
const GET_COSTLIEST_EFFECT: u32 = 0x0040_64d0;
const GET_ASSOCIATED_SKILL: u32 = 0x0040_65e0;
const IS_VALID_EFFECT_FOR_ENCHANTMENT: u32 = 0x0040_9720;
const EFFECT_ITEM_GET_AREA: u32 = 0x0040_4420;
const EFFECT_ITEM_IS_HOSTILE: u32 = 0x0040_4710;
const EFFECT_ITEM_GET_RAW_COST: u32 = 0x0040_3f00;
const EFFECT_ITEM_SETTING: u32 = 0x0082_5c00;
const EFFECT_ITEM_ACTOR_VALUE: u32 = 0x0040_3ea0;
const SETTING_HAS_FLAG: u32 = 0x0040_3e60;
const SPELL_HAS_FLAG: u32 = 0x0040_e210;
const ALCHEMY_HAS_FLAG: u32 = 0x0040_29d0;
const SOURCE_KIND: u32 = 0x0040_1170;
const ENCHANTMENT_MOVE_MODE: u32 = 0x005f_36f0;
const SETTING_VALUE: u32 = 0x0040_3e20;
const GET_SCALE: u32 = 0x0056_7400;
const SCALE_FACTOR: u32 = 0x0040_4010;
const DISTANCE_BETWEEN_FACING_ACTORS: u32 = 0x009a_64d0;
const LOCAL_LIST_CONSTRUCT: u32 = 0x0096_a2d0;
const LOCAL_LIST_REMOVE_ALL: u32 = 0x0047_0470;
const LOCAL_LIST_DESTRUCT: u32 = 0x0046_ffb0;
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
const COLLECT_TARGETS: u32 = 0x0081_8ce0;
const SETTING_DURATION: u32 = 0x0063_9b20;
const ACTIVE_EFFECT_SET_FLAG: u32 = 0x0080_3e80;
const ACTIVE_EFFECT_ADJUST_EFFECTIVENESS: u32 = 0x0080_4bc0;
const ACTIVE_EFFECT_SET_MAGNITUDE: u32 = 0x0069_3d50;
const ACTIVE_EFFECT_SET_DURATION: u32 = 0x0080_4cb0;
const ACTIVE_EFFECT_FLAGS: u32 = 0x007d_f1d0;
const MAGNITUDE_AND_DURATION: u32 = 0x0064_82f0;
const MAGNITUDE_ONLY: u32 = 0x0064_8620;
const DURATION_ONLY: u32 = 0x0064_8810;
const AT_LEAST: u32 = 0x004b_d510;
const ALCHEMY_MAGNITUDE_SCALE: u32 = 0x008b_ebb0;
const ALCHEMY_DURATION_SCALE: u32 = 0x008b_ec80;
const HANDLE_ENTRY_POINT: u32 = 0x005e_58f0;
const LOCATION: u32 = 0x008d_6f30;
const FIND_TARGETS_IN_AREA: u32 = 0x0081_6f10;
const DO_EXPLOSION: u32 = 0x0081_7b90;
const CHECK_WILL_KILL: u32 = 0x0081_7970;
const EXPLODE_ACTOR: u32 = 0x0081_8310;
const FIRE_PROJECTILE: u32 = 0x0081_8610;
const CHECK_ADDICTION: u32 = 0x0082_4e70;
const ANGLES: u32 = 0x0043_0830;
const ANGLE_FIRST: u32 = 0x005b_9e80;
const ANGLE_SECOND: u32 = 0x005c_53d0;
const POINT_SET: u32 = 0x0041_6870;
const START_EFFECT_SHADER: u32 = 0x0068_90b0;
const SET_IDLE: u32 = 0x0068_9920;
const SHADER_NODE: u32 = 0x0049_0e40;
const NODE_SET_SCALE: u32 = 0x0044_0490;
/// The player reference pointer.
const PLAYER: u32 = 0x011d_ea3c;
/// Game settings `fMagicDefaultTouchDistance`, `fMagicAreaScale`,
/// `fMagicAreaScaleMax`, `fMagicAreaScaleMin` (their values through
/// `00403e20`).
const TOUCH_DISTANCE: u32 = 0x011d_1a90;
const AREA_SCALE: u32 = 0x011d_1bbc;
const AREA_SCALE_MAX: u32 = 0x011d_1acc;
const AREA_SCALE_MIN: u32 = 0x011d_15a4;

fn dynamic_cast(e: &mut Engine, object: u32, from: u32, to: u32) -> u32 {
    e.call(RT_DYNAMIC_CAST, &args![object, 0u32, from, to, 0u32])
        .u32()
}

/// The address `00403e20` gives for a setting: the value's address.
fn setting_value(e: &mut Engine, setting: u32) -> u32 {
    let p = e.call(SETTING_VALUE, &args![setting]).u32();
    e.mem.u32(p)
}

/// `ActiveEffect` flags 4, 2 and 8 (`00803e80` three times).
fn flag_secondary(e: &mut Engine, effect: u32) {
    e.call(ACTIVE_EFFECT_SET_FLAG, &args![effect, 4u32]);
    e.call(ACTIVE_EFFECT_SET_FLAG, &args![effect, 2u32]);
    e.call(ACTIVE_EFFECT_SET_FLAG, &args![effect, 8u32]);
}

// Translated from 008153b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster`'s destructor body: puts back its vtable (`0107ac5c`),
/// deletes the `pLight` (+4) when there is one (`0080fcc0` with 1) and
/// clears it.
pub fn fn_008153b0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), MAGIC_CASTER_VTABLE);
    let light = e.mem.u32(this.addr().wrapping_add(4));
    // The exe tests the word, then its copy.
    let object = light;
    if light != 0 && object != 0 {
        e.call(MAGIC_LIGHT_DELETE, &args![object, 1u32]);
    }
    e.mem.set_u32(this.addr().wrapping_add(4), 0);
}

// Translated from 00815410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster::GetCasterAsActor` (Xbox PDB): when the caster's stats
/// object (slot +0x2C) answers true to its slot +0x100 (the actor test),
/// the `Actor` the caster is part of (`this - 0x88`, null for a null
/// `this`); otherwise null.
pub fn magic_caster_get_caster_as_actor(e: &mut Engine, this: Ptr) -> Ptr {
    let stats = e.vcall(this.addr(), 0x2c, &[]).u32();
    if stats != 0 && e.vcall(stats, 0x100, &[]).bool() {
        if this.is_null() {
            Ptr::NULL
        } else {
            Ptr::new(this.addr().wrapping_sub(0x88))
        }
    } else {
        Ptr::NULL
    }
}

// Translated from 00815600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster::CastAbility` (Xbox PDB): for a spell whose type (its
/// `MagicItem` part at +0x18, slot +0x18) is 4, makes it the current spell
/// (`SetCurrentSpell` with that part, or null) and runs `FindTargets(0.0,
/// 0, flag, 1.0, 0)`.
pub fn magic_caster_cast_ability(e: &mut Engine, this: Ptr, spell: u32, flag: u8) {
    if e.vcall(spell.wrapping_add(0x18), 0x18, &[]).u32() == 4 {
        let item = if spell == 0 {
            0
        } else {
            spell.wrapping_add(0x18)
        };
        e.vcall(this.addr(), 0x40, &args![item]);
        magic_caster_find_targets(e, this, 0.0, 0, flag, 1.0, 0);
    }
}

// Translated from 00815740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster::CastSpellImmediate` (Xbox PDB): sets the desired target
/// when given (slot +0x48), the current spell (slot +0x40), and runs
/// `FindTargets(0.0, 0, flag, effectiveness, hostile_only)`.
pub fn magic_caster_cast_spell_immediate(
    e: &mut Engine,
    this: Ptr,
    spell: u32,
    flag: u8,
    target: u32,
    effectiveness: f32,
    hostile_only: u8,
) {
    if target != 0 {
        e.vcall(this.addr(), 0x48, &args![target]);
    }
    e.vcall(this.addr(), 0x40, &args![spell]);
    let effectiveness = f32::from_bits(x87_quiet_bits(effectiveness.to_bits()));
    magic_caster_find_targets(e, this, 0.0, 0, flag, effectiveness, hostile_only);
}

// Translated from 008157a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster::TransferDisease` (Xbox PDB): for a spell of type 1 and a
/// non-null `target`, makes it the current spell and runs `FindTargets(0.0,
/// 0, flag, 1.0, 0)` (`target` is only tested).
pub fn magic_caster_transfer_disease(e: &mut Engine, this: Ptr, spell: u32, target: u32, flag: u8) {
    if e.vcall(spell.wrapping_add(0x18), 0x18, &[]).u32() == 1 && target != 0 {
        let item = if spell == 0 {
            0
        } else {
            spell.wrapping_add(0x18)
        };
        e.vcall(this.addr(), 0x40, &args![item]);
        magic_caster_find_targets(e, this, 0.0, 0, flag, 1.0, 0);
    }
}

// Translated from 00815820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster::CastWornEnchantment` (Xbox PDB): for a non-null
/// enchantment, makes it the current spell and runs `FindTargets(0.0,
/// source, flag, 1.0, 0)`.
pub fn magic_caster_cast_worn_enchantment(
    e: &mut Engine,
    this: Ptr,
    enchantment: u32,
    source: u32,
    flag: u8,
) {
    if enchantment == 0 {
        return;
    }
    e.vcall(this.addr(), 0x40, &args![enchantment]);
    magic_caster_find_targets(e, this, 0.0, source, flag, 1.0, 0);
}

// Translated from 00815b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster::InterruptCast` (Xbox PDB): for an actor caster, stops the
/// magic node when it has something (`00453470`, `004def90`) and deletes
/// the light; then, with a current spell, sets state 7 (`"Cannot Cast"`),
/// reports the cast (`SpellCast`, slot +0x24, with the spell and 0),
/// releases the spell's actor-value effect model when the node was stopped,
/// and clears the current spell and desired target.
pub fn magic_caster_interrupt_cast(e: &mut Engine, this: Ptr) {
    let actor = magic_caster_get_caster_as_actor(e, this).addr();
    let mut stopped = false;
    if actor != 0 {
        let caster = actor.wrapping_add(0x88);
        let node = e.vcall(caster, 0x30, &[]).u32();
        if node != 0 && e.call(MAGIC_NODE_COUNT, &args![node]).u32() != 0 {
            stopped = true;
            e.call(MAGIC_NODE_STOP, &args![node]);
        }
        let light = e.mem.u32(this.addr().wrapping_add(4));
        let object = light;
        if light != 0 && object != 0 {
            e.call(MAGIC_LIGHT_DELETE, &args![object, 1u32]);
        }
        e.mem.set_u32(this.addr().wrapping_add(4), 0);
    }
    if e.vcall(this.addr(), 0x34, &[]).u32() == 0 {
        return;
    }
    fn_00818af0(e, this, 7, TEXT_CANNOT_CAST);
    let spell = e.vcall(this.addr(), 0x34, &[]).u32();
    e.vcall(this.addr(), 0x24, &args![spell, 0u32]);
    if stopped {
        let spell = e.vcall(this.addr(), 0x34, &[]).u32();
        let setting = e.call(GET_AV_EFFECT, &args![spell, 0u32]).u32();
        if setting != 0 && e.call(MODEL_SIZE, &args![setting.wrapping_add(0x18)]).u32() != 0 {
            let model = e.vcall(setting.wrapping_add(0x18), 0x14, &[]).u32();
            let loader = e.global::<u32>(MODEL_LOADER);
            e.call(MODEL_LOADER_RELEASE, &args![loader, model, 0u32, 1u32]);
        }
    }
    e.vcall(this.addr(), 0x40, &args![0u32]);
    e.vcall(this.addr(), 0x48, &args![0u32]);
}

// Translated from 00815c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster::TargetSelf` (Xbox PDB): when the caster is also a
/// `MagicTarget` (`__RTDynamicCast`), applies `effect` to itself
/// (`00817940` with the current spell, `effect` and 0, the default point
/// passed along unread); false otherwise.
pub fn magic_caster_target_self(e: &mut Engine, this: Ptr, effect: u32) -> bool {
    let target = dynamic_cast(e, this.addr(), TYPE_MAGIC_CASTER, TYPE_MAGIC_TARGET);
    if target == 0 {
        return false;
    }
    let spell = e.vcall(this.addr(), 0x34, &[]).u32();
    let x = e.global::<u32>(DEFAULT_POINT);
    let y = e.global::<u32>(DEFAULT_POINT + 4);
    let z = e.global::<u32>(DEFAULT_POINT + 8);
    fn_00817940(e, this, target, x, y, z, spell, effect, 0) as u8 != 0
}

// Translated from 00815cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster::CreateActiveEffect` (Xbox PDB): `EffectArchetypes::
/// Activate(this, item, effect, source)` (cdecl); `EAX` is its result.
pub fn magic_caster_create_active_effect(
    e: &mut Engine,
    this: Ptr,
    item: u32,
    effect: u32,
    source: u32,
) -> u32 {
    e.call(
        EFFECT_ARCHETYPES_ACTIVATE,
        &args![this, item, effect, source],
    )
    .u32()
}

/// `FindTargetsInArea` around `center` for one active effect; returns the
/// byte the call fills (true when the area found nothing to hit).
fn find_in_area(
    e: &mut Engine,
    this: Ptr,
    spell: u32,
    effect: u32,
    center: u32,
    touch_target: u32,
    list: Ptr,
) -> bool {
    e.with_stack(4, |e, found| {
        let position = e.vcall(center, 0x1f4, &[]).u32();
        let x = e.mem.u32(position);
        let y = e.mem.u32(position.wrapping_add(4));
        let z = e.mem.u32(position.wrapping_add(8));
        let location = e.call(LOCATION, &args![center]).u32();
        e.call(
            FIND_TARGETS_IN_AREA,
            &args![
                this,
                spell,
                effect,
                location,
                x,
                y,
                z,
                touch_target,
                0u32,
                list,
                found,
                1.0f32.to_bits()
            ],
        );
        e.mem.u8(found.addr()) != 0
    })
}

/// `FindTargets`' scaling of an alchemy effect by an actor value of the
/// caster (`scale`), with the perk entry point 0xC for value 0x10.
fn alchemy_adjust(e: &mut Engine, actor: u32, effect: u32, scale: u32, effectiveness: f32) {
    let factor = e.call(scale, &args![actor]).f64();
    let value = x87_store32(x87_mul(factor, x87_m32(effectiveness.to_bits())));
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), value);
        if e.vcall(actor, 0x360, &[]).bool() {
            let item = e.call(WORD_AT_C, &args![effect]).u32();
            if e.call(EFFECT_ITEM_ACTOR_VALUE, &args![item]).u32() == 0x10 {
                e.call(HANDLE_ENTRY_POINT, &args![0xcu32, actor, slot]);
            }
        }
        let value = x87_quiet_bits(e.mem.u32(slot.addr()));
        e.call(ACTIVE_EFFECT_ADJUST_EFFECTIVENESS, &args![effect, value]);
    });
}

// Translated from 00815d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster::FindTargets` (Xbox PDB): applies the current spell. It
/// sets state 4 (`"Find Targets"`), finds the touch target (for touch or
/// poison spells: the caster's touch target, else the desired target when
/// it is within reach: `fMagicDefaultTouchDistance`, or the caster's own
/// reach (slot +0x380) unless the player touches an actor whose slot
/// +0x214 is 9, times the scaled caster scale), collects the targets around
/// it or the actor caster (`00818ce0`), then creates an active effect for
/// each effect item (an enchantment's only when valid for the source),
/// scales it by the caster's effectiveness (plain spells, all or only
/// hostile effects), the alchemy actor values and entry points (alchemy
/// items) or the ingredient's magnitude and duration (ingredients), marks
/// the secondary effects, and applies it to the caster (self range), to
/// the touch target and its area, or to the actor caster's area (touch
/// range); then runs the explosion (with its effect shader and
/// `SpecialIdle_AreaEffect` idle, scaled by `fMagicAreaScale` within its
/// minimum and maximum) or the actor explosion of a lethal touch effect,
/// fires the projectile for target effects or advances the caster's skill
/// on a hit, releases the actor-value effect model, clears the current
/// spell, desired target and state, and checks addiction for alchemy.
/// `value` is passed on to the ingredient computations; `mark` sets
/// flags 4 and 0x10 on each effect.
// Translation incomplete: 00815d00 - SEH exception frame (FS:[0]) that destroys the local target list on unwinding is not modelled.
pub fn magic_caster_find_targets(
    e: &mut Engine,
    this: Ptr,
    value: f32,
    source: u32,
    mark: u8,
    effectiveness: f32,
    hostile_only: u8,
) {
    let spell = e.vcall(this.addr(), 0x34, &[]).u32();
    if spell == 0 {
        return;
    }
    fn_00818af0(e, this, 4, TEXT_FIND_TARGETS);
    let caster_ref = e.vcall(this.addr(), 0x2c, &[]).u32();
    let actor = magic_caster_get_caster_as_actor(e, this).addr();
    let mut touch_target = 0u32;
    let is_alchemy = e.vcall(spell, 0x18, &[]).u32() == 7;
    let poison = is_alchemy
        && e.call(CAN_BE_POISON, &args![spell.wrapping_add(0xc)])
            .bool();
    let alchemy = if is_alchemy {
        dynamic_cast(e, spell, TYPE_MAGIC_ITEM, TYPE_ALCHEMY_ITEM)
    } else {
        0
    };
    if e.call(HAS_TOUCH_EFFECT, &args![spell.wrapping_add(0xc)])
        .bool()
        || poison
    {
        let mut blocked = if source != 0 {
            e.call(SOURCE_KIND, &args![source]).u32() == 0x29
        } else {
            false
        };
        if !blocked && e.vcall(spell, 0x18, &[]).u32() == 6 {
            let enchantment = dynamic_cast(e, spell, TYPE_MAGIC_ITEM, TYPE_ENCHANTMENT_ITEM);
            if enchantment != 0 {
                blocked = e.call(ENCHANTMENT_MOVE_MODE, &args![enchantment]).u32() == 2;
            }
        }
        if !blocked && (actor != 0 || e.vcall(this.addr(), 0x44, &[]).u32() == 0) {
            touch_target = e.vcall(this.addr(), 0x18, &[]).u32();
        }
        if touch_target == 0 {
            touch_target = e.vcall(this.addr(), 0x44, &[]).u32();
            if touch_target != 0
                && source == 0
                && !blocked
                && e.vcall(touch_target, 0x4, &[]).u32() != 0
            {
                let mut reach = x87_quiet_bits(setting_value(e, TOUCH_DISTANCE));
                if e.vcall(caster_ref, 0x100, &[]).bool() {
                    let own_reach = if caster_ref == e.global::<u32>(PLAYER) {
                        true
                    } else {
                        let other = e.vcall(touch_target, 0x4, &[]).u32();
                        if e.vcall(other, 0x100, &[]).bool() {
                            let other = e.vcall(touch_target, 0x4, &[]).u32();
                            e.vcall(other, 0x214, &[]).u32() != 9
                        } else {
                            true
                        }
                    };
                    if own_reach {
                        reach = x87_store32(e.vcall(caster_ref, 0x380, &[]).f64());
                    }
                }
                let scale = x87_store32(e.call(GET_SCALE, &args![caster_ref]).f64());
                let factor = e.call(SCALE_FACTOR, &args![1.0f32.to_bits(), scale]).f64();
                reach = x87_store32(x87_mul(factor, x87_m32(reach)));
                let other = e.vcall(touch_target, 0x4, &[]).u32();
                let distance = e
                    .call(
                        DISTANCE_BETWEEN_FACING_ACTORS,
                        &args![caster_ref, other, 0u32],
                    )
                    .f64();
                // `FCOMPP; TEST AH,5; JP`: cleared only when below.
                if x87_m32(reach) < distance {
                    touch_target = 0;
                }
            }
        }
    }
    let mut release_model = false;
    let node = e.vcall(this.addr(), 0x30, &[]).u32();
    if node != 0 && e.call(MAGIC_NODE_COUNT, &args![node]).u32() != 0 && source == 0 {
        release_model = true;
    }
    let target_ref = if touch_target != 0 {
        e.vcall(touch_target, 0x4, &[]).u32()
    } else {
        0
    };
    let mut hit = false;
    let ingredient = dynamic_cast(e, spell, TYPE_MAGIC_ITEM, TYPE_INGREDIENT_ITEM);
    let list_of_effects = spell.wrapping_add(0xc);
    let self_effect = e
        .call(GET_COSTLIEST_EFFECT, &args![list_of_effects, 0u32, 0u32])
        .u32();
    let touch_effect = e
        .call(GET_COSTLIEST_EFFECT, &args![list_of_effects, 1u32, 0u32])
        .u32();
    let spell_item = dynamic_cast(e, spell, TYPE_MAGIC_ITEM, TYPE_SPELL_ITEM);
    e.with_stack(8, |e, list| {
        e.call(LOCAL_LIST_CONSTRUCT, &args![list]);
        e.call(LOCAL_LIST_REMOVE_ALL, &args![list]);
        let collect_around = |e: &mut Engine, center: u32| {
            let position = e.vcall(center, 0x1f4, &[]).u32();
            let x = e.mem.u32(position);
            let y = e.mem.u32(position.wrapping_add(4));
            let z = e.mem.u32(position.wrapping_add(8));
            e.call(COLLECT_TARGETS, &args![this, spell, 1u32, x, y, z, list]);
        };
        if target_ref != 0 {
            collect_around(e, target_ref);
        } else if actor != 0
            && spell_item != 0
            && e.call(SPELL_HAS_FLAG, &args![spell_item, 0x80u32]).bool()
        {
            collect_around(e, actor);
        }
        let mut effect_node = if spell == 0 {
            0
        } else {
            spell.wrapping_add(0xc)
        };
        while effect_node != 0 {
            let slot = e.call(NODE_ITEM, &args![effect_node.wrapping_add(4)]).u32();
            let mut effect_item = e.mem.u32(slot);
            if effect_item != 0 && e.vcall(spell, 0x18, &[]).u32() == 6 && source != 0 {
                let setting = e.call(EFFECT_ITEM_SETTING, &args![effect_item]).u32();
                let valid = e
                    .call(
                        IS_VALID_EFFECT_FOR_ENCHANTMENT,
                        &args![source, 0u32, setting, 0u32],
                    )
                    .bool();
                if !valid {
                    effect_item = 0;
                }
            }
            if effect_item != 0 {
                let active = e
                    .vcall(this.addr(), 0x4c, &args![spell, effect_item, source])
                    .u32();
                let active_item = e.call(WORD_AT_C, &args![active]).u32();
                let active_setting = e.call(EFFECT_ITEM_SETTING, &args![active_item]).u32();
                let duration = x87_store32(e.call(SETTING_DURATION, &args![active_setting]).f64());
                if mark != 0 {
                    e.call(ACTIVE_EFFECT_SET_FLAG, &args![active, 4u32]);
                    e.call(ACTIVE_EFFECT_SET_FLAG, &args![active, 0x10u32]);
                }
                match e.vcall(spell, 0x18, &[]).u32() {
                    0 => {
                        // The exe tests `hostile_only` a second time before
                        // `IsHostile`; that test can only pass here.
                        let apply = if hostile_only == 0 {
                            true
                        } else {
                            let item = e.call(WORD_AT_C, &args![active]).u32();
                            e.call(EFFECT_ITEM_IS_HOSTILE, &args![item]).bool()
                        };
                        if apply {
                            let modifier = e
                                .vcall(this.addr(), 0x3c, &args![0u32, 0.0f32.to_bits()])
                                .f64();
                            let adjusted =
                                x87_store32(x87_mul(modifier, x87_m32(effectiveness.to_bits())));
                            e.call(ACTIVE_EFFECT_ADJUST_EFFECTIVENESS, &args![active, adjusted]);
                        }
                    }
                    7 => 'alchemy: {
                        if alchemy != 0 && e.call(ALCHEMY_HAS_FLAG, &args![alchemy, 4u32]).bool() {
                            alchemy_adjust(
                                e,
                                actor,
                                active,
                                ALCHEMY_MAGNITUDE_SCALE,
                                effectiveness,
                            );
                        }
                        if alchemy != 0 && e.call(ALCHEMY_HAS_FLAG, &args![alchemy, 2u32]).bool() {
                            let item = e.call(WORD_AT_C, &args![active]).u32();
                            if e.call(EFFECT_ITEM_ACTOR_VALUE, &args![item]).u32() != 0x36 {
                                alchemy_adjust(
                                    e,
                                    actor,
                                    active,
                                    ALCHEMY_DURATION_SCALE,
                                    effectiveness,
                                );
                                break 'alchemy;
                            }
                        }
                        if alchemy != 0 && e.call(ALCHEMY_HAS_FLAG, &args![alchemy, 2u32]).bool() {
                            let item = e.call(WORD_AT_C, &args![active]).u32();
                            if e.call(EFFECT_ITEM_ACTOR_VALUE, &args![item]).u32() == 0x36 {
                                let factor = e.with_stack(4, |e, slot| {
                                    e.mem.set_u32(slot.addr(), 1.0f32.to_bits());
                                    e.call(HANDLE_ENTRY_POINT, &args![0x2cu32, actor, slot]);
                                    e.mem.u32(slot.addr())
                                });
                                let adjusted = x87_store32(x87_mul(
                                    x87_m32(effectiveness.to_bits()),
                                    x87_m32(factor),
                                ));
                                e.call(
                                    ACTIVE_EFFECT_ADJUST_EFFECTIVENESS,
                                    &args![active, adjusted],
                                );
                            }
                        }
                    }
                    8 => {
                        if ingredient != 0
                            && e.vcall(ingredient.wrapping_add(0x3c), 0x4, &[]).bool()
                        {
                            e.call(
                                ACTIVE_EFFECT_SET_MAGNITUDE,
                                &args![active, 1.0f32.to_bits()],
                            );
                        } else {
                            let flags = e.call(ACTIVE_EFFECT_FLAGS, &args![active]).u32();
                            e.with_stack(8, |e, pair| {
                                // `pair` + 0 is the duration result, + 4 the
                                // magnitude result (`EBP-0x84`, `EBP-0x80`).
                                let first = pair;
                                let second = pair.byte_add(4);
                                e.mem.set_u32(first.addr(), 0.0f32.to_bits());
                                e.mem.set_u32(second.addr(), 0.0f32.to_bits());
                                let value = x87_quiet_bits(value.to_bits());
                                let has = |e: &mut Engine, bit: u32| {
                                    e.call(SETTING_HAS_FLAG, &args![flags, bit]).bool()
                                };
                                'computed: {
                                    if !has(e, 0x80) && !has(e, 0x100) {
                                        e.call(
                                            MAGNITUDE_AND_DURATION,
                                            &args![
                                                first, second, duration, value, 6u32, 0u32, 0u32,
                                                0u32, 0u32, 0u32
                                            ],
                                        );
                                        break 'computed;
                                    }
                                    if !has(e, 0x80) && has(e, 0x100) {
                                        e.call(
                                            MAGNITUDE_ONLY,
                                            &args![
                                                second, duration, value, 6u32, 0u32, 0u32, 0u32,
                                                0u32, 0u32
                                            ],
                                        );
                                        break 'computed;
                                    }
                                    if has(e, 0x80) && !has(e, 0x100) {
                                        e.call(
                                            DURATION_ONLY,
                                            &args![
                                                first, duration, value, 6u32, 0u32, 0u32, 0u32,
                                                0u32, 0u32
                                            ],
                                        );
                                    }
                                }
                                let one = 1.0f32.to_bits();
                                let magnitude = x87_quiet_bits(e.mem.u32(second.addr()));
                                let magnitude =
                                    x87_store32(e.call(AT_LEAST, &args![magnitude, one]).f64());
                                e.mem.set_u32(second.addr(), magnitude);
                                let time = x87_quiet_bits(e.mem.u32(first.addr()));
                                let time = x87_store32(e.call(AT_LEAST, &args![time, one]).f64());
                                e.mem.set_u32(first.addr(), time);
                                let magnitude = x87_quiet_bits(e.mem.u32(second.addr()));
                                e.call(ACTIVE_EFFECT_SET_MAGNITUDE, &args![active, magnitude]);
                                let time = x87_quiet_bits(e.mem.u32(first.addr()));
                                e.call(ACTIVE_EFFECT_SET_DURATION, &args![active, time]);
                            });
                        }
                    }
                    _ => {}
                }
                let range = e.call(WORD_AT_C, &args![effect_item]).u32();
                if range == 0 && !poison {
                    if effect_item != self_effect {
                        flag_secondary(e, active);
                    }
                    if e.vcall(this.addr(), 0x38, &args![active]).bool() {
                        hit = true;
                    }
                } else if e.call(WORD_AT_C, &args![effect_item]).u32() == 1 || poison {
                    let costliest = if poison { self_effect } else { touch_effect };
                    if effect_item != costliest {
                        flag_secondary(e, active);
                    }
                    if touch_target != 0 {
                        let x = e.global::<u32>(DEFAULT_POINT);
                        let y = e.global::<u32>(DEFAULT_POINT + 4);
                        let z = e.global::<u32>(DEFAULT_POINT + 8);
                        if fn_00817940(e, this, touch_target, x, y, z, spell, active, 0) as u8 != 0
                        {
                            hit = true;
                        }
                        if e.call(EFFECT_ITEM_GET_AREA, &args![effect_item]).i32() > 0
                            && target_ref != 0
                            && find_in_area(e, this, spell, active, target_ref, touch_target, list)
                        {
                            hit = false;
                        }
                    } else if !poison
                        && spell_item != 0
                        && e.call(SPELL_HAS_FLAG, &args![spell_item, 0x80u32]).bool()
                        && e.call(EFFECT_ITEM_GET_AREA, &args![effect_item]).i32() > 0
                        && actor != 0
                        && find_in_area(e, this, spell, active, actor, touch_target, list)
                    {
                        hit = false;
                    }
                }
                if active != 0 {
                    e.vcall(active, 0x0, &args![1u32]);
                }
            }
            if e.vcall(spell, 0x18, &[]).u32() == 8 {
                effect_node = 0;
            } else {
                let next = e.call(NODE_NEXT, &args![effect_node.wrapping_add(4)]).u32();
                effect_node = if next != 0 { next.wrapping_sub(4) } else { 0 };
            }
        }
        let av_setting = e.call(GET_AV_EFFECT, &args![spell, 0u32]).u32();
        let explosion = e
            .call(GET_COSTLIEST_EFFECT, &args![list_of_effects, 1u32, 1u32])
            .u32();
        let spell_area =
            spell_item != 0 && e.call(SPELL_HAS_FLAG, &args![spell_item, 0x80u32]).bool();
        let center = if target_ref != 0 {
            target_ref
        } else if actor != 0 && spell_area {
            actor
        } else {
            0
        };
        if center != 0 {
            e.with_stack(0xc, |e, point| {
                // `006815c0` run on the stack point (its result is unused).
                e.call(NODE_ITEM, &args![point]);
                let magic_node = if actor != 0 && source == 0 {
                    e.vcall(actor.wrapping_add(0x88), 0x30, &[]).u32()
                } else {
                    0
                };
                let position = if magic_node != 0 {
                    e.call(MAGIC_NODE_POSITION, &args![magic_node]).u32()
                } else {
                    e.vcall(center, 0x1f4, &[]).u32()
                };
                for i in 0..3u32 {
                    let word = e.mem.u32(position.wrapping_add(i * 4));
                    e.mem.set_u32(point.addr() + i * 4, word);
                }
                let x = e.mem.u32(point.addr());
                let y = e.mem.u32(point.addr() + 4);
                let z = e.mem.u32(point.addr() + 8);
                if explosion != 0 {
                    let location = e.call(LOCATION, &args![center]).u32();
                    let one = 1.0f32.to_bits();
                    e.call(
                        DO_EXPLOSION,
                        &args![this, x, y, z, location, spell, explosion, list, one, one],
                    );
                    if av_setting != 0
                        && e.call(MODEL_SIZE, &args![av_setting.wrapping_add(0x18)])
                            .u32()
                            != 0
                    {
                        area_effect_shader(e, center, av_setting, explosion, [x, y, z]);
                    }
                } else if e.vcall(center, 0x100, &[]).bool() && touch_effect != 0 {
                    let setting = e.call(EFFECT_ITEM_SETTING, &args![touch_effect]).u32();
                    if e.call(SETTING_HAS_FLAG, &args![setting, 0x2000_0000u32])
                        .bool()
                    {
                        let target = if center != 0 {
                            center.wrapping_add(0x94)
                        } else {
                            0
                        };
                        if e.call(CHECK_WILL_KILL, &args![this, target, spell]).bool() {
                            let cost = x87_store32(
                                e.call(EFFECT_ITEM_GET_RAW_COST, &args![touch_effect]).f64(),
                            );
                            e.call(
                                EXPLODE_ACTOR,
                                &args![this, center, x, y, z, touch_effect, cost],
                            );
                        }
                    }
                }
            });
        }
        e.call(LOCAL_LIST_REMOVE_ALL, &args![list]);
        if e.call(HAS_TARGET_EFFECT, &args![spell.wrapping_add(0xc)])
            .bool()
        {
            e.call(FIRE_PROJECTILE, &args![this]);
        } else if hit
            && actor != 0
            && (e.vcall(spell, 0x18, &[]).u32() == 0 || e.vcall(spell, 0x18, &[]).u32() == 5)
        {
            let skill = e
                .call(GET_ASSOCIATED_SKILL, &args![spell.wrapping_add(0xc)])
                .u32();
            e.vcall(actor, 0x480, &args![skill, 1u32, 0.0f32.to_bits()]);
        }
        if release_model
            && av_setting != 0
            && e.call(MODEL_SIZE, &args![av_setting.wrapping_add(0x18)])
                .u32()
                != 0
        {
            let model = e.vcall(av_setting.wrapping_add(0x18), 0x14, &[]).u32();
            let loader = e.global::<u32>(MODEL_LOADER);
            e.call(MODEL_LOADER_RELEASE, &args![loader, model, 0u32, 1u32]);
        }
        e.vcall(this.addr(), 0x40, &args![0u32]);
        e.vcall(this.addr(), 0x48, &args![0u32]);
        fn_00818af0(e, this, 0, 0);
        if is_alchemy && actor != 0 {
            let alchemy = dynamic_cast(e, spell, TYPE_MAGIC_ITEM, TYPE_ALCHEMY_ITEM);
            if alchemy != 0 {
                e.call(CHECK_ADDICTION, &args![actor.wrapping_add(0x94), alchemy]);
            }
        }
        e.call(LOCAL_LIST_DESTRUCT, &args![list]);
    });
}

/// `FindTargets`' explosion effect shader: the direction from the
/// center's angles, the shader started at `position` with the effect
/// setting's model, the `SpecialIdle_AreaEffect` idle, and the shader
/// node scaled by the explosion's area times `fMagicAreaScale` within
/// `fMagicAreaScaleMin`..`fMagicAreaScaleMax`.
fn area_effect_shader(
    e: &mut Engine,
    center: u32,
    av_setting: u32,
    explosion: u32,
    position: [u32; 3],
) {
    e.with_stack(0xc, |e, direction| {
        let angles = e.call(ANGLES, &args![center]).u32();
        let negated = x87_quiet_bits(e.mem.u32(angles)) ^ 0x8000_0000;
        let first = x87_store32(e.call(ANGLE_FIRST, &args![negated]).f64());
        let angles = e.call(ANGLES, &args![center]).u32();
        let negated = x87_quiet_bits(e.mem.u32(angles)) ^ 0x8000_0000;
        let second = e.call(ANGLE_SECOND, &args![negated]).f64();
        let second = x87_store32(-second);
        e.call(
            POINT_SET,
            &args![direction, second, first, 0.0f32.to_bits()],
        );
        let model = e.vcall(av_setting.wrapping_add(0x18), 0x14, &[]).u32();
        let location = e.call(LOCATION, &args![center]).u32();
        let dx = e.mem.u32(direction.addr());
        let dy = e.mem.u32(direction.addr() + 4);
        let dz = e.mem.u32(direction.addr() + 8);
        let one = 1.0f32.to_bits();
        let shader = e
            .call(
                START_EFFECT_SHADER,
                &args![
                    location,
                    one,
                    model,
                    dx,
                    dy,
                    dz,
                    position[0],
                    position[1],
                    position[2],
                    one,
                    7u32,
                    0u32
                ],
            )
            .u32();
        e.call(SET_IDLE, &args![shader, TEXT_SPECIAL_IDLE_AREA_EFFECT]);
        if e.call(SHADER_NODE, &args![shader]).u32() != 0 {
            let area = e.call(EFFECT_ITEM_GET_AREA, &args![explosion]).i32();
            let scale = setting_value(e, AREA_SCALE);
            let mut size = x87_store32(x87_mul(x87_load32(scale), f64::from(area)));
            let maximum = setting_value(e, AREA_SCALE_MAX);
            // `FCOMPP; TEST AH,5; JP`: clamped when the maximum is below.
            if x87_load32(maximum) < x87_m32(size) {
                size = x87_quiet_bits(setting_value(e, AREA_SCALE_MAX));
            } else {
                let minimum = setting_value(e, AREA_SCALE_MIN);
                // `TEST AH,0x41; JNZ`: raised when the minimum is above.
                if x87_load32(minimum) > x87_m32(size) {
                    size = x87_quiet_bits(setting_value(e, AREA_SCALE_MIN));
                }
            }
            let node = e.call(SHADER_NODE, &args![shader]).u32();
            e.call(NODE_SET_SCALE, &args![node, size]);
        }
    });
}

// Translated from 00817940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies `effect` to `target` (a `MagicTarget`): its virtual slot 0 with
/// the caster, `spell`, `effect` and `flag`. The point passed by value
/// (three words) is not read. `EAX` is the slot's result (its `AL` is the
/// callers' bool).
pub fn fn_00817940(
    e: &mut Engine,
    this: Ptr,
    target: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    spell: u32,
    effect: u32,
    flag: u8,
) -> u32 {
    e.vcall(target, 0x0, &args![this, spell, effect, u32::from(flag)])
        .u32()
}

// Translated from 008182d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRigidBody::applyLinearImpulse` (the engine map's name for this
/// body): activates the entity (`00c9c1d0`) and hands `impulse` to its
/// motion's virtual slot +0x4C (the motion from `009d9f40`).
pub fn hkp_rigid_body_apply_linear_impulse(e: &mut Engine, this: Ptr, impulse: u32) {
    e.call(HKP_ENTITY_ACTIVATE, &args![this]);
    let motion = e.call(HKP_ENTITY_MOTION, &args![this]).u32();
    e.vcall(motion, 0x4c, &args![impulse]);
}

// Translated from 00818af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the caster's state (`eState`, +8). Two words are taken (`RET 8`);
/// the second (callers pass a description string) is not read.
pub fn fn_00818af0(e: &mut Engine, this: Ptr, state: u32, _unused_2: u32) {
    e.mem.set_u32(this.addr().wrapping_add(8), state);
}

// Translated from 00818be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster::GetMagicCasterFormID` (Xbox PDB): the form id (+0xC,
/// `0084e3a0`) of the actor caster, or of the reference a
/// `NonActorMagicCaster` holds (slot +0x2C of its part at +0xC); 0
/// otherwise.
pub fn magic_caster_get_magic_caster_form_id(e: &mut Engine, this: Ptr) -> u32 {
    let actor = magic_caster_get_caster_as_actor(e, this).addr();
    let non_actor = dynamic_cast(
        e,
        this.addr(),
        TYPE_MAGIC_CASTER,
        TYPE_NON_ACTOR_MAGIC_CASTER,
    );
    if actor != 0 {
        return e.call(WORD_AT_C, &args![actor]).u32();
    }
    if non_actor != 0 && e.vcall(non_actor.wrapping_add(0xc), 0x2c, &[]).u32() != 0 {
        let reference = e.vcall(non_actor.wrapping_add(0xc), 0x2c, &[]).u32();
        return e.call(WORD_AT_C, &args![reference]).u32();
    }
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0081_53b0, fn_008153b0(Ptr)),
        entry!(0x0081_5410, magic_caster_get_caster_as_actor(Ptr) -> Ptr),
        entry!(0x0081_5600, magic_caster_cast_ability(Ptr, u32, u8)),
        entry!(
            0x0081_5740,
            magic_caster_cast_spell_immediate(Ptr, u32, u8, u32, f32, u8)
        ),
        entry!(
            0x0081_57a0,
            magic_caster_transfer_disease(Ptr, u32, u32, u8)
        ),
        entry!(
            0x0081_5820,
            magic_caster_cast_worn_enchantment(Ptr, u32, u32, u8)
        ),
        entry!(0x0081_5b00, magic_caster_interrupt_cast(Ptr)),
        entry!(0x0081_5c50, magic_caster_target_self(Ptr, u32) -> bool),
        entry!(
            0x0081_5cd0,
            magic_caster_create_active_effect(Ptr, u32, u32, u32) -> u32
        ),
        entry!(
            0x0081_5d00,
            magic_caster_find_targets(Ptr, f32, u32, u8, f32, u8)
        ),
        entry!(
            0x0081_7940,
            fn_00817940(Ptr, u32, u32, u32, u32, u32, u32, u8) -> u32
        ),
        entry!(0x0081_82d0, hkp_rigid_body_apply_linear_impulse(Ptr, u32)),
        entry!(0x0081_8af0, fn_00818af0(Ptr, u32, u32)),
        entry!(0x0081_8be0, magic_caster_get_magic_caster_form_id(Ptr) -> u32),
        entry!(0x0081_5380, magic_caster_start_cast(Ptr)),
        entry!(0x0081_53a0, fn_008153a0(Ptr, u32, u32) -> f64),
        entry!(0x0081_5480, magic_caster_cast_spell(Ptr, u32, u32, u8) -> bool),
        entry!(0x0081_5670, magic_caster_cast_addiction(Ptr, u32, u8)),
        entry!(0x0081_56e0, magic_caster_cast_radiation(Ptr, u32, u8)),
        entry!(0x0081_5870, magic_caster_release_cast(Ptr, u32) -> bool),
        entry!(
            0x0081_6f10,
            magic_caster_find_targets_in_area(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, f32
            )
        ),
        entry!(
            0x0081_7160,
            magic_caster_find_targets_ov2(Ptr, u32, u32, u32, u32, u32, u32, u32, u32, f32, f32)
        ),
        entry!(0x0081_7620, magic_caster_get_touch_target(Ptr) -> u32),
        entry!(0x0081_7970, magic_caster_check_will_kill(Ptr, u32, u32) -> bool),
        entry!(
            0x0081_7b90,
            magic_caster_do_explosion(Ptr, u32, u32, u32, u32, u32, u32, u32, f32, f32)
        ),
        entry!(0x0081_8110, fn_00818110(Ptr) -> u32),
        entry!(0x0081_8150, fn_00818150(Ptr, u32, u32, u32, u32, u32, f32)),
        entry!(
            0x0081_8310,
            magic_caster_explode_actor(Ptr, u32, u32, u32, u32, u32, f32)
        ),
        entry!(0x0081_8610, magic_caster_fire_projectile(Ptr)),
        entry!(0x0081_8b10, fn_00818b10(Ptr, f32)),
        entry!(0x0081_8ba0, fn_00818ba0(Ptr) -> bool),
        entry!(0x0081_8c60, magic_caster_get_magic_caster_by_numeric_id(u32) -> u32),
        entry!(
            0x0081_8cb0,
            magic_caster_check_cast(Ptr, u32, u32, u32, u32) -> bool
        ),
        entry!(0x0081_8ce0, fn_00818ce0(Ptr, u32, u32, u32, u32, u32, u32)),
        entry!(
            0x0081_90d0,
            magic_caster_run_explosion_los_pick(Ptr, u32, u32, u32, u32) -> bool
        ),
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

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// Gives `object` a vtable of 0x130 slots whose slot `offset` is
    /// `target` for each pair; other slots stay 0.
    fn vtable(e: &mut Engine, object: u32, at: u32, slots: &[(u32, u32)]) {
        let mut table = vec![0u32; 0x130];
        for (offset, target) in slots {
            table[(*offset / 4) as usize] = *target;
        }
        e.put_vtable(at, &table);
        e.mem.set_u32(object, at);
    }

    /// A double at `addr` returning `value` in `EAX`.
    fn returns(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| ret(value));
    }

    #[test]
    fn destructor_body_deletes_the_light() {
        let mut e = Engine::new();
        returns(&mut e, MAGIC_LIGHT_DELETE, 0);
        let caster = e.mem.alloc(0xc);
        e.mem.set_u32(caster + 4, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x0081_53b0, &args![caster]);
        assert_eq!(e.mem.u32(caster), MAGIC_CASTER_VTABLE);
        assert_eq!(e.mem.u32(caster + 4), 0);
        assert_eq!(calls_to(&e, MAGIC_LIGHT_DELETE), vec![vec![0x5000, 1]]);
        e.call(0x0081_53b0, &args![caster]);
        assert_eq!(calls_to(&e, MAGIC_LIGHT_DELETE).len(), 1);
    }

    #[test]
    fn caster_as_actor_needs_an_actor_stats_object() {
        let mut e = Engine::new();
        let stats = e.mem.alloc(8);
        vtable(&mut e, stats, 0x00a1_0000, &[(0x100, 0x00a1_1000)]);
        let is_actor = std::rc::Rc::new(std::cell::Cell::new(1u32));
        let flag = is_actor.clone();
        e.register_double(0x00a1_1000, move |_, _| ret(flag.get()));
        let caster = e.mem.alloc(0x100) + 0x88;
        vtable(&mut e, caster, 0x00a2_0000, &[(0x2c, 0x00a2_1000)]);
        returns(&mut e, 0x00a2_1000, stats);
        assert_eq!(e.call(0x0081_5410, &args![caster]).u32(), caster - 0x88);
        is_actor.set(0x100);
        assert_eq!(e.call(0x0081_5410, &args![caster]).u32(), 0);
        returns(&mut e, 0x00a2_1000, 0);
        assert_eq!(e.call(0x0081_5410, &args![caster]).u32(), 0);
    }

    /// A caster whose current spell (slot +0x34) is null, so `FindTargets`
    /// stops at once; slots +0x40 and +0x48 record.
    fn idle_caster() -> (Engine, u32) {
        let mut e = Engine::new();
        let caster = e.mem.alloc(0xc);
        vtable(
            &mut e,
            caster,
            0x00a3_0000,
            &[
                (0x34, 0x00a3_1034),
                (0x40, 0x00a3_1040),
                (0x48, 0x00a3_1048),
            ],
        );
        returns(&mut e, 0x00a3_1034, 0);
        returns(&mut e, 0x00a3_1040, 0);
        returns(&mut e, 0x00a3_1048, 0);
        (e, caster)
    }

    /// A spell object whose `MagicItem` part (+0x18) reports `kind`.
    fn spell_of_kind(e: &mut Engine, kind: u32) -> u32 {
        let spell = e.mem.alloc(0x40);
        vtable(
            e,
            spell + 0x18,
            0x00a4_0000 + kind * 0x1000,
            &[(0x18, 0x00a5_0000 + kind)],
        );
        returns(e, 0x00a5_0000 + kind, kind);
        spell
    }

    #[test]
    fn cast_ability_only_for_kind_four() {
        let (mut e, caster) = idle_caster();
        let ability = spell_of_kind(&mut e, 4);
        let other = spell_of_kind(&mut e, 2);
        e.call_log = Some(vec![]);
        e.call(0x0081_5600, &args![caster, ability, 1u32]);
        e.call(0x0081_5600, &args![caster, other, 1u32]);
        assert_eq!(
            calls_to(&e, 0x00a3_1040),
            vec![vec![caster, ability + 0x18]]
        );
        // FindTargets ran once and stopped without a current spell.
        assert_eq!(calls_to(&e, 0x00a3_1034).len(), 1);
    }

    #[test]
    fn cast_spell_immediate_sets_target_and_spell() {
        let (mut e, caster) = idle_caster();
        e.call_log = Some(vec![]);
        let words = args![caster, 0x77u32, 1u32, 0x88u32, 2.0f32, 1u32];
        e.call(0x0081_5740, &words);
        assert_eq!(calls_to(&e, 0x00a3_1048), vec![vec![caster, 0x88]]);
        assert_eq!(calls_to(&e, 0x00a3_1040), vec![vec![caster, 0x77]]);
        e.call(
            0x0081_5740,
            &args![caster, 0x77u32, 1u32, 0u32, 2.0f32, 1u32],
        );
        assert_eq!(calls_to(&e, 0x00a3_1048).len(), 1);
        assert_eq!(calls_to(&e, 0x00a3_1034).len(), 2);
    }

    #[test]
    fn transfer_disease_and_worn_enchantment() {
        let (mut e, caster) = idle_caster();
        let disease = spell_of_kind(&mut e, 1);
        e.call_log = Some(vec![]);
        e.call(0x0081_57a0, &args![caster, disease, 0u32, 0u32]);
        assert!(calls_to(&e, 0x00a3_1040).is_empty());
        e.call(0x0081_57a0, &args![caster, disease, 0x99u32, 0u32]);
        assert_eq!(
            calls_to(&e, 0x00a3_1040),
            vec![vec![caster, disease + 0x18]]
        );
        e.call(0x0081_5820, &args![caster, 0u32, 5u32, 0u32]);
        e.call(0x0081_5820, &args![caster, 0x66u32, 5u32, 0u32]);
        assert_eq!(calls_to(&e, 0x00a3_1040)[1], vec![caster, 0x66]);
        assert_eq!(calls_to(&e, 0x00a3_1034).len(), 2);
    }

    #[test]
    fn interrupt_cast_stops_the_node_and_releases_the_model() {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.set_global(MODEL_LOADER, 0x00a6_0000u32);
        let stats = e.mem.alloc(8);
        vtable(&mut e, stats, 0x00a1_0000, &[(0x100, 0x00a1_1000)]);
        returns(&mut e, 0x00a1_1000, 1);
        let actor = e.mem.alloc(0x100);
        let caster = actor + 0x88;
        vtable(
            &mut e,
            caster,
            0x00a3_0000,
            &[
                (0x24, 0x00a3_1024),
                (0x2c, 0x00a3_102c),
                (0x30, 0x00a3_1030),
                (0x34, 0x00a3_1034),
                (0x40, 0x00a3_1040),
                (0x48, 0x00a3_1048),
            ],
        );
        returns(&mut e, 0x00a3_102c, stats);
        returns(&mut e, 0x00a3_1030, 0x00a7_0000);
        returns(&mut e, 0x00a3_1034, 0x00a8_0000);
        for slot in [0x00a3_1024, 0x00a3_1040, 0x00a3_1048] {
            returns(&mut e, slot, 0);
        }
        returns(&mut e, MAGIC_NODE_COUNT, 2);
        returns(&mut e, MAGIC_NODE_STOP, 0);
        returns(&mut e, MAGIC_LIGHT_DELETE, 0);
        let setting = e.mem.alloc(0x40);
        vtable(&mut e, setting + 0x18, 0x00a9_0000, &[(0x14, 0x00a9_1014)]);
        returns(&mut e, 0x00a9_1014, 0x00aa_0000);
        returns(&mut e, GET_AV_EFFECT, setting);
        returns(&mut e, MODEL_SIZE, 1);
        returns(&mut e, MODEL_LOADER_RELEASE, 0);
        e.mem.set_u32(caster + 4, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x0081_5b00, &args![caster]);
        assert_eq!(calls_to(&e, MAGIC_NODE_STOP), vec![vec![0x00a7_0000]]);
        assert_eq!(calls_to(&e, MAGIC_LIGHT_DELETE), vec![vec![0x5000, 1]]);
        assert_eq!(e.mem.u32(caster + 8), 7);
        assert_eq!(
            calls_to(&e, 0x00a3_1024),
            vec![vec![caster, 0x00a8_0000, 0]]
        );
        assert_eq!(calls_to(&e, GET_AV_EFFECT), vec![vec![0x00a8_0000, 0]]);
        assert_eq!(
            calls_to(&e, MODEL_LOADER_RELEASE),
            vec![vec![0x00a6_0000, 0x00aa_0000, 0, 1]]
        );
        assert_eq!(calls_to(&e, 0x00a3_1040), vec![vec![caster, 0]]);
        assert_eq!(calls_to(&e, 0x00a3_1048), vec![vec![caster, 0]]);
        // Without a current spell: nothing after the actor part.
        returns(&mut e, 0x00a3_1034, 0);
        e.call_log = Some(vec![]);
        magic_caster_interrupt_cast(&mut e, Ptr::new(caster));
        assert!(calls_to(&e, 0x00a3_1024).is_empty());
    }

    #[test]
    fn target_self_applies_to_the_caster_as_target() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        let caster = e.mem.alloc(0xc);
        vtable(&mut e, caster, 0x00a3_0000, &[(0x34, 0x00a3_1034)]);
        returns(&mut e, 0x00a3_1034, 0x00a8_0000);
        let target = e.mem.alloc(8);
        vtable(&mut e, target, 0x00ab_0000, &[(0x0, 0x00ab_1000)]);
        returns(&mut e, 0x00ab_1000, 0x101);
        returns(&mut e, RT_DYNAMIC_CAST, target);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0081_5c50, &args![caster, 0x55u32]).bool());
        assert_eq!(
            calls_to(&e, RT_DYNAMIC_CAST),
            vec![vec![caster, 0, TYPE_MAGIC_CASTER, TYPE_MAGIC_TARGET, 0]]
        );
        assert_eq!(
            calls_to(&e, 0x00ab_1000),
            vec![vec![target, caster, 0x00a8_0000, 0x55, 0]]
        );
        returns(&mut e, RT_DYNAMIC_CAST, 0);
        assert!(!magic_caster_target_self(&mut e, Ptr::new(caster), 0x55));
    }

    #[test]
    fn create_active_effect_forwards_to_the_archetypes() {
        let mut e = Engine::new();
        returns(&mut e, EFFECT_ARCHETYPES_ACTIVATE, 0x1234);
        e.call_log = Some(vec![]);
        let r = e.call(0x0081_5cd0, &args![0x10u32, 0x20u32, 0x30u32, 0x40u32]);
        assert_eq!(r.u32(), 0x1234);
        assert_eq!(
            calls_to(&e, EFFECT_ARCHETYPES_ACTIVATE),
            vec![vec![0x10, 0x20, 0x30, 0x40]]
        );
    }

    #[test]
    fn apply_to_target_skips_the_point() {
        let mut e = Engine::new();
        let target = e.mem.alloc(8);
        vtable(&mut e, target, 0x00ab_0000, &[(0x0, 0x00ab_1000)]);
        returns(&mut e, 0x00ab_1000, 0x7);
        e.call_log = Some(vec![]);
        let words = args![0x10u32, target, 1u32, 2u32, 3u32, 0x40u32, 0x50u32, 0x1ffu32];
        assert_eq!(e.call(0x0081_7940, &words).u32(), 7);
        assert_eq!(
            calls_to(&e, 0x00ab_1000),
            vec![vec![target, 0x10, 0x40, 0x50, 0xff]]
        );
    }

    #[test]
    fn linear_impulse_activates_then_hands_to_the_motion() {
        let mut e = Engine::new();
        returns(&mut e, HKP_ENTITY_ACTIVATE, 0);
        let motion = e.mem.alloc(8);
        vtable(&mut e, motion, 0x00ac_0000, &[(0x4c, 0x00ac_104c)]);
        returns(&mut e, 0x00ac_104c, 0);
        returns(&mut e, HKP_ENTITY_MOTION, motion);
        e.call_log = Some(vec![]);
        e.call(0x0081_82d0, &args![0x900u32, 0x1234u32]);
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .collect();
        assert_eq!(
            order,
            vec![
                0x0081_82d0,
                HKP_ENTITY_ACTIVATE,
                HKP_ENTITY_MOTION,
                0x00ac_104c
            ]
        );
        assert_eq!(calls_to(&e, 0x00ac_104c), vec![vec![motion, 0x1234]]);
    }

    #[test]
    fn state_setter_ignores_the_second_word() {
        let mut e = Engine::new();
        let caster = e.mem.alloc(0xc);
        e.call(0x0081_8af0, &args![caster, 7u32, 0x5555u32]);
        assert_eq!(e.mem.u32(caster + 8), 7);
        assert_eq!(e.mem.u32(caster + 4), 0);
    }

    #[test]
    fn caster_form_id_of_actor_or_non_actor() {
        let mut e = Engine::new();
        returns(&mut e, WORD_AT_C, 0x0001_4000);
        let stats = e.mem.alloc(8);
        vtable(&mut e, stats, 0x00a1_0000, &[(0x100, 0x00a1_1000)]);
        let is_actor = std::rc::Rc::new(std::cell::Cell::new(1u32));
        let flag = is_actor.clone();
        e.register_double(0x00a1_1000, move |_, _| ret(flag.get()));
        let caster = e.mem.alloc(0x100) + 0x88;
        vtable(&mut e, caster, 0x00a2_0000, &[(0x2c, 0x00a2_1000)]);
        returns(&mut e, 0x00a2_1000, stats);
        returns(&mut e, RT_DYNAMIC_CAST, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0081_8be0, &args![caster]).u32(), 0x0001_4000);
        assert_eq!(calls_to(&e, WORD_AT_C), vec![vec![caster - 0x88]]);
        // A non-actor caster with a reference.
        is_actor.set(0);
        let non_actor = e.mem.alloc(0x20);
        vtable(&mut e, non_actor + 0xc, 0x00ad_0000, &[(0x2c, 0x00ad_102c)]);
        returns(&mut e, 0x00ad_102c, 0x00ae_0000);
        returns(&mut e, RT_DYNAMIC_CAST, non_actor);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0081_8be0, &args![caster]).u32(), 0x0001_4000);
        assert_eq!(calls_to(&e, WORD_AT_C), vec![vec![0x00ae_0000]]);
        // No reference: 0.
        returns(&mut e, 0x00ad_102c, 0);
        assert_eq!(
            magic_caster_get_magic_caster_form_id(&mut e, Ptr::new(caster)),
            0
        );
    }

    /// A non-actor caster casting a plain spell (kind 0) of one self-range
    /// effect: the effect is created, its effectiveness scaled by the
    /// caster's modifier, applied to the caster, and deleted; the state and
    /// current spell are cleared.
    #[test]
    fn find_targets_applies_a_self_effect() {
        let mut e = Engine::new();
        let caster = e.mem.alloc(0xc);
        let stats = e.mem.alloc(8);
        let spell = e.mem.alloc(0x40);
        let effect = e.mem.alloc(0x20);
        let active = e.mem.alloc(0x40);
        vtable(
            &mut e,
            caster,
            0x00a3_0000,
            &[
                (0x2c, 0x00a3_102c),
                (0x30, 0x00a3_1030),
                (0x34, 0x00a3_1034),
                (0x38, 0x00a3_1038),
                (0x3c, 0x00a3_103c),
                (0x40, 0x00a3_1040),
                (0x48, 0x00a3_1048),
                (0x4c, 0x00a3_104c),
            ],
        );
        returns(&mut e, 0x00a3_102c, stats);
        returns(&mut e, 0x00a3_1030, 0);
        returns(&mut e, 0x00a3_1034, spell);
        returns(&mut e, 0x00a3_1038, 1);
        e.register(0x00a3_103c, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        returns(&mut e, 0x00a3_1040, 0);
        returns(&mut e, 0x00a3_1048, 0);
        returns(&mut e, 0x00a3_104c, active);
        vtable(&mut e, stats, 0x00a1_0000, &[(0x100, 0x00a1_1000)]);
        returns(&mut e, 0x00a1_1000, 0);
        vtable(&mut e, spell, 0x00a4_0000, &[(0x18, 0x00a4_1018)]);
        returns(&mut e, 0x00a4_1018, 0);
        vtable(&mut e, active, 0x00af_0000, &[(0x0, 0x00af_1000)]);
        returns(&mut e, 0x00af_1000, 0);
        // The spell's effect list: one node at +0x10 holding `effect`.
        e.mem.set_u32(spell + 0x10, effect);
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(WORD_AT_C, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.mem.set_u32(active + 0xc, 0x00b0_0000);
        returns(&mut e, EFFECT_ITEM_SETTING, 0x00b1_0000);
        e.register(SETTING_DURATION, |_, _| Ret {
            st0: 10.0,
            ..Ret::default()
        });
        returns(&mut e, HAS_TOUCH_EFFECT, 0);
        returns(&mut e, HAS_TARGET_EFFECT, 0);
        returns(&mut e, RT_DYNAMIC_CAST, 0);
        e.register(GET_COSTLIEST_EFFECT, |e, a| {
            // The costliest self effect is the one effect.
            if a[1] == 0 {
                ret(e.mem.u32(a[0] + 4))
            } else {
                ret(0)
            }
        });
        for callee in [
            LOCAL_LIST_CONSTRUCT,
            LOCAL_LIST_REMOVE_ALL,
            LOCAL_LIST_DESTRUCT,
            ACTIVE_EFFECT_ADJUST_EFFECTIVENESS,
            GET_AV_EFFECT,
        ] {
            returns(&mut e, callee, 0);
        }
        e.call_log = Some(vec![]);
        let words = args![caster, 0.0f32, 0u32, 0u32, 2.0f32, 0u32];
        e.call(0x0081_5d00, &words);
        assert_eq!(
            calls_to(&e, 0x00a3_104c),
            vec![vec![caster, spell, effect, 0]]
        );
        assert_eq!(
            calls_to(&e, 0x00a3_103c),
            vec![vec![caster, 0, 0.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, ACTIVE_EFFECT_ADJUST_EFFECTIVENESS),
            vec![vec![active, 1.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, 0x00a3_1038), vec![vec![caster, active]]);
        assert_eq!(calls_to(&e, 0x00af_1000), vec![vec![active, 1]]);
        assert!(calls_to(&e, ACTIVE_EFFECT_SET_FLAG).is_empty());
        assert_eq!(calls_to(&e, 0x00a3_1040), vec![vec![caster, 0]]);
        assert_eq!(calls_to(&e, 0x00a3_1048), vec![vec![caster, 0]]);
        assert_eq!(e.mem.u32(caster + 8), 0);
        assert_eq!(calls_to(&e, LOCAL_LIST_DESTRUCT).len(), 1);
        // Without a current spell nothing else happens.
        returns(&mut e, 0x00a3_1034, 0);
        e.call_log = Some(vec![]);
        magic_caster_find_targets(&mut e, Ptr::new(caster), 0.0, 0, 0, 1.0, 0);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }
}

// ---------------------------------------------------------------------------
// The rest of `magiccaster.cpp`.
// ---------------------------------------------------------------------------

/// `0040a300` with the spell: `MagicItem::GetAVEffect` (see `GET_AV_EFFECT`).
/// `004062a0` (`thiscall`, `ECX` = the spell's effect list at +0xC): a `bool` that refuses
/// the cast.
const SPELL_DISABLED: u32 = 0x0040_62a0;
/// `"CAST: Spell is disabled"`, `"CAST: Already Casting Spell"`, `"Release Cast"` (state
/// descriptions, unread by the setter).
const TEXT_SPELL_DISABLED: u32 = 0x0107_acc8;
const TEXT_ALREADY_CASTING: u32 = 0x0107_acac;
const TEXT_RELEASE_CAST: u32 = 0x0107_ace0;
/// `.?AVIngredientItem@@` is `TYPE_INGREDIENT_ITEM`; `.?AVActor@@`, `.?AVCharacter@@` and
/// `.?AVMobileObject@@`.
const TYPE_ACTOR: u32 = 0x0118_46d4;
const TYPE_CHARACTER: u32 = 0x011a_0f14;
const TYPE_MOBILE_OBJECT: u32 = 0x0118_4920;
/// `00408da0` (`thiscall`, `ECX` = +0x30 of an object): a word; `00511840` (`thiscall`): a
/// sound form; `00933010` (`thiscall`, `RET 0x10`): plays a sound on the actor into a handle;
/// the handle's destructor `00483710`.
const OBJECT_SOUND_TEST: u32 = 0x0040_8da0;
const OBJECT_SOUND: u32 = 0x0051_1840;
const ACTOR_PLAY_SOUND: u32 = 0x0093_3010;
const SOUND_HANDLE_DESTRUCT: u32 = 0x0048_3710;
/// The spell's name into a string (`0040bef0`, `RET 8`), the string's text (`00559450`) and
/// destructor (`004037d0`).
const SPELL_NAME: u32 = 0x0040_bef0;
const STRING_TEXT: u32 = 0x0055_9450;
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// `007052f0` (`cdecl`, six words): shows a message (the text, 0, 0, 0, a `float`, 0) and the
/// `float` (`2.0`, `010162c0`).
const SHOW_MESSAGE: u32 = 0x0070_52f0;
const MESSAGE_TIME: u32 = 0x0101_62c0;
/// `MagicLight::Fade` (`0081c470`, `RET 8`), the `0.75` it gets on a release (`01016264`) and
/// the light's update (`0081c320`, `RET 4`).
const MAGIC_LIGHT_FADE: u32 = 0x0081_c470;
const RELEASE_FADE: u32 = 0x0101_6264;
const MAGIC_LIGHT_UPDATE: u32 = 0x0081_c320;
/// `0045c650` (`thiscall`): the character proxy's object list; `004ae750` the character
/// proxy of a phantom (`magicballprojectile.rs`).
const PROXY_LIST: u32 = 0x0045_c650;
const CHARACTER_PROXY: u32 = 0x004a_e750;
/// `004839c0` (`cdecl`): the form of an id.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `0044ddc0` (`thiscall`): an active effect's spell; `007df1d0` its effect setting;
/// `007df1f0` its duration (`ST0`); `00495460` its magnitude (`ST0`).
const ACTIVE_EFFECT_SPELL: u32 = 0x0044_ddc0;
const ACTIVE_EFFECT_SETTING: u32 = 0x007d_f1d0;
const ACTIVE_EFFECT_DURATION: u32 = 0x007d_f1f0;
const ACTIVE_EFFECT_MAGNITUDE: u32 = 0x0049_5460;
/// `00403e80` (`thiscall`): an effect setting's `bool`; `00586150` (`thiscall`): its actor
/// value.
const SETTING_USES_OWN_VALUE: u32 = 0x0040_3e80;
const SETTING_ACTOR_VALUE: u32 = 0x0058_6150;
/// `EffectItemList::IsHostile` (engine map, `00405d80`).
const EFFECT_LIST_IS_HOSTILE: u32 = 0x0040_5d80;
/// `00568ad0` (`thiscall`, `ST0`): the actor's health; `00404010` (`cdecl`, two `float`s,
/// `ST0`), `0040ebd0` (`cdecl`, two `float`s, `ST0`), `4.0` (`float`, `0101e570`);
/// `Actor::DifficultyLevelAdjustHealthModifier` (`008808a0`, `RET 8`).
const ACTOR_HEALTH: u32 = 0x0056_8ad0;
const DURATION_PART: u32 = 0x0040_4010;
const DURATION_SCALE: u32 = 0x0040_ebd0;
const FOUR: u32 = 0x0101_e570;
const ADJUST_HEALTH_MODIFIER: u32 = 0x0088_08a0;
/// `00717e50` on the process lists (`011e0e80`): the high actors; `005be5c0` (`RET 4`) their
/// count; `00968670` (`RET 4`) one of them.
const PROCESS_LISTS: u32 = 0x011e_0e80;
const HIGH_ACTORS: u32 = 0x0071_7e50;
const ACTOR_COUNT: u32 = 0x005b_e5c0;
const ACTOR_AT: u32 = 0x0096_8670;
/// `CombatUtilities` helpers: `009a6ae0` (`cdecl`, three words: a `bool`, the distance out).
const ACTOR_REACHES: u32 = 0x009a_6ae0;
/// `FLT_MAX` (`float`, `01016970`) and the default touch distance setting (`011cf84c`).
const FLOAT_MAX: u32 = 0x0101_6970;
const DEFAULT_TOUCH_SETTING: u32 = 0x011c_f84c;
/// `008d8520` (`thiscall`): the actor's process; `0045cd60` (`thiscall`): a word of it.
const ACTOR_PROCESS: u32 = 0x008d_8520;
const PROCESS_BUSY: u32 = 0x0045_cd60;
/// `00572380` (`RET 4`, `ST0`): the reference's distance to a point.
const DISTANCE_TO_POINT: u32 = 0x0057_2380;
/// `008ace90` (`thiscall`): a `bool` of the actor that excludes it.
const ACTOR_EXCLUDED: u32 = 0x008a_ce90;
/// `005ae3d0` (`RET 4`): adds a reference (through its address) to the target list.
const LIST_ADD: u32 = 0x005a_e3d0;
/// The area setting factor (`011d15e0`), `0.5` (`double`, `01011588`), `2.0` (`double`,
/// `01011590`), `0.0` (`double`, `01012060`), `0.75` (`double`, `0101de30`), `0.7`
/// (`double`, `0104ee98`).
const AREA_FACTOR: u32 = 0x011d_15e0;
const HALF: u32 = 0x0101_1588;
const TWO: u32 = 0x0101_1590;
const ZERO: u32 = 0x0101_2060;
const THREE_QUARTERS: u32 = 0x0101_de30;
const SEVEN_TENTHS: u32 = 0x0104_ee98;
/// `_ftol2` (`00ec62c0`, `ST0` passed as an `f64`).
const FTOL: u32 = 0x00ec_62c0;

fn ftol(e: &mut Engine, value: f64) -> u32 {
    e.call(FTOL, &args![value]).u32()
}

fn point_words(e: &mut Engine, point: u32) -> [u32; 3] {
    [
        e.mem.u32(point),
        e.mem.u32(point.wrapping_add(4)),
        e.mem.u32(point.wrapping_add(8)),
    ]
}

fn put_point(e: &mut Engine, point: u32, words: [u32; 3]) {
    for (i, word) in words.iter().enumerate() {
        e.mem.set_u32(point.wrapping_add(4 * i as u32), *word);
    }
}

/// The `float` value of a setting (its address from `00403e20`), loaded (`FLD`).
fn setting_load(e: &mut Engine, setting: u32) -> f64 {
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    x87_load32(e.mem.u32(value))
}

// Translated from 00815380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster::StartCast` (Xbox PDB): [`magic_caster_release_cast`] with 0.
pub fn magic_caster_start_cast(e: &mut Engine, this: Ptr) {
    magic_caster_release_cast(e, this, 0);
}

// Translated from 008153a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`, both words unread): `1.0` in `ST0`.
pub fn fn_008153a0(_e: &mut Engine, _this: Ptr, _unused_1: u32, _unused_2: u32) -> f64 {
    1.0
}

// Translated from 00815480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster::CastSpell` (Xbox PDB; `thiscall`, `RET 0xC`: spell, target, flag). A
/// disabled spell (`004062a0`) sets state 5 and fails. By the spell's type (slot +0x18): 1
/// goes to slot +0x10 (the `SpellItem` cast, the target, the flag), 4 to slot 0 and 0xA to
/// slot +4 (the cast and the flag); others fail with state 6 while a spell is current, or
/// become the current spell (slot +0x40) with the target as desired target (slot +0x48)
/// and start: slot +0x1C for a target effect with a target, slot +0x20 otherwise.
pub fn magic_caster_cast_spell(
    e: &mut Engine,
    this: Ptr,
    spell: u32,
    target: u32,
    flag: u8,
) -> bool {
    let base = this.addr();
    if e.call(SPELL_DISABLED, &args![spell.wrapping_add(0xc)])
        .bool()
    {
        fn_00818af0(e, this, 5, TEXT_SPELL_DISABLED);
        return false;
    }
    match e.vcall(spell, 0x18, &[]).u32() {
        1 => {
            let cast = dynamic_cast(e, spell, TYPE_MAGIC_ITEM, TYPE_SPELL_ITEM);
            e.vcall(base, 0x10, &args![cast, target, flag]);
            true
        }
        4 => {
            let cast = dynamic_cast(e, spell, TYPE_MAGIC_ITEM, TYPE_SPELL_ITEM);
            e.vcall(base, 0, &args![cast, flag]);
            true
        }
        0xa => {
            let cast = dynamic_cast(e, spell, TYPE_MAGIC_ITEM, TYPE_SPELL_ITEM);
            e.vcall(base, 4, &args![cast, flag]);
            true
        }
        _ => {
            if e.vcall(base, 0x34, &[]).u32() != 0 {
                fn_00818af0(e, this, 6, TEXT_ALREADY_CASTING);
                return false;
            }
            e.vcall(base, 0x40, &args![spell]);
            e.vcall(base, 0x48, &args![target]);
            if e.call(HAS_TARGET_EFFECT, &args![spell.wrapping_add(0xc)])
                .bool()
                && target != 0
            {
                e.vcall(base, 0x1c, &[]);
            } else {
                e.vcall(base, 0x20, &[]);
            }
            true
        }
    }
}

// Translated from 00815670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster::CastAddiction` (Xbox PDB; `thiscall`, `RET 8`): for a spell of type 0xA,
/// makes it the current spell and runs `FindTargets(0.0, 0, flag, 1.0, 0)`.
pub fn magic_caster_cast_addiction(e: &mut Engine, this: Ptr, spell: u32, flag: u8) {
    if e.vcall(spell.wrapping_add(0x18), 0x18, &[]).u32() == 0xa {
        let item = if spell != 0 {
            spell.wrapping_add(0x18)
        } else {
            0
        };
        e.vcall(this.addr(), 0x40, &args![item]);
        magic_caster_find_targets(e, this, 0.0, 0, flag, 1.0, 0);
    }
}

// Translated from 008156e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster::CastRadiation` (Xbox PDB; `thiscall`, `RET 8`): makes the spell the current
/// spell and runs `FindTargets(0.0, 0, flag, 1.0, 0)`.
pub fn magic_caster_cast_radiation(e: &mut Engine, this: Ptr, spell: u32, flag: u8) {
    let item = if spell != 0 {
        spell.wrapping_add(0x18)
    } else {
        0
    };
    e.vcall(this.addr(), 0x40, &args![item]);
    magic_caster_find_targets(e, this, 0.0, 0, flag, 1.0, 0);
}

// Translated from 00815870 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00815870 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// `MagicCaster::ReleaseCast` (Xbox PDB; `thiscall`, `RET 4`: the source). Without a
/// current spell, false. When `CheckCast` (slot +0x28 with the spell, a `float` and a word
/// it fills, 0) fails: (an object-sound branch the exe guards with a local it has just set
/// to null, translated as the exe has it), the spell's name into a local string, a message
/// for an actor caster whose slot +0x360 is true (ingredients only when their slot +4 says
/// so), state 7, `SpellCast(spell, 0)`, no current spell or desired target; false. Otherwise
/// state 3, `FindTargets(the float, source, 0, 1.0, 0)`, `SpellCast(spell, 1)`, the light
/// fades over `0.75`; true.
pub fn magic_caster_release_cast(e: &mut Engine, this: Ptr, source: u32) -> bool {
    let base = this.addr();
    if e.vcall(base, 0x34, &[]).u32() == 0 {
        return false;
    }
    e.with_stack(0x30, |e, locals| {
        let value = locals.addr();
        let word = value.wrapping_add(4);
        let name = value.wrapping_add(8);
        let handle = value.wrapping_add(0x10);
        e.mem.set_u32(value, 0.0f32.to_bits());
        let spell = e.vcall(base, 0x34, &[]).u32();
        let ingredient = dynamic_cast(e, spell, TYPE_MAGIC_ITEM, TYPE_INGREDIENT_ITEM);
        e.mem.set_u32(word, 0);
        let spell = e.vcall(base, 0x34, &[]).u32();
        if e.vcall(base, 0x28, &args![spell, value, word, 0u32]).bool() {
            fn_00818af0(e, this, 3, TEXT_RELEASE_CAST);
            let spell = e.vcall(base, 0x34, &[]).u32();
            let amount = f32::from_bits(x87_quiet_bits(e.mem.u32(value)));
            magic_caster_find_targets(e, this, amount, source, 0, 1.0, 0);
            e.vcall(base, 0x24, &args![spell, 1u32]);
            let light = e.mem.u32(base.wrapping_add(4));
            if light != 0 {
                let fade = x87_quiet_bits(e.mem.u32(RELEASE_FADE));
                e.call(MAGIC_LIGHT_FADE, &args![light, 0u32, fade]);
            }
            return true;
        }
        let actor = magic_caster_get_caster_as_actor(e, this).addr();
        let spell = e.vcall(base, 0x34, &[]).u32();
        let effect = e.call(GET_AV_EFFECT, &args![spell, 0u32]).u32();
        if actor != 0 && effect != 0 {
            let object = 0u32;
            if object != 0
                && e.call(OBJECT_SOUND_TEST, &args![object.wrapping_add(0x30)])
                    .u32()
                    != 0
                && ingredient == 0
            {
                let sound = e.call(OBJECT_SOUND, &args![object]).u32();
                e.call(
                    ACTOR_PLAY_SOUND,
                    &args![actor, handle, sound, 0u32, 0x4000_0102u32],
                );
                e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
            }
        }
        let count = e.mem.u32(word);
        let spell = e.vcall(base, 0x34, &[]).u32();
        e.call(SPELL_NAME, &args![spell, name, count]);
        if actor != 0
            && e.vcall(actor, 0x360, &[]).bool()
            && (ingredient == 0 || e.vcall(ingredient.wrapping_add(0x3c), 4, &[]).bool())
        {
            let text = e.call(STRING_TEXT, &args![name]).u32();
            let time = x87_quiet_bits(e.mem.u32(MESSAGE_TIME));
            e.call(SHOW_MESSAGE, &args![text, 0u32, 0u32, 0u32, time, 0u32]);
        }
        let text = e.call(STRING_TEXT, &args![name]).u32();
        fn_00818af0(e, this, 7, text);
        let spell = e.vcall(base, 0x34, &[]).u32();
        e.vcall(base, 0x24, &args![spell, 0u32]);
        e.vcall(base, 0x40, &args![0u32]);
        e.vcall(base, 0x48, &args![0u32]);
        e.call(STRING_DESTRUCT, &args![name]);
        false
    })
}

// Translated from 00816f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster::FindTargetsInArea` (Xbox PDB; `thiscall`, `RET 0x2C`: spell, active
/// effect, location, a point by value, the touch target, the projectile, the reference
/// list, a found flag's address, a scale). With a location and an area (the effect item's,
/// times the setting at `011d15e0` times the scale truncated): for each reference of the
/// list (stopping at an empty entry) whose magic target (slot +0x19C) exists, is not the
/// caster actor's, not the touch target, with 3D, within the area (`00572380`), not an
/// excluded actor, accepted by the projectile (slot +0x318) and hit
/// ([`fn_00817940`] with 1): the projectile's slot +0x308; and once (no touch target, not yet
/// found, an actor caster, spell type 0 or 5) the caster's skill advances (slot +0x480).
#[allow(clippy::too_many_arguments)]
pub fn magic_caster_find_targets_in_area(
    e: &mut Engine,
    this: Ptr,
    spell: u32,
    effect: u32,
    location: u32,
    x: u32,
    y: u32,
    z: u32,
    touch_target: u32,
    projectile: u32,
    list: u32,
    found: u32,
    scale: f32,
) {
    if location == 0 {
        return;
    }
    let item = e.call(WORD_AT_C, &args![effect]).u32();
    let mut area = e.call(EFFECT_ITEM_GET_AREA, &args![item]).i32();
    if area <= 0 {
        return;
    }
    let factor = x87_mul(setting_load(e, AREA_FACTOR), x87_m32(scale.to_bits()));
    area = (ftol(e, factor) as i32).wrapping_mul(area);
    let actor = magic_caster_get_caster_as_actor(e, this).addr();
    let mut already_found = e.mem.u8(found) != 0;
    e.with_stack(0x0c, |e, point| {
        let point = point.addr();
        put_point(e, point, [x, y, z]);
        let mut node = list;
        while node != 0 {
            let item = e.call(NODE_ITEM, &args![node]).u32();
            if e.mem.u32(item) == 0 {
                return;
            }
            let item = e.call(NODE_ITEM, &args![node]).u32();
            let reference = e.mem.u32(item);
            if reference != 0 {
                let target = e.vcall(reference, 0x19c, &[]).u32();
                let target_actor = if target != 0 {
                    e.call(GET_TARGET_AS_ACTOR, &args![target]).u32()
                } else {
                    0
                };
                let is_caster = actor != 0 && target_actor == actor;
                if target != 0
                    && !is_caster
                    && target != touch_target
                    && e.vcall(reference, 0x1d0, &[]).u32() != 0
                {
                    let distance =
                        x87_store32(e.call(DISTANCE_TO_POINT, &args![reference, point]).f64());
                    // FCOMPP (the distance against the area) / TEST AH,0x41 / JP: within
                    // when below or equal.
                    let within = x87_load32(distance) <= f64::from(area);
                    if within
                        && !(target_actor != 0
                            && e.call(ACTOR_EXCLUDED, &args![target_actor]).bool())
                        && (projectile == 0
                            || e.vcall(projectile, 0x318, &args![target, effect]).bool())
                        && fn_00817940(e, this, target, x, y, z, spell, effect, 1) as u8 != 0
                    {
                        if projectile != 0 {
                            e.vcall(projectile, 0x308, &args![target, effect]);
                        }
                        if touch_target == 0 && !already_found && actor != 0 {
                            let kind = e.vcall(spell, 0x18, &[]).u32();
                            if kind == 0 || e.vcall(spell, 0x18, &[]).u32() == 5 {
                                already_found = true;
                                e.mem.set_u8(found, 1);
                                let skill = e
                                    .call(GET_ASSOCIATED_SKILL, &args![spell.wrapping_add(0xc)])
                                    .u32();
                                e.vcall(actor, 0x480, &args![skill, 1u32, 0.0f32]);
                            }
                        }
                    }
                }
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    });
}

/// `MagicTarget::GetTargetAsActor` (engine map, `00822b40`).
const GET_TARGET_AS_ACTOR: u32 = 0x0082_2b40;

// Translated from 00817160 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00817160 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// `MagicCaster::FindTargets_ov2` (Xbox PDB; `thiscall`, `RET 0x28`: spell, location, a point
/// by value, the projectile, the reference hit, a word it never reads, effectiveness, scale):
/// a projectile's targets. The effectiveness is the caster's (slot +0x3C, for a spell of
/// type 0; `1.0` otherwise) times the argument; the targets near the point are collected
/// (`00818ce0` with range 2) into a local list. For each target-range effect item an active
/// effect is created (slot +0x4C), marked secondary unless it is the costliest, scaled, and
/// applied to the hit reference's magic target (when it is not the caster's, the projectile
/// accepts it and [`fn_00817940`] hits: the projectile caster's skill advances once for
/// spell types 0 and 5), then to the area ([`magic_caster_find_targets_in_area`]) and
/// deleted. Then, unless the projectile form's `004086a0` is 3: [`magic_caster_do_explosion`]
/// for the costliest area effect, or, for an actor hit by a costliest effect whose setting
/// has flag `0x20000000` and which would kill it ([`magic_caster_check_will_kill`]),
/// [`magic_caster_explode_actor`] with the effect's raw cost.
#[allow(clippy::too_many_arguments)]
pub fn magic_caster_find_targets_ov2(
    e: &mut Engine,
    this: Ptr,
    spell: u32,
    location: u32,
    x: u32,
    y: u32,
    z: u32,
    projectile: u32,
    reference: u32,
    _unused_8: u32,
    effectiveness: f32,
    scale: f32,
) {
    let base = this.addr();
    let mut entry = if spell != 0 {
        spell.wrapping_add(0xc)
    } else {
        0
    };
    let factor = if e.vcall(spell, 0x18, &[]).u32() == 0 {
        x87_store32(e.vcall(base, 0x3c, &args![0u32, 0.0f32]).f64())
    } else {
        1.0f32.to_bits()
    };
    let factor = x87_store32(x87_mul(
        x87_load32(factor),
        x87_m32(effectiveness.to_bits()),
    ));
    let costliest = e
        .call(
            GET_COSTLIEST_EFFECT,
            &args![spell.wrapping_add(0xc), 2u32, 0u32],
        )
        .u32();
    e.with_stack(0x10, |e, locals| {
        let list = locals.addr();
        let found = list.wrapping_add(8);
        e.call(LOCAL_LIST_CONSTRUCT, &args![list]);
        e.call(LOCAL_LIST_REMOVE_ALL, &args![list]);
        fn_00818ce0(e, this, spell, 2, x, y, z, list);
        e.mem.set_u8(found, 0);
        while entry != 0 {
            let item_slot = e.call(NODE_ITEM, &args![entry.wrapping_add(4)]).u32();
            let item = e.mem.u32(item_slot);
            if item != 0 && e.call(WORD_AT_C, &args![item]).u32() == 2 {
                let effect = e.vcall(base, 0x4c, &args![spell, item, 0u32]).u32();
                if item != costliest {
                    flag_secondary(e, effect);
                }
                e.call(
                    ACTIVE_EFFECT_ADJUST_EFFECTIVENESS,
                    &args![effect, x87_quiet_bits(factor)],
                );
                if reference != 0
                    && e.vcall(reference, 0x19c, &[]).u32() != 0
                    && reference != e.vcall(base, 0x2c, &[]).u32()
                {
                    let accepted = projectile == 0 || {
                        let target = e.vcall(reference, 0x19c, &[]).u32();
                        e.vcall(projectile, 0x318, &args![target, effect]).bool()
                    };
                    if accepted {
                        let target = e.vcall(reference, 0x19c, &[]).u32();
                        if fn_00817940(e, this, target, x, y, z, spell, effect, 0) as u8 != 0 {
                            let caster = e.call(PROJECTILE_CASTER, &args![projectile]).u32();
                            let actor = if caster != 0 {
                                magic_caster_get_caster_as_actor(e, Ptr::new(caster)).addr()
                            } else {
                                0
                            };
                            let advance = if actor != 0
                                && e.mem.u8(found) == 0
                                && e.vcall(spell, 0x18, &[]).u32() == 0
                            {
                                true
                            } else {
                                e.vcall(spell, 0x18, &[]).u32() == 5
                            };
                            if advance {
                                e.mem.set_u8(found, 1);
                                let skill = e
                                    .call(GET_ASSOCIATED_SKILL, &args![spell.wrapping_add(0xc)])
                                    .u32();
                                e.vcall(actor, 0x480, &args![skill, 1u32, 0.0f32]);
                            }
                        }
                    }
                }
                let target = if reference != 0 {
                    e.vcall(reference, 0x19c, &[]).u32()
                } else {
                    0
                };
                magic_caster_find_targets_in_area(
                    e,
                    this,
                    spell,
                    effect,
                    location,
                    x,
                    y,
                    z,
                    target,
                    projectile,
                    list,
                    found,
                    f32::from_bits(x87_quiet_bits(scale.to_bits())),
                );
                if effect != 0 {
                    e.vcall(effect, 0, &args![1u32]);
                }
            }
            let next = e.call(NODE_NEXT, &args![entry.wrapping_add(4)]).u32();
            entry = if next != 0 { next.wrapping_sub(4) } else { 0 };
        }
        let form = e.call(BASE_FORM, &args![projectile]).u32();
        let no_explosion = e.call(PROJECTILE_KIND_OF_FORM, &args![form]).u32() == 3;
        let area_effect = e
            .call(
                GET_COSTLIEST_EFFECT,
                &args![spell.wrapping_add(0xc), 2u32, 1u32],
            )
            .u32();
        if !no_explosion {
            if area_effect != 0 {
                magic_caster_do_explosion(
                    e,
                    this,
                    x,
                    y,
                    z,
                    location,
                    spell,
                    area_effect,
                    list,
                    f32::from_bits(x87_quiet_bits(effectiveness.to_bits())),
                    f32::from_bits(x87_quiet_bits(scale.to_bits())),
                );
            } else if reference != 0 && e.vcall(reference, 0x100, &[]).bool() && costliest != 0 {
                let setting = e.call(EFFECT_ITEM_SETTING, &args![costliest]).u32();
                if e.call(SETTING_HAS_FLAG, &args![setting, 0x2000_0000u32])
                    .bool()
                {
                    let target = if reference != 0 {
                        reference.wrapping_add(0x94)
                    } else {
                        0
                    };
                    if magic_caster_check_will_kill(e, this, target, spell) {
                        let cost =
                            x87_store32(e.call(EFFECT_ITEM_GET_RAW_COST, &args![costliest]).f64());
                        magic_caster_explode_actor(
                            e,
                            this,
                            reference,
                            x,
                            y,
                            z,
                            costliest,
                            f32::from_bits(cost),
                        );
                    }
                }
            }
        }
        e.call(LOCAL_LIST_REMOVE_ALL, &args![list]);
        e.call(LOCAL_LIST_DESTRUCT, &args![list]);
    });
}

/// `009185e0` (`thiscall`): the projectile's magic caster; `004fd400` its base form;
/// `004086a0` (`thiscall`): the projectile form's kind.
const PROJECTILE_CASTER: u32 = 0x0091_85e0;
const BASE_FORM: u32 = 0x004f_d400;
const PROJECTILE_KIND_OF_FORM: u32 = 0x0040_86a0;

/// [`magic_caster_get_touch_target`]'s test of one candidate against the best so far.
#[allow(clippy::too_many_arguments)]
fn consider_touch_candidate(
    e: &mut Engine,
    actor: u32,
    caster: u32,
    candidate: u32,
    reach: u32,
    best: &mut u32,
    best_distance: &mut u32,
) {
    let distance = e
        .call(
            DISTANCE_BETWEEN_FACING_ACTORS,
            &args![caster, candidate, 0u32],
        )
        .f64();
    // FCOMPP / TEST AH,1 / JNZ: on when the reach is not below (ordered).
    if !(x87_load32(reach) >= distance) {
        return;
    }
    e.with_stack(4, |e, out| {
        e.mem
            .set_u32(out.addr(), x87_quiet_bits(e.mem.u32(FLOAT_MAX)));
        let reaches = if actor != 0 {
            e.call(ACTOR_REACHES, &args![actor, candidate, out]).bool()
        } else {
            true
        };
        let measured = e.mem.u32(out.addr());
        if reaches && x87_load32(*best_distance) >= x87_load32(measured) {
            *best = candidate;
            *best_distance = x87_quiet_bits(measured);
        }
    });
}

// Translated from 00817620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster::GetTouchTarget` (Xbox PDB): the nearest actor in reach: the reach is an
/// actor caster's slot +0x380 times its scale (or the setting at `011cf84c`); the
/// candidates are the player and every high actor (`00717e50`: alive by slot +0x22C, with
/// 3D, not the caster) within the reach (`009a64d0`) that the caster actor reaches
/// (`009a6ae0`) nearer than the best so far. Answers the best's magic target (+0x94), or
/// null. (An inner walk over a list the exe starts empty is translated as the exe has it.)
pub fn magic_caster_get_touch_target(e: &mut Engine, this: Ptr) -> u32 {
    let actor = magic_caster_get_caster_as_actor(e, this).addr();
    let reach = if actor != 0 {
        let own = e.vcall(actor, 0x380, &[]).f64();
        let scale = e.call(GET_SCALE, &args![actor]).f64();
        x87_store32(x87_mul(scale, own))
    } else {
        x87_store32(setting_load(e, DEFAULT_TOUCH_SETTING))
    };
    let caster = if actor != 0 {
        actor
    } else {
        e.vcall(this.addr(), 0x2c, &[]).u32()
    };
    let mut best = 0u32;
    let mut best_distance = x87_quiet_bits(e.mem.u32(FLOAT_MAX));
    let player = e.mem.u32(PLAYER);
    consider_touch_candidate(
        e,
        actor,
        caster,
        player,
        reach,
        &mut best,
        &mut best_distance,
    );
    let actors = e.call(HIGH_ACTORS, &args![PROCESS_LISTS]).u32();
    let mut index = 0u32;
    while index < e.call(ACTOR_COUNT, &args![actors, 0u32]).u32() {
        let item = e.call(ACTOR_AT, &args![actors, index]).u32();
        let mut candidate = if item != 0 && e.vcall(item, 0x100, &[]).bool() {
            item
        } else {
            0
        };
        let mut node = 0u32;
        while candidate != 0 {
            if candidate != caster
                && !e.vcall(candidate, 0x22c, &args![0u32]).bool()
                && e.vcall(candidate, 0x1d0, &[]).u32() != 0
            {
                consider_touch_candidate(
                    e,
                    actor,
                    caster,
                    candidate,
                    reach,
                    &mut best,
                    &mut best_distance,
                );
            }
            candidate = next_live_character(e, &mut node);
        }
        index = index.wrapping_add(1);
    }
    if best != 0 {
        best.wrapping_add(0x94)
    } else {
        0
    }
}

/// The exe's walk to the next `Character` of a list whose process is idle; the list starts
/// empty in both callers, so it answers null at once.
fn next_live_character(e: &mut Engine, node: &mut u32) -> u32 {
    if *node == 0 {
        return 0;
    }
    let item = e.call(NODE_ITEM, &args![*node]).u32();
    if e.mem.u32(item) == 0 {
        return 0;
    }
    let item = e.call(NODE_ITEM, &args![*node]).u32();
    let reference = e.mem.u32(item);
    let mut candidate = dynamic_cast(e, reference, TYPE_ACTOR, TYPE_CHARACTER);
    *node = e.call(NODE_NEXT, &args![*node]).u32();
    while candidate != 0 {
        let process = e.call(ACTOR_PROCESS, &args![candidate]).u32();
        if process != 0 {
            let process = e.call(ACTOR_PROCESS, &args![candidate]).u32();
            if e.call(PROCESS_BUSY, &args![process]).u32() == 0 {
                return candidate;
            }
        }
        candidate = 0;
        if *node != 0 {
            let item = e.call(NODE_ITEM, &args![*node]).u32();
            let reference = e.mem.u32(item);
            candidate = dynamic_cast(e, reference, TYPE_ACTOR, TYPE_CHARACTER);
        }
        if *node != 0 {
            *node = e.call(NODE_NEXT, &args![*node]).u32();
        }
    }
    0
}

// Translated from 00817970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster::CheckWillKill` (Xbox PDB; `thiscall`, `RET 8`: a magic target, the spell):
/// for a hostile spell on an actor with 3D, adds up the health damage of the target's
/// active effects of that spell (actor value 0x10, setting flag 4 without flag 2: the
/// magnitude, times `0040ebd0(4.0, 00404010(0.0, duration))` with a duration), adjusts it
/// by difficulty (`008808a0` with the caster actor) and answers whether the actor's health
/// is at most that.
pub fn magic_caster_check_will_kill(e: &mut Engine, this: Ptr, target: u32, spell: u32) -> bool {
    if target == 0 || spell == 0 {
        return false;
    }
    if e.vcall(target, 4, &[]).u32() == 0 {
        return false;
    }
    let reference = e.vcall(target, 4, &[]).u32();
    if !e.vcall(reference, 0x100, &[]).bool() {
        return false;
    }
    let reference = e.vcall(target, 4, &[]).u32();
    if e.vcall(reference, 0x1d0, &[]).u32() == 0 {
        return false;
    }
    if !e
        .call(EFFECT_LIST_IS_HOSTILE, &args![spell.wrapping_add(0xc)])
        .bool()
    {
        return false;
    }
    let caster_actor = magic_caster_get_caster_as_actor(e, this).addr();
    let actor = e.vcall(target, 4, &[]).u32();
    let health = x87_store32(e.call(ACTOR_HEALTH, &args![actor]).f64());
    let mut node = e.vcall(target, 8, &[]).u32();
    let mut total = 0.0f32.to_bits();
    while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
        let item = e.call(NODE_ITEM, &args![node]).u32();
        let effect = e.mem.u32(item);
        if effect != 0 && e.call(ACTIVE_EFFECT_SPELL, &args![effect]).u32() == spell {
            let setting = e.call(ACTIVE_EFFECT_SETTING, &args![effect]).u32();
            let value = if e.call(SETTING_USES_OWN_VALUE, &args![setting]).bool() {
                e.call(SETTING_ACTOR_VALUE, &args![setting]).u32()
            } else {
                let effect_item = e.call(WORD_AT_C, &args![effect]).u32();
                e.call(EFFECT_ITEM_ACTOR_VALUE, &args![effect_item]).u32()
            };
            if value == 0x10
                && e.call(SETTING_HAS_FLAG, &args![setting, 4u32]).bool()
                && !e.call(SETTING_HAS_FLAG, &args![setting, 2u32]).bool()
            {
                let mut damage = x87_store32(e.call(ACTIVE_EFFECT_MAGNITUDE, &args![effect]).f64());
                let duration = e.call(ACTIVE_EFFECT_DURATION, &args![effect]).f64();
                // FCOMP / TEST AH,0x41 / JNZ: on when above.
                if duration > e.mem.f64(ZERO) {
                    let duration =
                        x87_store32(e.call(ACTIVE_EFFECT_DURATION, &args![effect]).f64());
                    let part = x87_store32(e.call(DURATION_PART, &args![0.0f32, duration]).f64());
                    let four = x87_quiet_bits(e.mem.u32(FOUR));
                    let scale = x87_store32(e.call(DURATION_SCALE, &args![four, part]).f64());
                    damage = x87_store32(x87_mul(x87_load32(damage), x87_m32(scale)));
                }
                total = x87_store32(x87_add(x87_load32(total), x87_m32(damage)));
            }
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    let negated = x87_store32(-x87_load32(total));
    let adjusted = e
        .call(ADJUST_HEALTH_MODIFIER, &args![actor, negated, caster_actor])
        .f64();
    let total = x87_store32(-adjusted);
    // FCOMPP (the health against the total) / TEST AH,0x41 / JP: true when below or equal.
    x87_load32(health) <= x87_load32(total)
}

/// `008256d0` (`thiscall`): a list node is empty.
const NODE_IS_EMPTY: u32 = 0x0082_56d0;

/// Explosion helpers: `00404390` (`EffectItem::GetMagnitude`, a word stays pushed for the
/// cost), `004044d0` (`EffectItem::GetDuration`), `00639b20` (a `float` of the effect setting,
/// `ST0`), `MagicFormulas::GetMagicEffectCost` (`006490b0`, `cdecl`, five words, `ST0`),
/// `00649300` (`cdecl`, six words, `ST0`: the force).
const EFFECT_ITEM_GET_MAGNITUDE: u32 = 0x0040_4390;
const EFFECT_ITEM_GET_DURATION: u32 = 0x0040_44d0;
const SETTING_BASE_COST: u32 = 0x0063_9b20;
const GET_MAGIC_EFFECT_COST: u32 = 0x0064_90b0;
const EXPLOSION_FORCE: u32 = 0x0064_9300;
/// The phantom: `0056e4e0` / `0056d730` construct / destroy its info (0x20 bytes), the
/// `00416870` point setter, `00439e90` (`RET 8`, a sum into an out point), `004a3e00`
/// (`cdecl`: a four-float copy), `005dbe10` (the default layer), `00622290` (`RET 0xC`),
/// `bhkAabbPhantom::bhkAabbPhantom` (`0056e550`, `RET 4`) and its destructor (`00c9f530`),
/// `bhkWorldObject::Add` (`00c85810`, `RET 4`), `bhkPhantom::Remove` (`00c9f420`), the
/// proxy list's entry (`00629ab0`, `RET 4`) and its reference (`004b59f0`, `cdecl`).
const PHANTOM_INFO_CONSTRUCT: u32 = 0x0056_e4e0;
const PHANTOM_INFO_DESTRUCT: u32 = 0x0056_d730;
const POINT_SUM: u32 = 0x0043_9e90;
const VECTOR_COPY: u32 = 0x004a_3e00;
const DEFAULT_LAYER: u32 = 0x005d_be10;
const FILTER_INFO: u32 = 0x0062_2290;
const AABB_PHANTOM_CONSTRUCT: u32 = 0x0056_e550;
const AABB_PHANTOM_DESTRUCT: u32 = 0x00c9_f530;
const WORLD_OBJECT_ADD: u32 = 0x00c8_5810;
const PHANTOM_REMOVE: u32 = 0x00c9_f420;
const PROXY_ENTRY: u32 = 0x0062_9ab0;
const ENTRY_REFERENCE: u32 = 0x004b_59f0;
const CELL_HAVOK_WORLD: u32 = 0x0045_43c0;
const NODE_NEXT_WORD: u32 = 0x0072_6070;
/// `00559450` (`thiscall`): the first word of an object.
const FIRST_WORD: u32 = 0x0055_9450;

// Translated from 00817b90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00817b90 - the FS:[0] exception frame and the /GS cookie check (00ec408c) are not translated
/// `MagicCaster::DoExplosion` (Xbox PDB; `thiscall`, `RET 0x24`: a point by value, the cell,
/// the spell, the effect item, the target list, effectiveness, scale). Only for an effect
/// whose setting has flag `0x20000000` and a radius (area times the setting at `011d15e0`
/// times the scale) above `0.0`: the cost (`006490b0`) of the scaled magnitude and duration;
/// an AABB phantom of half the radius around the point (layer 0x29) is added to the cell's
/// world and each body it touches gets [`fn_00818150`]; then each actor of the list (not the
/// caster actor, with 3D, within the radius, alive by slot +0x22C, not excluded) and the
/// player (likewise) that the effect would kill ([`magic_caster_check_will_kill`]) gets
/// [`magic_caster_explode_actor`] with the cost.
#[allow(clippy::too_many_arguments)]
pub fn magic_caster_do_explosion(
    e: &mut Engine,
    this: Ptr,
    x: u32,
    y: u32,
    z: u32,
    cell: u32,
    spell: u32,
    effect: u32,
    list: u32,
    effectiveness: f32,
    scale: f32,
) {
    let setting = e.call(EFFECT_ITEM_SETTING, &args![effect]).u32();
    if !e
        .call(SETTING_HAS_FLAG, &args![setting, 0x2000_0000u32])
        .bool()
    {
        return;
    }
    let area = e.call(EFFECT_ITEM_GET_AREA, &args![effect]).i32();
    let radius = x87_store32(x87_mul(
        x87_mul(setting_load(e, AREA_FACTOR), f64::from(area)),
        x87_m32(scale.to_bits()),
    ));
    // FCOMP / TEST AH,0x41 / JNZ: on when above.
    if !(x87_load32(radius) > e.mem.f64(ZERO)) {
        return;
    }
    let is_target = u32::from(e.call(WORD_AT_C, &args![effect]).u32() == 2);
    let magnitude = e.call(EFFECT_ITEM_GET_MAGNITUDE, &args![effect]).i32();
    let magnitude = ftol(
        e,
        x87_mul(f64::from(magnitude), x87_m32(effectiveness.to_bits())),
    );
    let duration = e.call(EFFECT_ITEM_GET_DURATION, &args![effect]).i32();
    let duration = ftol(
        e,
        x87_mul(f64::from(duration), x87_m32(effectiveness.to_bits())),
    );
    let area_words = e.call(EFFECT_ITEM_GET_AREA, &args![effect]).u32();
    let setting = e.call(EFFECT_ITEM_SETTING, &args![effect]).u32();
    let base_cost = x87_store32(e.call(SETTING_BASE_COST, &args![setting]).f64());
    let cost = x87_store32(
        e.call(
            GET_MAGIC_EFFECT_COST,
            &args![base_cost, area_words, duration, magnitude, is_target],
        )
        .f64(),
    );
    let half = x87_store32(x87_mul(x87_load32(radius), e.mem.f64(HALF)));
    e.with_stack(0x100, |e, frame| {
        let at = |offset: u32| frame.addr().wrapping_add(0x100 - offset);
        let point = at(0xf0);
        put_point(e, point, [x, y, z]);
        let info = at(0x80);
        e.call(PHANTOM_INFO_CONSTRUCT, &args![info]);
        let negative = x87_store32(-x87_load32(half));
        let corner = e
            .call(POINT_SET, &args![at(0x8c), negative, negative, negative])
            .u32();
        let low = e.call(POINT_SUM, &args![point, at(0x98), corner]).u32();
        e.call(VECTOR_COPY, &args![at(0x60), low]);
        let positive = x87_quiet_bits(half);
        let corner = e
            .call(POINT_SET, &args![at(0xa4), positive, positive, positive])
            .u32();
        let high = e.call(POINT_SUM, &args![point, at(0xb0), corner]).u32();
        e.call(VECTOR_COPY, &args![at(0x50), high]);
        let layer = e.call(DEFAULT_LAYER, &[]).u32();
        let filter = e
            .call(FILTER_INFO, &args![at(0xb4), 0x29u32, layer, 0u32])
            .u32();
        let word = e.call(FIRST_WORD, &args![filter]).u32();
        e.mem.set_u32(info, word);
        let phantom = at(0xcc);
        e.call(AABB_PHANTOM_CONSTRUCT, &args![phantom, info]);
        let world = e.call(CELL_HAVOK_WORLD, &args![cell]).u32();
        e.call(WORLD_OBJECT_ADD, &args![phantom, world]);
        let bodies = fn_00818110(e, Ptr::new(phantom));
        let mut index = 0i32;
        while index < e.call(NODE_NEXT_WORD, &args![bodies]).i32() {
            let entry = e.call(PROXY_ENTRY, &args![bodies, index as u32]).u32();
            let item = e.mem.u32(entry);
            let body = e.call(ENTRY_REFERENCE, &args![item]).u32();
            if body != 0 {
                fn_00818150(e, this, body, x, y, z, effect, f32::from_bits(cost));
            }
            index = index.wrapping_add(1);
        }
        e.call(PHANTOM_REMOVE, &args![phantom]);
        let caster_actor = magic_caster_get_caster_as_actor(e, this).addr();
        let mut node = list;
        while node != 0 {
            let item = e.call(NODE_ITEM, &args![node]).u32();
            if e.mem.u32(item) == 0 {
                break;
            }
            let item = e.call(NODE_ITEM, &args![node]).u32();
            let reference = e.mem.u32(item);
            let actor = if e.vcall(reference, 0x100, &[]).bool() {
                reference
            } else {
                0
            };
            if actor != 0 && actor != caster_actor && e.vcall(actor, 0x1d0, &[]).u32() != 0 {
                explode_if_killed(e, this, actor, point, radius, spell, x, y, z, effect, cost);
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
        let player = e.mem.u32(PLAYER);
        if player != 0 && player != caster_actor && e.vcall(player, 0x1d0, &[]).u32() != 0 {
            explode_if_killed(e, this, player, point, radius, spell, x, y, z, effect, cost);
        }
        e.call(AABB_PHANTOM_DESTRUCT, &args![phantom]);
        e.call(PHANTOM_INFO_DESTRUCT, &args![info]);
    });
}

/// [`magic_caster_do_explosion`]'s test and explosion of one actor.
#[allow(clippy::too_many_arguments)]
fn explode_if_killed(
    e: &mut Engine,
    this: Ptr,
    actor: u32,
    point: u32,
    radius: u32,
    spell: u32,
    x: u32,
    y: u32,
    z: u32,
    effect: u32,
    cost: u32,
) {
    let distance = x87_store32(e.call(DISTANCE_TO_POINT, &args![actor, point]).f64());
    // FCOMPP (the radius against the distance) / TEST AH,1 / JNZ: on when not below.
    if !(x87_load32(radius) >= x87_load32(distance)) {
        return;
    }
    if e.vcall(actor, 0x22c, &args![0u32]).bool() {
        return;
    }
    if e.call(ACTOR_EXCLUDED, &args![actor]).bool() {
        return;
    }
    let target = if actor != 0 {
        actor.wrapping_add(0x94)
    } else {
        0
    };
    if magic_caster_check_will_kill(e, this, target, spell) {
        magic_caster_explode_actor(e, this, actor, x, y, z, effect, f32::from_bits(cost));
    }
}

// Translated from 00818110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): the list of the bodies the phantom's character
/// proxy (`004ae750`) touches (`0045c650`), or null without a proxy.
pub fn fn_00818110(e: &mut Engine, this: Ptr) -> u32 {
    let proxy = e.call(CHARACTER_PROXY, &args![this]).u32();
    if proxy != 0 {
        e.call(PROXY_LIST, &args![proxy]).u32()
    } else {
        0
    }
}

/// `006310c0` (`thiscall`): a body's position; `00458620` (`cdecl`): a point copy;
/// `NiPoint3::UnitizeGetLength` (`00457910`, `ST0`); `004a51b0` (`RET 4`): a component's
/// address; `0045bb20` (`RET 8`): a scaled point into an out point; `004b4ea0` (`ST0`): the
/// body's mass; `005dbf20` (`RET 4`) and `00627920` (`RET 4`): the impulse scaled by it;
/// `hkpEntity::activate` (`00c9c1d0`); the settings `011d1884` (the force limit) and
/// `011d1b8c` (the force scale).
const BODY_POSITION: u32 = 0x0063_10c0;
const COPY_POINT: u32 = 0x0045_8620;
const UNITIZE_GET_LENGTH: u32 = 0x0045_7910;
const POINT_COMPONENT: u32 = 0x004a_51b0;
const POINT_SCALED: u32 = 0x0045_bb20;
const BODY_MASS: u32 = 0x004b_4ea0;
const MASS_VECTOR: u32 = 0x005d_bf20;
const VECTOR_MULTIPLY: u32 = 0x0062_7920;
const FORCE_LIMIT_SETTING: u32 = 0x011d_1884;
const FORCE_SCALE_SETTING: u32 = 0x011d_1b8c;
const POINT_DIFFERENCE: u32 = 0x0043_9ef0;
const POINT_LENGTH: u32 = 0x0045_7990;
const SELF_ITEM: u32 = 0x0068_15c0;

/// The force of an explosion (`00649300` with the cost and the area truncated) on
/// something at `distance`, the three other words given; lowered to the setting at
/// `011d1884`.
fn explosion_force(e: &mut Engine, effect: u32, cost: f32, distance: u32, rest: [u32; 3]) -> u32 {
    let area = e.call(EFFECT_ITEM_GET_AREA, &args![effect]).i32();
    let factor = setting_load(e, AREA_FACTOR);
    let area = ftol(e, x87_mul(factor, f64::from(area)));
    let cost = ftol(e, x87_load32(cost.to_bits()));
    x87_store32(
        e.call(
            EXPLOSION_FORCE,
            &args![cost, area, distance, rest[0], rest[1], rest[2]],
        )
        .f64(),
    )
}

/// Lowers the force to the setting at `011d1884` when that is below it.
fn limit_force(e: &mut Engine, force: u32) -> u32 {
    let limit = setting_load(e, FORCE_LIMIT_SETTING);
    // FCOMPP (the limit against the force) / TEST AH,5 / JP: lowered when the limit is below.
    if limit < x87_load32(force) {
        let value = e.call(SETTING_VALUE, &args![FORCE_LIMIT_SETTING]).u32();
        x87_quiet_bits(e.mem.u32(value))
    } else {
        force
    }
}

// Translated from 00818150 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00818150 - the /GS cookie check (00ec408c) is not translated
/// Unnamed in the engine map (`thiscall`, `RET 0x18`: a rigid body, a point by value, the
/// effect item, the cost): pushes the body away from the point: the force (`00649300` with
/// the cost, the area, the distance, 0, 0, `0.0`), when above `0.0`, limited (`011d1884`)
/// and scaled (`011d1b8c`), along the unit direction lifted by `0.5`, times the body's mass,
/// applied as a linear impulse after activating the body.
#[allow(clippy::too_many_arguments)]
pub fn fn_00818150(
    e: &mut Engine,
    _this: Ptr,
    body: u32,
    x: u32,
    y: u32,
    z: u32,
    effect: u32,
    cost: f32,
) {
    e.with_stack(0x80, |e, frame| {
        let at = |offset: u32| frame.addr().wrapping_add(0x80 - offset);
        let point = at(0x7c);
        put_point(e, point, [x, y, z]);
        let position = at(0x14);
        let direction = at(0x20);
        e.call(SELF_ITEM, &args![position]);
        let source = e.call(BODY_POSITION, &args![body]).u32();
        e.call(COPY_POINT, &args![position, source]);
        e.call(POINT_DIFFERENCE, &args![position, direction, point]);
        let distance = x87_store32(e.call(UNITIZE_GET_LENGTH, &args![direction]).f64());
        let mut force = explosion_force(e, effect, cost, x87_quiet_bits(distance), [0, 0, 0]);
        // FCOMP / TEST AH,0x41 / JNZ: on when above.
        if !(x87_load32(force) > e.mem.f64(ZERO)) {
            return;
        }
        force = limit_force(e, force);
        let scale_value = e.call(SETTING_VALUE, &args![FORCE_SCALE_SETTING]).u32();
        force = x87_store32(x87_mul(x87_load32(force), x87_m32(e.mem.u32(scale_value))));
        let component = e.call(POINT_COMPONENT, &args![direction, 2u32]).u32();
        let lifted = x87_store32(x87_add(x87_load32(e.mem.u32(component)), e.mem.f64(HALF)));
        e.mem.set_u32(component, lifted);
        let scaled = at(0x44);
        e.call(
            POINT_SCALED,
            &args![direction, scaled, x87_quiet_bits(force)],
        );
        let impulse = at(0x60);
        e.call(SELF_ITEM, &args![impulse]);
        e.call(VECTOR_COPY, &args![impulse, scaled]);
        let mass = x87_store32(e.call(BODY_MASS, &args![body]).f64());
        let factor = at(0x70);
        e.call(MASS_VECTOR, &args![factor, mass]);
        e.call(VECTOR_MULTIPLY, &args![impulse, factor]);
        e.call(HKP_ENTITY_ACTIVATE, &args![body]);
        hkp_rigid_body_apply_linear_impulse(e, Ptr::new(body), impulse);
    });
}

/// `Actor::GetFatiguePercentage` (`00893530`, `ST0`), `ActorValueOwner::
/// GetClampedActorValue` (`0066ef20`, `RET 4`), the push threshold setting (`011d180c`), the
/// processes' push (slot +0x418), `00476a80` (a transform's constructor), `NiTransform::
/// InvertNonUniform` (`004b4880`, `RET 4`), `00524c40` (`RET 8`: a transformed point),
/// `005b6f70` (`cdecl`: a `bool`), `0055d520` (the actor's name), `00703c00` (`cdecl`, a
/// console print) and `"An explosion of %.1f mag..."` (`0107ad0c`).
const ACTOR_FATIGUE: u32 = 0x0089_3530;
const CLAMPED_ACTOR_VALUE: u32 = 0x0066_ef20;
const PUSH_THRESHOLD_SETTING: u32 = 0x011d_180c;
const TRANSFORM_CONSTRUCT: u32 = 0x0047_6a80;
const INVERT_TRANSFORM: u32 = 0x004b_4880;
const TRANSFORM_POINT: u32 = 0x0052_4c40;
const NODE_TRANSFORM: u32 = 0x0046_1130;
const CONSOLE_ENABLED: u32 = 0x005b_6f70;
const ACTOR_NAME: u32 = 0x0055_d520;
const CONSOLE_PRINT: u32 = 0x0070_3c00;
const TEXT_EXPLOSION: u32 = 0x0107_ad0c;
const EXPLOSION_OFFSET: u32 = 0x011a_9478;

// Translated from 00818310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster::ExplodeActor` (Xbox PDB; `thiscall`, `RET 0x18`: the actor, a point by
/// value, the effect item, the cost). The force (`00649300` with the cost, the area, the
/// distance from the point to the actor's position raised by 0.75 of its height, actor
/// values 0xA and 0xB, the fatigue), limited (`011d1884`): beyond the setting at `011d180c`
/// the actor's process pushes it (slot +0x418); otherwise, above `0.0` and with an actor
/// caster, the actor's slot +0x478 knocks it down (the caster, `2.0 * force`, `0.0`, and the
/// default point and the explosion offset, both moved into the caster's node space when it
/// has a node). A force above `0.0` is printed to the console when it is enabled.
#[allow(clippy::too_many_arguments)]
pub fn magic_caster_explode_actor(
    e: &mut Engine,
    this: Ptr,
    actor: u32,
    x: u32,
    y: u32,
    z: u32,
    effect: u32,
    cost: f32,
) {
    e.with_stack(0xc0, |e, frame| {
        let at = |offset: u32| frame.addr().wrapping_add(0xc0 - offset);
        let point = at(0xb8);
        put_point(e, point, [x, y, z]);
        let top = e.vcall(actor, 0x1dc, &args![at(0x84)]).u32();
        let bottom = e.vcall(actor, 0x1d8, &args![at(0x90)]).u32();
        let height = x87_store32(x87_sub(
            x87_load32(e.mem.u32(top.wrapping_add(8))),
            x87_m32(e.mem.u32(bottom.wrapping_add(8))),
        ));
        let position = at(0x24);
        let source = e.vcall(actor, 0x1f4, &[]).u32();
        let words = point_words(e, source);
        put_point(e, position, words);
        let raised = x87_store32(x87_add(
            x87_mul(x87_load32(height), e.mem.f64(THREE_QUARTERS)),
            x87_m32(e.mem.u32(position.wrapping_add(8))),
        ));
        e.mem.set_u32(position.wrapping_add(8), raised);
        let difference = at(0x0c);
        e.call(POINT_DIFFERENCE, &args![position, difference, point]);
        let distance = x87_store32(e.call(POINT_LENGTH, &args![difference]).f64());
        let fatigue = x87_store32(e.call(ACTOR_FATIGUE, &args![actor]).f64());
        let value_b = e
            .call(
                CLAMPED_ACTOR_VALUE,
                &args![actor.wrapping_add(0xa4), 0xbu32],
            )
            .u32();
        let value_a = e
            .call(
                CLAMPED_ACTOR_VALUE,
                &args![actor.wrapping_add(0xa4), 0xau32],
            )
            .u32();
        let mut force = explosion_force(
            e,
            effect,
            cost,
            x87_quiet_bits(distance),
            [value_a, value_b, fatigue],
        );
        force = limit_force(e, force);
        let threshold = setting_load(e, PUSH_THRESHOLD_SETTING);
        // FCOMPP (the threshold against the force) / TEST AH,5 / JP: pushed when the
        // threshold is below.
        if threshold < x87_load32(force) {
            let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
            e.vcall(
                process,
                0x418,
                &args![actor, x, y, z, x87_quiet_bits(force)],
            );
        } else if x87_load32(force) > e.mem.f64(ZERO) {
            let caster_actor = magic_caster_get_caster_as_actor(e, this).addr();
            if caster_actor != 0 {
                let offset = at(0x40);
                let origin = at(0x34);
                let words = point_words(e, EXPLOSION_OFFSET);
                put_point(e, offset, words);
                let words = point_words(e, DEFAULT_POINT);
                put_point(e, origin, words);
                let node = e.vcall(caster_actor, 0x1d0, &[]).u32();
                if node != 0 {
                    let transform = at(0x78);
                    e.call(TRANSFORM_CONSTRUCT, &args![transform]);
                    let own = e.call(NODE_TRANSFORM, &args![node]).u32();
                    e.call(INVERT_TRANSFORM, &args![own, transform]);
                    let moved = e
                        .call(TRANSFORM_POINT, &args![transform, at(0x9c), point])
                        .u32();
                    let words = point_words(e, moved);
                    put_point(e, offset, words);
                    let moved = e
                        .call(TRANSFORM_POINT, &args![transform, at(0xa8), position])
                        .u32();
                    let words = point_words(e, moved);
                    put_point(e, origin, words);
                }
                let push = x87_store32(x87_mul(x87_load32(force), e.mem.f64(TWO)));
                e.vcall(
                    actor,
                    0x478,
                    &args![caster_actor, push, 0.0f32, origin, offset],
                );
            }
        }
        if x87_load32(force) > e.mem.f64(ZERO) && e.call(CONSOLE_ENABLED, &[]).bool() {
            let name = e.call(ACTOR_NAME, &args![actor]).u32();
            let value = x87_load32(force);
            e.call(CONSOLE_PRINT, &args![TEXT_EXPLOSION, value, name]);
        }
    });
}

/// `0x0063c8a0` (`RET 4`): adds a point; `CombatUtilities::GetAngleToProjectedTarget`
/// (`009a86e0`, `cdecl`, six words) and its `_ov2` (`009a87f0`, `cdecl`, eight words);
/// `MobileObject::GetLooking` (`00931d70`, `ST0`); `00981920` (a combat controller's
/// word); `MagicProjectile::Launch` (`0081e040`, `cdecl`, twelve words); the `MagicLight`
/// deleting destructor (`0080fcc0`).
const POINT_ADD: u32 = 0x0063_c8a0;
const ANGLE_TO_TARGET: u32 = 0x009a_86e0;
const ANGLE_TO_TARGET_OV2: u32 = 0x009a_87f0;
const GET_LOOKING: u32 = 0x0093_1d70;
const COMBAT_SEGMENT: u32 = 0x0098_1920;
const PROJECTILE_LAUNCH: u32 = 0x0081_e040;
const PROJECTILE_SPEED: u32 = 0x009b_0bd0;
const SPEED_SETTING: u32 = 0x011d_16f8;

// Translated from 00818610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster::FireProjectile` (Xbox PDB): with a current spell, a caster reference and a
/// target-range costliest effect, launches the projectile (`MagicProjectile::Launch`) from
/// the caster's position (an actor's raised by 0.7 of its height through `0063c8a0`, aimed by
/// its slot +0x2BC and its looking angle; others by `009a86e0` toward the desired target at
/// the effect's projectile speed, raised by half their height), or from the magic node
/// (slot +0x30); an actor caster other than the player's with a desired target aims with
/// `009a87f0` (its combat controller's segment, or 1). The light (+4) goes with the
/// projectile; any light left is deleted.
pub fn magic_caster_fire_projectile(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.vcall(base, 0x34, &[]).u32() == 0 {
        return;
    }
    let reference = e.vcall(base, 0x2c, &[]).u32();
    let actor = magic_caster_get_caster_as_actor(e, this).addr();
    let target = if e.vcall(base, 0x44, &[]).u32() != 0 {
        let desired = e.vcall(base, 0x44, &[]).u32();
        e.vcall(desired, 4, &[]).u32()
    } else {
        0
    };
    let spell = e.vcall(base, 0x34, &[]).u32();
    let effect = e
        .call(
            GET_COSTLIEST_EFFECT,
            &args![spell.wrapping_add(0xc), 2u32, 0u32],
        )
        .u32();
    let spell = e.vcall(base, 0x34, &[]).u32();
    let setting = e.call(GET_AV_EFFECT, &args![spell, 0u32]).u32();
    if effect == 0 || reference == 0 {
        return;
    }
    let launch = true;
    let effect_present = effect != 0;
    e.with_stack(0xd0, |e, frame| {
        let at = |offset: u32| frame.addr().wrapping_add(0xd0 - offset);
        let position = at(0x40);
        let offset = at(0x34);
        let angles = at(0x28);
        e.call(SELF_ITEM, &args![position]);
        let words = point_words(e, DEFAULT_POINT);
        put_point(e, offset, words);
        put_point(e, angles, words);
        if actor != 0 {
            let z = x87_store32(e.vcall(actor, 0x2bc, &args![0u32]).f64());
            e.mem.set_u32(angles.wrapping_add(8), z);
            let x = x87_store32(e.call(GET_LOOKING, &args![actor]).f64());
            e.mem.set_u32(angles, x);
            let source = e.vcall(reference, 0x1f4, &[]).u32();
            let words = point_words(e, source);
            put_point(e, position, words);
            let top = e.vcall(reference, 0x1dc, &args![at(0x64)]).u32();
            let bottom = e.vcall(reference, 0x1d8, &args![at(0x70)]).u32();
            let height = x87_store32(x87_sub(
                x87_load32(e.mem.u32(top.wrapping_add(8))),
                x87_m32(e.mem.u32(bottom.wrapping_add(8))),
            ));
            let lift = x87_store32(x87_mul(x87_load32(height), e.mem.f64(SEVEN_TENTHS)));
            e.mem.set_u32(offset.wrapping_add(8), lift);
            e.call(POINT_ADD, &args![position, offset]);
        } else {
            let speed = e.call(PROJECTILE_SPEED, &args![setting]).f64();
            let speed = x87_store32(x87_mul(setting_load(e, SPEED_SETTING), speed));
            let aim = e
                .call(
                    ANGLE_TO_TARGET,
                    &args![at(0x7c), reference, target, speed, 0.0f32, 2u32],
                )
                .u32();
            let own = e.call(ANGLES, &args![reference]).u32();
            let summed = e.call(POINT_SUM, &args![own, at(0x88), aim]).u32();
            let words = point_words(e, summed);
            put_point(e, angles, words);
            let source = e.vcall(reference, 0x1f4, &[]).u32();
            let words = point_words(e, source);
            put_point(e, position, words);
            let top = e.vcall(reference, 0x1dc, &args![at(0x94)]).u32();
            let bottom = e.vcall(reference, 0x1d8, &args![at(0xa0)]).u32();
            let raised = x87_store32(x87_add(
                x87_mul(
                    x87_sub(
                        x87_load32(e.mem.u32(top.wrapping_add(8))),
                        x87_m32(e.mem.u32(bottom.wrapping_add(8))),
                    ),
                    e.mem.f64(HALF),
                ),
                x87_m32(e.mem.u32(position.wrapping_add(8))),
            ));
            e.mem.set_u32(position.wrapping_add(8), raised);
        }
        let node = e.vcall(base, 0x30, &[]).u32();
        if node != 0 {
            let source = e.call(MAGIC_NODE_POSITION, &args![node]).u32();
            let words = point_words(e, source);
            put_point(e, position, words);
        }
        let player = e.mem.u32(PLAYER);
        let player_caster = if player != 0 {
            player.wrapping_add(0x88)
        } else {
            0
        };
        if base != player_caster && actor != 0 && target != 0 {
            let speed = if setting != 0 {
                x87_store32(e.call(PROJECTILE_SPEED, &args![setting]).f64())
            } else {
                1.0f32.to_bits()
            };
            let speed = x87_store32(x87_mul(setting_load(e, SPEED_SETTING), x87_m32(speed)));
            let combat = e.vcall(actor, 0x428, &[]).u32();
            let segment = if combat != 0 {
                e.call(COMBAT_SEGMENT, &args![combat]).u32()
            } else {
                1
            };
            let [px, py, pz] = point_words(e, position);
            let aim = e
                .call(
                    ANGLE_TO_TARGET_OV2,
                    &args![at(0xac), px, py, pz, target, speed, 0.0f32, segment],
                )
                .u32();
            let words = point_words(e, aim);
            put_point(e, angles, words);
        }
        if effect_present && launch {
            let [ax, ay, az] = point_words(e, angles);
            let [px, py, pz] = point_words(e, position);
            let spell = e.vcall(base, 0x34, &[]).u32();
            let light = e.mem.u32(base.wrapping_add(4));
            e.call(
                PROJECTILE_LAUNCH,
                &args![base, 0u32, light, spell, effect, setting, px, py, pz, ax, ay, az],
            );
            e.mem.set_u32(base.wrapping_add(4), 0);
        }
    });
    let light = e.mem.u32(base.wrapping_add(4));
    if light != 0 {
        let object = e.mem.u32(base.wrapping_add(4));
        if object != 0 {
            e.call(MAGIC_LIGHT_DELETE, &args![object, 1u32]);
        }
        e.mem.set_u32(base.wrapping_add(4), 0);
    }
}

// Translated from 00818b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: the elapsed time): updates the light
/// (+4, `0081c320`); without a current spell and once the light has faded
/// ([`fn_00818ba0`] false), deletes it and clears +4.
pub fn fn_00818b10(e: &mut Engine, this: Ptr, time: f32) {
    let base = this.addr();
    let light = e.mem.u32(base.wrapping_add(4));
    if light == 0 {
        return;
    }
    e.call(
        MAGIC_LIGHT_UPDATE,
        &args![light, x87_quiet_bits(time.to_bits())],
    );
    if e.vcall(base, 0x34, &[]).u32() != 0 {
        return;
    }
    let light = e.mem.u32(base.wrapping_add(4));
    if fn_00818ba0(e, Ptr::new(light)) {
        return;
    }
    let light = e.mem.u32(base.wrapping_add(4));
    let object = light;
    if object != 0 {
        e.call(MAGIC_LIGHT_DELETE, &args![object, 1u32]);
    }
    e.mem.set_u32(base.wrapping_add(4), 0);
}

// Translated from 00818ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, a `MagicLight`): true when its `float` at +0x14 is
/// above `0.0` (the `double` at `01012060`; not when unordered).
pub fn fn_00818ba0(e: &mut Engine, this: Ptr) -> bool {
    x87_load32(e.mem.u32(this.addr().wrapping_add(0x14))) > e.mem.f64(ZERO)
}

// Translated from 00818c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster::GetMagicCasterByNumericID` (Xbox PDB; `cdecl`): the form of the id
/// (`004839c0`); when its slot +0xF0 answers true, its slot +0x198 (the caster); null
/// otherwise.
pub fn magic_caster_get_magic_caster_by_numeric_id(e: &mut Engine, id: u32) -> u32 {
    let form = e.call(LOOKUP_FORM, &args![id]).u32();
    if form != 0 && e.vcall(form, 0xf0, &[]).bool() {
        e.vcall(form, 0x198, &[]).u32()
    } else {
        0
    }
}

// Translated from 00818cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster::CheckCast` (Xbox PDB; `thiscall`, `RET 0x10`: the spell and a last word it
/// never reads, a `float` and a word to fill): stores `1.0` and 0 where given; true.
pub fn magic_caster_check_cast(
    e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    value: u32,
    word: u32,
    _unused_4: u32,
) -> bool {
    if value != 0 {
        e.mem.set_u32(value, 1.0f32.to_bits());
    }
    if word != 0 {
        e.mem.set_u32(word, 0);
    }
    true
}

// Translated from 00818ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x18`: spell, range, a point by value, the
/// target list): collects the targets of the spell's largest area of that range (times the
/// setting at `011d15e0`) around the point: each high actor and the player within it (with
/// 3D, a magic target, not the caster actor, not excluded) that the explosion's line of
/// sight reaches ([`magic_caster_run_explosion_los_pick`]; not asked for a spell with flag
/// 0x10) is added (`005ae3d0`, as a `MobileObject` cast to `Actor`). (An inner walk over a
/// list the exe starts empty is translated as the exe has it.)
#[allow(clippy::too_many_arguments)]
pub fn fn_00818ce0(
    e: &mut Engine,
    this: Ptr,
    spell: u32,
    range: u32,
    x: u32,
    y: u32,
    z: u32,
    list: u32,
) {
    if list == 0 {
        return;
    }
    let mut largest = 0i32;
    let mut entry = if spell != 0 {
        spell.wrapping_add(0xc)
    } else {
        0
    };
    while entry != 0 && !e.call(NODE_IS_EMPTY, &args![entry.wrapping_add(4)]).bool() {
        let item_slot = e.call(NODE_ITEM, &args![entry.wrapping_add(4)]).u32();
        let item = e.mem.u32(item_slot);
        if item != 0
            && e.call(WORD_AT_C, &args![item]).u32() == range
            && e.call(EFFECT_ITEM_GET_AREA, &args![item]).i32() > largest
        {
            largest = e.call(EFFECT_ITEM_GET_AREA, &args![item]).i32();
        }
        let next = e.call(NODE_NEXT, &args![entry.wrapping_add(4)]).u32();
        entry = if next != 0 { next.wrapping_sub(4) } else { 0 };
    }
    if largest == 0 {
        return;
    }
    let spell_item = dynamic_cast(e, spell, TYPE_MAGIC_ITEM, TYPE_SPELL_ITEM);
    let no_sight = spell_item != 0 && e.call(SPELL_HAS_FLAG, &args![spell_item, 0x10u32]).bool();
    let radius = x87_store32(x87_mul(setting_load(e, AREA_FACTOR), f64::from(largest)));
    let caster_actor = magic_caster_get_caster_as_actor(e, this).addr();
    let actors = e.call(HIGH_ACTORS, &args![PROCESS_LISTS]).u32();
    e.with_stack(0x10, |e, locals| {
        let point = locals.addr();
        let slot = point.wrapping_add(0xc);
        put_point(e, point, [x, y, z]);
        let mut index = 0u32;
        while index < e.call(ACTOR_COUNT, &args![actors, 0u32]).u32() {
            let mut item = e.call(ACTOR_AT, &args![actors, index]).u32();
            let mut node = 0u32;
            while item != 0 {
                let target = e.vcall(item, 0x19c, &[]).u32();
                let actor = dynamic_cast(e, item, TYPE_MOBILE_OBJECT, TYPE_ACTOR);
                let is_caster = caster_actor != 0 && actor == caster_actor;
                if target != 0 && !is_caster && e.vcall(item, 0x1d0, &[]).u32() != 0 {
                    let distance =
                        x87_store32(e.call(DISTANCE_TO_POINT, &args![item, point]).f64());
                    // FCOMPP (the radius against the distance) / TEST AH,1 / JNZ: on when
                    // not below.
                    if x87_load32(radius) >= x87_load32(distance)
                        && !(actor != 0 && e.call(ACTOR_EXCLUDED, &args![actor]).bool())
                        && (no_sight
                            || magic_caster_run_explosion_los_pick(e, this, x, y, z, actor))
                    {
                        e.mem.set_u32(slot, actor);
                        e.call(LIST_ADD, &args![list, slot]);
                    }
                }
                item = next_live_character(e, &mut node);
            }
            index = index.wrapping_add(1);
        }
        let player = e.mem.u32(PLAYER);
        if player == caster_actor || e.vcall(player, 0x1d0, &[]).u32() == 0 {
            return;
        }
        let player = e.mem.u32(PLAYER);
        let distance = x87_store32(e.call(DISTANCE_TO_POINT, &args![player, point]).f64());
        if !(x87_load32(radius) >= x87_load32(distance)) {
            return;
        }
        let player = e.mem.u32(PLAYER);
        if e.call(ACTOR_EXCLUDED, &args![player]).bool() {
            return;
        }
        let player = e.mem.u32(PLAYER);
        if no_sight || magic_caster_run_explosion_los_pick(e, this, x, y, z, player) {
            let player = e.mem.u32(PLAYER);
            e.mem.set_u32(slot, player);
            e.call(LIST_ADD, &args![list, slot]);
        }
    });
}

/// `bhkCollisionFilter::SetMultiLayerCollision` (`00c82870`, `cdecl`, three words), the pick
/// record (`004a3c20` constructs it, `004a3f70` filter, `004a3da0` from, `004a3eb0` to, `RET 4`
/// each), the filter word (`BGSChangeFlags::IsForcedChange` `008c71b0` as its constructor,
/// `004a39f0` layer, `0059ce80` group), the controller's group (`00819250`) and
/// `Actor::GetHeight` (`008853a0`).
const SET_MULTI_LAYER_COLLISION: u32 = 0x00c8_2870;
const PICK_CONSTRUCT: u32 = 0x004a_3c20;
const PICK_SET_FILTER: u32 = 0x004a_3f70;
const PICK_SET_FROM: u32 = 0x004a_3da0;
const PICK_SET_TO: u32 = 0x004a_3eb0;
const FILTER_CONSTRUCT: u32 = 0x008c_71b0;
const FILTER_SET_LAYER: u32 = 0x004a_39f0;
const FILTER_SET_GROUP: u32 = 0x0059_ce80;
const CONTROLLER_GROUP: u32 = 0x0081_9250;
const GET_CHAR_CONTROLLER: u32 = 0x0093_06d0;
const GET_HEIGHT: u32 = 0x0088_53a0;
const PARENT_CELL: u32 = 0x008d_6f30;

// Translated from 008190d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008190d0 - the /GS cookie check (00ec408c) is not translated
/// `MagicCaster::RunExplosionLOSPick` (Xbox PDB; `thiscall`, `RET 0x10`: a point by value,
/// the actor): false without an actor or its cell's world. Sets layer 0x27's collision
/// (`00c82870(0x27, 0x26206, 0)`), and picks from the point to the actor's position raised
/// by half its height with a filter of layer 0x27 and the actor's controller group (or the
/// default); true when the world's pick (slot +0xC8) hits nothing.
#[allow(clippy::too_many_arguments)]
pub fn magic_caster_run_explosion_los_pick(
    e: &mut Engine,
    _this: Ptr,
    x: u32,
    y: u32,
    z: u32,
    actor: u32,
) -> bool {
    if actor == 0 {
        return false;
    }
    let world = if e.call(PARENT_CELL, &args![actor]).u32() != 0 {
        let cell = e.call(PARENT_CELL, &args![actor]).u32();
        e.call(CELL_HAVOK_WORLD, &args![cell]).u32()
    } else {
        0
    };
    if world == 0 {
        return false;
    }
    e.call(
        SET_MULTI_LAYER_COLLISION,
        &args![0x27u32, 0x0002_6206u32, 0u32],
    );
    e.with_stack(0x100, |e, frame| {
        let at = |offset: u32| frame.addr().wrapping_add(0x100 - offset);
        let point = at(0xe0);
        put_point(e, point, [x, y, z]);
        let target = at(0x20);
        let source = e.vcall(actor, 0x1f4, &[]).u32();
        let words = point_words(e, source);
        put_point(e, target, words);
        let height = e.call(GET_HEIGHT, &args![actor]).f64();
        let raised = x87_store32(x87_add(
            x87_mul(height, e.mem.f64(HALF)),
            x87_m32(e.mem.u32(target.wrapping_add(8))),
        ));
        e.mem.set_u32(target.wrapping_add(8), raised);
        let pick = at(0xd0);
        let filter = at(0xd4);
        e.call(PICK_CONSTRUCT, &args![pick]);
        e.call(FILTER_CONSTRUCT, &args![filter, 0u32]);
        e.call(FILTER_SET_LAYER, &args![filter, 0x27u32]);
        let group = if e.call(GET_CHAR_CONTROLLER, &args![actor]).u32() != 0 {
            let controller = e.call(GET_CHAR_CONTROLLER, &args![actor]).u32();
            e.call(CONTROLLER_GROUP, &args![controller]).u32()
        } else {
            e.call(DEFAULT_LAYER, &[]).u32()
        };
        e.call(FILTER_SET_GROUP, &args![filter, group]);
        let word = e.mem.u32(filter);
        e.call(PICK_SET_FILTER, &args![pick, word]);
        e.call(PICK_SET_FROM, &args![pick, point]);
        e.call(PICK_SET_TO, &args![pick, target]);
        !e.vcall(world, 0xc8, &args![pick]).bool()
    })
}

#[cfg(test)]
mod n047_tests {
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

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for &addr in addresses {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn slot(family: u32, slot: u32) -> u32 {
        0x0340_0000 + family * 0x1_0000 + slot
    }

    fn object(e: &mut Engine, family: u32, size: u32) -> u32 {
        let vtable = e.mem.alloc(0x500);
        for i in 0..0x140 {
            e.mem.set_u32(vtable + 4 * i, slot(family, 4 * i));
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    /// Float settings answer the low byte of their address; integer ones too.
    fn settings(e: &mut Engine) {
        e.register(SETTING_VALUE, |e, a| {
            let p = e.mem.alloc(4);
            e.mem.set_f32(p, (a[0] & 0xff) as f32);
            p.into_ret()
        });
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_1000,
            0x0101_2000,
            0x0101_6000,
            0x0101_d000,
            0x0101_e000,
            0x0104_e000,
            0x011a_9000,
            0x011d_e000,
            0x011f_4000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.set_f64(HALF, 0.5);
        e.mem.set_f64(TWO, 2.0);
        e.mem.set_f64(ZERO, 0.0);
        e.mem.set_f64(THREE_QUARTERS, 0.75);
        e.mem.set_f64(SEVEN_TENTHS, 0.7);
        e.mem.set_f32(FLOAT_MAX, f32::MAX);
        e.mem.set_f32(FOUR, 4.0);
        e.mem.set_f32(MESSAGE_TIME, 2.0);
        e.mem.set_f32(RELEASE_FADE, 0.75);
        e.mem.set_u32(PLAYER, 0x0bad_0001);
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            (value as i32).into_ret()
        });
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn small_ones() {
        let mut e = engine();
        let caster = object(&mut e, 0, 0x10);
        assert_eq!(e.call(0x0081_53a0, &args![caster, 1u32, 2u32]).f64(), 1.0);
        // StartCast: no current spell, nothing.
        e.register(slot(0, 0x34), |_, _| 0u32.into_ret());
        e.call(0x0081_5380, &args![caster]);
        assert_eq!(calls(&e, slot(0, 0x34)).len(), 1);
        let out = e.mem.alloc(8);
        assert!(e
            .call(0x0081_8cb0, &args![caster, 0u32, out, out + 4, 0u32])
            .bool());
        assert_eq!(e.mem.f32(out), 1.0);
        assert!(e
            .call(0x0081_8cb0, &args![caster, 0u32, 0u32, 0u32, 0u32])
            .bool());
        let light = e.mem.alloc(0x20);
        e.mem.set_f32(light + 0x14, 0.5);
        assert!(e.call(0x0081_8ba0, &args![light]).bool());
        e.mem.set_f32(light + 0x14, 0.0);
        assert!(!e.call(0x0081_8ba0, &args![light]).bool());
        e.mem.set_u32(light + 0x14, 0x7fc0_0000);
        assert!(!e.call(0x0081_8ba0, &args![light]).bool());
        // The phantom's bodies.
        e.register(CHARACTER_PROXY, |_, a| a[0].into_ret());
        e.register(PROXY_LIST, |_, a| (a[0] + 1).into_ret());
        assert_eq!(e.call(0x0081_8110, &args![0x4000u32]).u32(), 0x4001);
        e.register(CHARACTER_PROXY, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x0081_8110, &args![0x4000u32]).u32(), 0);
        // A caster by id.
        let form = object(&mut e, 1, 0x10);
        e.register_double(LOOKUP_FORM, move |_, _| form.into_ret());
        e.register(slot(1, 0xf0), |_, _| 1u32.into_ret());
        e.register(slot(1, 0x198), |_, _| 0x7788u32.into_ret());
        assert_eq!(e.call(0x0081_8c60, &args![0x14u32]).u32(), 0x7788);
        e.register(slot(1, 0xf0), |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x0081_8c60, &args![0x14u32]).u32(), 0);
    }

    #[test]
    fn light_update_deletes_a_faded_light_without_a_spell() {
        let mut e = engine();
        let caster = object(&mut e, 0, 0x10);
        let light = e.mem.alloc(0x20);
        e.mem.set_u32(caster + 4, light);
        e.register(slot(0, 0x34), |_, _| 0u32.into_ret());
        quiet(&mut e, &[MAGIC_LIGHT_UPDATE, MAGIC_LIGHT_DELETE]);
        e.mem.set_f32(light + 0x14, 1.0);
        e.call(0x0081_8b10, &args![caster, 0.5f32]);
        assert_eq!(
            calls(&e, MAGIC_LIGHT_UPDATE),
            vec![vec![light, 0.5f32.to_bits()]]
        );
        assert_eq!(e.mem.u32(caster + 4), light);
        e.mem.set_f32(light + 0x14, 0.0);
        e.call(0x0081_8b10, &args![caster, 0.5f32]);
        assert_eq!(calls(&e, MAGIC_LIGHT_DELETE), vec![vec![light, 1]]);
        assert_eq!(e.mem.u32(caster + 4), 0);
        e.call(0x0081_8b10, &args![caster, 0.5f32]);
        assert_eq!(calls(&e, MAGIC_LIGHT_UPDATE).len(), 2);
    }

    /// A spell object of `kind` whose +0x18 part answers `kind` too.
    fn spell(e: &mut Engine, family: u32, kind: u32) -> u32 {
        let spell = object(e, family, 0x40);
        let part_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(part_vtable + 0x18, slot(family + 1, 0x18));
        e.mem.set_u32(spell + 0x18, part_vtable);
        e.register_double(slot(family, 0x18), move |_, _| kind.into_ret());
        e.register_double(slot(family + 1, 0x18), move |_, _| kind.into_ret());
        spell
    }

    #[test]
    fn cast_spell_dispatches_by_type() {
        let mut e = engine();
        let caster = object(&mut e, 0, 0x10);
        e.register(SPELL_DISABLED, |_, _| 0u32.into_ret());
        e.register(RT_DYNAMIC_CAST, |_, a| (a[0] + 1).into_ret());
        e.register(HAS_TARGET_EFFECT, |_, _| 1u32.into_ret());
        quiet(
            &mut e,
            &[
                slot(0, 0x10),
                slot(0, 0),
                slot(0, 4),
                slot(0, 0x40),
                slot(0, 0x48),
                slot(0, 0x1c),
                slot(0, 0x20),
            ],
        );
        e.register(slot(0, 0x34), |_, _| 0u32.into_ret());
        let ability = spell(&mut e, 2, 1);
        assert!(e
            .call(0x0081_5480, &args![caster, ability, 0x99u32, 1u8])
            .bool());
        assert_eq!(
            calls(&e, slot(0, 0x10)),
            vec![vec![caster, ability + 1, 0x99, 1]]
        );
        let kind4 = spell(&mut e, 4, 4);
        e.call(0x0081_5480, &args![caster, kind4, 0x99u32, 0u8]);
        assert_eq!(calls(&e, slot(0, 0)), vec![vec![caster, kind4 + 1, 0]]);
        let kind10 = spell(&mut e, 6, 0xa);
        e.call(0x0081_5480, &args![caster, kind10, 0x99u32, 0u8]);
        assert_eq!(calls(&e, slot(0, 4)), vec![vec![caster, kind10 + 1, 0]]);
        let other = spell(&mut e, 8, 0);
        assert!(e
            .call(0x0081_5480, &args![caster, other, 0x99u32, 0u8])
            .bool());
        assert_eq!(calls(&e, slot(0, 0x40)), vec![vec![caster, other]]);
        assert_eq!(calls(&e, slot(0, 0x48)), vec![vec![caster, 0x99]]);
        assert_eq!(calls(&e, slot(0, 0x1c)).len(), 1);
        e.call(0x0081_5480, &args![caster, other, 0u32, 0u8]);
        assert_eq!(calls(&e, slot(0, 0x20)).len(), 1);
        // Busy and disabled.
        e.register(slot(0, 0x34), |_, _| 1u32.into_ret());
        assert!(!e.call(0x0081_5480, &args![caster, other, 0u32, 0u8]).bool());
        assert_eq!(e.mem.u32(caster + 8), 6);
        e.register(SPELL_DISABLED, |_, _| 1u32.into_ret());
        assert!(!e.call(0x0081_5480, &args![caster, other, 0u32, 0u8]).bool());
        assert_eq!(e.mem.u32(caster + 8), 5);
    }

    #[test]
    fn addiction_and_radiation_cast_through_find_targets() {
        let mut e = engine();
        let caster = object(&mut e, 0, 0x10);
        // FindTargets stops at once without a spell.
        e.register(slot(0, 0x34), |_, _| 0u32.into_ret());
        quiet(&mut e, &[slot(0, 0x40)]);
        let addiction = spell(&mut e, 2, 0xa);
        e.call(0x0081_5670, &args![caster, addiction, 1u8]);
        assert_eq!(
            calls(&e, slot(0, 0x40)),
            vec![vec![caster, addiction + 0x18]]
        );
        let other = spell(&mut e, 4, 1);
        e.call(0x0081_5670, &args![caster, other, 1u8]);
        assert_eq!(calls(&e, slot(0, 0x40)).len(), 1);
        e.call(0x0081_56e0, &args![caster, other, 1u8]);
        assert_eq!(calls(&e, slot(0, 0x40))[1], vec![caster, other + 0x18]);
        e.call(0x0081_56e0, &args![caster, 0u32, 1u8]);
        assert_eq!(calls(&e, slot(0, 0x40))[2], vec![caster, 0]);
    }

    #[test]
    fn release_cast_fails_or_casts() {
        let mut e = engine();
        let caster = object(&mut e, 0, 0x10);
        e.register(slot(0, 0x34), |_, _| 0x5500u32.into_ret());
        e.register(RT_DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.register(slot(0, 0x28), |e, a| {
            e.mem.set_f32(a[2], 3.5);
            0u32.into_ret()
        });
        // Not an actor caster.
        e.register(slot(0, 0x2c), |_, _| 0u32.into_ret());
        e.register(GET_AV_EFFECT, |_, _| 0u32.into_ret());
        e.register(STRING_TEXT, |_, _| 0x7e00u32.into_ret());
        quiet(
            &mut e,
            &[
                SPELL_NAME,
                slot(0, 0x24),
                slot(0, 0x40),
                slot(0, 0x48),
                STRING_DESTRUCT,
                MAGIC_LIGHT_FADE,
            ],
        );
        assert!(!e.call(0x0081_5870, &args![caster, 0x66u32]).bool());
        assert_eq!(e.mem.u32(caster + 8), 7);
        assert_eq!(calls(&e, slot(0, 0x24)), vec![vec![caster, 0x5500, 0]]);
        assert_eq!(calls(&e, slot(0, 0x40)), vec![vec![caster, 0]]);
        assert_eq!(calls(&e, SPELL_NAME)[0][0], 0x5500);
        assert!(calls(&e, SHOW_MESSAGE).is_empty());
        // CheckCast passes: FindTargets (it stops after the state, the spell is doubled
        // away), SpellCast with 1, and the light fades.
        e.register(slot(0, 0x28), |e, a| {
            e.mem.set_f32(a[2], 3.5);
            1u32.into_ret()
        });
        e.mem.set_u32(caster + 4, 0x4400);
        // The fifth question for the spell is FindTargets' own: no spell, it stops.
        let mut asked = 0;
        e.register_double(slot(0, 0x34), move |_, _| {
            asked += 1;
            (if asked < 5 { 0x5500u32 } else { 0 }).into_ret()
        });
        assert!(e.call(0x0081_5870, &args![caster, 0x66u32]).bool());
        assert_eq!(e.mem.u32(caster + 8), 3);
        assert_eq!(
            calls(&e, slot(0, 0x24)).last(),
            Some(&vec![caster, 0x5500, 1])
        );
        assert_eq!(
            calls(&e, MAGIC_LIGHT_FADE),
            vec![vec![0x4400, 0, 0.75f32.to_bits()]]
        );
        // No spell: false at once.
        e.register(slot(0, 0x34), |_, _| 0u32.into_ret());
        assert!(!e.call(0x0081_5870, &args![caster, 0x66u32]).bool());
    }

    /// A reference whose magic target (slot +0x19C) is `target` and which has 3D.
    fn reference(e: &mut Engine, family: u32, target: u32) -> u32 {
        let reference = object(e, family, 0x100);
        e.register_double(slot(family, 0x19c), move |_, _| target.into_ret());
        e.register(slot(family, 0x1d0), |_, _| 1u32.into_ret());
        reference
    }

    #[test]
    fn find_targets_in_area_hits_within_the_area() {
        let mut e = engine();
        let caster = object(&mut e, 0, 0x10);
        settings(&mut e);
        e.register(slot(0, 0x2c), |_, _| 0u32.into_ret());
        e.register(WORD_AT_C, |_, _| 0x2200u32.into_ret());
        e.register(EFFECT_ITEM_GET_AREA, |_, _| 2u32.into_ret());
        e.register(GET_TARGET_AS_ACTOR, |_, _| 0u32.into_ret());
        e.register(DISTANCE_TO_POINT, |e, a| {
            st0(f64::from(e.mem.u32(a[0] + 0x40)))
        });
        let target = object(&mut e, 9, 0x10);
        e.register(slot(9, 0), |_, _| 1u32.into_ret());
        let near = reference(&mut e, 2, target);
        e.mem.set_u32(near + 0x40, 400);
        let far = reference(&mut e, 3, target);
        e.mem.set_u32(far + 0x40, 600);
        // area 2 * (0xe0 * 1.0) = 448.
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(first, near);
        e.mem.set_u32(first + 4, second);
        e.mem.set_u32(second, far);
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let found = e.mem.alloc(4);
        e.call(
            0x0081_6f10,
            &args![
                caster, 0x5500u32, 0x6600u32, 0x7000u32, 1.0f32, 2.0f32, 3.0f32, 0u32, 0u32, first,
                found, 1.0f32
            ],
        );
        assert_eq!(
            calls(&e, slot(9, 0)),
            vec![vec![target, caster, 0x5500, 0x6600, 1]]
        );
        assert_eq!(e.mem.u8(found), 0);
        // No location: nothing.
        e.call(
            0x0081_6f10,
            &args![
                caster, 0x5500u32, 0x6600u32, 0u32, 1.0f32, 2.0f32, 3.0f32, 0u32, 0u32, first,
                found, 1.0f32
            ],
        );
        assert_eq!(calls(&e, slot(9, 0)).len(), 1);
        // The touch target is skipped.
        e.call(
            0x0081_6f10,
            &args![
                caster, 0x5500u32, 0x6600u32, 0x7000u32, 1.0f32, 2.0f32, 3.0f32, target, 0u32,
                first, found, 1.0f32
            ],
        );
        assert_eq!(calls(&e, slot(9, 0)).len(), 1);
    }

    #[test]
    fn check_will_kill_sums_the_health_damage() {
        let mut e = engine();
        let caster = object(&mut e, 0, 0x10);
        e.register(slot(0, 0x2c), |_, _| 0u32.into_ret());
        let target = object(&mut e, 1, 0x10);
        let actor = object(&mut e, 2, 0x10);
        e.register_double(slot(1, 4), move |_, _| actor.into_ret());
        e.register(slot(2, 0x100), |_, _| 1u32.into_ret());
        e.register(slot(2, 0x1d0), |_, _| 1u32.into_ret());
        e.register(EFFECT_LIST_IS_HOSTILE, |_, _| 1u32.into_ret());
        e.register(ACTOR_HEALTH, |_, _| st0(20.0));
        let effect = e.mem.alloc(4);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, effect);
        e.register_double(slot(1, 8), move |_, _| node.into_ret());
        e.register(NODE_IS_EMPTY, |e, a| (e.mem.u32(a[0]) == 0).into_ret());
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(ACTIVE_EFFECT_SPELL, |_, _| 0x5500u32.into_ret());
        e.register(ACTIVE_EFFECT_SETTING, |_, _| 0x6600u32.into_ret());
        e.register(SETTING_USES_OWN_VALUE, |_, _| 1u32.into_ret());
        e.register(SETTING_ACTOR_VALUE, |_, _| 0x10u32.into_ret());
        e.register(SETTING_HAS_FLAG, |_, a| (a[1] == 4).into_ret());
        e.register(ACTIVE_EFFECT_MAGNITUDE, |_, _| st0(5.0));
        e.register(ACTIVE_EFFECT_DURATION, |_, _| st0(0.0));
        e.register(ADJUST_HEALTH_MODIFIER, |_, a| {
            st0(f64::from(f32::from_bits(a[1])) * 4.0)
        });
        // 5 damage, adjusted to 20: kills at health 20.
        assert!(e
            .call(0x0081_7970, &args![caster, target, 0x5500u32])
            .bool());
        assert_eq!(calls(&e, ADJUST_HEALTH_MODIFIER)[0][1], (-5.0f32).to_bits());
        e.register(ACTOR_HEALTH, |_, _| st0(21.0));
        assert!(!e
            .call(0x0081_7970, &args![caster, target, 0x5500u32])
            .bool());
        // With a duration: 5 * 0040ebd0(4.0, 00404010(0.0, d)).
        e.register(ACTOR_HEALTH, |_, _| st0(30.0));
        e.register(ACTIVE_EFFECT_DURATION, |_, _| st0(2.0));
        e.register(DURATION_PART, |_, _| st0(3.0));
        e.register(DURATION_SCALE, |_, _| st0(1.5));
        assert!(e
            .call(0x0081_7970, &args![caster, target, 0x5500u32])
            .bool());
        assert_eq!(
            calls(&e, DURATION_SCALE)[0],
            vec![4.0f32.to_bits(), 3.0f32.to_bits()]
        );
        // Not hostile, or no spell: false.
        e.register(EFFECT_LIST_IS_HOSTILE, |_, _| 0u32.into_ret());
        assert!(!e
            .call(0x0081_7970, &args![caster, target, 0x5500u32])
            .bool());
        assert!(!e.call(0x0081_7970, &args![caster, target, 0u32]).bool());
    }

    #[test]
    fn collect_targets_adds_actors_in_the_area() {
        let mut e = engine();
        let caster = object(&mut e, 0, 0x10);
        settings(&mut e);
        e.register(slot(0, 0x2c), |_, _| 0u32.into_ret());
        let effect_list = e.mem.alloc(0x10);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, 0x2200);
        e.mem.set_u32(effect_list + 4, node);
        let spell_object = effect_list - 0xc;
        e.register(NODE_IS_EMPTY, |e, a| (e.mem.u32(a[0]) == 0).into_ret());
        e.register(NODE_ITEM, |e, a| {
            // The list head is a pointer to the node in these tests.
            let node = e.mem.u32(a[0]);
            node.into_ret()
        });
        e.register(NODE_NEXT, |_, _| 0u32.into_ret());
        e.register(WORD_AT_C, |_, _| 2u32.into_ret());
        e.register(EFFECT_ITEM_GET_AREA, |_, _| 3u32.into_ret());
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(SPELL_HAS_FLAG, |_, _| 1u32.into_ret());
        let near = reference(&mut e, 2, 0x9900);
        e.mem.set_u32(near + 0x40, 600);
        e.register(DISTANCE_TO_POINT, |e, a| {
            st0(f64::from(e.mem.u32(a[0] + 0x40)))
        });
        e.register(ACTOR_EXCLUDED, |_, _| 0u32.into_ret());
        e.register(HIGH_ACTORS, |_, _| 0x7000u32.into_ret());
        e.register(ACTOR_COUNT, |_, _| 1u32.into_ret());
        e.register_double(ACTOR_AT, move |_, _| near.into_ret());
        let player = object(&mut e, 3, 0x100);
        e.mem.set_u32(PLAYER, player);
        e.register(slot(3, 0x1d0), |_, _| 1u32.into_ret());
        e.mem.set_u32(player + 0x40, 700);
        let list = e.mem.alloc(8);
        quiet(&mut e, &[LIST_ADD]);
        // Radius 3 * 0xe0 = 672: the reference at 600 is in, the player at 700 is not.
        e.call(
            0x0081_8ce0,
            &args![caster, spell_object, 2u32, 1.0f32, 2.0f32, 3.0f32, list],
        );
        let added = calls(&e, LIST_ADD);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], list);
        // No list: nothing at all.
        e.call(
            0x0081_8ce0,
            &args![caster, spell_object, 2u32, 1.0f32, 2.0f32, 3.0f32, 0u32],
        );
        assert_eq!(calls(&e, LIST_ADD).len(), 1);
    }

    #[test]
    fn line_of_sight_pick_answers_whether_nothing_is_hit() {
        let mut e = engine();
        let caster = object(&mut e, 0, 0x10);
        let actor = object(&mut e, 1, 0x100);
        let world = object(&mut e, 2, 0x10);
        e.register(PARENT_CELL, |_, _| 0x5400u32.into_ret());
        e.register_double(CELL_HAVOK_WORLD, move |_, _| world.into_ret());
        let position = e.mem.alloc(0xc);
        e.mem.set_f32(position + 8, 10.0);
        e.register_double(slot(1, 0x1f4), move |_, _| position.into_ret());
        e.register(GET_HEIGHT, |_, _| st0(4.0));
        e.register(GET_CHAR_CONTROLLER, |_, _| 0u32.into_ret());
        e.register(DEFAULT_LAYER, |_, _| 0x31u32.into_ret());
        e.register(slot(2, 0xc8), |_, _| 0u32.into_ret());
        quiet(
            &mut e,
            &[
                SET_MULTI_LAYER_COLLISION,
                PICK_CONSTRUCT,
                FILTER_CONSTRUCT,
                FILTER_SET_LAYER,
                FILTER_SET_GROUP,
                PICK_SET_FILTER,
                PICK_SET_FROM,
            ],
        );
        e.register(PICK_SET_TO, |e, a| {
            assert_eq!(e.mem.f32(a[1] + 8), 12.0);
            Ret::default()
        });
        assert!(e
            .call(0x0081_90d0, &args![caster, 1.0f32, 2.0f32, 3.0f32, actor])
            .bool());
        assert_eq!(
            calls(&e, SET_MULTI_LAYER_COLLISION),
            vec![vec![0x27, 0x26206, 0]]
        );
        assert_eq!(calls(&e, FILTER_SET_GROUP)[0][1], 0x31);
        e.register(slot(2, 0xc8), |_, _| 1u32.into_ret());
        assert!(!e
            .call(0x0081_90d0, &args![caster, 1.0f32, 2.0f32, 3.0f32, actor])
            .bool());
        assert!(!e
            .call(0x0081_90d0, &args![caster, 1.0f32, 2.0f32, 3.0f32, 0u32])
            .bool());
    }

    #[test]
    fn fire_projectile_launches_from_the_caster() {
        let mut e = engine();
        let caster = object(&mut e, 0, 0x10);
        settings(&mut e);
        let reference = object(&mut e, 1, 0x100);
        e.register_double(slot(0, 0x2c), move |_, _| reference.into_ret());
        e.register(slot(1, 0x100), |_, _| 0u32.into_ret());
        e.register(slot(0, 0x34), |_, _| 0x5500u32.into_ret());
        e.register(slot(0, 0x44), |_, _| 0u32.into_ret());
        e.register(slot(0, 0x30), |_, _| 0u32.into_ret());
        e.register(GET_COSTLIEST_EFFECT, |_, _| 0x2200u32.into_ret());
        e.register(GET_AV_EFFECT, |_, _| 0x3300u32.into_ret());
        e.register(PROJECTILE_SPEED, |_, _| st0(2.0));
        e.register(ANGLE_TO_TARGET, |_, a| a[0].into_ret());
        e.register(ANGLES, |e, _| e.mem.alloc(0xc).into_ret());
        e.register(POINT_SUM, |e, a| {
            e.mem.set_f32(a[1], 0.25);
            a[1].into_ret()
        });
        let position = e.mem.alloc(0xc);
        e.mem.set_f32(position + 8, 1.0);
        e.register_double(slot(1, 0x1f4), move |_, _| position.into_ret());
        let top = e.mem.alloc(0xc);
        e.mem.set_f32(top + 8, 4.0);
        let bottom = e.mem.alloc(0xc);
        e.register_double(slot(1, 0x1dc), move |_, _| top.into_ret());
        e.register_double(slot(1, 0x1d8), move |_, _| bottom.into_ret());
        quiet(&mut e, &[SELF_ITEM, PROJECTILE_LAUNCH, MAGIC_LIGHT_DELETE]);
        e.mem.set_u32(caster + 4, 0x4400);
        e.call(0x0081_8610, &args![caster]);
        let launch = &calls(&e, PROJECTILE_LAUNCH)[0];
        assert_eq!(launch[..6], [caster, 0, 0x4400, 0x5500, 0x2200, 0x3300]);
        // Raised by half the height: 1 + 2 = 3; aimed at 0.25.
        assert_eq!(launch[8], 3.0f32.to_bits());
        assert_eq!(launch[9], 0.25f32.to_bits());
        assert_eq!(e.mem.u32(caster + 4), 0);
        assert_eq!(
            calls(&e, ANGLE_TO_TARGET)[0][1..],
            [reference, 0, (2.0f32 * 248.0).to_bits(), 0, 2]
        );
        assert!(calls(&e, MAGIC_LIGHT_DELETE).is_empty());
        // Without an effect nothing is launched.
        e.register(GET_COSTLIEST_EFFECT, |_, _| 0u32.into_ret());
        e.call(0x0081_8610, &args![caster]);
        assert_eq!(calls(&e, PROJECTILE_LAUNCH).len(), 1);
    }

    #[test]
    fn body_push_scales_the_force_by_the_mass() {
        let mut e = engine();
        settings(&mut e);
        e.register(SELF_ITEM, |_, a| a[0].into_ret());
        e.register(BODY_POSITION, |_, _| 0x9000u32.into_ret());
        quiet(
            &mut e,
            &[
                COPY_POINT,
                POINT_DIFFERENCE,
                POINT_SCALED,
                VECTOR_COPY,
                MASS_VECTOR,
                VECTOR_MULTIPLY,
                HKP_ENTITY_ACTIVATE,
                HKP_ENTITY_MOTION,
            ],
        );
        e.register(UNITIZE_GET_LENGTH, |_, _| st0(6.0));
        e.register(EFFECT_ITEM_GET_AREA, |_, _| 1u32.into_ret());
        e.register(EXPLOSION_FORCE, |_, _| st0(10.0));
        e.register(POINT_COMPONENT, |_, a| (a[0] + 8).into_ret());
        e.register(BODY_MASS, |_, _| st0(3.0));
        let body = 0x0b0d_0000u32;
        e.register(HKP_ENTITY_MOTION, |e, _| {
            let motion = e.mem.alloc(4);
            let vtable = e.mem.alloc(0x50);
            e.mem.set_u32(vtable + 0x4c, 0x0350_0000);
            e.mem.set_u32(motion, vtable);
            motion.into_ret()
        });
        quiet(&mut e, &[0x0350_0000]);
        e.call(
            0x0081_8150,
            &args![0u32, body, 1.0f32, 2.0f32, 3.0f32, 0x2200u32, 4.0f32],
        );
        assert_eq!(
            calls(&e, EXPLOSION_FORCE)[0],
            vec![4, 0xe0, 6.0f32.to_bits(), 0, 0, 0]
        );
        // 10 is below the limit (0x84) and times the scale (0x8c = 140).
        let scaled = &calls(&e, POINT_SCALED)[0];
        assert_eq!(scaled[2], 1400.0f32.to_bits());
        assert_eq!(calls(&e, MASS_VECTOR)[0][1], 3.0f32.to_bits());
        assert_eq!(calls(&e, HKP_ENTITY_ACTIVATE).len(), 2);
        // No force: nothing pushed.
        e.register(EXPLOSION_FORCE, |_, _| st0(0.0));
        e.call(
            0x0081_8150,
            &args![0u32, body, 1.0f32, 2.0f32, 3.0f32, 0x2200u32, 4.0f32],
        );
        assert_eq!(calls(&e, POINT_SCALED).len(), 1);
    }

    #[test]
    fn explosion_needs_the_flag_and_a_radius() {
        let mut e = engine();
        e.mem.set_u32(PLAYER, 0);
        let caster = object(&mut e, 0, 0x10);
        settings(&mut e);
        e.register(EFFECT_ITEM_SETTING, |_, _| 0x6600u32.into_ret());
        e.register(SETTING_HAS_FLAG, |_, _| 0u32.into_ret());
        e.call(
            0x0081_7b90,
            &args![
                caster, 1.0f32, 2.0f32, 3.0f32, 0x5400u32, 0x5500u32, 0x2200u32, 0u32, 1.0f32,
                1.0f32
            ],
        );
        assert!(calls(&e, EFFECT_ITEM_GET_AREA).is_empty());
        e.register(SETTING_HAS_FLAG, |_, _| 1u32.into_ret());
        e.register(EFFECT_ITEM_GET_AREA, |_, _| 0u32.into_ret());
        e.call(
            0x0081_7b90,
            &args![
                caster, 1.0f32, 2.0f32, 3.0f32, 0x5400u32, 0x5500u32, 0x2200u32, 0u32, 1.0f32,
                1.0f32
            ],
        );
        assert!(calls(&e, EFFECT_ITEM_GET_MAGNITUDE).is_empty());
        // With an area: the phantom is made, no bodies, no actors.
        e.register(EFFECT_ITEM_GET_AREA, |_, _| 2u32.into_ret());
        e.register(WORD_AT_C, |_, _| 2u32.into_ret());
        e.register(EFFECT_ITEM_GET_MAGNITUDE, |_, _| 10u32.into_ret());
        e.register(EFFECT_ITEM_GET_DURATION, |_, _| 4u32.into_ret());
        e.register(SETTING_BASE_COST, |_, _| st0(1.5));
        e.register(GET_MAGIC_EFFECT_COST, |_, _| st0(30.0));
        e.register(POINT_SET, |_, a| a[0].into_ret());
        e.register(POINT_SUM, |_, a| a[1].into_ret());
        e.register(DEFAULT_LAYER, |_, _| 0x31u32.into_ret());
        e.register(FILTER_INFO, |_, a| a[0].into_ret());
        e.register(FIRST_WORD, |_, _| 0x1234u32.into_ret());
        e.register(CELL_HAVOK_WORLD, |_, _| 0x5500u32.into_ret());
        e.register(CHARACTER_PROXY, |_, _| 0x8800u32.into_ret());
        let bodies = e.mem.alloc(8);
        e.register_double(PROXY_LIST, move |_, _| bodies.into_ret());
        e.register(NODE_NEXT_WORD, |_, _| 0u32.into_ret());
        e.register(slot(0, 0x2c), |_, _| 0u32.into_ret());
        quiet(
            &mut e,
            &[
                PHANTOM_INFO_CONSTRUCT,
                VECTOR_COPY,
                AABB_PHANTOM_CONSTRUCT,
                WORLD_OBJECT_ADD,
                PHANTOM_REMOVE,
                AABB_PHANTOM_DESTRUCT,
                PHANTOM_INFO_DESTRUCT,
            ],
        );
        e.call(
            0x0081_7b90,
            &args![
                caster, 1.0f32, 2.0f32, 3.0f32, 0x5400u32, 0x5500u32, 0x2200u32, 0u32, 2.0f32,
                1.0f32
            ],
        );
        assert_eq!(
            calls(&e, GET_MAGIC_EFFECT_COST)[0],
            vec![1.5f32.to_bits(), 2, 8, 20, 1]
        );
        // Radius 2 * 0xe0 = 448, half 224, layer 0x29.
        assert_eq!(calls(&e, POINT_SET)[1][1], 224.0f32.to_bits());
        assert_eq!(calls(&e, POINT_SET)[0][1], (-224.0f32).to_bits());
        assert_eq!(calls(&e, FILTER_INFO)[0][1..], [0x29, 0x31, 0]);
        assert_eq!(calls(&e, WORLD_OBJECT_ADD)[0][1], 0x5500);
        assert_eq!(calls(&e, PHANTOM_REMOVE).len(), 1);
    }

    /// An actor object (vtable family `actor_family`) with its `MagicCaster` part at +0x88
    /// (vtable family `caster_family`) whose stats object says it is an actor.
    fn actor_caster(e: &mut Engine, actor_family: u32, caster_family: u32) -> (u32, u32) {
        let actor = object(e, actor_family, 0x200);
        let caster_vtable = e.mem.alloc(0x500);
        for i in 0..0x140 {
            e.mem
                .set_u32(caster_vtable + 4 * i, slot(caster_family, 4 * i));
        }
        let caster = actor + 0x88;
        e.mem.set_u32(caster, caster_vtable);
        let stats = object(e, caster_family + 1, 0x10);
        e.register_double(slot(caster_family, 0x2c), move |_, _| stats.into_ret());
        e.register(slot(caster_family + 1, 0x100), |_, _| 1u32.into_ret());
        (actor, caster)
    }

    #[test]
    fn touch_target_picks_the_nearest_reachable_actor() {
        let mut e = engine();
        let (actor, caster) = actor_caster(&mut e, 1, 5);
        let player = object(&mut e, 3, 0x100);
        e.mem.set_u32(PLAYER, player);
        let other = object(&mut e, 4, 0x100);
        e.register(slot(4, 0x100), |_, _| 1u32.into_ret());
        e.register(slot(4, 0x22c), |_, _| 0u32.into_ret());
        e.register(slot(4, 0x1d0), |_, _| 1u32.into_ret());
        e.register(slot(1, 0x380), |_, _| st0(100.0));
        e.register(GET_SCALE, |_, _| st0(1.0));
        e.register(DISTANCE_BETWEEN_FACING_ACTORS, |_, _| st0(50.0));
        e.register(ACTOR_REACHES, |e, a| {
            e.mem.set_f32(a[2], 40.0);
            1u32.into_ret()
        });
        e.register(HIGH_ACTORS, |_, _| 0x7000u32.into_ret());
        e.register(ACTOR_COUNT, |_, _| 1u32.into_ret());
        e.register_double(ACTOR_AT, move |_, _| other.into_ret());
        // The player reaches at 40; the later actor at 40 wins too (not below).
        assert_eq!(e.call(0x0081_7620, &args![caster]).u32(), other + 0x94);
        assert_eq!(
            calls(&e, DISTANCE_BETWEEN_FACING_ACTORS)[0],
            vec![actor, player, 0]
        );
        // Out of reach: the reach (100) is below 150.
        e.register(DISTANCE_BETWEEN_FACING_ACTORS, |_, _| st0(150.0));
        assert_eq!(e.call(0x0081_7620, &args![caster]).u32(), 0);
    }

    #[test]
    fn explode_actor_pushes_or_knocks_down() {
        let mut e = engine();
        let (caster_actor, caster) = actor_caster(&mut e, 5, 6);
        e.register(slot(5, 0x1d0), |_, _| 0u32.into_ret());
        settings(&mut e);
        let actor = object(&mut e, 1, 0x100);
        let top = e.mem.alloc(0xc);
        e.mem.set_f32(top + 8, 2.0);
        let bottom = e.mem.alloc(0xc);
        e.register_double(slot(1, 0x1dc), move |_, _| top.into_ret());
        e.register_double(slot(1, 0x1d8), move |_, _| bottom.into_ret());
        let position = e.mem.alloc(0xc);
        e.register_double(slot(1, 0x1f4), move |_, _| position.into_ret());
        e.register(POINT_DIFFERENCE, |_, _| Ret::default());
        e.register(POINT_LENGTH, |_, _| st0(7.0));
        e.register(ACTOR_FATIGUE, |_, _| st0(0.5));
        e.register(CLAMPED_ACTOR_VALUE, |_, a| (a[1] + 0x10).into_ret());
        e.register(EFFECT_ITEM_GET_AREA, |_, _| 1u32.into_ret());
        e.register(EXPLOSION_FORCE, |_, _| st0(1000.0));
        let process = object(&mut e, 2, 0x10);
        e.register_double(ACTOR_PROCESS, move |_, _| process.into_ret());
        quiet(&mut e, &[slot(2, 0x418), slot(1, 0x478)]);
        e.register(CONSOLE_ENABLED, |_, _| 0u32.into_ret());
        // Limit 0x84 (132), threshold 0x0c (12): pushed with 132.
        e.call(
            0x0081_8310,
            &args![caster, actor, 1.0f32, 2.0f32, 3.0f32, 0x2200u32, 9.0f32],
        );
        assert_eq!(
            calls(&e, EXPLOSION_FORCE)[0],
            vec![9, 0xe0, 7.0f32.to_bits(), 0x1a, 0x1b, 0.5f32.to_bits()]
        );
        assert_eq!(
            calls(&e, slot(2, 0x418)),
            vec![vec![
                process,
                actor,
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                132.0f32.to_bits()
            ]]
        );
        // A force of 5 (not beyond the threshold) knocks the actor down.
        e.register(EXPLOSION_FORCE, |_, _| st0(5.0));
        e.register(CONSOLE_ENABLED, |_, _| 1u32.into_ret());
        e.register(ACTOR_NAME, |_, _| 0x7e00u32.into_ret());
        quiet(&mut e, &[CONSOLE_PRINT]);
        e.call(
            0x0081_8310,
            &args![caster, actor, 1.0f32, 2.0f32, 3.0f32, 0x2200u32, 9.0f32],
        );
        let knock = &calls(&e, slot(1, 0x478))[0];
        assert_eq!(knock[1..4], [caster_actor, 10.0f32.to_bits(), 0]);
        let print = &calls(&e, CONSOLE_PRINT)[0];
        assert_eq!(print[0], TEXT_EXPLOSION);
        assert_eq!(print[3], 0x7e00);
        assert_eq!(
            f64::from_bits(u64::from(print[1]) | u64::from(print[2]) << 32),
            5.0
        );
    }

    #[test]
    fn find_targets_ov2_applies_target_effects() {
        let mut e = engine();
        let caster = object(&mut e, 0, 0x10);
        settings(&mut e);
        let spell_object = object(&mut e, 1, 0x40);
        e.register(slot(1, 0x18), |_, _| 1u32.into_ret());
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, 0x2200);
        // The effect list's node (+4 of the list at +0xC).
        e.register_double(NODE_ITEM, move |_, a| {
            if a[0] == spell_object + 0x10 {
                node.into_ret()
            } else {
                a[0].into_ret()
            }
        });
        e.register(NODE_NEXT, |_, _| 0u32.into_ret());
        // The target collection finds an empty list; the area has no size.
        e.register(NODE_IS_EMPTY, |_, _| 1u32.into_ret());
        e.register(WORD_AT_C, |_, _| 2u32.into_ret());
        e.register(EFFECT_ITEM_GET_AREA, |_, _| 0u32.into_ret());
        e.register(GET_COSTLIEST_EFFECT, |_, a| {
            (if a[2] == 0 { 0x2200u32 } else { 0 }).into_ret()
        });
        let effect = object(&mut e, 2, 0x10);
        e.register_double(slot(0, 0x4c), move |_, _| effect.into_ret());
        quiet(
            &mut e,
            &[
                LOCAL_LIST_CONSTRUCT,
                LOCAL_LIST_REMOVE_ALL,
                LOCAL_LIST_DESTRUCT,
                ACTIVE_EFFECT_ADJUST_EFFECTIVENESS,
                slot(2, 0),
                ACTIVE_EFFECT_SET_FLAG,
            ],
        );
        e.register(BASE_FORM, |_, _| 0x4000u32.into_ret());
        e.register(PROJECTILE_KIND_OF_FORM, |_, _| 3u32.into_ret());
        e.call(
            0x0081_7160,
            &args![
                caster,
                spell_object,
                0x5400u32,
                1.0f32,
                2.0f32,
                3.0f32,
                0x6600u32,
                0u32,
                0u32,
                2.0f32,
                1.0f32
            ],
        );
        assert_eq!(
            calls(&e, slot(0, 0x4c)),
            vec![vec![caster, spell_object, 0x2200, 0]]
        );
        assert_eq!(
            calls(&e, ACTIVE_EFFECT_ADJUST_EFFECTIVENESS),
            vec![vec![effect, 2.0f32.to_bits()]]
        );
        assert!(calls(&e, ACTIVE_EFFECT_SET_FLAG).is_empty());
        assert_eq!(calls(&e, NODE_IS_EMPTY)[0], vec![spell_object + 0x10]);
        assert!(calls(&e, WORD_AT_C).contains(&vec![effect]));
        assert_eq!(calls(&e, slot(2, 0)), vec![vec![effect, 1]]);
        assert_eq!(calls(&e, PROJECTILE_KIND_OF_FORM), vec![vec![0x4000]]);
    }
}
