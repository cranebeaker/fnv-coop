//! `fallout/magic/bounditemeffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::activeeffect::{
    load_block_check, load_block_start, load_bytes, report_save_size, save_block_finish,
    save_block_start, save_bytes, save_position, save_version, size_reports_enabled,
    use_save_game_blocks, COUNTED_SIZE_FORMATS, WRITTEN_SIZE_FORMATS,
};

layout! {
    /// `BoundItemEffect` (Xbox PDB), 0xac bytes (the PC `Copy` allocates 0xac): an
    /// `AssociatedItemEffect` (0x4c bytes) with the displaced items.
    pub struct BoundItemEffect: 0xac {
        /// `pDisplacedWeapon` (Xbox PDB): `ItemChange*`.
        0x4c pDisplacedWeapon: u32,
        /// `fAppearTime` (Xbox PDB).
        0xa0 fAppearTime: f32,
        /// `bItemSummoned` (Xbox PDB).
        0xa4 bItemSummoned: u8,
        /// `bSpellModelCached` (Xbox PDB).
        0xa5 bSpellModelCached: u8,
        /// `bWeaponCached` (Xbox PDB).
        0xa6 bWeaponCached: u8,
        /// `bWornCached` (Xbox PDB).
        0xa7 bWornCached: u8,
        /// `bSetupItem` (Xbox PDB).
        0xa8 bSetupItem: u8,
    }
}

/// `pDisplacedItems` (Xbox PDB): 20 `ItemChange*` at +0x50.
const DISPLACED_ITEMS: u32 = 0x50;
/// The number of displaced item slots.
const DISPLACED_ITEM_COUNT: u32 = 20;
/// The source-file string the reports pass (`...\Magic\BoundItemEffect.cpp`, `01079810`).
const BOUND_ITEM_EFFECT_FILE: u32 = 0x0107_9810;
/// `BoundItemEffect`'s vtable (stored by the constructor and the destructor).
const BOUND_ITEM_EFFECT_VTABLE: u32 = 0x0107_979c;

/// `operator new` (`00401000`, cdecl: size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete` (`00401030`, cdecl: the block).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The allocator `00aa13e0` (cdecl: size).
const ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
/// `00806c20` (thiscall: caster, spell, effect item; unnamed): the `AssociatedItemEffect`
/// constructor.
const ASSOCIATED_ITEM_EFFECT_CONSTRUCTOR: u32 = 0x0080_6c20;
/// `0080d930` (thiscall; unnamed): the base part's destructor (the detect life effect's
/// destructor hands to it too).
const BASE_DESTRUCTOR_0080D930: u32 = 0x0080_d930;
/// `AssociatedItemEffect::CopyData` (`00806d50`, thiscall: the target effect).
const ASSOCIATED_ITEM_EFFECT_COPY_DATA: u32 = 0x0080_6d50;
/// `00806da0` (thiscall: one word; the map's name `ValueAndConditionsEffect::GetSaveSize`
/// may come from folded code; the base save size here).
const BASE_SAVE_SIZE_00806DA0: u32 = 0x0080_6da0;
/// `AssociatedItemEffect::SaveGame` (`00806de0`, thiscall: one word).
const ASSOCIATED_ITEM_EFFECT_SAVE_GAME: u32 = 0x0080_6de0;
/// `AssociatedItemEffect::LoadGame` (`00806e30`, thiscall: one word).
const ASSOCIATED_ITEM_EFFECT_LOAD_GAME: u32 = 0x0080_6e30;
/// `AssociatedItemEffect::InitLoadGame` (`00806e70`, thiscall: one word).
const ASSOCIATED_ITEM_EFFECT_INIT_LOAD_GAME: u32 = 0x0080_6e70;
/// `ActiveEffect::Dispel` (`00804210`, thiscall: one byte).
const ACTIVE_EFFECT_DISPEL: u32 = 0x0080_4210;
/// `__RTDynamicCast` (`00ec43fb`, cdecl: object, vfDelta, source type, target type, isReference).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `TESForm`'s RTTI type descriptor (`.?AVTESForm@@`).
const TES_FORM_TYPE: u32 = 0x0118_3028;
/// `TESBoundObject`'s RTTI type descriptor (`.?AVTESBoundObject@@`).
const TES_BOUND_OBJECT_TYPE: u32 = 0x0118_3108;
/// `TESObjectWEAP`'s RTTI type descriptor (`.?AVTESObjectWEAP@@`).
const TES_OBJECT_WEAP_TYPE: u32 = 0x0118_3998;
/// `TESObjectARMO`'s RTTI type descriptor (`.?AVTESObjectARMO@@`).
const TES_OBJECT_ARMO_TYPE: u32 = 0x0118_3a34;
/// `TESNPC`'s RTTI type descriptor (`.?AVTESNPC@@`).
const TES_NPC_TYPE: u32 = 0x0118_3a1c;
/// `TESBipedModelForm`'s RTTI type descriptor (`.?AVTESBipedModelForm@@`).
const TES_BIPED_MODEL_FORM_TYPE: u32 = 0x0118_3978;
/// `MagicTarget`'s RTTI type descriptor (`.?AVMagicTarget@@`).
const MAGIC_TARGET_TYPE: u32 = 0x011a_0ef8;
/// `Character`'s RTTI type descriptor (`.?AVCharacter@@`).
const CHARACTER_TYPE: u32 = 0x011a_0f14;
/// `ActiveEffect`'s RTTI type descriptor (`.?AVActiveEffect@@`).
const ACTIVE_EFFECT_TYPE: u32 = 0x011a_0d1c;
/// `BoundItemEffect`'s RTTI type descriptor (`.?AVBoundItemEffect@@`).
const BOUND_ITEM_EFFECT_TYPE: u32 = 0x011a_0ed8;
/// The `ModelLoader` pointer (`011c3b3c`).
const MODEL_LOADER_POINTER: u32 = 0x011c_3b3c;
/// `ModelLoader::QueueModel` (`00444040`, thiscall: seven words).
const MODEL_LOADER_QUEUE_MODEL: u32 = 0x0044_4040;
/// `ModelLoader::ReleaseModel` (`00445300`, thiscall: three words).
const MODEL_LOADER_RELEASE_MODEL: u32 = 0x0044_5300;
/// `ModelLoader::LoadFile` (`00447080`, thiscall: six words).
const MODEL_LOADER_LOAD_FILE: u32 = 0x0044_7080;
/// `0044ddc0` (thiscall): the word at +8 (an item change's form; an effect's spell).
const WORD_AT_8_0044DDC0: u32 = 0x0044_ddc0;
/// `0084e3a0` (thiscall): the word at +0xc (an effect's effect item; an item's setting).
const WORD_AT_C_0084E3A0: u32 = 0x0084_e3a0;
/// `0048cee0` (thiscall on a model, returns a word; unnamed): non-zero when it names a model.
const MODEL_LENGTH_0048CEE0: u32 = 0x0048_cee0;
/// `MagicItem::GetAVEffect` (`0040a300`, thiscall: one word).
const MAGIC_ITEM_GET_AV_EFFECT: u32 = 0x0040_a300;
/// `EffectItemList::GetCostliestEffect` (`004064d0`, thiscall: two words).
const EFFECT_ITEM_LIST_GET_COSTLIEST_EFFECT: u32 = 0x0040_64d0;
/// `0059bb30` (thiscall): the effect's target (+0x24).
const EFFECT_TARGET_0059BB30: u32 = 0x0059_bb30;
/// `MagicTarget::GetTargetAsActor` (`00822b40`).
const MAGIC_TARGET_GET_TARGET_AS_ACTOR: u32 = 0x0082_2b40;
/// `006733e0` (thiscall on the effect; the map's name `LowProcess::GetSecondGenericLocation`
/// comes from folded code): the associated item.
const ASSOCIATED_ITEM_006733E0: u32 = 0x0067_33e0;
/// `007af430` (thiscall; the map's name `BGSSaveFormBuffer::GetForm` may come from folded
/// code): a reference's base form.
const BASE_FORM_007AF430: u32 = 0x007a_f430;
/// `TESActorBase::GetSex` (`005f0cc0`).
const TES_ACTOR_BASE_GET_SEX: u32 = 0x005f_0cc0;
/// `TESBipedModelForm::GetBipedModel` (`004811e0`, thiscall: one word).
const TES_BIPED_MODEL_FORM_GET_BIPED_MODEL: u32 = 0x0048_11e0;
/// `TESBipedModelForm::FillsBipedSlot` (`00480af0`, thiscall: three words).
const TES_BIPED_MODEL_FORM_FILLS_BIPED_SLOT: u32 = 0x0048_0af0;
/// `00559450` (thiscall on an item change; unnamed): its extra-data list node.
const EXTRA_LISTS_00559450: u32 = 0x0055_9450;
/// `006815c0` (thiscall on a list node, returns the address of its item).
const LIST_NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070` (thiscall on a list node, returns the next node).
const LIST_NODE_NEXT: u32 = 0x0072_6070;
/// `008256d0` (thiscall on a list node, `AL`): true when the node's item and next are null.
const LIST_NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// `ExtraDataList::ClearScriptLocals` (`0041afa0`).
const EXTRA_DATA_LIST_CLEAR_SCRIPT_LOCALS: u32 = 0x0041_afa0;
/// `ItemChange::DeleteAllExtra` (`004bc780`).
const ITEM_CHANGE_DELETE_ALL_EXTRA: u32 = 0x004b_c780;
/// `004459e0` (thiscall: one word; with 1 here, an item change's deleting destructor).
const DELETE_004459E0: u32 = 0x0044_59e0;
/// `ItemChange::ItemChange` (`004bc550`, thiscall: form, count).
const ITEM_CHANGE_CONSTRUCTOR: u32 = 0x004b_c550;
/// `ItemChange::ItemChange_ov2` (`004bc650`, thiscall: the item change to copy).
const ITEM_CHANGE_COPY_CONSTRUCTOR: u32 = 0x004b_c650;
/// `ItemChange::ItemChange_ov3` (`0076b630`, thiscall: no words).
const ITEM_CHANGE_EMPTY_CONSTRUCTOR: u32 = 0x0076_b630;
/// `ItemChange::LoadGame` (`004be930`).
const ITEM_CHANGE_LOAD_GAME: u32 = 0x004b_e930;
/// `004be5a0` (thiscall on an item change, returns `AX`; unnamed): its save size.
const ITEM_CHANGE_SAVE_SIZE_004BE5A0: u32 = 0x004b_e5a0;
/// `004be6f0` (thiscall on an item change; unnamed): its save.
const ITEM_CHANGE_SAVE_004BE6F0: u32 = 0x004b_e6f0;
/// `004bed00` (thiscall on an item change; unnamed): its init after a load.
const ITEM_CHANGE_INIT_004BED00: u32 = 0x004b_ed00;
/// `ExtraDataList::ExtraDataList` (`00410360`).
const EXTRA_DATA_LIST_CONSTRUCTOR: u32 = 0x0041_0360;
/// `ExtraDataList::DuplicateExtraListForContainer` (`00412380`, thiscall: the source).
const EXTRA_DATA_LIST_DUPLICATE: u32 = 0x0041_2380;
/// `ExtraDataList::SetCanNotWear` (`0041ab70`, thiscall: one byte).
const EXTRA_DATA_LIST_SET_CAN_NOT_WEAR: u32 = 0x0041_ab70;
/// `ExtraDataList::RemoveCannotWearExtra` (`0041b040`).
const EXTRA_DATA_LIST_REMOVE_CANNOT_WEAR: u32 = 0x0041_b040;
/// `0041b010` (thiscall on an extra-data list: one word; unnamed).
const EXTRA_LIST_0041B010: u32 = 0x0041_b010;
/// `BaseExtraList::ItemsInList` (`0040fe20`).
const BASE_EXTRA_LIST_ITEMS_IN_LIST: u32 = 0x0040_fe20;
/// `ExtraDataList::GetCount` (`00418770`, returns `AX`).
const EXTRA_DATA_LIST_GET_COUNT: u32 = 0x0041_8770;
/// `ExtraDataList::GetContainerChanges` (`00418520`).
const EXTRA_DATA_LIST_GET_CONTAINER_CHANGES: u32 = 0x0041_8520;
/// `005d43c0` (thiscall on an actor; unnamed): its extra-data list.
const EXTRA_LIST_005D43C0: u32 = 0x005d_43c0;
/// `004bca60` (thiscall on an extra-data list, `AL`; unnamed).
const EXTRA_LIST_TEST_004BCA60: u32 = 0x004b_ca60;
/// `00905330` (thiscall on an item change's list: the address of an extra-data list; unnamed).
const LIST_REMOVE_00905330: u32 = 0x0090_5330;
/// `InventoryChanges::GetInventoryChanges` (`004bf220`, cdecl: the reference).
const INVENTORY_CHANGES_GET: u32 = 0x004b_f220;
/// `InventoryChanges::WearingObject` (`004bfda0`, thiscall: form, flag).
const INVENTORY_CHANGES_WEARING_OBJECT: u32 = 0x004b_fda0;
/// `InventoryChanges::GetWornItem` (`004c8c10`, thiscall: slot, flag).
const INVENTORY_CHANGES_GET_WORN_ITEM: u32 = 0x004c_8c10;
/// `InventoryChanges::GetObjectCount` (`004c8f30`, thiscall: the form).
const INVENTORY_CHANGES_GET_OBJECT_COUNT: u32 = 0x004c_8f30;
/// `Actor::GetItemCountinContainer` (`00575610`, thiscall: the form).
const ACTOR_GET_ITEM_COUNT: u32 = 0x0057_5610;
/// `Actor::QueueEquipObject` (`0088c650`, thiscall: six words).
const ACTOR_QUEUE_EQUIP_OBJECT: u32 = 0x0088_c650;
/// `Actor::QueueUnEquipObject` (`0088c790`, thiscall: six words).
const ACTOR_QUEUE_UNEQUIP_OBJECT: u32 = 0x0088_c790;
/// `008a6970` (thiscall on an actor, `AL`; unnamed).
const ACTOR_TEST_008A6970: u32 = 0x008a_6970;
/// `008d8520` (thiscall on an actor; the map's name `MiddleHighProcess::GetSavedAcquireObject`
/// may come from folded code): the object whose slots +0x148, +0x450, +0x454, +0x530 run here.
const ACTOR_OBJECT_008D8520: u32 = 0x008d_8520;
/// `0045cd60` (thiscall on that object; unnamed).
const PROCESS_TEST_0045CD60: u32 = 0x0045_cd60;
/// The player pointer (`011dea3c`).
const PLAYER_POINTER: u32 = 0x011d_ea3c;
/// `004b6220` (cdecl: three words, returns `ST0`; unnamed, `globalfunc.cpp`).
const MODEL_TIME_004B6220: u32 = 0x004b_6220;
/// `"Hit"` (`0103275c`).
const HIT_STRING: u32 = 0x0103_275c;
/// `"SpecialIdle_HitEffect"` (`0103bdb8`).
const SPECIAL_IDLE_HIT_EFFECT_STRING: u32 = 0x0103_bdb8;
/// `MagicModelHitEffect::MagicModelHitEffect_ov2` (`0081c720`, thiscall: three words).
const MAGIC_MODEL_HIT_EFFECT_CONSTRUCTOR: u32 = 0x0081_c720;
/// `-1.0` (`float`, `01012054`).
const MINUS_ONE: u32 = 0x0101_2054;
/// `ProcessLists::AddTempEffect` (`00973fd0`, thiscall: the effect).
const PROCESS_LISTS_ADD_TEMP_EFFECT: u32 = 0x0097_3fd0;
/// The `ProcessLists` object `00973fd0` is called on.
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// `MobileObject::PlaySoundByEditorName` (`00933270`, thiscall: five words).
const PLAY_SOUND_BY_EDITOR_NAME: u32 = 0x0093_3270;
/// `"ITMBoundDisappear"` (`010797f8`).
const ITM_BOUND_DISAPPEAR_STRING: u32 = 0x0107_97f8;
/// `00483710` (thiscall: the sound handle's destructor here; unnamed).
const SOUND_HANDLE_DESTROY_00483710: u32 = 0x0048_3710;
/// `007037c0` (thiscall: one word; the map's name `BGSMenuPacker::RecomputePacking` may come
/// from folded code).
const FINISH_HELPER_007037C0: u32 = 0x0070_37c0;
/// `005ae3d0` (thiscall on a list: the address of the item): the list's front insert.
const LIST_INSERT_FRONT_005AE3D0: u32 = 0x005a_e3d0;
/// `007df1d0` (thiscall on the effect, returns a pointer; unnamed, filed under
/// `statsmenu.cpp`): the effect setting.
const EFFECT_SETTING_007DF1D0: u32 = 0x007d_f1d0;

/// The address of displaced item slot `index`.
fn displaced_item(this: Ptr<BoundItemEffect>, index: u32) -> u32 {
    this.addr() + DISPLACED_ITEMS + 4 * index
}

/// `__RTDynamicCast(object, 0, from, to, 0)`.
fn cast(e: &mut Engine, object: u32, from: u32, to: u32) -> u32 {
    e.call(RT_DYNAMIC_CAST, &args![object, 0u32, from, to, 0u32])
        .u32()
}

/// The effect's target as an actor (`00822b40` of `0059bb30`), null without a target.
fn target_actor(e: &mut Engine, this: Ptr<BoundItemEffect>) -> u32 {
    if e.call(EFFECT_TARGET_0059BB30, &args![this]).u32() == 0 {
        0
    } else {
        let target = e.call(EFFECT_TARGET_0059BB30, &args![this]).u32();
        e.call(MAGIC_TARGET_GET_TARGET_AS_ACTOR, &args![target])
            .u32()
    }
}

/// The model loader (`*011c3b3c`).
fn model_loader(e: &Engine) -> u32 {
    e.global::<u32>(MODEL_LOADER_POINTER)
}

/// The model of a `TESModel` (slot +0x14).
fn model_of(e: &mut Engine, model_form: u32) -> u32 {
    e.vcall(model_form, 0x14, &[]).u32()
}

/// `ModelLoader::ReleaseModel(model, 0, 1)`.
fn release_model(e: &mut Engine, model: u32) {
    let loader = model_loader(e);
    e.call(
        MODEL_LOADER_RELEASE_MODEL,
        &args![loader, model, 0u32, 1u32],
    );
}

/// `ModelLoader::QueueModel(model, 0, 0, 0, 1, 1, 0)`.
fn queue_model(e: &mut Engine, model: u32) {
    let loader = model_loader(e);
    e.call(
        MODEL_LOADER_QUEUE_MODEL,
        &args![loader, model, 0u32, 0u32, 0u32, 1u32, 1u32, 0u32],
    );
}

/// `ModelLoader::LoadFile(model, 0, 1, 0, 0, 0)`.
fn load_model_file(e: &mut Engine, model: u32) -> u32 {
    let loader = model_loader(e);
    e.call(
        MODEL_LOADER_LOAD_FILE,
        &args![loader, model, 0u32, 1u32, 0u32, 0u32, 0u32],
    )
    .u32()
}

/// The actor's sex as these functions find it: `TESActorBase::GetSex` of its base form as
/// a `TESNPC`, 0 when it is none.
fn actor_sex(e: &mut Engine, actor: u32) -> u32 {
    let base = e.call(BASE_FORM_007AF430, &args![actor]).u32();
    let npc = cast(e, base, TES_BOUND_OBJECT_TYPE, TES_NPC_TYPE);
    if npc == 0 {
        0
    } else {
        e.call(TES_ACTOR_BASE_GET_SEX, &args![npc]).u32()
    }
}

/// The first extra-data list of an item change (`*006815c0(00559450(change))`), null when
/// it has no list.
fn first_extra_list(e: &mut Engine, change: u32) -> u32 {
    if e.call(EXTRA_LISTS_00559450, &args![change]).u32() == 0 {
        0
    } else {
        let lists = e.call(EXTRA_LISTS_00559450, &args![change]).u32();
        let slot = e.call(LIST_NODE_ITEM, &args![lists]).u32();
        e.mem.u32(slot)
    }
}

/// Releases a displaced item change: its first extra-data list (found beforehand) loses its
/// script locals, the change its extra data (`DeleteAllExtra`), and it is deleted
/// (`004459e0` with 1; skipped for null).
fn release_item_change(e: &mut Engine, change: u32, extra: u32) {
    if extra != 0 {
        e.call(EXTRA_DATA_LIST_CLEAR_SCRIPT_LOCALS, &args![extra]);
    }
    e.call(ITEM_CHANGE_DELETE_ALL_EXTRA, &args![change]);
    if change != 0 {
        e.call(DELETE_004459E0, &args![change, 1u32]);
    }
}

/// A copy of an item change in a new 12-byte block (`ItemChange::ItemChange_ov2`), null
/// when the allocation fails.
fn copy_item_change(e: &mut Engine, change: u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
    if block == 0 {
        0
    } else {
        e.call(ITEM_CHANGE_COPY_CONSTRUCTOR, &args![block, change])
            .u32()
    }
}

/// The spell's effect setting (`GetAVEffect` with 0) and its costliest effect item
/// (`GetCostliestEffect` of the spell's list with the effect item's +0xc word and 0).
fn setting_and_costliest(e: &mut Engine, this: Ptr<BoundItemEffect>) -> (u32, u32) {
    let spell = e.call(WORD_AT_8_0044DDC0, &args![this]).u32();
    let setting = e.call(MAGIC_ITEM_GET_AV_EFFECT, &args![spell, 0u32]).u32();
    let item = e.call(WORD_AT_C_0084E3A0, &args![this]).u32();
    let item_setting = e.call(WORD_AT_C_0084E3A0, &args![item]).u32();
    let spell = e.call(WORD_AT_8_0044DDC0, &args![this]).u32();
    let costliest = e
        .call(
            EFFECT_ITEM_LIST_GET_COSTLIEST_EFFECT,
            &args![spell.wrapping_add(0xc), item_setting, 0u32],
        )
        .u32();
    (setting, costliest)
}

/// Whether the model at +0x3c of a weapon (or +0x18 of a setting) names one (`0048cee0`).
fn has_model(e: &mut Engine, model_form: u32) -> bool {
    e.call(MODEL_LENGTH_0048CEE0, &args![model_form]).u32() != 0
}

// Translated from 00807e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`bounditemeffect.cpp`; thiscall, three words): the `BoundItemEffect`
/// constructor: the `AssociatedItemEffect` part (`00806c20` with the three words), its
/// vtable, no displaced weapon or items, appear time 0 and the five flags clear; returns
/// `this`.
pub fn fn_00807e10(
    e: &mut Engine,
    this: Ptr<BoundItemEffect>,
    caster: u32,
    spell: u32,
    item: u32,
) -> u32 {
    e.call(
        ASSOCIATED_ITEM_EFFECT_CONSTRUCTOR,
        &args![this, caster, spell, item],
    );
    e.mem.set_u32(this.addr(), BOUND_ITEM_EFFECT_VTABLE);
    e.set(this, BoundItemEffect::pDisplacedWeapon, 0);
    for index in 0..DISPLACED_ITEM_COUNT {
        e.mem.set_u32(displaced_item(this, index), 0);
    }
    e.mem.set_u32(this.addr() + 0xa0, 0);
    e.set(this, BoundItemEffect::bItemSummoned, 0);
    e.set(this, BoundItemEffect::bSpellModelCached, 0);
    e.set(this, BoundItemEffect::bWeaponCached, 0);
    e.set(this, BoundItemEffect::bWornCached, 0);
    e.set(this, BoundItemEffect::bSetupItem, 0);
    this.addr()
}

// Translated from 00807eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BoundItemEffect::__scalar_deleting_destructor` (Xbox PDB): the destructor
/// (`fn_00807ee0`), then `operator delete` when bit 0 of `flags` is set; returns `this`.
pub fn bound_item_effect_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BoundItemEffect>,
    flags: u32,
) -> u32 {
    fn_00807ee0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this.addr()
}

// Translated from 00807ee0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00807ee0 - the exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed (`bounditemeffect.cpp`): the `BoundItemEffect` destructor. With the vtable
/// back, it releases the cached models: the displaced weapon's (as a `TESObjectWEAP`, its
/// model +0x3c) when `bWeaponCached`; with `bWornCached` the biped model of each displaced
/// item that is a `TESBipedModelForm` (for the target's sex when the target's slot +4
/// reference has a `TESNPC` base; sets `bWornCached` again); then the displaced weapon and
/// items are released (`release_item_change`); with `bSpellModelCached`, a spell and an
/// effect item, the setting's model (+0x18) when the effect item is the costliest
/// (`bSpellModelCached` cleared). Last the base part's destructor (`0080d930`).
pub fn fn_00807ee0(e: &mut Engine, this: Ptr<BoundItemEffect>) {
    e.mem.set_u32(this.addr(), BOUND_ITEM_EFFECT_VTABLE);
    let weapon_change = e.get(this, BoundItemEffect::pDisplacedWeapon);
    if weapon_change != 0 && e.get(this, BoundItemEffect::bWeaponCached) != 0 {
        let change = e.get(this, BoundItemEffect::pDisplacedWeapon);
        let form = e.call(WORD_AT_8_0044DDC0, &args![change]).u32();
        let weapon = cast(e, form, TES_BOUND_OBJECT_TYPE, TES_OBJECT_WEAP_TYPE);
        if weapon != 0 && has_model(e, weapon + 0x3c) {
            let model = model_of(e, weapon + 0x3c);
            release_model(e, model);
        }
    }
    if e.get(this, BoundItemEffect::bWornCached) != 0 {
        let mut sex = 0;
        if e.call(EFFECT_TARGET_0059BB30, &args![this]).u32() != 0 {
            let target = e.call(EFFECT_TARGET_0059BB30, &args![this]).u32();
            if e.vcall(target, 4, &[]).u32() != 0 {
                let target = e.call(EFFECT_TARGET_0059BB30, &args![this]).u32();
                let reference = e.vcall(target, 4, &[]).u32();
                let base = e.call(BASE_FORM_007AF430, &args![reference]).u32();
                let npc = cast(e, base, TES_BOUND_OBJECT_TYPE, TES_NPC_TYPE);
                if npc != 0 {
                    sex = e.call(TES_ACTOR_BASE_GET_SEX, &args![npc]).u32();
                }
            }
        }
        for index in 0..DISPLACED_ITEM_COUNT {
            let change = e.mem.u32(displaced_item(this, index));
            if change == 0 || e.call(WORD_AT_8_0044DDC0, &args![change]).u32() == 0 {
                continue;
            }
            let change = e.mem.u32(displaced_item(this, index));
            let form = e.call(WORD_AT_8_0044DDC0, &args![change]).u32();
            let biped = cast(e, form, TES_BOUND_OBJECT_TYPE, TES_BIPED_MODEL_FORM_TYPE);
            if biped != 0 {
                e.set(this, BoundItemEffect::bWornCached, 1);
                let model = e
                    .call(TES_BIPED_MODEL_FORM_GET_BIPED_MODEL, &args![biped, sex])
                    .u32();
                release_model(e, model);
            }
        }
    }
    let weapon_change = e.get(this, BoundItemEffect::pDisplacedWeapon);
    if weapon_change != 0 {
        let extra = first_extra_list(e, weapon_change);
        release_item_change(e, weapon_change, extra);
    }
    for index in 0..DISPLACED_ITEM_COUNT {
        let change = e.mem.u32(displaced_item(this, index));
        if change != 0 {
            let extra = first_extra_list(e, change);
            release_item_change(e, change, extra);
        }
    }
    if e.get(this, BoundItemEffect::bSpellModelCached) != 0
        && e.call(WORD_AT_8_0044DDC0, &args![this]).u32() != 0
        && e.call(WORD_AT_C_0084E3A0, &args![this]).u32() != 0
    {
        let (setting, costliest) = setting_and_costliest(e, this);
        if has_model(e, setting.wrapping_add(0x18))
            && e.call(WORD_AT_C_0084E3A0, &args![this]).u32() == costliest
        {
            let model = model_of(e, setting.wrapping_add(0x18));
            release_model(e, model);
        }
        e.set(this, BoundItemEffect::bSpellModelCached, 0);
    }
    e.call(BASE_DESTRUCTOR_0080D930, &args![this]);
}

// Translated from 008082c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008082c0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `BoundItemEffect::Copy` (Xbox PDB): a new 0xac-byte effect (`fn_00807e10` with the
/// caster, spell and effect item; null when the allocation fails) given this effect's data
/// (`AssociatedItemEffect::CopyData`), copies of the displaced weapon and items
/// (`ItemChange_ov2`), the cached models queued again for it (`QueueModel`; the weapon's
/// with `bWeaponCached`, the displaced items' biped models with `bWornCached`, for the
/// sex of the target's slot +4 reference's `TESNPC` base), and `bItemSummoned`.
pub fn bound_item_effect_copy(e: &mut Engine, this: Ptr<BoundItemEffect>) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0xacu32]).u32();
    let copy = if block == 0 {
        0
    } else {
        let base = this.addr();
        let item = e.mem.u32(base + 0xc);
        let spell = e.mem.u32(base + 8);
        let caster = e.mem.u32(base + 0x28);
        fn_00807e10(e, Ptr::new(block), caster, spell, item)
    };
    let copy: Ptr<BoundItemEffect> = Ptr::new(copy);
    e.call(ASSOCIATED_ITEM_EFFECT_COPY_DATA, &args![this, copy]);
    let weapon_change = e.get(this, BoundItemEffect::pDisplacedWeapon);
    if weapon_change != 0 {
        let change = e.get(this, BoundItemEffect::pDisplacedWeapon);
        let copied = copy_item_change(e, change);
        e.set(copy, BoundItemEffect::pDisplacedWeapon, copied);
    }
    for index in 0..DISPLACED_ITEM_COUNT {
        let change = e.mem.u32(displaced_item(this, index));
        if change != 0 {
            let copied = copy_item_change(e, change);
            e.mem.set_u32(displaced_item(copy, index), copied);
        }
    }
    let weapon_change = e.get(this, BoundItemEffect::pDisplacedWeapon);
    if weapon_change != 0 && e.get(this, BoundItemEffect::bWeaponCached) != 0 {
        let change = e.get(this, BoundItemEffect::pDisplacedWeapon);
        let form = e.call(WORD_AT_8_0044DDC0, &args![change]).u32();
        let weapon = cast(e, form, TES_BOUND_OBJECT_TYPE, TES_OBJECT_WEAP_TYPE);
        if weapon != 0 && has_model(e, weapon + 0x3c) {
            e.set(copy, BoundItemEffect::bWeaponCached, 1);
            let model = model_of(e, weapon + 0x3c);
            queue_model(e, model);
        }
    }
    if e.get(this, BoundItemEffect::bWornCached) != 0 {
        let mut sex = 0;
        let target = e.call(EFFECT_TARGET_0059BB30, &args![this]).u32();
        if e.vcall(target, 4, &[]).u32() != 0 {
            let target = e.call(EFFECT_TARGET_0059BB30, &args![this]).u32();
            let reference = e.vcall(target, 4, &[]).u32();
            let base = e.call(BASE_FORM_007AF430, &args![reference]).u32();
            let npc = cast(e, base, TES_BOUND_OBJECT_TYPE, TES_NPC_TYPE);
            if npc != 0 {
                sex = e.call(TES_ACTOR_BASE_GET_SEX, &args![npc]).u32();
            }
        }
        for index in 0..DISPLACED_ITEM_COUNT {
            let change = e.mem.u32(displaced_item(this, index));
            if change == 0 || e.call(WORD_AT_8_0044DDC0, &args![change]).u32() == 0 {
                continue;
            }
            let change = e.mem.u32(displaced_item(this, index));
            let form = e.call(WORD_AT_8_0044DDC0, &args![change]).u32();
            let biped = cast(e, form, TES_BOUND_OBJECT_TYPE, TES_BIPED_MODEL_FORM_TYPE);
            if biped != 0 {
                e.set(copy, BoundItemEffect::bWornCached, 1);
                let model = e
                    .call(TES_BIPED_MODEL_FORM_GET_BIPED_MODEL, &args![biped, sex])
                    .u32();
                queue_model(e, model);
            }
        }
    }
    let summoned = e.get(this, BoundItemEffect::bItemSummoned);
    e.set(copy, BoundItemEffect::bItemSummoned, summoned);
    copy.addr()
}

// Translated from 008085f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008085f0 - the exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed (`bounditemeffect.cpp`; cdecl, three words): a new 0xac-byte `BoundItemEffect`
/// (`fn_00807e10` with the three words; null when the allocation fails).
pub fn fn_008085f0(e: &mut Engine, caster: u32, spell: u32, item: u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0xacu32]).u32();
    if block == 0 {
        0
    } else {
        fn_00807e10(e, Ptr::new(block), caster, spell, item)
    }
}

// Translated from 00808680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BoundItemEffect::Start` (Xbox PDB): without an associated item (`006733e0`) or a
/// target actor the effect is dispelled (`Dispel` with 0). Otherwise, with a spell, an
/// effect item and a setting model (+0x18 of `GetAVEffect`), it loads that model
/// (`LoadFile`): the appear time is 0 for a spell of type 4 (slot +0x18) or the time
/// `004b6220` gives for the loaded file with `"SpecialIdle_HitEffect"` and `"Hit"`; and
/// unless the effect item is the costliest, the model is released again.
pub fn bound_item_effect_start(e: &mut Engine, this: Ptr<BoundItemEffect>) {
    let actor = target_actor(e, this);
    if e.call(ASSOCIATED_ITEM_006733E0, &args![this]).u32() == 0 || actor == 0 {
        e.call(ACTIVE_EFFECT_DISPEL, &args![this, 0u32]);
        return;
    }
    if e.call(WORD_AT_8_0044DDC0, &args![this]).u32() == 0
        || e.call(WORD_AT_C_0084E3A0, &args![this]).u32() == 0
    {
        return;
    }
    let (setting, costliest) = setting_and_costliest(e, this);
    if !has_model(e, setting.wrapping_add(0x18)) {
        return;
    }
    let model = model_of(e, setting.wrapping_add(0x18));
    let file = load_model_file(e, model);
    let spell = e.call(WORD_AT_8_0044DDC0, &args![this]).u32();
    let spell_type_4 = e.vcall(spell, 0x18, &[]).u32() == 4;
    if !spell_type_4 {
        if file != 0 {
            let time = e
                .call(
                    MODEL_TIME_004B6220,
                    &args![file, SPECIAL_IDLE_HIT_EFFECT_STRING, HIT_STRING],
                )
                .f64();
            e.mem.set_u32(this.addr() + 0xa0, x87_store32(time));
        }
    } else {
        e.mem.set_u32(this.addr() + 0xa0, 0);
    }
    if e.call(WORD_AT_C_0084E3A0, &args![this]).u32() != costliest {
        let model = model_of(e, setting.wrapping_add(0x18));
        release_model(e, model);
    }
}

// Translated from 00808800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BoundItemEffect::Update` (Xbox PDB; the `float` argument is not read): before the item
/// is summoned, once the appear time is at most the elapsed seconds (`FCOMPP`, ordered) it
/// summons it (`fn_00808a30`) and sets `bItemSummoned`. After, for a target actor whose
/// `008d8520` object is set and an associated `TESObjectWEAP`: that object's slot +0x530
/// gets the weapon; with `bSetupItem`, when its slot +0x148 item change holds the weapon
/// and has extra data, the first list is made unwearable (`SetCanNotWear` 1) and
/// `bSetupItem` cleared; for the player, when neither slot +0x454 nor `008a6970` is true,
/// the effect is dispelled. Otherwise, with `bSetupItem` and an associated `TESObjectARMO`
/// the actor wears (`WearingObject` with 1), that list is made unwearable and
/// `bSetupItem` cleared.
pub fn bound_item_effect_update(e: &mut Engine, this: Ptr<BoundItemEffect>, _unused_1: u32) {
    if e.get(this, BoundItemEffect::bItemSummoned) == 0 {
        let elapsed = f32::from_bits(e.mem.u32(this.addr() + 4));
        let appear = f32::from_bits(e.mem.u32(this.addr() + 0xa0));
        // FLD elapsed; FLD appear; FCOMPP; TEST AH,0x41; JP: summoned when appear <= elapsed.
        if appear <= elapsed {
            fn_00808a30(e, this);
            e.set(this, BoundItemEffect::bItemSummoned, 1);
        }
        return;
    }
    let actor = target_actor(e, this);
    let item = e.call(ASSOCIATED_ITEM_006733E0, &args![this]).u32();
    let weapon = cast(e, item, TES_FORM_TYPE, TES_OBJECT_WEAP_TYPE);
    if actor != 0 && e.call(ACTOR_OBJECT_008D8520, &args![actor]).u32() != 0 && weapon != 0 {
        let object = e.call(ACTOR_OBJECT_008D8520, &args![actor]).u32();
        e.vcall(object, 0x530, &args![weapon]);
        if e.get(this, BoundItemEffect::bSetupItem) != 0 {
            let object = e.call(ACTOR_OBJECT_008D8520, &args![actor]).u32();
            if e.vcall(object, 0x148, &[]).u32() != 0 {
                let object = e.call(ACTOR_OBJECT_008D8520, &args![actor]).u32();
                let change = e.vcall(object, 0x148, &[]).u32();
                if weapon == e.call(WORD_AT_8_0044DDC0, &args![change]).u32()
                    && e.call(EXTRA_LISTS_00559450, &args![change]).u32() != 0
                {
                    e.set(this, BoundItemEffect::bSetupItem, 0);
                    let lists = e.call(EXTRA_LISTS_00559450, &args![change]).u32();
                    let slot = e.call(LIST_NODE_ITEM, &args![lists]).u32();
                    let extra = e.mem.u32(slot);
                    e.call(EXTRA_DATA_LIST_SET_CAN_NOT_WEAR, &args![extra, 1u32]);
                }
            }
        }
        if actor == e.global::<u32>(PLAYER_POINTER) {
            let object = e.call(ACTOR_OBJECT_008D8520, &args![actor]).u32();
            if !e.vcall(object, 0x454, &[]).bool()
                && !e.call(ACTOR_TEST_008A6970, &args![actor]).bool()
            {
                e.call(ACTIVE_EFFECT_DISPEL, &args![this, 0u32]);
            }
        }
        return;
    }
    if e.get(this, BoundItemEffect::bSetupItem) != 0 {
        let item = e.call(ASSOCIATED_ITEM_006733E0, &args![this]).u32();
        let armor = cast(e, item, TES_FORM_TYPE, TES_OBJECT_ARMO_TYPE);
        if armor != 0 {
            let changes = e.call(INVENTORY_CHANGES_GET, &args![actor]).u32();
            let extra = e
                .call(
                    INVENTORY_CHANGES_WEARING_OBJECT,
                    &args![changes, armor, 1u32],
                )
                .u32();
            if extra != 0 {
                e.set(this, BoundItemEffect::bSetupItem, 0);
                e.call(EXTRA_DATA_LIST_SET_CAN_NOT_WEAR, &args![extra, 1u32]);
            }
        }
    }
}

// Translated from 00808a30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00808a30 - the exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed (`bounditemeffect.cpp`): summons the bound item on the target actor. For a
/// `TESObjectWEAP`: when the actor's `008d8520` object holds a weapon (slot +0x148), that
/// item change's first extra-data list is kept unless it holds fewer than 2 items or 2
/// with a count over 1; the old weapon's model is loaded once (`bWeaponCached`); it is
/// unequipped (`QueueUnEquipObject` with the form, 1, that list, 0, 0, 1) and remembered
/// as a new `ItemChange` (count 0) with a duplicate of the kept list. The weapon is then
/// added (slot +0x190 with 0, 1), equipped unless already held (`QueueEquipObject` with 1,
/// 0, 1, 1, 1), made unwearable (or `bSetupItem` set when no list yet), and the object's
/// slots +0x450 (1) and +0x530 (the weapon) run. For a `TESObjectARMO`: each biped slot it
/// fills that the actor wears (`GetWornItem`) is remembered (a copy) after its biped model
/// is loaded (once, `bWornCached`; the actor's sex from its `TESNPC` base) and its list
/// handled (`0041b010` with 0; `00905330` when `004bca60`), then the worn change deleted;
/// the armor is added with a new list (slot +0x190 with 1), equipped (1, 0, 1, 0, 1) and
/// its worn list made unwearable (or `bSetupItem` set). Without either, the effect is
/// dispelled.
pub fn fn_00808a30(e: &mut Engine, this: Ptr<BoundItemEffect>) {
    e.with_stack(0x30, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x30 - offset;
        let actor = target_actor(e, this);
        let item = e.call(ASSOCIATED_ITEM_006733E0, &args![this]).u32();
        let weapon = cast(e, item, TES_FORM_TYPE, TES_OBJECT_WEAP_TYPE);
        let item = e.call(ASSOCIATED_ITEM_006733E0, &args![this]).u32();
        let armor = cast(e, item, TES_FORM_TYPE, TES_OBJECT_ARMO_TYPE);
        let process = |e: &mut Engine| e.call(ACTOR_OBJECT_008D8520, &args![actor]).u32();
        if weapon != 0 {
            let object = process(e);
            if e.vcall(object, 0x148, &[]).u32() != 0 {
                let object = process(e);
                let change = e.vcall(object, 0x148, &[]).u32();
                let node = e.mem.u32(change);
                let slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
                let mut extra = e.mem.u32(slot);
                let items = e.call(BASE_EXTRA_LIST_ITEMS_IN_LIST, &args![extra]).u32();
                if items < 2 {
                    extra = 0;
                } else if e.call(BASE_EXTRA_LIST_ITEMS_IN_LIST, &args![extra]).u32() == 2 {
                    let count = e.call(EXTRA_DATA_LIST_GET_COUNT, &args![extra]).u16();
                    if i32::from(count as i16) > 1 {
                        extra = 0;
                    }
                }
                let object = process(e);
                let change = e.vcall(object, 0x148, &[]).u32();
                let old = e.call(WORD_AT_8_0044DDC0, &args![change]).u32();
                if old != 0 && e.get(this, BoundItemEffect::bWeaponCached) == 0 {
                    let old_weapon = cast(e, old, TES_BOUND_OBJECT_TYPE, TES_OBJECT_WEAP_TYPE);
                    if old_weapon != 0 && has_model(e, old_weapon + 0x3c) {
                        let model = model_of(e, old_weapon + 0x3c);
                        load_model_file(e, model);
                        e.set(this, BoundItemEffect::bWeaponCached, 1);
                    }
                }
                let first = process(e);
                let second = process(e);
                let change = e.vcall(first, 0x148, &[]).u32();
                let node = e.mem.u32(change);
                let slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
                let held_extra = e.mem.u32(slot);
                let change = e.vcall(second, 0x148, &[]).u32();
                let form = e.call(WORD_AT_8_0044DDC0, &args![change]).u32();
                e.call(
                    ACTOR_QUEUE_UNEQUIP_OBJECT,
                    &args![actor, form, 1u32, held_extra, 0u32, 0u32, 1u32],
                );
                let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
                let remembered = if block == 0 {
                    0
                } else {
                    e.call(ITEM_CHANGE_CONSTRUCTOR, &args![block, old, 0u32])
                        .u32()
                };
                e.set(this, BoundItemEffect::pDisplacedWeapon, remembered);
                if extra != 0 {
                    let block = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
                    let list = if block == 0 {
                        0
                    } else {
                        e.call(EXTRA_DATA_LIST_CONSTRUCTOR, &args![block]).u32()
                    };
                    e.mem.set_u32(local(0x2c), list);
                    e.call(EXTRA_DATA_LIST_DUPLICATE, &args![list, extra]);
                    let change = e.get(this, BoundItemEffect::pDisplacedWeapon);
                    let lists = e.mem.u32(change);
                    e.call(LIST_INSERT_FRONT_005AE3D0, &args![lists, local(0x2c)]);
                }
            }
            e.vcall(actor, 0x190, &args![weapon, 0u32, 1u32]);
            let object = process(e);
            let holds = if e.vcall(object, 0x148, &[]).u32() == 0 {
                false
            } else {
                let object = process(e);
                let change = e.vcall(object, 0x148, &[]).u32();
                e.call(WORD_AT_8_0044DDC0, &args![change]).u32() == weapon
            };
            if !holds {
                e.call(
                    ACTOR_QUEUE_EQUIP_OBJECT,
                    &args![actor, weapon, 1u32, 0u32, 1u32, 1u32, 1u32],
                );
            }
            let mut set_up = false;
            if process(e) != 0 {
                let object = process(e);
                if e.vcall(object, 0x148, &[]).u32() != 0 {
                    let object = process(e);
                    let change = e.vcall(object, 0x148, &[]).u32();
                    if e.call(EXTRA_LISTS_00559450, &args![change]).u32() != 0 {
                        let object = process(e);
                        let change = e.vcall(object, 0x148, &[]).u32();
                        let lists = e.call(EXTRA_LISTS_00559450, &args![change]).u32();
                        let slot = e.call(LIST_NODE_ITEM, &args![lists]).u32();
                        let list = e.mem.u32(slot);
                        e.call(EXTRA_DATA_LIST_SET_CAN_NOT_WEAR, &args![list, 1u32]);
                        set_up = true;
                    }
                }
            }
            if !set_up {
                e.set(this, BoundItemEffect::bSetupItem, 1);
            }
            let object = process(e);
            e.vcall(object, 0x450, &args![1u32]);
            let object = process(e);
            e.vcall(object, 0x530, &args![weapon]);
        } else if armor != 0 {
            let sex = actor_sex(e, actor);
            let worn_cached = e.get(this, BoundItemEffect::bWornCached);
            for index in 0..DISPLACED_ITEM_COUNT {
                let mut worn = 0u32;
                if e.call(
                    TES_BIPED_MODEL_FORM_FILLS_BIPED_SLOT,
                    &args![armor + 0x70, index, 0u32, 0u32],
                )
                .bool()
                {
                    let list = e.call(EXTRA_LIST_005D43C0, &args![actor]).u32();
                    let changes = e
                        .call(EXTRA_DATA_LIST_GET_CONTAINER_CHANGES, &args![list])
                        .u32();
                    if changes != 0 {
                        worn = e
                            .call(
                                INVENTORY_CHANGES_GET_WORN_ITEM,
                                &args![changes, index, 0u32],
                            )
                            .u32();
                    }
                }
                if worn == 0 {
                    continue;
                }
                if e.call(WORD_AT_8_0044DDC0, &args![worn]).u32() != 0 && worn_cached == 0 {
                    let form = e.call(WORD_AT_8_0044DDC0, &args![worn]).u32();
                    let biped = cast(e, form, TES_BOUND_OBJECT_TYPE, TES_BIPED_MODEL_FORM_TYPE);
                    if biped != 0 {
                        let model = e
                            .call(TES_BIPED_MODEL_FORM_GET_BIPED_MODEL, &args![biped, sex])
                            .u32();
                        load_model_file(e, model);
                        e.set(this, BoundItemEffect::bWornCached, 1);
                    }
                }
                let node = e.mem.u32(worn);
                let slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
                let extra = e.mem.u32(slot);
                e.mem.set_u32(local(0x20), extra);
                if extra != 0 {
                    e.call(EXTRA_LIST_0041B010, &args![extra, 0u32]);
                    if e.call(EXTRA_LIST_TEST_004BCA60, &args![extra]).bool() {
                        let lists = e.call(EXTRA_LISTS_00559450, &args![worn]).u32();
                        e.call(LIST_REMOVE_00905330, &args![lists, local(0x20)]);
                    }
                }
                let copied = copy_item_change(e, worn);
                e.mem.set_u32(displaced_item(this, index), copied);
                e.call(DELETE_004459E0, &args![worn, 1u32]);
            }
            let block = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
            let list = if block == 0 {
                0
            } else {
                e.call(EXTRA_DATA_LIST_CONSTRUCTOR, &args![block]).u32()
            };
            e.vcall(actor, 0x190, &args![armor, list, 1u32]);
            e.call(
                ACTOR_QUEUE_EQUIP_OBJECT,
                &args![actor, armor, 1u32, 0u32, 1u32, 0u32, 1u32],
            );
            let changes = e.call(INVENTORY_CHANGES_GET, &args![actor]).u32();
            let worn_list = e
                .call(
                    INVENTORY_CHANGES_WEARING_OBJECT,
                    &args![changes, armor, 1u32],
                )
                .u32();
            if worn_list != 0 {
                e.call(EXTRA_DATA_LIST_SET_CAN_NOT_WEAR, &args![worn_list, 1u32]);
            } else {
                e.set(this, BoundItemEffect::bSetupItem, 1);
            }
        } else {
            e.call(ACTIVE_EFFECT_DISPEL, &args![this, 0u32]);
        }
    });
}

// Translated from 008091a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008091a0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `BoundItemEffect::Finish` (Xbox PDB): only for a summoned item and a target actor. A
/// `TESObjectWEAP` loses its unwearable list (`RemoveCannotWearExtra`), is unequipped (1,
/// 0, 0, 0, 1) and removed (slot +0x17c); a displaced weapon the actor still has is
/// equipped again (1, 0, 1, 0, 1), its cached model released, and it is released
/// (`release_item_change`, cleared), then the `008d8520` object's slot +0x450 runs with 1.
/// A `TESObjectARMO` the same way: for each biped slot it fills with a displaced item the
/// actor still has, that item is equipped again and, when the slot is not worn by it,
/// handed to `fn_0080a700`; its biped model released unless that took it; each displaced
/// item of those slots is released and cleared, and `bWornCached` cleared. Then, unless the
/// actor's slot +0x1d0 is null, its `008d8520` object null or `0045cd60` set, with a spell
/// and the costliest effect item: a setting model makes a `MagicModelHitEffect` (with -1)
/// added to the process lists (or destroyed), the spell model is released once, and
/// `"ITMBoundDisappear"` plays. Last `007037c0` with 0 and `bItemSummoned` cleared.
pub fn bound_item_effect_finish(e: &mut Engine, this: Ptr<BoundItemEffect>) {
    e.with_stack(0x10, |e, handle| {
        let actor = target_actor(e, this);
        if e.get(this, BoundItemEffect::bItemSummoned) == 0 || actor == 0 {
            return;
        }
        let process = |e: &mut Engine| e.call(ACTOR_OBJECT_008D8520, &args![actor]).u32();
        let item = e.call(ASSOCIATED_ITEM_006733E0, &args![this]).u32();
        let weapon = cast(e, item, TES_FORM_TYPE, TES_OBJECT_WEAP_TYPE);
        let item = e.call(ASSOCIATED_ITEM_006733E0, &args![this]).u32();
        let armor = cast(e, item, TES_FORM_TYPE, TES_OBJECT_ARMO_TYPE);
        if weapon != 0 {
            let changes = e.call(INVENTORY_CHANGES_GET, &args![actor]).u32();
            let worn = e
                .call(
                    INVENTORY_CHANGES_WEARING_OBJECT,
                    &args![changes, weapon, 0u32],
                )
                .u32();
            if worn != 0 {
                e.call(EXTRA_DATA_LIST_REMOVE_CANNOT_WEAR, &args![worn]);
            }
            e.call(
                ACTOR_QUEUE_UNEQUIP_OBJECT,
                &args![actor, weapon, 1u32, 0u32, 0u32, 0u32, 1u32],
            );
            e.vcall(
                actor,
                0x17c,
                &args![weapon, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32],
            );
            let change = e.get(this, BoundItemEffect::pDisplacedWeapon);
            if change != 0 {
                let extra = first_extra_list(e, change);
                let change = e.get(this, BoundItemEffect::pDisplacedWeapon);
                let form = e.call(WORD_AT_8_0044DDC0, &args![change]).u32();
                if e.call(ACTOR_GET_ITEM_COUNT, &args![actor, form]).i32() >= 1 {
                    let change = e.get(this, BoundItemEffect::pDisplacedWeapon);
                    let form = e.call(WORD_AT_8_0044DDC0, &args![change]).u32();
                    e.call(
                        ACTOR_QUEUE_EQUIP_OBJECT,
                        &args![actor, form, 1u32, 0u32, 1u32, 0u32, 1u32],
                    );
                }
                if e.get(this, BoundItemEffect::bWeaponCached) != 0 {
                    let change = e.get(this, BoundItemEffect::pDisplacedWeapon);
                    let form = e.call(WORD_AT_8_0044DDC0, &args![change]).u32();
                    let old = cast(e, form, TES_BOUND_OBJECT_TYPE, TES_OBJECT_WEAP_TYPE);
                    if old != 0 && has_model(e, old + 0x3c) {
                        let model = model_of(e, old + 0x3c);
                        release_model(e, model);
                    }
                    e.set(this, BoundItemEffect::bWeaponCached, 0);
                }
                let change = e.get(this, BoundItemEffect::pDisplacedWeapon);
                release_item_change(e, change, extra);
                e.set(this, BoundItemEffect::pDisplacedWeapon, 0);
                let object = process(e);
                e.vcall(object, 0x450, &args![1u32]);
            }
        } else if armor != 0 {
            let sex = actor_sex(e, actor);
            let changes = e.call(INVENTORY_CHANGES_GET, &args![actor]).u32();
            let worn = e
                .call(
                    INVENTORY_CHANGES_WEARING_OBJECT,
                    &args![changes, armor, 0u32],
                )
                .u32();
            if worn != 0 {
                e.call(EXTRA_DATA_LIST_REMOVE_CANNOT_WEAR, &args![worn]);
            }
            e.call(
                ACTOR_QUEUE_UNEQUIP_OBJECT,
                &args![actor, armor, 1u32, 0u32, 0u32, 0u32, 1u32],
            );
            e.vcall(
                actor,
                0x17c,
                &args![armor, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32],
            );
            for index in 0..DISPLACED_ITEM_COUNT {
                if !e
                    .call(
                        TES_BIPED_MODEL_FORM_FILLS_BIPED_SLOT,
                        &args![armor + 0x70, index, 0u32, 0u32],
                    )
                    .bool()
                    || e.mem.u32(displaced_item(this, index)) == 0
                {
                    continue;
                }
                let change = e.mem.u32(displaced_item(this, index));
                let extra = first_extra_list(e, change);
                let mut handed = false;
                let form = e.call(WORD_AT_8_0044DDC0, &args![change]).u32();
                if e.call(ACTOR_GET_ITEM_COUNT, &args![actor, form]).i32() >= 1 {
                    let form = e.call(WORD_AT_8_0044DDC0, &args![change]).u32();
                    e.call(
                        ACTOR_QUEUE_EQUIP_OBJECT,
                        &args![actor, form, 1u32, 0u32, 1u32, 0u32, 1u32],
                    );
                    let list = e.call(EXTRA_LIST_005D43C0, &args![actor]).u32();
                    let changes = e
                        .call(EXTRA_DATA_LIST_GET_CONTAINER_CHANGES, &args![list])
                        .u32();
                    let now_worn = e
                        .call(
                            INVENTORY_CHANGES_GET_WORN_ITEM,
                            &args![changes, index, 0u32],
                        )
                        .u32();
                    let same = now_worn != 0 && {
                        let worn_form = e.call(WORD_AT_8_0044DDC0, &args![now_worn]).u32();
                        worn_form == e.call(WORD_AT_8_0044DDC0, &args![change]).u32()
                    };
                    if !same {
                        handed = fn_0080a700(e, actor, change, this.addr());
                    }
                    if now_worn != 0 {
                        e.call(DELETE_004459E0, &args![now_worn, 1u32]);
                    }
                }
                if e.get(this, BoundItemEffect::bWornCached) != 0
                    && e.call(WORD_AT_8_0044DDC0, &args![change]).u32() != 0
                    && !handed
                {
                    let form = e.call(WORD_AT_8_0044DDC0, &args![change]).u32();
                    let biped = cast(e, form, TES_BOUND_OBJECT_TYPE, TES_BIPED_MODEL_FORM_TYPE);
                    if biped != 0 {
                        let model = e
                            .call(TES_BIPED_MODEL_FORM_GET_BIPED_MODEL, &args![biped, sex])
                            .u32();
                        release_model(e, model);
                    }
                }
                release_item_change(e, change, extra);
                e.mem.set_u32(displaced_item(this, index), 0);
            }
            e.set(this, BoundItemEffect::bWornCached, 0);
        }
        'spell: {
            if e.vcall(actor, 0x1d0, &[]).u32() == 0 || process(e) == 0 {
                break 'spell;
            }
            let object = process(e);
            if e.call(PROCESS_TEST_0045CD60, &args![object]).u32() != 0
                || e.call(WORD_AT_8_0044DDC0, &args![this]).u32() == 0
                || e.call(WORD_AT_C_0084E3A0, &args![this]).u32() == 0
            {
                break 'spell;
            }
            let (setting, costliest) = setting_and_costliest(e, this);
            if e.call(WORD_AT_C_0084E3A0, &args![this]).u32() != costliest {
                break 'spell;
            }
            let effect_setting = e.call(EFFECT_SETTING_007DF1D0, &args![this]).u32();
            if has_model(e, effect_setting.wrapping_add(0x18)) {
                let block = e.call(ALLOCATE_00AA13E0, &args![0x3cu32]).u32();
                let hit_effect = if block == 0 {
                    0
                } else {
                    let scale = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
                    let model = model_of(e, setting.wrapping_add(0x18));
                    e.call(
                        MAGIC_MODEL_HIT_EFFECT_CONSTRUCTOR,
                        &args![block, actor, model, scale],
                    )
                    .u32()
                };
                if e.vcall(hit_effect, 0xc4, &[]).bool() {
                    e.call(
                        PROCESS_LISTS_ADD_TEMP_EFFECT,
                        &args![PROCESS_LISTS, hit_effect],
                    );
                } else if hit_effect != 0 {
                    e.vcall(hit_effect, 0, &args![1u32]);
                }
                if e.get(this, BoundItemEffect::bSpellModelCached) != 0 {
                    let model = model_of(e, setting.wrapping_add(0x18));
                    release_model(e, model);
                    e.set(this, BoundItemEffect::bSpellModelCached, 0);
                }
            }
            e.call(
                PLAY_SOUND_BY_EDITOR_NAME,
                &args![
                    actor,
                    handle,
                    ITM_BOUND_DISAPPEAR_STRING,
                    0u32,
                    0x4000_0102u32,
                    1u32
                ],
            );
            e.call(SOUND_HANDLE_DESTROY_00483710, &args![handle]);
        }
        e.call(FINISH_HELPER_007037C0, &args![this, 0u32]);
        e.set(this, BoundItemEffect::bItemSummoned, 0);
    });
}

// Translated from 008099c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`bounditemeffect.cpp`; thiscall, one word): whether `target` is a `Character`
/// (`__RTDynamicCast` from `MagicTarget`); `this` is not read.
pub fn fn_008099c0(e: &mut Engine, _this: Ptr<BoundItemEffect>, target: u32) -> bool {
    cast(e, target, MAGIC_TARGET_TYPE, CHARACTER_TYPE) != 0
}

// Translated from 00809a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BoundItemEffect::GetSaveSize` (Xbox PDB): the base size (`00806da0`), 6 more with
/// save-game blocks, 1 and the displaced weapon's size (`004be5a0`), 1 per displaced item
/// slot and each item's size; the counted part is reported through `Error` when the
/// setting at `011de4e8` is set; then 5 more from save version 0x41 and 1 from 0x6b.
/// Returns `AX` (16-bit sums).
pub fn bound_item_effect_get_save_size(
    e: &mut Engine,
    this: Ptr<BoundItemEffect>,
    argument: u32,
) -> u16 {
    let base = e
        .call(BASE_SAVE_SIZE_00806DA0, &args![this, argument])
        .u16();
    let mut size = base;
    let start = size;
    if use_save_game_blocks(e) {
        size = size.wrapping_add(4);
        size = size.wrapping_add(2);
    }
    size = size.wrapping_add(1);
    let change = e.get(this, BoundItemEffect::pDisplacedWeapon);
    if change != 0 {
        let own = e.call(ITEM_CHANGE_SAVE_SIZE_004BE5A0, &args![change]).u16();
        size = size.wrapping_add(own);
    }
    for index in 0..DISPLACED_ITEM_COUNT {
        size = size.wrapping_add(1);
        let change = e.mem.u32(displaced_item(this, index));
        if change != 0 {
            let own = e.call(ITEM_CHANGE_SAVE_SIZE_004BE5A0, &args![change]).u16();
            size = size.wrapping_add(own);
        }
    }
    if size_reports_enabled(e) {
        let counted = u32::from(size).wrapping_sub(u32::from(start));
        report_save_size(
            e,
            COUNTED_SIZE_FORMATS,
            counted,
            BOUND_ITEM_EFFECT_FILE,
            0x29b,
        );
    }
    if save_version(e) >= 0x41 {
        size = size.wrapping_add(4);
        size = size.wrapping_add(1);
    }
    if save_version(e) >= 0x6b {
        size = size.wrapping_add(1);
    }
    size
}

// Translated from 00809bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BoundItemEffect::SaveGame` (Xbox PDB): `AssociatedItemEffect::SaveGame`; with
/// save-game blocks the marker and a size to fill in; a presence byte and the displaced
/// weapon (`004be6f0`), the same for each displaced item slot; the appear time and
/// `bItemSummoned` (save version 0x41 or later) and `bSetupItem` (0x6b or later). The
/// bytes written are reported through `Error` with the setting at `011de4e8`, and the
/// block's size stored with blocks.
pub fn bound_item_effect_save_game(e: &mut Engine, this: Ptr<BoundItemEffect>, argument: u32) {
    e.call(ASSOCIATED_ITEM_EFFECT_SAVE_GAME, &args![this, argument]);
    e.with_stack(0x30, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x30 - offset;
        e.mem.set_u16(local(0x10), 0);
        let mut size_at = 0u32;
        let mut start = save_position(e);
        if size_reports_enabled(e) {
            start = save_position(e);
        }
        if use_save_game_blocks(e) {
            size_at = save_block_start(e, local(0x18), local(0x10));
        }
        let present = local(1);
        let change = e.get(this, BoundItemEffect::pDisplacedWeapon);
        e.mem.set_u8(present, u8::from(change != 0));
        save_bytes(e, present, 1);
        if e.mem.u8(present) != 0 {
            let change = e.get(this, BoundItemEffect::pDisplacedWeapon);
            e.call(ITEM_CHANGE_SAVE_004BE6F0, &args![change]);
        }
        for index in 0..DISPLACED_ITEM_COUNT {
            let change = e.mem.u32(displaced_item(this, index));
            e.mem.set_u8(present, u8::from(change != 0));
            save_bytes(e, present, 1);
            if e.mem.u8(present) != 0 {
                let change = e.mem.u32(displaced_item(this, index));
                e.call(ITEM_CHANGE_SAVE_004BE6F0, &args![change]);
            }
        }
        if save_version(e) >= 0x41 {
            save_bytes(e, this.addr() + 0xa0, 4);
            save_bytes(e, this.addr() + 0xa4, 1);
        }
        if save_version(e) >= 0x6b {
            save_bytes(e, this.addr() + 0xa8, 1);
        }
        if size_reports_enabled(e) {
            let end = save_position(e);
            let written = end.wrapping_sub(start);
            report_save_size(
                e,
                WRITTEN_SIZE_FORMATS,
                written,
                BOUND_ITEM_EFFECT_FILE,
                0x2d1,
            );
        }
        if use_save_game_blocks(e) {
            save_block_finish(e, size_at, BOUND_ITEM_EFFECT_FILE, 0x2d1);
        }
    });
}

/// A new displaced item change for a load (`ItemChange_ov3` in a 12-byte block; null when
/// the allocation fails).
fn new_item_change(e: &mut Engine) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
    if block == 0 {
        0
    } else {
        e.call(ITEM_CHANGE_EMPTY_CONSTRUCTOR, &args![block]).u32()
    }
}

// Translated from 00809e60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00809e60 - the exception-unwinding frame (FS:[0]) is not translated.
/// `BoundItemEffect::LoadGame` (Xbox PDB): `AssociatedItemEffect::LoadGame`; with
/// save-game blocks the marker check and the block's size; the displaced weapon and items
/// (a presence byte, then a new `ItemChange` loaded by `ItemChange::LoadGame`; an item whose
/// form is null is deleted and its slot cleared); `bItemSummoned` set; the appear time and
/// `bItemSummoned` (save version 0x41 or later) and `bSetupItem` (0x6b or later); with
/// blocks the position is checked against the block's size.
pub fn bound_item_effect_load_game(e: &mut Engine, this: Ptr<BoundItemEffect>, argument: u32) {
    e.call(ASSOCIATED_ITEM_EFFECT_LOAD_GAME, &args![this, argument]);
    e.with_stack(0x30, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x30 - offset;
        e.mem.set_u16(local(0x18), 0);
        let mut start = 0u32;
        if use_save_game_blocks(e) {
            start = load_block_start(e, local(0x20), local(0x18), BOUND_ITEM_EFFECT_FILE, 0x2da);
        }
        let present = local(0xd);
        load_bytes(e, present, 1);
        if e.mem.u8(present) != 0 {
            let change = new_item_change(e);
            e.set(this, BoundItemEffect::pDisplacedWeapon, change);
            let change = e.get(this, BoundItemEffect::pDisplacedWeapon);
            e.call(ITEM_CHANGE_LOAD_GAME, &args![change]);
        }
        for index in 0..DISPLACED_ITEM_COUNT {
            load_bytes(e, present, 1);
            if e.mem.u8(present) == 0 {
                continue;
            }
            let change = new_item_change(e);
            e.mem.set_u32(displaced_item(this, index), change);
            let change = e.mem.u32(displaced_item(this, index));
            e.call(ITEM_CHANGE_LOAD_GAME, &args![change]);
            let change = e.mem.u32(displaced_item(this, index));
            if e.call(WORD_AT_8_0044DDC0, &args![change]).u32() == 0 {
                let change = e.mem.u32(displaced_item(this, index));
                if change != 0 {
                    e.call(DELETE_004459E0, &args![change, 1u32]);
                }
                e.mem.set_u32(displaced_item(this, index), 0);
            }
        }
        e.set(this, BoundItemEffect::bItemSummoned, 1);
        if save_version(e) >= 0x41 {
            load_bytes(e, this.addr() + 0xa0, 4);
            load_bytes(e, this.addr() + 0xa4, 1);
        }
        if save_version(e) >= 0x6b {
            load_bytes(e, this.addr() + 0xa8, 1);
        }
        if use_save_game_blocks(e) {
            load_block_check(e, start, local(0x18), BOUND_ITEM_EFFECT_FILE, 0x302);
        }
    });
}

/// Before save version 0x61 the loaded displaced item change in `slot` is replaced by a
/// copy (`ItemChange_ov2`; the loaded one deleted).
fn replace_old_item_change(e: &mut Engine, slot: u32) {
    if save_version(e) >= 0x61 {
        return;
    }
    let change = e.mem.u32(slot);
    let copied = copy_item_change(e, change);
    let old = e.mem.u32(slot);
    if old != 0 {
        e.call(DELETE_004459E0, &args![old, 1u32]);
    }
    e.mem.set_u32(slot, copied);
}

// Translated from 0080a2d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080a2d0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `BoundItemEffect::InitLoadGame` (Xbox PDB): `AssociatedItemEffect::InitLoadGame`; for a
/// summoned item the cached models are queued again (the displaced weapon's when it is a
/// `TESObjectWEAP` with a model, setting `bWeaponCached`; each displaced item's biped model
/// for the target actor's sex, setting `bWornCached`). The displaced weapon and each
/// displaced item get their init (`004bed00`), are replaced by a copy before save version
/// 0x61, and are unequipped (`QueueUnEquipObject` with 1, 0, 0, 0, 1). For a summoned item
/// the actor holds none of (`GetObjectCount` 0) the associated item (as a `TESBoundObject`)
/// is added (slot +0x190 with 0, 1) and equipped (1, 0, 1, 0, 1).
pub fn bound_item_effect_init_load_game(e: &mut Engine, this: Ptr<BoundItemEffect>, argument: u32) {
    e.call(
        ASSOCIATED_ITEM_EFFECT_INIT_LOAD_GAME,
        &args![this, argument],
    );
    let actor = target_actor(e, this);
    if e.get(this, BoundItemEffect::bItemSummoned) != 0 {
        let change = e.get(this, BoundItemEffect::pDisplacedWeapon);
        if change != 0 {
            let form = e.call(WORD_AT_8_0044DDC0, &args![change]).u32();
            let weapon = cast(e, form, TES_BOUND_OBJECT_TYPE, TES_OBJECT_WEAP_TYPE);
            if weapon != 0 && has_model(e, weapon + 0x3c) {
                e.set(this, BoundItemEffect::bWeaponCached, 1);
                let model = model_of(e, weapon + 0x3c);
                let loader = model_loader(e);
                e.call(
                    MODEL_LOADER_QUEUE_MODEL,
                    &args![loader, model, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32],
                );
            }
        }
        let sex = if actor == 0 { 0 } else { actor_sex(e, actor) };
        for index in 0..DISPLACED_ITEM_COUNT {
            let change = e.mem.u32(displaced_item(this, index));
            if change == 0 || e.call(WORD_AT_8_0044DDC0, &args![change]).u32() == 0 {
                continue;
            }
            let change = e.mem.u32(displaced_item(this, index));
            let form = e.call(WORD_AT_8_0044DDC0, &args![change]).u32();
            let biped = cast(e, form, TES_BOUND_OBJECT_TYPE, TES_BIPED_MODEL_FORM_TYPE);
            if biped != 0 {
                e.set(this, BoundItemEffect::bWornCached, 1);
                let model = e
                    .call(TES_BIPED_MODEL_FORM_GET_BIPED_MODEL, &args![biped, sex])
                    .u32();
                queue_model(e, model);
            }
        }
    }
    let change = e.get(this, BoundItemEffect::pDisplacedWeapon);
    if change != 0 {
        e.call(ITEM_CHANGE_INIT_004BED00, &args![change]);
        replace_old_item_change(e, this.addr() + 0x4c);
        let change = e.get(this, BoundItemEffect::pDisplacedWeapon);
        let form = e.call(WORD_AT_8_0044DDC0, &args![change]).u32();
        e.call(
            ACTOR_QUEUE_UNEQUIP_OBJECT,
            &args![actor, form, 1u32, 0u32, 0u32, 0u32, 1u32],
        );
    }
    for index in 0..DISPLACED_ITEM_COUNT {
        let change = e.mem.u32(displaced_item(this, index));
        if change == 0 {
            continue;
        }
        e.call(ITEM_CHANGE_INIT_004BED00, &args![change]);
        replace_old_item_change(e, displaced_item(this, index));
        let change = e.mem.u32(displaced_item(this, index));
        let form = e.call(WORD_AT_8_0044DDC0, &args![change]).u32();
        e.call(
            ACTOR_QUEUE_UNEQUIP_OBJECT,
            &args![actor, form, 1u32, 0u32, 0u32, 0u32, 1u32],
        );
    }
    if e.get(this, BoundItemEffect::bItemSummoned) != 0 {
        let list = e.call(EXTRA_LIST_005D43C0, &args![actor]).u32();
        let changes = e
            .call(EXTRA_DATA_LIST_GET_CONTAINER_CHANGES, &args![list])
            .u32();
        let item = e.call(ASSOCIATED_ITEM_006733E0, &args![this]).u32();
        let bound = cast(e, item, TES_FORM_TYPE, TES_BOUND_OBJECT_TYPE);
        if changes != 0
            && e.call(INVENTORY_CHANGES_GET_OBJECT_COUNT, &args![changes, bound])
                .u32()
                == 0
        {
            e.vcall(actor, 0x190, &args![bound, 0u32, 1u32]);
            e.call(
                ACTOR_QUEUE_EQUIP_OBJECT,
                &args![actor, bound, 1u32, 0u32, 1u32, 0u32, 1u32],
            );
        }
    }
}

// Translated from 0080a700 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080a700 - the exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed (`bounditemeffect.cpp`; cdecl: actor, item change, effect; returns `AL`): hands a
/// displaced item to another bound item effect of the actor. Without an actor or an item
/// that is a `TESBipedModelForm` it answers false. Otherwise it walks the actor's effects
/// (slot 8 of its `MagicTarget` at +0x94) until an empty node; for another
/// `BoundItemEffect` whose associated item is a `TESObjectARMO`, the first biped slot both
/// fill where that effect has no displaced item gets a copy of the item change, and the
/// answer is true.
pub fn fn_0080a700(e: &mut Engine, actor: u32, change: u32, effect: u32) -> bool {
    let biped = if change == 0 {
        0
    } else {
        let form = e.call(WORD_AT_8_0044DDC0, &args![change]).u32();
        cast(e, form, TES_BOUND_OBJECT_TYPE, TES_BIPED_MODEL_FORM_TYPE)
    };
    if actor == 0 || biped == 0 {
        return false;
    }
    let target = if actor == 0 { 0 } else { actor + 0x94 };
    let mut node = e.vcall(target, 8, &[]).u32();
    while node != 0 && !e.call(LIST_NODE_IS_EMPTY, &args![node]).bool() {
        let slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
        let other = e.mem.u32(slot);
        let bound = cast(e, other, ACTIVE_EFFECT_TYPE, BOUND_ITEM_EFFECT_TYPE);
        if bound != 0 && bound != effect {
            let item = e.call(ASSOCIATED_ITEM_006733E0, &args![bound]).u32();
            let armor = cast(e, item, TES_FORM_TYPE, TES_OBJECT_ARMO_TYPE);
            if armor != 0 {
                for index in 0..DISPLACED_ITEM_COUNT {
                    if e.call(
                        TES_BIPED_MODEL_FORM_FILLS_BIPED_SLOT,
                        &args![biped, index, 0u32, 0u32],
                    )
                    .bool()
                        && e.call(
                            TES_BIPED_MODEL_FORM_FILLS_BIPED_SLOT,
                            &args![armor + 0x70, index, 0u32, 0u32],
                        )
                        .bool()
                        && e.mem.u32(displaced_item(Ptr::new(bound), index)) == 0
                    {
                        let copied = copy_item_change(e, change);
                        e.mem
                            .set_u32(displaced_item(Ptr::new(bound), index), copied);
                        return true;
                    }
                }
            }
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    false
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0080_7e10,
            fn_00807e10(Ptr<BoundItemEffect>, u32, u32, u32) -> u32
        ),
        entry!(
            0x0080_7eb0,
            bound_item_effect_scalar_deleting_destructor(Ptr<BoundItemEffect>, u32) -> u32
        ),
        entry!(0x0080_7ee0, fn_00807ee0(Ptr<BoundItemEffect>)),
        entry!(
            0x0080_82c0,
            bound_item_effect_copy(Ptr<BoundItemEffect>) -> u32
        ),
        entry!(0x0080_85f0, fn_008085f0(u32, u32, u32) -> u32),
        entry!(0x0080_8680, bound_item_effect_start(Ptr<BoundItemEffect>)),
        entry!(
            0x0080_8800,
            bound_item_effect_update(Ptr<BoundItemEffect>, u32)
        ),
        entry!(0x0080_8a30, fn_00808a30(Ptr<BoundItemEffect>)),
        entry!(0x0080_91a0, bound_item_effect_finish(Ptr<BoundItemEffect>)),
        entry!(0x0080_99c0, fn_008099c0(Ptr<BoundItemEffect>, u32) -> bool),
        entry!(
            0x0080_9a00,
            bound_item_effect_get_save_size(Ptr<BoundItemEffect>, u32) -> u16
        ),
        entry!(
            0x0080_9bd0,
            bound_item_effect_save_game(Ptr<BoundItemEffect>, u32)
        ),
        entry!(
            0x0080_9e60,
            bound_item_effect_load_game(Ptr<BoundItemEffect>, u32)
        ),
        entry!(
            0x0080_a2d0,
            bound_item_effect_init_load_game(Ptr<BoundItemEffect>, u32)
        ),
        entry!(0x0080_a700, fn_0080a700(u32, u32, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::collections::{HashMap, VecDeque};
    use std::rc::Rc;

    const LOADER: u32 = 0x0600_0000;
    const SAVE_VERSION: u32 = 0x008d_f040;
    const USE_BLOCKS: u32 = 0x0086_2110;
    const SETTING_VALUE: u32 = 0x0040_8d60;
    const POSITION: u32 = 0x0082_5c00;
    const SAVE_BYTES: u32 = 0x0085_79b0;
    const LOAD_BYTES: u32 = 0x0085_79e0;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
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

    /// Doubles answering 0 for each address.
    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            answer(e, *addr, 0);
        }
    }

    /// An object whose vtable slots answer the given words; each slot's double sits at the
    /// slot's own address (`slot(e, object, offset)`).
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x600);
        for (offset, value) in slots {
            e.mem.set_u32(vtable + offset, vtable + offset);
            answer(e, vtable + offset, *value);
        }
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this, vtable);
        this
    }

    /// The double address of `object`'s vtable slot `offset`.
    fn slot(e: &Engine, object: u32, offset: u32) -> u32 {
        e.mem.u32(object) + offset
    }

    /// A form whose `TESModel` at +0x3c (and +0x18) answers `model` from slot +0x14.
    fn model_form(e: &mut Engine, model: u32) -> u32 {
        let form = e.mem.alloc(0x100);
        let model_object = object(e, &[(0x14, model)]);
        let vtable = e.mem.u32(model_object);
        e.mem.set_u32(form + 0x3c, vtable);
        e.mem.set_u32(form + 0x18, vtable);
        form
    }

    /// `__RTDynamicCast` answering `table[(object, target type)]`, else 0.
    fn casts(e: &mut Engine, table: &[(u32, u32, u32)]) {
        let map: HashMap<(u32, u32), u32> = table.iter().map(|(o, t, r)| ((*o, *t), *r)).collect();
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            map.get(&(a[0], a[3])).copied().unwrap_or(0).into_ret()
        });
    }

    fn rig() -> (Engine, Ptr<BoundItemEffect>) {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.set_global(MODEL_LOADER_POINTER, LOADER);
        e.set_global(MINUS_ONE, (-1.0f32).to_bits());
        let game = e.mem.alloc(0x100);
        e.set_global(0x011d_e45c, game);
        let this: Ptr<BoundItemEffect> = e.new_object();
        let vtable = e.mem.alloc(0x60);
        e.mem.set_u32(this.addr(), vtable);
        // Item changes and effects keep their words where the getters read them.
        e.register(WORD_AT_8_0044DDC0, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(WORD_AT_C_0084E3A0, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(MODEL_LENGTH_0048CEE0, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(LIST_NODE_ITEM, |_, a| a[0].into_ret());
        e.register(LIST_NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LIST_NODE_IS_EMPTY, |e, a| {
            u8::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        quiet(
            &mut e,
            &[
                MODEL_LOADER_RELEASE_MODEL,
                MODEL_LOADER_QUEUE_MODEL,
                MODEL_LOADER_LOAD_FILE,
                EXTRA_LISTS_00559450,
                EXTRA_DATA_LIST_CLEAR_SCRIPT_LOCALS,
                ITEM_CHANGE_DELETE_ALL_EXTRA,
                DELETE_004459E0,
                BASE_DESTRUCTOR_0080D930,
                ACTIVE_EFFECT_DISPEL,
                ACTOR_QUEUE_EQUIP_OBJECT,
                ACTOR_QUEUE_UNEQUIP_OBJECT,
                EXTRA_DATA_LIST_SET_CAN_NOT_WEAR,
                EXTRA_DATA_LIST_REMOVE_CANNOT_WEAR,
            ],
        );
        e.call_log = Some(vec![]);
        (e, this)
    }

    /// An item change (12 bytes) of `form`.
    fn change(e: &mut Engine, form: u32) -> u32 {
        let change = e.mem.alloc(0xc);
        e.mem.set_u32(change + 8, form);
        change
    }

    /// The effect's spell (+8, slot +0x18 answering `spell_type`), effect item (+0xc) and a
    /// setting with the model 0x99 from `GetAVEffect`; the costliest effect item answers 0.
    fn spell_setup(e: &mut Engine, this: Ptr<BoundItemEffect>, spell_type: u32) {
        let spell = object(e, &[(0x18, spell_type)]);
        e.mem.set_u32(this.addr() + 8, spell);
        let item = e.mem.alloc(0x10);
        e.mem.set_u32(this.addr() + 0xc, item);
        let setting = model_form(e, 0x99);
        answer(e, MAGIC_ITEM_GET_AV_EFFECT, setting);
        answer(e, EFFECT_ITEM_LIST_GET_COSTLIEST_EFFECT, 0);
    }

    #[test]
    fn constructor_sets_the_vtable_and_clears_the_state() {
        let (mut e, this) = rig();
        e.mem.write(this.addr() + 0x4c, &[0xff; 0x60]);
        answer(&mut e, ASSOCIATED_ITEM_EFFECT_CONSTRUCTOR, 0);
        let result = e.call(0x0080_7e10, &args![this, 1u32, 2u32, 3u32]).u32();
        assert_eq!(result, this.addr());
        assert_eq!(
            calls(&e, ASSOCIATED_ITEM_EFFECT_CONSTRUCTOR),
            vec![vec![this.addr(), 1, 2, 3]]
        );
        assert_eq!(e.mem.u32(this.addr()), BOUND_ITEM_EFFECT_VTABLE);
        assert!(e
            .mem
            .bytes(this.addr() + 0x4c, 0x5d)
            .iter()
            .all(|b| *b == 0));
        assert_eq!(e.mem.u8(this.addr() + 0xa9), 0xff);
    }

    #[test]
    fn scalar_deleting_destructor_deletes_only_with_bit_0() {
        for (flags, deletes) in [(1u32, 1usize), (2, 0)] {
            let (mut e, this) = rig();
            answer(&mut e, OPERATOR_DELETE, 0);
            assert_eq!(e.call(0x0080_7eb0, &args![this, flags]).u32(), this.addr());
            assert_eq!(calls(&e, BASE_DESTRUCTOR_0080D930), vec![vec![this.addr()]]);
            assert_eq!(calls(&e, OPERATOR_DELETE).len(), deletes);
        }
    }

    #[test]
    fn destructor_releases_cached_models_and_item_changes() {
        let (mut e, this) = rig();
        let weapon = model_form(&mut e, 0x77);
        let weapon_change = change(&mut e, 0x5000);
        e.set(this, BoundItemEffect::pDisplacedWeapon, weapon_change);
        e.set(this, BoundItemEffect::bWeaponCached, 1);
        let shirt = change(&mut e, 0x6000);
        e.mem.set_u32(displaced_item(this, 3), shirt);
        e.set(this, BoundItemEffect::bWornCached, 1);
        let target = object(&mut e, &[(4, 0x9000)]);
        answer(&mut e, EFFECT_TARGET_0059BB30, target);
        answer(&mut e, BASE_FORM_007AF430, 0x9100);
        answer(&mut e, TES_ACTOR_BASE_GET_SEX, 1);
        answer(&mut e, TES_BIPED_MODEL_FORM_GET_BIPED_MODEL, 0x88);
        casts(
            &mut e,
            &[
                (0x5000, TES_OBJECT_WEAP_TYPE, weapon),
                (0x9100, TES_NPC_TYPE, 0x9200),
                (0x6000, TES_BIPED_MODEL_FORM_TYPE, 0x6100),
            ],
        );
        e.call(0x0080_7ee0, &args![this]);
        assert_eq!(
            calls(&e, MODEL_LOADER_RELEASE_MODEL),
            vec![vec![LOADER, 0x77, 0, 1], vec![LOADER, 0x88, 0, 1]]
        );
        assert_eq!(
            calls(&e, TES_BIPED_MODEL_FORM_GET_BIPED_MODEL),
            vec![vec![0x6100, 1]]
        );
        assert_eq!(
            calls(&e, ITEM_CHANGE_DELETE_ALL_EXTRA),
            vec![vec![weapon_change], vec![shirt]]
        );
        assert_eq!(
            calls(&e, DELETE_004459E0),
            vec![vec![weapon_change, 1], vec![shirt, 1]]
        );
        assert_eq!(calls(&e, BASE_DESTRUCTOR_0080D930), vec![vec![this.addr()]]);
        assert_eq!(e.mem.u32(this.addr()), BOUND_ITEM_EFFECT_VTABLE);
    }

    #[test]
    fn copy_copies_items_and_queues_the_cached_models() {
        let (mut e, this) = rig();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        answer(&mut e, ASSOCIATED_ITEM_EFFECT_CONSTRUCTOR, 0);
        answer(&mut e, ASSOCIATED_ITEM_EFFECT_COPY_DATA, 0);
        e.register(ITEM_CHANGE_COPY_CONSTRUCTOR, |e, a| {
            let form = e.mem.u32(a[1] + 8);
            e.mem.set_u32(a[0] + 8, form);
            a[0].into_ret()
        });
        let weapon = model_form(&mut e, 0x77);
        let weapon_change = change(&mut e, 0x5000);
        e.set(this, BoundItemEffect::pDisplacedWeapon, weapon_change);
        e.set(this, BoundItemEffect::bWeaponCached, 1);
        e.set(this, BoundItemEffect::bItemSummoned, 1);
        let shirt = change(&mut e, 0x6000);
        e.mem.set_u32(displaced_item(this, 5), shirt);
        e.set(this, BoundItemEffect::bWornCached, 1);
        let target = object(&mut e, &[(4, 0)]);
        answer(&mut e, EFFECT_TARGET_0059BB30, target);
        answer(&mut e, TES_BIPED_MODEL_FORM_GET_BIPED_MODEL, 0x88);
        casts(
            &mut e,
            &[
                (0x5000, TES_OBJECT_WEAP_TYPE, weapon),
                (0x6000, TES_BIPED_MODEL_FORM_TYPE, 0x6100),
            ],
        );
        let copy = e.call(0x0080_82c0, &args![this]).u32();
        let copy: Ptr<BoundItemEffect> = Ptr::new(copy);
        assert_eq!(
            calls(&e, ASSOCIATED_ITEM_EFFECT_COPY_DATA),
            vec![vec![this.addr(), copy.addr()]]
        );
        let weapon_copy = e.get(copy, BoundItemEffect::pDisplacedWeapon);
        assert_ne!(weapon_copy, weapon_change);
        assert_eq!(e.mem.u32(weapon_copy + 8), 0x5000);
        assert_eq!(e.mem.u32(e.mem.u32(displaced_item(copy, 5)) + 8), 0x6000);
        assert_eq!(e.get(copy, BoundItemEffect::bWeaponCached), 1);
        assert_eq!(e.get(copy, BoundItemEffect::bWornCached), 1);
        assert_eq!(e.get(copy, BoundItemEffect::bItemSummoned), 1);
        assert_eq!(
            calls(&e, MODEL_LOADER_QUEUE_MODEL),
            vec![
                vec![LOADER, 0x77, 0, 0, 0, 1, 1, 0],
                vec![LOADER, 0x88, 0, 0, 0, 1, 1, 0]
            ]
        );
        // The sex stays 0 without a reference.
        assert_eq!(
            calls(&e, TES_BIPED_MODEL_FORM_GET_BIPED_MODEL),
            vec![vec![0x6100, 0]]
        );
    }

    #[test]
    fn fn_008085f0_allocates_and_constructs() {
        let (mut e, _) = rig();
        let block = e.mem.alloc(0xac);
        answer(&mut e, OPERATOR_NEW, block);
        answer(&mut e, ASSOCIATED_ITEM_EFFECT_CONSTRUCTOR, 0);
        assert_eq!(e.call(0x0080_85f0, &args![1u32, 2u32, 3u32]).u32(), block);
        assert_eq!(
            calls(&e, ASSOCIATED_ITEM_EFFECT_CONSTRUCTOR),
            vec![vec![block, 1, 2, 3]]
        );
        answer(&mut e, OPERATOR_NEW, 0);
        assert_eq!(e.call(0x0080_85f0, &args![1u32, 2u32, 3u32]).u32(), 0);
    }

    #[test]
    fn start_dispels_without_an_actor_or_item() {
        let (mut e, this) = rig();
        answer(&mut e, EFFECT_TARGET_0059BB30, 0);
        answer(&mut e, ASSOCIATED_ITEM_006733E0, 0x4000);
        e.call(0x0080_8680, &args![this]);
        assert_eq!(calls(&e, ACTIVE_EFFECT_DISPEL), vec![vec![this.addr(), 0]]);
    }

    #[test]
    fn start_loads_the_setting_model_and_reads_its_time() {
        for spell_type in [1u32, 4] {
            let (mut e, this) = rig();
            answer(&mut e, EFFECT_TARGET_0059BB30, 0x7000);
            answer(&mut e, MAGIC_TARGET_GET_TARGET_AS_ACTOR, 0x7100);
            answer(&mut e, ASSOCIATED_ITEM_006733E0, 0x4000);
            spell_setup(&mut e, this, spell_type);
            if spell_type == 4 {
                // The effect item is the costliest: the model stays loaded.
                let item = e.mem.u32(this.addr() + 0xc);
                answer(&mut e, EFFECT_ITEM_LIST_GET_COSTLIEST_EFFECT, item);
            }
            answer(&mut e, MODEL_LOADER_LOAD_FILE, 0x5555);
            e.register(MODEL_TIME_004B6220, |_, _| Ret {
                st0: 2.5,
                ..Ret::default()
            });
            e.mem.set_u32(this.addr() + 0xa0, 0x1234);
            e.call(0x0080_8680, &args![this]);
            assert_eq!(
                calls(&e, MODEL_LOADER_LOAD_FILE),
                vec![vec![LOADER, 0x99, 0, 1, 0, 0, 0]]
            );
            if spell_type == 4 {
                assert_eq!(e.mem.u32(this.addr() + 0xa0), 0);
                assert!(calls(&e, MODEL_LOADER_RELEASE_MODEL).is_empty());
            } else {
                assert_eq!(
                    calls(&e, MODEL_TIME_004B6220),
                    vec![vec![0x5555, SPECIAL_IDLE_HIT_EFFECT_STRING, HIT_STRING]]
                );
                assert_eq!(e.mem.f32(this.addr() + 0xa0), 2.5);
                assert_eq!(
                    calls(&e, MODEL_LOADER_RELEASE_MODEL),
                    vec![vec![LOADER, 0x99, 0, 1]]
                );
            }
        }
    }

    #[test]
    fn update_waits_for_the_appear_time_then_summons() {
        for (elapsed, appear, summons) in [
            (1.0f32, 2.0f32, false),
            (2.0, 2.0, true),
            (1.0, f32::NAN, false),
        ] {
            let (mut e, this) = rig();
            e.mem.set_f32(this.addr() + 4, elapsed);
            e.mem.set_f32(this.addr() + 0xa0, appear);
            // The summon finds neither a weapon nor an armor: it dispels.
            answer(&mut e, EFFECT_TARGET_0059BB30, 0);
            answer(&mut e, ASSOCIATED_ITEM_006733E0, 0);
            casts(&mut e, &[]);
            e.call(0x0080_8800, &args![this, 0u32]);
            assert_eq!(
                e.get(this, BoundItemEffect::bItemSummoned),
                u8::from(summons)
            );
            assert_eq!(calls(&e, ACTIVE_EFFECT_DISPEL).len(), usize::from(summons));
        }
    }

    #[test]
    fn update_keeps_the_weapon_set_up_and_dispels_for_the_player() {
        let (mut e, this) = rig();
        e.set(this, BoundItemEffect::bItemSummoned, 1);
        e.set(this, BoundItemEffect::bSetupItem, 1);
        let actor = 0x0700_0000u32;
        e.set_global(PLAYER_POINTER, actor);
        answer(&mut e, EFFECT_TARGET_0059BB30, 0x7000);
        answer(&mut e, MAGIC_TARGET_GET_TARGET_AS_ACTOR, actor);
        answer(&mut e, ASSOCIATED_ITEM_006733E0, 0x4000);
        casts(&mut e, &[(0x4000, TES_OBJECT_WEAP_TYPE, 0x4100)]);
        let held = change(&mut e, 0x4100);
        let process = object(&mut e, &[(0x530, 0), (0x148, held), (0x454, 0)]);
        answer(&mut e, ACTOR_OBJECT_008D8520, process);
        let lists = e.mem.alloc(8);
        e.mem.set_u32(lists, 0x3300);
        answer(&mut e, EXTRA_LISTS_00559450, lists);
        answer(&mut e, ACTOR_TEST_008A6970, 0);
        e.call(0x0080_8800, &args![this, 0u32]);
        assert_eq!(
            calls(&e, slot(&e, process, 0x530)),
            vec![vec![process, 0x4100]]
        );
        assert_eq!(
            calls(&e, EXTRA_DATA_LIST_SET_CAN_NOT_WEAR),
            vec![vec![0x3300, 1]]
        );
        assert_eq!(e.get(this, BoundItemEffect::bSetupItem), 0);
        assert_eq!(calls(&e, ACTIVE_EFFECT_DISPEL), vec![vec![this.addr(), 0]]);
    }

    #[test]
    fn update_sets_up_a_worn_armor() {
        let (mut e, this) = rig();
        e.set(this, BoundItemEffect::bItemSummoned, 1);
        e.set(this, BoundItemEffect::bSetupItem, 1);
        answer(&mut e, EFFECT_TARGET_0059BB30, 0x7000);
        answer(&mut e, MAGIC_TARGET_GET_TARGET_AS_ACTOR, 0x7100);
        answer(&mut e, ASSOCIATED_ITEM_006733E0, 0x4000);
        casts(&mut e, &[(0x4000, TES_OBJECT_ARMO_TYPE, 0x4200)]);
        answer(&mut e, ACTOR_OBJECT_008D8520, 0);
        answer(&mut e, INVENTORY_CHANGES_GET, 0x8000);
        answer(&mut e, INVENTORY_CHANGES_WEARING_OBJECT, 0x3400);
        e.call(0x0080_8800, &args![this, 0u32]);
        assert_eq!(calls(&e, INVENTORY_CHANGES_GET), vec![vec![0x7100]]);
        assert_eq!(
            calls(&e, INVENTORY_CHANGES_WEARING_OBJECT),
            vec![vec![0x8000, 0x4200, 1]]
        );
        assert_eq!(
            calls(&e, EXTRA_DATA_LIST_SET_CAN_NOT_WEAR),
            vec![vec![0x3400, 1]]
        );
        assert_eq!(e.get(this, BoundItemEffect::bSetupItem), 0);
    }

    #[test]
    fn summon_swaps_the_held_weapon() {
        let (mut e, this) = rig();
        let actor = object(&mut e, &[(0x190, 0)]);
        answer(&mut e, EFFECT_TARGET_0059BB30, 0x7000);
        answer(&mut e, MAGIC_TARGET_GET_TARGET_AS_ACTOR, actor);
        answer(&mut e, ASSOCIATED_ITEM_006733E0, 0x4000);
        let old_weapon = model_form(&mut e, 0x66);
        casts(
            &mut e,
            &[
                (0x4000, TES_OBJECT_WEAP_TYPE, 0x4100),
                (0x5000, TES_OBJECT_WEAP_TYPE, old_weapon),
            ],
        );
        // The held weapon's change: its list node's first extra list is 0x3300.
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, 0x3300);
        let held = change(&mut e, 0x5000);
        e.mem.set_u32(held, node);
        let process = object(&mut e, &[(0x148, held), (0x450, 0), (0x530, 0)]);
        answer(&mut e, ACTOR_OBJECT_008D8520, process);
        answer(&mut e, BASE_EXTRA_LIST_ITEMS_IN_LIST, 3);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ITEM_CHANGE_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            a[0].into_ret()
        });
        e.register(EXTRA_DATA_LIST_CONSTRUCTOR, |_, a| a[0].into_ret());
        quiet(
            &mut e,
            &[EXTRA_DATA_LIST_DUPLICATE, LIST_INSERT_FRONT_005AE3D0],
        );
        e.call(0x0080_8a30, &args![this]);
        assert_eq!(
            calls(&e, MODEL_LOADER_LOAD_FILE),
            vec![vec![LOADER, 0x66, 0, 1, 0, 0, 0]]
        );
        assert_eq!(e.get(this, BoundItemEffect::bWeaponCached), 1);
        assert_eq!(
            calls(&e, ACTOR_QUEUE_UNEQUIP_OBJECT),
            vec![vec![actor, 0x5000, 1, 0x3300, 0, 0, 1]]
        );
        let remembered = e.get(this, BoundItemEffect::pDisplacedWeapon);
        assert_eq!(
            calls(&e, ITEM_CHANGE_CONSTRUCTOR),
            vec![vec![remembered, 0x5000, 0]]
        );
        assert_eq!(calls(&e, EXTRA_DATA_LIST_DUPLICATE)[0][1], 0x3300);
        assert_eq!(calls(&e, LIST_INSERT_FRONT_005AE3D0)[0][0], 0x1111);
        assert_eq!(
            calls(&e, slot(&e, actor, 0x190)),
            vec![vec![actor, 0x4100, 0, 1]]
        );
        // The process still holds the old weapon: the new one is equipped.
        assert_eq!(
            calls(&e, ACTOR_QUEUE_EQUIP_OBJECT),
            vec![vec![actor, 0x4100, 1, 0, 1, 1, 1]]
        );
        // No extra list on the held change: set up later.
        assert_eq!(e.get(this, BoundItemEffect::bSetupItem), 1);
        assert_eq!(calls(&e, slot(&e, process, 0x450)), vec![vec![process, 1]]);
        assert_eq!(
            calls(&e, slot(&e, process, 0x530)),
            vec![vec![process, 0x4100]]
        );
    }

    #[test]
    fn summon_remembers_the_worn_items_an_armor_displaces() {
        let (mut e, this) = rig();
        let actor = object(&mut e, &[(0x190, 0)]);
        answer(&mut e, EFFECT_TARGET_0059BB30, 0x7000);
        answer(&mut e, MAGIC_TARGET_GET_TARGET_AS_ACTOR, actor);
        answer(&mut e, ASSOCIATED_ITEM_006733E0, 0x4000);
        answer(&mut e, BASE_FORM_007AF430, 0);
        casts(
            &mut e,
            &[
                (0x4000, TES_OBJECT_ARMO_TYPE, 0x4200),
                (0x6000, TES_BIPED_MODEL_FORM_TYPE, 0x6100),
            ],
        );
        // Slot 2 is filled and worn by a shirt whose list node has no extra list.
        e.register(TES_BIPED_MODEL_FORM_FILLS_BIPED_SLOT, |_, a| {
            u8::from(a[1] == 2).into_ret()
        });
        answer(&mut e, EXTRA_LIST_005D43C0, 0x3000);
        answer(&mut e, EXTRA_DATA_LIST_GET_CONTAINER_CHANGES, 0x3100);
        let node = e.mem.alloc(8);
        let shirt = change(&mut e, 0x6000);
        e.mem.set_u32(shirt, node);
        answer(&mut e, INVENTORY_CHANGES_GET_WORN_ITEM, shirt);
        answer(&mut e, TES_BIPED_MODEL_FORM_GET_BIPED_MODEL, 0x88);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ITEM_CHANGE_COPY_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(EXTRA_DATA_LIST_CONSTRUCTOR, |_, a| a[0].into_ret());
        answer(&mut e, INVENTORY_CHANGES_GET, 0x8000);
        answer(&mut e, INVENTORY_CHANGES_WEARING_OBJECT, 0x3400);
        e.call(0x0080_8a30, &args![this]);
        assert_eq!(
            calls(&e, INVENTORY_CHANGES_GET_WORN_ITEM),
            vec![vec![0x3100, 2, 0]]
        );
        assert_eq!(
            calls(&e, MODEL_LOADER_LOAD_FILE),
            vec![vec![LOADER, 0x88, 0, 1, 0, 0, 0]]
        );
        assert_eq!(e.get(this, BoundItemEffect::bWornCached), 1);
        assert_ne!(e.mem.u32(displaced_item(this, 2)), 0);
        assert_eq!(calls(&e, DELETE_004459E0), vec![vec![shirt, 1]]);
        let added = calls(&e, slot(&e, actor, 0x190));
        assert_eq!(added[0][1], 0x4200);
        assert_eq!(added[0][3], 1);
        assert_eq!(
            calls(&e, ACTOR_QUEUE_EQUIP_OBJECT),
            vec![vec![actor, 0x4200, 1, 0, 1, 0, 1]]
        );
        assert_eq!(
            calls(&e, EXTRA_DATA_LIST_SET_CAN_NOT_WEAR),
            vec![vec![0x3400, 1]]
        );
    }

    #[test]
    fn finish_gives_the_displaced_weapon_back() {
        let (mut e, this) = rig();
        e.set(this, BoundItemEffect::bItemSummoned, 1);
        let actor = object(&mut e, &[(0x17c, 0), (0x1d0, 1)]);
        answer(&mut e, EFFECT_TARGET_0059BB30, 0x7000);
        answer(&mut e, MAGIC_TARGET_GET_TARGET_AS_ACTOR, actor);
        answer(&mut e, ASSOCIATED_ITEM_006733E0, 0x4000);
        casts(&mut e, &[(0x4000, TES_OBJECT_WEAP_TYPE, 0x4100)]);
        answer(&mut e, INVENTORY_CHANGES_GET, 0x8000);
        answer(&mut e, INVENTORY_CHANGES_WEARING_OBJECT, 0x3400);
        let old = change(&mut e, 0x5000);
        e.set(this, BoundItemEffect::pDisplacedWeapon, old);
        answer(&mut e, ACTOR_GET_ITEM_COUNT, 1);
        let process = object(&mut e, &[(0x450, 0)]);
        answer(&mut e, ACTOR_OBJECT_008D8520, process);
        answer(&mut e, PROCESS_TEST_0045CD60, 1);
        quiet(&mut e, &[FINISH_HELPER_007037C0, PLAY_SOUND_BY_EDITOR_NAME]);
        e.call(0x0080_91a0, &args![this]);
        assert_eq!(
            calls(&e, EXTRA_DATA_LIST_REMOVE_CANNOT_WEAR),
            vec![vec![0x3400]]
        );
        assert_eq!(
            calls(&e, ACTOR_QUEUE_UNEQUIP_OBJECT),
            vec![vec![actor, 0x4100, 1, 0, 0, 0, 1]]
        );
        assert_eq!(
            calls(&e, slot(&e, actor, 0x17c)),
            vec![vec![actor, 0x4100, 0, 1, 0, 0, 0, 0, 0, 1, 0]]
        );
        assert_eq!(
            calls(&e, ACTOR_QUEUE_EQUIP_OBJECT),
            vec![vec![actor, 0x5000, 1, 0, 1, 0, 1]]
        );
        assert_eq!(calls(&e, DELETE_004459E0), vec![vec![old, 1]]);
        assert_eq!(e.get(this, BoundItemEffect::pDisplacedWeapon), 0);
        assert_eq!(calls(&e, slot(&e, process, 0x450)), vec![vec![process, 1]]);
        // 0045cd60 set: no spell model and no sound.
        assert!(calls(&e, PLAY_SOUND_BY_EDITOR_NAME).is_empty());
        assert_eq!(
            calls(&e, FINISH_HELPER_007037C0),
            vec![vec![this.addr(), 0]]
        );
        assert_eq!(e.get(this, BoundItemEffect::bItemSummoned), 0);
    }

    #[test]
    fn finish_releases_armor_slots_and_makes_the_hit_effect() {
        let (mut e, this) = rig();
        e.set(this, BoundItemEffect::bItemSummoned, 1);
        e.set(this, BoundItemEffect::bSpellModelCached, 1);
        let actor = object(&mut e, &[(0x17c, 0), (0x1d0, 1)]);
        answer(&mut e, EFFECT_TARGET_0059BB30, 0x7000);
        answer(&mut e, MAGIC_TARGET_GET_TARGET_AS_ACTOR, actor);
        answer(&mut e, ASSOCIATED_ITEM_006733E0, 0x4000);
        answer(&mut e, BASE_FORM_007AF430, 0);
        casts(&mut e, &[(0x4000, TES_OBJECT_ARMO_TYPE, 0x4200)]);
        answer(&mut e, INVENTORY_CHANGES_GET, 0x8000);
        answer(&mut e, INVENTORY_CHANGES_WEARING_OBJECT, 0);
        e.register(TES_BIPED_MODEL_FORM_FILLS_BIPED_SLOT, |_, a| {
            u8::from(a[1] == 4).into_ret()
        });
        let shirt = change(&mut e, 0x6000);
        e.mem.set_u32(displaced_item(this, 4), shirt);
        answer(&mut e, ACTOR_GET_ITEM_COUNT, 0);
        let process = object(&mut e, &[]);
        answer(&mut e, ACTOR_OBJECT_008D8520, process);
        answer(&mut e, PROCESS_TEST_0045CD60, 0);
        spell_setup(&mut e, this, 1);
        let item = e.mem.u32(this.addr() + 0xc);
        answer(&mut e, EFFECT_ITEM_LIST_GET_COSTLIEST_EFFECT, item);
        let setting = model_form(&mut e, 0x99);
        answer(&mut e, EFFECT_SETTING_007DF1D0, setting);
        answer(&mut e, ALLOCATE_00AA13E0, 0x9900);
        let hit = object(&mut e, &[(0xc4, 1)]);
        answer(&mut e, MAGIC_MODEL_HIT_EFFECT_CONSTRUCTOR, hit);
        quiet(
            &mut e,
            &[
                PROCESS_LISTS_ADD_TEMP_EFFECT,
                PLAY_SOUND_BY_EDITOR_NAME,
                SOUND_HANDLE_DESTROY_00483710,
                FINISH_HELPER_007037C0,
            ],
        );
        e.call(0x0080_91a0, &args![this]);
        assert_eq!(calls(&e, DELETE_004459E0), vec![vec![shirt, 1]]);
        assert_eq!(e.mem.u32(displaced_item(this, 4)), 0);
        assert_eq!(e.get(this, BoundItemEffect::bWornCached), 0);
        assert_eq!(
            calls(&e, MAGIC_MODEL_HIT_EFFECT_CONSTRUCTOR),
            vec![vec![0x9900, actor, 0x99, 0xbf80_0000]]
        );
        assert_eq!(
            calls(&e, PROCESS_LISTS_ADD_TEMP_EFFECT),
            vec![vec![PROCESS_LISTS, hit]]
        );
        assert_eq!(
            calls(&e, MODEL_LOADER_RELEASE_MODEL),
            vec![vec![LOADER, 0x99, 0, 1]]
        );
        assert_eq!(e.get(this, BoundItemEffect::bSpellModelCached), 0);
        let sound = calls(&e, PLAY_SOUND_BY_EDITOR_NAME);
        assert_eq!(sound.len(), 1);
        assert_eq!(sound[0][0], actor);
        assert_eq!(
            sound[0][2..],
            [ITM_BOUND_DISAPPEAR_STRING, 0, 0x4000_0102, 1]
        );
        assert_eq!(
            calls(&e, SOUND_HANDLE_DESTROY_00483710),
            vec![vec![sound[0][1]]]
        );
    }

    #[test]
    fn fn_008099c0_accepts_only_characters() {
        let (mut e, this) = rig();
        casts(&mut e, &[(0x1000, CHARACTER_TYPE, 0x0f6c)]);
        assert!(e.call(0x0080_99c0, &args![this, 0x1000u32]).bool());
        assert!(!e.call(0x0080_99c0, &args![this, 0x2000u32]).bool());
        assert_eq!(
            calls(&e, RT_DYNAMIC_CAST)[0],
            vec![0x1000, 0, MAGIC_TARGET_TYPE, CHARACTER_TYPE, 0]
        );
    }

    #[test]
    fn save_size_counts_the_displaced_items() {
        for (version, blocks, expected) in [
            (0x6bu8, 1u32, 10 + 6 + 1 + 7 + 20 + 3 + 5 + 1),
            (0x40, 0, 10 + 1 + 7 + 20 + 3),
        ] {
            let (mut e, this) = rig();
            answer(&mut e, SAVE_VERSION, u32::from(version));
            answer(&mut e, BASE_SAVE_SIZE_00806DA0, 0x0001_000a);
            answer(&mut e, USE_BLOCKS, blocks);
            let value = e.mem.alloc(4);
            answer(&mut e, SETTING_VALUE, value);
            e.register(ITEM_CHANGE_SAVE_SIZE_004BE5A0, |e, a| {
                e.mem.u32(a[0]).into_ret()
            });
            let weapon = e.mem.alloc(0xc);
            e.mem.set_u32(weapon, 7);
            e.set(this, BoundItemEffect::pDisplacedWeapon, weapon);
            let shirt = e.mem.alloc(0xc);
            e.mem.set_u32(shirt, 3);
            e.mem.set_u32(displaced_item(this, 9), shirt);
            let size = e.call(0x0080_9a00, &args![this, 0x77u32]).u16();
            assert_eq!(size, expected, "version {version:#x}");
            assert_eq!(
                calls(&e, BASE_SAVE_SIZE_00806DA0),
                vec![vec![this.addr(), 0x77]]
            );
        }
    }

    #[test]
    fn save_game_writes_presence_bytes_and_the_state() {
        let (mut e, this) = rig();
        answer(&mut e, ASSOCIATED_ITEM_EFFECT_SAVE_GAME, 0);
        answer(&mut e, SAVE_VERSION, 0x6b);
        answer(&mut e, USE_BLOCKS, 0);
        let value = e.mem.alloc(4);
        answer(&mut e, SETTING_VALUE, value);
        answer(&mut e, POSITION, 0);
        let writes = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&writes);
        e.register_double(SAVE_BYTES, move |e, a| {
            log.borrow_mut().push(e.mem.bytes(a[1], a[2]));
            Ret::default()
        });
        answer(&mut e, ITEM_CHANGE_SAVE_004BE6F0, 0);
        let shirt = e.mem.alloc(0xc);
        e.mem.set_u32(displaced_item(this, 0), shirt);
        e.mem.set_f32(this.addr() + 0xa0, 1.5);
        e.set(this, BoundItemEffect::bItemSummoned, 1);
        e.set(this, BoundItemEffect::bSetupItem, 1);
        e.call(0x0080_9bd0, &args![this, 0x77u32]);
        let writes = writes.borrow();
        assert_eq!(writes.len(), 1 + 20 + 3);
        assert_eq!(writes[0], vec![0]);
        assert_eq!(writes[1], vec![1]);
        assert_eq!(writes[2], vec![0]);
        assert_eq!(writes[21], 1.5f32.to_le_bytes().to_vec());
        assert_eq!(writes[22], vec![1]);
        assert_eq!(writes[23], vec![1]);
        assert_eq!(calls(&e, ITEM_CHANGE_SAVE_004BE6F0), vec![vec![shirt]]);
        assert_eq!(
            calls(&e, ASSOCIATED_ITEM_EFFECT_SAVE_GAME),
            vec![vec![this.addr(), 0x77]]
        );
    }

    #[test]
    fn load_game_reads_the_items_and_drops_formless_ones() {
        let (mut e, this) = rig();
        answer(&mut e, ASSOCIATED_ITEM_EFFECT_LOAD_GAME, 0);
        answer(&mut e, SAVE_VERSION, 0x41);
        answer(&mut e, USE_BLOCKS, 0);
        // Weapon present; item 0 present with a form, item 1 present without one.
        let mut data = vec![1u8, 1, 1];
        data.extend([0u8; 18]);
        data.extend(2.0f32.to_le_bytes());
        data.push(0);
        let stream = Rc::new(RefCell::new(data.into_iter().collect::<VecDeque<u8>>()));
        e.register_double(LOAD_BYTES, move |e, a| {
            for i in 0..a[2] {
                let byte = stream.borrow_mut().pop_front().unwrap();
                e.mem.set_u8(a[1] + i, byte);
            }
            Ret::default()
        });
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ITEM_CHANGE_EMPTY_CONSTRUCTOR, |_, a| a[0].into_ret());
        let loads = Rc::new(Cell::new(0u32));
        let count = Rc::clone(&loads);
        e.register_double(ITEM_CHANGE_LOAD_GAME, move |e, a| {
            count.set(count.get() + 1);
            // The third change loaded (item 1) has no form.
            if count.get() != 3 {
                e.mem.set_u32(a[0] + 8, 0x6000);
            }
            Ret::default()
        });
        e.call(0x0080_9e60, &args![this, 0x77u32]);
        assert_eq!(loads.get(), 3);
        assert_ne!(e.get(this, BoundItemEffect::pDisplacedWeapon), 0);
        assert_ne!(e.mem.u32(displaced_item(this, 0)), 0);
        assert_eq!(e.mem.u32(displaced_item(this, 1)), 0);
        assert_eq!(calls(&e, DELETE_004459E0).len(), 1);
        assert_eq!(e.mem.f32(this.addr() + 0xa0), 2.0);
        // Set to 1, then read from the save (0).
        assert_eq!(e.get(this, BoundItemEffect::bItemSummoned), 0);
    }

    #[test]
    fn init_load_game_requeues_models_and_unequips_displaced_items() {
        let (mut e, this) = rig();
        answer(&mut e, ASSOCIATED_ITEM_EFFECT_INIT_LOAD_GAME, 0);
        answer(&mut e, SAVE_VERSION, 0x60);
        e.set(this, BoundItemEffect::bItemSummoned, 1);
        let actor = object(&mut e, &[(0x190, 0)]);
        answer(&mut e, EFFECT_TARGET_0059BB30, 0x7000);
        answer(&mut e, MAGIC_TARGET_GET_TARGET_AS_ACTOR, actor);
        answer(&mut e, BASE_FORM_007AF430, 0);
        let weapon = model_form(&mut e, 0x77);
        casts(
            &mut e,
            &[
                (0x5000, TES_OBJECT_WEAP_TYPE, weapon),
                (0x4000, TES_BOUND_OBJECT_TYPE, 0x4300),
            ],
        );
        let old = change(&mut e, 0x5000);
        e.set(this, BoundItemEffect::pDisplacedWeapon, old);
        answer(&mut e, ITEM_CHANGE_INIT_004BED00, 0);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ITEM_CHANGE_COPY_CONSTRUCTOR, |e, a| {
            let form = e.mem.u32(a[1] + 8);
            e.mem.set_u32(a[0] + 8, form);
            a[0].into_ret()
        });
        answer(&mut e, EXTRA_LIST_005D43C0, 0x3000);
        answer(&mut e, EXTRA_DATA_LIST_GET_CONTAINER_CHANGES, 0x3100);
        answer(&mut e, ASSOCIATED_ITEM_006733E0, 0x4000);
        answer(&mut e, INVENTORY_CHANGES_GET_OBJECT_COUNT, 0);
        e.call(0x0080_a2d0, &args![this, 0x77u32]);
        assert_eq!(
            calls(&e, MODEL_LOADER_QUEUE_MODEL),
            vec![vec![LOADER, 0x77, 0, 0, 0, 1, 0, 0]]
        );
        assert_eq!(e.get(this, BoundItemEffect::bWeaponCached), 1);
        assert_eq!(calls(&e, ITEM_CHANGE_INIT_004BED00), vec![vec![old]]);
        assert_ne!(e.get(this, BoundItemEffect::pDisplacedWeapon), old);
        assert_eq!(calls(&e, DELETE_004459E0), vec![vec![old, 1]]);
        assert_eq!(
            calls(&e, ACTOR_QUEUE_UNEQUIP_OBJECT),
            vec![vec![actor, 0x5000, 1, 0, 0, 0, 1]]
        );
        assert_eq!(
            calls(&e, slot(&e, actor, 0x190)),
            vec![vec![actor, 0x4300, 0, 1]]
        );
        assert_eq!(
            calls(&e, ACTOR_QUEUE_EQUIP_OBJECT),
            vec![vec![actor, 0x4300, 1, 0, 1, 0, 1]]
        );
    }

    #[test]
    fn fn_0080a700_hands_the_item_to_another_bound_armor() {
        let (mut e, this) = rig();
        // The actor's magic target at +0x94: its slot 8 gives the effect list.
        let actor = e.mem.alloc(0x200);
        let other: Ptr<BoundItemEffect> = e.new_object();
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, other.addr());
        let target_vtable = e.mem.alloc(0x10);
        e.mem.set_u32(actor + 0x94, target_vtable);
        e.mem.set_u32(target_vtable + 8, target_vtable + 8);
        answer(&mut e, target_vtable + 8, node);
        e.register(ASSOCIATED_ITEM_006733E0, |_, a| (a[0] + 1).into_ret());
        casts(
            &mut e,
            &[
                (0x6000, TES_BIPED_MODEL_FORM_TYPE, 0x6100),
                (other.addr(), BOUND_ITEM_EFFECT_TYPE, other.addr()),
                (other.addr() + 1, TES_OBJECT_ARMO_TYPE, 0x4200),
            ],
        );
        e.register(TES_BIPED_MODEL_FORM_FILLS_BIPED_SLOT, |_, a| {
            u8::from(a[1] == 6).into_ret()
        });
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ITEM_CHANGE_COPY_CONSTRUCTOR, |_, a| a[0].into_ret());
        let shirt = change(&mut e, 0x6000);
        assert!(e.call(0x0080_a700, &args![actor, shirt, this]).bool());
        assert_ne!(e.mem.u32(displaced_item(other, 6)), 0);
        assert_eq!(calls(&e, ITEM_CHANGE_COPY_CONSTRUCTOR)[0][1], shirt);
        // The effect itself is skipped, and without an item the answer is false.
        assert!(!e.call(0x0080_a700, &args![actor, shirt, other]).bool());
        assert!(!e.call(0x0080_a700, &args![actor, 0u32, this]).bool());
    }
}
