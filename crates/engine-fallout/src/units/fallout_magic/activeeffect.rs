//! `fallout/magic/activeeffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00804b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`activeeffect.cpp`). `cdecl`, one stack word: true (`AL`) when it
/// is 1, 4 or 10, false otherwise. The meaning of those values is unconfirmed.
pub fn fn_00804b30(_e: &mut Engine, value: i32) -> bool {
    value == 1 || value == 4 || value == 10
}

layout! {
    /// `ActiveEffect` (Xbox PDB), 0x48 bytes there; the PC offsets below are
    /// the ones `CopyData` (`00804090`) and `SaveGame` (`00806230`) use.
    pub struct ActiveEffect: 0x48 {
        /// `fElapsedSeconds` (Xbox PDB).
        0x04 fElapsedSeconds: f32,
        /// `pSpell` (Xbox PDB): `MagicItem*`.
        0x08 pSpell: u32,
        /// `pEffect` (Xbox PDB): `EffectItem*`.
        0x0C pEffect: u32,
        /// `bActive` (Xbox PDB).
        0x10 bActive: u8,
        /// `bStarted` (Xbox PDB).
        0x11 bStarted: u8,
        /// `bFinished` (Xbox PDB).
        0x12 bFinished: u8,
        /// `bDone` (Xbox PDB).
        0x13 bDone: u8,
        /// `bWornEnchantment` (Xbox PDB).
        0x14 bWornEnchantment: u8,
        /// `iFlags` (Xbox PDB).
        0x18 iFlags: u32,
        /// `fMagnitude` (Xbox PDB).
        0x1C fMagnitude: f32,
        /// `fDuration` (Xbox PDB).
        0x20 fDuration: f32,
        /// `pTarget` (Xbox PDB): `MagicTarget*`.
        0x24 pTarget: u32,
        /// `pCaster` (Xbox PDB): `MagicCaster*`.
        0x28 pCaster: u32,
        /// `eSpellType` (Xbox PDB).
        0x2C eSpellType: u32,
        /// `pSource` (Xbox PDB): `TESBoundObject*`.
        0x3C pSource: u32,
        /// `pHitEffects` (Xbox PDB): `BSSimpleList<MagicHitEffect*>*`.
        0x40 pHitEffects: u32,
        /// `pDisplacementSpell` (Xbox PDB): `MagicItem*`.
        0x44 pDisplacementSpell: u32,
    }
}

/// `PersistentSound` (Xbox PDB): the effect's `BSSoundHandle` at +0x30.
const PERSISTENT_SOUND: u32 = 0x30;

/// `operator new` (`00401000`, cdecl: size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `005ae3d0` (thiscall on the list, one word: the address of the item; unnamed in
/// the map): the list's insert in front of the head node.
const LIST_INSERT_FRONT_005AE3D0: u32 = 0x005a_e3d0;
/// `00470440` (thiscall on a new 8-byte node, one word: the address of the item;
/// unnamed): the node constructor.
const LIST_NODE_INIT_00470440: u32 = 0x0047_0440;
/// `007df210` (thiscall, returns `AL`; unnamed, filed under `statsmenu.cpp`).
const EFFECT_TEST_007DF210: u32 = 0x007d_f210;
/// `008041a0` (thiscall, returns a word; the map's name
/// `LowProcess::GetGenericLocation` comes from folded code).
const EFFECT_VALUE_008041A0: u32 = 0x0080_41a0;
/// `00803e80` (thiscall, one flag word; unnamed, `fallout/magic`).
const EFFECT_FLAG_00803E80: u32 = 0x0080_3e80;
/// `0059bb30` (thiscall, returns a pointer; the map's name `D3DTexture_LockRect`
/// comes from folded code): the effect's magic target object.
const EFFECT_TARGET_0059BB30: u32 = 0x0059_bb30;
/// `006815c0` (thiscall on a list node, returns the address of its item).
const LIST_NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070` (thiscall on a list node, returns the next node).
const LIST_NODE_NEXT: u32 = 0x0072_6070;
/// `00833c20` (thiscall on an item, returns `AL`; unnamed).
const ITEM_TEST_00833C20: u32 = 0x0083_3c20;
/// `__RTDynamicCast` (`00ec43fb`, cdecl: object, vfDelta, source type, target type, isReference).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `MagicItem`'s RTTI type descriptor (`.?AVMagicItem@@`).
const MAGIC_ITEM_TYPE: u32 = 0x0118_3140;
/// `SpellItem`'s RTTI type descriptor (`.?AVSpellItem@@`).
const SPELL_ITEM_TYPE: u32 = 0x0118_3060;
/// `AlchemyItem`'s RTTI type descriptor (`.?AVAlchemyItem@@`).
const ALCHEMY_ITEM_TYPE: u32 = 0x0118_30cc;
/// `004030b0` (thiscall on the `AlchemyItem`, returns a word; unnamed).
const ALCHEMY_VALUE_004030B0: u32 = 0x0040_30b0;
/// `00403e20` (thiscall, returns the address of a `float`; unnamed, `effectitem.cpp`).
const FLOAT_ADDRESS_00403E20: u32 = 0x0040_3e20;
/// The object `fn_008043f0` asks `00403e20` for its `float`.
const FLOAT_OWNER_011D17DC: u32 = 0x011d_17dc;
/// `006b9130` (thiscall, returns a `float` in `ST0`; unnamed).
const EFFECT_REAL_006B9130: u32 = 0x006b_9130;
/// `00406cc0` (cdecl, one `float`, returns `ST0`; unnamed).
const FLOAT_FUNCTION_00406CC0: u32 = 0x0040_6cc0;
/// `0084e3a0` (thiscall, returns a word; unnamed getter).
const GETTER_0084E3A0: u32 = 0x0084_e3a0;
/// `00409140` (thiscall, returns a pointer; unnamed, `enchantmentitem.cpp`).
const GETTER_00409140: u32 = 0x0040_9140;
/// `TESCondition::IsTrue` (`00680c30`, thiscall: two words).
const TES_CONDITION_IS_TRUE: u32 = 0x0068_0c30;
/// `0044ddc0` (thiscall, returns a pointer; unnamed getter).
const GETTER_0044DDC0: u32 = 0x0044_ddc0;
/// The player pointer (as other units name `011dea3c`).
const PLAYER_POINTER: u32 = 0x011d_ea3c;
/// `Interface::IsInMenuMode` (`00702360`, no arguments).
const INTERFACE_IS_IN_MENU_MODE: u32 = 0x0070_2360;
/// `007df1d0` (thiscall, returns a pointer; unnamed, filed under `statsmenu.cpp`).
const EFFECT_POINTER_007DF1D0: u32 = 0x007d_f1d0;
/// `00403e60` (thiscall, one flag word, returns `AL`; unnamed, `effectitem.cpp`).
const FLAG_TEST_00403E60: u32 = 0x0040_3e60;
/// `0040df90` (cdecl, one word; unnamed).
const WORD_FUNCTION_0040DF90: u32 = 0x0040_df90;
/// `00495460` (thiscall, returns a `float` in `ST0`; unnamed).
const EFFECT_REAL_00495460: u32 = 0x0049_5460;
/// `00470470` (thiscall on the hit-effect list; unnamed).
const HIT_LIST_00470470: u32 = 0x0047_0470;
/// `004702f0` (thiscall on the hit-effect list, one word; unnamed).
const HIT_LIST_004702F0: u32 = 0x0047_02f0;
/// `008c7aa0` (no arguments, returns `AL`; unnamed).
const TEST_008C7AA0: u32 = 0x008c_7aa0;
/// `004537b0` (no arguments, returns a pointer; unnamed, `tes.cpp`).
const GETTER_004537B0: u32 = 0x0045_37b0;
/// `0087aec0` (thiscall, one word; unnamed, `taskqueueinterface.cpp`).
const TASK_QUEUE_0087AEC0: u32 = 0x0087_aec0;
/// `MagicHitEffect::InstantiateHitEffects` (`0081b8e0`, cdecl: the effect).
const MAGIC_HIT_EFFECT_INSTANTIATE: u32 = 0x0081_b8e0;
/// `007df1f0` (thiscall, returns a `float` in `ST0`; unnamed).
const EFFECT_REAL_007DF1F0: u32 = 0x007d_f1f0;
/// `00825c00` (thiscall, returns a pointer; unnamed getter, `fallout/magic`).
const GETTER_00825C00: u32 = 0x0082_5c00;
/// `BSSoundHandle::IsValid` (`00ad8ce0`).
const BS_SOUND_HANDLE_IS_VALID: u32 = 0x00ad_8ce0;
/// `BSSoundHandle::Stop` (`00ad88f0`).
const BS_SOUND_HANDLE_STOP: u32 = 0x00ad_88f0;
/// `BSSoundHandle::Release` (`00ad8d10`).
const BS_SOUND_HANDLE_RELEASE: u32 = 0x00ad_8d10;
/// `BSSoundHandle::SetPosition` (`00ad8b60`, three `float`s).
const BS_SOUND_HANDLE_SET_POSITION: u32 = 0x00ad_8b60;
/// `BSSoundHandle::Play` (`00ad8830`, one word).
const BS_SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
/// `BSAudio::QInstance` (`00453a70`).
const BS_AUDIO_Q_INSTANCE: u32 = 0x0045_3a70;
/// `BSAudio::GetSoundHandleByNumericID` (`00ad73b0`, thiscall: handle, id, flags).
const BS_AUDIO_GET_SOUND_HANDLE_BY_NUMERIC_ID: u32 = 0x00ad_73b0;
/// `00408680` (thiscall, one flag word, returns `AL`; unnamed, `effectsetting.cpp`).
const FLAG_TEST_00408680: u32 = 0x0040_8680;
/// `00404040` (cdecl, one `float`, returns `ST0`; unnamed, `effectitem.cpp`).
const FLOAT_FUNCTION_00404040: u32 = 0x0040_4040;
/// `00693d50` (thiscall, one `float`; unnamed).
const SET_REAL_00693D50: u32 = 0x0069_3d50;
/// `004fd3e0` (thiscall, returns a pointer; the map's name `TES::GetWorldSpace`
/// may come from folded code).
const GETTER_004FD3E0: u32 = 0x004f_d3e0;
/// `0041a250` (thiscall: constructs a sound handle; unnamed, `extradatalist.cpp`).
const SOUND_HANDLE_INIT_0041A250: u32 = 0x0041_a250;
/// `MagicTarget::GetTargetAsActor` (`00822b40`).
const MAGIC_TARGET_GET_TARGET_AS_ACTOR: u32 = 0x0082_2b40;
/// `004e75d0` (thiscall, one word: a 0x24-byte buffer; returns a pointer).
const FILL_004E75D0: u32 = 0x004e_75d0;
/// `005e39b0` (cdecl, one word; unnamed).
const WORD_FUNCTION_005E39B0: u32 = 0x005e_39b0;
/// `00933150` (thiscall on the actor, five words; unnamed, `mobileobject.cpp`).
const ACTOR_SOUND_00933150: u32 = 0x0093_3150;
/// `00418900` (thiscall, one word: assigns a sound handle; unnamed).
const SOUND_HANDLE_ASSIGN_00418900: u32 = 0x0041_8900;
/// `00483710` (thiscall: the sound handle's destructor here; unnamed).
const SOUND_HANDLE_DESTROY_00483710: u32 = 0x0048_3710;
/// `00865e50` (thiscall on the save buffer: data, size, word; unnamed).
const SAVE_DATA_00865E50: u32 = 0x0086_5e50;
/// `BGSSaveGameBuffer::SaveFormID` (`00865db0`: id, word).
const SAVE_FORM_ID: u32 = 0x0086_5db0;
/// `BGSSaveGameBuffer::SaveFormID_ov2` (`00865df0`: form, word).
const SAVE_FORM_ID_OV2: u32 = 0x0086_5df0;
/// `BGSSaveGameBuffer::StartVariableSizedValue` (`00865f20`).
const START_VARIABLE_SIZED_VALUE: u32 = 0x0086_5f20;
/// `BGSSaveGameBuffer::SaveVariableSizedValue` (`00865f60`: value).
const SAVE_VARIABLE_SIZED_VALUE: u32 = 0x0086_5f60;
/// `BGSSaveGameBuffer::SaveVariableSizedValue_ov2` (`00865ff0`: value, handle).
const SAVE_VARIABLE_SIZED_VALUE_OV2: u32 = 0x0086_5ff0;
/// `MagicCaster::GetMagicCasterFormID` (`00818be0`).
const MAGIC_CASTER_GET_FORM_ID: u32 = 0x0081_8be0;
/// `MagicTarget::GetMagicTargetFormID` (`008254c0`).
const MAGIC_TARGET_GET_FORM_ID: u32 = 0x0082_54c0;
/// `MagicItem::GetMagicItemFormID` (`0040a1e0`).
const MAGIC_ITEM_GET_FORM_ID: u32 = 0x0040_a1e0;
/// `EffectItemList::GetEffectItemIndex` (`00406e70`, thiscall: the effect item).
const EFFECT_ITEM_LIST_GET_INDEX: u32 = 0x0040_6e70;
/// `00810fd0` (thiscall, returns a word; unnamed).
const GETTER_00810FD0: u32 = 0x0081_0fd0;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `1.0` (`double`).
const ONE: u32 = 0x0101_2070;
/// `100.0` (`double`).
const HUNDRED: u32 = 0x0101_7a40;

// Translated from 007a7eb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a7eb0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `BSSimpleList<ActiveEffect *>::Insert` (Xbox PDB): inserts `item` (nothing when null)
/// into the list whose head node is `this`, before the first node whose item `compare`
/// (cdecl: new item, node item) does not put it after (result `<= 0`), or at the end. An
/// empty head node takes the item itself. A new node is 8 bytes (`operator new`, then
/// `00470440` with the address of the item); a null allocation links a null node.
/// Insertion before the head goes through `005ae3d0`.
pub fn bs_simple_list_active_effect_insert(e: &mut Engine, this: Ptr, item: u32, compare: u32) {
    if item == 0 {
        return;
    }
    let list = this.addr();
    e.with_stack(4, |e, item_slot| {
        e.mem.set_u32(item_slot.addr(), item);
        let mut previous = 0u32;
        let mut done = false;
        let mut node = list;
        while node != 0 && !done {
            if e.mem.u32(node) == 0 {
                e.mem.set_u32(node, item);
                done = true;
            } else {
                let current = e.mem.u32(node);
                if e.call(compare, &args![item, current]).i32() <= 0 {
                    if previous == 0 {
                        e.call(LIST_INSERT_FRONT_005AE3D0, &args![list, item_slot]);
                    } else {
                        let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
                        let new_node = if block == 0 {
                            0
                        } else {
                            e.call(LIST_NODE_INIT_00470440, &args![block, item_slot])
                                .u32()
                        };
                        e.mem.set_u32(previous + 4, new_node);
                        e.mem.set_u32(new_node.wrapping_add(4), node);
                    }
                    done = true;
                } else if e.mem.u32(node + 4) == 0 {
                    let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
                    let new_node = if block == 0 {
                        0
                    } else {
                        e.call(LIST_NODE_INIT_00470440, &args![block, item_slot])
                            .u32()
                    };
                    e.mem.set_u32(node + 4, new_node);
                    done = true;
                }
            }
            previous = node;
            node = e.mem.u32(node + 4);
        }
    });
}

// Translated from 00804090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActiveEffect::CopyData` (Xbox PDB): copies the effect's state into `target`: the
/// elapsed seconds, spell, effect item, the five state bytes, magnitude, duration,
/// target, caster, spell type, source, flags and displacement spell. The three floats
/// go through `FLD`/`FSTP` (a signalling NaN arrives quiet).
pub fn active_effect_copy_data(e: &mut Engine, this: Ptr<ActiveEffect>, target: Ptr<ActiveEffect>) {
    let from = this.addr();
    let to = target.addr();
    let quiet = |e: &mut Engine, offset: u32| {
        let bits = x87_quiet_bits(e.mem.u32(from + offset));
        e.mem.set_u32(to + offset, bits);
    };
    let word = |e: &mut Engine, offset: u32| {
        let value = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, value);
    };
    let byte = |e: &mut Engine, offset: u32| {
        let value = e.mem.u8(from + offset);
        e.mem.set_u8(to + offset, value);
    };
    quiet(e, 0x04);
    word(e, 0x08);
    word(e, 0x0c);
    for offset in 0x10..=0x14 {
        byte(e, offset);
    }
    quiet(e, 0x1c);
    quiet(e, 0x20);
    word(e, 0x24);
    word(e, 0x28);
    word(e, 0x2c);
    word(e, 0x3c);
    word(e, 0x18);
    word(e, 0x44);
}

// Translated from 00804160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActiveEffect::Finalize` (Xbox PDB): when `007df210` is true or `008041a0` is
/// non-zero, re-evaluates the effect's conditions at once (`fn_008043f0` with 0 seconds
/// and `force` set).
pub fn active_effect_finalize(e: &mut Engine, this: Ptr<ActiveEffect>) {
    if e.call(EFFECT_TEST_007DF210, &args![this]).bool()
        || e.call(EFFECT_VALUE_008041A0, &args![this]).u32() != 0
    {
        fn_008043f0(e, this, 0.0, true);
    }
}

// Translated from 00804210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActiveEffect::Dispel` (Xbox PDB): sets `bDone`, calls `00803e80` with 0x200 and,
/// when `now` is set, runs the update (`fn_00804560`) with 0 seconds.
pub fn active_effect_dispel(e: &mut Engine, this: Ptr<ActiveEffect>, now: bool) {
    e.set(this, ActiveEffect::bDone, 1);
    e.call(EFFECT_FLAG_00803E80, &args![this, 0x200u32]);
    if now {
        fn_00804560(e, this, 0.0);
    }
}

// Translated from 00804250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`activeeffect.cpp`): whether one of the target's own effects (the list the
/// target object's slot +8 returns) blocks this one: an effect (`00833c20` true) whose
/// spell is this effect's displacement spell, or whose spell is an `AlchemyItem` (type 7
/// from its slot +0x18) whose `004030b0` word is this effect's spell as a `SpellItem`
/// (`__RTDynamicCast`). Only looked at when the displacement spell is set. The answer is
/// false anyway when the target is an actor (target slot +0xc true, the actor at
/// `target - 0x94`) whose slot +8 of the object at +0xa4 returns more than 0 for 0x48.
pub fn fn_00804250(e: &mut Engine, this: Ptr<ActiveEffect>) -> bool {
    let mut found = false;
    let mut excluded = false;
    let target = e.get(this, ActiveEffect::pTarget);
    if e.vcall(target, 0xc, &[]).bool() {
        let actor = if target == 0 {
            0
        } else {
            target.wrapping_sub(0x94)
        };
        if actor != 0 && e.vcall(actor + 0xa4, 8, &args![0x48u32]).i32() > 0 {
            excluded = true;
        }
    }
    let spell = e.get(this, ActiveEffect::pSpell);
    let as_spell = e
        .call(
            RT_DYNAMIC_CAST,
            &args![spell, 0u32, MAGIC_ITEM_TYPE, SPELL_ITEM_TYPE, 0u32],
        )
        .u32();
    if e.get(this, ActiveEffect::pDisplacementSpell) != 0
        && e.call(EFFECT_TARGET_0059BB30, &args![this]).u32() != 0
    {
        let target_object = e.call(EFFECT_TARGET_0059BB30, &args![this]).u32();
        let mut node = e.vcall(target_object, 8, &[]).u32();
        while node != 0 && !found {
            let slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
            let item = e.mem.u32(slot);
            if item != 0 && e.call(ITEM_TEST_00833C20, &args![item]).bool() {
                let item_spell = e.mem.u32(item + 8);
                if item_spell == e.get(this, ActiveEffect::pDisplacementSpell) {
                    found = true;
                } else if e.vcall(item_spell, 0x18, &[]).u32() == 7 {
                    let alchemy = e
                        .call(
                            RT_DYNAMIC_CAST,
                            &args![item_spell, 0u32, MAGIC_ITEM_TYPE, ALCHEMY_ITEM_TYPE, 0u32],
                        )
                        .u32();
                    if alchemy != 0
                        && e.call(ALCHEMY_VALUE_004030B0, &args![alchemy]).u32() == as_spell
                    {
                        found = true;
                    }
                }
            }
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
        }
    }
    if found && excluded {
        found = false;
    }
    found
}

// Translated from 008043f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`activeeffect.cpp`): re-evaluates whether the effect is active. Only when
/// `007df210` is true or `008041a0` is non-zero, the target object (`0059bb30`) is set
/// and its slot +4 returns non-zero. Unless `force` is set, it does so only when the
/// step crosses a boundary: `00406cc0` of `006b9130 / s` and of
/// `(006b9130 + seconds) / s` differ (`s` the `float` `00403e20` gives for `011d17dc`).
/// Then the effect is active when its condition (`TESCondition::IsTrue` on
/// `00409140(0084e3a0(this))` with the target object's slot +4 result and 0) holds and
/// `fn_00804250` does not block it; an effect that becomes active again after it
/// finished (and is not done) clears `bStarted` and `bFinished`.
pub fn fn_008043f0(e: &mut Engine, this: Ptr<ActiveEffect>, seconds: f32, force: bool) {
    if !(e.call(EFFECT_TEST_007DF210, &args![this]).bool()
        || e.call(EFFECT_VALUE_008041A0, &args![this]).u32() != 0)
    {
        return;
    }
    let target_object = e.call(EFFECT_TARGET_0059BB30, &args![this]).u32();
    if target_object == 0 || e.vcall(target_object, 4, &[]).u32() == 0 {
        return;
    }
    let scale_address = e
        .call(FLOAT_ADDRESS_00403E20, &args![FLOAT_OWNER_011D17DC])
        .u32();
    let scale = x87_quiet_bits(e.mem.u32(scale_address));
    let changed = if force {
        true
    } else {
        let now = e.call(EFFECT_REAL_006B9130, &args![this]).f64();
        let quotient = x87_store32(x87_div(now, x87_m32(scale)));
        let first = e
            .call(FLOAT_FUNCTION_00406CC0, &args![x87_quiet_bits(quotient)])
            .f64();
        let now = e.call(EFFECT_REAL_006B9130, &args![this]).f64();
        let later = x87_add(now, x87_m32(seconds.to_bits()));
        let quotient = x87_store32(x87_div(later, x87_m32(scale)));
        let second = e
            .call(FLOAT_FUNCTION_00406CC0, &args![x87_quiet_bits(quotient)])
            .f64();
        // FSTP/FLD double, then FUCOMPP: equal only when ordered and equal.
        let first = x87_load64(first.to_bits());
        !(first == second)
    };
    if !changed {
        return;
    }
    let value = e.vcall(target_object, 4, &[]).u32();
    let owner = e.call(GETTER_0084E3A0, &args![this]).u32();
    let condition = e.call(GETTER_00409140, &args![owner]).u32();
    let holds = e
        .call(TES_CONDITION_IS_TRUE, &args![condition, value, 0u32])
        .u8();
    let blocked = fn_00804250(e, this);
    let active = holds & u8::from(!blocked);
    if active != 0
        && e.get(this, ActiveEffect::bActive) == 0
        && e.get(this, ActiveEffect::bFinished) != 0
        && e.get(this, ActiveEffect::bDone) == 0
    {
        e.set(this, ActiveEffect::bStarted, 0);
        e.set(this, ActiveEffect::bFinished, 0);
    }
    e.set(this, ActiveEffect::bActive, active);
}

/// Releases the effect's hit-effect list (`00470470`, then `004702f0` with 1) and
/// clears the pointer, as `fn_00804560` does twice.
fn release_hit_effects(e: &mut Engine, this: Ptr<ActiveEffect>) {
    let list = e.get(this, ActiveEffect::pHitEffects);
    if list != 0 {
        e.call(HIT_LIST_00470470, &args![list]);
        let list = e.get(this, ActiveEffect::pHitEffects);
        if list != 0 {
            e.call(HIT_LIST_004702F0, &args![list, 1u32]);
        }
        e.set(this, ActiveEffect::pHitEffects, 0);
    }
}

/// Whether the effect's target reference is the player and the interface is in
/// menu mode, as `fn_00804560` tests it.
fn player_in_menu_mode(e: &mut Engine, target_reference: u32) -> bool {
    target_reference == e.global::<u32>(PLAYER_POINTER)
        && e.call(INTERFACE_IS_IN_MENU_MODE, &[]).bool()
}

// Translated from 00804560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`activeeffect.cpp`; the effect's per-frame update): advances the effect by
/// `seconds`. It first re-evaluates the conditions (`fn_008043f0`, when `007df210` or
/// `008041a0` allow). With `seconds > 0` and the effect not done: an active effect that
/// has neither started nor finished starts (sound through `fn_00804cd0`, or flag 0x20
/// for the player in menu mode; `Start`, slot +0x50; then, unless flag 1 or the effect
/// setting's flag 0x8000000 says otherwise, flag 4 for spell type 4, the hit effects
/// (flag 0x40 for the player in menu mode), all as the exe tests them) and is marked
/// started. The elapsed time grows by `seconds`, clamped to the duration for an effect
/// with a duration (`fn_00804b30` false and flag 0x100 clear). An active effect runs
/// the delayed hit effects and sound (flags 0x40 and 0x20) outside menu mode and its
/// `Update` (slot +0xc). It is done when the effect setting has flag 0x80 or the
/// elapsed time reached the duration, or when the target actor's slot +0x22c is true
/// and the setting lacks flag 0x10000000. A started effect that is done or inactive
/// and not finished then finishes (`Finish`, slot +0x58) and stops its persistent sound.
pub fn fn_00804560(e: &mut Engine, this: Ptr<ActiveEffect>, seconds: f32) {
    let base = this.addr();
    if e.call(EFFECT_TEST_007DF210, &args![this]).bool()
        || e.call(EFFECT_VALUE_008041A0, &args![this]).u32() != 0
    {
        fn_008043f0(e, this, seconds, false);
    }
    'update: {
        let zero: f64 = e.global(ZERO);
        // FCOMP; TEST AH,0x41: skipped when below, equal or unordered.
        if !(x87_load32(seconds.to_bits()) > zero) || e.get(this, ActiveEffect::bDone) != 0 {
            break 'update;
        }
        let owner = e.call(GETTER_0044DDC0, &args![this]).u32();
        let target_object = e.call(EFFECT_TARGET_0059BB30, &args![this]).u32();
        let target_reference = e.vcall(target_object, 4, &[]).u32();
        let spell_type = e.get(this, ActiveEffect::eSpellType) as i32;
        let no_duration = fn_00804b30(e, spell_type);
        if e.get(this, ActiveEffect::bStarted) == 0
            && e.get(this, ActiveEffect::bFinished) == 0
            && e.get(this, ActiveEffect::bActive) != 0
        {
            if player_in_menu_mode(e, target_reference) {
                e.call(EFFECT_FLAG_00803E80, &args![this, 0x20u32]);
            } else {
                fn_00804cd0(e, this);
            }
            e.vcall(base, 0x50, &[]);
            if e.get(this, ActiveEffect::bDone) == 0 && e.get(this, ActiveEffect::bFinished) == 0 {
                'start: {
                    if fn_00804b80(e, this, 1) {
                        break 'start;
                    }
                    let setting = e.call(EFFECT_POINTER_007DF1D0, &args![this]).u32();
                    if e.call(FLAG_TEST_00403E60, &args![setting, 0x0800_0000u32])
                        .bool()
                    {
                        break 'start;
                    }
                    if e.get(this, ActiveEffect::eSpellType) == 4 {
                        let setting = e.call(EFFECT_POINTER_007DF1D0, &args![this]).u32();
                        if e.call(WORD_FUNCTION_0040DF90, &args![setting]).u32() == 0 {
                            break 'start;
                        }
                        let value = e.call(EFFECT_REAL_00495460, &args![this]).f64();
                        let hundred: f64 = e.global(HUNDRED);
                        // FCOMP; TEST AH,5; JP: skipped unless below.
                        if !(value < hundred) {
                            break 'start;
                        }
                    }
                    if e.get(this, ActiveEffect::eSpellType) == 4 {
                        e.call(EFFECT_FLAG_00803E80, &args![this, 4u32]);
                    }
                    let first = if e.vcall(owner, 0x18, &[]).u32() == 8 {
                        if owner == 0 {
                            0
                        } else {
                            owner.wrapping_sub(0x30)
                        }
                    } else {
                        0
                    };
                    let second = if e.vcall(owner, 0x18, &[]).u32() == 7 {
                        if owner == 0 {
                            0
                        } else {
                            owner.wrapping_sub(0x30)
                        }
                    } else {
                        0
                    };
                    let skip = if first != 0 {
                        e.vcall(first + 0x3c, 4, &[]).u8()
                    } else if second != 0 {
                        e.vcall(second + 0x3c, 4, &[]).u8()
                    } else {
                        0
                    };
                    if skip != 0 {
                        break 'start;
                    }
                    if player_in_menu_mode(e, target_reference) {
                        e.call(EFFECT_FLAG_00803E80, &args![this, 0x40u32]);
                        break 'start;
                    }
                    release_hit_effects(e, this);
                    if e.call(TEST_008C7AA0, &[]).bool() {
                        let queue = e.call(GETTER_004537B0, &[]).u32();
                        e.call(TASK_QUEUE_0087AEC0, &args![queue, this]);
                    } else {
                        let hit_effects = e.call(MAGIC_HIT_EFFECT_INSTANTIATE, &args![this]).u32();
                        e.set(this, ActiveEffect::pHitEffects, hit_effects);
                    }
                }
                e.set(this, ActiveEffect::bStarted, 1);
            }
        }
        if e.get(this, ActiveEffect::bDone) != 0 {
            break 'update;
        }
        let mut step = x87_quiet_bits(seconds.to_bits());
        let mut clamped = false;
        if !no_duration && !fn_00804b60(e, this) {
            let now = e.call(EFFECT_REAL_006B9130, &args![this]).f64();
            let later = x87_add(now, x87_m32(step));
            let duration = e.call(EFFECT_REAL_007DF1F0, &args![this]).f64();
            // FCOMP; TEST AH,5; JP: clamped only when the duration is below.
            if duration < later {
                let duration = e.call(EFFECT_REAL_007DF1F0, &args![this]).f64();
                let now = e.call(EFFECT_REAL_006B9130, &args![this]).f64();
                step = x87_store32(x87_sub(duration, now));
                let duration = e.call(EFFECT_REAL_007DF1F0, &args![this]).f64();
                e.mem.set_u32(base + 4, x87_store32(duration));
                clamped = true;
            }
        }
        if !clamped {
            let elapsed = x87_load32(e.mem.u32(base + 4));
            let elapsed = x87_add(elapsed, x87_m32(seconds.to_bits()));
            e.mem.set_u32(base + 4, x87_store32(elapsed));
        }
        if e.get(this, ActiveEffect::bActive) != 0 {
            let hit_effects_due = fn_00804b80(e, this, 0x40);
            let sound_due = fn_00804b80(e, this, 0x20);
            if (hit_effects_due || sound_due)
                && target_reference == e.global::<u32>(PLAYER_POINTER)
                && !e.call(INTERFACE_IS_IN_MENU_MODE, &[]).bool()
            {
                if hit_effects_due {
                    release_hit_effects(e, this);
                    let hit_effects = e.call(MAGIC_HIT_EFFECT_INSTANTIATE, &args![this]).u32();
                    e.set(this, ActiveEffect::pHitEffects, hit_effects);
                    fn_00804ba0(e, this, 0x40);
                }
                if sound_due {
                    fn_00804cd0(e, this);
                    fn_00804ba0(e, this, 0x20);
                }
            }
            e.vcall(base, 0xc, &args![x87_quiet_bits(step)]);
        }
        if !no_duration && !fn_00804b60(e, this) {
            let effect = e.get(this, ActiveEffect::pEffect);
            let setting = e.call(GETTER_00825C00, &args![effect]).u32();
            if e.call(FLAG_TEST_00403E60, &args![setting, 0x80u32]).bool() {
                e.set(this, ActiveEffect::bDone, 1);
            } else {
                let duration = e.call(EFFECT_REAL_007DF1F0, &args![this]).f64();
                let elapsed = x87_load32(e.mem.u32(base + 4));
                // FCOMPP; TEST AH,1: done unless below or unordered.
                if !(elapsed < duration || elapsed.is_nan() || duration.is_nan()) {
                    e.set(this, ActiveEffect::bDone, 1);
                }
            }
        }
        let target = e.get(this, ActiveEffect::pTarget);
        if e.vcall(target, 0xc, &[]).bool() {
            let target = e.get(this, ActiveEffect::pTarget);
            let actor = if target == 0 {
                0
            } else {
                target.wrapping_sub(0x94)
            };
            if actor != 0 && e.vcall(actor, 0x22c, &args![1u32]).bool() {
                let setting = e.call(EFFECT_POINTER_007DF1D0, &args![this]).u32();
                if !e
                    .call(FLAG_TEST_00403E60, &args![setting, 0x1000_0000u32])
                    .bool()
                {
                    e.set(this, ActiveEffect::bDone, 1);
                }
            }
        }
    }
    if e.get(this, ActiveEffect::bStarted) != 0
        && (e.get(this, ActiveEffect::bDone) != 0 || e.get(this, ActiveEffect::bActive) == 0)
        && e.get(this, ActiveEffect::bFinished) == 0
    {
        e.vcall(base, 0x58, &[]);
        e.set(this, ActiveEffect::bFinished, 1);
        if e.call(BS_SOUND_HANDLE_IS_VALID, &args![base + PERSISTENT_SOUND])
            .bool()
        {
            e.call(BS_SOUND_HANDLE_STOP, &args![base + PERSISTENT_SOUND]);
        }
    }
}

// Translated from 00804b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: whether flag 0x100 of `iFlags` is set.
pub fn fn_00804b60(e: &mut Engine, this: Ptr<ActiveEffect>) -> bool {
    e.get(this, ActiveEffect::iFlags) & 0x100 != 0
}

// Translated from 00804b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: whether any of `mask` is set in `iFlags`.
pub fn fn_00804b80(e: &mut Engine, this: Ptr<ActiveEffect>, mask: u32) -> bool {
    e.get(this, ActiveEffect::iFlags) & mask != 0
}

// Translated from 00804ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: clears `mask` in `iFlags`.
pub fn fn_00804ba0(e: &mut Engine, this: Ptr<ActiveEffect>, mask: u32) {
    let flags = e.get(this, ActiveEffect::iFlags);
    e.set(this, ActiveEffect::iFlags, !mask & flags);
}

// Translated from 00804bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActiveEffect::AdjustEffectiveness` (Xbox PDB): scales the effect by `factor`
/// unless the effect setting (`00825c00(0084e3a0(this))`) has flag 0x180 by
/// `00408680`, or `factor` is 1. With the setting's flag 0x100 (`00403e60`) it passes
/// `007df1f0 * factor` to `00693d50`; otherwise the magnitude becomes
/// `00404040(00495460 * factor)`, at least 1.
pub fn active_effect_adjust_effectiveness(e: &mut Engine, this: Ptr<ActiveEffect>, factor: f32) {
    let owner = e.call(GETTER_0084E3A0, &args![this]).u32();
    let setting = e.call(GETTER_00825C00, &args![owner]).u32();
    if e.call(FLAG_TEST_00408680, &args![setting, 0x180u32]).bool() {
        return;
    }
    let one: f64 = e.global(ONE);
    // FCOMP; TEST AH,0x44; JP: returns only when equal.
    if x87_load32(factor.to_bits()) == one {
        return;
    }
    let owner = e.call(GETTER_0084E3A0, &args![this]).u32();
    let setting = e.call(GETTER_00825C00, &args![owner]).u32();
    if e.call(FLAG_TEST_00403E60, &args![setting, 0x100u32]).bool() {
        let value = e.call(EFFECT_REAL_007DF1F0, &args![this]).f64();
        let scaled = x87_store32(x87_mul(value, x87_m32(factor.to_bits())));
        e.call(SET_REAL_00693D50, &args![this, x87_quiet_bits(scaled)]);
        return;
    }
    let value = e.call(EFFECT_REAL_00495460, &args![this]).f64();
    let scaled = x87_store32(x87_mul(value, x87_m32(factor.to_bits())));
    let result = e
        .call(FLOAT_FUNCTION_00404040, &args![x87_quiet_bits(scaled)])
        .f64();
    // FCOMP; TEST AH,5; JP: 1.0 only when below.
    let magnitude = if result < one {
        x87_store32(1.0)
    } else {
        let value = e.call(EFFECT_REAL_00495460, &args![this]).f64();
        let scaled = x87_store32(x87_mul(value, x87_m32(factor.to_bits())));
        let result = e
            .call(FLOAT_FUNCTION_00404040, &args![x87_quiet_bits(scaled)])
            .f64();
        x87_store32(result)
    };
    fn_00804cb0(e, this, f32::from_bits(x87_quiet_bits(magnitude)));
}

// Translated from 00804cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: stores `magnitude` in `fMagnitude` (`FLD`/`FSTP`: a signalling NaN
/// arrives quiet).
pub fn fn_00804cb0(e: &mut Engine, this: Ptr<ActiveEffect>, magnitude: f32) {
    e.mem
        .set_u32(this.addr() + 0x1c, x87_quiet_bits(magnitude.to_bits()));
}

// Translated from 00804cd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00804cd0 - the exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed (`activeeffect.cpp`): starts the effect's sound. The sound id is
/// `0084e3a0(004fd3e0(007df1d0(this)))` (none when `004fd3e0` returns null); nothing
/// happens without it, without a target object, or when the effect's owner
/// (`0044ddc0`) is of type 4 (slot +0x18). It is looping when the setting has flag 0x400
/// (`00403e60`), or, when `004fd3e0` is set, when `005e39b0` of the word at +4 of the
/// result of `004e75d0` is non-zero. An actor target plays it through `00933150` (flags
/// 0x112 looping, 0x102 not); another target through `BSAudio`, at the position its
/// reference's slot +0x1f4 gives. A looping sound replaces the effect's persistent sound.
pub fn fn_00804cd0(e: &mut Engine, this: Ptr<ActiveEffect>) {
    let base = this.addr();
    // The frame below `EBP` (0x80 bytes): `local(n)` is `EBP - n`.
    e.with_stack(0x80, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x80 - offset;
        let pointer = e.call(EFFECT_POINTER_007DF1D0, &args![this]).u32();
        let sound_id = if e.call(GETTER_004FD3E0, &args![pointer]).u32() == 0 {
            0
        } else {
            let pointer = e.call(EFFECT_POINTER_007DF1D0, &args![this]).u32();
            let object = e.call(GETTER_004FD3E0, &args![pointer]).u32();
            e.call(GETTER_0084E3A0, &args![object]).u32()
        };
        if sound_id == 0 || e.call(EFFECT_TARGET_0059BB30, &args![this]).u32() == 0 {
            return;
        }
        let owner = e.call(GETTER_0044DDC0, &args![this]).u32();
        if e.vcall(owner, 0x18, &[]).u32() == 4 {
            return;
        }
        let sound = local(0x24);
        e.call(SOUND_HANDLE_INIT_0041A250, &args![sound]);
        let actor = if e.call(EFFECT_TARGET_0059BB30, &args![this]).u32() == 0 {
            0
        } else {
            let target_object = e.call(EFFECT_TARGET_0059BB30, &args![this]).u32();
            e.call(MAGIC_TARGET_GET_TARGET_AS_ACTOR, &args![target_object])
                .u32()
        };
        let setting = e.call(EFFECT_POINTER_007DF1D0, &args![this]).u32();
        let mut looping = e.call(FLAG_TEST_00403E60, &args![setting, 0x400u32]).u8();
        let pointer = e.call(EFFECT_POINTER_007DF1D0, &args![this]).u32();
        if e.call(GETTER_004FD3E0, &args![pointer]).u32() != 0 {
            let buffer = local(0x68);
            let pointer = e.call(EFFECT_POINTER_007DF1D0, &args![this]).u32();
            let object = e.call(GETTER_004FD3E0, &args![pointer]).u32();
            let result = e.call(FILL_004E75D0, &args![object, buffer]).u32();
            let word = e.mem.u32(result + 4);
            looping = u8::from(e.call(WORD_FUNCTION_005E39B0, &args![word]).u32() != 0);
        }
        if actor != 0 {
            let flags: u32 = if looping != 0 { 0x112 } else { 0x102 };
            let handle = local(0x74);
            let result = e
                .call(
                    ACTOR_SOUND_00933150,
                    &args![actor, handle, sound_id, u32::from(looping), flags, 0u32],
                )
                .u32();
            e.call(SOUND_HANDLE_ASSIGN_00418900, &args![sound, result]);
            e.call(SOUND_HANDLE_DESTROY_00483710, &args![handle]);
        } else {
            let target_object = e.call(EFFECT_TARGET_0059BB30, &args![this]).u32();
            let reference = e.vcall(target_object, 4, &[]).u32();
            if reference != 0 {
                let handle = local(0x44);
                let audio = e.call(BS_AUDIO_Q_INSTANCE, &[]).u32();
                e.call(
                    BS_AUDIO_GET_SOUND_HANDLE_BY_NUMERIC_ID,
                    &args![audio, handle, sound_id, 0x102u32],
                );
                let position = e.vcall(reference, 0x1f4, &[]).u32();
                let x = e.mem.u32(position);
                let y = e.mem.u32(position + 4);
                let z = e.mem.u32(position + 8);
                e.call(
                    BS_SOUND_HANDLE_SET_POSITION,
                    &args![
                        handle,
                        x87_quiet_bits(x),
                        x87_quiet_bits(y),
                        x87_quiet_bits(z)
                    ],
                );
                e.call(BS_SOUND_HANDLE_PLAY, &args![handle, u32::from(looping)]);
                e.call(SOUND_HANDLE_DESTROY_00483710, &args![handle]);
            }
        }
        if looping != 0 {
            let persistent = base + PERSISTENT_SOUND;
            if e.call(BS_SOUND_HANDLE_IS_VALID, &args![persistent]).bool() {
                e.call(BS_SOUND_HANDLE_STOP, &args![persistent]);
                e.call(BS_SOUND_HANDLE_RELEASE, &args![persistent]);
            }
            e.call(SOUND_HANDLE_ASSIGN_00418900, &args![persistent, sound]);
        }
        e.call(SOUND_HANDLE_DESTROY_00483710, &args![sound]);
    });
}

// Translated from 00806200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActiveEffect::CheckDeletedCaster` (Xbox PDB): when `caster` is the effect's caster,
/// clears it and returns `true`.
pub fn active_effect_check_deleted_caster(
    e: &mut Engine,
    this: Ptr<ActiveEffect>,
    caster: u32,
) -> bool {
    if caster == e.get(this, ActiveEffect::pCaster) {
        e.set(this, ActiveEffect::pCaster, 0);
        true
    } else {
        false
    }
}

// Translated from 00806230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActiveEffect::SaveGame_ov2` (Xbox PDB): writes the effect to the save buffer: the
/// elapsed seconds, the five state bytes, magnitude, duration and flags (`00865e50`),
/// the caster's, target's and displacement spell's form ids (0 when unset), the source
/// form, and the hit effects as a variable-sized value: for each, its type byte (slot
/// +0x9c) and its own save (slot +0xa8 with the buffer); the count closes the value.
pub fn active_effect_save_game_ov2(e: &mut Engine, this: Ptr<ActiveEffect>, buffer: u32) {
    let base = this.addr();
    for (offset, size) in [
        (0x04u32, 4u32),
        (0x10, 1),
        (0x11, 1),
        (0x12, 1),
        (0x13, 1),
        (0x14, 1),
        (0x1c, 4),
        (0x20, 4),
        (0x18, 4),
    ] {
        e.call(
            SAVE_DATA_00865E50,
            &args![buffer, base + offset, size, 0u32],
        );
    }
    let caster = e.get(this, ActiveEffect::pCaster);
    let id = if caster == 0 {
        0
    } else {
        e.call(MAGIC_CASTER_GET_FORM_ID, &args![caster]).u32()
    };
    e.call(SAVE_FORM_ID, &args![buffer, id, 0u32]);
    let target = e.get(this, ActiveEffect::pTarget);
    let id = if target == 0 {
        0
    } else {
        e.call(MAGIC_TARGET_GET_FORM_ID, &args![target]).u32()
    };
    e.call(SAVE_FORM_ID, &args![buffer, id, 0u32]);
    let displacement = e.get(this, ActiveEffect::pDisplacementSpell);
    let id = if displacement == 0 {
        0
    } else {
        e.call(MAGIC_ITEM_GET_FORM_ID, &args![displacement]).u32()
    };
    e.call(SAVE_FORM_ID, &args![buffer, id, 0u32]);
    let source = e.get(this, ActiveEffect::pSource);
    e.call(SAVE_FORM_ID_OV2, &args![buffer, source, 0u32]);
    let mut count = 0u32;
    let handle = e.call(START_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    let mut node = e.get(this, ActiveEffect::pHitEffects);
    while node != 0 {
        let slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
        let hit_effect = e.mem.u32(slot);
        if hit_effect != 0 {
            let kind = e.vcall(hit_effect, 0x9c, &[]).u8();
            e.with_stack(4, |e, byte| {
                e.mem.set_u8(byte.addr(), kind);
                e.call(SAVE_DATA_00865E50, &args![buffer, byte, 1u32, 0u32]);
            });
            e.vcall(hit_effect, 0xa8, &args![buffer]);
            count = count.wrapping_add(1);
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    e.call(SAVE_VARIABLE_SIZED_VALUE_OV2, &args![buffer, count, handle]);
}

// Translated from 00806840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl: save buffer, effect): writes one active effect of a list: its spell's
/// form id (0 when unset), the effect item's index in the spell's list (`+0xc`), the
/// word `00810fd0` gives for its effect setting, then the effect's own save (slot +0x14)
/// as a variable-sized value closed with the number of bytes it wrote (`0084e3a0`
/// before and after).
pub fn fn_00806840(e: &mut Engine, buffer: u32, effect: Ptr<ActiveEffect>) {
    let spell = e.get(effect, ActiveEffect::pSpell);
    let id = if spell == 0 {
        0
    } else {
        e.call(MAGIC_ITEM_GET_FORM_ID, &args![spell]).u32()
    };
    e.call(SAVE_FORM_ID, &args![buffer, id, 0u32]);
    let item = e.get(effect, ActiveEffect::pEffect);
    let spell = e.get(effect, ActiveEffect::pSpell);
    let index = e
        .call(
            EFFECT_ITEM_LIST_GET_INDEX,
            &args![spell.wrapping_add(0xc), item],
        )
        .u8();
    e.with_stack(4, |e, byte| {
        e.mem.set_u8(byte.addr(), index);
        e.call(SAVE_DATA_00865E50, &args![buffer, byte, 1u32, 0u32]);
    });
    let item = e.get(effect, ActiveEffect::pEffect);
    let setting = e.call(GETTER_00825C00, &args![item]).u32();
    let value = e.call(GETTER_00810FD0, &args![setting]).u32();
    e.call(SAVE_VARIABLE_SIZED_VALUE, &args![buffer, value]);
    let handle = e.call(START_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    let start = e.call(GETTER_0084E3A0, &args![buffer]).u32();
    e.vcall(effect.addr(), 0x14, &args![buffer]);
    let end = e.call(GETTER_0084E3A0, &args![buffer]).u32();
    e.call(
        SAVE_VARIABLE_SIZED_VALUE_OV2,
        &args![buffer, end.wrapping_sub(start), handle],
    );
}

// Translated from 00806a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActiveEffect::SaveActiveEffectList` (Xbox PDB, cdecl: save buffer, list): writes each
/// effect of the list with `fn_00806840` as one variable-sized value closed with their
/// count.
pub fn active_effect_save_active_effect_list(e: &mut Engine, buffer: u32, list: u32) {
    let mut count = 0u32;
    let handle = e.call(START_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    let mut node = list;
    while node != 0 {
        let slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
        let effect = e.mem.u32(slot);
        if effect != 0 {
            fn_00806840(e, buffer, Ptr::new(effect));
            count = count.wrapping_add(1);
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    e.call(SAVE_VARIABLE_SIZED_VALUE_OV2, &args![buffer, count, handle]);
}

/// The `TESSaveLoadGame` pointer (`011de45c`) the save and load functions below pass as
/// `this` to `SaveNumericID`/`LoadNumericID`/`UseSaveGameBlocks` (Xbox PDB names of those
/// callees) and to the byte readers and writers.
const SAVE_LOAD_GAME_POINTER: u32 = 0x011d_e45c;
/// `008579b0` (thiscall on the save-load game: address, size; unnamed): writes bytes.
const SAVE_BYTES_008579B0: u32 = 0x0085_79b0;
/// `008579e0` (thiscall on the save-load game: address, size; unnamed): reads bytes.
const LOAD_BYTES_008579E0: u32 = 0x0085_79e0;
/// `TESSaveLoadGame::SaveNumericID` (`00857a10`, address, size).
const SAVE_NUMERIC_ID: u32 = 0x0085_7a10;
/// `TESSaveLoadGame::LoadNumericID` (`00857aa0`, address, size).
const LOAD_NUMERIC_ID: u32 = 0x0085_7aa0;
/// `00857bd0` (thiscall on the save-load game, one word; unnamed).
const SKIP_BYTES_00857BD0: u32 = 0x0085_7bd0;
/// `008df040` (thiscall): the byte at +0x80 of the save-load game, which these functions
/// compare with 0x2a, 0x41 and 0x48 (a format version, unconfirmed).
const SAVE_VERSION_008DF040: u32 = 0x008d_f040;
/// `TESSaveLoadGame::UseSaveGameBlocks` (`00862110`, returns `AL`).
const USE_SAVE_GAME_BLOCKS: u32 = 0x0086_2110;
/// `004fd3c0` (thiscall): the word at +0x84 of the save-load game (a record, read below
/// at +0, +5 and +9).
const LOAD_RECORD_004FD3C0: u32 = 0x004f_d3c0;
/// `008256d0` (thiscall on a list node, `AL`): true when the node's item and next are
/// both null.
const LIST_NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// `00408d60` (thiscall on a setting): the address of its value (+4).
const SETTING_VALUE_ADDRESS: u32 = 0x0040_8d60;
/// The setting whose value byte enables the size reports.
const SETTING_011DE4E8: u32 = 0x011d_e4e8;
/// `004839c0` (cdecl, one word: a form id; returns the form).
const LOOKUP_FORM_004839C0: u32 = 0x0048_39c0;
/// `Error` (`0040fbe0`, cdecl, variable arguments; map name, `tesdatahandler.cpp`).
const ERROR_0040FBE0: u32 = 0x0040_fbe0;
/// `005b5e40` (cdecl, variable arguments, a format first; unnamed, `tesscriptfunctions.cpp`).
const MESSAGE_005B5E40: u32 = 0x005b_5e40;
/// `SaveGameWarning` (`00845cc0`, cdecl, variable arguments).
const SAVE_GAME_WARNING: u32 = 0x0084_5cc0;
/// The block marker the save writes and the load expects (`0x424c4f4b`).
const BLOCK_MARKER: u32 = 0x424c_4f4b;
/// `00803d30` (thiscall on a new 0x48-byte block: caster, spell, effect item; unnamed): an
/// `ActiveEffect` constructor (stores the vtable `0107935c`).
const ACTIVE_EFFECT_CONSTRUCTOR_00803D30: u32 = 0x0080_3d30;
/// `MagicCaster::GetMagicCasterByNumericID` (`00818c60`, cdecl: id).
const MAGIC_CASTER_BY_NUMERIC_ID: u32 = 0x0081_8c60;
/// `MagicTarget::GetMagicTargetByNumericID` (`00825550`, cdecl: id).
const MAGIC_TARGET_BY_NUMERIC_ID: u32 = 0x0082_5550;
/// `MagicItem::GetMagicItemByNumericID` (`0040a250`, cdecl: id).
const MAGIC_ITEM_BY_NUMERIC_ID: u32 = 0x0040_a250;
/// `EffectItemList::GetEffectItemByIndex` (`00406ee0`, thiscall: index).
const EFFECT_ITEM_BY_INDEX: u32 = 0x0040_6ee0;
/// `EffectArchetypes::Activate` (map name, `00403ab0`, cdecl: four words): makes the
/// active effect of an effect item.
const EFFECT_ARCHETYPES_ACTIVATE: u32 = 0x0040_3ab0;
/// `ProcessLists::AddTempEffect` (`00973fd0`, thiscall: the effect).
const PROCESS_LISTS_ADD_TEMP_EFFECT: u32 = 0x0097_3fd0;
/// The `ProcessLists` object `00973fd0` is called on.
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// `008d8520` (thiscall on an actor; the map's name `MiddleHighProcess::GetSavedAcquireObject`
/// may come from folded code).
const ACTOR_OBJECT_008D8520: u32 = 0x008d_8520;
/// `TESBoundObject`'s RTTI type descriptor (`.?AVTESBoundObject@@`).
const TES_BOUND_OBJECT_TYPE: u32 = 0x0118_3108;
/// `TESForm`'s RTTI type descriptor (`.?AVTESForm@@`).
const TES_FORM_TYPE: u32 = 0x0118_3028;
/// `ActiveEffect`'s RTTI type descriptor (`.?AVActiveEffect@@`).
const ACTIVE_EFFECT_TYPE: u32 = 0x011a_0d1c;
/// `VampirismEffect`'s RTTI type descriptor (`.?AVVampirismEffect@@`).
const VAMPIRISM_EFFECT_TYPE: u32 = 0x011a_0d3c;
/// The compare function the loads insert effects with (`00825270`).
const EFFECT_COMPARE_00825270: u32 = 0x0082_5270;
/// The allocator `00aa13e0` (cdecl: size).
const ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
/// `0081c530` (thiscall on a 0x3c-byte block; unnamed): the hit effect of kind 5.
const HIT_EFFECT_KIND_5_CONSTRUCTOR: u32 = 0x0081_c530;
/// `0081f220` (thiscall on a 0x6c-byte block; unnamed): the hit effect of kind 6.
const HIT_EFFECT_KIND_6_CONSTRUCTOR: u32 = 0x0081_f220;
/// `BGSLoadGameBuffer::LoadFormID` (`008648a0`, thiscall, no stack words).
const LOAD_FORM_ID: u32 = 0x0086_48a0;
/// `BGSLoadGameBuffer::LoadFormID_ov2` (`008648e0`, thiscall: address).
const LOAD_FORM_ID_OV2: u32 = 0x0086_48e0;
/// `00864980` (thiscall on the load buffer: address, size; unnamed): reads bytes.
const LOAD_DATA_00864980: u32 = 0x0086_4980;
/// `BGSLoadGameBuffer::LoadVariableSizedValue` (`00864a60`).
const LOAD_VARIABLE_SIZED_VALUE: u32 = 0x0086_4a60;
/// `0049bb50` (thiscall on the load buffer, one word; unnamed).
const SKIP_0049BB50: u32 = 0x0049_bb50;
/// `00984f60` (thiscall on a hit effect: the active effect; unnamed).
const HIT_EFFECT_SET_EFFECT_00984F60: u32 = 0x0098_4f60;

/// The save-load game object (`*011de45c`).
pub(crate) fn save_load_game(e: &Engine) -> u32 {
    e.global::<u32>(SAVE_LOAD_GAME_POINTER)
}

/// The save-load game's version byte (`008df040`).
pub(crate) fn save_version(e: &mut Engine) -> u8 {
    let game = save_load_game(e);
    e.call(SAVE_VERSION_008DF040, &args![game]).u8()
}

/// `008579b0` on the save-load game: writes `size` bytes at `address`.
pub(crate) fn save_bytes(e: &mut Engine, address: u32, size: u32) {
    let game = save_load_game(e);
    e.call(SAVE_BYTES_008579B0, &args![game, address, size]);
}

/// `008579e0` on the save-load game: reads `size` bytes to `address`.
pub(crate) fn load_bytes(e: &mut Engine, address: u32, size: u32) {
    let game = save_load_game(e);
    e.call(LOAD_BYTES_008579E0, &args![game, address, size]);
}

/// `00825c00` on the save-load game (its word at +0x14).
pub(crate) fn save_position(e: &mut Engine) -> u32 {
    let game = save_load_game(e);
    e.call(GETTER_00825C00, &args![game]).u32()
}

/// `UseSaveGameBlocks` on the save-load game.
pub(crate) fn use_save_game_blocks(e: &mut Engine) -> bool {
    let game = save_load_game(e);
    e.call(USE_SAVE_GAME_BLOCKS, &args![game]).bool()
}

/// Whether the value byte of the setting at `011de4e8` is set.
pub(crate) fn size_reports_enabled(e: &mut Engine) -> bool {
    let value = e
        .call(SETTING_VALUE_ADDRESS, &args![SETTING_011DE4E8])
        .u32();
    e.mem.u8(value) != 0
}

/// Walks a `BSSimpleList` from `node` as these functions do: it stops at a null node or at
/// one `008256d0` reports empty, and hands `body` each node's item (`*006815c0(node)`).
fn for_each_until_empty(e: &mut Engine, mut node: u32, mut body: impl FnMut(&mut Engine, u32)) {
    while node != 0 && !e.call(LIST_NODE_IS_EMPTY, &args![node]).bool() {
        let slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
        let item = e.mem.u32(slot);
        body(e, item);
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
}

/// Walks a `BSSimpleList` from `node` to its end and hands `body` each non-null item.
fn for_each_item(e: &mut Engine, mut node: u32, mut body: impl FnMut(&mut Engine, u32)) {
    while node != 0 {
        let slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
        let item = e.mem.u32(slot);
        if item != 0 {
            body(e, item);
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
}

/// Adds the hit effect at `item_slot` (a stack word) to the effect's hit-effect list, as
/// both loads do: a new 8-byte node (`operator new`, `00470440`) for an empty list, the
/// list's front insert (`005ae3d0`) otherwise.
fn add_hit_effect(e: &mut Engine, this: Ptr<ActiveEffect>, item_slot: u32) {
    if e.get(this, ActiveEffect::pHitEffects) == 0 {
        let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
        let node = if block == 0 {
            0
        } else {
            e.call(LIST_NODE_INIT_00470440, &args![block, item_slot])
                .u32()
        };
        e.set(this, ActiveEffect::pHitEffects, node);
    } else {
        let list = e.get(this, ActiveEffect::pHitEffects);
        e.call(LIST_INSERT_FRONT_005AE3D0, &args![list, item_slot]);
    }
}

/// Makes a hit effect of `kind` as both loads do: kind 5 a 0x3c-byte `0081c530`, kind 6 a
/// 0x6c-byte `0081f220` (null when the allocation fails); any other kind is reported
/// through `005b5e40` with `format` and gives null.
fn new_hit_effect(e: &mut Engine, kind: u8, format: u32) -> u32 {
    match kind {
        5 => {
            let block = e.call(ALLOCATE_00AA13E0, &args![0x3cu32]).u32();
            if block == 0 {
                0
            } else {
                e.call(HIT_EFFECT_KIND_5_CONSTRUCTOR, &args![block]).u32()
            }
        }
        6 => {
            let block = e.call(ALLOCATE_00AA13E0, &args![0x6cu32]).u32();
            if block == 0 {
                0
            } else {
                e.call(HIT_EFFECT_KIND_6_CONSTRUCTOR, &args![block]).u32()
            }
        }
        _ => {
            e.call(MESSAGE_005B5E40, &args![format, u32::from(kind)]);
            0
        }
    }
}

/// The source-file string these reports pass (`...\Magic\ActiveEffect.cpp`, `01079438`).
const ACTIVE_EFFECT_FILE: u32 = 0x0107_9438;
/// The formats the size reports of a save function use (with and without a record).
pub(crate) const COUNTED_SIZE_FORMATS: [u32; 2] = [0x0101_2cb0, 0x0101_2c78];
/// The formats the size reports after a save block is written use.
pub(crate) const WRITTEN_SIZE_FORMATS: [u32; 2] = [0x0101_53a0, 0x0101_536c];

/// `Error` with a size a save function counted or wrote, as `fn_00805a00`,
/// `fn_00805b40` and the bound item effect's save functions report it: with the record of
/// `004fd3e0` (`formats[0]`) its id, the name slot +0x130 of the form of that id
/// (`004839c0`) and its word at +5 too; `file` and `line` name the caller's source.
pub(crate) fn report_save_size(e: &mut Engine, formats: [u32; 2], size: u32, file: u32, line: u32) {
    let game = save_load_game(e);
    let record = e.call(GETTER_004FD3E0, &args![game]).u32();
    if record != 0 {
        let id = e.mem.u32(record);
        let form = e.call(LOOKUP_FORM_004839C0, &args![id]).u32();
        let word = e.mem.u32(record + 5);
        let name = e.vcall(form, 0x130, &[]).u32();
        let id = e.mem.u32(record);
        e.call(
            ERROR_0040FBE0,
            &args![formats[0], size, id, name, word, line, file],
        );
    } else {
        e.call(ERROR_0040FBE0, &args![formats[1], size, line, file]);
    }
}

/// The start of a save block with save-game blocks on: the block marker (written from the
/// stack word `marker_slot`) and the 2-byte size at `size_slot` to fill in later; returns
/// the position of that size (`00825c00` between the two writes).
pub(crate) fn save_block_start(e: &mut Engine, marker_slot: u32, size_slot: u32) -> u32 {
    e.mem.set_u32(marker_slot, BLOCK_MARKER);
    save_bytes(e, marker_slot, 4);
    let size_at = save_position(e);
    save_bytes(e, size_slot, 2);
    size_at
}

/// The end of a save block: a size over 0xffff is reported through `005b5e40` (with
/// `file` and `line`), and the 16 bits of the size are stored at `size_at`.
pub(crate) fn save_block_finish(e: &mut Engine, size_at: u32, file: u32, line: u32) {
    let now = save_position(e);
    if now > size_at.wrapping_add(0xffff) {
        e.call(MESSAGE_005B5E40, &args![0x0101_5318u32, file, line]);
    }
    e.mem.set_u16(size_at, now.wrapping_sub(size_at) as u16);
}

/// The start of a save block when loading with save-game blocks on: reads the marker into
/// the stack word `marker_slot` and reports a wrong one through `005b5e40` (with the record
/// of `004fd3c0`, or the save version), then reads the 2-byte size into `size_slot`;
/// returns the position before the size (`00825c00`).
pub(crate) fn load_block_start(
    e: &mut Engine,
    marker_slot: u32,
    size_slot: u32,
    file: u32,
    line: u32,
) -> u32 {
    load_bytes(e, marker_slot, 4);
    if e.mem.u32(marker_slot) != BLOCK_MARKER {
        let game = save_load_game(e);
        let record = e.call(LOAD_RECORD_004FD3C0, &args![game]).u32();
        if record != 0 {
            let id = e.mem.u32(record);
            let form = e.call(LOOKUP_FORM_004839C0, &args![id]).u32();
            let word = e.mem.u32(record + 5);
            let byte = u32::from(e.mem.u8(record + 9));
            let name = e.vcall(form, 0x130, &[]).u32();
            let id = e.mem.u32(record);
            e.call(
                MESSAGE_005B5E40,
                &args![0x0101_5718u32, file, line, id, name, byte, word],
            );
        } else {
            let version = u32::from(save_version(e));
            e.call(
                MESSAGE_005B5E40,
                &args![0x0101_56a8u32, file, line, version],
            );
        }
    }
    let start = save_position(e);
    load_bytes(e, size_slot, 2);
    start
}

/// The end of a loaded save block: the position (`00825c00`) is compared with the block's
/// start plus its 16-bit size (re-read from `size_slot`); a position past or short of it is
/// reported through `005b5e40` with the difference, `file` and `line`, and with the
/// record of `004fd3c0` (its id, name, byte +9 and word +5) or the save version.
pub(crate) fn load_block_check(e: &mut Engine, start: u32, size_slot: u32, file: u32, line: u32) {
    let now = save_position(e);
    let game = save_load_game(e);
    let record = e.call(LOAD_RECORD_004FD3C0, &args![game]).u32();
    let expected = |e: &Engine| u32::from(e.mem.u16(size_slot)).wrapping_add(start);
    if record != 0 {
        let id = e.mem.u32(record);
        let form = e.call(LOOKUP_FORM_004839C0, &args![id]).u32();
        let report = |e: &mut Engine, format: u32, difference: u32| {
            let word = e.mem.u32(record + 5);
            let byte = u32::from(e.mem.u8(record + 9));
            let name = e.vcall(form, 0x130, &[]).u32();
            let id = e.mem.u32(record);
            e.call(
                MESSAGE_005B5E40,
                &args![format, difference, file, line, id, name, byte, word],
            );
        };
        if now > expected(e) {
            let difference = now.wrapping_sub(expected(e));
            report(e, 0x0101_5588, difference);
        } else if now < expected(e) {
            let difference = expected(e).wrapping_sub(now);
            report(e, 0x0101_5500, difference);
        }
    } else if now > expected(e) {
        let version = u32::from(save_version(e));
        let difference = now.wrapping_sub(expected(e));
        e.call(
            MESSAGE_005B5E40,
            &args![0x0101_54a0u32, difference, file, line, version],
        );
    } else if now < expected(e) {
        let version = u32::from(save_version(e));
        let difference = expected(e).wrapping_sub(now);
        e.call(
            MESSAGE_005B5E40,
            &args![0x0101_5440u32, difference, file, line, version],
        );
    }
}

// Translated from 00803fe0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00803fe0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `ActiveEffect::Copy` (Xbox PDB): a new 0x48-byte `ActiveEffect` (`operator new`, then
/// `00803d30` with the caster, spell and effect item; null when the allocation fails),
/// given this effect's data through `CopyData` (slot +0x44); returns it.
pub fn active_effect_copy(e: &mut Engine, this: Ptr<ActiveEffect>) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x48u32]).u32();
    let copy = if block == 0 {
        0
    } else {
        let item = e.get(this, ActiveEffect::pEffect);
        let spell = e.get(this, ActiveEffect::pSpell);
        let caster = e.get(this, ActiveEffect::pCaster);
        e.call(
            ACTIVE_EFFECT_CONSTRUCTOR_00803D30,
            &args![block, caster, spell, item],
        )
        .u32()
    };
    e.vcall(this.addr(), 0x44, &args![copy]);
    copy
}

// Translated from 00804f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActiveEffect::GetSaveSize` (Xbox PDB): 0x1c bytes, plus (save version 0x2a or later)
/// one count byte and, per hit effect until an empty node, one type byte and its own size
/// (slot +0xd0 with `this` and `argument`), plus 4 (version 0x48 or later) or 1 (0x41 or
/// later). Returns `AX` (16-bit sums).
pub fn active_effect_get_save_size(e: &mut Engine, this: Ptr<ActiveEffect>, argument: u32) -> u16 {
    let mut size: u16 = 4 + 1 + 1 + 1 + 1 + 4 + 4 + 4 + 4 + 4;
    if save_version(e) >= 0x2a {
        size += 1;
        let list = e.get(this, ActiveEffect::pHitEffects);
        for_each_until_empty(e, list, |e, item| {
            size = size.wrapping_add(1);
            let own = e.vcall(item, 0xd0, &args![this, argument]).u16();
            size = size.wrapping_add(own);
        });
    }
    if save_version(e) >= 0x48 {
        size = size.wrapping_add(4);
    } else if save_version(e) >= 0x41 {
        size = size.wrapping_add(1);
    }
    size
}

// Translated from 008050d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActiveEffect::SaveGame` (Xbox PDB): writes the elapsed seconds, the five state bytes,
/// magnitude and duration (`008579b0`), the caster's and target's form ids and the
/// source's `0084e3a0` word (0 when unset; `SaveNumericID`); from save version 0x2a a
/// count byte (stored afterwards where `00825c00` pointed before it was written) and per
/// hit effect until an empty node its type (slot +0x9c) and its own save (slot +0xd4 with
/// `this` and `argument`); then the flags (version 0x48 or later) or flag 1 as a byte
/// (0x41 or later).
pub fn active_effect_save_game(e: &mut Engine, this: Ptr<ActiveEffect>, argument: u32) {
    let base = this.addr();
    e.with_stack(0x24, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x24 - offset;
        for (offset, size) in [
            (0x04u32, 4u32),
            (0x10, 1),
            (0x11, 1),
            (0x12, 1),
            (0x13, 1),
            (0x14, 1),
            (0x1c, 4),
            (0x20, 4),
        ] {
            save_bytes(e, base + offset, size);
        }
        let caster = e.get(this, ActiveEffect::pCaster);
        let id = if caster == 0 {
            0
        } else {
            e.call(MAGIC_CASTER_GET_FORM_ID, &args![caster]).u32()
        };
        e.mem.set_u32(local(8), id);
        let game = save_load_game(e);
        e.call(SAVE_NUMERIC_ID, &args![game, local(8), 4u32]);
        let target = e.get(this, ActiveEffect::pTarget);
        let id = if target == 0 {
            0
        } else {
            e.call(MAGIC_TARGET_GET_FORM_ID, &args![target]).u32()
        };
        e.mem.set_u32(local(4), id);
        let game = save_load_game(e);
        e.call(SAVE_NUMERIC_ID, &args![game, local(4), 4u32]);
        let source = e.get(this, ActiveEffect::pSource);
        let id = if source == 0 {
            0
        } else {
            e.call(GETTER_0084E3A0, &args![source]).u32()
        };
        e.mem.set_u32(local(0xc), id);
        let game = save_load_game(e);
        e.call(SAVE_NUMERIC_ID, &args![game, local(0xc), 4u32]);
        if save_version(e) >= 0x2a {
            e.mem.set_u8(local(0xd), 0);
            let count_at = save_position(e);
            save_bytes(e, local(0xd), 1);
            let list = e.get(this, ActiveEffect::pHitEffects);
            for_each_until_empty(e, list, |e, item| {
                let kind = e.vcall(item, 0x9c, &[]).u8();
                e.mem.set_u8(local(0x1d), kind);
                save_bytes(e, local(0x1d), 1);
                e.vcall(item, 0xd4, &args![this, argument]);
                let count = e.mem.u8(local(0xd)).wrapping_add(1);
                e.mem.set_u8(local(0xd), count);
            });
            let count = e.mem.u8(local(0xd));
            e.mem.set_u8(count_at, count);
        }
        if save_version(e) >= 0x48 {
            save_bytes(e, base + 0x18, 4);
        } else if save_version(e) >= 0x41 {
            let flag = fn_00804b80(e, this, 1);
            e.mem.set_u8(local(0x1e), u8::from(flag));
            save_bytes(e, local(0x1e), 1);
        }
    });
}

// Translated from 00805330 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00805330 - the exception-unwinding frame (FS:[0]) is not translated.
/// `ActiveEffect::LoadGame` (Xbox PDB): reads what `SaveGame` wrote: the elapsed seconds,
/// state bytes, magnitude and duration (`008579e0`), then the caster, target and source
/// ids into +0x28, +0x24 and +0x3c (`LoadNumericID`); from save version 0x2a a count and
/// that many hit effects (a type byte: 5 or 6 make one, another is reported through
/// `005b5e40` and leaves null, whose slot +0xd8 the exe then calls), each loaded (slot
/// +0xd8 with `this` and `argument`) and added to the list; then the flags (version 0x48
/// or later) or a byte that, when set, runs `fn_00805650` (0x41 or later).
pub fn active_effect_load_game(e: &mut Engine, this: Ptr<ActiveEffect>, argument: u32) {
    let base = this.addr();
    e.with_stack(0x58, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x58 - offset;
        for (offset, size) in [
            (0x04u32, 4u32),
            (0x10, 1),
            (0x11, 1),
            (0x12, 1),
            (0x13, 1),
            (0x14, 1),
            (0x1c, 4),
            (0x20, 4),
        ] {
            load_bytes(e, base + offset, size);
        }
        for (slot, field) in [(0x14u32, 0x28u32), (0x10, 0x24), (0x18, 0x3c)] {
            let game = save_load_game(e);
            e.call(LOAD_NUMERIC_ID, &args![game, local(slot), 4u32]);
            let id = e.mem.u32(local(slot));
            e.mem.set_u32(base + field, id);
        }
        if save_version(e) >= 0x2a {
            load_bytes(e, local(0x19), 1);
            let mut index = 0u32;
            while index < u32::from(e.mem.u8(local(0x19))) {
                load_bytes(e, local(0x25), 1);
                let kind = e.mem.u8(local(0x25));
                let hit_effect = new_hit_effect(e, kind, 0x0107_93b8);
                e.mem.set_u32(local(0x24), hit_effect);
                e.vcall(hit_effect, 0xd8, &args![this, argument]);
                add_hit_effect(e, this, local(0x24));
                index += 1;
            }
        }
        if save_version(e) >= 0x48 {
            load_bytes(e, base + 0x18, 4);
        } else if save_version(e) >= 0x41 {
            load_bytes(e, local(0x26), 1);
            if e.mem.u8(local(0x26)) != 0 {
                fn_00805650(e, this);
            }
        }
    });
}

// Translated from 00805650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`activeeffect.cpp`): sets flags 2 and 4 of the effect (`00803e80` twice).
pub fn fn_00805650(e: &mut Engine, this: Ptr<ActiveEffect>) {
    e.call(EFFECT_FLAG_00803E80, &args![this, 2u32]);
    e.call(EFFECT_FLAG_00803E80, &args![this, 4u32]);
}

// Translated from 00805670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActiveEffect::InitLoadGame` (Xbox PDB): turns the loaded ids into objects: the caster
/// (`GetMagicCasterByNumericID`) and target (`GetMagicTargetByNumericID`) when set; the
/// source, only when the target id was set, becomes the form of its id (`004839c0`) cast
/// through `__RTDynamicCast` (`01183028` to `01183108`). From save version 0x2a each hit
/// effect until an empty node runs its slot +0xdc with `this` and `argument`.
pub fn active_effect_init_load_game(e: &mut Engine, this: Ptr<ActiveEffect>, argument: u32) {
    let caster = e.get(this, ActiveEffect::pCaster);
    if caster != 0 {
        let object = e.call(MAGIC_CASTER_BY_NUMERIC_ID, &args![caster]).u32();
        e.set(this, ActiveEffect::pCaster, object);
    }
    let target = e.get(this, ActiveEffect::pTarget);
    if target != 0 {
        let object = e.call(MAGIC_TARGET_BY_NUMERIC_ID, &args![target]).u32();
        e.set(this, ActiveEffect::pTarget, object);
    }
    let source = e.get(this, ActiveEffect::pSource);
    if target != 0 {
        let form = e.call(LOOKUP_FORM_004839C0, &args![source]).u32();
        let cast = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TES_FORM_TYPE, TES_BOUND_OBJECT_TYPE, 0u32],
            )
            .u32();
        e.set(this, ActiveEffect::pSource, cast);
    }
    if save_version(e) >= 0x2a {
        let list = e.get(this, ActiveEffect::pHitEffects);
        for_each_until_empty(e, list, |e, item| {
            e.vcall(item, 0xdc, &args![this, argument]);
        });
    }
}

// Translated from 00805760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActiveEffect::FinishInitLoadGame` (Xbox PDB): from save version 0x2a each hit effect
/// until an empty node runs its slot +0xe0 (`this`, `argument`, 0) and is handed to
/// `ProcessLists::AddTempEffect`; then the sound restarts (`fn_00804cd0`) when the effect
/// setting has flag 0x400.
pub fn active_effect_finish_init_load_game(e: &mut Engine, this: Ptr<ActiveEffect>, argument: u32) {
    if save_version(e) >= 0x2a {
        let list = e.get(this, ActiveEffect::pHitEffects);
        for_each_until_empty(e, list, |e, item| {
            e.vcall(item, 0xe0, &args![this, argument, 0u32]);
            e.call(PROCESS_LISTS_ADD_TEMP_EFFECT, &args![PROCESS_LISTS, item]);
        });
    }
    let setting = e.call(EFFECT_POINTER_007DF1D0, &args![this]).u32();
    if e.call(FLAG_TEST_00403E60, &args![setting, 0x400u32]).bool() {
        fn_00804cd0(e, this);
    }
}

// Translated from 00805810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl: effect, argument): the bytes `SaveActiveEffect` writes for one effect:
/// 2 more from save version 0x2a, 4 + 1, and the effect's own size (slot +0x10 with
/// `argument`). Returns `AX`.
pub fn fn_00805810(e: &mut Engine, effect: Ptr<ActiveEffect>, argument: u32) -> u16 {
    let mut size = 0u16;
    if save_version(e) >= 0x2a {
        size += 2;
    }
    size += 4;
    size += 1;
    let own = e.vcall(effect.addr(), 0x10, &args![argument]).u16();
    size.wrapping_add(own)
}

// Translated from 00805880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActiveEffect::SaveActiveEffect` (Xbox PDB, cdecl: effect, argument): from save version
/// 0x2a its size (slot +0x10) as 2 bytes; its spell's form id (`SaveNumericID`); the
/// effect item's index in the spell's list as a byte; then its own save (slot +0x18).
pub fn active_effect_save_active_effect(e: &mut Engine, effect: Ptr<ActiveEffect>, argument: u32) {
    e.with_stack(0xc, |e, frame| {
        let local = |offset: u32| frame.addr() + 0xc - offset;
        if save_version(e) >= 0x2a {
            let size = e.vcall(effect.addr(), 0x10, &args![argument]).u16();
            e.mem.set_u16(local(0xc), size);
            save_bytes(e, local(0xc), 2);
        }
        let spell = e.get(effect, ActiveEffect::pSpell);
        let id = e.call(MAGIC_ITEM_GET_FORM_ID, &args![spell]).u32();
        e.mem.set_u32(local(4), id);
        let game = save_load_game(e);
        e.call(SAVE_NUMERIC_ID, &args![game, local(4), 4u32]);
        let item = e.get(effect, ActiveEffect::pEffect);
        let spell = e.get(effect, ActiveEffect::pSpell);
        let index = e
            .call(
                EFFECT_ITEM_LIST_GET_INDEX,
                &args![spell.wrapping_add(0xc), item],
            )
            .u8();
        e.mem.set_u8(local(5), index);
        save_bytes(e, local(5), 1);
        e.vcall(effect.addr(), 0x18, &args![argument]);
    });
}

// Translated from 00805920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActiveEffect::LoadActiveEffect` (Xbox PDB, cdecl: argument): reads what
/// `SaveActiveEffect` wrote (the size, 0x1c before save version 0x2a; the spell id; the
/// effect item's index). With the spell (`GetMagicItemByNumericID`) and its effect item
/// (`GetEffectItemByIndex`) it makes the effect (`00403ab0` with 0, spell, item, 0), loads
/// it (slot +0x20) and returns it; otherwise it passes the size to `00857bd0`, reports the
/// id and index through `005b5e40` and returns null.
pub fn active_effect_load_active_effect(e: &mut Engine, argument: u32) -> u32 {
    e.with_stack(0x18, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x18 - offset;
        e.mem.set_u16(local(0xc), 0x1c);
        if save_version(e) >= 0x2a {
            load_bytes(e, local(0xc), 2);
        }
        let game = save_load_game(e);
        e.call(LOAD_NUMERIC_ID, &args![game, local(8), 4u32]);
        load_bytes(e, local(0xd), 1);
        let id = e.mem.u32(local(8));
        let spell = e.call(MAGIC_ITEM_BY_NUMERIC_ID, &args![id]).u32();
        if spell != 0 {
            let index = u32::from(e.mem.u8(local(0xd)));
            let item = e
                .call(EFFECT_ITEM_BY_INDEX, &args![spell.wrapping_add(0xc), index])
                .u32();
            if item != 0 {
                let effect = e
                    .call(EFFECT_ARCHETYPES_ACTIVATE, &args![0u32, spell, item, 0u32])
                    .u32();
                e.vcall(effect, 0x20, &args![argument]);
                return effect;
            }
        }
        let size = u32::from(e.mem.u16(local(0xc)));
        let game = save_load_game(e);
        e.call(SKIP_BYTES_00857BD0, &args![game, size]);
        let index = u32::from(e.mem.u8(local(0xd)));
        let id = e.mem.u32(local(8));
        e.call(MESSAGE_005B5E40, &args![0x0107_93e0u32, id, index]);
        0
    })
}

// Translated from 00805a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl: list, argument): the bytes a list of effects takes in the save: 6 more
/// with save-game blocks, 2, and `fn_00805810` per effect until an empty node. With the
/// setting at `011de4e8` set it reports the size through `Error`. Returns `AX`.
pub fn fn_00805a00(e: &mut Engine, list: u32, argument: u32) -> u16 {
    let mut size = 0u16;
    let start = size;
    if use_save_game_blocks(e) {
        size += 4;
        size += 2;
    }
    size += 2;
    for_each_until_empty(e, list, |e, effect| {
        let own = fn_00805810(e, Ptr::new(effect), argument);
        size = size.wrapping_add(own);
    });
    if size_reports_enabled(e) {
        let counted = u32::from(size).wrapping_sub(u32::from(start));
        report_save_size(e, COUNTED_SIZE_FORMATS, counted, ACTIVE_EFFECT_FILE, 0x3c8);
    }
    size
}

// Translated from 00805b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl: list, argument): writes a list of effects. With save-game blocks it
/// writes the block marker and a 2-byte size to fill in; then a 2-byte count, each effect
/// until an empty node (`SaveActiveEffect`), and the count where it was. With the setting
/// at `011de4e8` set it reports the bytes written through `Error`; with blocks it reports
/// a size over 0xffff through `005b5e40` and stores the size (16 bits) after the marker.
pub fn fn_00805b40(e: &mut Engine, list: u32, argument: u32) {
    e.with_stack(0x2c, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x2c - offset;
        e.mem.set_u16(local(0xc), 0);
        let mut size_at = 0u32;
        let mut start = save_position(e);
        if size_reports_enabled(e) {
            start = save_position(e);
        }
        if use_save_game_blocks(e) {
            size_at = save_block_start(e, local(0x18), local(0xc));
        }
        e.mem.set_u16(local(0x14), 0);
        let count_at = save_position(e);
        save_bytes(e, local(0x14), 2);
        for_each_until_empty(e, list, |e, effect| {
            active_effect_save_active_effect(e, Ptr::new(effect), argument);
            let count = e.mem.u16(local(0x14)).wrapping_add(1);
            e.mem.set_u16(local(0x14), count);
        });
        let count = e.mem.u16(local(0x14));
        e.mem.set_u16(count_at, count);
        if size_reports_enabled(e) {
            let end = save_position(e);
            let written = end.wrapping_sub(start);
            report_save_size(e, WRITTEN_SIZE_FORMATS, written, ACTIVE_EFFECT_FILE, 0x3e0);
        }
        if use_save_game_blocks(e) {
            save_block_finish(e, size_at, ACTIVE_EFFECT_FILE, 0x3e0);
        }
    });
}

// Translated from 00805d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl: list, argument): reads a list of effects written by `fn_00805b40`. With
/// save-game blocks it checks the block marker (reporting a wrong one through `005b5e40`)
/// and reads the block's size. The list's effects (until an empty node) are destroyed
/// (slot 0 with 1; `00495460` runs first for `VampirismEffect`s) and the list cleared
/// (`00470470`); then each of the count effects (`LoadActiveEffect`) is inserted with
/// `00825270` (and, for a `VampirismEffect`, `008d8520` runs on `argument` when set).
/// With blocks the position is then checked against the block's size.
pub fn fn_00805d40(e: &mut Engine, list: u32, argument: u32) {
    e.with_stack(0x4c, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x4c - offset;
        e.mem.set_u16(local(0xc), 0);
        let mut start = 0u32;
        if use_save_game_blocks(e) {
            start = load_block_start(e, local(0x1c), local(0xc), ACTIVE_EFFECT_FILE, 0x3e8);
        }
        for_each_until_empty(e, list, |e, item| {
            let cast = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![item, 0u32, ACTIVE_EFFECT_TYPE, VAMPIRISM_EFFECT_TYPE, 0u32],
                )
                .u32();
            if cast != 0 {
                // The result goes to a local nothing reads (FSTP).
                e.call(EFFECT_REAL_00495460, &args![item]);
            }
            if item != 0 {
                e.vcall(item, 0, &args![1u32]);
            }
        });
        e.call(HIT_LIST_00470470, &args![list]);
        load_bytes(e, local(0x14), 2);
        let mut index = 0i32;
        while index < i32::from(e.mem.u16(local(0x14))) {
            let effect = active_effect_load_active_effect(e, argument);
            let cast = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![
                        effect,
                        0u32,
                        ACTIVE_EFFECT_TYPE,
                        VAMPIRISM_EFFECT_TYPE,
                        0u32
                    ],
                )
                .u32();
            if cast != 0 && argument != 0 {
                e.call(ACTOR_OBJECT_008D8520, &args![argument]);
            }
            if effect != 0 {
                bs_simple_list_active_effect_insert(
                    e,
                    Ptr::new(list),
                    effect,
                    EFFECT_COMPARE_00825270,
                );
            }
            index += 1;
        }
        if use_save_game_blocks(e) {
            load_block_check(e, start, local(0xc), ACTIVE_EFFECT_FILE, 0x40f);
        }
    });
}

// Translated from 008060e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl: list, argument): each effect until an empty node runs its slot +0x28
/// with `argument`.
pub fn fn_008060e0(e: &mut Engine, list: u32, argument: u32) {
    for_each_until_empty(e, list, |e, effect| {
        e.vcall(effect, 0x28, &args![argument]);
    });
}

// Translated from 00806130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl: list, actor): each effect until an empty node runs its slot +0x2c with
/// `actor`; then, with an actor whose `008d8520` word is set, that object's slot +0x580
/// runs with 1, 1, 1.
pub fn fn_00806130(e: &mut Engine, list: u32, actor: u32) {
    for_each_until_empty(e, list, |e, effect| {
        e.vcall(effect, 0x2c, &args![actor]);
    });
    if actor != 0 && e.call(ACTOR_OBJECT_008D8520, &args![actor]).u32() != 0 {
        let object = e.call(ACTOR_OBJECT_008D8520, &args![actor]).u32();
        e.vcall(object, 0x580, &args![1u32, 1u32, 1u32]);
    }
}

// Translated from 008061b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl: list, argument): each effect until an empty node runs its slot +0x34
/// with `argument`.
pub fn fn_008061b0(e: &mut Engine, list: u32, argument: u32) {
    for_each_until_empty(e, list, |e, effect| {
        e.vcall(effect, 0x34, &args![argument]);
    });
}

// Translated from 00806420 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00806420 - the exception-unwinding frame (FS:[0]) is not translated.
/// `ActiveEffect::LoadGame_ov2` (Xbox PDB): reads what `SaveGame_ov2` wrote from the load
/// buffer: the elapsed seconds, state bytes, magnitude, duration and flags (`00864980`),
/// the caster, target and displacement spell ids (`LoadFormID_ov2`), the source (the form
/// of `LoadFormID`'s id cast from `TESForm` to `TESBoundObject`), then the hit
/// effects (a variable-sized count; each a type byte, 5 or 6 making one, another reported
/// through `005b5e40` and leaving null; then `00984f60` with `this`, its load (slot +0xb0)
/// and the list).
pub fn active_effect_load_game_ov2(e: &mut Engine, this: Ptr<ActiveEffect>, buffer: u32) {
    let base = this.addr();
    e.with_stack(0x4c, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x4c - offset;
        for (offset, size) in [
            (0x04u32, 4u32),
            (0x10, 1),
            (0x11, 1),
            (0x12, 1),
            (0x13, 1),
            (0x14, 1),
            (0x1c, 4),
            (0x20, 4),
            (0x18, 4),
        ] {
            e.call(LOAD_DATA_00864980, &args![buffer, base + offset, size]);
        }
        for offset in [0x28u32, 0x24, 0x44] {
            e.call(LOAD_FORM_ID_OV2, &args![buffer, base + offset]);
        }
        let id = e.call(LOAD_FORM_ID, &args![buffer]).u32();
        let form = e.call(LOOKUP_FORM_004839C0, &args![id]).u32();
        let source = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TES_FORM_TYPE, TES_BOUND_OBJECT_TYPE, 0u32],
            )
            .u32();
        e.set(this, ActiveEffect::pSource, source);
        let count = e.call(LOAD_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
        for _ in 0..count {
            e.mem.set_u8(local(0x19), 0);
            e.call(LOAD_DATA_00864980, &args![buffer, local(0x19), 1u32]);
            let kind = e.mem.u8(local(0x19));
            let hit_effect = new_hit_effect(e, kind, 0x0107_947c);
            e.mem.set_u32(local(0x18), hit_effect);
            e.call(HIT_EFFECT_SET_EFFECT_00984F60, &args![hit_effect, this]);
            e.vcall(hit_effect, 0xb0, &args![buffer]);
            add_hit_effect(e, this, local(0x18));
        }
    });
}

// Translated from 008066d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActiveEffect::InitLoadGame_ov2` (Xbox PDB): the caster, target and displacement spell
/// ids become their objects (`GetMagicCasterByNumericID`, `GetMagicTargetByNumericID`,
/// `GetMagicItemByNumericID`; 0 stays 0), and each hit effect runs its slot +0xb8 with
/// `argument`.
pub fn active_effect_init_load_game_ov2(e: &mut Engine, this: Ptr<ActiveEffect>, argument: u32) {
    let caster = e.get(this, ActiveEffect::pCaster);
    let caster = if caster == 0 {
        0
    } else {
        e.call(MAGIC_CASTER_BY_NUMERIC_ID, &args![caster]).u32()
    };
    e.set(this, ActiveEffect::pCaster, caster);
    let target = e.get(this, ActiveEffect::pTarget);
    let target = if target == 0 {
        0
    } else {
        e.call(MAGIC_TARGET_BY_NUMERIC_ID, &args![target]).u32()
    };
    e.set(this, ActiveEffect::pTarget, target);
    let spell = e.get(this, ActiveEffect::pDisplacementSpell);
    let spell = if spell == 0 {
        0
    } else {
        e.call(MAGIC_ITEM_BY_NUMERIC_ID, &args![spell]).u32()
    };
    e.set(this, ActiveEffect::pDisplacementSpell, spell);
    let list = e.get(this, ActiveEffect::pHitEffects);
    for_each_item(e, list, |e, hit_effect| {
        e.vcall(hit_effect, 0xb8, &args![argument]);
    });
}

// Translated from 008067b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActiveEffect::FinishLoadGame` (Xbox PDB): restarts the sound (`fn_00804cd0`) when the
/// effect setting has flag 0x400; each hit effect runs its slot +0xbc with `argument` and
/// is handed to `ProcessLists::AddTempEffect`.
pub fn active_effect_finish_load_game(e: &mut Engine, this: Ptr<ActiveEffect>, argument: u32) {
    let setting = e.call(EFFECT_POINTER_007DF1D0, &args![this]).u32();
    if e.call(FLAG_TEST_00403E60, &args![setting, 0x400u32]).bool() {
        fn_00804cd0(e, this);
    }
    let list = e.get(this, ActiveEffect::pHitEffects);
    for_each_item(e, list, |e, hit_effect| {
        e.vcall(hit_effect, 0xbc, &args![argument]);
        e.call(
            PROCESS_LISTS_ADD_TEMP_EFFECT,
            &args![PROCESS_LISTS, hit_effect],
        );
    });
}

// Translated from 00806910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActiveEffect::LoadActiveEffect_ov2` (Xbox PDB, cdecl: load buffer): reads what
/// `fn_00806840` wrote: the spell (`LoadFormID_ov2`, `GetMagicItemByNumericID`), the
/// effect item's index byte and the setting's word. When the spell has that effect item
/// and its setting's `00810fd0` word matches, it reads the size, makes the effect
/// (`00403ab0` with 0, spell, item, 0), loads it (slot +0x1c) and returns it; otherwise it
/// passes the size to `0049bb50`, warns (`SaveGameWarning`) and returns null.
pub fn active_effect_load_active_effect_ov2(e: &mut Engine, buffer: u32) -> u32 {
    e.with_stack(0x1c, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x1c - offset;
        e.mem.set_u32(local(0xc), 0);
        e.call(LOAD_FORM_ID_OV2, &args![buffer, local(0xc)]);
        let id = e.mem.u32(local(0xc));
        let spell = if id == 0 {
            0
        } else {
            e.call(MAGIC_ITEM_BY_NUMERIC_ID, &args![id]).u32()
        };
        e.mem.set_u8(local(5), 0);
        e.call(LOAD_DATA_00864980, &args![buffer, local(5), 1u32]);
        let value = e.call(LOAD_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
        let index = u32::from(e.mem.u8(local(5)));
        if spell != 0 {
            let item = e
                .call(EFFECT_ITEM_BY_INDEX, &args![spell.wrapping_add(0xc), index])
                .u32();
            if item != 0 {
                let setting = e.call(GETTER_00825C00, &args![item]).u32();
                if e.call(GETTER_00810FD0, &args![setting]).u32() == value {
                    e.call(LOAD_VARIABLE_SIZED_VALUE, &args![buffer]);
                    let effect = e
                        .call(EFFECT_ARCHETYPES_ACTIVATE, &args![0u32, spell, item, 0u32])
                        .u32();
                    e.vcall(effect, 0x1c, &args![buffer]);
                    return effect;
                }
            }
        }
        let size = e.call(LOAD_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
        e.call(SKIP_0049BB50, &args![buffer, size]);
        e.call(SAVE_GAME_WARNING, &args![0x0107_94a8u32, id, index, value]);
        0
    })
}

// Translated from 00806a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActiveEffect::LoadActiveEffectList` (Xbox PDB, cdecl: load buffer, list): reverts the
/// list (`RevertActiveEffectList`), then inserts each of the count effects
/// (`LoadActiveEffect_ov2`, nulls skipped) with `00825270`.
pub fn active_effect_load_active_effect_list(e: &mut Engine, buffer: u32, list: u32) {
    active_effect_revert_active_effect_list(e, buffer, list);
    let count = e.call(LOAD_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    for _ in 0..count {
        let effect = active_effect_load_active_effect_ov2(e, buffer);
        if effect != 0 {
            bs_simple_list_active_effect_insert(e, Ptr::new(list), effect, EFFECT_COMPARE_00825270);
        }
    }
}

// Translated from 00806b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl: argument, list): each effect of the list runs its slot +0x24 with
/// `argument`.
pub fn fn_00806b00(e: &mut Engine, argument: u32, list: u32) {
    for_each_item(e, list, |e, effect| {
        e.vcall(effect, 0x24, &args![argument]);
    });
}

// Translated from 00806b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActiveEffect::FinishLoadActiveEffectList` (Xbox PDB, cdecl: argument, list): each
/// effect of the list runs its slot +0x40 with `argument`.
pub fn active_effect_finish_load_active_effect_list(e: &mut Engine, argument: u32, list: u32) {
    for_each_item(e, list, |e, effect| {
        e.vcall(effect, 0x40, &args![argument]);
    });
}

// Translated from 00806ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActiveEffect::RevertActiveEffectList` (Xbox PDB, cdecl: argument, list): each effect
/// of the list runs its slot +0x30 with `argument` and is destroyed (slot 0 with 1); then
/// the list is cleared (`00470470`).
pub fn active_effect_revert_active_effect_list(e: &mut Engine, argument: u32, list: u32) {
    for_each_item(e, list, |e, effect| {
        e.vcall(effect, 0x30, &args![argument]);
        e.vcall(effect, 0, &args![1u32]);
    });
    e.call(HIT_LIST_00470470, &args![list]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_4b30, fn_00804b30(i32) -> bool),
        entry!(
            0x007a_7eb0,
            bs_simple_list_active_effect_insert(Ptr, u32, u32)
        ),
        entry!(
            0x0080_4090,
            active_effect_copy_data(Ptr<ActiveEffect>, Ptr<ActiveEffect>)
        ),
        entry!(0x0080_4160, active_effect_finalize(Ptr<ActiveEffect>)),
        entry!(0x0080_4210, active_effect_dispel(Ptr<ActiveEffect>, bool)),
        entry!(0x0080_4250, fn_00804250(Ptr<ActiveEffect>) -> bool),
        entry!(0x0080_43f0, fn_008043f0(Ptr<ActiveEffect>, f32, bool)),
        entry!(0x0080_4560, fn_00804560(Ptr<ActiveEffect>, f32)),
        entry!(0x0080_4b60, fn_00804b60(Ptr<ActiveEffect>) -> bool),
        entry!(0x0080_4b80, fn_00804b80(Ptr<ActiveEffect>, u32) -> bool),
        entry!(0x0080_4ba0, fn_00804ba0(Ptr<ActiveEffect>, u32)),
        entry!(
            0x0080_4bc0,
            active_effect_adjust_effectiveness(Ptr<ActiveEffect>, f32)
        ),
        entry!(0x0080_4cb0, fn_00804cb0(Ptr<ActiveEffect>, f32)),
        entry!(0x0080_4cd0, fn_00804cd0(Ptr<ActiveEffect>)),
        entry!(
            0x0080_6200,
            active_effect_check_deleted_caster(Ptr<ActiveEffect>, u32) -> bool
        ),
        entry!(
            0x0080_6230,
            active_effect_save_game_ov2(Ptr<ActiveEffect>, u32)
        ),
        entry!(0x0080_6840, fn_00806840(u32, Ptr<ActiveEffect>)),
        entry!(0x0080_6a10, active_effect_save_active_effect_list(u32, u32)),
        entry!(0x0080_3fe0, active_effect_copy(Ptr<ActiveEffect>) -> u32),
        entry!(
            0x0080_4f80,
            active_effect_get_save_size(Ptr<ActiveEffect>, u32) -> u16
        ),
        entry!(0x0080_50d0, active_effect_save_game(Ptr<ActiveEffect>, u32)),
        entry!(0x0080_5330, active_effect_load_game(Ptr<ActiveEffect>, u32)),
        entry!(0x0080_5650, fn_00805650(Ptr<ActiveEffect>)),
        entry!(
            0x0080_5670,
            active_effect_init_load_game(Ptr<ActiveEffect>, u32)
        ),
        entry!(
            0x0080_5760,
            active_effect_finish_init_load_game(Ptr<ActiveEffect>, u32)
        ),
        entry!(0x0080_5810, fn_00805810(Ptr<ActiveEffect>, u32) -> u16),
        entry!(
            0x0080_5880,
            active_effect_save_active_effect(Ptr<ActiveEffect>, u32)
        ),
        entry!(0x0080_5920, active_effect_load_active_effect(u32) -> u32),
        entry!(0x0080_5a00, fn_00805a00(u32, u32) -> u16),
        entry!(0x0080_5b40, fn_00805b40(u32, u32)),
        entry!(0x0080_5d40, fn_00805d40(u32, u32)),
        entry!(0x0080_60e0, fn_008060e0(u32, u32)),
        entry!(0x0080_6130, fn_00806130(u32, u32)),
        entry!(0x0080_61b0, fn_008061b0(u32, u32)),
        entry!(
            0x0080_6420,
            active_effect_load_game_ov2(Ptr<ActiveEffect>, u32)
        ),
        entry!(
            0x0080_66d0,
            active_effect_init_load_game_ov2(Ptr<ActiveEffect>, u32)
        ),
        entry!(
            0x0080_67b0,
            active_effect_finish_load_game(Ptr<ActiveEffect>, u32)
        ),
        entry!(0x0080_6910, active_effect_load_active_effect_ov2(u32) -> u32),
        entry!(0x0080_6a90, active_effect_load_active_effect_list(u32, u32)),
        entry!(0x0080_6b00, fn_00806b00(u32, u32)),
        entry!(
            0x0080_6b50,
            active_effect_finish_load_active_effect_list(u32, u32)
        ),
        entry!(
            0x0080_6ba0,
            active_effect_revert_active_effect_list(u32, u32)
        ),
    ]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_00804b30_accepts_only_1_4_and_10() {
        let mut e = Engine::new();
        for value in [
            -1i32,
            0,
            1,
            2,
            3,
            4,
            5,
            9,
            10,
            11,
            0x100,
            0x401,
            i32::MIN,
            i32::MAX,
        ] {
            let expected = matches!(value, 1 | 4 | 10);
            assert_eq!(
                e.call(0x0080_4b30, &args![value]).bool(),
                expected,
                "{value}"
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Calls = Vec<Vec<u32>>;

    fn answer(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax,
            ..Ret::default()
        });
    }

    fn answer_float(e: &mut Engine, addr: u32, st0: f64) {
        e.register_double(addr, move |_, _| Ret {
            st0,
            ..Ret::default()
        });
    }

    fn calls(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An object whose vtable has `slots` (byte offset, target); every other slot is 0.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x240);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this, vtable);
        this
    }

    fn effect(e: &mut Engine, slots: &[(u32, u32)]) -> Ptr<ActiveEffect> {
        Ptr::new(object(e, slots))
    }

    // ---- BSSimpleList<ActiveEffect *>::Insert ----

    const COMPARE: u32 = 0x0300_0000;

    fn compare_words(_: &mut Engine, a: &[u32]) -> Ret {
        Ret {
            eax: (a[0] as i32).wrapping_sub(a[1] as i32) as u32,
            ..Ret::default()
        }
    }

    /// Doubles `operator new` and the node constructor (which copies the item
    /// through the address it gets and clears the link).
    fn list_engine() -> Engine {
        let mut e = Engine::new();
        e.register(COMPARE, compare_words);
        e.register(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(LIST_NODE_INIT_00470440, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], item);
            e.mem.set_u32(a[0] + 4, 0);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(LIST_INSERT_FRONT_005AE3D0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn insert_ignores_a_null_item_and_fills_an_empty_head() {
        let mut e = list_engine();
        let head = e.mem.alloc(8);
        e.call(0x007a_7eb0, &args![head, 0u32, COMPARE]);
        assert_eq!(e.mem.u32(head), 0);
        e.call(0x007a_7eb0, &args![head, 5u32, COMPARE]);
        assert_eq!(e.mem.u32(head), 5);
        assert_eq!(e.mem.u32(head + 4), 0);
        assert!(calls(&e, OPERATOR_NEW).is_empty());
    }

    #[test]
    fn insert_appends_after_the_last_node_and_before_a_later_one() {
        let mut e = list_engine();
        let head = e.mem.alloc(8);
        e.mem.set_u32(head, 10);
        // 20 sorts after 10: appended.
        e.call(0x007a_7eb0, &args![head, 20u32, COMPARE]);
        let second = e.mem.u32(head + 4);
        assert_eq!(e.mem.u32(second), 20);
        // 15 sorts before 20: a node between them.
        e.call(0x007a_7eb0, &args![head, 15u32, COMPARE]);
        let middle = e.mem.u32(head + 4);
        assert_eq!(e.mem.u32(middle), 15);
        assert_eq!(e.mem.u32(middle + 4), second);
        // 5 sorts before the head: the list's front insert with the item's address.
        e.call(0x007a_7eb0, &args![head, 5u32, COMPARE]);
        let front = calls(&e, LIST_INSERT_FRONT_005AE3D0);
        assert_eq!(front.len(), 1);
        assert_eq!(front[0][0], head);
        assert_eq!(calls(&e, OPERATOR_NEW).len(), 2);
    }

    // ---- CopyData, flags, magnitude, caster ----

    #[test]
    fn copy_data_copies_the_state_and_quiets_float_nans() {
        let mut e = Engine::new();
        let from = e.mem.alloc(0x48);
        let to = e.mem.alloc(0x48);
        for offset in (0..0x48).step_by(4) {
            e.mem.set_u32(from + offset, 0x1111_0000 + offset);
            e.mem.set_u32(to + offset, 0xeeee_eeee);
        }
        e.mem.set_u32(from + 4, 0x7f80_0001);
        e.mem.set_u32(from + 0x10, 0x0403_0201);
        e.call(0x0080_4090, &args![from, to]);
        assert_eq!(e.mem.u32(to + 4), 0x7fc0_0001);
        assert_eq!(e.mem.u32(to + 0x10), 0x0403_0201);
        // Only the byte at +0x14 is copied.
        assert_eq!(e.mem.u32(to + 0x14), 0xeeee_ee14);
        for offset in [
            0x08u32, 0x0c, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x3c, 0x44,
        ] {
            assert_eq!(
                e.mem.u32(to + offset),
                0x1111_0000 + offset,
                "offset {offset:#x}"
            );
        }
        for offset in [0x00u32, 0x30, 0x34, 0x38, 0x40] {
            assert_eq!(e.mem.u32(to + offset), 0xeeee_eeee, "offset {offset:#x}");
        }
    }

    #[test]
    fn flag_helpers_test_and_clear_bits() {
        let mut e = Engine::new();
        let this = effect(&mut e, &[]);
        e.set(this, ActiveEffect::iFlags, 0x160);
        assert!(e.call(0x0080_4b60, &args![this]).bool());
        assert!(e.call(0x0080_4b80, &args![this, 0x40u32]).bool());
        assert!(!e.call(0x0080_4b80, &args![this, 0x81u32]).bool());
        e.call(0x0080_4ba0, &args![this, 0x140u32]);
        assert_eq!(e.get(this, ActiveEffect::iFlags), 0x20);
        assert!(!e.call(0x0080_4b60, &args![this]).bool());
    }

    #[test]
    fn set_magnitude_quiets_a_signalling_nan() {
        let mut e = Engine::new();
        let this = effect(&mut e, &[]);
        e.call(0x0080_4cb0, &args![this, 2.5f32]);
        assert_eq!(e.get(this, ActiveEffect::fMagnitude), 2.5);
        e.call(0x0080_4cb0, &args![this, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x1c), 0x7fc0_0001);
    }

    #[test]
    fn check_deleted_caster_clears_only_its_own_caster() {
        let mut e = Engine::new();
        let this = effect(&mut e, &[]);
        e.set(this, ActiveEffect::pCaster, 0x4444);
        assert!(!e.call(0x0080_6200, &args![this, 0x5555u32]).bool());
        assert_eq!(e.get(this, ActiveEffect::pCaster), 0x4444);
        assert!(e.call(0x0080_6200, &args![this, 0x4444u32]).bool());
        assert_eq!(e.get(this, ActiveEffect::pCaster), 0);
    }

    // ---- Finalize, Dispel ----

    #[test]
    fn finalize_reevaluates_only_when_allowed() {
        for (test, word, runs) in [(0u32, 0u32, false), (1, 0, true), (0, 7, true)] {
            let mut e = Engine::new();
            let this = effect(&mut e, &[]);
            answer(&mut e, EFFECT_TEST_007DF210, test);
            answer(&mut e, EFFECT_VALUE_008041A0, word);
            answer(&mut e, EFFECT_TARGET_0059BB30, 0);
            e.call_log = Some(vec![]);
            e.call(0x0080_4160, &args![this]);
            assert_eq!(!calls(&e, EFFECT_TARGET_0059BB30).is_empty(), runs);
        }
    }

    #[test]
    fn dispel_marks_done_and_updates_when_asked() {
        for now in [false, true] {
            let mut e = Engine::new();
            let this = effect(&mut e, &[]);
            answer(&mut e, EFFECT_FLAG_00803E80, 0);
            answer(&mut e, EFFECT_TEST_007DF210, 0);
            answer(&mut e, EFFECT_VALUE_008041A0, 0);
            e.map(ZERO, 8);
            e.call_log = Some(vec![]);
            e.call(0x0080_4210, &args![this, u32::from(now)]);
            assert_eq!(e.get(this, ActiveEffect::bDone), 1);
            assert_eq!(
                calls(&e, EFFECT_FLAG_00803E80),
                vec![vec![this.addr(), 0x200]]
            );
            assert_eq!(calls(&e, EFFECT_TEST_007DF210).len(), usize::from(now));
        }
    }

    // ---- fn_00804250 ----

    const TARGET_IS_ACTOR: u32 = 0x0300_0100;
    const ACTOR_VALUE: u32 = 0x0300_0104;
    const LIST_FIRST: u32 = 0x0300_0108;
    const SPELL_KIND: u32 = 0x0300_010c;

    /// An effect whose target object's list holds one item whose spell has
    /// type `item_spell_kind`; the target is an actor answering `actor_value`.
    fn blocking_setup(
        e: &mut Engine,
        item_spell_kind: u32,
        actor_value: i32,
    ) -> (Ptr<ActiveEffect>, u32) {
        let this = effect(e, &[]);
        // Target (MagicTarget at actor + 0x94): slot +0xc says actor.
        let actor = e.mem.alloc(0x200);
        let target_vtable = e.mem.alloc(0x10);
        e.mem.set_u32(target_vtable + 0xc, TARGET_IS_ACTOR);
        e.mem.set_u32(actor + 0x94, target_vtable);
        let value_vtable = e.mem.alloc(0x10);
        e.mem.set_u32(value_vtable + 8, ACTOR_VALUE);
        e.mem.set_u32(actor + 0xa4, value_vtable);
        e.set(this, ActiveEffect::pTarget, actor + 0x94);
        answer(e, TARGET_IS_ACTOR, 1);
        answer(e, ACTOR_VALUE, actor_value as u32);
        let target_object = object(e, &[(8, LIST_FIRST)]);
        answer(e, EFFECT_TARGET_0059BB30, target_object);
        let node = e.mem.alloc(8);
        answer(e, LIST_FIRST, node);
        let item = e.mem.alloc(0x10);
        let item_spell = object(e, &[(0x18, SPELL_KIND)]);
        answer(e, SPELL_KIND, item_spell_kind);
        e.mem.set_u32(item + 8, item_spell);
        e.mem.set_u32(node, item);
        e.register(LIST_NODE_ITEM, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        answer(e, LIST_NODE_NEXT, 0);
        answer(e, ITEM_TEST_00833C20, 1);
        (this, item_spell)
    }

    #[test]
    fn fn_00804250_finds_the_displacement_spell_unless_excluded() {
        for (value, want) in [(0i32, true), (1, false)] {
            let mut e = Engine::new();
            let (this, item_spell) = blocking_setup(&mut e, 0, value);
            answer(&mut e, RT_DYNAMIC_CAST, 0);
            e.set(this, ActiveEffect::pDisplacementSpell, item_spell);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0080_4250, &args![this]).bool(), want);
            let value_calls = calls(&e, ACTOR_VALUE);
            assert_eq!(value_calls.len(), 1);
            assert_eq!(value_calls[0][1], 0x48);
        }
        // No displacement spell: the list is not looked at.
        let mut e = Engine::new();
        let (this, _) = blocking_setup(&mut e, 0, 0);
        answer(&mut e, RT_DYNAMIC_CAST, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0080_4250, &args![this]).bool());
        assert!(calls(&e, LIST_FIRST).is_empty());
    }

    #[test]
    fn fn_00804250_matches_an_alchemy_item_by_its_word() {
        for (word, want) in [(0x7777u32, true), (0x7778, false)] {
            let mut e = Engine::new();
            let (this, _) = blocking_setup(&mut e, 7, 0);
            e.set(this, ActiveEffect::pDisplacementSpell, 0x9999);
            e.set(this, ActiveEffect::pSpell, 0x1234);
            e.register(RT_DYNAMIC_CAST, |_, a| Ret {
                eax: if a[3] == SPELL_ITEM_TYPE {
                    0x7777
                } else {
                    0x6666
                },
                ..Ret::default()
            });
            answer(&mut e, ALCHEMY_VALUE_004030B0, word);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0080_4250, &args![this]).bool(), want);
            let casts = calls(&e, RT_DYNAMIC_CAST);
            assert_eq!(
                casts[0],
                vec![0x1234, 0, MAGIC_ITEM_TYPE, SPELL_ITEM_TYPE, 0]
            );
            assert_eq!(casts[1][2..4], [MAGIC_ITEM_TYPE, ALCHEMY_ITEM_TYPE]);
            assert_eq!(calls(&e, ALCHEMY_VALUE_004030B0), vec![vec![0x6666]]);
        }
    }

    // ---- fn_008043f0 ----

    const TARGET_REFERENCE: u32 = 0x0300_0200;

    /// The callees of `fn_008043f0` with the target object's slot +4 returning
    /// `reference`, the condition answering `holds`, and nothing blocking.
    fn evaluate_setup(e: &mut Engine, reference: u32, holds: u32) -> Ptr<ActiveEffect> {
        let this = effect(e, &[]);
        answer(e, EFFECT_TEST_007DF210, 1);
        answer(e, EFFECT_VALUE_008041A0, 0);
        let target_object = object(e, &[(4, TARGET_REFERENCE)]);
        answer(e, EFFECT_TARGET_0059BB30, target_object);
        answer(e, TARGET_REFERENCE, reference);
        let scale = e.mem.alloc(4);
        e.mem.set_f32(scale, 2.0);
        answer(e, FLOAT_ADDRESS_00403E20, scale);
        answer(e, GETTER_0084E3A0, 0x5100);
        answer(e, GETTER_00409140, 0x5200);
        answer(e, TES_CONDITION_IS_TRUE, holds);
        // fn_00804250: the target is no actor, no displacement spell.
        let target = object(e, &[(0xc, TARGET_IS_ACTOR)]);
        answer(e, TARGET_IS_ACTOR, 0);
        e.set(this, ActiveEffect::pTarget, target);
        answer(e, RT_DYNAMIC_CAST, 0);
        this
    }

    #[test]
    fn fn_008043f0_forced_sets_the_active_state_and_reopens_a_finished_effect() {
        let mut e = Engine::new();
        let this = evaluate_setup(&mut e, 0x4000, 1);
        e.set(this, ActiveEffect::bStarted, 1);
        e.set(this, ActiveEffect::bFinished, 1);
        e.call_log = Some(vec![]);
        e.call(0x0080_43f0, &args![this, 0.0f32, 1u32]);
        assert_eq!(e.get(this, ActiveEffect::bActive), 1);
        assert_eq!(e.get(this, ActiveEffect::bStarted), 0);
        assert_eq!(e.get(this, ActiveEffect::bFinished), 0);
        assert_eq!(
            calls(&e, TES_CONDITION_IS_TRUE),
            vec![vec![0x5200, 0x4000, 0]]
        );
        assert_eq!(
            calls(&e, FLOAT_ADDRESS_00403E20),
            vec![vec![FLOAT_OWNER_011D17DC]]
        );
        // The condition fails: inactive, the other state kept.
        let mut e = Engine::new();
        let this = evaluate_setup(&mut e, 0x4000, 0);
        e.set(this, ActiveEffect::bActive, 1);
        e.set(this, ActiveEffect::bFinished, 1);
        e.call(0x0080_43f0, &args![this, 0.0f32, 1u32]);
        assert_eq!(e.get(this, ActiveEffect::bActive), 0);
        assert_eq!(e.get(this, ActiveEffect::bFinished), 1);
    }

    #[test]
    fn fn_008043f0_unforced_needs_a_changed_boundary() {
        for (first, second, changes) in [
            (3.0f64, 3.0f64, false),
            (3.0, 4.0, true),
            (f64::NAN, f64::NAN, true),
        ] {
            let mut e = Engine::new();
            let this = evaluate_setup(&mut e, 0x4000, 1);
            answer_float(&mut e, EFFECT_REAL_006B9130, 5.0);
            let mut n = 0;
            e.register_double(FLOAT_FUNCTION_00406CC0, move |_, _| {
                n += 1;
                Ret {
                    st0: if n == 1 { first } else { second },
                    ..Ret::default()
                }
            });
            e.call_log = Some(vec![]);
            e.call(0x0080_43f0, &args![this, 1.0f32, 0u32]);
            // 5 / 2 and (5 + 1) / 2, as floats.
            let floors = calls(&e, FLOAT_FUNCTION_00406CC0);
            assert_eq!(floors, vec![vec![2.5f32.to_bits()], vec![3.0f32.to_bits()]]);
            assert_eq!(e.get(this, ActiveEffect::bActive), u8::from(changes));
        }
    }

    #[test]
    fn fn_008043f0_returns_early_without_a_reference() {
        let mut e = Engine::new();
        let this = evaluate_setup(&mut e, 0, 1);
        e.call_log = Some(vec![]);
        e.call(0x0080_43f0, &args![this, 0.0f32, 1u32]);
        assert!(calls(&e, FLOAT_ADDRESS_00403E20).is_empty());
        assert_eq!(e.get(this, ActiveEffect::bActive), 0);
    }

    // ---- fn_00804560 ----

    const START: u32 = 0x0300_0300;
    const UPDATE: u32 = 0x0300_0304;
    const FINISH: u32 = 0x0300_0308;
    const OWNER_KIND: u32 = 0x0300_030c;
    const ACTOR_SLOT_22C: u32 = 0x0300_0310;

    /// An effect for `fn_00804560`: conditions not re-evaluated, an owner of
    /// type `owner_kind`, a target that is no actor, duration `duration`,
    /// `006b9130` 0.
    fn update_setup(e: &mut Engine, owner_kind: u32, duration: f64) -> Ptr<ActiveEffect> {
        let this = effect(e, &[(0xc, UPDATE), (0x50, START), (0x58, FINISH)]);
        for f in [START, UPDATE, FINISH] {
            answer(e, f, 0);
        }
        answer(e, EFFECT_TEST_007DF210, 0);
        answer(e, EFFECT_VALUE_008041A0, 0);
        e.map(ZERO, 8);
        e.mem.set_f64(ZERO, 0.0);
        e.map(HUNDRED, 8);
        e.mem.set_f64(HUNDRED, 100.0);
        e.map(PLAYER_POINTER, 4);
        e.mem.set_u32(PLAYER_POINTER, 0x0abc_0000);
        let owner = object(e, &[(0x18, OWNER_KIND)]);
        answer(e, OWNER_KIND, owner_kind);
        answer(e, GETTER_0044DDC0, owner);
        let target_object = object(e, &[(4, TARGET_REFERENCE)]);
        answer(e, EFFECT_TARGET_0059BB30, target_object);
        answer(e, TARGET_REFERENCE, 0x0def_0000);
        let target = object(e, &[(0xc, TARGET_IS_ACTOR)]);
        answer(e, TARGET_IS_ACTOR, 0);
        e.set(this, ActiveEffect::pTarget, target);
        answer(e, EFFECT_POINTER_007DF1D0, 0x5300);
        answer(e, FLAG_TEST_00403E60, 0);
        answer(e, MAGIC_HIT_EFFECT_INSTANTIATE, 0x5400);
        answer(e, TEST_008C7AA0, 0);
        answer_float(e, EFFECT_REAL_006B9130, 0.0);
        answer_float(e, EFFECT_REAL_007DF1F0, duration);
        answer(e, GETTER_00825C00, 0x5500);
        // fn_00804cd0 stops at once: no sound id.
        answer(e, GETTER_004FD3E0, 0);
        answer(e, BS_SOUND_HANDLE_IS_VALID, 1);
        answer(e, BS_SOUND_HANDLE_STOP, 0);
        this
    }

    #[test]
    fn fn_00804560_starts_an_active_effect_and_advances_it() {
        let mut e = Engine::new();
        let this = update_setup(&mut e, 0, 10.0);
        e.set(this, ActiveEffect::bActive, 1);
        e.set(this, ActiveEffect::fElapsedSeconds, 1.0);
        e.call_log = Some(vec![]);
        e.call(0x0080_4560, &args![this, 0.5f32]);
        assert_eq!(calls(&e, START).len(), 1);
        assert_eq!(e.get(this, ActiveEffect::bStarted), 1);
        assert_eq!(e.get(this, ActiveEffect::pHitEffects), 0x5400);
        assert_eq!(e.get(this, ActiveEffect::fElapsedSeconds), 1.5);
        assert_eq!(calls(&e, UPDATE), vec![vec![this.addr(), 0.5f32.to_bits()]]);
        assert_eq!(e.get(this, ActiveEffect::bDone), 0);
        assert!(calls(&e, FINISH).is_empty());
        // The setting flag 0x8000000 was asked, then 0x80 for done.
        let flags: Vec<u32> = calls(&e, FLAG_TEST_00403E60).iter().map(|c| c[1]).collect();
        assert_eq!(flags, vec![0x0800_0000, 0x80]);
    }

    #[test]
    fn fn_00804560_clamps_to_the_duration_and_finishes() {
        let mut e = Engine::new();
        let this = update_setup(&mut e, 0, 2.0);
        answer_float(&mut e, EFFECT_REAL_006B9130, 1.75);
        e.set(this, ActiveEffect::bActive, 1);
        e.set(this, ActiveEffect::bStarted, 1);
        e.set(this, ActiveEffect::fElapsedSeconds, 1.75);
        e.call_log = Some(vec![]);
        e.call(0x0080_4560, &args![this, 0.5f32]);
        assert!(calls(&e, START).is_empty());
        // Clamped: elapsed becomes the duration, the step what was left of it.
        assert_eq!(e.get(this, ActiveEffect::fElapsedSeconds), 2.0);
        assert_eq!(
            calls(&e, UPDATE),
            vec![vec![this.addr(), 0.25f32.to_bits()]]
        );
        assert_eq!(e.get(this, ActiveEffect::bDone), 1);
        assert_eq!(calls(&e, FINISH).len(), 1);
        assert_eq!(e.get(this, ActiveEffect::bFinished), 1);
        assert_eq!(
            calls(&e, BS_SOUND_HANDLE_STOP),
            vec![vec![this.addr() + PERSISTENT_SOUND]]
        );
    }

    #[test]
    fn fn_00804560_without_time_only_finishes_an_inactive_started_effect() {
        let mut e = Engine::new();
        let this = update_setup(&mut e, 0, 2.0);
        e.set(this, ActiveEffect::bStarted, 1);
        e.call_log = Some(vec![]);
        e.call(0x0080_4560, &args![this, 0.0f32]);
        assert!(calls(&e, UPDATE).is_empty());
        assert_eq!(calls(&e, FINISH).len(), 1);
        assert_eq!(e.get(this, ActiveEffect::bFinished), 1);
        // Already finished: nothing more.
        e.call_log = Some(vec![]);
        e.call(0x0080_4560, &args![this, 0.0f32]);
        assert!(calls(&e, FINISH).is_empty());
    }

    #[test]
    fn fn_00804560_marks_done_for_an_actor_target() {
        let mut e = Engine::new();
        let this = update_setup(&mut e, 0, 10.0);
        let actor = object(&mut e, &[(0x22c, ACTOR_SLOT_22C)]);
        let target_vtable = e.mem.alloc(0x10);
        e.mem.set_u32(target_vtable + 0xc, TARGET_IS_ACTOR);
        e.mem.set_u32(actor + 0x94, target_vtable);
        e.set(this, ActiveEffect::pTarget, actor + 0x94);
        answer(&mut e, TARGET_IS_ACTOR, 1);
        answer(&mut e, ACTOR_SLOT_22C, 1);
        e.set(this, ActiveEffect::bActive, 1);
        e.set(this, ActiveEffect::bStarted, 1);
        e.call_log = Some(vec![]);
        e.call(0x0080_4560, &args![this, 0.5f32]);
        assert_eq!(calls(&e, ACTOR_SLOT_22C), vec![vec![actor, 1]]);
        assert_eq!(e.get(this, ActiveEffect::bDone), 1);
        assert_eq!(calls(&e, FINISH).len(), 1);
    }

    #[test]
    fn fn_00804560_runs_delayed_hit_effects_for_the_player_outside_menus() {
        let mut e = Engine::new();
        let this = update_setup(&mut e, 0, 10.0);
        answer(&mut e, TARGET_REFERENCE, 0x0abc_0000);
        answer(&mut e, INTERFACE_IS_IN_MENU_MODE, 0);
        e.set(this, ActiveEffect::bActive, 1);
        e.set(this, ActiveEffect::bStarted, 1);
        e.set(this, ActiveEffect::iFlags, 0x60);
        e.set(this, ActiveEffect::pHitEffects, 0x5600);
        answer(&mut e, HIT_LIST_00470470, 0);
        answer(&mut e, HIT_LIST_004702F0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0080_4560, &args![this, 0.5f32]);
        assert_eq!(calls(&e, HIT_LIST_00470470), vec![vec![0x5600]]);
        assert_eq!(calls(&e, HIT_LIST_004702F0), vec![vec![0x5600, 1]]);
        assert_eq!(e.get(this, ActiveEffect::pHitEffects), 0x5400);
        assert_eq!(e.get(this, ActiveEffect::iFlags), 0);
    }

    // ---- AdjustEffectiveness ----

    fn adjust_setup(e: &mut Engine, flag_180: u32, flag_100: u32) -> Ptr<ActiveEffect> {
        let this = effect(e, &[]);
        e.map(ONE, 8);
        e.mem.set_f64(ONE, 1.0);
        answer(e, GETTER_0084E3A0, 0x5100);
        answer(e, GETTER_00825C00, 0x5200);
        answer(e, FLAG_TEST_00408680, flag_180);
        answer(e, FLAG_TEST_00403E60, flag_100);
        answer(e, SET_REAL_00693D50, 0);
        answer_float(e, EFFECT_REAL_007DF1F0, 4.0);
        answer_float(e, EFFECT_REAL_00495460, 6.0);
        e.register(FLOAT_FUNCTION_00404040, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        this
    }

    #[test]
    fn adjust_effectiveness_skips_flagged_settings_and_factor_one() {
        let mut e = Engine::new();
        let this = adjust_setup(&mut e, 1, 0);
        e.call(0x0080_4bc0, &args![this, 0.5f32]);
        assert_eq!(calls(&e, FLAG_TEST_00408680), vec![vec![0x5200, 0x180]]);
        assert!(calls(&e, FLAG_TEST_00403E60).is_empty());
        let mut e = Engine::new();
        let this = adjust_setup(&mut e, 0, 0);
        e.call(0x0080_4bc0, &args![this, 1.0f32]);
        assert!(calls(&e, FLAG_TEST_00403E60).is_empty());
    }

    #[test]
    fn adjust_effectiveness_scales_the_duration_or_the_magnitude() {
        let mut e = Engine::new();
        let this = adjust_setup(&mut e, 0, 1);
        e.call(0x0080_4bc0, &args![this, 0.5f32]);
        assert_eq!(
            calls(&e, SET_REAL_00693D50),
            vec![vec![this.addr(), 2.0f32.to_bits()]]
        );
        // Magnitude 6 * 0.5 = 3.
        let mut e = Engine::new();
        let this = adjust_setup(&mut e, 0, 0);
        e.call(0x0080_4bc0, &args![this, 0.5f32]);
        assert_eq!(e.get(this, ActiveEffect::fMagnitude), 3.0);
        assert_eq!(calls(&e, FLOAT_FUNCTION_00404040).len(), 2);
        // 6 * 0.125 is below 1: the magnitude is 1.
        let mut e = Engine::new();
        let this = adjust_setup(&mut e, 0, 0);
        e.call(0x0080_4bc0, &args![this, 0.125f32]);
        assert_eq!(e.get(this, ActiveEffect::fMagnitude), 1.0);
        assert_eq!(calls(&e, FLOAT_FUNCTION_00404040).len(), 1);
    }

    // ---- fn_00804cd0 ----

    const POSITION_SLOT: u32 = 0x0300_0400;

    fn sound_setup(e: &mut Engine, actor: u32, looping: u32) -> Ptr<ActiveEffect> {
        let this = effect(e, &[]);
        answer(e, EFFECT_POINTER_007DF1D0, 0x5300);
        e.register(GETTER_004FD3E0, |_, a| Ret {
            eax: if a[0] == 0x5300 { 0x5310 } else { 0 },
            ..Ret::default()
        });
        answer(e, GETTER_0084E3A0, 0x77);
        let reference = object(e, &[(0x1f4, POSITION_SLOT)]);
        let target_object = object(e, &[(4, TARGET_REFERENCE)]);
        answer(e, TARGET_REFERENCE, reference);
        answer(e, EFFECT_TARGET_0059BB30, target_object);
        let owner = object(e, &[(0x18, OWNER_KIND)]);
        answer(e, OWNER_KIND, 0);
        answer(e, GETTER_0044DDC0, owner);
        answer(e, MAGIC_TARGET_GET_TARGET_AS_ACTOR, actor);
        answer(e, FLAG_TEST_00403E60, 0);
        let filled = e.mem.alloc(8);
        e.mem.set_u32(filled + 4, 0x99);
        answer(e, FILL_004E75D0, filled);
        answer(e, WORD_FUNCTION_005E39B0, looping);
        let position = e.mem.alloc(12);
        e.mem.set_f32(position, 1.0);
        e.mem.set_f32(position + 4, 2.0);
        e.mem.set_u32(position + 8, 0x7f80_0001);
        answer(e, POSITION_SLOT, position);
        for f in [
            SOUND_HANDLE_INIT_0041A250,
            ACTOR_SOUND_00933150,
            SOUND_HANDLE_ASSIGN_00418900,
            SOUND_HANDLE_DESTROY_00483710,
            BS_AUDIO_GET_SOUND_HANDLE_BY_NUMERIC_ID,
            BS_SOUND_HANDLE_SET_POSITION,
            BS_SOUND_HANDLE_PLAY,
            BS_SOUND_HANDLE_STOP,
            BS_SOUND_HANDLE_RELEASE,
        ] {
            answer(e, f, 0);
        }
        answer(e, BS_AUDIO_Q_INSTANCE, 0x5800);
        answer(e, BS_SOUND_HANDLE_IS_VALID, 1);
        e.call_log = Some(vec![]);
        this
    }

    #[test]
    fn fn_00804cd0_needs_a_sound_id() {
        let mut e = Engine::new();
        let this = sound_setup(&mut e, 0, 0);
        answer(&mut e, GETTER_004FD3E0, 0);
        e.call(0x0080_4cd0, &args![this]);
        assert!(calls(&e, SOUND_HANDLE_INIT_0041A250).is_empty());
        assert!(calls(&e, GETTER_0084E3A0).is_empty());
    }

    #[test]
    fn fn_00804cd0_plays_through_the_actor_and_keeps_a_looping_sound() {
        let mut e = Engine::new();
        let this = sound_setup(&mut e, 0x6000, 1);
        e.call(0x0080_4cd0, &args![this]);
        let sound = calls(&e, SOUND_HANDLE_INIT_0041A250)[0][0];
        let played = calls(&e, ACTOR_SOUND_00933150);
        assert_eq!(played.len(), 1);
        assert_eq!(played[0][0], 0x6000);
        assert_eq!(played[0][2..], [0x77, 1, 0x112, 0]);
        assert_eq!(calls(&e, FILL_004E75D0)[0][0], 0x5310);
        assert_eq!(calls(&e, WORD_FUNCTION_005E39B0), vec![vec![0x99]]);
        let persistent = this.addr() + PERSISTENT_SOUND;
        assert_eq!(calls(&e, BS_SOUND_HANDLE_STOP), vec![vec![persistent]]);
        assert_eq!(calls(&e, BS_SOUND_HANDLE_RELEASE), vec![vec![persistent]]);
        let assigned = calls(&e, SOUND_HANDLE_ASSIGN_00418900);
        assert_eq!(assigned[0][0], sound);
        assert_eq!(assigned[1], vec![persistent, sound]);
        assert!(calls(&e, BS_AUDIO_Q_INSTANCE).is_empty());
    }

    #[test]
    fn fn_00804cd0_plays_at_the_reference_position_otherwise() {
        let mut e = Engine::new();
        let this = sound_setup(&mut e, 0, 0);
        e.call(0x0080_4cd0, &args![this]);
        let handle = calls(&e, BS_AUDIO_GET_SOUND_HANDLE_BY_NUMERIC_ID);
        assert_eq!(handle.len(), 1);
        assert_eq!(handle[0][0], 0x5800);
        assert_eq!(handle[0][2..], [0x77, 0x102]);
        let position = calls(&e, BS_SOUND_HANDLE_SET_POSITION);
        assert_eq!(
            position[0][1..],
            [1.0f32.to_bits(), 2.0f32.to_bits(), 0x7fc0_0001]
        );
        assert_eq!(calls(&e, BS_SOUND_HANDLE_PLAY)[0][1], 0);
        // Not looping: the persistent sound is left alone.
        assert!(calls(&e, BS_SOUND_HANDLE_STOP).is_empty());
        assert!(calls(&e, ACTOR_SOUND_00933150).is_empty());
    }

    // ---- saving ----

    const KIND_SLOT: u32 = 0x0300_0500;
    const SAVE_SLOT: u32 = 0x0300_0504;

    fn save_engine() -> Engine {
        let mut e = Engine::new();
        for f in [
            SAVE_DATA_00865E50,
            SAVE_FORM_ID,
            SAVE_FORM_ID_OV2,
            SAVE_VARIABLE_SIZED_VALUE,
            SAVE_VARIABLE_SIZED_VALUE_OV2,
            SAVE_SLOT,
        ] {
            answer(&mut e, f, 0);
        }
        answer(&mut e, START_VARIABLE_SIZED_VALUE, 0x88);
        answer(&mut e, MAGIC_CASTER_GET_FORM_ID, 0x0c00);
        answer(&mut e, MAGIC_TARGET_GET_FORM_ID, 0x0d00);
        answer(&mut e, MAGIC_ITEM_GET_FORM_ID, 0x0e00);
        answer(&mut e, KIND_SLOT, 3);
        e.register(LIST_NODE_ITEM, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(LIST_NODE_NEXT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e
    }

    /// A list of `items` (8-byte nodes: item, next).
    fn list_of(e: &mut Engine, items: &[u32]) -> u32 {
        let mut next = 0;
        for item in items.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    #[test]
    fn save_game_writes_fields_ids_and_hit_effects() {
        let mut e = save_engine();
        let this = effect(&mut e, &[]);
        e.set(this, ActiveEffect::pTarget, 0x2400);
        e.set(this, ActiveEffect::pSource, 0x3c00);
        let hit = object(&mut e, &[(0x9c, KIND_SLOT), (0xa8, SAVE_SLOT)]);
        let list = list_of(&mut e, &[hit, 0, hit]);
        e.set(this, ActiveEffect::pHitEffects, list);
        e.call_log = Some(vec![]);
        e.call(0x0080_6230, &args![this, 0xb000u32]);
        let base = this.addr();
        let data = calls(&e, SAVE_DATA_00865E50);
        let fields: Vec<(u32, u32)> = data[..9].iter().map(|c| (c[1] - base, c[2])).collect();
        assert_eq!(
            fields,
            vec![
                (4, 4),
                (0x10, 1),
                (0x11, 1),
                (0x12, 1),
                (0x13, 1),
                (0x14, 1),
                (0x1c, 4),
                (0x20, 4),
                (0x18, 4)
            ]
        );
        assert_eq!(data.len(), 11);
        assert_eq!(data[9][2], 1);
        assert_eq!(
            calls(&e, SAVE_FORM_ID),
            vec![
                vec![0xb000, 0, 0],
                vec![0xb000, 0x0d00, 0],
                vec![0xb000, 0, 0]
            ]
        );
        assert!(calls(&e, MAGIC_CASTER_GET_FORM_ID).is_empty());
        assert_eq!(calls(&e, MAGIC_TARGET_GET_FORM_ID), vec![vec![0x2400]]);
        assert_eq!(calls(&e, SAVE_FORM_ID_OV2), vec![vec![0xb000, 0x3c00, 0]]);
        assert_eq!(
            calls(&e, SAVE_SLOT),
            vec![vec![hit, 0xb000], vec![hit, 0xb000]]
        );
        assert_eq!(
            calls(&e, SAVE_VARIABLE_SIZED_VALUE_OV2),
            vec![vec![0xb000, 2, 0x88]]
        );
    }

    #[test]
    fn save_effect_and_effect_list_write_each_effect() {
        let mut e = save_engine();
        let first = effect(&mut e, &[(0x14, SAVE_SLOT)]);
        e.set(first, ActiveEffect::pSpell, 0x7000);
        e.set(first, ActiveEffect::pEffect, 0x7100);
        let second = effect(&mut e, &[(0x14, SAVE_SLOT)]);
        e.set(second, ActiveEffect::pSpell, 0);
        e.set(second, ActiveEffect::pEffect, 0x7200);
        answer(&mut e, EFFECT_ITEM_LIST_GET_INDEX, 0x105);
        answer(&mut e, GETTER_00825C00, 0x7300);
        answer(&mut e, GETTER_00810FD0, 0x42);
        let mut position = 0x100u32;
        e.register_double(GETTER_0084E3A0, move |_, _| {
            position += 0x10;
            Ret {
                eax: position,
                ..Ret::default()
            }
        });
        let list = list_of(&mut e, &[first.addr(), 0, second.addr()]);
        e.call_log = Some(vec![]);
        e.call(0x0080_6a10, &args![0xb000u32, list]);
        assert_eq!(
            calls(&e, SAVE_FORM_ID),
            vec![vec![0xb000, 0x0e00, 0], vec![0xb000, 0, 0]]
        );
        let index = calls(&e, EFFECT_ITEM_LIST_GET_INDEX);
        assert_eq!(index, vec![vec![0x700c, 0x7100], vec![0xc, 0x7200]]);
        assert_eq!(
            calls(&e, SAVE_VARIABLE_SIZED_VALUE),
            vec![vec![0xb000, 0x42], vec![0xb000, 0x42]]
        );
        assert_eq!(
            calls(&e, SAVE_SLOT),
            vec![vec![first.addr(), 0xb000], vec![second.addr(), 0xb000]]
        );
        // Each effect's value closes with its byte count; the list's with the count.
        assert_eq!(
            calls(&e, SAVE_VARIABLE_SIZED_VALUE_OV2),
            vec![
                vec![0xb000, 0x10, 0x88],
                vec![0xb000, 0x10, 0x88],
                vec![0xb000, 2, 0x88]
            ]
        );
    }
}

#[cfg(test)]
mod save_load_tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    type Calls = Vec<Vec<u32>>;
    type Writes = Rc<RefCell<Vec<Vec<u8>>>>;

    fn calls(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn answer(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax,
            ..Ret::default()
        });
    }

    /// An object whose vtable slots (byte offsets) are doubles at `0x0300_0000 + slot`
    /// answering 0.
    fn object(e: &mut Engine, slots: &[u32]) -> u32 {
        let vtable = e.mem.alloc(0x600);
        for slot in slots {
            e.mem.set_u32(vtable + slot, 0x0300_0000 + slot);
            if !e.is_translated(0x0300_0000 + slot) {
                answer(e, 0x0300_0000 + slot, 0);
            }
        }
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this, vtable);
        this
    }

    /// A list of `items` (8-byte nodes: item, next).
    fn list(e: &mut Engine, items: &[u32]) -> u32 {
        let mut next = 0;
        for item in items.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    /// The save-load game (version `version`, its position a scratch buffer) and the list
    /// node doubles.
    fn rig(version: u8) -> Engine {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        let game = e.mem.alloc(0x100);
        e.set_global(SAVE_LOAD_GAME_POINTER, game);
        answer(&mut e, SAVE_VERSION_008DF040, u32::from(version));
        e.register(LIST_NODE_IS_EMPTY, |e, a| {
            u8::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(LIST_NODE_ITEM, |_, a| a[0].into_ret());
        e.register(LIST_NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.call_log = Some(vec![]);
        e
    }

    /// Records the bytes each write (`008579b0`, `SaveNumericID`) passes.
    fn record_writes(e: &mut Engine) -> Writes {
        let writes: Writes = Rc::default();
        for addr in [SAVE_BYTES_008579B0, SAVE_NUMERIC_ID] {
            let log = Rc::clone(&writes);
            e.register_double(addr, move |e, a| {
                log.borrow_mut().push(e.mem.bytes(a[1], a[2]));
                Ret::default()
            });
        }
        writes
    }

    /// Feeds `data` to the reads (`008579e0`, `LoadNumericID`, `00864980`; the address and
    /// size are the last two words).
    fn feed(e: &mut Engine, data: &[u8]) {
        let stream = Rc::new(RefCell::new(data.iter().copied().collect::<VecDeque<u8>>()));
        for addr in [LOAD_BYTES_008579E0, LOAD_NUMERIC_ID, LOAD_DATA_00864980] {
            let stream = Rc::clone(&stream);
            e.register_double(addr, move |e, a| {
                let (address, size) = (a[1], a[2]);
                for i in 0..size {
                    let byte = stream.borrow_mut().pop_front().unwrap();
                    e.mem.set_u8(address + i, byte);
                }
                Ret::default()
            });
        }
    }

    fn effect(e: &mut Engine) -> Ptr<ActiveEffect> {
        Ptr::new(object(e, &[0x10, 0x18, 0x44]))
    }

    #[test]
    fn copy_makes_a_new_effect_and_hands_it_to_copy_data() {
        let mut e = rig(0);
        let this = effect(&mut e);
        e.set(this, ActiveEffect::pCaster, 0x28);
        e.set(this, ActiveEffect::pSpell, 0x08);
        e.set(this, ActiveEffect::pEffect, 0x0c);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ACTIVE_EFFECT_CONSTRUCTOR_00803D30, |_, a| a[0].into_ret());
        let copy = e.call(0x0080_3fe0, &args![this]).u32();
        assert_ne!(copy, 0);
        assert_eq!(
            calls(&e, ACTIVE_EFFECT_CONSTRUCTOR_00803D30),
            vec![vec![copy, 0x28, 0x08, 0x0c]]
        );
        assert_eq!(calls(&e, 0x0300_0044), vec![vec![this.addr(), copy]]);
        // A failed allocation copies into null.
        answer(&mut e, OPERATOR_NEW, 0);
        assert_eq!(e.call(0x0080_3fe0, &args![this]).u32(), 0);
        assert_eq!(calls(&e, ACTIVE_EFFECT_CONSTRUCTOR_00803D30).len(), 1);
        assert_eq!(calls(&e, 0x0300_0044)[1], vec![this.addr(), 0]);
    }

    #[test]
    fn save_size_counts_hit_effects_and_the_version_tail() {
        for (version, expected) in [
            (0x48u8, 28 + 1 + 4 + 6 + 4),
            (0x41, 28 + 1 + 4 + 6 + 1),
            (0x29, 28),
        ] {
            let mut e = rig(version);
            let this = effect(&mut e);
            let first = object(&mut e, &[0xd0]);
            let second = object(&mut e, &[0xd0]);
            let head = list(&mut e, &[first, second]);
            e.set(this, ActiveEffect::pHitEffects, head);
            // Slot +0xd0 answers 3 for the first, 0x10005 (AX 5) for the second.
            e.register_double(0x0300_00d0, move |_, a| {
                (if a[0] == first { 3u32 } else { 0x0001_0005 }).into_ret()
            });
            let size = e.call(0x0080_4f80, &args![this, 0x77u32]).u16();
            assert_eq!(size, expected, "version {version:#x}");
            if version >= 0x2a {
                assert_eq!(calls(&e, 0x0300_00d0)[0], vec![first, this.addr(), 0x77]);
            }
        }
    }

    #[test]
    fn save_game_writes_the_fields_ids_hit_effects_and_flag() {
        let mut e = rig(0x41);
        let writes = record_writes(&mut e);
        let this = effect(&mut e);
        let base = this.addr();
        e.mem.set_u32(base + 4, 0x1111_1111);
        e.mem.write(base + 0x10, &[1, 2, 3, 4, 5]);
        e.mem.set_u32(base + 0x1c, 0x2222_2222);
        e.mem.set_u32(base + 0x20, 0x3333_3333);
        e.set(this, ActiveEffect::iFlags, 1);
        e.set(this, ActiveEffect::pCaster, 0x4000);
        e.set(this, ActiveEffect::pSource, 0x5000);
        answer(&mut e, MAGIC_CASTER_GET_FORM_ID, 0xc0);
        answer(&mut e, GETTER_0084E3A0, 0x50);
        let position = e.mem.alloc(4);
        answer(&mut e, GETTER_00825C00, position);
        let hit = object(&mut e, &[0x9c, 0xd4]);
        answer(&mut e, 0x0300_009c, 6);
        let head = list(&mut e, &[hit]);
        e.set(this, ActiveEffect::pHitEffects, head);
        e.call(0x0080_50d0, &args![this, 0x77u32]);
        let writes = writes.borrow();
        let expected: Vec<Vec<u8>> = vec![
            0x1111_1111u32.to_le_bytes().to_vec(),
            vec![1],
            vec![2],
            vec![3],
            vec![4],
            vec![5],
            0x2222_2222u32.to_le_bytes().to_vec(),
            0x3333_3333u32.to_le_bytes().to_vec(),
            0xc0u32.to_le_bytes().to_vec(),
            0u32.to_le_bytes().to_vec(),
            0x50u32.to_le_bytes().to_vec(),
            vec![0],
            vec![6],
            vec![1],
        ];
        assert_eq!(*writes, expected);
        assert_eq!(calls(&e, 0x0300_00d4), vec![vec![hit, base, 0x77]]);
        assert_eq!(
            e.mem.u8(position),
            1,
            "the count goes where 00825c00 pointed"
        );
    }

    #[test]
    fn load_game_reads_fields_ids_and_hit_effects() {
        for version in [0x48u8, 0x41] {
            let mut e = rig(version);
            let this = effect(&mut e);
            let base = this.addr();
            let mut data = vec![];
            data.extend(0x1111_1111u32.to_le_bytes());
            data.extend([1, 2, 3, 4, 5]);
            data.extend(0x2222_2222u32.to_le_bytes());
            data.extend(0x3333_3333u32.to_le_bytes());
            data.extend(0xc0u32.to_le_bytes());
            data.extend(0x70u32.to_le_bytes());
            data.extend(0x50u32.to_le_bytes());
            data.extend([2, 5, 6]);
            if version == 0x48 {
                data.extend(0x99u32.to_le_bytes());
            } else {
                data.push(1);
            }
            feed(&mut e, &data);
            e.register(ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
            let five = object(&mut e, &[0xd8]);
            let six = object(&mut e, &[0xd8]);
            answer(&mut e, HIT_EFFECT_KIND_5_CONSTRUCTOR, five);
            answer(&mut e, HIT_EFFECT_KIND_6_CONSTRUCTOR, six);
            e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
            e.register(LIST_NODE_INIT_00470440, |e, a| {
                let item = e.mem.u32(a[1]);
                e.mem.set_u32(a[0], item);
                a[0].into_ret()
            });
            let front = Rc::new(RefCell::new(vec![]));
            let seen = Rc::clone(&front);
            e.register_double(LIST_INSERT_FRONT_005AE3D0, move |e, a| {
                seen.borrow_mut().push(e.mem.u32(a[1]));
                Ret::default()
            });
            e.register(EFFECT_FLAG_00803E80, |_, _| Ret::default());
            e.call(0x0080_5330, &args![this, 0x77u32]);
            assert_eq!(e.mem.u32(base + 4), 0x1111_1111);
            assert_eq!(e.mem.bytes(base + 0x10, 5), vec![1, 2, 3, 4, 5]);
            assert_eq!(e.get(this, ActiveEffect::pCaster), 0xc0);
            assert_eq!(e.get(this, ActiveEffect::pTarget), 0x70);
            assert_eq!(e.get(this, ActiveEffect::pSource), 0x50);
            let node = e.get(this, ActiveEffect::pHitEffects);
            assert_eq!(e.mem.u32(node), five);
            assert_eq!(*front.borrow(), vec![six]);
            assert_eq!(calls(&e, 0x0300_00d8).len(), 2);
            assert_eq!(calls(&e, 0x0300_00d8)[0], vec![five, base, 0x77]);
            if version == 0x48 {
                assert_eq!(e.get(this, ActiveEffect::iFlags), 0x99);
                assert!(calls(&e, EFFECT_FLAG_00803E80).is_empty());
            } else {
                assert_eq!(
                    calls(&e, EFFECT_FLAG_00803E80),
                    vec![vec![base, 2], vec![base, 4]]
                );
            }
        }
    }

    #[test]
    fn load_game_reports_an_unknown_hit_effect_kind_then_calls_through_null() {
        let mut e = rig(0x2a);
        let this = effect(&mut e);
        let mut data = vec![0u8; 17 + 12];
        data.extend([1, 9]);
        feed(&mut e, &data);
        e.register(MESSAGE_005B5E40, |_, _| Ret::default());
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.call(0x0080_5330, &args![this, 0u32]);
        }));
        assert!(result.is_err(), "slot +0xd8 of a null hit effect");
        assert_eq!(calls(&e, MESSAGE_005B5E40), vec![vec![0x0107_93b8, 9]]);
    }

    #[test]
    fn fn_00805650_sets_flags_2_and_4() {
        let mut e = rig(0);
        e.register(EFFECT_FLAG_00803E80, |_, _| Ret::default());
        e.call(0x0080_5650, &args![0x1234u32]);
        assert_eq!(
            calls(&e, EFFECT_FLAG_00803E80),
            vec![vec![0x1234, 2], vec![0x1234, 4]]
        );
    }

    #[test]
    fn init_load_game_resolves_ids_and_casts_the_source_only_with_a_target() {
        for target in [0u32, 9] {
            let mut e = rig(0x2a);
            let this = effect(&mut e);
            e.set(this, ActiveEffect::pCaster, 7);
            e.set(this, ActiveEffect::pTarget, target);
            e.set(this, ActiveEffect::pSource, 0x55);
            answer(&mut e, MAGIC_CASTER_BY_NUMERIC_ID, 0x700);
            answer(&mut e, MAGIC_TARGET_BY_NUMERIC_ID, 0x900);
            answer(&mut e, LOOKUP_FORM_004839C0, 0x5500);
            answer(&mut e, RT_DYNAMIC_CAST, 0x5501);
            let hit = object(&mut e, &[0xdc]);
            let head = list(&mut e, &[hit]);
            e.set(this, ActiveEffect::pHitEffects, head);
            e.call(0x0080_5670, &args![this, 0x77u32]);
            assert_eq!(e.get(this, ActiveEffect::pCaster), 0x700);
            if target == 0 {
                assert_eq!(e.get(this, ActiveEffect::pTarget), 0);
                assert_eq!(e.get(this, ActiveEffect::pSource), 0x55);
            } else {
                assert_eq!(e.get(this, ActiveEffect::pTarget), 0x900);
                assert_eq!(e.get(this, ActiveEffect::pSource), 0x5501);
                assert_eq!(
                    calls(&e, RT_DYNAMIC_CAST),
                    vec![vec![0x5500, 0, TES_FORM_TYPE, TES_BOUND_OBJECT_TYPE, 0]]
                );
            }
            assert_eq!(calls(&e, 0x0300_00dc), vec![vec![hit, this.addr(), 0x77]]);
        }
    }

    #[test]
    fn finish_init_load_game_adds_temp_effects_and_restarts_a_looping_sound() {
        let mut e = rig(0x2a);
        let this = effect(&mut e);
        let hit = object(&mut e, &[0xe0]);
        let head = list(&mut e, &[hit]);
        e.set(this, ActiveEffect::pHitEffects, head);
        e.register(PROCESS_LISTS_ADD_TEMP_EFFECT, |_, _| Ret::default());
        answer(&mut e, EFFECT_POINTER_007DF1D0, 0x4444);
        answer(&mut e, FLAG_TEST_00403E60, 0);
        e.call(0x0080_5760, &args![this, 0x77u32]);
        assert_eq!(
            calls(&e, 0x0300_00e0),
            vec![vec![hit, this.addr(), 0x77, 0]]
        );
        assert_eq!(
            calls(&e, PROCESS_LISTS_ADD_TEMP_EFFECT),
            vec![vec![PROCESS_LISTS, hit]]
        );
        assert_eq!(calls(&e, FLAG_TEST_00403E60), vec![vec![0x4444, 0x400]]);
        // With flag 0x400 the sound restarts (its first call is 007df1d0 again).
        answer(&mut e, FLAG_TEST_00403E60, 1);
        answer(&mut e, GETTER_004FD3E0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0080_5760, &args![this, 0x77u32]);
        assert_eq!(calls(&e, GETTER_004FD3E0).len(), 1);
    }

    #[test]
    fn fn_00805810_counts_the_effect_record() {
        for (version, expected) in [(0x2au8, 2 + 4 + 1 + 5), (0x29, 4 + 1 + 5)] {
            let mut e = rig(version);
            let this = effect(&mut e);
            answer(&mut e, 0x0300_0010, 0x0007_0005);
            assert_eq!(e.call(0x0080_5810, &args![this, 0x77u32]).u16(), expected);
            assert_eq!(calls(&e, 0x0300_0010), vec![vec![this.addr(), 0x77]]);
        }
    }

    #[test]
    fn save_active_effect_writes_size_id_index_then_its_own_save() {
        let mut e = rig(0x2a);
        let writes = record_writes(&mut e);
        let this = effect(&mut e);
        e.set(this, ActiveEffect::pSpell, 0x8000);
        e.set(this, ActiveEffect::pEffect, 0x8100);
        answer(&mut e, 0x0300_0010, 0x1234_0102);
        answer(&mut e, MAGIC_ITEM_GET_FORM_ID, 0xabcd);
        answer(&mut e, EFFECT_ITEM_LIST_GET_INDEX, 0x0103);
        e.call(0x0080_5880, &args![this, 0x77u32]);
        assert_eq!(
            *writes.borrow(),
            vec![vec![2, 1], 0xabcdu32.to_le_bytes().to_vec(), vec![3]]
        );
        assert_eq!(
            calls(&e, EFFECT_ITEM_LIST_GET_INDEX),
            vec![vec![0x800c, 0x8100]]
        );
        assert_eq!(calls(&e, 0x0300_0018), vec![vec![this.addr(), 0x77]]);
    }

    #[test]
    fn load_active_effect_makes_the_effect_or_reports_and_skips() {
        let mut e = rig(0x2a);
        let mut data = vec![0x10, 0];
        data.extend(0x77u32.to_le_bytes());
        data.push(2);
        feed(&mut e, &data);
        answer(&mut e, MAGIC_ITEM_BY_NUMERIC_ID, 0x8000);
        answer(&mut e, EFFECT_ITEM_BY_INDEX, 0x8100);
        let made = object(&mut e, &[0x20]);
        answer(&mut e, EFFECT_ARCHETYPES_ACTIVATE, made);
        assert_eq!(e.call(0x0080_5920, &args![0x99u32]).u32(), made);
        assert_eq!(calls(&e, MAGIC_ITEM_BY_NUMERIC_ID), vec![vec![0x77]]);
        assert_eq!(calls(&e, EFFECT_ITEM_BY_INDEX), vec![vec![0x800c, 2]]);
        assert_eq!(
            calls(&e, EFFECT_ARCHETYPES_ACTIVATE),
            vec![vec![0, 0x8000, 0x8100, 0]]
        );
        assert_eq!(calls(&e, 0x0300_0020), vec![vec![made, 0x99]]);
        // Before version 0x2a the size is 0x1c; without the effect item it is skipped.
        let mut e = rig(0x29);
        let mut data = vec![];
        data.extend(0x77u32.to_le_bytes());
        data.push(2);
        feed(&mut e, &data);
        answer(&mut e, MAGIC_ITEM_BY_NUMERIC_ID, 0x8000);
        answer(&mut e, EFFECT_ITEM_BY_INDEX, 0);
        e.register(SKIP_BYTES_00857BD0, |_, _| Ret::default());
        e.register(MESSAGE_005B5E40, |_, _| Ret::default());
        assert_eq!(e.call(0x0080_5920, &args![0x99u32]).u32(), 0);
        let game = save_load_game(&e);
        assert_eq!(calls(&e, SKIP_BYTES_00857BD0), vec![vec![game, 0x1c]]);
        assert_eq!(
            calls(&e, MESSAGE_005B5E40),
            vec![vec![0x0107_93e0, 0x77, 2]]
        );
    }

    #[test]
    fn fn_00805a00_counts_the_list_and_reports_the_size() {
        let mut e = rig(0x2a);
        answer(&mut e, USE_SAVE_GAME_BLOCKS, 1);
        let value = e.mem.alloc(4);
        e.mem.set_u8(value, 1);
        answer(&mut e, SETTING_VALUE_ADDRESS, value);
        answer(&mut e, GETTER_004FD3E0, 0);
        e.register(ERROR_0040FBE0, |_, _| Ret::default());
        let first = object(&mut e, &[0x10]);
        answer(&mut e, 0x0300_0010, 3);
        let head = list(&mut e, &[first]);
        let size = e.call(0x0080_5a00, &args![head, 0x77u32]).u16();
        // 4 + 2 + 2, then 2 + 4 + 1 + 3 for the effect.
        assert_eq!(size, 18);
        assert_eq!(
            calls(&e, ERROR_0040FBE0),
            vec![vec![0x0101_2c78, 18, 0x3c8, 0x0107_9438]]
        );
        // With a record: its id, the form's name slot and its word at +5.
        let record = e.mem.alloc(0x10);
        e.mem.set_u32(record, 0x42);
        e.mem.set_u32(record + 5, 0x0506_0708);
        answer(&mut e, GETTER_004FD3E0, record);
        let form = object(&mut e, &[0x130]);
        answer(&mut e, LOOKUP_FORM_004839C0, form);
        answer(&mut e, 0x0300_0130, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x0080_5a00, &args![0u32, 0x77u32]);
        assert_eq!(
            calls(&e, ERROR_0040FBE0),
            vec![vec![
                0x0101_2cb0,
                8,
                0x42,
                0x6000,
                0x0506_0708,
                0x3c8,
                0x0107_9438
            ]]
        );
    }

    #[test]
    fn fn_00805b40_writes_the_block_count_and_size() {
        let mut e = rig(0x2a);
        let writes = record_writes(&mut e);
        answer(&mut e, USE_SAVE_GAME_BLOCKS, 1);
        let value = e.mem.alloc(4);
        answer(&mut e, SETTING_VALUE_ADDRESS, value);
        let buffer = e.mem.alloc(0x40);
        // The position advances 2 per call: start, size slot, count slot, end.
        let position = Rc::new(RefCell::new(buffer));
        let at = Rc::clone(&position);
        e.register_double(GETTER_00825C00, move |_, _| {
            let now = *at.borrow();
            *at.borrow_mut() = now + 2;
            now.into_ret()
        });
        let first = object(&mut e, &[0x10, 0x18]);
        answer(&mut e, MAGIC_ITEM_GET_FORM_ID, 1);
        answer(&mut e, EFFECT_ITEM_LIST_GET_INDEX, 0);
        let head = list(&mut e, &[first, first]);
        e.call(0x0080_5b40, &args![head, 0x77u32]);
        let writes = writes.borrow();
        assert_eq!(writes[0], BLOCK_MARKER.to_le_bytes().to_vec());
        assert_eq!(writes[1], vec![0, 0]);
        assert_eq!(writes[2], vec![0, 0]);
        // Size slot at buffer + 2, count slot at buffer + 4, last position buffer + 6.
        assert_eq!(e.mem.u16(buffer + 4), 2);
        assert_eq!(e.mem.u16(buffer + 2), 4);
        assert_eq!(calls(&e, 0x0300_0018).len(), 2);
    }

    #[test]
    fn fn_00805d40_replaces_the_list_with_the_loaded_effects() {
        let mut e = rig(0x2a);
        answer(&mut e, USE_SAVE_GAME_BLOCKS, 0);
        let old = object(&mut e, &[0]);
        let head = list(&mut e, &[old]);
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        answer(&mut e, EFFECT_REAL_00495460, 0);
        e.register(HIT_LIST_00470470, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        answer(&mut e, ACTOR_OBJECT_008D8520, 0);
        // Count 1; the effect record: size, id, index.
        let mut data = vec![1, 0, 0x10, 0];
        data.extend(0x77u32.to_le_bytes());
        data.push(0);
        feed(&mut e, &data);
        answer(&mut e, MAGIC_ITEM_BY_NUMERIC_ID, 0x8000);
        answer(&mut e, EFFECT_ITEM_BY_INDEX, 0x8100);
        let made = object(&mut e, &[0x20]);
        answer(&mut e, EFFECT_ARCHETYPES_ACTIVATE, made);
        e.call(0x0080_5d40, &args![head, 0x99u32]);
        assert_eq!(calls(&e, 0x0300_0000), vec![vec![old, 1]]);
        assert_eq!(calls(&e, EFFECT_REAL_00495460), vec![vec![old]]);
        assert_eq!(e.mem.u32(head), made, "the loaded effect fills the head");
        assert_eq!(calls(&e, ACTOR_OBJECT_008D8520), vec![vec![0x99]]);
    }

    #[test]
    fn fn_00805d40_reports_a_wrong_marker_and_a_short_block() {
        let mut e = rig(0x2a);
        answer(&mut e, USE_SAVE_GAME_BLOCKS, 1);
        answer(&mut e, LOAD_RECORD_004FD3C0, 0);
        let positions = Rc::new(RefCell::new(vec![0x100u32, 0x150]));
        let next = Rc::clone(&positions);
        e.register_double(GETTER_00825C00, move |_, _| {
            next.borrow_mut().remove(0).into_ret()
        });
        e.register(MESSAGE_005B5E40, |_, _| Ret::default());
        e.register(HIT_LIST_00470470, |_, _| Ret::default());
        // Marker 0, size 0x60, count 0.
        let mut data = vec![0u8; 4];
        data.extend([0x60, 0, 0, 0]);
        feed(&mut e, &data);
        e.call(0x0080_5d40, &args![0u32, 0u32]);
        assert_eq!(
            calls(&e, MESSAGE_005B5E40),
            vec![
                vec![0x0101_56a8, 0x0107_9438, 0x3e8, 0x2a],
                vec![0x0101_5440, 0x10, 0x0107_9438, 0x40f, 0x2a]
            ]
        );
    }

    #[test]
    fn list_walkers_call_their_slots_until_an_empty_node() {
        for (addr, slot) in [(0x0080_60e0u32, 0x28u32), (0x0080_61b0, 0x34)] {
            let mut e = rig(0);
            let first = object(&mut e, &[slot]);
            let head = list(&mut e, &[first, 0]);
            // The second node (null item, no next) is empty: the walk stops there.
            e.call(addr, &args![head, 0x77u32]);
            assert_eq!(calls(&e, 0x0300_0000 + slot), vec![vec![first, 0x77]]);
        }
    }

    #[test]
    fn fn_00806130_tells_the_actor_object_after_the_effects() {
        let mut e = rig(0);
        let first = object(&mut e, &[0x2c]);
        let head = list(&mut e, &[first]);
        let process = object(&mut e, &[0x580]);
        answer(&mut e, ACTOR_OBJECT_008D8520, process);
        e.call(0x0080_6130, &args![head, 0x99u32]);
        assert_eq!(calls(&e, 0x0300_002c), vec![vec![first, 0x99]]);
        assert_eq!(calls(&e, 0x0300_0580), vec![vec![process, 1, 1, 1]]);
        // Without an actor nothing more runs.
        e.call_log = Some(vec![]);
        e.call(0x0080_6130, &args![0u32, 0u32]);
        assert!(calls(&e, ACTOR_OBJECT_008D8520).is_empty());
    }

    #[test]
    fn load_game_ov2_reads_the_buffer_and_the_hit_effects() {
        let mut e = rig(0);
        let this = effect(&mut e);
        let base = this.addr();
        let mut data = vec![];
        data.extend(0x1111_1111u32.to_le_bytes());
        data.extend([1, 2, 3, 4, 5]);
        data.extend(0x2222_2222u32.to_le_bytes());
        data.extend(0x3333_3333u32.to_le_bytes());
        data.extend(0x4444_4444u32.to_le_bytes());
        data.push(6);
        feed(&mut e, &data);
        e.register(LOAD_FORM_ID_OV2, |_, _| Ret::default());
        answer(&mut e, LOAD_FORM_ID, 0x50);
        answer(&mut e, LOOKUP_FORM_004839C0, 0x5000);
        answer(&mut e, RT_DYNAMIC_CAST, 0x5001);
        answer(&mut e, LOAD_VARIABLE_SIZED_VALUE, 1);
        e.register(ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        let six = object(&mut e, &[0xb0]);
        answer(&mut e, HIT_EFFECT_KIND_6_CONSTRUCTOR, six);
        e.register(HIT_EFFECT_SET_EFFECT_00984F60, |_, _| Ret::default());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(LIST_NODE_INIT_00470440, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], item);
            a[0].into_ret()
        });
        e.call(0x0080_6420, &args![this, 0x88u32]);
        assert_eq!(e.mem.u32(base + 0x18), 0x4444_4444);
        assert_eq!(e.mem.bytes(base + 0x10, 5), vec![1, 2, 3, 4, 5]);
        assert_eq!(
            calls(&e, LOAD_FORM_ID_OV2),
            vec![
                vec![0x88, base + 0x28],
                vec![0x88, base + 0x24],
                vec![0x88, base + 0x44]
            ]
        );
        assert_eq!(e.get(this, ActiveEffect::pSource), 0x5001);
        assert_eq!(
            calls(&e, HIT_EFFECT_SET_EFFECT_00984F60),
            vec![vec![six, base]]
        );
        assert_eq!(calls(&e, 0x0300_00b0), vec![vec![six, 0x88]]);
        let node = e.get(this, ActiveEffect::pHitEffects);
        assert_eq!(e.mem.u32(node), six);
    }

    #[test]
    fn init_load_game_ov2_resolves_the_three_ids() {
        let mut e = rig(0);
        let this = effect(&mut e);
        e.set(this, ActiveEffect::pCaster, 7);
        e.set(this, ActiveEffect::pTarget, 0);
        e.set(this, ActiveEffect::pDisplacementSpell, 9);
        answer(&mut e, MAGIC_CASTER_BY_NUMERIC_ID, 0x700);
        answer(&mut e, MAGIC_ITEM_BY_NUMERIC_ID, 0x900);
        let hit = object(&mut e, &[0xb8]);
        let head = list(&mut e, &[0, hit]);
        e.set(this, ActiveEffect::pHitEffects, head);
        e.call(0x0080_66d0, &args![this, 0x77u32]);
        assert_eq!(e.get(this, ActiveEffect::pCaster), 0x700);
        assert_eq!(e.get(this, ActiveEffect::pTarget), 0);
        assert_eq!(e.get(this, ActiveEffect::pDisplacementSpell), 0x900);
        assert!(calls(&e, MAGIC_TARGET_BY_NUMERIC_ID).is_empty());
        assert_eq!(calls(&e, 0x0300_00b8), vec![vec![hit, 0x77]]);
    }

    #[test]
    fn finish_load_game_hands_each_hit_effect_to_the_process_lists() {
        let mut e = rig(0);
        let this = effect(&mut e);
        answer(&mut e, EFFECT_POINTER_007DF1D0, 0x4444);
        answer(&mut e, FLAG_TEST_00403E60, 0);
        e.register(PROCESS_LISTS_ADD_TEMP_EFFECT, |_, _| Ret::default());
        let hit = object(&mut e, &[0xbc]);
        let head = list(&mut e, &[hit, 0]);
        e.set(this, ActiveEffect::pHitEffects, head);
        e.call(0x0080_67b0, &args![this, 0x77u32]);
        assert_eq!(calls(&e, 0x0300_00bc), vec![vec![hit, 0x77]]);
        assert_eq!(
            calls(&e, PROCESS_LISTS_ADD_TEMP_EFFECT),
            vec![vec![PROCESS_LISTS, hit]]
        );
    }

    #[test]
    fn load_active_effect_ov2_checks_the_setting_word() {
        for matches in [true, false] {
            let mut e = rig(0);
            e.register(LOAD_FORM_ID_OV2, |e, a| {
                e.mem.set_u32(a[1], 0x77);
                Ret::default()
            });
            feed(&mut e, &[3]);
            let values = Rc::new(RefCell::new(vec![0x10u32, 0x20, 0x30]));
            let next = Rc::clone(&values);
            e.register_double(LOAD_VARIABLE_SIZED_VALUE, move |_, _| {
                next.borrow_mut().remove(0).into_ret()
            });
            answer(&mut e, MAGIC_ITEM_BY_NUMERIC_ID, 0x8000);
            answer(&mut e, EFFECT_ITEM_BY_INDEX, 0x8100);
            answer(&mut e, GETTER_00825C00, 0x8200);
            answer(&mut e, GETTER_00810FD0, if matches { 0x10 } else { 0x11 });
            let made = object(&mut e, &[0x1c]);
            answer(&mut e, EFFECT_ARCHETYPES_ACTIVATE, made);
            e.register(SKIP_0049BB50, |_, _| Ret::default());
            e.register(SAVE_GAME_WARNING, |_, _| Ret::default());
            let result = e.call(0x0080_6910, &args![0x88u32]).u32();
            assert_eq!(calls(&e, EFFECT_ITEM_BY_INDEX), vec![vec![0x800c, 3]]);
            if matches {
                assert_eq!(result, made);
                assert_eq!(calls(&e, 0x0300_001c), vec![vec![made, 0x88]]);
            } else {
                assert_eq!(result, 0);
                assert_eq!(calls(&e, SKIP_0049BB50), vec![vec![0x88, 0x20]]);
                assert_eq!(
                    calls(&e, SAVE_GAME_WARNING),
                    vec![vec![0x0107_94a8, 0x77, 3, 0x10]]
                );
            }
        }
    }

    #[test]
    fn load_active_effect_list_reverts_then_inserts_the_loaded_effects() {
        let mut e = rig(0);
        let old = object(&mut e, &[0, 0x30]);
        let head = list(&mut e, &[old]);
        e.register(HIT_LIST_00470470, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        let values = Rc::new(RefCell::new(vec![1u32, 0x10, 0x20]));
        let next = Rc::clone(&values);
        e.register_double(LOAD_VARIABLE_SIZED_VALUE, move |_, _| {
            next.borrow_mut().remove(0).into_ret()
        });
        e.register(LOAD_FORM_ID_OV2, |e, a| {
            e.mem.set_u32(a[1], 0x77);
            Ret::default()
        });
        feed(&mut e, &[0]);
        answer(&mut e, MAGIC_ITEM_BY_NUMERIC_ID, 0x8000);
        answer(&mut e, EFFECT_ITEM_BY_INDEX, 0x8100);
        answer(&mut e, GETTER_00825C00, 0x8200);
        answer(&mut e, GETTER_00810FD0, 0x10);
        let made = object(&mut e, &[0x1c]);
        answer(&mut e, EFFECT_ARCHETYPES_ACTIVATE, made);
        e.call(0x0080_6a90, &args![0x88u32, head]);
        assert_eq!(calls(&e, 0x0300_0030), vec![vec![old, 0x88]]);
        assert_eq!(calls(&e, 0x0300_0000), vec![vec![old, 1]]);
        assert_eq!(e.mem.u32(head), made);
    }

    #[test]
    fn item_walkers_skip_null_items_and_call_their_slots() {
        for (addr, slot) in [(0x0080_6b00u32, 0x24u32), (0x0080_6b50, 0x40)] {
            let mut e = rig(0);
            let first = object(&mut e, &[slot]);
            let head = list(&mut e, &[0, first]);
            e.call(addr, &args![0x77u32, head]);
            assert_eq!(calls(&e, 0x0300_0000 + slot), vec![vec![first, 0x77]]);
        }
    }

    #[test]
    fn revert_list_reverts_and_destroys_each_effect_then_clears_the_list() {
        let mut e = rig(0);
        let first = object(&mut e, &[0, 0x30]);
        let head = list(&mut e, &[first, 0]);
        e.register(HIT_LIST_00470470, |_, _| Ret::default());
        e.call(0x0080_6ba0, &args![0x77u32, head]);
        assert_eq!(calls(&e, 0x0300_0030), vec![vec![first, 0x77]]);
        assert_eq!(calls(&e, 0x0300_0000), vec![vec![first, 1]]);
        assert_eq!(calls(&e, HIT_LIST_00470470), vec![vec![head]]);
    }
}
