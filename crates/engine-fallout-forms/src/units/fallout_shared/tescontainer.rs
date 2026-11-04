//! `fallout shared/tescontainer.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A `TESContainer` keeps its items in a `BSSimpleList<ContainerObject *>`
//! at +4. The code walks it only through small out-of-line helpers, which
//! the translations call by address:
//! - `00717e50`: the address of the field at +4 (on a `TESContainer`, its
//!   list's first node),
//! - `006815c0`: its own `this` (on a node, the address of the node's item),
//! - `00726070`: the word at +4 (on a node, the next node),
//! - `008256d0`: whether a node's item and next are both 0,
//! - `005ae3d0` (add an item), `00905330` (remove an item), `0063f7b0`
//!   (remove the first item) and `005ae380` (count the non-null items).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleList;

/// `__RTDynamicCast` (`00ec43fb`, `cdecl`: object, vftable offset, source
/// type, target type, is-reference).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors (`.?AV<Class>@@` at descriptor + 8, read from the
/// exe).
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_SCRIPTABLE_FORM: u32 = 0x0118_3254;
const TYPE_TES_BOUND_OBJECT: u32 = 0x0118_3108;
const TYPE_BASE_FORM_COMPONENT: u32 = 0x0118_3040;
const TYPE_TES_CONTAINER: u32 = 0x0118_6c58;
const TYPE_TES_FULL_NAME: u32 = 0x0118_3158;
const TYPE_TES_OBJECT_WEAP: u32 = 0x0118_3998;
const TYPE_TES_OBJECT_ARMO: u32 = 0x0118_3a34;
const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;
const TYPE_ACTOR: u32 = 0x0118_46d4;

/// `00726070` (`ECX` = object): the word at `+4`; on a
/// `TESScriptableForm`, its script; on a list node, the next node.
const WORD_AT_4: u32 = 0x0072_6070;
/// `00717e50` (`ECX` = object): the address `this + 4`.
const ADDRESS_OF_4: u32 = 0x0071_7e50;
/// `006815c0` (`ECX` = object): returns `this`.
const SAME_ADDRESS: u32 = 0x0068_15c0;
/// `008256d0` (`ECX` = node): true when the node's item and next are 0.
const NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// `005ae3d0` (`ECX` = first node, pointer to the item): adds the item.
const LIST_ADD: u32 = 0x005a_e3d0;
/// `00905330` (`ECX` = node, pointer to the item): unlinks the item.
const LIST_REMOVE: u32 = 0x0090_5330;
/// `0063f7b0` (`ECX` = first node): removes the first item.
const LIST_REMOVE_FIRST: u32 = 0x0063_f7b0;
/// `005ae380` (`ECX` = first node): number of non-null items.
const LIST_COUNT: u32 = 0x005a_e380;

/// `MemoryManager` allocation (`00401000`, `cdecl` size) and release
/// (`00401030`, `cdecl` block).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;

/// `ContainerItemExtra::ContainerItemExtra` (Xbox PDB name of `0040e690`).
const EXTRA_CONSTRUCT: u32 = 0x0040_e690;
/// `ContainerItemExtra::InitItem` (Xbox PDB name of `0040e7f0`; owner form).
const EXTRA_INIT_ITEM: u32 = 0x0040_e7f0;
/// `0040e920` (`ECX` = extra, other extra): true when they differ (other
/// null, or owner, owner word or health multiplier unequal).
const EXTRA_DIFFERS: u32 = 0x0040_e920;
/// `0040e980` (`ECX` = extra, source): copies the source's three words.
const EXTRA_COPY: u32 = 0x0040_e980;
/// `0040e9c0` (`ECX` = extra): true when it has no owner and a health
/// multiplier of 1.0.
const EXTRA_IS_DEFAULT: u32 = 0x0040_e9c0;
/// `0040ea20` (`ECX` = extra, extra-data list, health form): puts the
/// extra's data on the list.
const EXTRA_APPLY: u32 = 0x0040_ea20;
/// `0040eb90` (`ECX` = extra, `float`): sets the health multiplier.
const EXTRA_SET_HEALTH: u32 = 0x0040_eb90;
/// `00487620` (`ECX` = `ContainerObject`): releases its `pItemExtra`.
const CONTAINER_OBJECT_DESTRUCT: u32 = 0x0048_7620;

/// `TESForm::GetFile` (Xbox PDB name of `00484e60`; `ECX` = form, index).
const TES_FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB name of `00485d50`; `cdecl`
/// pointer to the form id, file).
const TES_FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (`cdecl` form id): the loaded form with that id, or 0.
const FORM_BY_ID: u32 = 0x0048_39c0;
/// `00474cb0` (`ECX` = form): the length of the name from the form's
/// virtual slot `0x130`, or 0.
const FORM_NAME_LENGTH: u32 = 0x0047_4cb0;
/// `0084e3a0` (`ECX` = form): the word at +0xC (`TESForm::iFormID`).
const WORD_AT_C: u32 = 0x0084_e3a0;
/// `00440d80` (`ECX` = form): whether bit 0x20 of the word at +8
/// (`TESForm::iFormFlags`) is set.
const FORM_FLAG_20: u32 = 0x0044_0d80;
/// `00401170` (`ECX` = form): the byte at +4 (`TESForm::cFormType`).
const FORM_TYPE: u32 = 0x0040_1170;
/// `005b5e40` (`cdecl` format, ...): the error print.
const PRINT_ERROR: u32 = 0x005b_5e40;
/// `"MASTERFILE: Unable to find container object (%08X) on unknown owner."`
const MESSAGE_UNKNOWN_OWNER: u32 = 0x0101_c048;
/// `"MASTERFILE: Unable to find container object (%08X) on owner object (%08X)."`
const MESSAGE_OWNER_ID: u32 = 0x0101_c090;
/// `"MASTERFILE: Unable to find container object (%08X) on owner object \"%s\"."`
const MESSAGE_OWNER_NAME: u32 = 0x0101_c0e0;

/// `BaseFormComponent`'s constructor (`00402d30`, sets its vtable).
const BASE_FORM_COMPONENT_CONSTRUCT: u32 = 0x0040_2d30;
/// `TESContainer`'s vtable (RTTI `.?AVTESContainer@@`).
const TES_CONTAINER_VTABLE: u32 = 0x0101_c130;
/// `0096a2d0` (`ECX` = list): clears its two words.
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// `00483710` (`ECX` = component): `TESContainer`'s
/// `InitializeDataComponent` slot (vtable entry 0), an empty body.
const INITIALIZE_DATA_COMPONENT: u32 = 0x0048_3710;
/// `0046ffb0` (`ECX` = list): the list's destructor.
const LIST_DESTRUCT: u32 = 0x0046_ffb0;

/// `ContainerItemExtra::Save` (engine map name of `00481310`; `ECX` =
/// the `ContainerObject`).
const CONTAINER_OBJECT_SAVE: u32 = 0x0048_1310;
/// `00ec7d40` (`cdecl` int): its absolute value.
const ABSOLUTE_VALUE: u32 = 0x00ec_7d40;
/// `TESWeightForm::GetFormWeight` (Xbox PDB name of `0048ebc0`; `cdecl`
/// form, byte flag; `float` in ST0).
const GET_FORM_WEIGHT: u32 = 0x0048_ebc0;
/// `TESActorBase::GetDesirability` (engine map name of `005f0d60`; `ECX` =
/// the actor argument, object; `float` in ST0).
const GET_DESIRABILITY: u32 = 0x005f_0d60;
/// `TESBipedModelForm::FillsBipedSlot` (Xbox PDB name of `00480af0`; `ECX`
/// = the biped model form, slot, two more words).
const FILLS_BIPED_SLOT: u32 = 0x0048_0af0;
/// `TESObjectARMO`'s `TESBipedModelForm` component offset (as `004829c0`
/// adds it).
const ARMOR_BIPED_MODEL_FORM: u32 = 0x70;
/// `-FLT_MAX` (`float`): the starting best desirability.
const LOWEST_FLOAT: u32 = 0x0101_5f5c;

/// `ExtraDataList::ExtraDataList` (Xbox PDB name of `00410360`).
const EXTRA_DATA_LIST_CONSTRUCT: u32 = 0x0041_0360;
/// `ExtraDataList::SetCount` (Xbox PDB name of `00419ad0`; 16-bit count).
const EXTRA_DATA_LIST_SET_COUNT: u32 = 0x0041_9ad0;
/// `ExtraDataList::GetScript` (`00418800`), `SetScript` (`00419ed0`) and
/// `GetScriptLocals` (`00418830`) (Xbox PDB).
const EXTRA_DATA_LIST_GET_SCRIPT: u32 = 0x0041_8800;
const EXTRA_DATA_LIST_SET_SCRIPT: u32 = 0x0041_9ed0;
const EXTRA_DATA_LIST_GET_SCRIPT_LOCALS: u32 = 0x0041_8830;
/// `00419f80` (`ECX` = extra-data list, locals): stores the locals.
const EXTRA_DATA_LIST_SET_LOCALS: u32 = 0x0041_9f80;
/// `0041d280` (`ECX` = extra-data list, one word).
const EXTRA_DATA_LIST_0041D280: u32 = 0x0041_d280;
/// `005abf60` (`ECX` = script): the locals made for it.
const SCRIPT_MAKE_LOCALS: u32 = 0x005a_bf60;
/// `Script::Run` (Xbox PDB name of `005ac1e0`; `ECX` = script, four words).
const SCRIPT_RUN: u32 = 0x005a_c1e0;
/// `TESObjectREFR::TESObjectREFR` (Xbox PDB name of `0055a2f0`), 0x68 bytes.
const TES_OBJECT_REFR_CONSTRUCT: u32 = 0x0055_a2f0;
/// `TESForm::SetTemporary` (Xbox PDB name of `00484490`).
const TES_FORM_SET_TEMPORARY: u32 = 0x0048_4490;
/// `TESHealthForm::GetFormAsHealthForm` (Xbox PDB name of `004872e0`,
/// `cdecl` form).
const GET_FORM_AS_HEALTH_FORM: u32 = 0x0048_72e0;
/// `00574fa0` (`ECX` = reference, object, extra-data list, count): adds the
/// item to a reference that is not an `Actor`.
const REFERENCE_ADD_ITEM: u32 = 0x0057_4fa0;
/// The `Actor` virtual slot `004821a0` calls with the same three words.
const ACTOR_ADD_ITEM_SLOT: u32 = 0x190;
/// The deleting destructor's virtual slot of a `TESForm`.
const DELETING_DESTRUCTOR_SLOT: u32 = 0x10;

/// The player character pointer (`011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// `004037b0` / `004037d0`: `BSStringT<char>` constructor and destructor.
const BS_STRING_CONSTRUCT: u32 = 0x0040_37b0;
const BS_STRING_DESTRUCT: u32 = 0x0040_37d0;
/// `00406f60` (`cdecl` string, format, ...): formats into the string.
const BS_STRING_FORMAT: u32 = 0x0040_6f60;
/// `00559450` (`ECX` = string): its character pointer (the word at +0).
const BS_STRING_TEXT: u32 = 0x0055_9450;
/// `00403df0` (`ECX` = setting): its string value (the word at +4), or 0.
const SETTING_STRING: u32 = 0x0040_3df0;
/// The settings built by static initializers `00f6d140` (`sAddItemtoInventory`,
/// default `"added"`) and `00f6d0e0` (`sPlural`, default `"(s)"`).
const SETTING_ADD_ITEM_TO_INVENTORY: u32 = 0x011d_4e7c;
const SETTING_PLURAL: u32 = 0x011d_3114;
/// `"%i %s%s %s"` and `"%s %s"`.
const FORMAT_COUNT_NAME: u32 = 0x0101_c178;
const FORMAT_NAME: u32 = 0x0101_2058;
/// `"Interface\Icons\Message Icons\glow_message_giftbox.dds"`.
const GIFT_ICON: u32 = 0x0101_c140;
/// `2.0` (`float`): the message's display time argument.
const MESSAGE_TIME: u32 = 0x0101_62c0;
/// `Actor::GetPickUpSoundName` (Xbox PDB name of `008adcf0`; `ECX` =
/// actor, object, two words).
const GET_PICK_UP_SOUND_NAME: u32 = 0x008a_dcf0;
/// `007052f0` (`cdecl`, six words): shows the message.
const SHOW_MESSAGE: u32 = 0x0070_52f0;

/// `00408da0` (`ECX` = `TESFullName`): its name text.
const FULL_NAME_TEXT: u32 = 0x0040_8da0;
/// The empty string `GetFullName` answers without a `TESFullName`.
const EMPTY_STRING: u32 = 0x0101_1584;

/// `004c29a0` (`ECX` = inventory changes, object, extra-data list, count).
const INVENTORY_CHANGES_ADD: u32 = 0x004c_29a0;
/// `011dea0c` and `011ddf38`: the singletons `00482770` asks before
/// marking the change (through `004f1540`, `0042ce10` and `00452e90`).
const SINGLETON_011DEA0C: u32 = 0x011d_ea0c;
const SINGLETON_011DDF38: u32 = 0x011d_df38;
const QUERY_004F1540: u32 = 0x004f_1540;
const QUERY_0042CE10: u32 = 0x0042_ce10;
const QUERY_00452E90: u32 = 0x0045_2e90;
/// `TESForm::ForceChange` (Xbox PDB name of `00484b90`; change flags).
const TES_FORM_FORCE_CHANGE: u32 = 0x0048_4b90;
/// The change flag `00482770` forces.
const CHANGE_FLAG_INVENTORY: u32 = 0x0800_0000;

layout! {
    /// `ContainerObject` (Xbox PDB), 0x0C bytes on both builds.
    pub struct ContainerObject: 0x0C {
        /// `i_count` (Xbox PDB).
        0x00 i_count: i32,
        /// `pObj` (Xbox PDB): `TESBoundObject *` (a form id before
        /// `InitItem`).
        0x04 pObj: u32,
        /// `pItemExtra` (Xbox PDB): `ContainerItemExtra *`.
        0x08 pItemExtra: u32,
    }

    /// `TESContainer` (Xbox PDB), 0x0C bytes: the `BaseFormComponent`
    /// vtable and the item list.
    pub struct TESContainer: 0x0C {
        /// `objectList` (Xbox PDB): `BSSimpleList<ContainerObject *>`.
        0x04 objectList: Inline<BSSimpleList>,
    }
}

/// The first node of the container's list (`00717e50`).
fn list_head(e: &mut Engine, container: u32) -> u32 {
    e.call(ADDRESS_OF_4, &args![container]).u32()
}

/// The item of a node (`006815c0`, then the word there).
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(SAME_ADDRESS, &args![node]).u32();
    e.mem.u32(slot)
}

fn node_next(e: &mut Engine, node: u32) -> u32 {
    e.call(WORD_AT_4, &args![node]).u32()
}

fn node_is_empty(e: &mut Engine, node: u32) -> bool {
    e.call(NODE_IS_EMPTY, &args![node]).bool()
}

/// `new ContainerObject` (0x0C bytes, `004818b0`), or 0 when the
/// allocation fails.
fn new_container_object(e: &mut Engine) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x0cu32]).u32();
    if block == 0 {
        0
    } else {
        fn_004818b0(e, Ptr::new(block)).addr()
    }
}

/// `new ContainerItemExtra` (0x0C bytes), or 0.
fn new_item_extra(e: &mut Engine) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x0cu32]).u32();
    if block == 0 {
        0
    } else {
        e.call(EXTRA_CONSTRUCT, &args![block]).u32()
    }
}

/// `new ExtraDataList` (0x20 bytes), or 0.
fn new_extra_data_list(e: &mut Engine) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
    if block == 0 {
        0
    } else {
        e.call(EXTRA_DATA_LIST_CONSTRUCT, &args![block]).u32()
    }
}

/// Adds `item` to the container's list through a stack slot, as the game
/// passes it (`005ae3d0` takes the item's address).
fn add_to_list(e: &mut Engine, container: u32, item: u32) -> u32 {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), item);
        let head = list_head(e, container);
        e.call(LIST_ADD, &args![head, slot]);
        e.mem.u32(slot.addr())
    })
}

// Translated from 00481390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerObject::InitItem` (Xbox PDB): turns the loaded form id in
/// `pObj` into the `TESBoundObject` (the id adjusted by the owner's file,
/// `TESForm::AddCompileIndex`, looked up and cast); prints a `MASTERFILE`
/// error naming the owner when there is none, then initializes the item
/// extra with the owner.
pub fn container_object_init_item(e: &mut Engine, this: Ptr<ContainerObject>, owner: Ptr) {
    let form_id = e.get(this, ContainerObject::pObj);
    e.with_stack(4, |e, id| {
        e.mem.set_u32(id.addr(), form_id);
        let file = e
            .call(TES_FORM_GET_FILE, &args![owner, 0xffff_ffffu32])
            .u32();
        e.call(TES_FORM_ADD_COMPILE_INDEX, &args![id, file]);
        let adjusted = e.mem.u32(id.addr());
        let form = e.call(FORM_BY_ID, &args![adjusted]).u32();
        let object = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_BOUND_OBJECT, 0u32],
            )
            .u32();
        e.set(this, ContainerObject::pObj, object);
        if e.get(this, ContainerObject::pObj) == 0 {
            if owner.is_null() {
                let adjusted = e.mem.u32(id.addr());
                e.call(PRINT_ERROR, &args![MESSAGE_UNKNOWN_OWNER, adjusted]);
            } else if e.call(FORM_NAME_LENGTH, &args![owner]).u32() != 0 {
                let name = e.vcall(owner.addr(), 0x130, &[]).u32();
                let adjusted = e.mem.u32(id.addr());
                e.call(PRINT_ERROR, &args![MESSAGE_OWNER_NAME, adjusted, name]);
            } else {
                let owner_id = e.call(WORD_AT_C, &args![owner]).u32();
                let adjusted = e.mem.u32(id.addr());
                e.call(PRINT_ERROR, &args![MESSAGE_OWNER_ID, adjusted, owner_id]);
            }
        }
    });
    let extra = e.get(this, ContainerObject::pItemExtra);
    if extra != 0 {
        e.call(EXTRA_INIT_ITEM, &args![extra, owner]);
    }
}

// Translated from 00481470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether two `ContainerObject`s differ: `other` null, count or object
/// unequal, one extra missing while the other is not the default, or both
/// extras present and different (`0040e920`).
pub fn fn_00481470(
    e: &mut Engine,
    this: Ptr<ContainerObject>,
    other: Ptr<ContainerObject>,
) -> bool {
    if other.is_null() {
        return true;
    }
    if e.get(this, ContainerObject::i_count) != e.get(other, ContainerObject::i_count) {
        return true;
    }
    if e.get(this, ContainerObject::pObj) != e.get(other, ContainerObject::pObj) {
        return true;
    }
    let mine = e.get(this, ContainerObject::pItemExtra);
    if mine != 0
        && !e.call(EXTRA_IS_DEFAULT, &args![mine]).bool()
        && e.get(other, ContainerObject::pItemExtra) == 0
    {
        return true;
    }
    if e.get(this, ContainerObject::pItemExtra) == 0 {
        let theirs = e.get(other, ContainerObject::pItemExtra);
        if theirs != 0 && !e.call(EXTRA_IS_DEFAULT, &args![theirs]).bool() {
            return true;
        }
    }
    let mine = e.get(this, ContainerObject::pItemExtra);
    if mine != 0 {
        let theirs = e.get(other, ContainerObject::pItemExtra);
        if theirs != 0 && e.call(EXTRA_DIFFERS, &args![mine, theirs]).bool() {
            return true;
        }
    }
    false
}

// Translated from 00481540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a `ContainerObject` from `source`: object and count, and a new
/// copy of its extra when that is not the default.
// Translation incomplete: 00481540 - SEH exception unwinding is unsupported.
pub fn fn_00481540(e: &mut Engine, this: Ptr<ContainerObject>, source: Ptr<ContainerObject>) {
    if source.is_null() {
        return;
    }
    let object = e.get(source, ContainerObject::pObj);
    e.set(this, ContainerObject::pObj, object);
    let count = e.get(source, ContainerObject::i_count);
    e.set(this, ContainerObject::i_count, count);
    let extra = e.get(source, ContainerObject::pItemExtra);
    if extra != 0 && !e.call(EXTRA_IS_DEFAULT, &args![extra]).bool() {
        let copy = new_item_extra(e);
        e.set(this, ContainerObject::pItemExtra, copy);
        let source_extra = e.get(source, ContainerObject::pItemExtra);
        let copy = e.get(this, ContainerObject::pItemExtra);
        e.call(EXTRA_COPY, &args![copy, source_extra]);
    }
}

// Translated from 00481610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESContainer::TESContainer` (Xbox PDB): the `BaseFormComponent` base,
/// the `TESContainer` vtable, an empty item list, then
/// `InitializeDataComponent` (`00483710`).
// Translation incomplete: 00481610 - SEH exception unwinding is unsupported.
pub fn tes_container_tes_container(e: &mut Engine, this: Ptr<TESContainer>) -> Ptr<TESContainer> {
    e.call(BASE_FORM_COMPONENT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), TES_CONTAINER_VTABLE);
    e.call(LIST_CONSTRUCT, &args![this.addr() + 4]);
    e.call(INITIALIZE_DATA_COMPONENT, &args![this]);
    this
}

// Translated from 00481680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESContainer`'s destructor: frees the items
/// (`ClearDataComponent`), then destroys the list.
// Translation incomplete: 00481680 - SEH exception unwinding is unsupported.
pub fn fn_00481680(e: &mut Engine, this: Ptr<TESContainer>) {
    e.mem.set_u32(this.addr(), TES_CONTAINER_VTABLE);
    tes_container_clear_data_component(e, this);
    e.call(LIST_DESTRUCT, &args![this.addr() + 4]);
}

// Translated from 004816e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESContainer::ClearDataComponent` (Xbox PDB): `FreeObjectList`.
pub fn tes_container_clear_data_component(e: &mut Engine, this: Ptr<TESContainer>) {
    tes_container_free_object_list(e, this);
}

// Translated from 00481700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESContainer::FreeObjectList` (Xbox PDB): deletes the first item and
/// removes it from the list until the first node holds none. (The
/// compiler's null check before the delete repeats the loop's test.)
pub fn tes_container_free_object_list(e: &mut Engine, this: Ptr<TESContainer>) {
    let head = list_head(e, this.addr());
    loop {
        let item = node_item(e, head);
        if item == 0 {
            break;
        }
        fn_00481760(e, Ptr::new(item), 1);
        e.call(LIST_REMOVE_FIRST, &args![head]);
    }
}

// Translated from 00481760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerObject`'s deleting destructor: destroys it (`00487620`,
/// which frees its extra) and frees it when bit 0 of `flags` is set.
pub fn fn_00481760(e: &mut Engine, this: Ptr<ContainerObject>, flags: u32) -> Ptr<ContainerObject> {
    e.call(CONTAINER_OBJECT_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00481790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `item` from the container's list and deletes it.
pub fn fn_00481790(e: &mut Engine, this: Ptr<TESContainer>, item: Ptr<ContainerObject>) {
    if item.is_null() {
        return;
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), item.addr());
        let head = list_head(e, this.addr());
        e.call(LIST_REMOVE, &args![head, slot]);
        let item = e.mem.u32(slot.addr());
        if item != 0 {
            fn_00481760(e, Ptr::new(item), 1);
        }
    });
}

// Translated from 004817f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a new `ContainerObject` made from a two-word record (object, then
/// count) and returns it; 0 when `record` is null.
// Translation incomplete: 004817f0 - SEH exception unwinding is unsupported.
pub fn fn_004817f0(e: &mut Engine, this: Ptr<TESContainer>, record: Ptr) -> Ptr<ContainerObject> {
    if record.is_null() {
        return Ptr::NULL;
    }
    // The first call's result is not used.
    list_head(e, this.addr());
    let item: Ptr<ContainerObject> = Ptr::new(new_container_object(e));
    let object = e.mem.u32(record.addr());
    e.set(item, ContainerObject::pObj, object);
    let count = e.mem.i32(record.addr() + 4);
    e.set(item, ContainerObject::i_count, count);
    Ptr::new(add_to_list(e, this.addr(), item.addr()))
}

// Translated from 004818b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerObject`'s constructor: count 1, no object, no extra.
pub fn fn_004818b0(e: &mut Engine, this: Ptr<ContainerObject>) -> Ptr<ContainerObject> {
    e.set(this, ContainerObject::i_count, 1);
    e.set(this, ContainerObject::pObj, 0);
    e.set(this, ContainerObject::pItemExtra, 0);
    this
}

// Translated from 004818e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `count` of `object` with the given extra: adds to the count of the
/// item with that object and an equal extra (both none, or `0040e920` says
/// equal), else adds a new item (with a copy of the extra). Returns the
/// item; 0 when `object` or `count` is 0.
// Translation incomplete: 004818e0 - SEH exception unwinding is unsupported.
pub fn fn_004818e0(
    e: &mut Engine,
    this: Ptr<TESContainer>,
    object: u32,
    count: i32,
    extra: u32,
) -> Ptr<ContainerObject> {
    if object == 0 || count == 0 {
        return Ptr::NULL;
    }
    let mut found = 0u32;
    let mut node = list_head(e, this.addr());
    while node != 0 && !node_is_empty(e, node) {
        found = node_item(e, node);
        if e.mem.u32(found + 4) == object {
            let theirs = e.mem.u32(found + 8);
            if theirs != 0 {
                if extra != 0 && !e.call(EXTRA_DIFFERS, &args![theirs, extra]).bool() {
                    break;
                }
            } else if extra == 0 {
                break;
            }
        }
        found = 0;
        node = node_next(e, node);
    }
    if found != 0 {
        let total = e.mem.i32(found).wrapping_add(count);
        e.mem.set_i32(found, total);
        return Ptr::new(found);
    }
    let item: Ptr<ContainerObject> = Ptr::new(new_container_object(e));
    e.set(item, ContainerObject::i_count, count);
    e.set(item, ContainerObject::pObj, object);
    if extra != 0 {
        let copy = new_item_extra(e);
        e.set(item, ContainerObject::pItemExtra, copy);
        let copy = e.get(item, ContainerObject::pItemExtra);
        e.call(EXTRA_COPY, &args![copy, extra]);
    }
    Ptr::new(add_to_list(e, this.addr(), item.addr()))
}

// Translated from 00481a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a new copy (`00481540`) of `source` to the list.
// Translation incomplete: 00481a90 - SEH exception unwinding is unsupported.
pub fn fn_00481a90(e: &mut Engine, this: Ptr<TESContainer>, source: Ptr<ContainerObject>) {
    if source.is_null() {
        return;
    }
    let item = new_container_object(e);
    fn_00481540(e, Ptr::new(item), source);
    add_to_list(e, this.addr(), item);
}

// Translated from 00481b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESContainer::Save` (Xbox PDB): saves every item (`00481310`) whose
/// object does not have form flag 0x20.
pub fn tes_container_save(e: &mut Engine, this: Ptr<TESContainer>) {
    let mut node = list_head(e, this.addr());
    if node_item(e, node) == 0 {
        node = 0;
    }
    while node != 0 {
        let item = node_item(e, node);
        let object = e.mem.u32(item + 4);
        if !e.call(FORM_FLAG_20, &args![object]).bool() {
            e.call(CONTAINER_OBJECT_SAVE, &args![item]);
        }
        node = node_next(e, node);
    }
}

// Translated from 00481bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESContainer::InitItem` (Xbox PDB): initializes every item with the
/// owner (`ContainerObject::InitItem`) and removes and deletes the items
/// whose object was not found.
pub fn tes_container_init_item(e: &mut Engine, this: Ptr<TESContainer>, owner: Ptr) {
    e.with_stack(4, |e, slot| {
        let mut node = list_head(e, this.addr());
        let mut previous = 0u32;
        while node != 0 && !node_is_empty(e, node) {
            let item = node_item(e, node);
            e.mem.set_u32(slot.addr(), item);
            container_object_init_item(e, Ptr::new(item), owner);
            let item = e.mem.u32(slot.addr());
            if e.mem.u32(item + 4) == 0 {
                if previous != 0 {
                    e.call(LIST_REMOVE, &args![previous, slot]);
                    node = node_next(e, previous);
                } else {
                    e.call(LIST_REMOVE_FIRST, &args![node]);
                }
                let item = e.mem.u32(slot.addr());
                if item != 0 {
                    fn_00481760(e, Ptr::new(item), 1);
                }
                e.mem.set_u32(slot.addr(), 0);
            } else {
                previous = node;
                node = node_next(e, node);
            }
        }
    });
}

// Translated from 00481c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESContainer::CopyComponent` (Xbox PDB): when `other` is a
/// `TESContainer`, frees this list and adds a copy of each of its items.
pub fn tes_container_copy_component(e: &mut Engine, this: Ptr<TESContainer>, other: Ptr) {
    let source = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_TES_CONTAINER,
                0u32
            ],
        )
        .u32();
    if source == 0 {
        return;
    }
    tes_container_free_object_list(e, this);
    let mut node = list_head(e, source);
    while node != 0 && !node_is_empty(e, node) {
        let item = node_item(e, node);
        fn_00481a90(e, this, Ptr::new(item));
        node = node_next(e, node);
    }
}

// Translated from 00481d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESContainer::CompareComponent` (Xbox PDB): true when the containers
/// differ: `other` is not a `TESContainer`, an item of this one has no
/// equal item there (`00481470`), or the item counts differ.
pub fn tes_container_compare_component(
    e: &mut Engine,
    this: Ptr<TESContainer>,
    other: Ptr,
) -> bool {
    let theirs = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_TES_CONTAINER,
                0u32
            ],
        )
        .u32();
    if theirs == 0 {
        return true;
    }
    let mut matched = 0i32;
    let mut node = list_head(e, this.addr());
    while node != 0 {
        let item = node_item(e, node);
        if item != 0 {
            let mut other_node = list_head(e, theirs);
            let mut found = false;
            while other_node != 0 {
                let other_item = node_item(e, other_node);
                if other_item != 0 && !fn_00481470(e, Ptr::new(other_item), Ptr::new(item)) {
                    found = true;
                    break;
                }
                other_node = node_next(e, other_node);
            }
            if !found {
                return true;
            }
            matched = matched.wrapping_add(1);
        }
        node = node_next(e, node);
    }
    let head = list_head(e, theirs);
    e.call(LIST_COUNT, &args![head]).i32() != matched
}

// Translated from 00481e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The container's total weight: for each item whose object does not have
/// form flag 0x20, `|count|` times the object's weight
/// (`TESWeightForm::GetFormWeight` with `flag`), summed in a `float`.
pub fn fn_00481e10(e: &mut Engine, this: Ptr<TESContainer>, flag: u8) -> f32 {
    let mut total = 0.0f32;
    let mut node = list_head(e, this.addr());
    if node_item(e, node) == 0 {
        node = 0;
    }
    while node != 0 {
        let item = node_item(e, node);
        node = node_next(e, node);
        let object = e.mem.u32(item + 4);
        if e.call(FORM_FLAG_20, &args![object]).bool() {
            continue;
        }
        let count = e.mem.u32(item);
        let count = e.call(ABSOLUTE_VALUE, &args![count]).i32();
        let object = e.mem.u32(item + 4);
        let weight = e.call(GET_FORM_WEIGHT, &args![object, flag as u32]).f64();
        total = (weight * count as f64 + total as f64) as f32;
    }
    total
}

// Translated from 00481eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether an item of the container has `object`.
pub fn fn_00481eb0(e: &mut Engine, this: Ptr<TESContainer>, object: u32) -> bool {
    let mut node = list_head(e, this.addr());
    if node_item(e, node) == 0 {
        node = 0;
    }
    while node != 0 {
        let item = node_item(e, node);
        if e.mem.u32(item + 4) == object {
            return true;
        }
        node = node_next(e, node);
    }
    false
}

// Translated from 00481f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESContainer::IsGold` (Xbox PDB; `cdecl`): whether the form's id
/// (`0084e3a0`) is 0xF.
pub fn tes_container_is_gold(e: &mut Engine, form: Ptr) -> bool {
    !form.is_null() && e.call(WORD_AT_C, &args![form]).u32() == 0xf
}

// Translated from 00481f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESContainer::ContainerCanHoldType` (Xbox PDB; `cdecl`): whether a form
/// type can be held, by the compiler's jump table at `00481f68` (index
/// bytes at `00481f70` for types 0x18 to 0x74).
pub fn tes_container_container_can_hold_type(_e: &mut Engine, form_type: u8) -> bool {
    matches!(
        form_type,
        0x18 | 0x19
            | 0x1a
            | 0x1d
            | 0x1e
            | 0x1f
            | 0x28
            | 0x29
            | 0x2e
            | 0x2f
            | 0x31
            | 0x32
            | 0x34
            | 0x67
            | 0x6c
            | 0x73
            | 0x74
    )
}

// Translated from 00481fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds every item of `other` (`004818e0`: object, count, extra).
pub fn fn_00481fd0(e: &mut Engine, this: Ptr<TESContainer>, other: Ptr<TESContainer>) {
    if other.is_null() {
        return;
    }
    let mut node = list_head(e, other.addr());
    while node != 0 {
        let item = node_item(e, node);
        if item != 0 {
            let extra = e.mem.u32(item + 8);
            let count = e.mem.i32(item);
            let object = e.mem.u32(item + 4);
            fn_004818e0(e, this, object, count, extra);
        }
        node = node_next(e, node);
    }
}

// Translated from 00482040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Multiplies every item's count by `factor` (32-bit `IMUL`).
pub fn fn_00482040(e: &mut Engine, this: Ptr<TESContainer>, factor: i32) {
    let mut node = list_head(e, this.addr());
    while node != 0 {
        let item = node_item(e, node);
        if item != 0 {
            let count = e.mem.i32(item).wrapping_mul(factor);
            e.mem.set_i32(item, count);
        }
        node = node_next(e, node);
    }
}

// Translated from 00482090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets every item's health multiplier (`0040eb90`), giving items without
/// an extra a new one, and frees the extras that are then the default.
// Translation incomplete: 00482090 - SEH exception unwinding is unsupported.
pub fn fn_00482090(e: &mut Engine, this: Ptr<TESContainer>, health: f32) {
    let mut node = list_head(e, this.addr());
    while node != 0 && !node_is_empty(e, node) {
        let item: Ptr<ContainerObject> = Ptr::new(node_item(e, node));
        if e.get(item, ContainerObject::pItemExtra) == 0 {
            let extra = new_item_extra(e);
            e.set(item, ContainerObject::pItemExtra, extra);
        }
        let extra = e.get(item, ContainerObject::pItemExtra);
        e.call(
            EXTRA_SET_HEALTH,
            &args![extra, x87_quiet_bits(health.to_bits())],
        );
        let extra = e.get(item, ContainerObject::pItemExtra);
        if e.call(EXTRA_IS_DEFAULT, &args![extra]).bool() {
            let extra = e.get(item, ContainerObject::pItemExtra);
            e.call(OPERATOR_DELETE, &args![extra]);
            e.set(item, ContainerObject::pItemExtra, 0);
        }
        node = node_next(e, node);
    }
}

// Translated from 004821a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives the container's items to `target` (a reference): each item with a
/// script once per count, each with its own extra-data list holding the
/// script, its locals (initialized by running the script on a temporary
/// reference) and the item's extra; others in one call with their count.
/// Items go through the `Actor`'s slot `0x190` when `target` is an
/// `Actor`, else `00574fa0`. When the player is the target and `quiet` is
/// 0, shows `"<count> <name>(s) added"` (or `"<name> added"`) with the gift
/// icon for the last item when its form type is one of the listed ones.
// Translation incomplete: 004821a0 - SEH exception unwinding is unsupported.
pub fn fn_004821a0(e: &mut Engine, this: Ptr<TESContainer>, target: Ptr, quiet: u8) {
    if target.is_null() {
        return;
    }
    let actor = e
        .call(
            RT_DYNAMIC_CAST,
            &args![target, 0u32, TYPE_TES_OBJECT_REFR, TYPE_ACTOR, 0u32],
        )
        .u32();
    let mut last: Ptr<ContainerObject> = Ptr::NULL;
    let mut node = list_head(e, this.addr());
    while node != 0 {
        let item: Ptr<ContainerObject> = Ptr::new(node_item(e, node));
        if !item.is_null() {
            let object = e.get(item, ContainerObject::pObj);
            let script = tes_scriptable_form_get_form_script(e, Ptr::new(object)).addr();
            let count = e.get(item, ContainerObject::i_count);
            last = item;
            if script != 0 {
                let mut copy = 0i32;
                while copy < count {
                    give_scripted_copy(e, target, actor, item, script);
                    copy += 1;
                }
            } else {
                let mut list = 0u32;
                if e.get(item, ContainerObject::pItemExtra) != 0 {
                    list = new_extra_data_list(e);
                    let count = e.get(item, ContainerObject::i_count) as u16;
                    e.call(EXTRA_DATA_LIST_SET_COUNT, &args![list, count as u32]);
                    let object = e.get(item, ContainerObject::pObj);
                    let health = e.call(GET_FORM_AS_HEALTH_FORM, &args![object]).u32();
                    let extra = e.get(item, ContainerObject::pItemExtra);
                    e.call(EXTRA_APPLY, &args![extra, list, health]);
                }
                let object = e.get(item, ContainerObject::pObj);
                let count = e.get(item, ContainerObject::i_count);
                if actor != 0 {
                    e.vcall(actor, ACTOR_ADD_ITEM_SLOT, &args![object, list, count]);
                } else {
                    e.call(REFERENCE_ADD_ITEM, &args![target, object, list, count]);
                }
            }
        }
        node = node_next(e, node);
    }

    if actor != e.global::<u32>(PLAYER) || last.is_null() || quiet != 0 {
        return;
    }
    e.with_stack(8, |e, text| {
        e.call(BS_STRING_CONSTRUCT, &args![text]);
        if e.get(last, ContainerObject::i_count) > 1 {
            let added = e
                .call(SETTING_STRING, &args![SETTING_ADD_ITEM_TO_INVENTORY])
                .u32();
            let plural = e.call(SETTING_STRING, &args![SETTING_PLURAL]).u32();
            let object = e.get(last, ContainerObject::pObj);
            let name = tes_full_name_get_full_name(e, Ptr::new(object)).addr();
            let count = e.get(last, ContainerObject::i_count);
            e.call(
                BS_STRING_FORMAT,
                &args![text, FORMAT_COUNT_NAME, count, name, plural, added],
            );
        } else {
            let added = e
                .call(SETTING_STRING, &args![SETTING_ADD_ITEM_TO_INVENTORY])
                .u32();
            let object = e.get(last, ContainerObject::pObj);
            let name = tes_full_name_get_full_name(e, Ptr::new(object)).addr();
            e.call(BS_STRING_FORMAT, &args![text, FORMAT_NAME, name, added]);
        }
        let object = e.get(last, ContainerObject::pObj);
        let form_type = e.call(FORM_TYPE, &args![object]).u32();
        // The compiler's switch (jump table at 00482660, index bytes at
        // 0048266c); both of its cases run the same code.
        if matches!(
            form_type,
            0x18 | 0x19
                | 0x1e
                | 0x1f
                | 0x28
                | 0x29
                | 0x2e
                | 0x2f
                | 0x32
                | 0x67
                | 0x6c
                | 0x73
                | 0x74
        ) {
            let sound = if quiet != 0 {
                0
            } else {
                let player = e.global::<u32>(PLAYER);
                let object = e.get(last, ContainerObject::pObj);
                e.call(GET_PICK_UP_SOUND_NAME, &args![player, object, 1u32, 0u32])
                    .u32()
            };
            let message = e.call(BS_STRING_TEXT, &args![text]).u32();
            let time = x87_quiet_bits(e.global::<u32>(MESSAGE_TIME));
            e.call(
                SHOW_MESSAGE,
                &args![message, 0u32, GIFT_ICON, sound, time, 0u32],
            );
        }
        e.call(BS_STRING_DESTRUCT, &args![text]);
    });
}

/// One copy of a scripted item for `004821a0`: a new extra-data list with
/// count 1; when it has no script yet, the script, new locals, a run of the
/// script on a temporary reference (deleted after) and the item's extra;
/// then the item goes to the target.
fn give_scripted_copy(
    e: &mut Engine,
    target: Ptr,
    actor: u32,
    item: Ptr<ContainerObject>,
    script: u32,
) {
    let list = new_extra_data_list(e);
    e.call(EXTRA_DATA_LIST_SET_COUNT, &args![list, 1u32]);
    if list != 0 && e.call(EXTRA_DATA_LIST_GET_SCRIPT, &args![list]).u32() == 0 {
        e.call(EXTRA_DATA_LIST_SET_SCRIPT, &args![list, script]);
        let locals = e.call(SCRIPT_MAKE_LOCALS, &args![script]).u32();
        e.call(EXTRA_DATA_LIST_SET_LOCALS, &args![list, locals]);
        let block = e.call(OPERATOR_NEW, &args![0x68u32]).u32();
        let reference = if block == 0 {
            0
        } else {
            e.call(TES_OBJECT_REFR_CONSTRUCT, &args![block]).u32()
        };
        e.call(TES_FORM_SET_TEMPORARY, &args![reference]);
        let locals = e
            .call(EXTRA_DATA_LIST_GET_SCRIPT_LOCALS, &args![list])
            .u32();
        e.call(SCRIPT_RUN, &args![script, reference, locals, 0u32, 0u32]);
        if reference != 0 {
            e.vcall(reference, DELETING_DESTRUCTOR_SLOT, &args![1u32]);
        }
        if e.get(item, ContainerObject::pItemExtra) != 0 {
            let object = e.get(item, ContainerObject::pObj);
            let health = e.call(GET_FORM_AS_HEALTH_FORM, &args![object]).u32();
            let extra = e.get(item, ContainerObject::pItemExtra);
            e.call(EXTRA_APPLY, &args![extra, list, health]);
        }
    }
    let object = e.get(item, ContainerObject::pObj);
    if actor != 0 {
        e.vcall(actor, ACTOR_ADD_ITEM_SLOT, &args![object, list, 1u32]);
    } else {
        e.call(REFERENCE_ADD_ITEM, &args![target, object, list, 1u32]);
    }
}

// Translated from 004826d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESScriptableForm::GetFormScript` (Xbox PDB; `cdecl`, one word): casts
/// the form from `TESForm` to `TESScriptableForm` (`__RTDynamicCast`) and
/// answers the component's word at `+4` (`00726070`), or 0 when the form
/// has no scriptable component.
pub fn tes_scriptable_form_get_form_script(e: &mut Engine, form: Ptr) -> Ptr {
    let scriptable = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_SCRIPTABLE_FORM, 0u32],
        )
        .u32();
    if scriptable == 0 {
        Ptr::new(0)
    } else {
        e.call(WORD_AT_4, &args![scriptable]).ptr()
    }
}

// Translated from 00482720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFullName::GetFullName` (Xbox PDB; `cdecl`): the form's name text
/// (`00408da0` on its `TESFullName`), or the empty string at `01011584`
/// when it has no `TESFullName`.
pub fn tes_full_name_get_full_name(e: &mut Engine, form: Ptr) -> Ptr {
    let full_name = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_FULL_NAME, 0u32],
        )
        .u32();
    if full_name == 0 {
        Ptr::new(EMPTY_STRING)
    } else {
        e.call(FULL_NAME_TEXT, &args![full_name]).ptr()
    }
}

// Translated from 00482770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds every item to the inventory changes `changes` (`004c29a0`), each
/// with a new extra-data list (`0041d280` with `source`, the count, the
/// item's extra); when one was added and the changes have a form (+4),
/// forces its change flag 0x08000000 unless the singletons' checks
/// (`004f1540` on `011dea0c` false, or `0042ce10` true and `00452e90`
/// false on `011ddf38`) say not to.
// Translation incomplete: 00482770 - SEH exception unwinding is unsupported.
pub fn fn_00482770(e: &mut Engine, this: Ptr<TESContainer>, source: u32, changes: Ptr) {
    if changes.is_null() {
        return;
    }
    let mut added = false;
    let mut node = list_head(e, this.addr());
    while node != 0 && !node_is_empty(e, node) {
        let item: Ptr<ContainerObject> = Ptr::new(node_item(e, node));
        if !item.is_null() {
            added = true;
            let list = new_extra_data_list(e);
            e.call(EXTRA_DATA_LIST_0041D280, &args![list, source]);
            let count = e.get(item, ContainerObject::i_count) as u16;
            e.call(EXTRA_DATA_LIST_SET_COUNT, &args![list, count as u32]);
            if e.get(item, ContainerObject::pItemExtra) != 0 {
                let object = e.get(item, ContainerObject::pObj);
                let health = e.call(GET_FORM_AS_HEALTH_FORM, &args![object]).u32();
                let extra = e.get(item, ContainerObject::pItemExtra);
                e.call(EXTRA_APPLY, &args![extra, list, health]);
            }
            let count = e.get(item, ContainerObject::i_count);
            let object = e.get(item, ContainerObject::pObj);
            e.call(INVENTORY_CHANGES_ADD, &args![changes, object, list, count]);
        }
        node = node_next(e, node);
    }
    if !added || e.call(WORD_AT_4, &args![changes]).u32() == 0 {
        return;
    }
    let first = e.global::<u32>(SINGLETON_011DEA0C);
    if !e.call(QUERY_004F1540, &args![first]).bool() {
        return;
    }
    let second = e.global::<u32>(SINGLETON_011DDF38);
    if e.call(QUERY_0042CE10, &args![second]).bool() {
        let second = e.global::<u32>(SINGLETON_011DDF38);
        if !e.call(QUERY_00452E90, &args![second]).bool() {
            return;
        }
    }
    let form = e.call(WORD_AT_4, &args![changes]).u32();
    e.call(TES_FORM_FORCE_CHANGE, &args![form, CHANGE_FLAG_INVENTORY]);
}

/// The item with the highest desirability (`005f0d60` with `actor`) among
/// those whose object casts to `target_type` and passes `accept`; 0 if
/// none. A strictly greater value replaces the best (`FCOMPP`, `TEST AH,5`,
/// `JP`: unordered or not greater keeps it).
fn most_desirable(
    e: &mut Engine,
    this: Ptr<TESContainer>,
    actor: Ptr,
    target_type: u32,
    accept: impl Fn(&mut Engine, u32) -> bool,
) -> u32 {
    let mut node = list_head(e, this.addr());
    let mut best_value = x87_quiet(f32::from_bits(e.global::<u32>(LOWEST_FLOAT)));
    let mut best = 0u32;
    while node != 0 {
        let item = node_item(e, node);
        if item != 0 {
            let object = e.mem.u32(item + 4);
            let cast = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![object, 0u32, TYPE_TES_BOUND_OBJECT, target_type, 0u32],
                )
                .u32();
            if cast != 0 && accept(e, cast) {
                let value = e.call(GET_DESIRABILITY, &args![actor, cast]).f32();
                if best_value < value {
                    best_value = x87_quiet(value);
                    best = cast;
                }
            }
        }
        node = node_next(e, node);
    }
    best
}

// Translated from 00482910 (decompiled, FalloutNV.exe 1.4.0.525)
/// The container's most desirable weapon (`TESObjectWEAP`) for `actor`
/// (`005f0d60`), or 0.
pub fn fn_00482910(e: &mut Engine, this: Ptr<TESContainer>, actor: Ptr) -> u32 {
    most_desirable(e, this, actor, TYPE_TES_OBJECT_WEAP, |_, _| true)
}

// Translated from 004829c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The container's most desirable armor (`TESObjectARMO`) for `actor` that
/// fills biped slot `slot` (`TESBipedModelForm::FillsBipedSlot`, with 0
/// and 0); none when `slot` is -1.
pub fn fn_004829c0(e: &mut Engine, this: Ptr<TESContainer>, actor: Ptr, slot: i32) -> u32 {
    most_desirable(e, this, actor, TYPE_TES_OBJECT_ARMO, |e, armor| {
        slot != -1
            && e.call(
                FILLS_BIPED_SLOT,
                &args![armor + ARMOR_BIPED_MODEL_FORM, slot, 0u32, 0u32],
            )
            .bool()
    })
}

// Translated from 00482a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESContainer::GetObjectCount` (Xbox PDB): the summed count of the
/// items with `object`, up to the first node without an item.
pub fn tes_container_get_object_count(e: &mut Engine, this: Ptr<TESContainer>, object: u32) -> i32 {
    let mut total = 0i32;
    let mut node = list_head(e, this.addr());
    while node != 0 && node_item(e, node) != 0 {
        let item = node_item(e, node);
        if e.mem.u32(item + 4) == object {
            let item = node_item(e, node);
            total = total.wrapping_add(e.mem.i32(item));
        }
        node = node_next(e, node);
    }
    total
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00481390,
            container_object_init_item(Ptr<ContainerObject>, Ptr)
        ),
        entry!(
            0x00481470,
            fn_00481470(Ptr<ContainerObject>, Ptr<ContainerObject>) -> bool
        ),
        entry!(
            0x00481540,
            fn_00481540(Ptr<ContainerObject>, Ptr<ContainerObject>)
        ),
        entry!(
            0x00481610,
            tes_container_tes_container(Ptr<TESContainer>) -> Ptr<TESContainer>
        ),
        entry!(0x00481680, fn_00481680(Ptr<TESContainer>)),
        entry!(
            0x004816e0,
            tes_container_clear_data_component(Ptr<TESContainer>)
        ),
        entry!(
            0x00481700,
            tes_container_free_object_list(Ptr<TESContainer>)
        ),
        entry!(
            0x00481760,
            fn_00481760(Ptr<ContainerObject>, u32) -> Ptr<ContainerObject>
        ),
        entry!(
            0x00481790,
            fn_00481790(Ptr<TESContainer>, Ptr<ContainerObject>)
        ),
        entry!(
            0x004817f0,
            fn_004817f0(Ptr<TESContainer>, Ptr) -> Ptr<ContainerObject>
        ),
        entry!(
            0x004818b0,
            fn_004818b0(Ptr<ContainerObject>) -> Ptr<ContainerObject>
        ),
        entry!(
            0x004818e0,
            fn_004818e0(Ptr<TESContainer>, u32, i32, u32) -> Ptr<ContainerObject>
        ),
        entry!(
            0x00481a90,
            fn_00481a90(Ptr<TESContainer>, Ptr<ContainerObject>)
        ),
        entry!(0x00481b40, tes_container_save(Ptr<TESContainer>)),
        entry!(0x00481bb0, tes_container_init_item(Ptr<TESContainer>, Ptr)),
        entry!(
            0x00481c80,
            tes_container_copy_component(Ptr<TESContainer>, Ptr)
        ),
        entry!(
            0x00481d10,
            tes_container_compare_component(Ptr<TESContainer>, Ptr) -> bool
        ),
        entry!(0x00481e10, fn_00481e10(Ptr<TESContainer>, u8) -> f32),
        entry!(0x00481eb0, fn_00481eb0(Ptr<TESContainer>, u32) -> bool),
        entry!(0x00481f10, tes_container_is_gold(Ptr) -> bool),
        entry!(0x00481f30, tes_container_container_can_hold_type(u8) -> bool),
        entry!(
            0x00481fd0,
            fn_00481fd0(Ptr<TESContainer>, Ptr<TESContainer>)
        ),
        entry!(0x00482040, fn_00482040(Ptr<TESContainer>, i32)),
        entry!(0x00482090, fn_00482090(Ptr<TESContainer>, f32)),
        entry!(0x004821a0, fn_004821a0(Ptr<TESContainer>, Ptr, u8)),
        entry!(0x004826d0, tes_scriptable_form_get_form_script(Ptr) -> Ptr),
        entry!(0x00482720, tes_full_name_get_full_name(Ptr) -> Ptr),
        entry!(0x00482770, fn_00482770(Ptr<TESContainer>, u32, Ptr)),
        entry!(0x00482910, fn_00482910(Ptr<TESContainer>, Ptr) -> u32),
        entry!(0x004829c0, fn_004829c0(Ptr<TESContainer>, Ptr, i32) -> u32),
        entry!(
            0x00482a90,
            tes_container_get_object_count(Ptr<TESContainer>, u32) -> i32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(v: u32) -> Ret {
        Ret {
            eax: v,
            ..Ret::default()
        }
    }

    fn ret_f(v: f64) -> Ret {
        Ret {
            st0: v,
            ..Ret::default()
        }
    }

    /// An engine with doubles for the list helpers and the allocator.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(ADDRESS_OF_4, |_, a| ret(a[0] + 4));
        e.register(SAME_ADDRESS, |_, a| ret(a[0]));
        e.register(WORD_AT_4, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(NODE_IS_EMPTY, |e, a| {
            ret((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.register(EXTRA_CONSTRUCT, |_, a| ret(a[0]));
        e.register(EXTRA_DATA_LIST_CONSTRUCT, |_, a| ret(a[0]));
        e.register(CONTAINER_OBJECT_DESTRUCT, |_, _| Ret::default());
        e.register(EXTRA_COPY, |_, _| Ret::default());
        // Append the item at the end of the list.
        e.register(LIST_ADD, |e, a| {
            let item = e.mem.u32(a[1]);
            let mut node = a[0];
            if e.mem.u32(node) == 0 && e.mem.u32(node + 4) == 0 {
                e.mem.set_u32(node, item);
                return Ret::default();
            }
            while e.mem.u32(node + 4) != 0 {
                node = e.mem.u32(node + 4);
            }
            let fresh = e.mem.alloc(8);
            e.mem.set_u32(fresh, item);
            e.mem.set_u32(node + 4, fresh);
            Ret::default()
        });
        // Remove the first item: the head takes the next node's item.
        e.register(LIST_REMOVE_FIRST, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next == 0 {
                e.mem.set_u32(a[0], 0);
            } else {
                let item = e.mem.u32(next);
                let after = e.mem.u32(next + 4);
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e
    }

    fn object(e: &mut Engine, count: i32, form: u32, extra: u32) -> u32 {
        let p = e.mem.alloc(0x0c);
        e.mem.set_i32(p, count);
        e.mem.set_u32(p + 4, form);
        e.mem.set_u32(p + 8, extra);
        p
    }

    /// A `TESContainer` whose list holds `items` (first node inline).
    fn container(e: &mut Engine, items: &[u32]) -> u32 {
        let c = e.mem.alloc(0x0c);
        let mut node = c + 4;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        c
    }

    fn items(e: &Engine, c: u32) -> Vec<u32> {
        let mut out = vec![];
        let mut node = c + 4;
        while node != 0 {
            let item = e.mem.u32(node);
            if item != 0 {
                out.push(item);
            }
            node = e.mem.u32(node + 4);
        }
        out
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .map(|l| {
                l.iter()
                    .filter(|c| c.0 == addr)
                    .map(|c| c.1.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// `__RTDynamicCast` answering `answer(object, target type)`.
    fn cast_by(e: &mut Engine, answer: fn(u32, u32) -> u32) {
        e.register_double(RT_DYNAMIC_CAST, move |_, a| ret(answer(a[0], a[3])));
    }

    fn init_item_engine(found: bool, name_length: u32) -> Engine {
        let mut e = engine();
        e.register(TES_FORM_GET_FILE, |_, _| ret(0x77));
        e.register(TES_FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(FORM_BY_ID, |_, a| ret(a[0] + 0x10));
        if found {
            cast_by(&mut e, |o, _| o);
        } else {
            cast_by(&mut e, |_, _| 0);
        }
        e.register_double(FORM_NAME_LENGTH, move |_, _| ret(name_length));
        e.register(WORD_AT_C, |_, _| ret(0x1234));
        e.register(PRINT_ERROR, |_, _| Ret::default());
        e.register(EXTRA_INIT_ITEM, |_, _| Ret::default());
        e.register(0x0050_0000, |_, _| ret(0x0400_0000));
        e
    }

    #[test]
    fn init_item_resolves_the_object_or_reports_the_owner() {
        let mut e = init_item_engine(true, 0);
        let item = object(&mut e, 1, 0x42, 0x0600_0000);
        let owner = e.mem.alloc(0x20);
        e.call(0x0048_1390, &args![item, owner]);
        assert_eq!(e.mem.u32(item + 4), 0x0100_0052);
        assert_eq!(calls_to(&e, TES_FORM_GET_FILE)[0], vec![owner, 0xffff_ffff]);
        assert_eq!(calls_to(&e, FORM_BY_ID)[0], vec![0x0100_0042]);
        assert_eq!(
            calls_to(&e, RT_DYNAMIC_CAST)[0],
            vec![0x0100_0052, 0, TYPE_TES_FORM, TYPE_TES_BOUND_OBJECT, 0]
        );
        assert_eq!(calls_to(&e, EXTRA_INIT_ITEM)[0], vec![0x0600_0000, owner]);
        assert!(calls_to(&e, PRINT_ERROR).is_empty());

        let mut e = init_item_engine(false, 0);
        let item = object(&mut e, 1, 0x42, 0);
        e.call(0x0048_1390, &args![item, 0u32]);
        assert_eq!(
            calls_to(&e, PRINT_ERROR)[0],
            vec![MESSAGE_UNKNOWN_OWNER, 0x0100_0042]
        );
        assert!(calls_to(&e, EXTRA_INIT_ITEM).is_empty());

        let mut e = init_item_engine(false, 0);
        let item = object(&mut e, 1, 0x42, 0);
        let owner = e.mem.alloc(0x20);
        e.call(0x0048_1390, &args![item, owner]);
        assert_eq!(
            calls_to(&e, PRINT_ERROR)[0],
            vec![MESSAGE_OWNER_ID, 0x0100_0042, 0x1234]
        );

        let mut e = init_item_engine(false, 5);
        let item = object(&mut e, 1, 0x42, 0);
        let owner = e.mem.alloc(0x20);
        e.put_vtable(0x0700_0000, &[0; 0x4d]);
        e.mem.set_u32(0x0700_0130, 0x0050_0000);
        e.mem.set_u32(owner, 0x0700_0000);
        e.call(0x0048_1390, &args![item, owner]);
        assert_eq!(
            calls_to(&e, PRINT_ERROR)[0],
            vec![MESSAGE_OWNER_NAME, 0x0100_0042, 0x0400_0000]
        );
    }

    #[test]
    fn container_objects_differ_by_count_object_and_extra() {
        let mut e = engine();
        // Extra 0x10 is the default, 0x20 is not; 0x30 and 0x40 differ.
        e.register(EXTRA_IS_DEFAULT, |_, a| ret((a[0] == 0x10) as u32));
        e.register(EXTRA_DIFFERS, |_, a| ret((a[0] != a[1]) as u32));
        type Item = (i32, u32, u32);
        let differs = |e: &mut Engine, a: Item, b: Option<Item>| {
            let x = object(e, a.0, a.1, a.2);
            let y = b.map_or(0, |b| object(e, b.0, b.1, b.2));
            e.call(0x0048_1470, &args![x, y]).bool()
        };
        assert!(differs(&mut e, (1, 5, 0), None));
        assert!(differs(&mut e, (1, 5, 0), Some((2, 5, 0))));
        assert!(differs(&mut e, (1, 5, 0), Some((1, 6, 0))));
        assert!(!differs(&mut e, (1, 5, 0), Some((1, 5, 0))));
        assert!(!differs(&mut e, (1, 5, 0x10), Some((1, 5, 0))));
        assert!(differs(&mut e, (1, 5, 0x20), Some((1, 5, 0))));
        assert!(differs(&mut e, (1, 5, 0), Some((1, 5, 0x20))));
        assert!(differs(&mut e, (1, 5, 0x30), Some((1, 5, 0x40))));
        assert!(!differs(&mut e, (1, 5, 0x30), Some((1, 5, 0x30))));
    }

    #[test]
    fn copy_takes_object_count_and_a_new_extra() {
        let mut e = engine();
        e.register(EXTRA_IS_DEFAULT, |_, a| ret((a[0] == 0x10) as u32));
        let source = object(&mut e, 3, 0x55, 0x20);
        let item = object(&mut e, 0, 0, 0);
        e.call(0x0048_1540, &args![item, source]);
        assert_eq!(e.mem.i32(item), 3);
        assert_eq!(e.mem.u32(item + 4), 0x55);
        let extra = e.mem.u32(item + 8);
        assert_ne!(extra, 0);
        assert_eq!(calls_to(&e, OPERATOR_NEW)[0], vec![0x0c]);
        assert_eq!(calls_to(&e, EXTRA_COPY)[0], vec![extra, 0x20]);

        let source = object(&mut e, 2, 0x56, 0x10);
        let item = object(&mut e, 0, 0, 0);
        e.call(0x0048_1540, &args![item, source]);
        assert_eq!(e.mem.u32(item + 8), 0);
        assert_eq!(calls_to(&e, EXTRA_COPY).len(), 1);
    }

    #[test]
    fn constructor_sets_the_vtable_and_an_empty_list() {
        let mut e = engine();
        e.register(BASE_FORM_COMPONENT_CONSTRUCT, |_, _| Ret::default());
        e.register(LIST_CONSTRUCT, |_, _| Ret::default());
        e.register(INITIALIZE_DATA_COMPONENT, |_, _| Ret::default());
        let c = e.mem.alloc(0x0c);
        let r = e.call(0x0048_1610, &args![c]).u32();
        assert_eq!(r, c);
        assert_eq!(e.mem.u32(c), TES_CONTAINER_VTABLE);
        assert_eq!(calls_to(&e, BASE_FORM_COMPONENT_CONSTRUCT)[0], vec![c]);
        assert_eq!(calls_to(&e, LIST_CONSTRUCT)[0], vec![c + 4]);
        assert_eq!(calls_to(&e, INITIALIZE_DATA_COMPONENT)[0], vec![c]);
    }

    #[test]
    fn destructor_frees_the_items_then_the_list() {
        let mut e = engine();
        e.register(LIST_DESTRUCT, |_, _| Ret::default());
        let a = object(&mut e, 1, 5, 0);
        let c = container(&mut e, &[a]);
        e.call(0x0048_1680, &args![c]);
        assert_eq!(e.mem.u32(c), TES_CONTAINER_VTABLE);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![a]]);
        assert_eq!(calls_to(&e, LIST_DESTRUCT)[0], vec![c + 4]);
    }

    #[test]
    fn clear_and_free_delete_every_item() {
        let mut e = engine();
        let a = object(&mut e, 1, 5, 0);
        let b = object(&mut e, 2, 6, 0);
        let c = container(&mut e, &[a, b]);
        e.call(0x0048_16e0, &args![c]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![a], vec![b]]);
        assert_eq!(calls_to(&e, LIST_REMOVE_FIRST).len(), 2);
        assert!(items(&e, c).is_empty());

        let mut e = engine();
        let c = container(&mut e, &[]);
        e.call(0x0048_1700, &args![c]);
        assert!(calls_to(&e, LIST_REMOVE_FIRST).is_empty());
    }

    #[test]
    fn deleting_destructor_frees_only_with_bit_0() {
        let mut e = engine();
        let a = object(&mut e, 1, 5, 0);
        assert_eq!(e.call(0x0048_1760, &args![a, 0u32]).u32(), a);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(calls_to(&e, CONTAINER_OBJECT_DESTRUCT)[0], vec![a]);
        e.call(0x0048_1760, &args![a, 1u32]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![a]]);
    }

    #[test]
    fn remove_unlinks_and_deletes_the_item() {
        let mut e = engine();
        e.register(LIST_REMOVE, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x0300_0000);
            Ret::default()
        });
        let c = container(&mut e, &[]);
        e.call(0x0048_1790, &args![c, 0u32]);
        assert!(calls_to(&e, LIST_REMOVE).is_empty());
        e.call(0x0048_1790, &args![c, 0x0300_0000u32]);
        assert_eq!(calls_to(&e, LIST_REMOVE)[0][0], c + 4);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![0x0300_0000]]);
    }

    #[test]
    fn add_from_record_takes_object_then_count() {
        let mut e = engine();
        let c = container(&mut e, &[]);
        assert_eq!(e.call(0x0048_17f0, &args![c, 0u32]).u32(), 0);
        let record = e.mem.alloc(8);
        e.mem.set_u32(record, 0x77);
        e.mem.set_i32(record + 4, 4);
        let item = e.call(0x0048_17f0, &args![c, record]).u32();
        assert_eq!(items(&e, c), vec![item]);
        assert_eq!(e.mem.i32(item), 4);
        assert_eq!(e.mem.u32(item + 4), 0x77);
        assert_eq!(e.mem.u32(item + 8), 0);
        assert_eq!(calls_to(&e, ADDRESS_OF_4).len(), 2);
    }

    #[test]
    fn container_object_constructor_counts_one() {
        let mut e = engine();
        let item = object(&mut e, 9, 9, 9);
        assert_eq!(e.call(0x0048_18b0, &args![item]).u32(), item);
        assert_eq!(e.mem.i32(item), 1);
        assert_eq!(e.mem.u32(item + 4), 0);
        assert_eq!(e.mem.u32(item + 8), 0);
    }

    #[test]
    fn add_object_merges_equal_items_or_adds_a_new_one() {
        let mut e = engine();
        e.register(EXTRA_DIFFERS, |_, a| ret((a[0] != a[1]) as u32));
        let a = object(&mut e, 2, 5, 0);
        let b = object(&mut e, 1, 6, 0x30);
        let c = container(&mut e, &[a, b]);
        assert_eq!(e.call(0x0048_18e0, &args![c, 0u32, 1i32, 0u32]).u32(), 0);
        assert_eq!(e.call(0x0048_18e0, &args![c, 5u32, 0i32, 0u32]).u32(), 0);
        assert_eq!(e.call(0x0048_18e0, &args![c, 5u32, 3i32, 0u32]).u32(), a);
        assert_eq!(e.mem.i32(a), 5);
        assert_eq!(e.call(0x0048_18e0, &args![c, 6u32, 2i32, 0x30u32]).u32(), b);
        assert_eq!(e.mem.i32(b), 3);
        // Same object, no extra on the item: a new item with a copied extra.
        let fresh = e.call(0x0048_18e0, &args![c, 5u32, 4i32, 0x40u32]).u32();
        assert_eq!(items(&e, c), vec![a, b, fresh]);
        assert_eq!(e.mem.i32(fresh), 4);
        assert_eq!(e.mem.u32(fresh + 4), 5);
        let extra = e.mem.u32(fresh + 8);
        assert_eq!(calls_to(&e, EXTRA_COPY)[0], vec![extra, 0x40]);
    }

    #[test]
    fn add_copy_appends_a_copy() {
        let mut e = engine();
        let source = object(&mut e, 3, 0x55, 0);
        let c = container(&mut e, &[]);
        e.call(0x0048_1a90, &args![c, 0u32]);
        assert!(items(&e, c).is_empty());
        e.call(0x0048_1a90, &args![c, source]);
        let added = items(&e, c);
        assert_eq!(added.len(), 1);
        assert_eq!(e.mem.i32(added[0]), 3);
        assert_eq!(e.mem.u32(added[0] + 4), 0x55);
    }

    #[test]
    fn save_skips_objects_with_flag_20() {
        let mut e = engine();
        e.register(FORM_FLAG_20, |_, a| ret((a[0] == 6) as u32));
        e.register(CONTAINER_OBJECT_SAVE, |_, _| Ret::default());
        let a = object(&mut e, 1, 5, 0);
        let b = object(&mut e, 1, 6, 0);
        let c = container(&mut e, &[a, b]);
        e.call(0x0048_1b40, &args![c]);
        assert_eq!(calls_to(&e, CONTAINER_OBJECT_SAVE), vec![vec![a]]);

        let mut e = engine();
        let c = container(&mut e, &[]);
        e.call(0x0048_1b40, &args![c]);
        assert!(calls_to(&e, FORM_FLAG_20).is_empty());
    }

    #[test]
    fn init_item_list_drops_unresolved_items() {
        let mut e = engine();
        e.register(TES_FORM_GET_FILE, |_, _| ret(0));
        e.register(TES_FORM_ADD_COMPILE_INDEX, |_, _| Ret::default());
        e.register(FORM_BY_ID, |_, a| ret(a[0]));
        // Form ids 0x10 and up resolve to themselves.
        cast_by(&mut e, |o, _| if o >= 0x10 { o } else { 0 });
        e.register(PRINT_ERROR, |_, _| Ret::default());
        e.register(WORD_AT_C, |_, _| ret(0));
        e.register(FORM_NAME_LENGTH, |_, _| ret(0));
        e.register(LIST_REMOVE, |e, a| {
            // Unlink the node after `a[0]`, which holds the item.
            let item = e.mem.u32(a[1]);
            let next = e.mem.u32(a[0] + 4);
            assert_eq!(e.mem.u32(next), item);
            let after = e.mem.u32(next + 4);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        let a = object(&mut e, 1, 1, 0);
        let b = object(&mut e, 1, 0x20, 0);
        let d = object(&mut e, 1, 2, 0);
        let f = object(&mut e, 1, 0x30, 0);
        let c = container(&mut e, &[a, b, d, f]);
        let owner = e.mem.alloc(0x20);
        e.call(0x0048_1bb0, &args![c, owner]);
        assert_eq!(items(&e, c), vec![b, f]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![a], vec![d]]);
        assert_eq!(calls_to(&e, LIST_REMOVE_FIRST).len(), 1);
        assert_eq!(calls_to(&e, LIST_REMOVE).len(), 1);
    }

    #[test]
    fn copy_component_replaces_the_items() {
        let mut e = engine();
        cast_by(&mut e, |o, _| if o == 0x0300_0000 { 0 } else { o });
        let old = object(&mut e, 1, 9, 0);
        let c = container(&mut e, &[old]);
        let x = object(&mut e, 2, 5, 0);
        let y = object(&mut e, 3, 6, 0);
        let other = container(&mut e, &[x, y]);
        e.call(0x0048_1c80, &args![c, 0x0300_0000u32]);
        assert_eq!(items(&e, c), vec![old]);
        e.call(0x0048_1c80, &args![c, other]);
        let copies = items(&e, c);
        assert_eq!(copies.len(), 2);
        assert_eq!(e.mem.u32(copies[0] + 4), 5);
        assert_eq!(e.mem.u32(copies[1] + 4), 6);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![old]]);
        assert_eq!(
            calls_to(&e, RT_DYNAMIC_CAST)[1],
            vec![other, 0, TYPE_BASE_FORM_COMPONENT, TYPE_TES_CONTAINER, 0]
        );
    }

    #[test]
    fn compare_component_needs_equal_items_and_counts() {
        let mut e = engine();
        cast_by(&mut e, |o, _| if o == 0x0300_0000 { 0 } else { o });
        e.register(EXTRA_IS_DEFAULT, |_, _| ret(1));
        e.register(LIST_COUNT, |e, a| {
            let mut n = 0;
            let mut node = a[0];
            while node != 0 {
                n += (e.mem.u32(node) != 0) as u32;
                node = e.mem.u32(node + 4);
            }
            ret(n)
        });
        let a = object(&mut e, 1, 5, 0);
        let b = object(&mut e, 2, 6, 0);
        let c = container(&mut e, &[a, b]);
        let a2 = object(&mut e, 1, 5, 0);
        let b2 = object(&mut e, 2, 6, 0);
        let same = container(&mut e, &[b2, a2]);
        let b3 = object(&mut e, 3, 6, 0);
        let changed = container(&mut e, &[a2, b3]);
        let more = container(&mut e, &[a2, b2, b3]);
        assert!(e.call(0x0048_1d10, &args![c, 0x0300_0000u32]).bool());
        assert!(!e.call(0x0048_1d10, &args![c, same]).bool());
        assert!(e.call(0x0048_1d10, &args![c, changed]).bool());
        assert!(e.call(0x0048_1d10, &args![c, more]).bool());
    }

    #[test]
    fn weight_sums_count_times_weight() {
        let mut e = engine();
        e.register(FORM_FLAG_20, |_, a| ret((a[0] == 7) as u32));
        e.register(ABSOLUTE_VALUE, |_, a| ret((a[0] as i32).unsigned_abs()));
        e.register(GET_FORM_WEIGHT, |_, a| {
            ret_f(if a[0] == 5 { 1.5 } else { 0.25 } + a[1] as f64)
        });
        let a = object(&mut e, -2, 5, 0);
        let b = object(&mut e, 4, 6, 0);
        let skipped = object(&mut e, 4, 7, 0);
        let c = container(&mut e, &[a, skipped, b]);
        assert_eq!(e.call(0x0048_1e10, &args![c, 0u8]).f32(), 4.0);
        assert_eq!(calls_to(&e, GET_FORM_WEIGHT)[0], vec![5, 0]);
        assert_eq!(e.call(0x0048_1e10, &args![c, 1u8]).f32(), 10.0);

        let c = container(&mut e, &[]);
        assert_eq!(e.call(0x0048_1e10, &args![c, 0u8]).f32(), 0.0);
    }

    #[test]
    fn has_object_finds_an_item() {
        let mut e = engine();
        let a = object(&mut e, 1, 5, 0);
        let b = object(&mut e, 1, 6, 0);
        let c = container(&mut e, &[a, b]);
        assert!(e.call(0x0048_1eb0, &args![c, 6u32]).bool());
        assert!(!e.call(0x0048_1eb0, &args![c, 7u32]).bool());
        let c = container(&mut e, &[]);
        assert!(!e.call(0x0048_1eb0, &args![c, 0u32]).bool());
    }

    #[test]
    fn gold_is_form_id_f() {
        let mut e = engine();
        e.register(WORD_AT_C, |e, a| ret(e.mem.u32(a[0] + 0x0c)));
        let form = e.mem.alloc(0x18);
        e.mem.set_u32(form + 0x0c, 0xf);
        assert!(e.call(0x0048_1f10, &args![form]).bool());
        e.mem.set_u32(form + 0x0c, 0x10);
        assert!(!e.call(0x0048_1f10, &args![form]).bool());
        assert!(!e.call(0x0048_1f10, &args![0u32]).bool());
        assert_eq!(calls_to(&e, WORD_AT_C).len(), 2);
    }

    #[test]
    fn can_hold_type_follows_the_jump_table() {
        let mut e = engine();
        let held: Vec<u32> = (0..=0xffu32)
            .filter(|t| e.call(0x0048_1f30, &args![*t as u8]).bool())
            .collect();
        assert_eq!(
            held,
            vec![
                0x18, 0x19, 0x1a, 0x1d, 0x1e, 0x1f, 0x28, 0x29, 0x2e, 0x2f, 0x31, 0x32, 0x34, 0x67,
                0x6c, 0x73, 0x74
            ]
        );
    }

    #[test]
    fn add_all_adds_every_item_of_the_other() {
        let mut e = engine();
        let a = object(&mut e, 2, 5, 0);
        let c = container(&mut e, &[a]);
        let x = object(&mut e, 3, 5, 0);
        let y = object(&mut e, 1, 6, 0);
        let other = container(&mut e, &[x, y]);
        e.call(0x0048_1fd0, &args![c, 0u32]);
        assert_eq!(e.mem.i32(a), 2);
        e.call(0x0048_1fd0, &args![c, other]);
        assert_eq!(e.mem.i32(a), 5);
        let all = items(&e, c);
        assert_eq!(all.len(), 2);
        assert_eq!(e.mem.u32(all[1] + 4), 6);
    }

    #[test]
    fn multiply_scales_every_count() {
        let mut e = engine();
        let a = object(&mut e, 2, 5, 0);
        let b = object(&mut e, -3, 6, 0);
        let c = container(&mut e, &[a, b]);
        e.call(0x0048_2040, &args![c, 4i32]);
        assert_eq!(e.mem.i32(a), 8);
        assert_eq!(e.mem.i32(b), -12);
    }

    #[test]
    fn set_health_gives_extras_and_frees_default_ones() {
        let mut e = engine();
        e.register(EXTRA_SET_HEALTH, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        e.register(EXTRA_IS_DEFAULT, |e, a| {
            ret((e.mem.f32(a[0] + 8) == 1.0) as u32)
        });
        let existing = e.mem.alloc(0x0c);
        let a = object(&mut e, 1, 5, existing);
        let b = object(&mut e, 1, 6, 0);
        let c = container(&mut e, &[a, b]);
        e.call(0x0048_2090, &args![c, 0.5f32]);
        assert_eq!(e.mem.u32(a + 8), existing);
        assert_eq!(e.mem.f32(existing + 8), 0.5);
        assert_ne!(e.mem.u32(b + 8), 0);
        e.call(0x0048_2090, &args![c, 1.0f32]);
        assert_eq!(e.mem.u32(a + 8), 0);
        assert_eq!(e.mem.u32(b + 8), 0);
        assert_eq!(calls_to(&e, OPERATOR_DELETE).len(), 2);
        // A signalling NaN is passed quieted, as FLD/FSTP leaves it.
        e.call(0x0048_2090, &args![c, f32::from_bits(0x7f80_0001)]);
        assert_eq!(calls_to(&e, EXTRA_SET_HEALTH)[4][1], 0x7fc0_0001);
    }

    const PLAYER_REF: u32 = 0x0500_0000;
    const PLAYER_VTABLE: u32 = 0x0500_1000;
    const ACTOR_ADD: u32 = 0x0061_0000;
    const TEMPORARY_DELETE: u32 = 0x0060_0000;

    fn give_engine(actor: bool) -> Engine {
        let mut e = engine();
        e.map(0x011d_e000, 0x1000);
        e.map(0x0101_6000, 0x1000);
        e.set_global(MESSAGE_TIME, 2.0f32);
        e.set_global(PLAYER, PLAYER_REF);
        if actor {
            cast_by(&mut e, |o, t| match t {
                TYPE_ACTOR => o,
                TYPE_TES_SCRIPTABLE_FORM if o == 9 => 0x0900_0000,
                TYPE_TES_FULL_NAME => o + 0x100,
                _ => 0,
            });
        } else {
            cast_by(&mut e, |o, t| match t {
                TYPE_TES_SCRIPTABLE_FORM if o == 9 => 0x0900_0000,
                _ => 0,
            });
        }
        e.register(WORD_AT_4, |e, a| {
            if a[0] == 0x0900_0000 {
                ret(0x0a00_0000)
            } else {
                ret(e.mem.u32(a[0] + 4))
            }
        });
        e.register(EXTRA_DATA_LIST_SET_COUNT, |_, _| Ret::default());
        e.register(GET_FORM_AS_HEALTH_FORM, |_, a| ret(a[0] + 0x1000));
        e.register(EXTRA_APPLY, |_, _| Ret::default());
        e.register(REFERENCE_ADD_ITEM, |_, _| Ret::default());
        e.register(EXTRA_DATA_LIST_GET_SCRIPT, |_, _| ret(0));
        e.register(EXTRA_DATA_LIST_SET_SCRIPT, |_, _| Ret::default());
        e.register(SCRIPT_MAKE_LOCALS, |_, _| ret(0x0b00_0000));
        e.register(EXTRA_DATA_LIST_SET_LOCALS, |_, _| Ret::default());
        e.register(TES_OBJECT_REFR_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x0c00_0000);
            ret(a[0])
        });
        e.put_vtable(0x0c00_0000, &[0, 0, 0, 0, TEMPORARY_DELETE]);
        e.register(TEMPORARY_DELETE, |_, _| Ret::default());
        e.register(TES_FORM_SET_TEMPORARY, |_, _| Ret::default());
        e.register(EXTRA_DATA_LIST_GET_SCRIPT_LOCALS, |_, _| ret(0x0d00_0000));
        e.register(SCRIPT_RUN, |_, _| Ret::default());
        e.register(BS_STRING_CONSTRUCT, |_, _| Ret::default());
        e.register(BS_STRING_DESTRUCT, |_, _| Ret::default());
        e.register(BS_STRING_FORMAT, |_, _| Ret::default());
        e.register(BS_STRING_TEXT, |_, _| ret(0x0e00_0000));
        e.register(SETTING_STRING, |_, a| ret(a[0] + 4));
        e.register(FULL_NAME_TEXT, |_, a| ret(a[0] + 1));
        e.register(FORM_TYPE, |_, a| ret(if a[0] == 5 { 0x28 } else { 0x20 }));
        e.register(GET_PICK_UP_SOUND_NAME, |_, _| ret(0x0f00_0000));
        e.register(SHOW_MESSAGE, |_, _| Ret::default());
        // The player's vtable, with the add-item slot.
        e.put_vtable(PLAYER_VTABLE, &[0; 0x65]);
        e.mem
            .set_u32(PLAYER_VTABLE + ACTOR_ADD_ITEM_SLOT, ACTOR_ADD);
        e.register(ACTOR_ADD, |_, _| Ret::default());
        e.map(PLAYER_REF, 0x10);
        e.mem.set_u32(PLAYER_REF, PLAYER_VTABLE);
        e
    }

    #[test]
    fn give_items_to_a_reference_and_the_player() {
        // A plain reference: one call per item with its count.
        let mut e = give_engine(false);
        let a = object(&mut e, 3, 5, 0x30);
        let b = object(&mut e, 2, 6, 0);
        let c = container(&mut e, &[a, b]);
        let target = e.mem.alloc(0x68);
        e.call(0x0048_21a0, &args![c, target, 0u8]);
        let adds = calls_to(&e, REFERENCE_ADD_ITEM);
        assert_eq!(adds.len(), 2);
        let list = adds[0][2];
        assert_ne!(list, 0);
        assert_eq!(adds[0], vec![target, 5, list, 3]);
        assert_eq!(adds[1], vec![target, 6, 0, 2]);
        assert_eq!(calls_to(&e, EXTRA_DATA_LIST_SET_COUNT)[0], vec![list, 3]);
        assert_eq!(calls_to(&e, EXTRA_APPLY)[0], vec![0x30, list, 0x1005]);
        assert!(calls_to(&e, SHOW_MESSAGE).is_empty());

        // The player: a scripted item once per count, then the message.
        let mut e = give_engine(true);
        let s = object(&mut e, 2, 9, 0);
        let last = object(&mut e, 4, 5, 0);
        let c = container(&mut e, &[s, last]);
        e.call(0x0048_21a0, &args![c, PLAYER_REF, 0u8]);
        let adds = calls_to(&e, ACTOR_ADD);
        assert_eq!(adds.len(), 3);
        assert_eq!(adds[0][..2], [PLAYER_REF, 9]);
        assert_eq!(adds[0][3], 1);
        assert_eq!(adds[2], vec![PLAYER_REF, 5, 0, 4]);
        let runs = calls_to(&e, SCRIPT_RUN);
        assert_eq!(runs.len(), 2);
        assert_eq!(runs[0][0], 0x0a00_0000);
        assert_eq!(runs[0][2..], [0x0d00_0000, 0, 0]);
        assert_eq!(calls_to(&e, TEMPORARY_DELETE).len(), 2);
        assert_eq!(
            calls_to(&e, BS_STRING_FORMAT)[0][1..],
            [
                FORMAT_COUNT_NAME,
                4,
                5 + 0x101,
                SETTING_PLURAL + 4,
                SETTING_ADD_ITEM_TO_INVENTORY + 4
            ]
        );
        assert_eq!(
            calls_to(&e, GET_PICK_UP_SOUND_NAME)[0],
            vec![PLAYER_REF, 5, 1, 0]
        );
        assert_eq!(
            calls_to(&e, SHOW_MESSAGE)[0],
            vec![0x0e00_0000, 0, GIFT_ICON, 0x0f00_0000, 2.0f32.to_bits(), 0]
        );

        // Quiet: no message.
        let mut e = give_engine(true);
        let one = object(&mut e, 1, 6, 0);
        let c = container(&mut e, &[one]);
        e.call(0x0048_21a0, &args![c, PLAYER_REF, 1u8]);
        assert!(calls_to(&e, BS_STRING_CONSTRUCT).is_empty());
        // Count 1 and a type outside the switch: the text, no message.
        e.call(0x0048_21a0, &args![c, PLAYER_REF, 0u8]);
        assert_eq!(
            calls_to(&e, BS_STRING_FORMAT)[0][1..],
            [FORMAT_NAME, 6 + 0x101, SETTING_ADD_ITEM_TO_INVENTORY + 4]
        );
        assert!(calls_to(&e, SHOW_MESSAGE).is_empty());
        assert_eq!(calls_to(&e, BS_STRING_DESTRUCT).len(), 1);
    }

    #[test]
    fn script_answers_the_scriptable_component_word() {
        let mut e = engine();
        cast_by(&mut e, |o, _| if o == 0x0300_0000 { o + 0x40 } else { 0 });
        e.register(WORD_AT_4, |_, a| ret(a[0] + 0x1000));
        let script = e.call(0x0048_26d0, &args![0x0300_0000u32]).u32();
        assert_eq!(script, 0x0300_1040);
        assert_eq!(
            calls_to(&e, RT_DYNAMIC_CAST)[0],
            vec![0x0300_0000, 0, TYPE_TES_FORM, TYPE_TES_SCRIPTABLE_FORM, 0]
        );
        assert_eq!(e.call(0x0048_26d0, &args![0x0400_0000u32]).u32(), 0);
        assert_eq!(calls_to(&e, WORD_AT_4).len(), 1);
    }

    #[test]
    fn full_name_or_the_empty_string() {
        let mut e = engine();
        cast_by(&mut e, |o, _| if o == 0x0300_0000 { o + 0x30 } else { 0 });
        e.register(FULL_NAME_TEXT, |_, a| ret(a[0] + 1));
        assert_eq!(
            e.call(0x0048_2720, &args![0x0300_0000u32]).u32(),
            0x0300_0031
        );
        assert_eq!(
            calls_to(&e, RT_DYNAMIC_CAST)[0],
            vec![0x0300_0000, 0, TYPE_TES_FORM, TYPE_TES_FULL_NAME, 0]
        );
        assert_eq!(
            e.call(0x0048_2720, &args![0x0400_0000u32]).u32(),
            EMPTY_STRING
        );
    }

    fn changes_engine(first: bool, second: bool, third: bool) -> Engine {
        let mut e = engine();
        e.map(0x011d_d000, 0x2000);
        e.set_global(SINGLETON_011DEA0C, 0x0710_0000u32);
        e.set_global(SINGLETON_011DDF38, 0x0720_0000u32);
        e.register(EXTRA_DATA_LIST_0041D280, |_, _| Ret::default());
        e.register(EXTRA_DATA_LIST_SET_COUNT, |_, _| Ret::default());
        e.register(GET_FORM_AS_HEALTH_FORM, |_, a| ret(a[0] + 0x1000));
        e.register(EXTRA_APPLY, |_, _| Ret::default());
        e.register(INVENTORY_CHANGES_ADD, |_, _| Ret::default());
        e.register_double(QUERY_004F1540, move |_, _| ret(first as u32));
        e.register_double(QUERY_0042CE10, move |_, _| ret(second as u32));
        e.register_double(QUERY_00452E90, move |_, _| ret(third as u32));
        e.register(TES_FORM_FORCE_CHANGE, |_, _| Ret::default());
        e
    }

    #[test]
    fn add_to_inventory_changes_and_mark_the_change() {
        let mut e = changes_engine(true, false, false);
        let a = object(&mut e, 0x1_0003, 5, 0x30);
        let c = container(&mut e, &[a]);
        let changes = e.mem.alloc(0x10);
        e.mem.set_u32(changes + 4, 0x0730_0000);
        e.call(0x0048_2770, &args![c, 0x99u32, 0u32]);
        assert!(calls_to(&e, INVENTORY_CHANGES_ADD).is_empty());
        e.call(0x0048_2770, &args![c, 0x99u32, changes]);
        let add = &calls_to(&e, INVENTORY_CHANGES_ADD)[0];
        let list = add[2];
        assert_eq!(*add, vec![changes, 5, list, 0x1_0003]);
        assert_eq!(calls_to(&e, EXTRA_DATA_LIST_0041D280)[0], vec![list, 0x99]);
        assert_eq!(calls_to(&e, EXTRA_DATA_LIST_SET_COUNT)[0], vec![list, 3]);
        assert_eq!(calls_to(&e, EXTRA_APPLY)[0], vec![0x30, list, 0x1005]);
        assert_eq!(calls_to(&e, QUERY_004F1540)[0], vec![0x0710_0000]);
        assert_eq!(
            calls_to(&e, TES_FORM_FORCE_CHANGE)[0],
            vec![0x0730_0000, CHANGE_FLAG_INVENTORY]
        );

        for (first, second, third, marks) in [
            (false, false, false, 0),
            (true, true, false, 0),
            (true, true, true, 1),
        ] {
            let mut e = changes_engine(first, second, third);
            let a = object(&mut e, 1, 5, 0);
            let c = container(&mut e, &[a]);
            let changes = e.mem.alloc(0x10);
            e.mem.set_u32(changes + 4, 0x0730_0000);
            e.call(0x0048_2770, &args![c, 0u32, changes]);
            assert_eq!(calls_to(&e, TES_FORM_FORCE_CHANGE).len(), marks);
        }

        // No item added: nothing is marked.
        let mut e = changes_engine(true, false, false);
        let c = container(&mut e, &[]);
        let changes = e.mem.alloc(0x10);
        e.call(0x0048_2770, &args![c, 0u32, changes]);
        assert!(calls_to(&e, QUERY_004F1540).is_empty());
    }

    fn desirability_engine() -> Engine {
        let mut e = engine();
        e.map(0x0101_5000, 0x1000);
        e.set_global(LOWEST_FLOAT, -f32::MAX);
        // Objects 0x10 and up cast; 0x40 has a NaN value.
        cast_by(&mut e, |o, _| if o >= 0x10 { o } else { 0 });
        e.register(GET_DESIRABILITY, |_, a| {
            ret_f(if a[1] == 0x40 { f64::NAN } else { a[1] as f64 })
        });
        e.register(FILLS_BIPED_SLOT, |_, a| ret((a[0] != 0x30 + 0x70) as u32));
        e
    }

    #[test]
    fn most_desirable_weapon_is_the_highest_value() {
        let mut e = desirability_engine();
        let a = object(&mut e, 1, 0x20, 0);
        let b = object(&mut e, 1, 0x40, 0);
        let d = object(&mut e, 1, 0x30, 0);
        let g = object(&mut e, 1, 5, 0);
        let c = container(&mut e, &[a, b, d, g]);
        assert_eq!(e.call(0x0048_2910, &args![c, 0x77u32]).u32(), 0x30);
        assert_eq!(calls_to(&e, GET_DESIRABILITY)[0], vec![0x77, 0x20]);
        assert_eq!(
            calls_to(&e, RT_DYNAMIC_CAST)[0],
            vec![0x20, 0, TYPE_TES_BOUND_OBJECT, TYPE_TES_OBJECT_WEAP, 0]
        );
        let c = container(&mut e, &[g]);
        assert_eq!(e.call(0x0048_2910, &args![c, 0x77u32]).u32(), 0);
    }

    #[test]
    fn most_desirable_armor_fills_the_slot() {
        let mut e = desirability_engine();
        let a = object(&mut e, 1, 0x20, 0);
        let d = object(&mut e, 1, 0x30, 0);
        let c = container(&mut e, &[a, d]);
        assert_eq!(e.call(0x0048_29c0, &args![c, 0x77u32, 2i32]).u32(), 0x20);
        assert_eq!(
            calls_to(&e, FILLS_BIPED_SLOT)[0],
            vec![0x20 + 0x70, 2, 0, 0]
        );
        assert_eq!(
            calls_to(&e, RT_DYNAMIC_CAST)[0],
            vec![0x20, 0, TYPE_TES_BOUND_OBJECT, TYPE_TES_OBJECT_ARMO, 0]
        );
        assert_eq!(e.call(0x0048_29c0, &args![c, 0x77u32, -1i32]).u32(), 0);
        assert_eq!(calls_to(&e, FILLS_BIPED_SLOT).len(), 2);
    }

    #[test]
    fn object_count_sums_matching_items() {
        let mut e = engine();
        let a = object(&mut e, 2, 5, 0);
        let b = object(&mut e, 4, 6, 0);
        let d = object(&mut e, 3, 5, 0);
        let c = container(&mut e, &[a, b, d]);
        assert_eq!(e.call(0x0048_2a90, &args![c, 5u32]).i32(), 5);
        assert_eq!(e.call(0x0048_2a90, &args![c, 7u32]).i32(), 0);
    }
}
