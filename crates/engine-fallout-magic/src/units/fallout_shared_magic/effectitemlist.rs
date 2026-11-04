//! `fallout shared/magic/effectitemlist.cpp` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00402c90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00402c90 - the exception-unwinding frame (FS:[0]) is not translated.
/// `EffectItemList::EffectItemList` (the Xbox PDB name the engine map gives
/// it). Runs the constructor `00402d00` on the object (it sets a base vtable
/// and constructs a string at `+4`, see `run_00402d00.rs`), `004056a0` on
/// the member at `+0xc`, then stores the vtable pointers `01011d0c` at `+0`
/// and `01011cf4` at `+0xc` (the RTTI name of both is `.?AVMagicItem@@`).
/// Returns `this`. The exception-unwinding frame is not translated.
pub fn effect_item_list_effect_item_list(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0040_2d00, &args![this]);
    e.call(0x0040_56a0, &args![this.byte_add(0xc)]);
    e.mem.set_u32(this.addr(), 0x0101_1d0c);
    e.mem.set_u32(this.addr() + 0xc, 0x0101_1cf4);
    this
}

// Translated from 004059d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00403d50` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004059d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0040_3d50, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00406e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StatsMenu::StatusDataList` scalar deleting destructor (engine map name): runs the destructor body `00406de0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn stats_menu_status_data_list_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0040_6de0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

layout! {
    /// `EffectItemList` (Xbox PDB), 0x10 bytes: the vtable, the
    /// `BSSimpleList<EffectItem *>` (+4, its first node embedded) and
    /// `iHostileCount`.
    pub struct EffectItemList: 0x10 {
        /// `iHostileCount` (Xbox PDB).
        0x0C iHostileCount: i32,
    }
}

/// `EffectItemList`'s vtable (RTTI `.?AVEffectItemList@@`).
const EFFECT_ITEM_LIST_VTABLE: u32 = 0x0101_2508;
/// List constructor (`BSSimpleList`), thiscall.
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// Thiscall on a list: `AL`, whether it is empty.
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// Thiscall: returns `this` (on a list node: the address of its item).
const SELF: u32 = 0x0068_15c0;
/// Thiscall: the word at +4 (on a list node: the next node).
const NEXT_NODE: u32 = 0x0072_6070;
/// Thiscall on the list (address of the item word), `RET 4`: appends it.
const LIST_APPEND: u32 = 0x0090_5820;
/// Thiscall on the list: removes its first node.
const LIST_REMOVE_FIRST: u32 = 0x0063_f7b0;
/// Thiscall on an effect item: its effect setting (the word the code reads
/// through `00825c00`, folded code).
const ITEM_EFFECT_SETTING: u32 = 0x0082_5c00;
/// Thiscall: the word at +0xc (on an effect item: `EffectItemData::iRange`).
const WORD_AT_C: u32 = 0x0084_e3a0;
/// `EffectItem::IsUsageMonitorEffect` (`00404730`), `EffectItem::IsHostile`
/// (`00404710`), `EffectItem::GetArea` (`00404420`) and
/// `EffectItem::GetAssociatedSkill` (`00404650`) (engine map), thiscall.
const ITEM_IS_USAGE_MONITOR: u32 = 0x0040_4730;
const ITEM_IS_HOSTILE: u32 = 0x0040_4710;
const ITEM_GET_AREA: u32 = 0x0040_4420;
const ITEM_GET_ASSOCIATED_SKILL: u32 = 0x0040_4650;
/// `EffectItem::GetCost` (engine map), thiscall (actor), `RET 4`: a float
/// in `ST0`.
const ITEM_GET_COST: u32 = 0x0040_3f80;
/// `EffectItem::GetEffectName` (engine map), thiscall (result string),
/// `RET 4`: the result.
const ITEM_GET_EFFECT_NAME: u32 = 0x0040_4670;
/// `EffectItem::Load` (engine map), thiscall (file, name), `RET 8`: `AL`.
const ITEM_LOAD: u32 = 0x0040_4120;
/// Thiscall on an effect setting (string), `RET 4`: the string (its name,
/// `effectitem.cpp`).
const SETTING_NAME_004046B0: u32 = 0x0040_46b0;
/// Thiscall on an effect setting (flag mask), `RET 4`: `AL`.
const SETTING_HAS_FLAG: u32 = 0x0040_3e60;
/// Thiscall: `this + 0x1c` (on an effect item: its conditions).
const PLUS_1C: u32 = 0x0040_9140;
/// `TESCondition::IsTrue_ov2` (engine map), thiscall (object, word, address
/// of a result byte, word), `RET 0x10`: `AL`.
const CONDITION_IS_TRUE: u32 = 0x0068_0c60;
/// The `EffectItem` constructor (`run_00403b30.rs`), thiscall (effect
/// setting), `RET 4`.
const EFFECT_ITEM_CONSTRUCT: u32 = 0x0040_3b30;
/// Thiscall on an effect setting: `AL` (`run_00403b30.rs`).
const SETTING_HAS_ACTOR_VALUE: u32 = 0x0040_3c60;
/// Thiscall on an effect setting: the word at +0x9c (folded code).
const WORD_AT_9C: u32 = 0x0058_6150;
/// Thiscall on an effect setting: the word at +0x98 (`magicballprojectile.rs`).
const WORD_AT_98: u32 = 0x0081_0fd0;
/// Cdecl (index, mask): `AL` (`run_00403b30.rs`).
const TABLE_HAS_FLAG: u32 = 0x0040_3c80;
/// `EffectSettingCollection::GetUsageMonitorEffectSetting` (Xbox PDB), cdecl.
const GET_USAGE_MONITOR_EFFECT_SETTING: u32 = 0x0040_8b50;
/// Cdecl (float): a float in `ST0` (`effectitem.cpp`; `GetCost` passes the
/// total through it).
const COST_ROUND_00404040: u32 = 0x0040_4040;
/// The compiler's float to integer conversion (`_ftol2`): the value in
/// `ST0` (a leading `f64`), the integer in `EAX`.
const FTOL2: u32 = 0x00ec_62c0;
/// Cdecl (text, ...): the warning print.
const PRINT_WARNING: u32 = 0x005b_5e40;
/// The allocator, cdecl (size).
const ALLOCATE: u32 = 0x0040_1000;
/// `TESFile::GetTESChunk` (`004726b0`) and `TESFile::GetChunkData`
/// (`004727f0`, address of the buffer, `RET 4`) (engine map), thiscall.
const FILE_GET_CHUNK: u32 = 0x0047_26b0;
const FILE_GET_CHUNK_DATA: u32 = 0x0047_27f0;
/// Thiscall on the file: moves to the next chunk (`004726f0`).
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
/// `TESForm::AddCompileIndex` (engine map), cdecl (address of the form id,
/// file).
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// Cdecl (form id): the form with that id, or 0 (`tesform.cpp`).
const LOOK_UP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (CRT), cdecl (object, offset, source type, target type,
/// is-reference), and the descriptors of `TESForm` and `EffectSetting`.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_EFFECT_SETTING: u32 = 0x0118_37c4;
/// Thiscall on a string: its characters.
const STRING_TEXT: u32 = 0x0055_9450;
/// The 8-byte string destructor, thiscall.
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// The `StatsMenu::StatusDataList` constructor (`00406db0`), `Register`
/// (`007e0b20`, item, `RET 4`) and `Flush` (`00406de0`) (engine map),
/// thiscall on the 12-byte list.
const STATUS_LIST_CONSTRUCT: u32 = 0x0040_6db0;
const STATUS_LIST_REGISTER: u32 = 0x007e_0b20;
const STATUS_LIST_FLUSH: u32 = 0x0040_6de0;
/// Cdecl (buffer, size, text): copies text into the buffer.
const BUFFER_COPY_00406D30: u32 = 0x0040_6d30;
/// Cdecl (buffer, size, text): appends text to the buffer.
const BUFFER_APPEND_00406D50: u32 = 0x0040_6d50;
/// Cdecl (buffer, size, format, ...): formats into the buffer.
const BUFFER_FORMAT: u32 = 0x0040_6d00;
/// Cdecl (value, mask): `AL` (`run_00406cc0.rs`).
const VALUE_HAS_FLAG_00406D70: u32 = 0x0040_6d70;
/// Cdecl (float): a float in `ST0` (`run_00406cc0.rs`).
const FLOAT_FUNCTION_00406CC0: u32 = 0x0040_6cc0;
/// Cdecl (float): an integer in `EAX` (`run_00406cc0.rs`).
const FLOAT_TO_INT_00406D90: u32 = 0x0040_6d90;
/// Thiscall on the player: a float in `ST0` (`actor.cpp`; the medicine and
/// the food multipliers `BuildMenuString` applies).
const PLAYER_MEDICINE_MULT_008BEBB0: u32 = 0x008b_ebb0;
const PLAYER_FOOD_MULT_008BEC80: u32 = 0x008b_ec80;
/// `Actor::GetRadiationResistanceMult` (engine map), thiscall: `ST0`.
const PLAYER_RADIATION_RESISTANCE_MULT: u32 = 0x008c_4330;
/// `BGSEntryPoint::HandleEntryPoint` (engine map), cdecl (entry point, actor,
/// address of the float).
const HANDLE_ENTRY_POINT: u32 = 0x005e_58f0;
/// `ActorValue::GetActorValueAbbreviation` (engine map), cdecl (index).
const ACTOR_VALUE_ABBREVIATION: u32 = 0x0066_eb00;
/// The player.
const PLAYER_011DEA3C: u32 = 0x011d_ea3c;
/// The effect item `Load` made last (the conditions that follow are its).
const CURRENT_EFFECT_ITEM: u32 = 0x011c_33d4;
/// `"{unknown}"`.
const UNKNOWN_TEXT: u32 = 0x0101_20d8;
/// The `Load` warnings.
const NO_EFFECT_ITEM_TEXT: u32 = 0x0101_2520;
const NO_EFFECT_SETTING_TEXT: u32 = 0x0101_2578;
const LOAD_FAILED_TEXT: u32 = 0x0101_25c0;
/// `GetEffectItemIndex`'s warning.
const NOT_IN_LIST_TEXT: u32 = 0x0101_2658;
/// `"\n"`, `", "`, `"%s %+i"`, `"%s %+i(%d%c)"`, `"%s %+i(%.1f%c)"`.
const NEWLINE_TEXT: u32 = 0x0101_2648;
const COMMA_TEXT: u32 = 0x0101_2630;
const VALUE_FORMAT: u32 = 0x0101_2608;
const VALUE_WHOLE_DURATION_FORMAT: u32 = 0x0101_2610;
const VALUE_DURATION_FORMAT: u32 = 0x0101_2620;
/// `0.0`, `3600.0` and `60.0` (`double`).
const ZERO_DOUBLE: u32 = 0x0101_2060;
const HOUR_DOUBLE: u32 = 0x0101_2640;
const MINUTE_DOUBLE: u32 = 0x0101_2638;
/// The chunk tags `Load` handles: `"CTDA"` (a condition) and `"EFID"` (an
/// effect).
const CHUNK_CTDA: u32 = 0x4144_5443;
const CHUNK_EFID: u32 = 0x4449_4645;

/// The node after `cursor`'s embedded node, as the code walks the list:
/// `00726070` on `cursor + 4`, then back by 4 to the next "list" (0 at the
/// end).
fn next_cursor(e: &mut Engine, cursor: u32) -> u32 {
    let next = e.call(NEXT_NODE, &args![cursor + 4]).u32();
    if next == 0 {
        0
    } else {
        next - 4
    }
}

/// The item of `cursor`'s node (`006815c0` on `cursor + 4`, then the word).
fn cursor_item(e: &mut Engine, cursor: u32) -> u32 {
    let slot = e.call(SELF, &args![cursor + 4]).u32();
    e.mem.u32(slot)
}

// Translated from 004056a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `EffectItemList` constructor (unnamed in the engine map): the list
/// (+4, `0096a2d0`), the vtable, `iHostileCount` 0. Returns `this`.
pub fn fn_004056a0(e: &mut Engine, this: Ptr<EffectItemList>) -> Ptr<EffectItemList> {
    e.call(LIST_CONSTRUCT, &args![this.addr() + 4]);
    e.mem.set_u32(this.addr(), EFFECT_ITEM_LIST_VTABLE);
    e.set(this, EffectItemList::iHostileCount, 0);
    this
}

/// A name for the warnings: `name`, or `"{unknown}"` when it is null.
fn name_or_unknown(name: u32) -> u32 {
    if name == 0 {
        UNKNOWN_TEXT
    } else {
        name
    }
}

// Translated from 00405760 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00405760 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the new effect item and the name string) is not translated.
/// `EffectItemList::Load` (Xbox PDB): reads the current chunk of `file`. A
/// `"CTDA"` chunk loads into the effect item read last (`011c33d4`; counts
/// it hostile when it is), or warns when there is none. An `"EFID"` chunk
/// reads the effect setting's form id, adds the file's compile index and
/// looks it up; without an `EffectSetting` it warns. Otherwise a new effect
/// item (0x24 bytes, `00403b30`) is made for it, the file moves on
/// (`004726f0`), and the item loads (`EffectItem::Load`): added on success
/// (`AddEffect`); on failure the warning names the setting, the item is
/// deleted (`004059d0` with 1) and 0 kept. The item (or 0) becomes the
/// current one.
pub fn effect_item_list_load(e: &mut Engine, this: Ptr<EffectItemList>, file: u32, name: u32) {
    let chunk = e.call(FILE_GET_CHUNK, &args![file]).u32();
    if chunk == CHUNK_CTDA {
        let current = e.mem.u32(CURRENT_EFFECT_ITEM);
        if current == 0 {
            e.call(
                PRINT_WARNING,
                &args![NO_EFFECT_ITEM_TEXT, name_or_unknown(name)],
            );
            return;
        }
        e.call(ITEM_LOAD, &args![current, file, name]);
        let current = e.mem.u32(CURRENT_EFFECT_ITEM);
        if e.call(ITEM_IS_HOSTILE, &args![current]).bool() {
            let count = e.get(this, EffectItemList::iHostileCount);
            e.set(this, EffectItemList::iHostileCount, count.wrapping_add(1));
        }
        return;
    }
    if chunk != CHUNK_EFID {
        return;
    }
    let (form_id, setting) = e.with_stack(4, |e, id| {
        e.call(FILE_GET_CHUNK_DATA, &args![file, id]);
        e.call(ADD_COMPILE_INDEX, &args![id, file]);
        let form_id = e.mem.u32(id.addr());
        let form = e.call(LOOK_UP_FORM, &args![form_id]).u32();
        let setting = e
            .call(
                DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_EFFECT_SETTING, 0u32],
            )
            .u32();
        (form_id, setting)
    });
    if setting == 0 {
        e.call(
            PRINT_WARNING,
            &args![NO_EFFECT_SETTING_TEXT, form_id, name_or_unknown(name)],
        );
        return;
    }
    let block = e.call(ALLOCATE, &args![0x24u32]).u32();
    let mut item = if block != 0 {
        e.call(EFFECT_ITEM_CONSTRUCT, &args![block, setting]).u32()
    } else {
        0
    };
    e.call(FILE_NEXT_CHUNK, &args![file]);
    // The code clears a local here that it never reads (when the setting
    // has an actor value and that is 0x36).
    if e.call(SETTING_HAS_ACTOR_VALUE, &args![setting]).bool() {
        e.call(WORD_AT_9C, &args![setting]);
    }
    if e.call(ITEM_LOAD, &args![item, file, name]).bool() {
        effect_item_list_add_effect(e, this, item);
    } else {
        let effect_name = name_or_unknown(name);
        e.with_stack(8, |e, text| {
            let name = e.call(SETTING_NAME_004046B0, &args![setting, text]).u32();
            let characters = e.call(STRING_TEXT, &args![name]).u32();
            e.call(
                PRINT_WARNING,
                &args![LOAD_FAILED_TEXT, characters, effect_name],
            );
            e.call(STRING_DESTRUCT, &args![text]);
        });
        if item != 0 {
            fn_004059d0(e, Ptr::new(item), 1);
        }
        item = 0;
    }
    e.mem.set_u32(CURRENT_EFFECT_ITEM, item);
}

// Translated from 00405d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectItemList::AddEffect` (Xbox PDB): unless `item` is a usage monitor
/// effect and the list already has one, appends it (`00905820` with the
/// address of the argument) and counts it hostile when it is.
pub fn effect_item_list_add_effect(e: &mut Engine, this: Ptr<EffectItemList>, item: u32) {
    if e.call(ITEM_IS_USAGE_MONITOR, &args![item]).bool()
        && effect_item_list_get_usage_monitor_effect(e, this) != 0
    {
        return;
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), item);
        e.call(LIST_APPEND, &args![this.addr() + 4, slot]);
    });
    if e.call(ITEM_IS_HOSTILE, &args![item]).bool() {
        let count = e.get(this, EffectItemList::iHostileCount);
        e.set(this, EffectItemList::iHostileCount, count.wrapping_add(1));
    }
}

// Translated from 00405d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectItemList::IsHostile` (Xbox PDB): `iHostileCount > 0`.
pub fn effect_item_list_is_hostile(e: &mut Engine, this: Ptr<EffectItemList>) -> bool {
    e.get(this, EffectItemList::iHostileCount) > 0
}

// Translated from 00405da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectItemList::CanBePoison` (Xbox PDB): false for an empty list;
/// otherwise false as soon as an effect is neither flagged `0x400000`
/// (`00405e40`) nor hostile, true when none is.
pub fn effect_item_list_can_be_poison(e: &mut Engine, this: Ptr<EffectItemList>) -> bool {
    let mut cursor = this.addr();
    if e.call(LIST_IS_EMPTY, &args![cursor + 4]).bool() {
        return false;
    }
    while cursor != 0 {
        let item = cursor_item(e, cursor);
        if item != 0
            && !fn_00405e40(e, Ptr::new(item))
            && !e.call(ITEM_IS_HOSTILE, &args![item]).bool()
        {
            return false;
        }
        cursor = next_cursor(e, cursor);
    }
    true
}

// Translated from 00405e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on an effect item): `00405e60` on
/// its effect setting (+0x14); the result passes through `EAX`.
pub fn fn_00405e40(e: &mut Engine, this: Ptr) -> bool {
    let setting = e.mem.u32(this.addr() + 0x14);
    fn_00405e60(e, Ptr::new(setting))
}

// Translated from 00405e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on an effect setting): whether it
/// has flag `0x400000` (`00403e60`; the result passes through `EAX`).
pub fn fn_00405e60(e: &mut Engine, this: Ptr) -> bool {
    e.call(SETTING_HAS_FLAG, &args![this, 0x0040_0000u32])
        .bool()
}

/// `HasTargetEffect` and `HasTouchEffect`: whether an effect of range
/// `range` (+0xc) is not flagged `0x400000`; false for an empty list.
fn has_effect_of_range(e: &mut Engine, this: Ptr<EffectItemList>, range: u32) -> bool {
    let mut cursor = this.addr();
    if e.call(LIST_IS_EMPTY, &args![cursor + 4]).bool() {
        return false;
    }
    let mut found = false;
    while cursor != 0 && !found {
        let item = cursor_item(e, cursor);
        if item != 0
            && e.call(WORD_AT_C, &args![item]).u32() == range
            && !fn_00405e40(e, Ptr::new(item))
        {
            found = true;
        }
        cursor = next_cursor(e, cursor);
    }
    found
}

// Translated from 00405f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectItemList::HasTargetEffect` (Xbox PDB): an effect of range 2.
pub fn effect_item_list_has_target_effect(e: &mut Engine, this: Ptr<EffectItemList>) -> bool {
    has_effect_of_range(e, this, 2)
}

// Translated from 00405fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectItemList::HasTouchEffect` (Xbox PDB): an effect of range 1.
pub fn effect_item_list_has_touch_effect(e: &mut Engine, this: Ptr<EffectItemList>) -> bool {
    has_effect_of_range(e, this, 1)
}

// Translated from 00406140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): whether an effect's
/// setting has a flag of `mask` (`00403e60`) and the effect is not flagged
/// `0x400000`; false for an empty list.
pub fn fn_00406140(e: &mut Engine, this: Ptr<EffectItemList>, mask: u32) -> bool {
    let mut cursor = this.addr();
    if e.call(LIST_IS_EMPTY, &args![cursor + 4]).bool() {
        return false;
    }
    let mut found = false;
    while cursor != 0 && !found {
        let item = cursor_item(e, cursor);
        if item != 0 {
            let setting = e.call(ITEM_EFFECT_SETTING, &args![item]).u32();
            if e.call(SETTING_HAS_FLAG, &args![setting, mask]).bool()
                && !fn_00405e40(e, Ptr::new(item))
            {
                found = true;
            }
        }
        cursor = next_cursor(e, cursor);
    }
    found
}

// Translated from 00406200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectItemList::GetUsageMonitorEffect` (Xbox PDB): the first effect
/// whose setting is the usage monitor effect setting (`00408b50`), or 0.
pub fn effect_item_list_get_usage_monitor_effect(e: &mut Engine, this: Ptr<EffectItemList>) -> u32 {
    let mut found = 0;
    let mut cursor = this.addr();
    while cursor != 0 && !e.call(LIST_IS_EMPTY, &args![cursor + 4]).bool() && found == 0 {
        let item = cursor_item(e, cursor);
        let setting = e.call(ITEM_EFFECT_SETTING, &args![item]).u32();
        let monitor = e.call(GET_USAGE_MONITOR_EFFECT_SETTING, &args![]).u32();
        if setting == monitor {
            found = item;
        } else {
            cursor = next_cursor(e, cursor);
        }
    }
    found
}

// Translated from 00406340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: deletes every effect (`004059d0` with 1) and
/// removes its node (`0063f7b0`); a node without an effect is stepped over.
pub fn fn_00406340(e: &mut Engine, this: Ptr<EffectItemList>) {
    let mut cursor = this.addr();
    while cursor != 0 {
        let item = cursor_item(e, cursor);
        if item != 0 {
            // The code tests the item for null again before the delete.
            if item != 0 {
                fn_004059d0(e, Ptr::new(item), 1);
            }
            e.call(LIST_REMOVE_FIRST, &args![cursor + 4]);
        } else {
            cursor = next_cursor(e, cursor);
        }
    }
}

// Translated from 004063e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectItemList::GetCost` (Xbox PDB; vtable +8): 0.0 for an empty list;
/// otherwise the sum of the costs (`EffectItem::GetCost(actor)`) of the
/// effects not flagged `0x400000`, 0.0 when that is negative, else through
/// `00404040`.
pub fn effect_item_list_get_cost(e: &mut Engine, this: Ptr<EffectItemList>, actor: u32) -> f32 {
    let mut cursor = this.addr();
    if e.call(LIST_IS_EMPTY, &args![cursor + 4]).bool() {
        return 0.0;
    }
    let mut total = 0.0f32.to_bits();
    while cursor != 0 {
        let item = cursor_item(e, cursor);
        if !fn_00405e40(e, Ptr::new(item)) {
            let cost = e.call(ITEM_GET_COST, &args![item, actor]).f64();
            total = x87_store32(x87_add(cost, x87_m32(total)));
        }
        cursor = next_cursor(e, cursor);
    }
    let zero: f64 = e.global(ZERO_DOUBLE);
    // `FCOMP; TEST AH,5; JP`: 0.0 only when the total is below zero
    // (ordered).
    let result = if x87_load32(total) < zero {
        0.0f32.to_bits()
    } else {
        let rounded = e
            .call(COST_ROUND_00404040, &args![x87_quiet_bits(total)])
            .f64();
        x87_store32(rounded)
    };
    f32::from_bits(x87_quiet_bits(result))
}

// Translated from 004064b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on an effect setting): whether its
/// row of the `00403c80` table (index `00810fd0`) has a flag of `0x7c`.
pub fn fn_004064b0(e: &mut Engine, this: Ptr) -> bool {
    let index = e.call(WORD_AT_98, &args![this]).u32();
    e.call(TABLE_HAS_FLAG, &args![index, 0x7cu32]).bool()
}

// Translated from 004064d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectItemList::GetCostliestEffect` (Xbox PDB): of the effects not
/// flagged `0x400000` (and with an area when `need_area` is set) whose range
/// is `range` (any range for 3), the first with the highest cost
/// (`EffectItem::GetCost(0)` truncated); 0 for an empty list or when none
/// costs more than -1.
#[allow(clippy::nonminimal_bool)] // The code tests `range != 3` after `range == 3`.
pub fn effect_item_list_get_costliest_effect(
    e: &mut Engine,
    this: Ptr<EffectItemList>,
    range: u32,
    need_area: u8,
) -> u32 {
    let mut best_item = 0;
    let mut cursor = this.addr();
    if e.call(LIST_IS_EMPTY, &args![cursor + 4]).bool() {
        return 0;
    }
    let mut best = -1i32;
    while cursor != 0 {
        let item = cursor_item(e, cursor);
        let cost = e.call(ITEM_GET_COST, &args![item, 0u32]).f64();
        let cost = e.call(FTOL2, &args![cost]).i32();
        if !fn_00405e40(e, Ptr::new(item)) && cost > best {
            let area_ok = need_area == 0 || e.call(ITEM_GET_AREA, &args![item]).i32() > 0;
            if area_ok
                && (range == 3 || (range != 3 && e.call(WORD_AT_C, &args![item]).u32() == range))
            {
                best_item = item;
                best = cost;
            }
        }
        cursor = next_cursor(e, cursor);
    }
    best_item
}

// Translated from 004065e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectItemList::GetAssociatedSkill` (Xbox PDB): the associated skill of
/// the costliest effect of any range (`GetCostliestEffect(3, 0)`), -1
/// without one.
pub fn effect_item_list_get_associated_skill(e: &mut Engine, this: Ptr<EffectItemList>) -> u32 {
    let costliest = effect_item_list_get_costliest_effect(e, this, 3, 0);
    if costliest == 0 {
        return 0xffff_ffff;
    }
    e.call(ITEM_GET_ASSOCIATED_SKILL, &args![costliest]).u32()
}

/// `ftol` of `factor * value` stored as a `float` (the `FILD`, `FMUL double`,
/// `FSTP float` of `BuildMenuString`'s multipliers), with entry point 0xc for
/// actor value 0x10 in between.
fn scaled_magnitude(e: &mut Engine, magnitude: i32, factor: f64, actor_value: u32) -> i32 {
    e.with_stack(4, |e, value| {
        let scaled = x87_store32(x87_mul(factor, f64::from(magnitude)));
        e.mem.set_u32(value.addr(), scaled);
        if actor_value == 0x10 {
            let player = e.mem.u32(PLAYER_011DEA3C);
            e.call(HANDLE_ENTRY_POINT, &args![0xcu32, player, value]);
        }
        let scaled = x87_load32(e.mem.u32(value.addr()));
        e.call(FTOL2, &args![scaled]).i32()
    })
}

// Translated from 00406620 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00406620 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the status list and the name string) and the /GS security cookie check (00ec408c) are not translated.
/// `EffectItemList::BuildMenuString` (Xbox PDB): writes the effects into
/// `text` (`size` bytes). Effects whose conditions hold for the player (and
/// that are not usage monitors) and whose setting has flag 0x2000 go in by
/// name, separated by `"\n"`; the others are collected in a
/// `StatsMenu::StatusDataList` and then, for those with a magnitude and an
/// actor value, written as `"%s %+i"` (abbreviation, magnitude) with the
/// duration in seconds, minutes or hours (`"(%d%c)"`, or `"(%.1f%c)"` when
/// not whole), separated by `", "`. The magnitude is negated for actor
/// values flagged 0x200, scaled by the medicine (vtable +0) and food
/// (vtable +4) multipliers of the player, by the radiation resistance for
/// actor value 0x36, and at least 1 when not negative.
pub fn effect_item_list_build_menu_string(
    e: &mut Engine,
    this: Ptr<EffectItemList>,
    text: u32,
    size: u32,
) {
    // Frame: the condition result byte (+0), the status list (+4, 12
    // bytes), the name string (+0x10, 8 bytes), the line buffer (+0x18,
    // 0x84 bytes).
    e.with_stack(0x9c, |e, frame| {
        let condition_result = frame.addr();
        let status_list = frame.addr() + 4;
        let name_string = frame.addr() + 0x10;
        let line = frame.addr() + 0x18;
        e.mem.set_u8(condition_result, 0);
        e.call(STATUS_LIST_CONSTRUCT, &args![status_list]);
        let medicine = e.vcall(this.addr(), 0, &args![]).bool();
        let food = e.vcall(this.addr(), 4, &args![]).bool();
        let mut named = false;
        e.mem.set_u8(text, 0);
        e.mem.set_u8(line, 0);
        let mut node = if this.is_null() { 0 } else { this.addr() + 4 };
        while node != 0 && node_item(e, node) != 0 {
            let item = node_item(e, node);
            let mut listed = !e.call(ITEM_IS_USAGE_MONITOR, &args![item]).bool();
            if listed {
                let item = node_item(e, node);
                if e.call(PLUS_1C, &args![item]).u32() != 0 {
                    let player = e.mem.u32(PLAYER_011DEA3C);
                    let item = node_item(e, node);
                    let conditions = e.call(PLUS_1C, &args![item]).u32();
                    listed = e
                        .call(
                            CONDITION_IS_TRUE,
                            &args![conditions, player, 0u32, condition_result, 0u32],
                        )
                        .bool();
                }
            }
            if listed {
                let item = node_item(e, node);
                let setting = e.call(ITEM_EFFECT_SETTING, &args![item]).u32();
                if e.call(SETTING_HAS_FLAG, &args![setting, 0x2000u32]).bool() {
                    let item = node_item(e, node);
                    let name = e
                        .call(ITEM_GET_EFFECT_NAME, &args![item, name_string])
                        .u32();
                    let characters = e.call(STRING_TEXT, &args![name]).u32();
                    e.call(BUFFER_COPY_00406D30, &args![line, 0x80u32, characters]);
                    e.call(STRING_DESTRUCT, &args![name_string]);
                    if named {
                        e.call(BUFFER_APPEND_00406D50, &args![text, size, NEWLINE_TEXT]);
                        e.call(BUFFER_APPEND_00406D50, &args![text, size, line]);
                    } else {
                        e.call(BUFFER_APPEND_00406D50, &args![text, size, line]);
                        named = true;
                    }
                } else {
                    let item = node_item(e, node);
                    e.call(STATUS_LIST_REGISTER, &args![status_list, item]);
                }
            }
            node = e.call(NEXT_NODE, &args![node]).u32();
        }
        let mut node = status_list + 4;
        while node != 0 && node_item(e, node) != 0 {
            let has_magnitude = item_word(e, node, 0) != 0;
            if has_magnitude && item_word(e, node, 0x10) != 0xffff_ffff {
                write_effect_line(e, node, medicine, food, text, size, line);
            }
            node = e.call(NEXT_NODE, &args![node]).u32();
        }
        e.call(STATUS_LIST_FLUSH, &args![status_list]);
    });
}

/// The item of a list node, read through `006815c0` as the code does each
/// time.
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(SELF, &args![node]).u32();
    e.mem.u32(slot)
}

/// The word at `offset` of the item of a list node (read through `006815c0`).
fn item_word(e: &mut Engine, node: u32, offset: u32) -> u32 {
    let item = node_item(e, node);
    e.mem.u32(item.wrapping_add(offset))
}

/// One status line of `BuildMenuString` (`node` holds an effect item with a
/// magnitude and an actor value), appended to `text`.
fn write_effect_line(
    e: &mut Engine,
    node: u32,
    medicine: bool,
    food: bool,
    text: u32,
    size: u32,
    line: u32,
) {
    let actor_value = item_word(e, node, 0x10);
    let mut magnitude = item_word(e, node, 0) as i32;
    let mut duration = x87_store32(f64::from(item_word(e, node, 8) as i32));
    let mut unit = b's';
    let hour: f64 = e.global(HOUR_DOUBLE);
    let minute: f64 = e.global(MINUTE_DOUBLE);
    // `FCOMP; TEST AH,1; JNZ`: on when not below (and ordered).
    let not_below = |value: f64, limit: f64| {
        matches!(
            value.partial_cmp(&limit),
            Some(std::cmp::Ordering::Greater | std::cmp::Ordering::Equal)
        )
    };
    if not_below(x87_load32(duration), hour) {
        duration = x87_store32(x87_div(x87_load32(duration), hour));
        unit = b'h';
    } else if not_below(x87_load32(duration), minute) {
        duration = x87_store32(x87_div(x87_load32(duration), minute));
        unit = b'm';
    }
    if e.call(VALUE_HAS_FLAG_00406D70, &args![actor_value, 0x200u32])
        .bool()
    {
        magnitude = magnitude.wrapping_neg();
    }
    if medicine {
        let player = e.mem.u32(PLAYER_011DEA3C);
        let factor = e.call(PLAYER_MEDICINE_MULT_008BEBB0, &args![player]).f64();
        magnitude = scaled_magnitude(e, magnitude, factor, actor_value);
    }
    if food && actor_value != 0x36 {
        let player = e.mem.u32(PLAYER_011DEA3C);
        let factor = e.call(PLAYER_FOOD_MULT_008BEC80, &args![player]).f64();
        magnitude = scaled_magnitude(e, magnitude, factor, actor_value);
    }
    if actor_value == 0x36 && magnitude > 0 {
        let player = e.mem.u32(PLAYER_011DEA3C);
        let factor = e
            .call(PLAYER_RADIATION_RESISTANCE_MULT, &args![player])
            .f64();
        let scaled = x87_store32(x87_mul(factor, f64::from(magnitude)));
        let value = e
            .call(FLOAT_FUNCTION_00406CC0, &args![x87_quiet_bits(scaled)])
            .f64();
        magnitude = e.call(FTOL2, &args![value]).i32();
    }
    if food && actor_value == 0x36 {
        magnitude = e.with_stack(4, |e, factor| {
            e.mem.set_u32(factor.addr(), 1.0f32.to_bits());
            let player = e.mem.u32(PLAYER_011DEA3C);
            e.call(HANDLE_ENTRY_POINT, &args![0x2cu32, player, factor]);
            let factor = e.mem.u32(factor.addr());
            let scaled = x87_mul(f64::from(magnitude), x87_m32(factor));
            e.call(FTOL2, &args![scaled]).i32()
        });
    }
    if (0..1).contains(&magnitude) {
        magnitude = 1;
    }
    if e.mem.u8(text) != 0 {
        e.call(BUFFER_APPEND_00406D50, &args![text, 0x80u32, COMMA_TEXT]);
    }
    if item_word(e, node, 8) == 0 {
        let abbreviation = e.call(ACTOR_VALUE_ABBREVIATION, &args![actor_value]).u32();
        e.call(
            BUFFER_FORMAT,
            &args![line, 0x80u32, VALUE_FORMAT, abbreviation, magnitude],
        );
    } else {
        let rounded = e
            .call(FLOAT_FUNCTION_00406CC0, &args![x87_quiet_bits(duration)])
            .f64();
        // `FUCOMPP; TEST AH,0x44; JNP`: the whole-number form when the
        // duration equals `00406cc0`'s result (ordered).
        if x87_load32(duration) == rounded {
            let whole = e
                .call(FLOAT_TO_INT_00406D90, &args![x87_quiet_bits(duration)])
                .u32();
            let abbreviation = e.call(ACTOR_VALUE_ABBREVIATION, &args![actor_value]).u32();
            e.call(
                BUFFER_FORMAT,
                &args![
                    line,
                    0x80u32,
                    VALUE_WHOLE_DURATION_FORMAT,
                    abbreviation,
                    magnitude,
                    whole,
                    u32::from(unit)
                ],
            );
        } else {
            let abbreviation = e.call(ACTOR_VALUE_ABBREVIATION, &args![actor_value]).u32();
            e.call(
                BUFFER_FORMAT,
                &args![
                    line,
                    0x80u32,
                    VALUE_DURATION_FORMAT,
                    abbreviation,
                    magnitude,
                    x87_load32(duration),
                    u32::from(unit)
                ],
            );
        }
    }
    e.call(BUFFER_APPEND_00406D50, &args![text, size, line]);
}

// Translated from 00406e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectItemList::GetEffectItemIndex` (Xbox PDB): the index (a byte) of
/// `item` in the list; warns and returns 0 when it is not there.
pub fn effect_item_list_get_effect_item_index(
    e: &mut Engine,
    this: Ptr<EffectItemList>,
    item: u32,
) -> u8 {
    let mut index = 0u8;
    let mut node = if this.is_null() { 0 } else { this.addr() + 4 };
    while node != 0 {
        let slot = e.call(SELF, &args![node]).u32();
        if item == e.mem.u32(slot) {
            return index;
        }
        index = index.wrapping_add(1);
        node = e.call(NEXT_NODE, &args![node]).u32();
    }
    e.call(PRINT_WARNING, &args![NOT_IN_LIST_TEXT]);
    0
}

/// Thiscall on a list: the number of its nodes holding an item (`005ae380`,
/// folded code named `VATS::GetCount` in the engine map).
const LIST_COUNT_005AE380: u32 = 0x005a_e380;
/// Thiscall on an effect item (the other item), `RET 4`: `AL`, whether the
/// two items differ (`effectitem.cpp`).
const ITEM_DIFFERS_004042A0: u32 = 0x0040_42a0;
/// Thiscall on an effect item: a word compared with `HasScript`'s argument
/// (`effectitem.cpp`).
const ITEM_MEMBER_004045E0: u32 = 0x0040_45e0;
/// `EffectItem::GetDuration` (`004044d0`) and `EffectItem::GetMagnitude`
/// (`00404390`) (engine map), thiscall: an integer.
const ITEM_GET_DURATION: u32 = 0x0040_44d0;
const ITEM_GET_MAGNITUDE: u32 = 0x0040_4390;
/// The effect item copy constructor (`run_00403b30.rs`), thiscall (source
/// item), `RET 4`.
const EFFECT_ITEM_COPY_00403CA0: u32 = 0x0040_3ca0;
/// `GetEffectItemByIndex`'s warning.
const NO_ITEM_AT_INDEX_TEXT: u32 = 0x0101_26c8;

// Translated from 00405a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): whether the list differs
/// from `other`. True when their item counts (`005ae380`, `other`'s first)
/// differ; false for an empty list; otherwise the two lists are walked side
/// by side and the answer is true at the first pair of items (both non-null)
/// that `004042a0` says differ, false at the end.
pub fn fn_00405a00(e: &mut Engine, this: Ptr<EffectItemList>, other: u32) -> bool {
    let other_count = e
        .call(LIST_COUNT_005AE380, &args![other.wrapping_add(4)])
        .u32();
    let count = e.call(LIST_COUNT_005AE380, &args![this.addr() + 4]).u32();
    if other_count != count {
        return true;
    }
    if e.call(LIST_IS_EMPTY, &args![this.addr() + 4]).bool() {
        return false;
    }
    let mut cursor = this.addr();
    let mut other_cursor = other;
    while cursor != 0 {
        let item = cursor_item(e, cursor);
        let other_item = cursor_item(e, other_cursor);
        if item != 0
            && other_item != 0
            && e.call(ITEM_DIFFERS_004042A0, &args![item, other_item])
                .bool()
        {
            return true;
        }
        cursor = next_cursor(e, cursor);
        other_cursor = next_cursor(e, other_cursor);
    }
    false
}

// Translated from 00405b10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00405b10 - the C++ exception frame (FS:[0] chain, unwind state 0 around the new effect item) is not translated.
/// Unnamed in the engine map (`thiscall`, `RET 4`): makes the list a copy
/// of `source`'s. Empties it (`00406340`), then for each item of `source`
/// allocates 0x24 bytes, copies the item into them (`00403ca0`; 0 when the
/// allocation failed) and adds that (`AddEffect`).
pub fn fn_00405b10(e: &mut Engine, this: Ptr<EffectItemList>, source: u32) {
    fn_00406340(e, this);
    let mut cursor = source;
    while cursor != 0 {
        let item = cursor_item(e, cursor);
        if item != 0 {
            let memory = e.call(ALLOCATE, &args![0x24u32]).u32();
            let copy = if memory == 0 {
                0
            } else {
                e.call(EFFECT_ITEM_COPY_00403CA0, &args![memory, item])
                    .u32()
            };
            effect_item_list_add_effect(e, this, copy);
        }
        cursor = next_cursor(e, cursor);
    }
}

// Translated from 00405c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectItemList::GetEffectScore` (engine map; `thiscall`, `RET 0xc`, a
/// float in `ST0`): the sum, as a `float`, of magnitude times duration
/// (`EffectItem::GetMagnitude`, `EffectItem::GetDuration`, a duration below
/// 1 counting as 1; `IMUL`) over the items whose setting's `00810fd0` word
/// is `archetype`, whose setting's actor value (`00586150`, when the setting
/// has one, `00403c60`) is `actor_value` unless that is 0x4d, and whose
/// range (`0084e3a0`) is `range` unless that is 3. Items whose setting has
/// flag 4 (`00403e60`) are subtracted.
pub fn effect_item_list_get_effect_score(
    e: &mut Engine,
    this: Ptr<EffectItemList>,
    archetype: u32,
    range: u32,
    actor_value: u32,
) -> f32 {
    let mut total = 0.0f32.to_bits();
    let mut cursor = this.addr();
    while cursor != 0 {
        let item = cursor_item(e, cursor);
        if item != 0 {
            let setting = e.call(ITEM_EFFECT_SETTING, &args![item]).u32();
            if e.call(WORD_AT_98, &args![setting]).u32() == archetype
                && (actor_value == 0x4d
                    || !e.call(SETTING_HAS_ACTOR_VALUE, &args![setting]).bool()
                    || e.call(WORD_AT_9C, &args![setting]).u32() == actor_value)
                && (range == 3 || e.call(WORD_AT_C, &args![item]).u32() == range)
            {
                let duration = if e.call(ITEM_GET_DURATION, &args![item]).i32() > 0 {
                    e.call(ITEM_GET_DURATION, &args![item]).i32()
                } else {
                    1
                };
                let magnitude = e.call(ITEM_GET_MAGNITUDE, &args![item]).i32();
                let value = x87_store32(f64::from(magnitude.wrapping_mul(duration)));
                total = if e.call(SETTING_HAS_FLAG, &args![setting, 4u32]).bool() {
                    x87_store32(x87_sub(x87_load32(total), x87_m32(value)))
                } else {
                    x87_store32(x87_add(x87_load32(total), x87_m32(value)))
                };
            }
        }
        cursor = next_cursor(e, cursor);
    }
    f32::from_bits(x87_quiet_bits(total))
}

// Translated from 00405e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectItemList::HasScript` (engine map; `thiscall`, `RET 4`): false
/// for an empty list; otherwise whether an item's `004045e0` word is
/// `script` and the item is not flagged `0x400000` (`00405e40`).
pub fn effect_item_list_has_script(e: &mut Engine, this: Ptr<EffectItemList>, script: u32) -> bool {
    let mut cursor = this.addr();
    if e.call(LIST_IS_EMPTY, &args![cursor + 4]).bool() {
        return false;
    }
    let mut found = false;
    while cursor != 0 && !found {
        let item = cursor_item(e, cursor);
        if item != 0
            && e.call(ITEM_MEMBER_004045E0, &args![item]).u32() == script
            && !fn_00405e40(e, Ptr::new(item))
        {
            found = true;
        }
        cursor = next_cursor(e, cursor);
    }
    found
}

// Translated from 00406090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): an effect of range 0
/// not flagged `0x400000`; false for an empty list (the same body as
/// `HasTargetEffect` with range 0).
pub fn fn_00406090(e: &mut Engine, this: Ptr<EffectItemList>) -> bool {
    has_effect_of_range(e, this, 0)
}

// Translated from 004062a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): false for an empty
/// list; otherwise whether an item is flagged `0x400000` (`00405e40`).
pub fn fn_004062a0(e: &mut Engine, this: Ptr<EffectItemList>) -> bool {
    let mut found = false;
    let mut cursor = this.addr();
    if e.call(LIST_IS_EMPTY, &args![cursor + 4]).bool() {
        return false;
    }
    while cursor != 0 && !found {
        let item = cursor_item(e, cursor);
        if item != 0 && fn_00405e40(e, Ptr::new(item)) {
            found = true;
        }
        cursor = next_cursor(e, cursor);
    }
    found
}

// Translated from 00406ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectItemList::GetEffectItemByIndex` (engine map; `thiscall`, `RET 4`):
/// the item at `index` (a byte) of the list; warns (`010126c8`) and returns
/// 0 when the list is shorter.
pub fn effect_item_list_get_effect_item_by_index(
    e: &mut Engine,
    this: Ptr<EffectItemList>,
    index: u8,
) -> u32 {
    let mut position = 0u8;
    let mut node = if this.is_null() { 0 } else { this.addr() + 4 };
    while node != 0 {
        if position == index {
            let slot = e.call(SELF, &args![node]).u32();
            return e.mem.u32(slot);
        }
        position = position.wrapping_add(1);
        node = e.call(NEXT_NODE, &args![node]).u32();
    }
    e.call(PRINT_WARNING, &args![NO_ITEM_AT_INDEX_TEXT]);
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00402c90, effect_item_list_effect_item_list(Ptr) -> Ptr),
        entry!(0x0040_59d0, fn_004059d0(Ptr, u32) -> Ptr),
        entry!(0x0040_6e40, stats_menu_status_data_list_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x0040_56a0,
            fn_004056a0(Ptr<EffectItemList>) -> Ptr<EffectItemList>
        ),
        entry!(
            0x0040_5760,
            effect_item_list_load(Ptr<EffectItemList>, u32, u32)
        ),
        entry!(
            0x0040_5d20,
            effect_item_list_add_effect(Ptr<EffectItemList>, u32)
        ),
        entry!(
            0x0040_5d80,
            effect_item_list_is_hostile(Ptr<EffectItemList>) -> bool
        ),
        entry!(
            0x0040_5da0,
            effect_item_list_can_be_poison(Ptr<EffectItemList>) -> bool
        ),
        entry!(0x0040_5e40, fn_00405e40(Ptr) -> bool),
        entry!(0x0040_5e60, fn_00405e60(Ptr) -> bool),
        entry!(
            0x0040_5f30,
            effect_item_list_has_target_effect(Ptr<EffectItemList>) -> bool
        ),
        entry!(
            0x0040_5fe0,
            effect_item_list_has_touch_effect(Ptr<EffectItemList>) -> bool
        ),
        entry!(0x0040_6140, fn_00406140(Ptr<EffectItemList>, u32) -> bool),
        entry!(
            0x0040_6200,
            effect_item_list_get_usage_monitor_effect(Ptr<EffectItemList>) -> u32
        ),
        entry!(0x0040_6340, fn_00406340(Ptr<EffectItemList>)),
        entry!(
            0x0040_63e0,
            effect_item_list_get_cost(Ptr<EffectItemList>, u32) -> f32
        ),
        entry!(0x0040_64b0, fn_004064b0(Ptr) -> bool),
        entry!(
            0x0040_64d0,
            effect_item_list_get_costliest_effect(Ptr<EffectItemList>, u32, u8) -> u32
        ),
        entry!(
            0x0040_65e0,
            effect_item_list_get_associated_skill(Ptr<EffectItemList>) -> u32
        ),
        entry!(
            0x0040_6620,
            effect_item_list_build_menu_string(Ptr<EffectItemList>, u32, u32)
        ),
        entry!(
            0x0040_6e70,
            effect_item_list_get_effect_item_index(Ptr<EffectItemList>, u32) -> u8
        ),
        entry!(0x0040_5a00, fn_00405a00(Ptr<EffectItemList>, u32) -> bool),
        entry!(0x0040_5b10, fn_00405b10(Ptr<EffectItemList>, u32)),
        entry!(
            0x0040_5c00,
            effect_item_list_get_effect_score(Ptr<EffectItemList>, u32, u32, u32) -> f32
        ),
        entry!(
            0x0040_5e80,
            effect_item_list_has_script(Ptr<EffectItemList>, u32) -> bool
        ),
        entry!(0x0040_6090, fn_00406090(Ptr<EffectItemList>) -> bool),
        entry!(0x0040_62a0, fn_004062a0(Ptr<EffectItemList>) -> bool),
        entry!(
            0x0040_6ee0,
            effect_item_list_get_effect_item_by_index(Ptr<EffectItemList>, u8) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn effect_item_list_effect_item_list_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0040_2c90,
            size: 0x10,
            globals: &[],
            returns: &[],
            calls: &[(0x0040_2d00, &[A::T(0x0)]), (0x0040_56a0, &[A::T(0xc)])],
            stores: &[(0x0, 4, 0x0101_1d0c), (0xc, 4, 0x0101_1cf4)],
        });
    }
    #[test]
    fn at_004059d0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0040_3d50, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0040_59d0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0040_59d0)
                .collect();
            let mut expected = vec![(0x0040_3d50, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_00406e40_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0040_6de0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0040_6e40, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0040_6e40)
                .collect();
            let mut expected = vec![(0x0040_6de0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod effect_item_list_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn returns(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    /// An engine whose list helpers behave as the game's: `006815c0` the
    /// node itself, `00726070` its next pointer (+4), `008256d0` whether
    /// the first item is null.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [0x0101_2000, 0x011c_3000, 0x011d_e000] {
            e.map(page, 0x1000);
        }
        e.register(SELF, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(NEXT_NODE, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(LIST_IS_EMPTY, |e, a| Ret {
            eax: u32::from(e.mem.u32(a[0]) == 0),
            ..Ret::default()
        });
        e
    }

    /// An effect item: magnitude, area, duration, range, actor value, the
    /// effect setting (+0x14).
    fn item(e: &mut Engine, words: [u32; 6]) -> u32 {
        let item = e.mem.alloc(0x24);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(item + 4 * i as u32, *word);
        }
        item
    }

    /// A list holding `items` (the first in the embedded node).
    fn list_of(e: &mut Engine, items: &[u32]) -> Ptr<EffectItemList> {
        let list: Ptr<EffectItemList> = e.new_object();
        let mut node = list.addr() + 4;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        list
    }

    /// Doubles reading an effect setting's flags from its first word, an
    /// item's setting from +0x14 and its range from +0xc.
    fn setting_flags(e: &mut Engine) {
        e.register(SETTING_HAS_FLAG, |e, a| Ret {
            eax: u32::from(e.mem.u32(a[0]) & a[1] != 0),
            ..Ret::default()
        });
        e.register(ITEM_EFFECT_SETTING, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x14),
            ..Ret::default()
        });
        e.register(WORD_AT_C, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0xc),
            ..Ret::default()
        });
    }

    fn setting(e: &mut Engine, flags: u32) -> u32 {
        let setting = e.mem.alloc(0xb0);
        e.mem.set_u32(setting, flags);
        setting
    }

    #[test]
    fn constructor_sets_the_vtable_and_clears_the_count() {
        let mut e = engine();
        quiet(&mut e, &[LIST_CONSTRUCT]);
        let list: Ptr<EffectItemList> = e.new_object();
        e.set(list, EffectItemList::iHostileCount, 5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0040_56a0, &args![list]).u32(), list.addr());
        assert_eq!(e.mem.u32(list.addr()), EFFECT_ITEM_LIST_VTABLE);
        assert_eq!(e.get(list, EffectItemList::iHostileCount), 0);
        assert_eq!(calls_to(&e, LIST_CONSTRUCT), vec![vec![list.addr() + 4]]);
    }

    #[test]
    fn add_effect_appends_and_counts_hostile_effects() {
        let mut e = engine();
        setting_flags(&mut e);
        returns(&mut e, ITEM_IS_USAGE_MONITOR, 0);
        returns(&mut e, ITEM_IS_HOSTILE, 1);
        e.register(LIST_APPEND, |e, a| Ret {
            eax: e.mem.u32(a[1]),
            ..Ret::default()
        });
        let list = list_of(&mut e, &[]);
        e.call_log = Some(vec![]);
        e.call(0x0040_5d20, &args![list, 0x5000u32]);
        let appends = calls_to(&e, LIST_APPEND);
        assert_eq!(appends.len(), 1);
        assert_eq!(appends[0][0], list.addr() + 4);
        assert_eq!(e.get(list, EffectItemList::iHostileCount), 1);
        assert!(e.call(0x0040_5d80, &args![list]).bool());
        // A second usage monitor effect is not added.
        let monitor_setting = setting(&mut e, 0);
        returns(&mut e, GET_USAGE_MONITOR_EFFECT_SETTING, monitor_setting);
        let monitor = item(&mut e, [0, 0, 0, 0, 0, monitor_setting]);
        let list = list_of(&mut e, &[monitor]);
        returns(&mut e, ITEM_IS_USAGE_MONITOR, 1);
        e.call_log = Some(vec![]);
        e.call(0x0040_5d20, &args![list, 0x5100u32]);
        assert!(calls_to(&e, LIST_APPEND).is_empty());
        assert!(!e.call(0x0040_5d80, &args![list]).bool());
    }

    #[test]
    fn can_be_poison_needs_every_effect_hostile_or_flagged() {
        let mut e = engine();
        setting_flags(&mut e);
        e.register(ITEM_IS_HOSTILE, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        let plain = setting(&mut e, 0);
        let flagged = setting(&mut e, 0x0040_0000);
        let hostile = item(&mut e, [1, 0, 0, 0, 0, plain]);
        let harmless = item(&mut e, [0, 0, 0, 0, 0, plain]);
        let script = item(&mut e, [0, 0, 0, 0, 0, flagged]);
        let empty = list_of(&mut e, &[]);
        assert!(!e.call(0x0040_5da0, &args![empty]).bool());
        let list = list_of(&mut e, &[hostile, script]);
        assert!(e.call(0x0040_5da0, &args![list]).bool());
        let list = list_of(&mut e, &[hostile, harmless]);
        assert!(!e.call(0x0040_5da0, &args![list]).bool());
        assert!(e.call(0x0040_5e40, &args![script]).bool());
        assert!(!e.call(0x0040_5e60, &args![plain]).bool());
    }

    #[test]
    fn target_and_touch_effects_by_range() {
        let mut e = engine();
        setting_flags(&mut e);
        let plain = setting(&mut e, 0);
        let flagged = setting(&mut e, 0x0040_0000);
        let touch = item(&mut e, [0, 0, 0, 1, 0, plain]);
        let target_flagged = item(&mut e, [0, 0, 0, 2, 0, flagged]);
        let target = item(&mut e, [0, 0, 0, 2, 0, plain]);
        let list = list_of(&mut e, &[touch, target_flagged]);
        assert!(!e.call(0x0040_5f30, &args![list]).bool());
        assert!(e.call(0x0040_5fe0, &args![list]).bool());
        let list = list_of(&mut e, &[touch, target]);
        assert!(e.call(0x0040_5f30, &args![list]).bool());
        let empty = list_of(&mut e, &[]);
        assert!(!e.call(0x0040_5fe0, &args![empty]).bool());
    }

    #[test]
    fn setting_flag_search() {
        let mut e = engine();
        setting_flags(&mut e);
        let with = setting(&mut e, 0x2_0000);
        let flagged = setting(&mut e, 0x2_0000 | 0x0040_0000);
        let without = setting(&mut e, 0);
        let a = item(&mut e, [0, 0, 0, 0, 0, without]);
        let b = item(&mut e, [0, 0, 0, 0, 0, flagged]);
        let c = item(&mut e, [0, 0, 0, 0, 0, with]);
        let list = list_of(&mut e, &[a, b]);
        assert!(!e.call(0x0040_6140, &args![list, 0x2_0000u32]).bool());
        let list = list_of(&mut e, &[a, b, c]);
        assert!(e.call(0x0040_6140, &args![list, 0x2_0000u32]).bool());
    }

    #[test]
    fn usage_monitor_effect_is_found_by_setting() {
        let mut e = engine();
        setting_flags(&mut e);
        let monitor = setting(&mut e, 0);
        let other = setting(&mut e, 0);
        returns(&mut e, GET_USAGE_MONITOR_EFFECT_SETTING, monitor);
        let a = item(&mut e, [0, 0, 0, 0, 0, other]);
        let b = item(&mut e, [0, 0, 0, 0, 0, monitor]);
        let list = list_of(&mut e, &[a, b]);
        assert_eq!(e.call(0x0040_6200, &args![list]).u32(), b);
        let list = list_of(&mut e, &[a]);
        assert_eq!(e.call(0x0040_6200, &args![list]).u32(), 0);
    }

    #[test]
    fn remove_effects_deletes_each_item() {
        let mut e = engine();
        quiet(&mut e, &[0x0040_3d50, 0x0040_1030]);
        // `0063f7b0` drops the head item (moves the next one up).
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
        let list = list_of(&mut e, &[0x5000, 0x5100]);
        e.call_log = Some(vec![]);
        e.call(0x0040_6340, &args![list]);
        assert_eq!(calls_to(&e, 0x0040_3d50), vec![vec![0x5000], vec![0x5100]]);
        assert_eq!(calls_to(&e, LIST_REMOVE_FIRST).len(), 2);
        assert_eq!(e.mem.u32(list.addr() + 4), 0);
    }

    #[test]
    fn cost_sums_the_unflagged_effects() {
        let mut e = engine();
        setting_flags(&mut e);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.register(ITEM_GET_COST, |e, a| Ret {
            st0: f64::from(e.mem.i32(a[0])),
            ..Ret::default()
        });
        e.register(COST_ROUND_00404040, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])) + 0.5,
            ..Ret::default()
        });
        let plain = setting(&mut e, 0);
        let flagged = setting(&mut e, 0x0040_0000);
        let a = item(&mut e, [3, 0, 0, 0, 0, plain]);
        let b = item(&mut e, [100, 0, 0, 0, 0, flagged]);
        let c = item(&mut e, [4, 0, 0, 0, 0, plain]);
        let list = list_of(&mut e, &[a, b, c]);
        assert_eq!(e.call(0x0040_63e0, &args![list, 0x7000u32]).f32(), 7.5);
        // A negative total is 0.
        let d = item(&mut e, [(-9i32) as u32, 0, 0, 0, 0, plain]);
        let list = list_of(&mut e, &[a, d]);
        assert_eq!(e.call(0x0040_63e0, &args![list, 0x7000u32]).f32(), 0.0);
        let empty = list_of(&mut e, &[]);
        assert_eq!(e.call(0x0040_63e0, &args![empty, 0x7000u32]).f32(), 0.0);
    }

    #[test]
    fn table_flag_7c() {
        let mut e = engine();
        returns(&mut e, WORD_AT_98, 5);
        returns(&mut e, TABLE_HAS_FLAG, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0040_64b0, &args![0x5000u32]).bool());
        assert_eq!(calls_to(&e, TABLE_HAS_FLAG), vec![vec![5, 0x7c]]);
    }

    #[test]
    fn costliest_effect_and_its_skill() {
        let mut e = engine();
        setting_flags(&mut e);
        e.register(ITEM_GET_COST, |e, a| Ret {
            st0: f64::from(e.mem.i32(a[0])),
            ..Ret::default()
        });
        e.register(FTOL2, |_, a| Ret {
            eax: f64::take(a, &mut 0) as i32 as u32,
            ..Ret::default()
        });
        e.register(ITEM_GET_AREA, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(ITEM_GET_ASSOCIATED_SKILL, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x10),
            ..Ret::default()
        });
        let plain = setting(&mut e, 0);
        let flagged = setting(&mut e, 0x0040_0000);
        let cheap = item(&mut e, [5, 1, 0, 1, 31, plain]);
        let dear = item(&mut e, [9, 0, 0, 2, 32, plain]);
        let dearest = item(&mut e, [20, 0, 0, 1, 33, flagged]);
        let list = list_of(&mut e, &[cheap, dear, dearest]);
        assert_eq!(e.call(0x0040_64d0, &args![list, 3u32, 0u8]).u32(), dear);
        assert_eq!(e.call(0x0040_64d0, &args![list, 1u32, 0u8]).u32(), cheap);
        // With an area required only the first qualifies.
        assert_eq!(e.call(0x0040_64d0, &args![list, 3u32, 1u8]).u32(), cheap);
        assert_eq!(e.call(0x0040_65e0, &args![list]).u32(), 32);
        let empty = list_of(&mut e, &[]);
        assert_eq!(e.call(0x0040_65e0, &args![empty]).u32(), 0xffff_ffff);
    }

    #[test]
    fn effect_item_index() {
        let mut e = engine();
        quiet(&mut e, &[PRINT_WARNING]);
        let list = list_of(&mut e, &[0x5000, 0x5100, 0x5200]);
        assert_eq!(e.call(0x0040_6e70, &args![list, 0x5200u32]).u32(), 2);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0040_6e70, &args![list, 0x5300u32]).u32(), 0);
        assert_eq!(calls_to(&e, PRINT_WARNING), vec![vec![NOT_IN_LIST_TEXT]]);
    }

    #[test]
    fn load_reads_effects_and_their_conditions() {
        let mut e = engine();
        let chunk = e.mem.alloc(4);
        e.register_double(FILE_GET_CHUNK, move |e, _| Ret {
            eax: e.mem.u32(chunk),
            ..Ret::default()
        });
        e.register(FILE_GET_CHUNK_DATA, |e, a| {
            e.mem.set_u32(a[1], 0x14b);
            Ret::default()
        });
        quiet(&mut e, &[ADD_COMPILE_INDEX, FILE_NEXT_CHUNK, PRINT_WARNING]);
        returns(&mut e, LOOK_UP_FORM, 0x6000);
        returns(&mut e, DYNAMIC_CAST, 0x6000);
        returns(&mut e, ALLOCATE, 0x6100);
        e.register(EFFECT_ITEM_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        returns(&mut e, SETTING_HAS_ACTOR_VALUE, 0);
        returns(&mut e, ITEM_LOAD, 1);
        returns(&mut e, ITEM_IS_USAGE_MONITOR, 0);
        returns(&mut e, ITEM_IS_HOSTILE, 1);
        quiet(&mut e, &[LIST_APPEND]);
        let list = list_of(&mut e, &[]);
        e.mem.set_u32(chunk, CHUNK_EFID);
        e.call_log = Some(vec![]);
        e.call(0x0040_5760, &args![list, 0x7000u32, 0x7100u32]);
        assert_eq!(calls_to(&e, LOOK_UP_FORM), vec![vec![0x14b]]);
        assert_eq!(
            calls_to(&e, DYNAMIC_CAST),
            vec![vec![0x6000, 0, TYPE_TES_FORM, TYPE_EFFECT_SETTING, 0]]
        );
        assert_eq!(
            calls_to(&e, EFFECT_ITEM_CONSTRUCT),
            vec![vec![0x6100, 0x6000]]
        );
        assert_eq!(calls_to(&e, ITEM_LOAD), vec![vec![0x6100, 0x7000, 0x7100]]);
        assert_eq!(e.mem.u32(CURRENT_EFFECT_ITEM), 0x6100);
        assert_eq!(e.get(list, EffectItemList::iHostileCount), 1);
        // A condition loads into that item (and counts it again).
        e.mem.set_u32(chunk, CHUNK_CTDA);
        e.call_log = Some(vec![]);
        e.call(0x0040_5760, &args![list, 0x7000u32, 0u32]);
        assert_eq!(calls_to(&e, ITEM_LOAD), vec![vec![0x6100, 0x7000, 0]]);
        assert_eq!(e.get(list, EffectItemList::iHostileCount), 2);
        // Without a current item the condition is reported.
        e.mem.set_u32(CURRENT_EFFECT_ITEM, 0);
        e.call_log = Some(vec![]);
        e.call(0x0040_5760, &args![list, 0x7000u32, 0u32]);
        assert_eq!(
            calls_to(&e, PRINT_WARNING),
            vec![vec![NO_EFFECT_ITEM_TEXT, UNKNOWN_TEXT]]
        );
        // An unknown setting is reported.
        e.mem.set_u32(chunk, CHUNK_EFID);
        returns(&mut e, DYNAMIC_CAST, 0);
        e.call_log = Some(vec![]);
        e.call(0x0040_5760, &args![list, 0x7000u32, 0x7100u32]);
        assert_eq!(
            calls_to(&e, PRINT_WARNING),
            vec![vec![NO_EFFECT_SETTING_TEXT, 0x14b, 0x7100]]
        );
    }

    #[test]
    fn load_failure_deletes_the_item() {
        let mut e = engine();
        returns(&mut e, FILE_GET_CHUNK, CHUNK_EFID);
        quiet(
            &mut e,
            &[
                FILE_GET_CHUNK_DATA,
                ADD_COMPILE_INDEX,
                FILE_NEXT_CHUNK,
                PRINT_WARNING,
                STRING_DESTRUCT,
                0x0040_3d50,
                0x0040_1030,
            ],
        );
        returns(&mut e, LOOK_UP_FORM, 0x6000);
        returns(&mut e, DYNAMIC_CAST, 0x6000);
        returns(&mut e, ALLOCATE, 0x6100);
        e.register(EFFECT_ITEM_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        returns(&mut e, SETTING_HAS_ACTOR_VALUE, 1);
        returns(&mut e, WORD_AT_9C, 0x36);
        returns(&mut e, ITEM_LOAD, 0);
        e.register(SETTING_NAME_004046B0, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        returns(&mut e, STRING_TEXT, 0x6200);
        let list = list_of(&mut e, &[]);
        e.mem.set_u32(CURRENT_EFFECT_ITEM, 0x5555);
        e.call_log = Some(vec![]);
        e.call(0x0040_5760, &args![list, 0x7000u32, 0x7100u32]);
        assert_eq!(
            calls_to(&e, PRINT_WARNING),
            vec![vec![LOAD_FAILED_TEXT, 0x6200, 0x7100]]
        );
        let string = calls_to(&e, SETTING_NAME_004046B0)[0][1];
        assert_eq!(calls_to(&e, STRING_DESTRUCT), vec![vec![string]]);
        assert_eq!(calls_to(&e, 0x0040_3d50), vec![vec![0x6100]]);
        assert_eq!(e.mem.u32(CURRENT_EFFECT_ITEM), 0);
    }

    /// Doubles for `BuildMenuString`: vtable slots 0 and 4 answer the
    /// medicine and food flags, every effect's conditions hold.
    fn menu_doubles(e: &mut Engine, list: Ptr<EffectItemList>, medicine: u32, food: u32) {
        e.put_vtable(0x0109_0000, &[0x0050_0000, 0x0050_0004]);
        e.mem.set_u32(list.addr(), 0x0109_0000);
        returns(e, 0x0050_0000, medicine);
        returns(e, 0x0050_0004, food);
        setting_flags(e);
        returns(e, ITEM_IS_USAGE_MONITOR, 0);
        e.register(PLUS_1C, |_, a| Ret {
            eax: a[0] + 0x1c,
            ..Ret::default()
        });
        returns(e, CONDITION_IS_TRUE, 1);
        quiet(
            e,
            &[STATUS_LIST_CONSTRUCT, STATUS_LIST_FLUSH, STRING_DESTRUCT],
        );
        // The status list keeps its items in nodes from +4.
        e.register(STATUS_LIST_REGISTER, |e, a| {
            let mut node = a[0] + 4;
            while e.mem.u32(node) != 0 {
                let next = e.mem.u32(node + 4);
                if next == 0 {
                    let fresh = e.mem.alloc(8);
                    e.mem.set_u32(node + 4, fresh);
                    node = fresh;
                    break;
                }
                node = next;
            }
            e.mem.set_u32(node, a[1]);
            Ret::default()
        });
        e.register(BUFFER_APPEND_00406D50, |e, a| {
            let mut end = a[0];
            while e.mem.u8(end) != 0 {
                end += 1;
            }
            let mut from = a[2];
            loop {
                let c = e.mem.u8(from);
                e.mem.set_u8(end, c);
                if c == 0 {
                    break;
                }
                end += 1;
                from += 1;
            }
            Ret::default()
        });
        returns(e, VALUE_HAS_FLAG_00406D70, 0);
        e.register(ACTOR_VALUE_ABBREVIATION, |_, a| Ret {
            eax: 0x0600_0000 + a[0],
            ..Ret::default()
        });
        e.register(FTOL2, |_, a| Ret {
            eax: f64::take(a, &mut 0) as i32 as u32,
            ..Ret::default()
        });
        e.register(FLOAT_FUNCTION_00406CC0, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])).floor(),
            ..Ret::default()
        });
        e.register(FLOAT_TO_INT_00406D90, |_, a| Ret {
            eax: f32::from_bits(a[0]) as i32 as u32,
            ..Ret::default()
        });
        // The line formatter writes a one-letter marker of its format.
        e.register(BUFFER_FORMAT, |e, a| {
            let marker = match a[2] {
                VALUE_FORMAT => b'a',
                VALUE_WHOLE_DURATION_FORMAT => b'b',
                _ => b'c',
            };
            e.mem.set_u8(a[0], marker);
            e.mem.set_u8(a[0] + 1, 0);
            Ret::default()
        });
        e.set_global(HOUR_DOUBLE, 3600.0f64);
        e.set_global(MINUTE_DOUBLE, 60.0f64);
    }

    fn text_of(e: &Engine, mut at: u32) -> String {
        let mut out = String::new();
        while e.mem.u8(at) != 0 {
            out.push(e.mem.u8(at) as char);
            at += 1;
        }
        out
    }

    #[test]
    fn menu_string_lists_names_and_values() {
        let mut e = engine();
        e.set_global(COMMA_TEXT, 0x0000_202c_u32);
        e.set_global(NEWLINE_TEXT, 0x0au32);
        let named = setting(&mut e, 0x2000);
        let plain = setting(&mut e, 0);
        // Name-only effects get their name (`GetEffectName` gives "N").
        let first = item(&mut e, [0, 0, 0, 0, 0, named]);
        let second = item(&mut e, [0, 0, 0, 0, 0, named]);
        // Value effects: no duration, a whole number of minutes, 90 s.
        let a = item(&mut e, [5, 0, 0, 0, 7, plain]);
        let b = item(&mut e, [0xffff_fffd, 0, 120, 0, 8, plain]);
        let c = item(&mut e, [2, 0, 90, 0, 9, plain]);
        let skipped = item(&mut e, [0, 0, 0, 0, 10, plain]);
        let list = list_of(&mut e, &[first, a, second, b, c, skipped]);
        menu_doubles(&mut e, list, 0, 0);
        e.register(ITEM_GET_EFFECT_NAME, |e, a| {
            let text = e.mem.alloc(2);
            e.mem.set_u8(text, b'N');
            e.mem.set_u32(a[1], text);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register(STRING_TEXT, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(BUFFER_COPY_00406D30, |e, a| {
            let c = e.mem.u8(a[2]);
            e.mem.set_u8(a[0], c);
            e.mem.set_u8(a[0] + 1, 0);
            Ret::default()
        });
        let text = e.mem.alloc(0x100);
        e.call_log = Some(vec![]);
        e.call(0x0040_6620, &args![list, text, 0x100u32]);
        assert_eq!(text_of(&e, text), "N\nN, a, b, c");
        let formats = calls_to(&e, BUFFER_FORMAT);
        assert_eq!(formats[0][2..], [VALUE_FORMAT, 0x0600_0007, 5]);
        assert_eq!(
            formats[1][2..],
            [
                VALUE_WHOLE_DURATION_FORMAT,
                0x0600_0008,
                0xffff_fffd,
                2,
                u32::from(b'm')
            ]
        );
        let bits = 1.5f64.to_bits();
        assert_eq!(
            formats[2][2..],
            [
                VALUE_DURATION_FORMAT,
                0x0600_0009,
                2,
                bits as u32,
                (bits >> 32) as u32,
                u32::from(b'm')
            ]
        );
        assert_eq!(calls_to(&e, STATUS_LIST_REGISTER).len(), 4);
    }

    #[test]
    fn menu_string_scales_medicine_and_radiation() {
        let mut e = engine();
        e.mem.set_u32(PLAYER_011DEA3C, 0x4444);
        let plain = setting(&mut e, 0);
        let health = item(&mut e, [10, 0, 0, 0, 0x10, plain]);
        let radiation = item(&mut e, [10, 0, 0, 0, 0x36, plain]);
        let list = list_of(&mut e, &[health, radiation]);
        menu_doubles(&mut e, list, 1, 1);
        e.register(PLAYER_MEDICINE_MULT_008BEBB0, |_, _| Ret {
            st0: 1.5,
            ..Ret::default()
        });
        e.register(PLAYER_FOOD_MULT_008BEC80, |_, _| Ret {
            st0: 2.0,
            ..Ret::default()
        });
        e.register(PLAYER_RADIATION_RESISTANCE_MULT, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        // The entry points double the value they are given.
        e.register(HANDLE_ENTRY_POINT, |e, a| {
            let value = e.mem.f32(a[2]) * 2.0;
            e.mem.set_f32(a[2], value);
            Ret::default()
        });
        let text = e.mem.alloc(0x100);
        e.call_log = Some(vec![]);
        e.call(0x0040_6620, &args![list, text, 0x100u32]);
        let formats = calls_to(&e, BUFFER_FORMAT);
        // Health: 10 * 1.5 = 15, doubled to 30; * 2.0 = 60, doubled to 120.
        assert_eq!(formats[0][4], 120);
        // Radiation: 10 * 1.5 = 15 (no entry point: not 0x10); 15 * 0.5 =
        // 7.5, floored to 7; the food entry point (0x2c) doubles 1.0: 14.
        assert_eq!(formats[1][4], 14);
        let points: Vec<u32> = calls_to(&e, HANDLE_ENTRY_POINT)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(points, vec![0xc, 0xc, 0x2c]);
    }
}

#[cfg(test)]
mod list_query_tests {
    use super::*;

    /// List helpers as the game's: `006815c0` the node itself, `00726070`
    /// the word at +4, `008256d0` whether the first item is null.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(SELF, |_, a| a[0].into_ret());
        e.register(NEXT_NODE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LIST_IS_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0).into_ret()
        });
        e.register(PRINT_WARNING, |_, _| Ret::default());
        e
    }

    fn list_of(e: &mut Engine, items: &[u32]) -> Ptr<EffectItemList> {
        let list: Ptr<EffectItemList> = e.new_object();
        let mut node = list.addr() + 4;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        list
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// An item the doubles below read: bit 0 of its first word the
    /// `0x400000` flag, the word at +4 its range, +8 its `004045e0` word,
    /// and itself as its effect setting (+0x14).
    fn item(e: &mut Engine, flagged: bool, range: u32, word: u32) -> u32 {
        let item = e.mem.alloc(0x24);
        e.mem.set_u32(item, u32::from(flagged));
        e.mem.set_u32(item + 4, range);
        e.mem.set_u32(item + 8, word);
        e.mem.set_u32(item + 0x14, item);
        item
    }

    fn item_doubles(e: &mut Engine) {
        e.register(SETTING_HAS_FLAG, |e, a| {
            u32::from(a[1] == 0x0040_0000 && e.mem.u32(a[0]) & 1 != 0).into_ret()
        });
        e.register(WORD_AT_C, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(ITEM_MEMBER_004045E0, |e, a| e.mem.u32(a[0] + 8).into_ret());
    }

    #[test]
    fn lists_differ_by_count_or_by_an_item() {
        let mut e = engine();
        e.register(LIST_COUNT_005AE380, |e, a| {
            let mut node = a[0];
            let mut count = 0u32;
            while node != 0 {
                if e.mem.u32(node) != 0 {
                    count += 1;
                }
                node = e.mem.u32(node + 4);
            }
            count.into_ret()
        });
        e.register(ITEM_DIFFERS_004042A0, |_, a| {
            u32::from(a[0] % 0x100 != a[1] % 0x100).into_ret()
        });
        let a = list_of(&mut e, &[0x1010, 0x1020]);
        let b = list_of(&mut e, &[0x2010, 0x2020]);
        let c = list_of(&mut e, &[0x2010, 0x2030]);
        let d = list_of(&mut e, &[0x2010]);
        let empty = list_of(&mut e, &[0]);
        let empty2 = list_of(&mut e, &[0]);
        assert!(!fn_00405a00(&mut e, a, b.addr()));
        assert!(fn_00405a00(&mut e, a, c.addr()));
        assert!(fn_00405a00(&mut e, a, d.addr()));
        assert!(!e.call(0x0040_5a00, &args![empty, empty2]).bool());
        e.call_log = Some(vec![]);
        fn_00405a00(&mut e, a, d.addr());
        assert_eq!(
            calls_to(&e, LIST_COUNT_005AE380),
            vec![vec![d.addr() + 4], vec![a.addr() + 4]]
        );
    }

    #[test]
    fn copy_empties_then_adds_a_copy_of_each_item() {
        let mut e = engine();
        e.register(LIST_REMOVE_FIRST, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.register(0x0040_59d0, |_, _| Ret::default());
        e.register(ITEM_IS_USAGE_MONITOR, |_, _| Ret::default());
        e.register(ITEM_IS_HOSTILE, |_, _| Ret::default());
        e.register(LIST_APPEND, |_, _| Ret::default());
        e.register(ALLOCATE, |e, _| e.mem.alloc(0x24).into_ret());
        e.register(EFFECT_ITEM_COPY_00403CA0, |_, a| a[0].into_ret());
        let this = list_of(&mut e, &[0]);
        let source = list_of(&mut e, &[0x1010, 0, 0x1030]);
        e.call_log = Some(vec![]);
        e.call(0x0040_5b10, &args![this, source]);
        let copies = calls_to(&e, EFFECT_ITEM_COPY_00403CA0);
        assert_eq!(copies.len(), 2);
        assert_eq!(copies[0][1], 0x1010);
        assert_eq!(copies[1][1], 0x1030);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0x24], vec![0x24]]);
        assert_eq!(calls_to(&e, LIST_APPEND).len(), 2);
        // A failed allocation adds a null item.
        e.register(ALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        fn_00405b10(&mut e, this, source.addr());
        assert!(calls_to(&e, EFFECT_ITEM_COPY_00403CA0).is_empty());
        assert_eq!(calls_to(&e, ITEM_IS_USAGE_MONITOR), vec![vec![0], vec![0]]);
    }

    #[test]
    fn effect_score_sums_matching_items() {
        let mut e = engine();
        // Item words: +0 archetype, +4 range, +8 actor value (0 for none),
        // +0xc duration, +0x10 magnitude, +0x18 subtract flag.
        e.register(ITEM_EFFECT_SETTING, |_, a| a[0].into_ret());
        e.register(WORD_AT_98, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(WORD_AT_C, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(SETTING_HAS_ACTOR_VALUE, |e, a| {
            u32::from(e.mem.u32(a[0] + 8) != 0).into_ret()
        });
        e.register(WORD_AT_9C, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(ITEM_GET_DURATION, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(ITEM_GET_MAGNITUDE, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(SETTING_HAS_FLAG, |e, a| {
            u32::from(a[1] == 4 && e.mem.u32(a[0] + 0x18) != 0).into_ret()
        });
        let mut make = |words: [u32; 7]| {
            let item = e.mem.alloc(0x20);
            for (i, w) in words.iter().enumerate() {
                e.mem.set_u32(item + 4 * i as u32, *w);
            }
            item
        };
        let plain = make([5, 2, 0, 10, 3, 0, 0]); // +30
        let short = make([5, 1, 7, 0, 4, 0, 0]); // duration 0 counts 1: +4
        let minus = make([5, 2, 7, (-2i32) as u32, 6, 0, 1]); // -6
        let other_av = make([5, 2, 8, 1, 100, 0, 0]);
        let other_kind = make([6, 2, 0, 1, 100, 0, 0]);
        let list = list_of(&mut e, &[plain, 0, short, minus, other_av, other_kind]);
        assert_eq!(
            effect_item_list_get_effect_score(&mut e, list, 5, 3, 7),
            28.0
        );
        assert_eq!(
            e.call(0x0040_5c00, &args![list, 5u32, 3u32, 0x4du32]).f32(),
            128.0
        );
        assert_eq!(
            effect_item_list_get_effect_score(&mut e, list, 5, 2, 7),
            24.0
        );
        assert_eq!(
            effect_item_list_get_effect_score(&mut e, list, 9, 3, 0x4d),
            0.0
        );
    }

    #[test]
    fn has_script_finds_an_unflagged_match() {
        let mut e = engine();
        item_doubles(&mut e);
        let flagged = item(&mut e, true, 0, 0x77);
        let plain = item(&mut e, false, 0, 0x77);
        let empty = list_of(&mut e, &[0]);
        assert!(!effect_item_list_has_script(&mut e, empty, 0x77));
        let only_flagged = list_of(&mut e, &[flagged]);
        assert!(!effect_item_list_has_script(&mut e, only_flagged, 0x77));
        let both = list_of(&mut e, &[flagged, 0, plain]);
        assert!(e.call(0x0040_5e80, &args![both, 0x77u32]).bool());
        assert!(!effect_item_list_has_script(&mut e, both, 0x78));
    }

    #[test]
    fn range_zero_effect_and_flagged_effect() {
        let mut e = engine();
        item_doubles(&mut e);
        let flagged_self = item(&mut e, true, 0, 0);
        let self_item = item(&mut e, false, 0, 0);
        let touch = item(&mut e, false, 1, 0);
        let empty = list_of(&mut e, &[0]);
        assert!(!e.call(0x0040_6090, &args![empty]).bool());
        assert!(!e.call(0x0040_62a0, &args![empty]).bool());
        let a = list_of(&mut e, &[touch, flagged_self]);
        assert!(!fn_00406090(&mut e, a));
        assert!(fn_004062a0(&mut e, a));
        let b = list_of(&mut e, &[touch, 0, self_item]);
        assert!(fn_00406090(&mut e, b));
        assert!(!fn_004062a0(&mut e, b));
    }

    #[test]
    fn item_by_index_or_warning() {
        let mut e = engine();
        let list = list_of(&mut e, &[0x10, 0x20, 0x30]);
        assert_eq!(e.call(0x0040_6ee0, &args![list, 0u32]).u32(), 0x10);
        // Only the low byte of the index word counts.
        assert_eq!(e.call(0x0040_6ee0, &args![list, 0x102u32]).u32(), 0x30);
        e.call_log = Some(vec![]);
        assert_eq!(
            effect_item_list_get_effect_item_by_index(&mut e, list, 3),
            0
        );
        assert_eq!(
            calls_to(&e, PRINT_WARNING),
            vec![vec![NO_ITEM_AT_INDEX_TEXT]]
        );
        e.call_log = Some(vec![]);
        assert_eq!(
            effect_item_list_get_effect_item_by_index(&mut e, Ptr::new(0), 0),
            0
        );
        assert_eq!(calls_to(&e, PRINT_WARNING).len(), 1);
    }
}
