//! `fallout/magic/magictarget.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `MagicTarget` is the base every object that spells can affect derives
//! from (actors at `Actor + 0x94`). Nearly every function walks the target's
//! active-effect list (`BSSimpleList<ActiveEffect *>`, from the virtual
//! `GetActiveEffectList`, slot +0x08) node by node through the list helpers
//! `008256d0` (node empty), `006815c0` (the node's item word) and `00726070`
//! (next node), and reads the effects through their one-field getters
//! (offsets in the constants below, checked against the getters' bodies).
//!
//! x87 note: floats are computed in `f64` and rounded to `f32` where the code
//! stores a `float`; an `FLD`/`FSTP` copy goes through [`x87_quiet`].

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleList;

layout! {
    /// `MagicTarget` (Xbox PDB), 0x10 bytes. PC offsets checked at
    /// 00823cf7 (`bUpdating`) and 0082437c (`PostUpdateDispelList`).
    pub struct MagicTarget: 0x10 {
        /// `bUpdating` (Xbox PDB): set while `UpdateTarget` walks the list;
        /// dispels requested meanwhile are queued instead of run.
        0x05 bUpdating: bool,
        /// `PostUpdateDispelList` (Xbox PDB): `BSSimpleList<SpellDispelData *>`.
        0x08 PostUpdateDispelList: Inline<BSSimpleList>,
    }

    /// `SpellDispelData` (Xbox PDB), 0x0C bytes; offsets checked at
    /// 008242f9-00824310 and 008243aa-008243bb.
    pub struct SpellDispelData: 0x0C {
        /// `pSpell` (Xbox PDB): `MagicItem *`.
        0x00 pSpell: u32,
        /// `pCaster` (Xbox PDB): `MagicCaster *`.
        0x04 pCaster: u32,
        /// `pActiveEffect` (Xbox PDB): `ActiveEffect *`.
        0x08 pActiveEffect: u32,
    }
}

/// `MagicTarget` virtual slots (Xbox PDB vtable order).
const SLOT_GET_TARGET_STATS_OBJECT: u32 = 0x04;
const SLOT_GET_ACTIVE_EFFECT_LIST: u32 = 0x08;
const SLOT_MAGIC_TARGET_IS_ACTOR: u32 = 0x0C;
const SLOT_IS_INVULNERABLE: u32 = 0x10;
const SLOT_EFFECT_ADDED: u32 = 0x14;
const SLOT_EFFECT_REMOVED: u32 = 0x18;
const SLOT_EFFECT_REFLECTED: u32 = 0x1C;
const SLOT_EFFECT_ABSORBED: u32 = 0x20;
const SLOT_CHECK_RESISTANCE: u32 = 0x24;
const SLOT_CHECK_ABSORB: u32 = 0x28;
const SLOT_CHECK_REFLECT: u32 = 0x2C;

/// `ActiveEffect` virtual slots used here (Xbox PDB vtable order).
const ACTIVE_EFFECT_SLOT_DELETE: u32 = 0x00;
const ACTIVE_EFFECT_SLOT_COPY: u32 = 0x04;
const ACTIVE_EFFECT_SLOT_FINALIZE: u32 = 0x08;
const ACTIVE_EFFECT_SLOT_CHECK_TARGET: u32 = 0x4C;

/// `MagicItem` virtual slot +0x18: the spell type, compared with small
/// numbers (4, 7, 10, ...).
const MAGIC_ITEM_SLOT_TYPE: u32 = 0x18;
/// `MagicItem` virtual slots +0x1C and +0x20: the `bool`s `AddTarget` and
/// `CheckAddEffect` test before the reflect/absorb checks.
const MAGIC_ITEM_SLOT_1C: u32 = 0x1C;
const MAGIC_ITEM_SLOT_20: u32 = 0x20;

/// Distance from the `Actor` to its `MagicTarget` base (`SUB reg,0x94`).
const ACTOR_MAGIC_TARGET: u32 = 0x94;
/// Distance from the `Actor` to its `MagicCaster` base (`ADD reg,0x88`).
const ACTOR_MAGIC_CASTER: u32 = 0x88;
/// The player (`PlayerCharacter *` global, the address the other units use).
const PLAYER: u32 = 0x011d_ea3c;

/// List helpers (bodies: both words zero / the node itself / word at +4).
const LIST_NODE_IS_EMPTY: u32 = 0x0082_56d0;
const LIST_NODE_ITEM: u32 = 0x0068_15c0;
const LIST_NODE_NEXT: u32 = 0x0072_6070;

/// `ActiveEffect` getters (body: the field at the offset; Xbox PDB names).
/// +0x08 `pSpell`.
const EFFECT_SPELL: u32 = 0x0044_ddc0;
/// +0x0C `pEffect` (`EffectItem *`). The same one-word getter at +0x0C is
/// also used on other objects (an `EffectItem`'s +0x0C, a form's id).
const EFFECT_ITEM: u32 = 0x0084_e3a0;
/// The same getter read as a form's id (`TESForm` +0x0C).
const FORM_ID_OF: u32 = EFFECT_ITEM;
/// +0x10 `bActive`.
const EFFECT_IS_ACTIVE: u32 = 0x0083_3c20;
/// +0x11 `bStarted`.
const EFFECT_HAS_STARTED: u32 = 0x0082_4060;
/// +0x13 `bDone`.
const EFFECT_IS_DONE: u32 = 0x0070_5cf0;
/// `iFlags` (+0x18) bit 0x80.
const EFFECT_FLAG_80: u32 = 0x007d_f210;
/// +0x1C `fMagnitude` (`float` in ST0).
const EFFECT_MAGNITUDE: u32 = 0x0049_5460;
/// +0x20 `fDuration` (`float` in ST0).
const EFFECT_DURATION: u32 = 0x007d_f1f0;
/// +0x24 `pTarget` (the engine map's name for the folded body is
/// `D3DTexture_LockRect`).
const EFFECT_TARGET: u32 = 0x0059_bb30;
/// +0x28 `pCaster`.
const EFFECT_CASTER: u32 = 0x0045_cd60;
/// +0x2C `eSpellType`.
const EFFECT_SPELL_TYPE: u32 = 0x0055_b980;
/// +0x3C `pSource`.
const EFFECT_SOURCE: u32 = 0x0063_9b40;
/// +0x44 `pDisplacementSpell` (map name of the folded body:
/// `LowProcess::GetGenericLocation`).
const EFFECT_DISPLACEMENT_SPELL: u32 = 0x0080_41a0;
/// `pEffect->pEffectSetting` (`00825c00` on the `EffectItem`).
const EFFECT_SETTING_OF: u32 = 0x007d_f1d0;
/// `ActiveEffect::Dispel` (`bool` argument).
const ACTIVE_EFFECT_DISPEL: u32 = 0x0080_4210;

/// `EffectItem::pEffectSetting` (+0x14).
const EFFECT_ITEM_SETTING: u32 = 0x0082_5c00;
/// `EffectItem::IsUsageMonitorEffect` (Xbox PDB).
const EFFECT_ITEM_IS_USAGE_MONITOR: u32 = 0x0040_4730;
/// `EffectItem::IsHostile` (Xbox PDB).
const EFFECT_ITEM_IS_HOSTILE: u32 = 0x0040_4710;
/// `EffectItem::GetEffectName` (Xbox PDB): fills the `BSStringT` at its
/// argument and returns it.
const EFFECT_ITEM_GET_EFFECT_NAME: u32 = 0x0040_4670;
/// The `EffectItem`'s actor value: -1 unless `00403ee0`, else the word at
/// +0x10.
const EFFECT_ITEM_ACTOR_VALUE: u32 = 0x0040_3ea0;
/// `EffectItemList::GetUsageMonitorEffect` (Xbox PDB).
const EFFECT_ITEM_LIST_USAGE_MONITOR: u32 = 0x0040_6200;
/// `EffectSetting` flag test: `iFlags (+0x58) & mask` (through `0058cba0`).
const EFFECT_SETTING_HAS_FLAG: u32 = 0x0040_3e60;
/// `EffectSetting::eArchetype` (+0x98).
const EFFECT_SETTING_ARCHETYPE: u32 = 0x0081_0fd0;

/// `__RTDynamicCast` (cdecl, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors (decorated names read at +8).
const MAGIC_ITEM_TYPE: u32 = 0x0118_3140;
const ALCHEMY_ITEM_TYPE: u32 = 0x0118_30cc;
const ENCHANTMENT_ITEM_TYPE: u32 = 0x0118_3594;
const INGREDIENT_ITEM_TYPE: u32 = 0x0118_39dc;
const TES_OBJECT_REFR_TYPE: u32 = 0x0118_41cc;
const ACTOR_TYPE: u32 = 0x0118_46d4;
const TES_FORM_TYPE: u32 = 0x0118_3028;
const TES_IMAGE_SPACE_MODIFIER_TYPE: u32 = 0x0118_62b8;
const ACTIVE_EFFECT_TYPE: u32 = 0x011a_0d1c;
const VALUE_AND_CONDITIONS_EFFECT_TYPE: u32 = 0x011a_13d0;
const LIMB_CONDITION_EFFECT_TYPE: u32 = 0x011a_118c;

/// `BSStringT` helpers: the C string (`00559450`), the destructor (`004037d0`).
const BS_STRING_C_STR: u32 = 0x0055_9450;
const BS_STRING_DESTROY: u32 = 0x0040_37d0;
/// The display name of a `MagicItem` (`MapMarkerData::GetLocationName` in the
/// engine map, a folded body).
const MAGIC_ITEM_NAME: u32 = 0x0040_8da0;
/// The full name of a target-stats object.
const STATS_OBJECT_NAME: u32 = 0x0055_d520;
/// Whether the debug log is on (the byte at `011c3534`).
const DEBUG_LOG_ENABLED: u32 = 0x005b_9210;
/// The debug log printer (cdecl, format and arguments).
const DEBUG_LOG: u32 = 0x0070_3c00;
/// `sprintf_s`-style formatter (cdecl: buffer, size, format, arguments).
const FORMAT_STRING: u32 = 0x0040_6d00;
/// A string setting's text (`00403df0`, thiscall on the setting).
const SETTING_STRING: u32 = 0x0040_3df0;
/// A float setting's value address (`00403e20`, thiscall on the setting).
const SETTING_FLOAT_ADDRESS: u32 = 0x0040_3e20;
/// The HUD message printer (cdecl, six words).
const SHOW_HUD_MESSAGE: u32 = 0x0070_52f0;
/// `BGSEntryPoint::HandleEntryPoint` (Xbox PDB), cdecl.
const HANDLE_ENTRY_POINT: u32 = 0x005e_58f0;
/// `BGSDefaultObjectManager::GetDefaultObject` (Xbox PDB), cdecl.
const GET_DEFAULT_OBJECT: u32 = 0x0058_db10;
/// `ImageSpaceModifierInstanceForm::Trigger` and `TriggerIfNotActive`
/// (Xbox PDB), cdecl (modifier, strength, word).
const IMAGE_SPACE_MODIFIER_TRIGGER: u32 = 0x0052_99a0;
const IMAGE_SPACE_MODIFIER_TRIGGER_IF_NOT_ACTIVE: u32 = 0x0052_9ae0;
/// Form lookup by id (cdecl).
const LOOKUP_FORM_BY_ID: u32 = 0x0048_39c0;

/// The empty string `"{NULL}"`.
const NULL_NAME: u32 = 0x0107_bcf8;
/// `"Target: %s FULLY RESISTED Spell: %s Effect: %s"`.
const FULLY_RESISTED_FORMAT: u32 = 0x0107_bcc8;
/// `"Invalid Target: Target: %s Spell: %s Effect: %s"`.
const INVALID_TARGET_FORMAT: u32 = 0x0107_bc98;
/// `"Effect %s from spell %s added to %s. Mag: %.2f/%d Dur: %.2f/%d"`.
const EFFECT_ADDED_FORMAT: u32 = 0x0107_bc58;
/// `"UIHealthChemsWearOff"` and `"UIHealthChemsAddicted"` (sound names).
const CHEMS_WEAR_OFF_SOUND: u32 = 0x0107_bd00;
const CHEMS_ADDICTED_SOUND: u32 = 0x0107_bd18;
/// `"%c%.30s%.1f%.1f"`.
const SORT_KEY_FORMAT: u32 = 0x0107_bd30;
/// The string settings `UpdateTarget` and `CheckAddiction` format their HUD
/// message with (their names are filled by the static initializers).
const WEAR_OFF_WITHDRAWAL_MESSAGE: u32 = 0x011d_1fc8;
const WEAR_OFF_MESSAGE: u32 = 0x011d_3c04;
const ADDICTED_MESSAGE: u32 = 0x011d_4cf0;
/// The float setting `CheckAddiction` compares the usage magnitude with.
const ADDICTION_USAGE_THRESHOLD: u32 = 0x011d_1878;

/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `100.0` (`double`).
const HUNDRED: u32 = 0x0101_7a40;
/// The `float` display time of the HUD messages (2.0 in the exe).
const HUD_MESSAGE_TIME: u32 = 0x0101_62c0;
/// The `float` sort value of an effect without a magnitude or duration
/// (1000.0 in the exe).
const NO_VALUE_SORT_KEY: u32 = 0x0101_3974;

/// Size of the stack buffers the HUD message and sort keys are formatted in.
const TEXT_BUFFER: u32 = 0x104;
/// Size of a `BSStringT`.
const BS_STRING_SIZE: u32 = 8;
/// `1.0f`, pushed as an argument.
const ONE_F32: f32 = 1.0;

fn effect_list(e: &mut Engine, this: Ptr<MagicTarget>) -> u32 {
    e.vcall(this.addr(), SLOT_GET_ACTIVE_EFFECT_LIST, &[]).u32()
}

fn node_is_empty(e: &mut Engine, node: u32) -> bool {
    e.call(LIST_NODE_IS_EMPTY, &args![node]).bool()
}

fn node_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
    e.mem.u32(slot)
}

fn node_next(e: &mut Engine, node: u32) -> u32 {
    e.call(LIST_NODE_NEXT, &args![node]).u32()
}

fn word_of(e: &mut Engine, getter: u32, object: u32) -> u32 {
    e.call(getter, &args![object]).u32()
}

fn flag_of(e: &mut Engine, getter: u32, object: u32) -> bool {
    e.call(getter, &args![object]).bool()
}

fn float_of(e: &mut Engine, getter: u32, object: u32) -> f32 {
    e.call(getter, &args![object]).f32()
}

fn dispel_effect(e: &mut Engine, effect: u32, remove: bool) {
    e.call(ACTIVE_EFFECT_DISPEL, &args![effect, remove]);
}

/// `effect->pEffect->pEffectSetting` has setting flag `mask`.
fn effect_setting_has_flag(e: &mut Engine, effect: u32, mask: u32) -> bool {
    let setting = word_of(e, EFFECT_SETTING_OF, effect);
    e.call(EFFECT_SETTING_HAS_FLAG, &args![setting, mask])
        .bool()
}

fn spell_type(e: &mut Engine, spell: u32) -> u32 {
    e.vcall(spell, MAGIC_ITEM_SLOT_TYPE, &[]).u32()
}

fn dynamic_cast(e: &mut Engine, object: u32, from: u32, to: u32) -> u32 {
    e.call(RT_DYNAMIC_CAST, &args![object, 0u32, from, to, 0u32])
        .u32()
}

/// The actor whose `MagicTarget` base `this` is, as the code computes it
/// (`this - 0x94`, null stays null).
fn actor_of(this: u32) -> u32 {
    if this == 0 {
        0
    } else {
        this.wrapping_sub(ACTOR_MAGIC_TARGET)
    }
}

/// `object + offset`, null staying null (the compiler's base-class
/// adjustment).
fn adjusted(object: u32, offset: u32) -> u32 {
    if object == 0 {
        0
    } else {
        object.wrapping_add(offset)
    }
}

/// The two `EffectItem`s of two effects compared by `004042a0`
/// (`effect->pEffect` against `other->pEffect`).
fn effect_items_match(e: &mut Engine, effect: u32, other: u32) -> bool {
    let other_item = word_of(e, EFFECT_ITEM, other);
    let item = word_of(e, EFFECT_ITEM, effect);
    e.call(0x0040_42a0, &args![item, other_item]).bool()
}

/// The target-stats object's full name, or `"{NULL}"`.
fn stats_name(e: &mut Engine, stats: u32) -> u32 {
    if stats == 0 {
        NULL_NAME
    } else {
        word_of(e, STATS_OBJECT_NAME, stats)
    }
}

/// The name of the target's stats object, asking the target twice as the
/// debug logs do.
fn target_name(e: &mut Engine, this: Ptr<MagicTarget>) -> u32 {
    if e.vcall(this.addr(), SLOT_GET_TARGET_STATS_OBJECT, &[])
        .u32()
        == 0
    {
        NULL_NAME
    } else {
        let stats = e
            .vcall(this.addr(), SLOT_GET_TARGET_STATS_OBJECT, &[])
            .u32();
        word_of(e, STATS_OBJECT_NAME, stats)
    }
}

/// Formats the HUD message setting `setting` with the spell's name into
/// `buffer`.
fn format_hud_message(e: &mut Engine, buffer: Ptr, setting: u32, spell: u32) {
    let name = word_of(e, MAGIC_ITEM_NAME, spell);
    let format = word_of(e, SETTING_STRING, setting);
    e.call(FORMAT_STRING, &args![buffer, TEXT_BUFFER, format, name]);
}

/// The `float` HUD message time, copied through the FPU.
fn hud_message_time(e: &mut Engine) -> u32 {
    x87_quiet_bits(e.global::<u32>(HUD_MESSAGE_TIME))
}

// Translated from 00822b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicTarget::GetTargetAsActor` (Xbox PDB): the actor this target is the
/// base of (`this - 0x94`), or null when `MagicTargetIsActor` says no.
pub fn magic_target_get_target_as_actor(e: &mut Engine, this: Ptr<MagicTarget>) -> Ptr {
    if e.vcall(this.addr(), SLOT_MAGIC_TARGET_IS_ACTOR, &[]).bool() {
        Ptr::new(actor_of(this.addr()))
    } else {
        Ptr::NULL
    }
}

// Translated from 00822b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicTarget::IsSpellTarget` (Xbox PDB): whether an effect of `spell`
/// (active, unless `include_inactive`) that is not a usage-monitor effect is
/// on this target.
pub fn magic_target_is_spell_target(
    e: &mut Engine,
    this: Ptr<MagicTarget>,
    spell: u32,
    include_inactive: bool,
) -> bool {
    let mut node = effect_list(e, this);
    let mut found = false;
    while node != 0 && !found {
        let effect = node_item(e, node);
        if effect != 0
            && word_of(e, EFFECT_SPELL, effect) == spell
            && (include_inactive || flag_of(e, EFFECT_IS_ACTIVE, effect))
        {
            let item = word_of(e, EFFECT_ITEM, effect);
            if !flag_of(e, EFFECT_ITEM_IS_USAGE_MONITOR, item) {
                found = true;
            }
        }
        node = node_next(e, node);
    }
    found
}

// Translated from 00822c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicTarget::IsLastEffect` (Xbox PDB): whether `effect` (of `spell`) is
/// the only active, non-usage-monitor effect of `spell` left on the target.
/// False at once for a usage-monitor effect.
pub fn magic_target_is_last_effect(
    e: &mut Engine,
    this: Ptr<MagicTarget>,
    spell: u32,
    effect: u32,
) -> bool {
    let item = word_of(e, EFFECT_ITEM, effect);
    if flag_of(e, EFFECT_ITEM_IS_USAGE_MONITOR, item) {
        return false;
    }
    let mut node = effect_list(e, this);
    let mut others = 0u32;
    while node != 0 {
        let other = node_item(e, node);
        if other != 0
            && word_of(e, EFFECT_SPELL, other) == spell
            && flag_of(e, EFFECT_IS_ACTIVE, other)
        {
            let other_item = word_of(e, EFFECT_ITEM, other);
            if !flag_of(e, EFFECT_ITEM_IS_USAGE_MONITOR, other_item) && other != effect {
                others = others.wrapping_add(1);
            }
        }
        node = node_next(e, node);
    }
    others == 0
}

// Translated from 00822cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether an effect whose effect setting has archetype `archetype` (active,
/// unless `include_inactive`) is on this target.
pub fn fn_00822cf0(
    e: &mut Engine,
    this: Ptr<MagicTarget>,
    archetype: u32,
    include_inactive: bool,
) -> bool {
    let mut node = effect_list(e, this);
    let mut found = false;
    while node != 0 && !found {
        let effect = node_item(e, node);
        if effect != 0 {
            let setting = word_of(e, EFFECT_SETTING_OF, effect);
            if word_of(e, EFFECT_SETTING_ARCHETYPE, setting) == archetype
                && (include_inactive || flag_of(e, EFFECT_IS_ACTIVE, effect))
            {
                found = true;
            }
        }
        node = node_next(e, node);
    }
    found
}

// Translated from 00822d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether an effect with effect setting `setting` is on this target.
pub fn fn_00822d80(e: &mut Engine, this: Ptr<MagicTarget>, setting: u32) -> bool {
    let mut node = effect_list(e, this);
    let mut found = false;
    while node != 0 && !node_is_empty(e, node) && !found {
        let effect = node_item(e, node);
        if effect != 0 && word_of(e, EFFECT_SETTING_OF, effect) == setting {
            found = true;
        }
        node = node_next(e, node);
    }
    found
}

// Translated from 00822e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether an effect is on this target whose effect setting passes
/// `00403c60` and has flag 4, and whose effect item's actor value is 0x10.
pub fn fn_00822e00(e: &mut Engine, this: Ptr<MagicTarget>) -> bool {
    let mut node = effect_list(e, this);
    let mut found = false;
    while node != 0 && !node_is_empty(e, node) && !found {
        let effect = node_item(e, node);
        if effect != 0 {
            let setting = word_of(e, EFFECT_SETTING_OF, effect);
            if flag_of(e, 0x0040_3c60, setting) {
                let item = word_of(e, EFFECT_ITEM, effect);
                if word_of(e, EFFECT_ITEM_ACTOR_VALUE, item) == 0x10
                    && effect_setting_has_flag(e, effect, 4)
                {
                    found = true;
                }
            }
        }
        node = node_next(e, node);
    }
    found
}

// Translated from 00822eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicTarget::GetUsageMonitorEffectMagnitude` (Xbox PDB): the summed
/// magnitude of the target's effects that are `spell`'s usage-monitor
/// effect (0 when `spell` is null, has none, or is not on the target).
pub fn magic_target_get_usage_monitor_effect_magnitude(
    e: &mut Engine,
    this: Ptr<MagicTarget>,
    spell: u32,
) -> f32 {
    let mut total = 0.0f32;
    let monitor = if spell == 0 {
        0
    } else {
        // `MagicItem` +0x0C: its `EffectItemList`.
        word_of(e, EFFECT_ITEM_LIST_USAGE_MONITOR, spell.wrapping_add(0x0C))
    };
    if monitor != 0 && magic_target_is_spell_target(e, this, spell, true) {
        let mut node = effect_list(e, this);
        while node != 0 && !node_is_empty(e, node) {
            let effect = node_item(e, node);
            if effect != 0 && word_of(e, EFFECT_ITEM, effect) == monitor {
                let magnitude = float_of(e, EFFECT_MAGNITUDE, effect);
                total = (magnitude as f64 + total as f64) as f32;
            }
            node = node_next(e, node);
        }
    }
    total
}

// Translated from 00822f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// For an actor target and an actor value other than -1 and 0x4D: whether
/// one of its effects on actor value `actor_value` has effect-setting flag
/// 2. An effect of spell type 4 without flag 0x80 is skipped when the actor's
/// virtual +0x360 says yes.
pub fn fn_00822f70(e: &mut Engine, this: Ptr<MagicTarget>, actor_value: i32) -> bool {
    let mut found = false;
    let actor = magic_target_get_target_as_actor(e, this).addr();
    if actor == 0 || actor_value == -1 || actor_value == 0x4D {
        return false;
    }
    let mut node = effect_list(e, this);
    while node != 0 && !found && !node_is_empty(e, node) {
        let effect = node_item(e, node);
        if effect != 0 {
            let item = word_of(e, EFFECT_ITEM, effect);
            if word_of(e, EFFECT_ITEM_ACTOR_VALUE, item) == actor_value as u32 {
                let skippable = word_of(e, EFFECT_SPELL_TYPE, effect) == 4
                    && !flag_of(e, EFFECT_FLAG_80, effect);
                let skipped = e.vcall(actor, 0x360, &[]).bool()
                    && skippable
                    && effect_setting_has_flag(e, effect, 2);
                if !skipped && effect_setting_has_flag(e, effect, 2) {
                    found = true;
                }
            }
        }
        node = node_next(e, node);
    }
    found
}

// Translated from 008230f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicTarget::AddTarget` (Xbox PDB): reflects or absorbs `effect` when
/// the target's checks say so (then reports it through `EffectReflected` /
/// `EffectAbsorbed` and returns false); otherwise `CheckAddEffect`. The
/// checks are skipped when the effect item's +0x0C is null, and when the
/// effect setting's flag 0x40 test and the spell's virtual +0x1C say so, or
/// the spell's virtual +0x20 does. The fourth word is never read.
pub fn magic_target_add_target(
    e: &mut Engine,
    this: Ptr<MagicTarget>,
    caster: u32,
    spell: u32,
    effect: u32,
    _unused_4: u32,
) -> bool {
    let item = word_of(e, EFFECT_ITEM, effect);
    if word_of(e, EFFECT_ITEM, item) != 0 {
        let item = word_of(e, EFFECT_ITEM, effect);
        let skip = fn_008231f0(e, Ptr::new(item)) && e.vcall(spell, MAGIC_ITEM_SLOT_1C, &[]).bool();
        if !skip && !e.vcall(spell, MAGIC_ITEM_SLOT_20, &[]).bool() {
            if e.vcall(
                this.addr(),
                SLOT_CHECK_REFLECT,
                &args![caster, spell, effect],
            )
            .bool()
            {
                e.vcall(this.addr(), SLOT_EFFECT_REFLECTED, &args![caster, effect]);
                return false;
            }
            if e.vcall(
                this.addr(),
                SLOT_CHECK_ABSORB,
                &args![caster, spell, effect, 0u32],
            )
            .bool()
            {
                e.vcall(this.addr(), SLOT_EFFECT_ABSORBED, &args![caster, effect]);
                return false;
            }
        }
    }
    magic_target_check_add_effect(e, this, caster, spell, effect)
}

// Translated from 008231f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An `EffectItem`'s effect setting has flag 0x40 (`007df1b0` on
/// `pEffectSetting`, +0x14).
pub fn fn_008231f0(e: &mut Engine, this: Ptr) -> bool {
    let setting = e.mem.u32(this.addr() + 0x14);
    e.call(0x007d_f1b0, &args![setting]).bool()
}

/// The resistance `CheckAddEffect` applies: 1.0 for spell type 4 and for
/// spells whose effect setting has flag 0x40 and whose virtual +0x1C says
/// yes, else the target's `CheckResistance`.
fn effect_resistance(
    e: &mut Engine,
    this: Ptr<MagicTarget>,
    caster: u32,
    spell: u32,
    effect: u32,
) -> f32 {
    let item = word_of(e, EFFECT_ITEM, effect);
    let ignores = fn_008231f0(e, Ptr::new(item)) && e.vcall(spell, MAGIC_ITEM_SLOT_1C, &[]).bool();
    let value = if spell_type(e, spell) == 4 || ignores {
        1.0
    } else {
        e.vcall(
            this.addr(),
            SLOT_CHECK_RESISTANCE,
            &args![caster, spell, effect],
        )
        .f32()
    };
    x87_quiet(value)
}

/// Writes one of `CheckAddEffect`'s "target / spell / effect" debug lines
/// (with `extra` words after the effect name).
fn log_target_spell_effect(
    e: &mut Engine,
    this: Ptr<MagicTarget>,
    format: u32,
    spell: u32,
    effect: u32,
    extra: &[u32],
) {
    let target = target_name(e, this);
    e.with_stack(BS_STRING_SIZE, |e, temp| {
        let item = word_of(e, EFFECT_ITEM, effect);
        let name = e
            .call(EFFECT_ITEM_GET_EFFECT_NAME, &args![item, temp])
            .u32();
        let effect_name = word_of(e, BS_STRING_C_STR, name);
        let spell_name = word_of(e, MAGIC_ITEM_NAME, spell);
        let mut words = args![format, target, spell_name, effect_name];
        words.extend_from_slice(extra);
        e.call(DEBUG_LOG, &words);
        e.call(BS_STRING_DESTROY, &args![temp]);
    });
}

/// `CheckAddEffect`'s entry-point adjustment of an actor target's effect
/// duration: `HandleEntryPoint(entry_point, actor, &duration)`, then the
/// effect's `00693d50` with the result.
fn adjust_duration_by_entry_point(e: &mut Engine, actor: u32, effect: u32, entry_point: u32) {
    let duration = float_of(e, EFFECT_DURATION, effect);
    let adjusted_duration = e.with_stack(4, |e, slot| {
        e.mem.set_f32(slot.addr(), duration);
        e.call(HANDLE_ENTRY_POINT, &args![entry_point, actor, slot]);
        e.mem.u32(slot.addr())
    });
    e.call(
        0x0069_3d50,
        &args![effect, x87_quiet_bits(adjusted_duration)],
    );
}

// Translated from 00823210 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00823210 - C++ exception frame (FS:[0]) unwinding of the debug-log strings not translated
/// `MagicTarget::CheckAddEffect` (Xbox PDB): applies resistance to `effect`
/// of `spell` from `caster` and, unless it is fully resisted, invalid for the
/// target or merged into an existing effect, adds a copy of it to the
/// target's active effects. Returns whether an effect was added or merged.
///
/// In order: flags the target's script (`Script::SetActionFlag` with 0x2000);
/// resistance (above); a resistance `<= 0.0` logs "FULLY RESISTED" and fails;
/// a hostile effect on an invulnerable target fails; the effect's
/// `CheckTarget` failing logs "Invalid Target" and fails. A usage-monitor
/// effect is merged into the first existing one (magnitudes added,
/// `004f5d90` with 0) unless a withdrawal (`00825610`) is running. Spell
/// type 10 and non-hostile alchemy items adjust the duration of actor
/// targets through entry points 0x18 / 0x19. Archetypes 0x22 and 0x23 store
/// `007e06b0` in their value-and-conditions / limb-condition effect. Then
/// the existing effects of the spell are dispelled (or, for effects with
/// flag 0x100, those whose items `0040dfa0` matches), or the effect is
/// merged by `00824c00` / `00824af0`; finally the effect's copy is
/// attached (`007037c0`), scaled by the resistance (not for ingredients),
/// inserted in order (`ActiveEffectListCompareFn`), reported through
/// `EffectAdded` and finalized.
pub fn magic_target_check_add_effect(
    e: &mut Engine,
    this: Ptr<MagicTarget>,
    caster: u32,
    spell: u32,
    effect: u32,
) -> bool {
    let stats = e
        .vcall(this.addr(), SLOT_GET_TARGET_STATS_OBJECT, &[])
        .u32();
    if stats != 0 {
        let script_target = e.call(0x005d_43c0, &args![stats]).u32();
        let item = word_of(e, EFFECT_ITEM, effect);
        let setting = word_of(e, EFFECT_ITEM_SETTING, item);
        e.call(0x005a_c750, &args![setting, script_target, 0x2000u32]);
    }

    let resistance = effect_resistance(e, this, caster, spell, effect);
    let zero: f64 = e.global(ZERO);
    if resistance as f64 <= zero {
        // The code compares the caster with the player's `MagicCaster` and
        // computes the player's `MagicTarget`, then uses neither.
        let _player = e.global::<u32>(PLAYER);
        if debug_log_enabled(e) {
            let resisted = f64::from(resistance);
            let mut extra = Vec::new();
            resisted.put(&mut extra);
            log_target_spell_effect(e, this, FULLY_RESISTED_FORMAT, spell, effect, &extra);
        }
        return false;
    }

    if e.vcall(this.addr(), SLOT_IS_INVULNERABLE, &[]).bool() {
        let item = word_of(e, EFFECT_ITEM, effect);
        if flag_of(e, EFFECT_ITEM_IS_HOSTILE, item) {
            return false;
        }
    }
    if !e
        .vcall(effect, ACTIVE_EFFECT_SLOT_CHECK_TARGET, &args![this])
        .bool()
    {
        if debug_log_enabled(e) {
            log_target_spell_effect(e, this, INVALID_TARGET_FORMAT, spell, effect, &[]);
        }
        return false;
    }

    let monitor = word_of(e, EFFECT_ITEM_LIST_USAGE_MONITOR, spell.wrapping_add(0x0C));
    if word_of(e, EFFECT_ITEM, effect) == monitor {
        if fn_00825610(e, this, spell, true) {
            return false;
        }
        let mut node = effect_list(e, this);
        let mut merged = false;
        while node != 0 && !node_is_empty(e, node) && !merged {
            let existing = node_item(e, node);
            if existing != 0 && word_of(e, EFFECT_ITEM, existing) == monitor {
                let old = float_of(e, EFFECT_MAGNITUDE, existing);
                let new = float_of(e, EFFECT_MAGNITUDE, effect);
                let sum = (new as f64 + old as f64) as f32;
                e.call(0x0080_4cb0, &args![existing, sum]);
                e.call(0x004f_5d90, &args![existing, 0.0f32]);
                merged = true;
            }
            node = node_next(e, node);
        }
        if merged {
            return true;
        }
    }

    if spell_type(e, spell) == 10 {
        let actor = magic_target_get_target_as_actor(e, this).addr();
        if actor != 0 {
            adjust_duration_by_entry_point(e, actor, effect, 0x18);
        }
    } else if spell_type(e, spell) == 7
        && dynamic_cast(e, spell, MAGIC_ITEM_TYPE, ALCHEMY_ITEM_TYPE) != 0
    {
        let item = word_of(e, EFFECT_ITEM, effect);
        if !flag_of(e, EFFECT_ITEM_IS_HOSTILE, item) {
            let actor = magic_target_get_target_as_actor(e, this).addr();
            if actor != 0 {
                adjust_duration_by_entry_point(e, actor, effect, 0x19);
            }
        }
    }

    let setting = word_of(e, EFFECT_SETTING_OF, effect);
    if word_of(e, EFFECT_SETTING_ARCHETYPE, setting) == 0x22 {
        let cast = dynamic_cast(
            e,
            effect,
            ACTIVE_EFFECT_TYPE,
            VALUE_AND_CONDITIONS_EFFECT_TYPE,
        );
        if cast != 0 {
            let value = e.call(0x007e_06b0, &[]).u32();
            e.call(0x0058_9f50, &args![cast, value]);
        }
    } else {
        let setting = word_of(e, EFFECT_SETTING_OF, effect);
        if word_of(e, EFFECT_SETTING_ARCHETYPE, setting) == 0x23 {
            let cast = dynamic_cast(e, effect, ACTIVE_EFFECT_TYPE, LIMB_CONDITION_EFFECT_TYPE);
            if cast != 0 {
                let value = e.call(0x007e_06b0, &[]).u32();
                e.call(0x0058_9f50, &args![cast, value]);
            }
        }
    }

    let mut dispel_spell = true;
    let mut merge_positive_duration = false;
    let mut merge_same_source = false;
    if e.call(0x0080_4b60, &args![effect]).bool() {
        dispel_spell = false;
        // The flag test here calls `0058cba0` directly.
        let setting = word_of(e, EFFECT_SETTING_OF, effect);
        if e.call(0x0058_cba0, &args![setting, 0x30000u32]).bool() {
            let mut node = effect_list(e, this);
            while node != 0 && !node_is_empty(e, node) {
                let existing = node_item(e, node);
                if existing != 0 {
                    let existing_item = word_of(e, EFFECT_ITEM, existing);
                    let item = word_of(e, EFFECT_ITEM, effect);
                    if e.call(0x0040_dfa0, &args![item, existing_item]).bool() {
                        let existing_spell = word_of(e, EFFECT_SPELL, existing);
                        magic_target_dispel(e, this, existing_spell, 0, existing);
                        dispel_spell = false;
                    }
                }
                node = node_next(e, node);
            }
        }
    } else {
        match spell_type(e, spell) {
            5 | 7 => {
                let duration = float_of(e, EFFECT_DURATION, effect);
                merge_positive_duration = duration as f64 > zero;
                dispel_spell = false;
            }
            6 => {
                // The spell seen from its base at -0x18.
                let base = adjusted(spell, 0u32.wrapping_sub(0x18));
                if fn_00823c20(e, Ptr::new(base)) {
                    dispel_spell = false;
                    merge_same_source = true;
                }
                let source = word_of(e, EFFECT_SOURCE, effect);
                if source != 0 && e.call(0x0040_1170, &args![source]).u32() == 0x19 {
                    dispel_spell = false;
                }
            }
            8 => dispel_spell = false,
            _ => {}
        }
    }

    if dispel_spell {
        magic_target_dispel(e, this, spell, caster, effect);
    } else if merge_positive_duration {
        if fn_00824c00(e, this, spell, caster, effect) {
            return true;
        }
    } else if merge_same_source && fn_00824af0(e, this, spell, caster, effect) {
        return true;
    }

    let copy = e.vcall(effect, ACTIVE_EFFECT_SLOT_COPY, &[]).u32();
    if copy == 0 {
        return false;
    }
    e.call(0x0070_37c0, &args![copy, this]);
    if dynamic_cast(e, spell, MAGIC_ITEM_TYPE, INGREDIENT_ITEM_TYPE) == 0 {
        e.call(0x0080_4bc0, &args![copy, x87_quiet(resistance)]);
    }
    if effect_list(e, this) == 0 {
        if copy != 0 {
            e.vcall(copy, ACTIVE_EFFECT_SLOT_DELETE, &args![1u32]);
        }
        return false;
    }
    let list = effect_list(e, this);
    e.call(0x007a_7eb0, &args![list, copy, 0x0082_5270u32]);
    e.vcall(this.addr(), SLOT_EFFECT_ADDED, &args![copy]);
    e.vcall(copy, ACTIVE_EFFECT_SLOT_FINALIZE, &[]);
    if debug_log_enabled(e) {
        let target = stats_name(e, stats);
        let copy_item = word_of(e, EFFECT_ITEM, copy);
        let duration_value = e.call(0x0040_44d0, &args![copy_item]).u32();
        let duration = f64::from(x87_quiet(float_of(e, EFFECT_DURATION, copy)));
        let copy_item = word_of(e, EFFECT_ITEM, copy);
        let magnitude_value = e.call(0x0040_4390, &args![copy_item]).u32();
        let magnitude = f64::from(x87_quiet(float_of(e, EFFECT_MAGNITUDE, copy)));
        let spell_name = word_of(e, MAGIC_ITEM_NAME, spell);
        e.with_stack(BS_STRING_SIZE, |e, temp| {
            let copy_item = word_of(e, EFFECT_ITEM, copy);
            let name = e
                .call(EFFECT_ITEM_GET_EFFECT_NAME, &args![copy_item, temp])
                .u32();
            let effect_name = word_of(e, BS_STRING_C_STR, name);
            e.call(
                DEBUG_LOG,
                &args![
                    EFFECT_ADDED_FORMAT,
                    effect_name,
                    spell_name,
                    target,
                    magnitude,
                    magnitude_value,
                    duration,
                    duration_value
                ],
            );
            e.call(BS_STRING_DESTROY, &args![temp]);
        });
    }
    true
}

/// Whether the debug log is on.
fn debug_log_enabled(e: &mut Engine) -> bool {
    e.call(DEBUG_LOG_ENABLED, &[]).bool()
}

// Translated from 00823c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `0040e210(this, 2)`: the spell-item test `CheckAddEffect` uses for spell
/// type 6 (its result in `AL`).
pub fn fn_00823c20(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x0040_e210, &args![this, 2u32]).bool()
}

// Translated from 00823c40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00823c40 - /GS stack cookie check (00ec408c) not translated
/// `MagicTarget::UpdateTarget` (Xbox PDB): advances every active effect by
/// `seconds` and removes the finished ones. Actor targets without
/// `MiddleHighProcess::GetSavedAcquireObject` (`008d8520`, map name of a
/// folded body) are skipped. While it runs `bUpdating` is set, so dispels
/// are queued; the queue is run at the end.
///
/// Effects already flagged for removal are not updated; effects of spell
/// types 1, 4 and 10 (`00804b30`) with setting flag 2 that have started are
/// updated only with flag 0x80 or a displacement spell. When a done effect
/// is the player's last effect of an alchemy item (spell type 7), it shows
/// the "wear off" HUD message (the withdrawal text when a withdrawal effect
/// runs and the player's +0xA4 virtual +0x08 with 0x48 says no; nothing when
/// it says yes), plays `UIHealthChemsWearOff`, and for an alchemy item with
/// a withdrawal effect (`004030b0`) triggers the image space modifier
/// default object 0x14.
pub fn magic_target_update_target(e: &mut Engine, this: Ptr<MagicTarget>, seconds: f32) {
    let head = effect_list(e, this);
    let mut node = head;
    if e.vcall(this.addr(), SLOT_MAGIC_TARGET_IS_ACTOR, &[]).bool() {
        let _stats = e
            .vcall(this.addr(), SLOT_GET_TARGET_STATS_OBJECT, &[])
            .u32();
        let actor = actor_of(this.addr());
        if actor == 0 || e.call(0x008d_8520, &args![actor]).u32() == 0 {
            return;
        }
    }
    e.set(this, MagicTarget::bUpdating, true);
    while node != 0 && !node_is_empty(e, node) {
        let mut next = node_next(e, node);
        let effect = node_item(e, node);
        if effect != 0 {
            let flagged = fn_008240f0(e, Ptr::new(effect));
            let mut update = !flagged;
            if update {
                let kind = word_of(e, EFFECT_SPELL_TYPE, effect);
                if e.call(0x0080_4b30, &args![kind]).bool()
                    && effect_setting_has_flag(e, effect, 2)
                    && flag_of(e, EFFECT_HAS_STARTED, effect)
                {
                    update = flag_of(e, EFFECT_FLAG_80, effect)
                        || word_of(e, EFFECT_DISPLACEMENT_SPELL, effect) != 0;
                }
            }
            if update {
                e.call(0x0080_4560, &args![effect, x87_quiet(seconds)]);
            }
            if !flagged && flag_of(e, EFFECT_IS_DONE, effect) {
                let spell = word_of(e, EFFECT_SPELL, effect);
                let target = word_of(e, EFFECT_TARGET, effect);
                fn_008240a0(e, Ptr::new(effect), true);
                if flag_of(e, EFFECT_IS_ACTIVE, effect) && !fn_00824080(e, Ptr::new(effect)) {
                    let player = e.global::<u32>(PLAYER);
                    let player_target = adjusted(player, ACTOR_MAGIC_TARGET);
                    if target == player_target
                        && spell_type(e, spell) == 7
                        && magic_target_is_last_effect(e, Ptr::new(target), spell, effect)
                    {
                        show_wear_off(e, target, spell);
                    }
                }
            }
            if fn_008240f0(e, Ptr::new(effect)) {
                if node == head {
                    next = node;
                }
                magic_target_remove_effect(e, this, effect, true);
            }
        }
        node = next;
    }
    e.set(this, MagicTarget::bUpdating, false);
    magic_target_dispel_queued_spells(e, this);
}

/// The "wear off" message of `UpdateTarget` for the player's last effect of
/// `spell`.
fn show_wear_off(e: &mut Engine, target: u32, spell: u32) {
    e.with_stack(TEXT_BUFFER, |e, buffer| {
        let mut show = true;
        if fn_00825610(e, Ptr::new(target), spell, true) {
            let player = e.global::<u32>(PLAYER);
            let base = player.wrapping_add(0xA4);
            if e.vcall(base, 0x08, &args![0x48u32]).u32() == 0 {
                format_hud_message(e, buffer, WEAR_OFF_WITHDRAWAL_MESSAGE, spell);
            } else {
                show = false;
            }
        } else {
            format_hud_message(e, buffer, WEAR_OFF_MESSAGE, spell);
        }
        if show {
            let time = hud_message_time(e);
            e.call(
                SHOW_HUD_MESSAGE,
                &args![buffer, 0u32, 0u32, CHEMS_WEAR_OFF_SOUND, time, 0u32],
            );
            let alchemy = dynamic_cast(e, spell, MAGIC_ITEM_TYPE, ALCHEMY_ITEM_TYPE);
            if alchemy != 0 && e.call(0x0040_30b0, &args![alchemy]).u32() != 0 {
                let form = e.call(GET_DEFAULT_OBJECT, &args![0x14u32]).u32();
                let modifier = dynamic_cast(e, form, TES_FORM_TYPE, TES_IMAGE_SPACE_MODIFIER_TYPE);
                if modifier != 0 {
                    e.call(
                        IMAGE_SPACE_MODIFIER_TRIGGER_IF_NOT_ACTIVE,
                        &args![modifier, ONE_F32, 0u32],
                    );
                }
            }
        }
    });
}

// Translated from 00824080 (decompiled, FalloutNV.exe 1.4.0.525)
/// An `ActiveEffect`'s `iFlags` (+0x18) bit 0x200.
pub fn fn_00824080(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x18) & 0x200 != 0
}

// Translated from 008240a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears an `ActiveEffect`'s `iFlags` (+0x18) bit 0x80000000 (the
/// "remove after the update" mark).
pub fn fn_008240a0(e: &mut Engine, this: Ptr, set: bool) {
    let flags = e.mem.u32(this.addr() + 0x18);
    let flags = if set {
        flags | 0x8000_0000
    } else {
        flags & 0x7fff_ffff
    };
    e.mem.set_u32(this.addr() + 0x18, flags);
}

// Translated from 008240f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An `ActiveEffect`'s `iFlags` (+0x18) bit 0x80000000.
pub fn fn_008240f0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x18) & 0x8000_0000 != 0
}

// Translated from 00824290 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00824290 - C++ exception frame (FS:[0]) unwinding of the allocation not translated
/// Queues a dispel for after the update: allocates a `SpellDispelData`
/// (`00401000`, 12 bytes; `00824340` zeroes it), fills it and adds it to
/// `PostUpdateDispelList` (`005ae3d0` with the pointer's address). A failed
/// allocation writes through null, as the game does.
pub fn fn_00824290(e: &mut Engine, this: Ptr<MagicTarget>, spell: u32, caster: u32, effect: u32) {
    let block = e.call(0x0040_1000, &args![0x0Cu32]).u32();
    let data = if block == 0 {
        0
    } else {
        e.call(0x0082_4340, &args![block]).u32()
    };
    let data: Ptr<SpellDispelData> = Ptr::new(data);
    e.set(data, SpellDispelData::pSpell, spell);
    e.set(data, SpellDispelData::pCaster, caster);
    e.set(data, SpellDispelData::pActiveEffect, effect);
    let list = this.at(MagicTarget::PostUpdateDispelList);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), data.addr());
        e.call(0x005a_e3d0, &args![list, slot]);
    });
}

// Translated from 00824370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicTarget::DispelQueuedSpells` (Xbox PDB): runs `Dispel` for every
/// queued `SpellDispelData`, frees each (`00401030`), then clears the list
/// (`00470470`).
pub fn magic_target_dispel_queued_spells(e: &mut Engine, this: Ptr<MagicTarget>) {
    let list = this.at(MagicTarget::PostUpdateDispelList).addr();
    let mut node = list;
    while node != 0 && !node_is_empty(e, node) {
        let data: Ptr<SpellDispelData> = Ptr::new(node_item(e, node));
        if !data.is_null() {
            let spell = e.get(data, SpellDispelData::pSpell);
            let caster = e.get(data, SpellDispelData::pCaster);
            let effect = e.get(data, SpellDispelData::pActiveEffect);
            magic_target_dispel(e, this, spell, caster, effect);
            e.call(0x0040_1030, &args![data]);
        }
        node = node_next(e, node);
    }
    e.call(0x0047_0470, &args![list]);
}

/// Dispels `effect`, or queues the dispel when the target is updating.
fn dispel_or_queue(e: &mut Engine, this: Ptr<MagicTarget>, spell: u32, caster: u32, effect: u32) {
    if e.get(this, MagicTarget::bUpdating) {
        fn_00824290(e, this, spell, caster, effect);
    } else {
        dispel_effect(e, effect, true);
    }
}

// Translated from 00824400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicTarget::Dispel` (Xbox PDB): dispels the effects of `spell` (from
/// `caster` when not null) that are not done. With `effect` given, only the
/// first one from the same source whose effect item `004042a0` does not
/// match `effect`'s, then stops. Dispels are queued while updating.
pub fn magic_target_dispel(
    e: &mut Engine,
    this: Ptr<MagicTarget>,
    spell: u32,
    caster: u32,
    effect: u32,
) {
    let mut node = effect_list(e, this);
    while node != 0 && !node_is_empty(e, node) {
        let existing = node_item(e, node);
        let next = node_next(e, node);
        if existing != 0
            && !flag_of(e, EFFECT_IS_DONE, existing)
            && word_of(e, EFFECT_SPELL, existing) == spell
        {
            let same_caster = caster == 0 || word_of(e, EFFECT_CASTER, existing) == caster;
            if same_caster {
                let same_source = effect == 0
                    || word_of(e, EFFECT_SOURCE, effect) == word_of(e, EFFECT_SOURCE, existing);
                if same_source {
                    if effect == 0 {
                        dispel_or_queue(e, this, spell, caster, existing);
                    } else if !effect_items_match(e, existing, effect) {
                        dispel_or_queue(e, this, spell, caster, existing);
                        return;
                    }
                }
            }
        }
        node = next;
    }
}

// Translated from 00824570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicTarget::Dispel_ov2` (Xbox PDB): dispels the effects of `spell`
/// (from `caster` when not null) with effect item `item` that are not done.
pub fn magic_target_dispel_ov2(
    e: &mut Engine,
    this: Ptr<MagicTarget>,
    item: u32,
    spell: u32,
    caster: u32,
) {
    let mut node = effect_list(e, this);
    while node != 0 && !node_is_empty(e, node) {
        let effect = node_item(e, node);
        let next = node_next(e, node);
        if effect != 0
            && !flag_of(e, EFFECT_IS_DONE, effect)
            && word_of(e, EFFECT_SPELL, effect) == spell
        {
            let same_caster = caster == 0 || word_of(e, EFFECT_CASTER, effect) == caster;
            if same_caster && word_of(e, EFFECT_ITEM, effect) == item {
                dispel_effect(e, effect, true);
            }
        }
        node = next;
    }
}

// Translated from 00824630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dispels by spell type. For type 1, on an actor's stats object: as long
/// as an entry of the list `00717e50` gives (at `004181e0(actor) + 0x7C`)
/// has an object at +0x18 whose virtual +0x18 returns 1, hands it to the
/// actor's virtual +0x3E4 and starts over. For any other type, dispels
/// (without removal) the effects that are not done whose spell has that type,
/// or, for type 5, whose spell is an alchemy item that can be poison.
pub fn fn_00824630(e: &mut Engine, this: Ptr<MagicTarget>, kind: u32) {
    if kind == 1 {
        let stats = e
            .vcall(this.addr(), SLOT_GET_TARGET_STATS_OBJECT, &[])
            .u32();
        let actor = dynamic_cast(e, stats, TES_OBJECT_REFR_TYPE, ACTOR_TYPE);
        if actor == 0 {
            return;
        }
        loop {
            let owner = e.call(0x0041_81e0, &args![actor]).u32();
            let mut node = e.call(0x0071_7e50, &args![owner.wrapping_add(0x7C)]).u32();
            let mut handed = false;
            while node != 0 && !node_is_empty(e, node) {
                let next = node_next(e, node);
                let entry = node_item(e, node);
                if entry != 0 && e.vcall(entry.wrapping_add(0x18), 0x18, &[]).u32() == 1 {
                    e.vcall(actor, 0x3E4, &args![entry]);
                    handed = true;
                    break;
                }
                node = next;
            }
            if !handed {
                break;
            }
        }
    } else {
        let mut node = effect_list(e, this);
        while node != 0 && !node_is_empty(e, node) {
            let effect = node_item(e, node);
            let next = node_next(e, node);
            if effect != 0 && !flag_of(e, EFFECT_IS_DONE, effect) {
                let spell = word_of(e, EFFECT_SPELL, effect);
                let alchemy = dynamic_cast(e, spell, MAGIC_ITEM_TYPE, ALCHEMY_ITEM_TYPE);
                let poison = if alchemy == 0 {
                    false
                } else {
                    // `AlchemyItem` +0x3C: its `EffectItemList`.
                    e.call(0x0040_5da0, &args![alchemy.wrapping_add(0x3C)])
                        .bool()
                };
                let spell = word_of(e, EFFECT_SPELL, effect);
                if spell_type(e, spell) == kind || (poison && kind == 5) {
                    dispel_effect(e, effect, false);
                }
            }
            node = next;
        }
    }
}

// Translated from 00824800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicTarget::Dispel_ov3` (Xbox PDB): dispels and removes (without
/// deleting) every effect not done whose effect setting has archetype
/// `archetype` (and whose caster is `caster` when not null). After a removal
/// it looks at the same node again.
pub fn magic_target_dispel_ov3(
    e: &mut Engine,
    this: Ptr<MagicTarget>,
    archetype: u32,
    caster: u32,
) {
    let mut node = effect_list(e, this);
    while node != 0 && !node_is_empty(e, node) {
        let effect = node_item(e, node);
        let same_caster = caster == 0 || caster == word_of(e, EFFECT_CASTER, effect);
        if effect != 0 && !flag_of(e, EFFECT_IS_DONE, effect) {
            let setting = word_of(e, EFFECT_SETTING_OF, effect);
            if word_of(e, EFFECT_SETTING_ARCHETYPE, setting) == archetype && same_caster {
                dispel_effect(e, effect, true);
                magic_target_remove_effect(e, this, effect, false);
                continue;
            }
        }
        node = node_next(e, node);
    }
}

// Translated from 008248e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dispels every effect not done whose source is `source` (nothing for a
/// null source), passing `remove` on.
pub fn fn_008248e0(e: &mut Engine, this: Ptr<MagicTarget>, source: u32, remove: bool) {
    if source == 0 {
        return;
    }
    let mut node = effect_list(e, this);
    while node != 0 && !node_is_empty(e, node) {
        let effect = node_item(e, node);
        node = node_next(e, node);
        if effect != 0
            && !flag_of(e, EFFECT_IS_DONE, effect)
            && word_of(e, EFFECT_SOURCE, effect) == source
        {
            dispel_effect(e, effect, remove);
        }
    }
}

// Translated from 00824970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicTarget::Dispel_ov4` (Xbox PDB): dispels every effect.
pub fn magic_target_dispel_ov4(e: &mut Engine, this: Ptr<MagicTarget>) {
    let mut node = effect_list(e, this);
    while node != 0 && !node_is_empty(e, node) {
        let effect = node_item(e, node);
        node = node_next(e, node);
        if effect != 0 {
            dispel_effect(e, effect, true);
        }
    }
}

// Translated from 008249d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dispels (passing `remove` on) every effect whose spell type is 0, 2, 3, 7
/// or 8, or whose spell is an enchantment item whose +0x34 word is 0.
pub fn fn_008249d0(e: &mut Engine, this: Ptr<MagicTarget>, remove: bool) {
    let mut node = effect_list(e, this);
    while node != 0 && !node_is_empty(e, node) {
        let effect = node_item(e, node);
        node = node_next(e, node);
        if effect == 0 {
            continue;
        }
        let spell = word_of(e, EFFECT_SPELL, effect);
        let listed = spell_type(e, spell) == 0
            || spell_type(e, spell) == 2
            || spell_type(e, spell) == 3
            || spell_type(e, spell) == 7
            || spell_type(e, spell) == 8;
        if listed {
            dispel_effect(e, effect, remove);
        } else {
            let enchantment = dynamic_cast(e, spell, MAGIC_ITEM_TYPE, ENCHANTMENT_ITEM_TYPE);
            // `005f36f0`: the word at +0x34 (map name of a folded body:
            // `ActorMover::GetPreferredMoveMode`).
            if enchantment != 0 && e.call(0x005f_36f0, &args![enchantment]).u32() == 0 {
                dispel_effect(e, effect, remove);
            }
        }
    }
}

// Translated from 00824af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the first effect not done of `spell` (from `caster` when not null)
/// whose effect item `004042a0` does not match `effect`'s (any, for a null
/// `effect`), and resets it (`004f5d90` with 0.0). Returns whether it found
/// one.
pub fn fn_00824af0(
    e: &mut Engine,
    this: Ptr<MagicTarget>,
    spell: u32,
    caster: u32,
    effect: u32,
) -> bool {
    let mut found = false;
    let mut node = effect_list(e, this);
    while !found && node != 0 && !node_is_empty(e, node) {
        let existing = node_item(e, node);
        let next = node_next(e, node);
        if existing != 0
            && !flag_of(e, EFFECT_IS_DONE, existing)
            && word_of(e, EFFECT_SPELL, existing) == spell
            && (caster == 0 || word_of(e, EFFECT_CASTER, existing) == caster)
            && (effect == 0 || !effect_items_match(e, existing, effect))
        {
            e.call(0x004f_5d90, &args![existing, 0.0f32]);
            found = true;
        }
        node = next;
    }
    found
}

// Translated from 00824c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the first effect not done of `spell` (from `caster` when not null)
/// whose effect item `004042a0` does not match `effect`'s, and extends it:
/// `00693d50` with its duration plus `effect`'s. For an alchemy item whose
/// list word at +0xA0 (`00726070` on +0x9C) is 0x0D, every effect counts as
/// one of `spell`. Returns whether it extended one.
pub fn fn_00824c00(
    e: &mut Engine,
    this: Ptr<MagicTarget>,
    spell: u32,
    caster: u32,
    effect: u32,
) -> bool {
    let mut found = false;
    let mut node = effect_list(e, this);
    let mut any_effect = false;
    if spell_type(e, spell) == 7 {
        let alchemy = dynamic_cast(e, spell, MAGIC_ITEM_TYPE, ALCHEMY_ITEM_TYPE);
        if alchemy != 0 && node_next(e, alchemy.wrapping_add(0x9C)) == 0x0D {
            any_effect = true;
        }
    }
    while !found && node != 0 {
        if node_is_empty(e, node) {
            return found;
        }
        let existing = node_item(e, node);
        let next = node_next(e, node);
        let mut matches = existing != 0 && word_of(e, EFFECT_SPELL, existing) == spell;
        if any_effect {
            let alchemy = dynamic_cast(e, spell, MAGIC_ITEM_TYPE, ALCHEMY_ITEM_TYPE);
            if alchemy != 0 {
                matches = node_next(e, alchemy.wrapping_add(0x9C)) == 0x0D;
            }
        }
        if existing != 0
            && !flag_of(e, EFFECT_IS_DONE, existing)
            && matches
            && (caster == 0 || word_of(e, EFFECT_CASTER, existing) == caster)
            && (effect == 0 || !effect_items_match(e, existing, effect))
        {
            let existing_duration = float_of(e, EFFECT_DURATION, existing);
            let added = float_of(e, EFFECT_DURATION, effect);
            let total = (added as f64 + existing_duration as f64) as f32;
            e.call(0x0069_3d50, &args![existing, x87_quiet(total)]);
            found = true;
        }
        node = next;
    }
    found
}

// Translated from 00824df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// For every effect of `spell`: `008d8040(effect, value)` (map name of a
/// folded body: `LowProcess::SetGenericLocation`), then `008043f0` with 0.0
/// and true.
pub fn fn_00824df0(e: &mut Engine, this: Ptr<MagicTarget>, value: u32, spell: u32) {
    let mut node = effect_list(e, this);
    while node != 0 && !node_is_empty(e, node) {
        let effect = node_item(e, node);
        node = node_next(e, node);
        if effect != 0 && word_of(e, EFFECT_SPELL, effect) == spell {
            e.call(0x008d_8040, &args![effect, value]);
            e.call(0x0080_43f0, &args![effect, 0.0f32, true]);
        }
    }
}

// Translated from 00824e70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00824e70 - /GS stack cookie check (00ec408c) not translated
/// `MagicTarget::CheckAddiction` (Xbox PDB): rolls whether the actor target
/// gets addicted to `item` (an alchemy item with a withdrawal effect,
/// `004030b0`, and an addiction chance, `00644a50`, other than 0.0).
///
/// With a usage-monitor effect, the chance is the usage magnitude above the
/// setting at `011d1878` divided by 100 (no addiction at or below it).
/// Entry point 0x17 adjusts the chance; a roll (`00487f50` mod 100) at or
/// above `chance * 100` calls `008255a0` and fails. Otherwise it dispels the
/// usage-monitor effects, casts the withdrawal effect from the actor's
/// `MagicCaster` (virtual +0x04 with the effect and 0), runs `00824df0`, and
/// when this is a new addiction for the player (actor virtual +0x360 says
/// yes and no withdrawal of `item` runs) counts it (`MiscStatManager` stat
/// 10), shows the "addicted" HUD message, plays `UIHealthChemsAddicted` and
/// triggers the image space modifier form 0x19482. Returns true only then.
pub fn magic_target_check_addiction(e: &mut Engine, this: Ptr<MagicTarget>, item: u32) -> bool {
    let chance = if item == 0 {
        0.0
    } else {
        float_of(e, 0x0064_4a50, item)
    };
    let chance = x87_quiet(chance);
    if item == 0 || e.call(0x0040_30b0, &args![item]).u32() == 0 {
        return false;
    }
    let zero: f64 = e.global(ZERO);
    if chance as f64 == zero {
        return false;
    }
    let actor = magic_target_get_target_as_actor(e, this).addr();
    if actor == 0 {
        return false;
    }
    let spell = adjusted(item, 0x30);
    // `AlchemyItem` +0x3C: its `EffectItemList`.
    let monitor = word_of(e, EFFECT_ITEM_LIST_USAGE_MONITOR, item.wrapping_add(0x3C));
    e.with_stack(4, |e, slot| {
        e.mem.set_f32(slot.addr(), chance);
        if monitor != 0 {
            let magnitude = x87_quiet(magic_target_get_usage_monitor_effect_magnitude(
                e, this, spell,
            ));
            let threshold_address = e
                .call(SETTING_FLOAT_ADDRESS, &args![ADDICTION_USAGE_THRESHOLD])
                .u32();
            let threshold = e.mem.f32(threshold_address);
            if threshold >= magnitude {
                return false;
            }
            let threshold_address = e
                .call(SETTING_FLOAT_ADDRESS, &args![ADDICTION_USAGE_THRESHOLD])
                .u32();
            let threshold = e.mem.f32(threshold_address);
            let hundred: f64 = e.global(HUNDRED);
            let excess = ((magnitude as f64 - threshold as f64) / hundred) as f32;
            e.mem.set_f32(slot.addr(), excess);
        }
        e.call(HANDLE_ENTRY_POINT, &args![0x17u32, actor, slot]);
        let roll = e.call(0x0048_7f50, &[]).u32() % 100;
        let hundred: f64 = e.global(HUNDRED);
        let threshold = e.mem.f32(slot.addr()) as f64 * hundred;
        // `FCOMPP` with the roll in ST0: unordered counts as below.
        if roll as i32 as f64 >= threshold {
            e.call(0x0082_55a0, &args![this]);
            return false;
        }
        true
    }) && addict(e, this, item, actor, monitor)
}

/// The second half of `CheckAddiction`, after a successful roll.
fn addict(e: &mut Engine, this: Ptr<MagicTarget>, item: u32, actor: u32, monitor: u32) -> bool {
    let new_addiction = e.vcall(actor, 0x360, &[]).bool()
        && !fn_00825610(
            e,
            Ptr::new(actor.wrapping_add(ACTOR_MAGIC_TARGET)),
            adjusted(item, 0x30),
            true,
        );
    if monitor != 0 {
        magic_target_dispel_ov2(e, this, monitor, adjusted(item, 0x30), 0);
    }
    let withdrawal = e.call(0x0040_30b0, &args![item]).u32();
    e.vcall(
        actor.wrapping_add(ACTOR_MAGIC_CASTER),
        0x04,
        &args![withdrawal, 0u32],
    );
    let withdrawal = e.call(0x0040_30b0, &args![item]).u32();
    let withdrawal_spell = adjusted(withdrawal, 0x18);
    fn_00824df0(e, this, adjusted(item, 0x30), withdrawal_spell);
    if !new_addiction {
        return false;
    }
    e.call(0x004d_5c60, &args![10u32]);
    let form = e.call(LOOKUP_FORM_BY_ID, &args![0x0001_9482u32]).u32();
    let modifier = dynamic_cast(e, form, TES_FORM_TYPE, TES_IMAGE_SPACE_MODIFIER_TYPE);
    e.with_stack(TEXT_BUFFER, |e, buffer| {
        format_hud_message(e, buffer, ADDICTED_MESSAGE, item.wrapping_add(0x30));
        let time = hud_message_time(e);
        e.call(
            SHOW_HUD_MESSAGE,
            &args![buffer, 1u32, 0u32, CHEMS_ADDICTED_SOUND, time, 0u32],
        );
    });
    e.call(
        IMAGE_SPACE_MODIFIER_TRIGGER,
        &args![modifier, ONE_F32, 0u32],
    );
    true
}

// Translated from 008251c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicTarget::RemoveEffect` (Xbox PDB): without `now`, only marks the
/// effect for removal after the update. With it, removes the effect from
/// the list (`00905330` with the address of the argument), reports
/// `EffectRemoved`, and deletes it: through the task queue
/// (`TaskQueueInterface::QueueDeleteActiveEffect` on the queue at
/// `011df1a8`) when the counter `0043d4d0` finds on `011c3ea4` is above 1
/// and the byte at `011dfa19` is set, else directly (virtual +0x00 with 1).
pub fn magic_target_remove_effect(e: &mut Engine, this: Ptr<MagicTarget>, effect: u32, now: bool) {
    if !now {
        fn_008240a0(e, Ptr::new(effect), true);
        return;
    }
    let effect = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), effect);
        let list = effect_list(e, this);
        e.call(0x0090_5330, &args![list, slot]);
        e.mem.u32(slot.addr())
    });
    e.vcall(this.addr(), SLOT_EFFECT_REMOVED, &args![effect]);
    let counter = e.call(0x0043_d4d0, &args![0x011c_3ea4u32]).u32();
    if e.mem.i32(counter) > 1 && e.call(0x0071_3d90, &[]).bool() {
        let queue = e.call(0x0045_37b0, &[]).u32();
        e.call(0x0087_b890, &args![queue, effect]);
    } else if effect != 0 {
        e.vcall(effect, ACTIVE_EFFECT_SLOT_DELETE, &args![1u32]);
    }
}

/// Formats an effect's sort key, `"%c%.30s%.1f%.1f"` with `'A'`, the effect
/// name, its magnitude and its duration (1000.0 for effects whose setting
/// has flag 0x100 / 0x80), into `buffer`.
fn format_sort_key(e: &mut Engine, effect: u32, buffer: Ptr) {
    let item = word_of(e, EFFECT_ITEM, effect);
    let setting = word_of(e, EFFECT_ITEM_SETTING, item);
    let duration = if e
        .call(EFFECT_SETTING_HAS_FLAG, &args![setting, 0x80u32])
        .bool()
    {
        e.global::<f32>(NO_VALUE_SORT_KEY)
    } else {
        float_of(e, EFFECT_DURATION, effect)
    };
    let setting = word_of(e, EFFECT_ITEM_SETTING, item);
    let magnitude = if e
        .call(EFFECT_SETTING_HAS_FLAG, &args![setting, 0x100u32])
        .bool()
    {
        e.global::<f32>(NO_VALUE_SORT_KEY)
    } else {
        float_of(e, EFFECT_MAGNITUDE, effect)
    };
    let duration = f64::from(x87_quiet(duration));
    let magnitude = f64::from(x87_quiet(magnitude));
    e.with_stack(BS_STRING_SIZE, |e, temp| {
        let name = e
            .call(EFFECT_ITEM_GET_EFFECT_NAME, &args![item, temp])
            .u32();
        let text = word_of(e, BS_STRING_C_STR, name);
        e.call(
            0x00ec_623a,
            &args![buffer, SORT_KEY_FORMAT, 0x41u32, text, magnitude, duration],
        );
        e.call(BS_STRING_DESTROY, &args![temp]);
    });
}

// Translated from 00825270 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00825270 - C++ exception frame (FS:[0]) unwinding of the name strings and /GS stack cookie check (00ec408c) not translated
/// `MagicTarget::ActiveEffectListCompareFn` (Xbox PDB), cdecl: orders two
/// effects by their sort keys (see `format_sort_key`), compared with
/// `0046fd30`.
pub fn magic_target_active_effect_list_compare_fn(e: &mut Engine, first: u32, second: u32) -> i32 {
    e.with_stack(TEXT_BUFFER, |e, first_key| {
        format_sort_key(e, first, first_key);
        e.with_stack(TEXT_BUFFER, |e, second_key| {
            format_sort_key(e, second, second_key);
            e.call(0x0046_fd30, &args![first_key, second_key]).i32()
        })
    })
}

// Translated from 008254c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicTarget::GetMagicTargetFormID` (Xbox PDB): the form id (+0x0C, read
/// by `0084e3a0`) of the actor, or of the object the target's virtual +0x04
/// returns (seen from `this - 0x0C`, null staying null).
pub fn magic_target_get_magic_target_form_id(e: &mut Engine, this: Ptr<MagicTarget>) -> u32 {
    if e.vcall(this.addr(), SLOT_MAGIC_TARGET_IS_ACTOR, &[]).bool() {
        let actor = actor_of(this.addr());
        word_of(e, FORM_ID_OF, actor)
    } else {
        let owner = adjusted(this.addr(), 0u32.wrapping_sub(0x0C));
        let stats = e
            .vcall(owner.wrapping_add(0x0C), SLOT_GET_TARGET_STATS_OBJECT, &[])
            .u32();
        word_of(e, FORM_ID_OF, stats)
    }
}

// Translated from 00825550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicTarget::GetMagicTargetByNumericID` (Xbox PDB), cdecl: the form with
/// id `form_id` (`004839c0`) seen as a `MagicTarget` (its virtual +0x19C)
/// when its virtual +0xF0 says yes, else null.
pub fn magic_target_get_magic_target_by_numeric_id(e: &mut Engine, form_id: u32) -> u32 {
    let form = e.call(LOOKUP_FORM_BY_ID, &args![form_id]).u32();
    if form != 0 && e.vcall(form, 0xF0, &[]).bool() {
        e.vcall(form, 0x19C, &[]).u32()
    } else {
        0
    }
}

// Translated from 00825610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether an effect of a spell of type 10 runs on the target (active,
/// unless `include_inactive`), with displacement spell `spell` when `spell`
/// is not null. `UpdateTarget` and `CheckAddEffect` use it to find a running
/// withdrawal.
pub fn fn_00825610(
    e: &mut Engine,
    this: Ptr<MagicTarget>,
    spell: u32,
    include_inactive: bool,
) -> bool {
    let mut node = effect_list(e, this);
    let mut found = false;
    while node != 0 && !found {
        let effect = node_item(e, node);
        let effect_spell = if effect == 0 {
            0
        } else {
            word_of(e, EFFECT_SPELL, effect)
        };
        if effect_spell != 0
            && spell_type(e, effect_spell) == 10
            && (spell == 0 || word_of(e, EFFECT_DISPLACEMENT_SPELL, effect) == spell)
            && (include_inactive || flag_of(e, EFFECT_IS_ACTIVE, effect))
        {
            found = true;
        }
        node = node_next(e, node);
    }
    found
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00822b40,
            magic_target_get_target_as_actor(Ptr<MagicTarget>) -> Ptr
        ),
        entry!(
            0x00822b90,
            magic_target_is_spell_target(Ptr<MagicTarget>, u32, bool) -> bool
        ),
        entry!(
            0x00822c30,
            magic_target_is_last_effect(Ptr<MagicTarget>, u32, u32) -> bool
        ),
        entry!(0x00822cf0, fn_00822cf0(Ptr<MagicTarget>, u32, bool) -> bool),
        entry!(0x00822d80, fn_00822d80(Ptr<MagicTarget>, u32) -> bool),
        entry!(0x00822e00, fn_00822e00(Ptr<MagicTarget>) -> bool),
        entry!(
            0x00822eb0,
            magic_target_get_usage_monitor_effect_magnitude(Ptr<MagicTarget>, u32) -> f32
        ),
        entry!(0x00822f70, fn_00822f70(Ptr<MagicTarget>, i32) -> bool),
        entry!(
            0x008230f0,
            magic_target_add_target(Ptr<MagicTarget>, u32, u32, u32, u32) -> bool
        ),
        entry!(0x008231f0, fn_008231f0(Ptr) -> bool),
        entry!(
            0x00823210,
            magic_target_check_add_effect(Ptr<MagicTarget>, u32, u32, u32) -> bool
        ),
        entry!(0x00823c20, fn_00823c20(Ptr) -> bool),
        entry!(
            0x00823c40,
            magic_target_update_target(Ptr<MagicTarget>, f32)
        ),
        entry!(0x00824080, fn_00824080(Ptr) -> bool),
        entry!(0x008240a0, fn_008240a0(Ptr, bool)),
        entry!(0x008240f0, fn_008240f0(Ptr) -> bool),
        entry!(0x00824290, fn_00824290(Ptr<MagicTarget>, u32, u32, u32)),
        entry!(
            0x00824370,
            magic_target_dispel_queued_spells(Ptr<MagicTarget>)
        ),
        entry!(
            0x00824400,
            magic_target_dispel(Ptr<MagicTarget>, u32, u32, u32)
        ),
        entry!(
            0x00824570,
            magic_target_dispel_ov2(Ptr<MagicTarget>, u32, u32, u32)
        ),
        entry!(0x00824630, fn_00824630(Ptr<MagicTarget>, u32)),
        entry!(
            0x00824800,
            magic_target_dispel_ov3(Ptr<MagicTarget>, u32, u32)
        ),
        entry!(0x008248e0, fn_008248e0(Ptr<MagicTarget>, u32, bool)),
        entry!(0x00824970, magic_target_dispel_ov4(Ptr<MagicTarget>)),
        entry!(0x008249d0, fn_008249d0(Ptr<MagicTarget>, bool)),
        entry!(
            0x00824af0,
            fn_00824af0(Ptr<MagicTarget>, u32, u32, u32) -> bool
        ),
        entry!(
            0x00824c00,
            fn_00824c00(Ptr<MagicTarget>, u32, u32, u32) -> bool
        ),
        entry!(0x00824df0, fn_00824df0(Ptr<MagicTarget>, u32, u32)),
        entry!(
            0x00824e70,
            magic_target_check_addiction(Ptr<MagicTarget>, u32) -> bool
        ),
        entry!(
            0x008251c0,
            magic_target_remove_effect(Ptr<MagicTarget>, u32, bool)
        ),
        entry!(0x00825270, magic_target_active_effect_list_compare_fn(u32, u32) -> i32),
        entry!(
            0x008254c0,
            magic_target_get_magic_target_form_id(Ptr<MagicTarget>) -> u32
        ),
        entry!(0x00825550, magic_target_get_magic_target_by_numeric_id(u32) -> u32),
        entry!(0x00825610, fn_00825610(Ptr<MagicTarget>, u32, bool) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test vtables and the fake addresses their slots point at.
    const TARGET_VTABLE: u32 = 0x0e00_0000;
    const TARGET_FN: u32 = 0x0f00_0000;
    const SPELL_VTABLE: u32 = 0x0e01_0000;
    const SPELL_FN: u32 = 0x0f01_0000;
    const EFFECT_VTABLE: u32 = 0x0e02_0000;
    const EFFECT_FN: u32 = 0x0f02_0000;
    const ACTOR_VTABLE: u32 = 0x0e03_0000;
    const ACTOR_FN: u32 = 0x0f03_0000;
    const CASTER_VTABLE: u32 = 0x0e04_0000;
    const CASTER_FN: u32 = 0x0f04_0000;
    const FORM_VTABLE: u32 = 0x0e05_0000;
    const FORM_FN: u32 = 0x0f05_0000;
    /// A test word the counter double points at.
    const COUNTER: u32 = 0x0f10_0000;

    /// Test data kept past the target's real fields (target-relative).
    const T_STATS: u32 = 0x100;
    const T_LIST: u32 = 0x104;
    const T_IS_ACTOR: u32 = 0x108;
    const T_INVULNERABLE: u32 = 0x109;
    const T_ABSORB: u32 = 0x10A;
    const T_REFLECT: u32 = 0x10B;
    const T_RESISTANCE: u32 = 0x10C;
    /// Actor-relative test data.
    const A_SAYS_YES_360: u32 = 0x300;
    const A_HANDED_NODE: u32 = 0x304;
    /// Effect-relative test data.
    const E_COPY: u32 = 0x50;
    const E_VALID_TARGET: u32 = 0x54;
    /// Spell-relative test data.
    const S_TYPE: u32 = 0x40;
    const S_SLOT_1C: u32 = 0x44;
    const S_SLOT_20: u32 = 0x45;
    /// Object +0x40: what the usage-monitor double returns for an
    /// `EffectItemList` at the object.
    const LIST_MONITOR: u32 = 0x40;
    /// Object +0x5C: the RTTI type a cast to succeeds for.
    const RTTI: u32 = 0x5C;
    /// Effect item test data.
    const I_USAGE_MONITOR: u32 = 0x20;
    const I_ACTOR_VALUE: u32 = 0x24;
    /// Effect setting test data.
    const SETTING_FLAG_C60: u32 = 0x9C;
    const SETTING_FLAG_1B0: u32 = 0x9D;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn ret_f32(v: f32) -> Ret {
        Ret {
            st0: v as f64,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn vtable(e: &mut Engine, table: u32, functions: u32, slots: u32) {
        let entries: Vec<u32> = (0..slots).map(|i| functions + 4 * i).collect();
        e.put_vtable(table, &entries);
    }

    fn f64_words(v: f64) -> Vec<u32> {
        let mut words = Vec::new();
        v.put(&mut words);
        words
    }

    /// An engine with the list helpers, the effect getters, the casts and
    /// the test vtables standing in for the game's code.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(LIST_NODE_IS_EMPTY, |e, a| {
            ret((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        e.register(LIST_NODE_ITEM, |_, a| ret(a[0]));
        e.register(LIST_NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(EFFECT_SPELL, |e, a| ret(e.mem.u32(a[0] + 0x08)));
        e.register(EFFECT_ITEM, |e, a| ret(e.mem.u32(a[0] + 0x0C)));
        e.register(EFFECT_IS_ACTIVE, |e, a| ret(e.mem.u8(a[0] + 0x10) as u32));
        e.register(EFFECT_HAS_STARTED, |e, a| ret(e.mem.u8(a[0] + 0x11) as u32));
        e.register(EFFECT_IS_DONE, |e, a| ret(e.mem.u8(a[0] + 0x13) as u32));
        e.register(EFFECT_FLAG_80, |e, a| {
            ret((e.mem.u32(a[0] + 0x18) & 0x80 != 0) as u32)
        });
        e.register(0x0080_4b60, |e, a| {
            ret((e.mem.u32(a[0] + 0x18) & 0x100 != 0) as u32)
        });
        e.register(EFFECT_MAGNITUDE, |e, a| ret_f32(e.mem.f32(a[0] + 0x1C)));
        e.register(EFFECT_DURATION, |e, a| ret_f32(e.mem.f32(a[0] + 0x20)));
        e.register(EFFECT_TARGET, |e, a| ret(e.mem.u32(a[0] + 0x24)));
        e.register(EFFECT_CASTER, |e, a| ret(e.mem.u32(a[0] + 0x28)));
        e.register(EFFECT_SPELL_TYPE, |e, a| ret(e.mem.u32(a[0] + 0x2C)));
        e.register(EFFECT_SOURCE, |e, a| ret(e.mem.u32(a[0] + 0x3C)));
        e.register(EFFECT_DISPLACEMENT_SPELL, |e, a| {
            ret(e.mem.u32(a[0] + 0x44))
        });
        e.register(EFFECT_SETTING_OF, |e, a| {
            let item = e.mem.u32(a[0] + 0x0C);
            ret(e.mem.u32(item + 0x14))
        });
        e.register(EFFECT_ITEM_SETTING, |e, a| ret(e.mem.u32(a[0] + 0x14)));
        e.register(EFFECT_ITEM_IS_USAGE_MONITOR, |e, a| {
            ret(e.mem.u8(a[0] + I_USAGE_MONITOR) as u32)
        });
        e.register(EFFECT_ITEM_ACTOR_VALUE, |e, a| {
            ret(e.mem.u32(a[0] + I_ACTOR_VALUE))
        });
        e.register(EFFECT_ITEM_LIST_USAGE_MONITOR, |e, a| {
            ret(e.mem.u32(a[0] + LIST_MONITOR))
        });
        e.register(EFFECT_SETTING_HAS_FLAG, |e, a| {
            ret((e.mem.u32(a[0] + 0x58) & a[1] != 0) as u32)
        });
        e.register(0x0058_cba0, |e, a| {
            ret((e.mem.u32(a[0] + 0x58) & a[1] != 0) as u32)
        });
        e.register(EFFECT_SETTING_ARCHETYPE, |e, a| ret(e.mem.u32(a[0] + 0x98)));
        e.register(0x0040_3c60, |e, a| {
            ret(e.mem.u8(a[0] + SETTING_FLAG_C60) as u32)
        });
        e.register(0x007d_f1b0, |e, a| {
            ret(e.mem.u8(a[0] + SETTING_FLAG_1B0) as u32)
        });
        // `004042a0`: the two effect items are the same.
        e.register(0x0040_42a0, |_, a| ret((a[0] == a[1]) as u32));
        e.register(ACTIVE_EFFECT_DISPEL, |e, a| {
            e.mem.set_u8(a[0] + 0x13, 1);
            ret(0)
        });
        e.register(RT_DYNAMIC_CAST, |e, a| {
            if a[0] != 0 && e.mem.u32(a[0] + RTTI) == a[3] {
                ret(a[0])
            } else {
                ret(0)
            }
        });
        e.register(DEBUG_LOG_ENABLED, |_, _| ret(0));

        vtable(&mut e, TARGET_VTABLE, TARGET_FN, 12);
        e.register(TARGET_FN + SLOT_GET_TARGET_STATS_OBJECT, |e, a| {
            ret(e.mem.u32(a[0] + T_STATS))
        });
        e.register(TARGET_FN + SLOT_GET_ACTIVE_EFFECT_LIST, |e, a| {
            ret(e.mem.u32(a[0] + T_LIST))
        });
        e.register(TARGET_FN + SLOT_MAGIC_TARGET_IS_ACTOR, |e, a| {
            ret(e.mem.u8(a[0] + T_IS_ACTOR) as u32)
        });
        e.register(TARGET_FN + SLOT_IS_INVULNERABLE, |e, a| {
            ret(e.mem.u8(a[0] + T_INVULNERABLE) as u32)
        });
        for slot in [
            SLOT_EFFECT_ADDED,
            SLOT_EFFECT_REMOVED,
            SLOT_EFFECT_REFLECTED,
            SLOT_EFFECT_ABSORBED,
        ] {
            e.register(TARGET_FN + slot, |_, _| ret(0));
        }
        e.register(TARGET_FN + SLOT_CHECK_RESISTANCE, |e, a| {
            ret_f32(e.mem.f32(a[0] + T_RESISTANCE))
        });
        e.register(TARGET_FN + SLOT_CHECK_ABSORB, |e, a| {
            ret(e.mem.u8(a[0] + T_ABSORB) as u32)
        });
        e.register(TARGET_FN + SLOT_CHECK_REFLECT, |e, a| {
            ret(e.mem.u8(a[0] + T_REFLECT) as u32)
        });

        vtable(&mut e, SPELL_VTABLE, SPELL_FN, 9);
        e.register(SPELL_FN + MAGIC_ITEM_SLOT_TYPE, |e, a| {
            ret(e.mem.u32(a[0] + S_TYPE))
        });
        e.register(SPELL_FN + MAGIC_ITEM_SLOT_1C, |e, a| {
            ret(e.mem.u8(a[0] + S_SLOT_1C) as u32)
        });
        e.register(SPELL_FN + MAGIC_ITEM_SLOT_20, |e, a| {
            ret(e.mem.u8(a[0] + S_SLOT_20) as u32)
        });

        vtable(&mut e, EFFECT_VTABLE, EFFECT_FN, 20);
        e.register(EFFECT_FN + ACTIVE_EFFECT_SLOT_DELETE, |_, _| ret(0));
        e.register(EFFECT_FN + ACTIVE_EFFECT_SLOT_COPY, |e, a| {
            ret(e.mem.u32(a[0] + E_COPY))
        });
        e.register(EFFECT_FN + ACTIVE_EFFECT_SLOT_FINALIZE, |_, _| ret(0));
        e.register(EFFECT_FN + ACTIVE_EFFECT_SLOT_CHECK_TARGET, |e, a| {
            ret(e.mem.u8(a[0] + E_VALID_TARGET) as u32)
        });

        vtable(&mut e, ACTOR_VTABLE, ACTOR_FN, 0x3E8 / 4);
        e.register(ACTOR_FN + 0x360, |e, a| {
            ret(e.mem.u8(a[0] + A_SAYS_YES_360) as u32)
        });
        e.register(ACTOR_FN + 0x3E4, |e, a| {
            let node = e.mem.u32(a[0] + A_HANDED_NODE);
            e.mem.set_u32(node, 0);
            ret(0)
        });
        vtable(&mut e, CASTER_VTABLE, CASTER_FN, 2);
        e.register(CASTER_FN + 4, |_, _| ret(0));

        e.map(ZERO, 8);
        e.map(HUNDRED, 8);
        e.set_global(HUNDRED, 100.0f64);
        e.map(PLAYER, 4);
        e.map(HUD_MESSAGE_TIME, 4);
        e.set_global(HUD_MESSAGE_TIME, 2.0f32);
        e.map(NO_VALUE_SORT_KEY, 4);
        e.set_global(NO_VALUE_SORT_KEY, 1000.0f32);
        // Room for the made-up name and form addresses (0x0102_xxxx).
        e.map(0x0102_0000, 0x1000);
        e.call_log = Some(vec![]);
        e
    }

    /// An actor block with its `MagicTarget` base (returned) at +0x94 and an
    /// empty effect list.
    fn target(e: &mut Engine, is_actor: bool) -> Ptr<MagicTarget> {
        let actor = e.mem.alloc(0x400);
        e.mem.set_u32(actor, ACTOR_VTABLE);
        e.mem.set_u32(actor + ACTOR_MAGIC_CASTER, CASTER_VTABLE);
        let this = actor + ACTOR_MAGIC_TARGET;
        e.mem.set_u32(this, TARGET_VTABLE);
        e.mem.set_u8(this + T_IS_ACTOR, is_actor as u8);
        let empty = list(e, &[]);
        e.mem.set_u32(this + T_LIST, empty);
        Ptr::new(this)
    }

    fn set_effects(e: &mut Engine, this: Ptr<MagicTarget>, effects: &[u32]) -> u32 {
        let head = list(e, effects);
        e.mem.set_u32(this.addr() + T_LIST, head);
        head
    }

    /// A `BSSimpleList` chain holding `items` (one empty node for none).
    fn list(e: &mut Engine, items: &[u32]) -> u32 {
        let nodes: Vec<u32> = (0..items.len().max(1)).map(|_| e.mem.alloc(8)).collect();
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(nodes[i], *item);
            let next = nodes.get(i + 1).copied().unwrap_or(0);
            e.mem.set_u32(nodes[i] + 4, next);
        }
        nodes[0]
    }

    fn spell(e: &mut Engine, kind: u32) -> u32 {
        let spell = e.mem.alloc(0x100);
        e.mem.set_u32(spell, SPELL_VTABLE);
        e.mem.set_u32(spell + S_TYPE, kind);
        e.mem.set_u32(spell + RTTI, MAGIC_ITEM_TYPE);
        spell
    }

    fn setting(e: &mut Engine, flags: u32, archetype: u32) -> u32 {
        let setting = e.mem.alloc(0xA0);
        e.mem.set_u32(setting + 0x58, flags);
        e.mem.set_u32(setting + 0x98, archetype);
        setting
    }

    fn item(e: &mut Engine, setting: u32) -> u32 {
        let item = e.mem.alloc(0x30);
        e.mem.set_u32(item + 0x14, setting);
        item
    }

    /// An active effect of `spell` with effect item `item`.
    fn effect(e: &mut Engine, spell: u32, item: u32) -> u32 {
        let effect = e.mem.alloc(0x60);
        e.mem.set_u32(effect, EFFECT_VTABLE);
        e.mem.set_u32(effect + 0x08, spell);
        e.mem.set_u32(effect + 0x0C, item);
        e.mem.set_u8(effect + 0x10, 1);
        effect
    }

    fn plain_effect(e: &mut Engine, spell: u32) -> u32 {
        let setting = setting(e, 0, 0);
        let item = item(e, setting);
        effect(e, spell, item)
    }

    #[test]
    fn get_target_as_actor_is_the_actor_base() {
        let mut e = engine();
        let this = target(&mut e, true);
        assert_eq!(
            magic_target_get_target_as_actor(&mut e, this).addr(),
            this.addr() - 0x94
        );
        let other = target(&mut e, false);
        assert_eq!(e.call(0x0082_2b40, &args![other]).u32(), 0);
    }

    #[test]
    fn is_spell_target_skips_inactive_and_usage_monitor_effects() {
        let mut e = engine();
        let this = target(&mut e, false);
        let s = spell(&mut e, 0);
        let inactive = plain_effect(&mut e, s);
        e.mem.set_u8(inactive + 0x10, 0);
        let monitor = plain_effect(&mut e, s);
        let monitor_item = e.mem.u32(monitor + 0x0C);
        e.mem.set_u8(monitor_item + I_USAGE_MONITOR, 1);
        set_effects(&mut e, this, &[inactive, monitor]);
        assert!(!magic_target_is_spell_target(&mut e, this, s, false));
        assert!(e.call(0x0082_2b90, &args![this, s, true]).bool());
        assert!(!magic_target_is_spell_target(&mut e, this, s + 8, true));
    }

    #[test]
    fn is_last_effect_counts_the_other_active_effects() {
        let mut e = engine();
        let this = target(&mut e, false);
        let s = spell(&mut e, 0);
        let first = plain_effect(&mut e, s);
        let second = plain_effect(&mut e, s);
        set_effects(&mut e, this, &[first, second]);
        assert!(!magic_target_is_last_effect(&mut e, this, s, first));
        e.mem.set_u8(second + 0x10, 0);
        assert!(e.call(0x0082_2c30, &args![this, s, first]).bool());
        let item = e.mem.u32(first + 0x0C);
        e.mem.set_u8(item + I_USAGE_MONITOR, 1);
        assert!(!magic_target_is_last_effect(&mut e, this, s, first));
    }

    #[test]
    fn archetype_search_needs_an_active_effect_unless_told() {
        let mut e = engine();
        let this = target(&mut e, false);
        let s = spell(&mut e, 0);
        let set = setting(&mut e, 0, 0x22);
        let it = item(&mut e, set);
        let fx = effect(&mut e, s, it);
        e.mem.set_u8(fx + 0x10, 0);
        set_effects(&mut e, this, &[fx]);
        assert!(!fn_00822cf0(&mut e, this, 0x22, false));
        assert!(e.call(0x0082_2cf0, &args![this, 0x22u32, true]).bool());
        assert!(!fn_00822cf0(&mut e, this, 0x23, true));
    }

    #[test]
    fn setting_search_finds_the_effect_setting() {
        let mut e = engine();
        let this = target(&mut e, false);
        let s = spell(&mut e, 0);
        let set = setting(&mut e, 0, 0);
        let it = item(&mut e, set);
        let fx = effect(&mut e, s, it);
        assert!(!fn_00822d80(&mut e, this, set));
        set_effects(&mut e, this, &[fx]);
        assert!(e.call(0x0082_2d80, &args![this, set]).bool());
        assert!(!fn_00822d80(&mut e, this, set + 4));
    }

    #[test]
    fn actor_value_0x10_search_needs_both_flags() {
        let mut e = engine();
        let this = target(&mut e, false);
        let s = spell(&mut e, 0);
        let set = setting(&mut e, 4, 0);
        let it = item(&mut e, set);
        e.mem.set_u32(it + I_ACTOR_VALUE, 0x10);
        let fx = effect(&mut e, s, it);
        set_effects(&mut e, this, &[fx]);
        assert!(!fn_00822e00(&mut e, this));
        e.mem.set_u8(set + SETTING_FLAG_C60, 1);
        assert!(e.call(0x0082_2e00, &args![this]).bool());
        e.mem.set_u32(set + 0x58, 0);
        assert!(!fn_00822e00(&mut e, this));
    }

    #[test]
    fn usage_monitor_magnitude_sums_the_monitor_effects() {
        let mut e = engine();
        let this = target(&mut e, false);
        let s = spell(&mut e, 0);
        let set = setting(&mut e, 0, 0);
        let monitor_item = item(&mut e, set);
        e.mem.set_u32(s + 0x0C + LIST_MONITOR, monitor_item);
        let other = plain_effect(&mut e, s);
        let first = effect(&mut e, s, monitor_item);
        e.mem.set_f32(first + 0x1C, 2.5);
        let second = effect(&mut e, s, monitor_item);
        e.mem.set_f32(second + 0x1C, 1.25);
        set_effects(&mut e, this, &[other, first, second]);
        assert_eq!(
            magic_target_get_usage_monitor_effect_magnitude(&mut e, this, s),
            3.75
        );
        assert_eq!(e.call(0x0082_2eb0, &args![this, 0u32]).f32(), 0.0);
        // Without another effect of the spell it is not a target of it.
        set_effects(&mut e, this, &[first]);
        let item_of_first = e.mem.u32(first + 0x0C);
        e.mem.set_u8(item_of_first + I_USAGE_MONITOR, 1);
        assert_eq!(
            magic_target_get_usage_monitor_effect_magnitude(&mut e, this, s),
            0.0
        );
    }

    #[test]
    fn actor_value_effect_search_skips_type_4_effects_the_actor_ignores() {
        let mut e = engine();
        let this = target(&mut e, true);
        let actor = this.addr() - 0x94;
        let s = spell(&mut e, 0);
        let set = setting(&mut e, 2, 0);
        let it = item(&mut e, set);
        e.mem.set_u32(it + I_ACTOR_VALUE, 7);
        let fx = effect(&mut e, s, it);
        e.mem.set_u32(fx + 0x2C, 4);
        set_effects(&mut e, this, &[fx]);
        e.mem.set_u8(actor + A_SAYS_YES_360, 1);
        assert!(!fn_00822f70(&mut e, this, 7));
        e.mem.set_u8(actor + A_SAYS_YES_360, 0);
        assert!(e.call(0x0082_2f70, &args![this, 7u32]).bool());
        assert!(!fn_00822f70(&mut e, this, -1));
        assert!(!fn_00822f70(&mut e, this, 0x4D));
        assert!(!fn_00822f70(&mut e, this, 8));
    }

    #[test]
    fn add_target_reflects_absorbs_or_checks_the_effect() {
        let mut e = engine();
        let this = target(&mut e, false);
        let s = spell(&mut e, 0);
        let fx = plain_effect(&mut e, s);
        let it = e.mem.u32(fx + 0x0C);
        e.mem.set_u32(it + 0x0C, 1);
        e.mem.set_u8(this.addr() + T_REFLECT, 1);
        assert!(!magic_target_add_target(&mut e, this, 0x55, s, fx, 0));
        assert_eq!(
            calls(&e, TARGET_FN + SLOT_EFFECT_REFLECTED),
            vec![vec![this.addr(), 0x55, fx]]
        );
        e.mem.set_u8(this.addr() + T_REFLECT, 0);
        e.mem.set_u8(this.addr() + T_ABSORB, 1);
        assert!(!magic_target_add_target(&mut e, this, 0x55, s, fx, 0));
        assert_eq!(
            calls(&e, TARGET_FN + SLOT_CHECK_ABSORB),
            vec![vec![this.addr(), 0x55, s, fx, 0]]
        );
        assert_eq!(calls(&e, TARGET_FN + SLOT_EFFECT_ABSORBED).len(), 1);
        // Spell slot +0x20 skips the checks: `CheckAddEffect`, fully
        // resisted (resistance 0.0).
        e.mem.set_u8(s + S_SLOT_20, 1);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x0082_30f0, &args![this, 0x55u32, s, fx, 0u32])
            .bool());
        assert!(calls(&e, TARGET_FN + SLOT_CHECK_ABSORB).is_empty());
        assert_eq!(calls(&e, TARGET_FN + SLOT_CHECK_RESISTANCE).len(), 1);
    }

    #[test]
    fn effect_item_setting_flag_0x40() {
        let mut e = engine();
        let set = setting(&mut e, 0, 0);
        let it = item(&mut e, set);
        assert!(!fn_008231f0(&mut e, Ptr::new(it)));
        e.mem.set_u8(set + SETTING_FLAG_1B0, 1);
        assert!(e.call(0x0082_31f0, &args![it]).bool());
        assert_eq!(calls(&e, 0x007d_f1b0), vec![vec![set], vec![set]]);
    }

    /// A target, a spell of `kind` and a not-yet-added effect of it (whose
    /// copy is the fourth value), set up to pass every check of
    /// `CheckAddEffect`.
    fn add_effect_setup(e: &mut Engine, kind: u32) -> (Ptr<MagicTarget>, u32, u32, u32) {
        let this = target(e, false);
        e.mem.set_f32(this.addr() + T_RESISTANCE, 0.5);
        let s = spell(e, kind);
        let fx = plain_effect(e, s);
        e.mem.set_u8(fx + E_VALID_TARGET, 1);
        let copy = plain_effect(e, s);
        e.mem.set_u32(fx + E_COPY, copy);
        for addr in [0x0070_37c0, 0x0080_4bc0, 0x007a_7eb0] {
            e.register(addr, |_, _| ret(0));
        }
        (this, s, fx, copy)
    }

    #[test]
    fn check_add_effect_logs_a_fully_resisted_effect() {
        let mut e = engine();
        let (this, s, fx, _) = add_effect_setup(&mut e, 0);
        e.mem.set_f32(this.addr() + T_RESISTANCE, -0.0);
        e.register(DEBUG_LOG_ENABLED, |_, _| ret(1));
        e.register(EFFECT_ITEM_GET_EFFECT_NAME, |_, a| ret(a[1]));
        e.register(BS_STRING_C_STR, |_, _| ret(0x0102_0000));
        e.register(MAGIC_ITEM_NAME, |_, _| ret(0x0102_0010));
        e.register(DEBUG_LOG, |_, _| ret(0));
        e.register(BS_STRING_DESTROY, |_, _| ret(0));
        assert!(!magic_target_check_add_effect(&mut e, this, 0x55, s, fx));
        let mut expected = vec![FULLY_RESISTED_FORMAT, NULL_NAME, 0x0102_0010, 0x0102_0000];
        expected.extend(f64_words(-0.0));
        assert_eq!(calls(&e, DEBUG_LOG), vec![expected]);
        assert!(calls(&e, EFFECT_FN + ACTIVE_EFFECT_SLOT_CHECK_TARGET).is_empty());
    }

    #[test]
    fn check_add_effect_accepts_a_nan_resistance() {
        let mut e = engine();
        let (this, s, fx, _) = add_effect_setup(&mut e, 0);
        e.mem.set_u32(this.addr() + T_RESISTANCE, 0x7fc0_0000);
        e.mem.set_u8(fx + E_VALID_TARGET, 0);
        assert!(!magic_target_check_add_effect(&mut e, this, 0x55, s, fx));
        assert_eq!(
            calls(&e, EFFECT_FN + ACTIVE_EFFECT_SLOT_CHECK_TARGET).len(),
            1
        );
    }

    #[test]
    fn check_add_effect_rejects_invalid_targets_and_hostile_effects() {
        let mut e = engine();
        let (this, s, fx, _) = add_effect_setup(&mut e, 0);
        e.mem.set_u8(fx + E_VALID_TARGET, 0);
        assert!(!e.call(0x0082_3210, &args![this, 0x55u32, s, fx]).bool());
        assert_eq!(
            calls(&e, EFFECT_FN + ACTIVE_EFFECT_SLOT_CHECK_TARGET),
            vec![vec![fx, this.addr()]]
        );
        e.mem.set_u8(this.addr() + T_INVULNERABLE, 1);
        e.register(EFFECT_ITEM_IS_HOSTILE, |_, _| ret(1));
        e.call_log = Some(vec![]);
        assert!(!magic_target_check_add_effect(&mut e, this, 0x55, s, fx));
        assert!(calls(&e, EFFECT_FN + ACTIVE_EFFECT_SLOT_CHECK_TARGET).is_empty());
    }

    #[test]
    fn check_add_effect_merges_a_usage_monitor_effect() {
        let mut e = engine();
        let (this, s, fx, _) = add_effect_setup(&mut e, 0);
        let monitor = e.mem.u32(fx + 0x0C);
        e.mem.set_u32(s + 0x0C + LIST_MONITOR, monitor);
        e.mem.set_f32(fx + 0x1C, 1.5);
        let existing = effect(&mut e, s, monitor);
        e.mem.set_f32(existing + 0x1C, 2.0);
        set_effects(&mut e, this, &[existing]);
        e.register(0x0080_4cb0, |_, _| ret(0));
        e.register(0x004f_5d90, |_, _| ret(0));
        assert!(magic_target_check_add_effect(&mut e, this, 0x55, s, fx));
        assert_eq!(
            calls(&e, 0x0080_4cb0),
            vec![vec![existing, 3.5f32.to_bits()]]
        );
        assert_eq!(calls(&e, 0x004f_5d90), vec![vec![existing, 0]]);
        assert!(calls(&e, EFFECT_FN + ACTIVE_EFFECT_SLOT_COPY).is_empty());
    }

    #[test]
    fn check_add_effect_dispels_the_spell_and_adds_the_copy() {
        let mut e = engine();
        let (this, s, fx, copy) = add_effect_setup(&mut e, 0);
        let old = plain_effect(&mut e, s);
        e.mem.set_u32(old + 0x28, 0x55);
        let head = set_effects(&mut e, this, &[old]);
        assert!(magic_target_check_add_effect(&mut e, this, 0x55, s, fx));
        assert_eq!(calls(&e, ACTIVE_EFFECT_DISPEL), vec![vec![old, 1]]);
        assert_eq!(calls(&e, 0x0070_37c0), vec![vec![copy, this.addr()]]);
        assert_eq!(calls(&e, 0x0080_4bc0), vec![vec![copy, 0.5f32.to_bits()]]);
        assert_eq!(calls(&e, 0x007a_7eb0), vec![vec![head, copy, 0x0082_5270]]);
        assert_eq!(
            calls(&e, TARGET_FN + SLOT_EFFECT_ADDED),
            vec![vec![this.addr(), copy]]
        );
        assert_eq!(
            calls(&e, EFFECT_FN + ACTIVE_EFFECT_SLOT_FINALIZE),
            vec![vec![copy]]
        );
        // No effect list: the copy is deleted.
        e.mem.set_u32(this.addr() + T_LIST, 0);
        e.call_log = Some(vec![]);
        assert!(!magic_target_check_add_effect(&mut e, this, 0x55, s, fx));
        assert_eq!(
            calls(&e, EFFECT_FN + ACTIVE_EFFECT_SLOT_DELETE),
            vec![vec![copy, 1]]
        );
    }

    #[test]
    fn check_add_effect_extends_an_effect_with_a_duration() {
        let mut e = engine();
        let (this, s, fx, _) = add_effect_setup(&mut e, 5);
        e.mem.set_f32(fx + 0x20, 10.0);
        let old = plain_effect(&mut e, s);
        e.mem.set_f32(old + 0x20, 4.0);
        e.mem.set_u32(old + 0x28, 0x55);
        set_effects(&mut e, this, &[old]);
        e.register(0x0069_3d50, |_, _| ret(0));
        assert!(magic_target_check_add_effect(&mut e, this, 0x55, s, fx));
        assert_eq!(calls(&e, 0x0069_3d50), vec![vec![old, 14.0f32.to_bits()]]);
        assert!(calls(&e, ACTIVE_EFFECT_DISPEL).is_empty());
    }

    #[test]
    fn check_add_effect_adjusts_a_withdrawal_duration_by_entry_point() {
        let mut e = engine();
        let (this, s, fx, _) = add_effect_setup(&mut e, 10);
        e.mem.set_u8(this.addr() + T_IS_ACTOR, 1);
        e.mem.set_f32(fx + 0x20, 8.0);
        e.register(HANDLE_ENTRY_POINT, |e, a| {
            e.mem.set_f32(a[2], 6.0);
            ret(0)
        });
        e.register(0x0069_3d50, |_, _| ret(0));
        assert!(magic_target_check_add_effect(&mut e, this, 0x55, s, fx));
        let entry = &calls(&e, HANDLE_ENTRY_POINT)[0];
        assert_eq!(entry[..2], [0x18, this.addr() - 0x94]);
        assert_eq!(calls(&e, 0x0069_3d50), vec![vec![fx, 6.0f32.to_bits()]]);
    }

    #[test]
    fn spell_item_test_passes_2() {
        let mut e = engine();
        e.register(0x0040_e210, |_, _| ret(1));
        assert!(fn_00823c20(&mut e, Ptr::new(0x1234)));
        assert!(e.call(0x0082_3c20, &args![0x1234u32]).bool());
        assert_eq!(calls(&e, 0x0040_e210)[0], vec![0x1234, 2]);
    }

    fn update_doubles(e: &mut Engine) {
        e.register(0x0080_4b30, |_, a| {
            ret((a[0] == 1 || a[0] == 4 || a[0] == 10) as u32)
        });
        e.register(0x0080_4560, |_, _| ret(0));
        // `00905330` takes the effect out of the list (the next node moves into
        // the emptied one).
        e.register(0x0090_5330, |e, a| {
            let effect = e.mem.u32(a[1]);
            let mut node = a[0];
            while node != 0 {
                if e.mem.u32(node) == effect {
                    let next = e.mem.u32(node + 4);
                    let (item, after) = if next == 0 {
                        (0, 0)
                    } else {
                        (e.mem.u32(next), e.mem.u32(next + 4))
                    };
                    e.mem.set_u32(node, item);
                    e.mem.set_u32(node + 4, after);
                    break;
                }
                node = e.mem.u32(node + 4);
            }
            ret(0)
        });
        e.register(0x0047_0470, |_, _| ret(0));
        e.map(COUNTER, 4);
        e.register(0x0043_d4d0, |_, _| ret(COUNTER));
    }

    #[test]
    fn update_target_updates_and_removes_effects() {
        let mut e = engine();
        update_doubles(&mut e);
        let this = target(&mut e, false);
        let s = spell(&mut e, 0);
        let live = plain_effect(&mut e, s);
        let finished = plain_effect(&mut e, s);
        e.mem.set_u32(finished + 0x18, 0x8000_0000);
        let head = set_effects(&mut e, this, &[live, finished]);
        magic_target_update_target(&mut e, this, 0.25);
        assert_eq!(calls(&e, 0x0080_4560), vec![vec![live, 0.25f32.to_bits()]]);
        assert_eq!(calls(&e, 0x0090_5330).len(), 1);
        assert_eq!(calls(&e, 0x0090_5330)[0][0], head);
        assert_eq!(
            calls(&e, TARGET_FN + SLOT_EFFECT_REMOVED),
            vec![vec![this.addr(), finished]]
        );
        assert_eq!(
            calls(&e, EFFECT_FN + ACTIVE_EFFECT_SLOT_DELETE),
            vec![vec![finished, 1]]
        );
        assert!(!e.get(this, MagicTarget::bUpdating));
        assert_eq!(calls(&e, 0x0047_0470), vec![vec![this.addr() + 8]]);
    }

    #[test]
    fn update_target_holds_back_started_effects_without_flag_0x80() {
        let mut e = engine();
        update_doubles(&mut e);
        let this = target(&mut e, false);
        let s = spell(&mut e, 0);
        let set = setting(&mut e, 2, 0);
        let it = item(&mut e, set);
        let fx = effect(&mut e, s, it);
        e.mem.set_u32(fx + 0x2C, 4);
        e.mem.set_u8(fx + 0x11, 1);
        set_effects(&mut e, this, &[fx]);
        magic_target_update_target(&mut e, this, 1.0);
        assert!(calls(&e, 0x0080_4560).is_empty());
        e.mem.set_u32(fx + 0x44, 0x99);
        magic_target_update_target(&mut e, this, 1.0);
        assert_eq!(calls(&e, 0x0080_4560).len(), 1);
    }

    #[test]
    fn update_target_skips_an_actor_without_a_process() {
        let mut e = engine();
        update_doubles(&mut e);
        let this = target(&mut e, true);
        let fx = plain_effect(&mut e, 0);
        set_effects(&mut e, this, &[fx]);
        e.register(0x008d_8520, |_, _| ret(0));
        e.call(0x0082_3c40, &args![this, 1.0f32]);
        assert!(calls(&e, 0x0080_4560).is_empty());
        assert_eq!(calls(&e, 0x008d_8520), vec![vec![this.addr() - 0x94]]);
    }

    #[test]
    fn update_target_shows_the_wear_off_message_for_the_players_last_chem() {
        let mut e = engine();
        update_doubles(&mut e);
        let this = target(&mut e, false);
        e.set_global(PLAYER, this.addr() - 0x94);
        let s = spell(&mut e, 7);
        let fx = plain_effect(&mut e, s);
        e.mem.set_u8(fx + 0x13, 1);
        e.mem.set_u32(fx + 0x24, this.addr());
        set_effects(&mut e, this, &[fx]);
        e.register(MAGIC_ITEM_NAME, |_, _| ret(0x0102_0010));
        e.register(SETTING_STRING, |_, _| ret(0x0102_0020));
        e.register(FORMAT_STRING, |_, _| ret(0));
        e.register(SHOW_HUD_MESSAGE, |_, _| ret(0));
        magic_target_update_target(&mut e, this, 1.0);
        let format = &calls(&e, FORMAT_STRING)[0];
        assert_eq!(format[1..], [TEXT_BUFFER, 0x0102_0020, 0x0102_0010]);
        assert_eq!(calls(&e, SETTING_STRING), vec![vec![WEAR_OFF_MESSAGE]]);
        let hud = &calls(&e, SHOW_HUD_MESSAGE)[0];
        assert_eq!(hud[0], format[0]);
        assert_eq!(hud[1..], [0, 0, CHEMS_WEAR_OFF_SOUND, 2.0f32.to_bits(), 0]);
        // The done effect was marked and removed.
        assert_eq!(
            calls(&e, TARGET_FN + SLOT_EFFECT_REMOVED),
            vec![vec![this.addr(), fx]]
        );
    }

    #[test]
    fn effect_flag_bits() {
        let mut e = engine();
        let fx = e.mem.alloc(0x60);
        e.mem.set_u32(fx + 0x18, 0x200);
        assert!(fn_00824080(&mut e, Ptr::new(fx)));
        assert!(!e.call(0x0082_40f0, &args![fx]).bool());
        e.call(0x0082_40a0, &args![fx, true]);
        assert_eq!(e.mem.u32(fx + 0x18), 0x8000_0200);
        assert!(fn_008240f0(&mut e, Ptr::new(fx)));
        fn_008240a0(&mut e, Ptr::new(fx), false);
        assert_eq!(e.mem.u32(fx + 0x18), 0x200);
        e.mem.set_u32(fx + 0x18, 0);
        assert!(!e.call(0x0082_4080, &args![fx]).bool());
    }

    fn queue_doubles(e: &mut Engine) {
        e.register(0x0040_1000, |e, a| ret(e.mem.alloc(a[0])));
        e.register(0x0082_4340, |_, a| ret(a[0]));
        e.register(0x005a_e3d0, |_, _| ret(0));
    }

    #[test]
    fn queued_dispel_data_goes_to_the_post_update_list() {
        let mut e = engine();
        queue_doubles(&mut e);
        let this = target(&mut e, false);
        let added = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let seen = added.clone();
        e.register_double(0x005a_e3d0, move |e, a| {
            seen.set(e.mem.u32(a[1]));
            ret(0)
        });
        fn_00824290(&mut e, this, 0x11, 0x22, 0x33);
        assert_eq!(calls(&e, 0x005a_e3d0)[0][0], this.addr() + 8);
        assert_eq!(calls(&e, 0x0040_1000), vec![vec![0x0C]]);
        let block = calls(&e, 0x0082_4340)[0][0];
        assert_eq!(added.get(), block);
        assert_eq!(e.mem.u32(block), 0x11);
        assert_eq!(e.mem.u32(block + 4), 0x22);
        assert_eq!(e.mem.u32(block + 8), 0x33);
        e.call(0x0082_4290, &args![this, 1u32, 2u32, 3u32]);
        assert_eq!(calls(&e, 0x005a_e3d0).len(), 2);
    }

    #[test]
    fn dispel_queued_spells_runs_and_frees_each_entry() {
        let mut e = engine();
        e.register(0x0040_1030, |_, _| ret(0));
        e.register(0x0047_0470, |_, _| ret(0));
        let this = target(&mut e, false);
        let s = spell(&mut e, 0);
        let fx = plain_effect(&mut e, s);
        set_effects(&mut e, this, &[fx]);
        let data = e.mem.alloc(0x0C);
        e.mem.set_u32(data, s);
        e.mem.set_u32(this.addr() + 8, data);
        magic_target_dispel_queued_spells(&mut e, this);
        assert_eq!(calls(&e, ACTIVE_EFFECT_DISPEL), vec![vec![fx, 1]]);
        assert_eq!(calls(&e, 0x0040_1030), vec![vec![data]]);
        assert_eq!(calls(&e, 0x0047_0470), vec![vec![this.addr() + 8]]);
        e.call(0x0082_4370, &args![this]);
        assert_eq!(calls(&e, 0x0047_0470).len(), 2);
    }

    #[test]
    fn dispel_matches_spell_caster_and_queues_while_updating() {
        let mut e = engine();
        queue_doubles(&mut e);
        let this = target(&mut e, false);
        let s = spell(&mut e, 0);
        let done = plain_effect(&mut e, s);
        e.mem.set_u8(done + 0x13, 1);
        let other_caster = plain_effect(&mut e, s);
        e.mem.set_u32(other_caster + 0x28, 0x66);
        let mine = plain_effect(&mut e, s);
        e.mem.set_u32(mine + 0x28, 0x55);
        set_effects(&mut e, this, &[done, other_caster, mine]);
        magic_target_dispel(&mut e, this, s, 0x55, 0);
        assert_eq!(calls(&e, ACTIVE_EFFECT_DISPEL), vec![vec![mine, 1]]);
        // While updating the dispel is queued.
        e.mem.set_u8(mine + 0x13, 0);
        e.set(this, MagicTarget::bUpdating, true);
        e.call(0x0082_4400, &args![this, s, 0x55u32, 0u32]);
        assert_eq!(calls(&e, ACTIVE_EFFECT_DISPEL).len(), 1);
        let block = calls(&e, 0x0082_4340)[0][0];
        assert_eq!(e.mem.u32(block + 8), mine);
    }

    #[test]
    fn dispel_with_an_effect_stops_after_the_first_other_item() {
        let mut e = engine();
        let this = target(&mut e, false);
        let s = spell(&mut e, 0);
        let given = plain_effect(&mut e, s);
        let given_item = e.mem.u32(given + 0x0C);
        let same = effect(&mut e, s, given_item);
        let first_other = plain_effect(&mut e, s);
        let second_other = plain_effect(&mut e, s);
        set_effects(&mut e, this, &[same, first_other, second_other]);
        magic_target_dispel(&mut e, this, s, 0, given);
        assert_eq!(calls(&e, ACTIVE_EFFECT_DISPEL), vec![vec![first_other, 1]]);
        // A different source is not dispelled.
        e.mem.set_u32(given + 0x3C, 9);
        e.call_log = Some(vec![]);
        magic_target_dispel(&mut e, this, s, 0, given);
        assert!(calls(&e, ACTIVE_EFFECT_DISPEL).is_empty());
    }

    #[test]
    fn dispel_ov2_matches_the_effect_item() {
        let mut e = engine();
        let this = target(&mut e, false);
        let s = spell(&mut e, 0);
        let a = plain_effect(&mut e, s);
        let b = plain_effect(&mut e, s);
        let b_item = e.mem.u32(b + 0x0C);
        set_effects(&mut e, this, &[a, b]);
        magic_target_dispel_ov2(&mut e, this, b_item, s, 0);
        assert_eq!(calls(&e, ACTIVE_EFFECT_DISPEL), vec![vec![b, 1]]);
        e.mem.set_u8(b + 0x13, 0);
        e.call(0x0082_4570, &args![this, b_item, s, 0x77u32]);
        assert_eq!(calls(&e, ACTIVE_EFFECT_DISPEL).len(), 1);
    }

    #[test]
    fn dispel_by_type_matches_the_spell_type_or_poison() {
        let mut e = engine();
        let this = target(&mut e, false);
        let three = spell(&mut e, 3);
        let poison = spell(&mut e, 2);
        e.mem.set_u32(poison + RTTI, ALCHEMY_ITEM_TYPE);
        e.register(0x0040_5da0, |_, _| ret(1));
        let a = plain_effect(&mut e, three);
        let b = plain_effect(&mut e, poison);
        set_effects(&mut e, this, &[a, b]);
        fn_00824630(&mut e, this, 3);
        assert_eq!(calls(&e, ACTIVE_EFFECT_DISPEL), vec![vec![a, 0]]);
        e.call(0x0082_4630, &args![this, 5u32]);
        assert_eq!(calls(&e, ACTIVE_EFFECT_DISPEL)[1], vec![b, 0]);
        assert_eq!(calls(&e, 0x0040_5da0)[1], vec![poison + 0x3C]);
    }

    #[test]
    fn dispel_by_type_1_hands_entries_to_the_actor() {
        let mut e = engine();
        let this = target(&mut e, false);
        let actor = e.mem.alloc(0x400);
        e.mem.set_u32(actor, ACTOR_VTABLE);
        e.mem.set_u32(actor + RTTI, ACTOR_TYPE);
        e.mem.set_u32(this.addr() + T_STATS, actor);
        let owner = e.mem.alloc(0x100);
        e.mem.set_u32(actor + 0x200, owner);
        e.register(0x0041_81e0, |e, a| ret(e.mem.u32(a[0] + 0x200)));
        e.register(0x0071_7e50, |_, a| ret(a[0] + 4));
        // An entry with an object at +0x18 whose virtual +0x18 returns 1.
        let entry = e.mem.alloc(0x40);
        e.mem.set_u32(entry + 0x18, FORM_VTABLE);
        vtable(&mut e, FORM_VTABLE, FORM_FN, 8);
        e.register(FORM_FN + 0x18, |_, _| ret(1));
        let node = owner + 0x7C + 4;
        e.mem.set_u32(node, entry);
        e.mem.set_u32(actor + A_HANDED_NODE, node);
        fn_00824630(&mut e, this, 1);
        assert_eq!(calls(&e, ACTOR_FN + 0x3E4), vec![vec![actor, entry]]);
        assert_eq!(calls(&e, 0x0041_81e0).len(), 2);
    }

    #[test]
    fn dispel_ov3_dispels_and_marks_by_archetype() {
        let mut e = engine();
        let this = target(&mut e, false);
        let s = spell(&mut e, 0);
        let set = setting(&mut e, 0, 0x30);
        let it = item(&mut e, set);
        let fx = effect(&mut e, s, it);
        let other = plain_effect(&mut e, s);
        set_effects(&mut e, this, &[fx, other]);
        magic_target_dispel_ov3(&mut e, this, 0x30, 0);
        assert_eq!(calls(&e, ACTIVE_EFFECT_DISPEL), vec![vec![fx, 1]]);
        assert!(fn_008240f0(&mut e, Ptr::new(fx)));
        e.mem.set_u8(fx + 0x13, 0);
        e.call(0x0082_4800, &args![this, 0x30u32, 0x55u32]);
        assert_eq!(calls(&e, ACTIVE_EFFECT_DISPEL).len(), 1);
    }

    #[test]
    fn dispel_by_source_passes_remove_on() {
        let mut e = engine();
        let this = target(&mut e, false);
        let a = plain_effect(&mut e, 0);
        e.mem.set_u32(a + 0x3C, 0x42);
        let b = plain_effect(&mut e, 0);
        set_effects(&mut e, this, &[a, b]);
        fn_008248e0(&mut e, this, 0, true);
        assert!(calls(&e, ACTIVE_EFFECT_DISPEL).is_empty());
        e.call(0x0082_48e0, &args![this, 0x42u32, false]);
        assert_eq!(calls(&e, ACTIVE_EFFECT_DISPEL), vec![vec![a, 0]]);
    }

    #[test]
    fn dispel_ov4_dispels_everything() {
        let mut e = engine();
        let this = target(&mut e, false);
        let a = plain_effect(&mut e, 0);
        let b = plain_effect(&mut e, 0);
        set_effects(&mut e, this, &[a, 0, b]);
        magic_target_dispel_ov4(&mut e, this);
        assert_eq!(
            calls(&e, ACTIVE_EFFECT_DISPEL),
            vec![vec![a, 1], vec![b, 1]]
        );
        e.call(0x0082_4970, &args![this]);
        assert_eq!(calls(&e, ACTIVE_EFFECT_DISPEL).len(), 4);
    }

    #[test]
    fn dispel_listed_types_and_unflagged_enchantments() {
        let mut e = engine();
        let this = target(&mut e, false);
        e.register(0x005f_36f0, |e, a| ret(e.mem.u32(a[0] + 0x34)));
        let two = spell(&mut e, 2);
        let enchantment = spell(&mut e, 5);
        e.mem.set_u32(enchantment + RTTI, ENCHANTMENT_ITEM_TYPE);
        let five = spell(&mut e, 5);
        let a = plain_effect(&mut e, two);
        let b = plain_effect(&mut e, enchantment);
        let c = plain_effect(&mut e, five);
        set_effects(&mut e, this, &[a, b, c]);
        fn_008249d0(&mut e, this, false);
        assert_eq!(
            calls(&e, ACTIVE_EFFECT_DISPEL),
            vec![vec![a, 0], vec![b, 0]]
        );
        e.mem.set_u32(enchantment + 0x34, 1);
        e.call(0x0082_49d0, &args![this, true]);
        assert_eq!(calls(&e, ACTIVE_EFFECT_DISPEL)[2..], [vec![a, 1]]);
    }

    #[test]
    fn reset_finds_an_effect_with_another_item() {
        let mut e = engine();
        e.register(0x004f_5d90, |_, _| ret(0));
        let this = target(&mut e, false);
        let s = spell(&mut e, 0);
        let given = plain_effect(&mut e, s);
        let given_item = e.mem.u32(given + 0x0C);
        let same = effect(&mut e, s, given_item);
        let other = plain_effect(&mut e, s);
        set_effects(&mut e, this, &[same, other]);
        assert!(fn_00824af0(&mut e, this, s, 0, given));
        assert_eq!(calls(&e, 0x004f_5d90), vec![vec![other, 0]]);
        assert!(!e.call(0x0082_4af0, &args![this, s, 0x55u32, 0u32]).bool());
    }

    #[test]
    fn extend_adds_the_durations() {
        let mut e = engine();
        e.register(0x0069_3d50, |_, _| ret(0));
        let this = target(&mut e, false);
        let s = spell(&mut e, 0);
        let given = plain_effect(&mut e, s);
        e.mem.set_f32(given + 0x20, 0.5);
        let other = plain_effect(&mut e, s);
        e.mem.set_f32(other + 0x20, 2.0);
        set_effects(&mut e, this, &[other]);
        assert!(fn_00824c00(&mut e, this, s, 0, given));
        assert_eq!(calls(&e, 0x0069_3d50), vec![vec![other, 2.5f32.to_bits()]]);
        let s2 = spell(&mut e, 0);
        assert!(!e.call(0x0082_4c00, &args![this, s2, 0u32, given]).bool());
    }

    #[test]
    fn extend_counts_every_effect_for_a_listed_alchemy_item() {
        let mut e = engine();
        e.register(0x0069_3d50, |_, _| ret(0));
        let this = target(&mut e, false);
        let chem = spell(&mut e, 7);
        e.mem.set_u32(chem + RTTI, ALCHEMY_ITEM_TYPE);
        e.mem.set_u32(chem + 0x9C + 4, 0x0D);
        let given = plain_effect(&mut e, chem);
        let unrelated = plain_effect(&mut e, 0x99);
        set_effects(&mut e, this, &[unrelated]);
        assert!(fn_00824c00(&mut e, this, chem, 0, given));
        assert_eq!(calls(&e, 0x0069_3d50)[0][0], unrelated);
    }

    #[test]
    fn spell_effects_get_the_value_and_a_reset() {
        let mut e = engine();
        e.register(0x008d_8040, |_, _| ret(0));
        e.register(0x0080_43f0, |_, _| ret(0));
        let this = target(&mut e, false);
        let s = spell(&mut e, 0);
        let a = plain_effect(&mut e, s);
        let b = plain_effect(&mut e, 0);
        set_effects(&mut e, this, &[a, b]);
        fn_00824df0(&mut e, this, 0x77, s);
        assert_eq!(calls(&e, 0x008d_8040), vec![vec![a, 0x77]]);
        assert_eq!(calls(&e, 0x0080_43f0), vec![vec![a, 0, 1]]);
        e.call(0x0082_4df0, &args![this, 0x77u32, 0x1234u32]);
        assert_eq!(calls(&e, 0x008d_8040).len(), 1);
    }

    fn addiction_setup(e: &mut Engine, roll: u32) -> (Ptr<MagicTarget>, u32) {
        let this = target(e, true);
        let chem = e.mem.alloc(0x100);
        e.register(0x0064_4a50, |_, _| ret_f32(0.25));
        e.register(0x0040_30b0, |_, a| ret(a[0] + 0x80));
        e.register(HANDLE_ENTRY_POINT, |_, _| ret(0));
        e.register(0x0082_55a0, |_, _| ret(0));
        e.register_double(0x0048_7f50, move |_, _| ret(roll));
        (this, chem)
    }

    #[test]
    fn check_addiction_fails_a_roll_at_or_above_the_chance() {
        let mut e = engine();
        let (this, chem) = addiction_setup(&mut e, 125);
        assert!(!magic_target_check_addiction(&mut e, this, chem));
        assert_eq!(calls(&e, 0x0082_55a0), vec![vec![this.addr()]]);
        let entry = &calls(&e, HANDLE_ENTRY_POINT)[0];
        assert_eq!(entry[..2], [0x17, this.addr() - 0x94]);
        assert!(!e.call(0x0082_4e70, &args![this, 0u32]).bool());
    }

    #[test]
    fn check_addiction_needs_usage_above_the_threshold() {
        let mut e = engine();
        let (this, chem) = addiction_setup(&mut e, 0);
        let monitor = e.mem.alloc(0x30);
        e.mem.set_u32(chem + 0x3C + LIST_MONITOR, monitor);
        let threshold = e.mem.alloc(8);
        e.mem.set_f32(threshold + 4, 50.0);
        e.register_double(SETTING_FLOAT_ADDRESS, move |_, _| ret(threshold + 4));
        // No usage at all: not above the threshold.
        assert!(!magic_target_check_addiction(&mut e, this, chem));
        assert!(calls(&e, HANDLE_ENTRY_POINT).is_empty());
        assert_eq!(
            calls(&e, SETTING_FLOAT_ADDRESS),
            vec![vec![ADDICTION_USAGE_THRESHOLD]]
        );
    }

    #[test]
    fn check_addiction_addicts_the_actor() {
        let mut e = engine();
        let (this, chem) = addiction_setup(&mut e, 10);
        let actor = this.addr() - 0x94;
        e.mem.set_u8(actor + A_SAYS_YES_360, 1);
        e.register(0x008d_8040, |_, _| ret(0));
        e.register(0x0080_43f0, |_, _| ret(0));
        e.register(0x004d_5c60, |_, _| ret(0));
        e.register(LOOKUP_FORM_BY_ID, |_, _| ret(0x0102_0100));
        e.register(MAGIC_ITEM_NAME, |_, _| ret(0x0102_0010));
        e.register(SETTING_STRING, |_, _| ret(0x0102_0020));
        e.register(FORMAT_STRING, |_, _| ret(0));
        e.register(SHOW_HUD_MESSAGE, |_, _| ret(0));
        e.register(IMAGE_SPACE_MODIFIER_TRIGGER, |_, _| ret(0));
        assert!(magic_target_check_addiction(&mut e, this, chem));
        assert!(calls(&e, 0x0082_55a0).is_empty());
        assert_eq!(
            calls(&e, CASTER_FN + 4),
            vec![vec![actor + 0x88, chem + 0x80, 0]]
        );
        assert_eq!(calls(&e, 0x004d_5c60), vec![vec![10]]);
        assert_eq!(calls(&e, LOOKUP_FORM_BY_ID), vec![vec![0x0001_9482]]);
        assert_eq!(calls(&e, SETTING_STRING), vec![vec![ADDICTED_MESSAGE]]);
        assert_eq!(calls(&e, MAGIC_ITEM_NAME), vec![vec![chem + 0x30]]);
        assert_eq!(
            calls(&e, SHOW_HUD_MESSAGE)[0][1..],
            [1, 0, CHEMS_ADDICTED_SOUND, 2.0f32.to_bits(), 0]
        );
        // The form is no image space modifier here: the cast gives null.
        assert_eq!(
            calls(&e, IMAGE_SPACE_MODIFIER_TRIGGER),
            vec![vec![0, 1.0f32.to_bits(), 0]]
        );
    }

    #[test]
    fn remove_effect_marks_queues_or_deletes() {
        let mut e = engine();
        update_doubles(&mut e);
        let this = target(&mut e, false);
        let fx = plain_effect(&mut e, 0);
        magic_target_remove_effect(&mut e, this, fx, false);
        assert!(fn_008240f0(&mut e, Ptr::new(fx)));
        assert!(calls(&e, 0x0090_5330).is_empty());
        magic_target_remove_effect(&mut e, this, fx, true);
        assert_eq!(
            calls(&e, EFFECT_FN + ACTIVE_EFFECT_SLOT_DELETE),
            vec![vec![fx, 1]]
        );
        // Through the task queue.
        e.mem.set_u32(COUNTER, 2);
        e.register(0x0071_3d90, |_, _| ret(1));
        e.register(0x0045_37b0, |_, _| ret(0x0102_0200));
        e.register(0x0087_b890, |_, _| ret(0));
        e.call(0x0082_51c0, &args![this, fx, true]);
        assert_eq!(calls(&e, 0x0087_b890), vec![vec![0x0102_0200, fx]]);
        assert_eq!(calls(&e, EFFECT_FN + ACTIVE_EFFECT_SLOT_DELETE).len(), 1);
    }

    #[test]
    fn compare_fn_formats_both_sort_keys() {
        let mut e = engine();
        e.register(EFFECT_ITEM_GET_EFFECT_NAME, |_, a| ret(a[1]));
        e.register(BS_STRING_C_STR, |_, _| ret(0x0102_0000));
        e.register(BS_STRING_DESTROY, |_, _| ret(0));
        e.register(0x00ec_623a, |_, _| ret(0));
        e.register(0x0046_fd30, |_, _| ret(-1i32 as u32));
        let flagged = setting(&mut e, 0x80, 0);
        let flagged_item = item(&mut e, flagged);
        let first = effect(&mut e, 0, flagged_item);
        e.mem.set_f32(first + 0x1C, 3.0);
        let second = plain_effect(&mut e, 0);
        e.mem.set_f32(second + 0x20, 5.0);
        let r = magic_target_active_effect_list_compare_fn(&mut e, first, second);
        assert_eq!(r, -1);
        let keys = calls(&e, 0x00ec_623a);
        let mut expected = vec![keys[0][0], SORT_KEY_FORMAT, 0x41, 0x0102_0000];
        expected.extend(f64_words(3.0));
        expected.extend(f64_words(1000.0));
        assert_eq!(keys[0], expected);
        let mut expected = vec![keys[1][0], SORT_KEY_FORMAT, 0x41, 0x0102_0000];
        expected.extend(f64_words(0.0));
        expected.extend(f64_words(5.0));
        assert_eq!(keys[1], expected);
        assert_eq!(calls(&e, 0x0046_fd30), vec![vec![keys[0][0], keys[1][0]]]);
        assert_eq!(e.call(0x0082_5270, &args![second, first]).i32(), -1);
    }

    #[test]
    fn form_id_of_the_actor_or_the_stats_object() {
        let mut e = engine();
        let this = target(&mut e, true);
        e.mem.set_u32(this.addr() - 0x94 + 0x0C, 0x14);
        assert_eq!(magic_target_get_magic_target_form_id(&mut e, this), 0x14);
        let other = target(&mut e, false);
        let stats = e.mem.alloc(0x20);
        e.mem.set_u32(stats + 0x0C, 0x0001_2345);
        e.mem.set_u32(other.addr() + T_STATS, stats);
        assert_eq!(e.call(0x0082_54c0, &args![other]).u32(), 0x0001_2345);
    }

    #[test]
    fn magic_target_by_numeric_id_asks_the_form() {
        let mut e = engine();
        let form = e.mem.alloc(0x20);
        e.mem.set_u32(form, FORM_VTABLE);
        e.mem.set_u32(form + 4, 1);
        vtable(&mut e, FORM_VTABLE, FORM_FN, 0x1A0 / 4);
        e.register(FORM_FN + 0xF0, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(FORM_FN + 0x19C, |_, a| ret(a[0] + 0x10));
        e.register_double(LOOKUP_FORM_BY_ID, move |_, a| {
            ret(if a[0] == 7 { form } else { 0 })
        });
        assert_eq!(
            magic_target_get_magic_target_by_numeric_id(&mut e, 7),
            form + 0x10
        );
        assert_eq!(e.call(0x0082_5550, &args![8u32]).u32(), 0);
        e.mem.set_u32(form + 4, 0);
        assert_eq!(magic_target_get_magic_target_by_numeric_id(&mut e, 7), 0);
    }

    #[test]
    fn withdrawal_search_matches_type_10_and_the_displacement_spell() {
        let mut e = engine();
        let this = target(&mut e, false);
        let withdrawal = spell(&mut e, 10);
        let fx = plain_effect(&mut e, withdrawal);
        e.mem.set_u32(fx + 0x44, 0x88);
        e.mem.set_u8(fx + 0x10, 0);
        set_effects(&mut e, this, &[fx]);
        assert!(!fn_00825610(&mut e, this, 0x88, false));
        assert!(e.call(0x0082_5610, &args![this, 0x88u32, true]).bool());
        assert!(fn_00825610(&mut e, this, 0, true));
        assert!(!fn_00825610(&mut e, this, 0x89, true));
    }
}
