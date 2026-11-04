//! `fallout/interface/menus/levelupmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;
use crate::units::fallout_interface::startmenu::ListBox;

/// The constructor body `007883b0` (`this` plus two stack words, `RET 8`).
const CONSTRUCTOR_BODY_007883B0: u32 = 0x0078_83b0;
/// The vtable stored at `+0` (RTTI class `BSSimpleArray<EffectItemData,...>`).
const VTABLE_00788300: u32 = 0x0107_3e50;

// Translated from 00788300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `levelupmenu.cpp`): stores the vtable `01073e50` (RTTI
/// class `BSSimpleArray<EffectItemData,...>`), then calls `007883b0(this, 0, 0)`. Returns
/// `this`.
pub fn fn_00788300(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_00788300);
    e.call(CONSTRUCTOR_BODY_007883B0, &args![this, 0u32, 0u32]);
    this
}

/// `TESCondition::IsTrue_ov2` (engine map), thiscall (object, 0, result
/// byte, 0), `RET 0x10`: `AL`.
const CONDITION_IS_TRUE: u32 = 0x0068_0c60;
/// Thiscall (word), `RET 4`: `AL` (`effectitem.rs`).
const FLAG_TEST_00403E60: u32 = 0x0040_3e60;
/// Cdecl (value, mask): `AL` (`run_00406cc0.rs`).
const FLAG_TEST_00406D70: u32 = 0x0040_6d70;
/// The `ListBoxBase` constructor (`menumanager.rs`), thiscall.
const LIST_BOX_BASE_CONSTRUCT: u32 = 0x0072_40c0;
/// List constructor, thiscall (the embedded list at +4).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// List destructor, thiscall.
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// Thiscall (word, word), `RET 8`: the constructors pass 0 and 1.
const LIST_BOX_INIT_FLAGS: u32 = 0x007a_1950;
/// Cdecl (list box): the constructor hands the new list box to it
/// (`menumanager.rs`).
const LIST_BOX_REGISTER: u32 = 0x0072_1130;
/// Cdecl (list box): the destructor hands the list box to it.
const LIST_BOX_UNREGISTER: u32 = 0x0072_11c0;
/// `ListBox<BGSPerk *>`'s vtable (RTTI `.?AV?$ListBox@PAVBGSPerk@@@@`).
const PERK_LIST_BOX_VTABLE: u32 = 0x0107_3e28;
/// `Tile::GetMenu` (engine map), thiscall on a tile.
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// `Tile::GetMenuTile` (engine map), thiscall on a tile.
const TILE_GET_MENU_TILE: u32 = 0x00a0_3d40;
/// `Menu::RenderTemplate` (engine map), thiscall (parent tile, template
/// name, word), `RET 0xC`: the new tile.
const MENU_RENDER_TEMPLATE: u32 = 0x00a1_ddb0;
/// `Tile::GetValue` (engine map), thiscall (trait id), `RET 4`.
const TILE_GET_VALUE: u32 = 0x00a0_0f30;
/// Thiscall on a tile (trait id, integer), `RET 8`.
const TILE_SET_INT: u32 = 0x0070_0320;
/// Thiscall on a tile (trait id, text, word), `RET 0xC`.
const TILE_SET_STRING: u32 = 0x00a0_1350;
/// The allocator, cdecl (size).
const ALLOCATE: u32 = 0x0040_1000;
/// The allocator's free, cdecl (block).
const FREE: u32 = 0x0040_1030;
/// Thiscall on a new 0xC-byte item (tile, address of the data word),
/// `RET 8`: the item.
const ITEM_CONSTRUCT: u32 = 0x007a_62e0;
/// `BSSimpleList<ActiveEffect_P>::Insert` (engine map; folded code),
/// thiscall on the embedded list (item, compare function), `RET 8`.
const LIST_INSERT_SORTED: u32 = 0x007a_7eb0;
/// Thiscall on the embedded list (address of the item word), `RET 4`.
const LIST_APPEND: u32 = 0x0090_5820;
/// Thiscall on a list box (flag bit), `RET 4`: `AL` (`menumanager.rs`).
const LIST_BOX_HAS_FLAG: u32 = 0x0072_42c0;
/// `ListBox<StartMenu::Option_P>::InitTilePosition` (engine map; shared
/// body), thiscall (tile), `RET 4`.
const INIT_TILE_POSITION: u32 = 0x0072_69d0;
/// Thiscall on a list box, no arguments: `EAX` (compared with 1 after an
/// item is added; subtracted from by `NumUnfiltered`).
const LIST_BOX_COUNT_00726C40: u32 = 0x0072_6c40;
/// `Tile::AddUserTrait` (engine map), cdecl (name, word): the trait id.
const ADD_USER_TRAIT: u32 = 0x00a0_0940;
/// Thiscall on a tile (trait id), `RET 4`: a float in `ST0`.
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
/// `Tile::GetValueQ` (engine map), thiscall (trait id), `RET 4`.
const TILE_GET_VALUE_Q: u32 = 0x00a0_0e90;
/// `Tile::Value::ClearActions` (engine map), thiscall.
const VALUE_CLEAR_ACTIONS: u32 = 0x00a0_9200;
/// `Tile::Value::AddAction_ov2` (engine map), thiscall (action, tile,
/// trait id), `RET 0xC`.
const VALUE_ADD_ACTION_TILE: u32 = 0x00a0_9130;
/// `Tile::Value::AddAction` (engine map), thiscall (action, float), `RET 8`.
const VALUE_ADD_ACTION_FLOAT: u32 = 0x00a0_9080;
/// `Tile::Value::CalculateValue` (engine map), thiscall (word), `RET 4`.
const VALUE_CALCULATE: u32 = 0x00a0_9410;
/// Thiscall on a list node: returns the node itself (`run_006815c0.rs`); the
/// list box code calls it to reach the node's item word.
const NODE_SELF: u32 = 0x0068_15c0;
/// Thiscall on a list node: the next node (`menumanager.rs`).
const NODE_NEXT: u32 = 0x0072_6070;
/// Thiscall on a list node (word), `RET 4`: the `RemoveAll` loop passes 0
/// before deleting the node.
const NODE_SET_NEXT: u32 = 0x006e_cd40;
/// Thiscall on a list node (flags), `RET 4`: its deleting destructor.
const NODE_DELETE: u32 = 0x0047_02f0;
/// Thiscall on an item's data (flags), `RET 4`.
const ITEM_DATA_DELETE: u32 = 0x0044_59e0;
/// The byte that, when set, keeps the item tiles when a list box empties.
const KEEP_ITEM_TILES: u32 = 0x011d_8909;
/// Thiscall on a tile (trait id, float, word), `RET 0xC`.
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
/// `Tile::Poke` (engine map), thiscall (trait id, float), `RET 8`.
const TILE_POKE: u32 = 0x00a0_1290;
/// `Tile::TextToTrait` (engine map), cdecl (name): the trait id.
const TEXT_TO_TRAIT: u32 = 0x00a0_1860;
/// `Tile::IsTrue` (engine map), thiscall (trait id), `RET 4`: `AL`.
const TILE_IS_TRUE: u32 = 0x00a0_1230;
/// Thiscall on the embedded list (compare function), `RET 4`
/// (`run_0083fd60.rs`).
const LIST_SORT: u32 = 0x0083_fd60;
/// Thiscall on a list box (tile), `RET 4`: `AL`, whether the tile is one of
/// its items (`menumanager.rs`).
const LIST_BOX_BELONGS_TO: u32 = 0x0072_42f0;
/// Cdecl, no arguments: an object (`globalfunc.rs`).
const FOCUS_OWNER_004B7210: u32 = 0x004b_7210;
/// `InterfaceManager::SetCurrentFocusTarget` (engine map), thiscall (tile,
/// trait id, word), `RET 0xC`.
const SET_CURRENT_FOCUS_TARGET: u32 = 0x0071_5860;
/// Cdecl, no arguments: `AL` (`globalfunc.rs`).
const FLAG_004B71D0: u32 = 0x004b_71d0;
/// Cdecl (float, float): a float in `ST0` (`run_0040ea20.rs`).
const FLOAT_PAIR_0040EBD0: u32 = 0x0040_ebd0;
/// The compiler's float to integer conversion (`_ftol2`): the value in
/// `ST0` (a leading `f64`), the integer in `EAX`.
const FTOL2: u32 = 0x00ec_62c0;
/// Cdecl (float): a float in `ST0` (`teswater.rs`).
const ROUND_FLOAT: u32 = 0x004e_4430;
/// Cdecl (integer, integer): `EAX`.
const INT_PAIR_00647B70: u32 = 0x0064_7b70;
/// Thiscall on a tile: `AL` (the byte the highlight box copies).
const TILE_FLAG_GETTER_0071AA80: u32 = 0x0071_aa80;
/// Thiscall on a tile (byte), `RET 4`.
const TILE_FLAG_SETTER_0071AA60: u32 = 0x0071_aa60;
/// `Tile::AddNeedsUpdate` (engine map), thiscall (flags), `RET 4`.
const TILE_ADD_NEEDS_UPDATE: u32 = 0x00a0_7530;
/// Cdecl (tile, name): a tile found by name below it.
const TILE_FIND_BY_NAME: u32 = 0x00a0_8b20;
/// `Tile::GetChildByName` (engine map), thiscall (name), `RET 4`.
const TILE_GET_CHILD_BY_NAME: u32 = 0x00a0_3da0;
/// Thiscall on the array, no arguments: `AL`, whether it is full
/// (`run_00438af0.rs`).
const ARRAY_IS_FULL: u32 = 0x0043_8b90;
/// Thiscall on the array: the grown reserved size (`run_009a3910.rs`).
const ARRAY_GROWN_SIZE: u32 = 0x009a_3910;
/// Thiscall on the array (new reserved size, element count), `RET 8`.
const ARRAY_RESIZE: u32 = 0x009a_3c50;
/// Cdecl (size, place): placement `new` (`run_006e6da0.rs`): the place.
const PLACEMENT_NEW: u32 = 0x006e_6da0;
/// Thiscall: the 0x14-byte element constructor.
const ELEMENT_CONSTRUCT: u32 = 0x0040_3ba0;

/// `"_number_of_visible_items"`.
const NUMBER_OF_VISIBLE_ITEMS_TEXT: u32 = 0x0106_f860;
/// `"_num_filtered"`.
const NUM_FILTERED_TEXT: u32 = 0x0106_f8d0;
/// `"_current_value"`.
const CURRENT_VALUE_TEXT: u32 = 0x0107_02d8;
/// `"_SetInCode"`.
const SET_IN_CODE_TEXT: u32 = 0x0107_0250;
/// `"_enabled"`.
const ENABLED_TEXT: u32 = 0x0107_0244;
/// `"_selected"`.
const SELECTED_TEXT: u32 = 0x0107_1b64;
/// `"_highlight_y"`.
const HIGHLIGHT_Y_TEXT: u32 = 0x0106_f8c0;
/// `"_selected_height"`.
const SELECTED_HEIGHT_TEXT: u32 = 0x0106_f8ac;
/// `"lb_highlight_box"`.
const HIGHLIGHT_BOX_TEXT: u32 = 0x0106_f898;
/// `"top"`, `"bottom"`, `"left"`, `"right"`.
const TOP_TEXT: u32 = 0x0106_f894;
const BOTTOM_TEXT: u32 = 0x0106_f88c;
const LEFT_TEXT: u32 = 0x0106_f884;
const RIGHT_TEXT: u32 = 0x0106_f87c;
/// `"_scroll_delta"`.
const SCROLL_DELTA_TEXT: u32 = 0x0106_f90c;
/// `"_number_of_items"`.
const NUMBER_OF_ITEMS_TEXT: u32 = 0x0106_f8f8;
/// `0.0` (`double`).
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// `1.0` (`double`).
const ONE_DOUBLE: u32 = 0x0101_2070;

/// The function-local statics the list box methods keep trait ids in: the
/// guard word (bit `bit` says the id has been looked up) and the id.
#[derive(Clone, Copy)]
struct StaticTrait {
    guard: u32,
    bit: u32,
    cache: u32,
    name: u32,
}

/// `007877a0`'s `"_selected"` id.
const SELECTED_TRAIT_007877A0: StaticTrait = StaticTrait {
    guard: 0x011d_a094,
    bit: 1,
    cache: 0x011d_a090,
    name: SELECTED_TEXT,
};
/// `NumUnfiltered`'s `"_num_filtered"` id.
const NUM_FILTERED_TRAIT: StaticTrait = StaticTrait {
    guard: 0x011d_a09c,
    bit: 1,
    cache: 0x011d_a098,
    name: NUM_FILTERED_TEXT,
};
/// `IsEnabled`'s `"_enabled"` id.
const ENABLED_TRAIT: StaticTrait = StaticTrait {
    guard: 0x011d_a0a4,
    bit: 1,
    cache: 0x011d_a0a0,
    name: ENABLED_TEXT,
};
/// `00787c40`'s `"_selected"` id.
const SELECTED_TRAIT_00787C40: StaticTrait = StaticTrait {
    guard: 0x011d_a0ac,
    bit: 1,
    cache: 0x011d_a0a8,
    name: SELECTED_TEXT,
};
/// `UpdateHighlightBoxPos`'s `"_highlight_y"` id (bit 1 of the guard).
const HIGHLIGHT_Y_TRAIT: StaticTrait = StaticTrait {
    guard: 0x011d_a0b8,
    bit: 1,
    cache: 0x011d_a0b4,
    name: HIGHLIGHT_Y_TEXT,
};
/// `UpdateHighlightBoxPos`'s `"_selected_height"` id (bit 2 of the guard).
const SELECTED_HEIGHT_TRAIT: StaticTrait = StaticTrait {
    guard: 0x011d_a0b8,
    bit: 2,
    cache: 0x011d_a0b0,
    name: SELECTED_HEIGHT_TEXT,
};

/// Looks the static trait id up the first time: the guard bit is set before
/// `Tile::AddUserTrait(name, -1)` runs, as the code does. Returns the id
/// read back from the static.
fn static_trait(e: &mut Engine, which: StaticTrait) -> u32 {
    if e.mem.u32(which.guard) & which.bit == 0 {
        let guard = e.mem.u32(which.guard);
        e.mem.set_u32(which.guard, guard | which.bit);
        let id = e
            .call(ADD_USER_TRAIT, &args![which.name, 0xffff_ffffu32])
            .u32();
        e.mem.set_u32(which.cache, id);
    }
    e.mem.u32(which.cache)
}

/// The item word of a list node, reached through `006815c0` as the code
/// does.
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let node = e.call(NODE_SELF, &args![node]).u32();
    e.mem.u32(node)
}

/// The first node of the embedded list (`this + 4`; the code guards a null
/// `this`).
fn first_node(this: Ptr<ListBox>) -> u32 {
    if this.is_null() {
        0
    } else {
        this.addr() + 4
    }
}

/// `FCOMP; FNSTSW AX; TEST AH,0x41; JNZ`: the code goes on only when the
/// value is strictly above the other (not less, equal or unordered).
fn above(a: f64, b: f64) -> bool {
    a.partial_cmp(&b) == Some(std::cmp::Ordering::Greater)
}

// Translated from 00785150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `levelupmenu.cpp`): `TESCondition::IsTrue_ov2`
/// (`00680c60`) on the condition at `this + 0x40` with `object`, 0, a stack
/// byte for its out parameter and 0; returns its `AL`.
pub fn fn_00785150(e: &mut Engine, this: Ptr, object: u32) -> bool {
    e.with_stack(4, |e, result| {
        e.call(
            CONDITION_IS_TRUE,
            &args![this.addr() + 0x40, object, 0u32, result, 0u32],
        )
        .bool()
    })
}

// Translated from 007869e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `first = 00403e60(this, 4)`; `second` is 0
/// when `value` is -1, otherwise `00406d70(value, 0x200)`; returns 1 when
/// exactly one of them is set.
pub fn fn_007869e0(e: &mut Engine, this: Ptr, value: i32) -> u8 {
    let first = e.call(FLAG_TEST_00403E60, &args![this, 4u32]).u8();
    let mut second = 0u8;
    if value != -1 {
        second = e.call(FLAG_TEST_00406D70, &args![value, 0x200u32]).u8();
    }
    if (first == 0 || second != 0) && (first != 0 || second == 0) {
        0
    } else {
        1
    }
}

// Translated from 00786bb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00786bb0 - the C++ exception frame (FS:[0] chain, unwind state 0 after the embedded list is built) is not translated.
/// The `ListBox<BGSPerk *>` constructor (engine map unnamed; RTTI of the
/// vtable `01073e28`): the `ListBoxBase` base (`007240c0`), the embedded
/// list (+4, `0096a2d0`), the vtable, the fields cleared, `007a1950(this, 0,
/// 1)` and the list box handed to `00721130`. Returns `this`.
pub fn fn_00786bb0(e: &mut Engine, this: Ptr<ListBox>) -> Ptr<ListBox> {
    e.call(LIST_BOX_BASE_CONSTRUCT, &args![this]);
    e.call(LIST_CONSTRUCT, &args![this.addr() + 4]);
    e.mem.set_u32(this.addr(), PERK_LIST_BOX_VTABLE);
    e.set(this, ListBox::pScrollBar, Ptr::NULL);
    e.set(this, ListBox::pHighlightedItem, Ptr::NULL);
    e.set(this, ListBox::pContainerRect, Ptr::NULL);
    e.set(this, ListBox::pTemplateName, Ptr::NULL);
    e.set(this, ListBox::usFlags, 0);
    e.set(this, ListBox::usNextIndex, 0);
    e.set(this, ListBox::fSavedScrollbarValue, 0.0);
    e.set(this, ListBox::fSavedHighlightIndex, 0.0);
    e.set(this, ListBox::fTotalListHeight, 0.0);
    e.call(LIST_BOX_INIT_FLAGS, &args![this, 0u32, 1u32]);
    e.call(LIST_BOX_REGISTER, &args![this]);
    this
}

// Translated from 00786c80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00786c80 - the C++ exception frame (FS:[0] chain, unwind state 0 around the body) is not translated.
/// The `ListBox<BGSPerk *>` destructor: the vtable, `RemoveAll`, the list
/// box handed to `007211c0`, then the embedded list destroyed (`0046ffb0`).
pub fn fn_00786c80(e: &mut Engine, this: Ptr<ListBox>) {
    e.mem.set_u32(this.addr(), PERK_LIST_BOX_VTABLE);
    list_box_bgs_perk_remove_all(e, this);
    e.call(LIST_BOX_UNREGISTER, &args![this]);
    e.call(LIST_DESTRUCT, &args![this.addr() + 4]);
}

// Translated from 00786cf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00786cf0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the item allocations) is not translated.
/// `ListBox<BGSPerk_P>::AddItem` (engine map): with a container and a
/// template name (`template`, else `pTemplateName`), renders the template
/// in the container's menu (`Menu::RenderTemplate`), gives the new tile
/// trait `0xfaa` -1 when it has none and the text `text` (trait `0xfc4`)
/// when there is one, and adds an item (tile, `data`): sorted with
/// `compare` (then `Synchronize` when flag 0 is set) or appended (then,
/// with flag 0, `InitTilePosition` and `SetTotalListHeight`, and the tile
/// gets the next index as trait `0xfac`). When the list then counts 1
/// (`00726c40`) and the container's `"_number_of_visible_items"` is above
/// `0.0`, the container's height value (`0xfb0`) is rebuilt from the tile's
/// height times that number. Returns the tile, or 0 without a container or
/// template.
pub fn list_box_bgs_perk_add_item(
    e: &mut Engine,
    this: Ptr<ListBox>,
    data: u32,
    text: u32,
    compare: u32,
    template: u32,
) -> u32 {
    if e.get(this, ListBox::pContainerRect).is_null() {
        return 0;
    }
    if e.get(this, ListBox::pTemplateName).is_null() && template == 0 {
        return 0;
    }
    let name = if template != 0 {
        template
    } else {
        e.get(this, ListBox::pTemplateName).addr()
    };
    let container = e.get(this, ListBox::pContainerRect);
    let menu = e.call(TILE_GET_MENU, &args![container]).u32();
    let container = e.get(this, ListBox::pContainerRect);
    let tile = e
        .call(MENU_RENDER_TEMPLATE, &args![menu, container, name, 0u32])
        .u32();
    if e.call(TILE_GET_VALUE, &args![tile, 0xfaau32]).u32() == 0 {
        e.call(TILE_SET_INT, &args![tile, 0xfaau32, 0xffff_ffffu32]);
    }
    if text != 0 {
        e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, text, 1u32]);
    }
    // The item is built with the address of the `data` argument's stack
    // slot.
    let item = e.with_stack(4, |e, data_slot| {
        e.mem.set_u32(data_slot.addr(), data);
        let block = e.call(ALLOCATE, &args![0xcu32]).u32();
        if block != 0 {
            e.call(ITEM_CONSTRUCT, &args![block, tile, data_slot]).u32()
        } else {
            0
        }
    });
    if compare != 0 {
        e.call(LIST_INSERT_SORTED, &args![this.addr() + 4, item, compare]);
        if e.call(LIST_BOX_HAS_FLAG, &args![this, 0u32]).bool() {
            list_box_bgs_perk_synchronize(e, this);
        }
    } else {
        e.with_stack(4, |e, item_slot| {
            e.mem.set_u32(item_slot.addr(), item);
            e.call(LIST_APPEND, &args![this.addr() + 4, item_slot]);
        });
        if e.call(LIST_BOX_HAS_FLAG, &args![this, 0u32]).bool() {
            e.call(INIT_TILE_POSITION, &args![this, tile]);
            list_box_bgs_perk_set_total_list_height(e, this);
        }
        let index = e.get(this, ListBox::usNextIndex);
        e.set(this, ListBox::usNextIndex, index.wrapping_add(1));
        e.call(TILE_SET_INT, &args![tile, 0xfacu32, u32::from(index)]);
    }
    if e.call(LIST_BOX_COUNT_00726C40, &args![this]).u32() == 1 {
        let visible = e
            .call(
                ADD_USER_TRAIT,
                &args![NUMBER_OF_VISIBLE_ITEMS_TEXT, 0xffff_ffffu32],
            )
            .u32();
        let container = e.get(this, ListBox::pContainerRect);
        let count = e.call(TILE_GET_FLOAT, &args![container, visible]).f64();
        let zero: f64 = e.global(ZERO_DOUBLE);
        if above(count, zero) {
            let container = e.get(this, ListBox::pContainerRect);
            let value = e.call(TILE_GET_VALUE_Q, &args![container, 0xfb0u32]).u32();
            e.call(VALUE_CLEAR_ACTIONS, &args![value]);
            e.call(
                VALUE_ADD_ACTION_TILE,
                &args![value, 0x7d0u32, tile, 0xfb0u32],
            );
            let visible = e
                .call(
                    ADD_USER_TRAIT,
                    &args![NUMBER_OF_VISIBLE_ITEMS_TEXT, 0xffff_ffffu32],
                )
                .u32();
            let container = e.get(this, ListBox::pContainerRect);
            let count = e.call(TILE_GET_FLOAT, &args![container, visible]).f64();
            e.call(
                VALUE_ADD_ACTION_FLOAT,
                &args![value, 0x7d3u32, x87_store32(count)],
            );
            e.call(VALUE_CALCULATE, &args![value, 0u32]);
        }
    }
    tile
}

/// Deletes the `ListBoxItem` (`pTile` +0, `data` +4) that `item` reads each
/// time: the tile through its deleting destructor (vtable slot 0, flag 1)
/// unless `KEEP_ITEM_TILES` is set, the data (`004459e0` with 1) when flag
/// bit 1 is set, then the item freed.
fn delete_item(e: &mut Engine, this: Ptr<ListBox>, item: impl Fn(&mut Engine) -> u32) {
    if e.global::<u8>(KEEP_ITEM_TILES) == 0 {
        let entry = item(e);
        let tile = e.mem.u32(entry);
        if tile != 0 {
            e.vcall(tile, 0, &args![1u32]);
        }
    }
    if e.call(LIST_BOX_HAS_FLAG, &args![this, 1u32]).bool() {
        let entry = item(e);
        let data = e.mem.u32(entry + 4);
        if data != 0 {
            e.call(ITEM_DATA_DELETE, &args![data, 1u32]);
        }
    }
    let entry = item(e);
    e.call(FREE, &args![entry]);
}

// Translated from 00786f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSPerk_P>::RemoveAll` (engine map): while the second node
/// (+8) and its item are non-null, deletes the item, clears the node's next
/// pointer (`006ecd40` with 0), deletes the node (`004702f0` with 1) and
/// makes the following node the second; then deletes the first item (+4).
/// Clears the first item, `usNextIndex` and `fTotalListHeight`, sets the
/// container's `"_num_filtered"` to `0.0`, then `SetTotalListHeight`,
/// clears `pHighlightedItem` and `UpdateHighlightBoxPos`.
pub fn list_box_bgs_perk_remove_all(e: &mut Engine, this: Ptr<ListBox>) {
    let base = this.addr();
    loop {
        let second = e.mem.u32(base + 8);
        if second == 0 || node_item(e, second) == 0 {
            break;
        }
        let second = e.mem.u32(base + 8);
        let next = e.call(NODE_NEXT, &args![second]).u32();
        delete_item(e, this, |e| {
            let node = e.mem.u32(base + 8);
            node_item(e, node)
        });
        let second = e.mem.u32(base + 8);
        e.call(NODE_SET_NEXT, &args![second, 0u32]);
        let second = e.mem.u32(base + 8);
        if second != 0 {
            e.call(NODE_DELETE, &args![second, 1u32]);
        }
        e.mem.set_u32(base + 8, next);
    }
    if e.mem.u32(base + 4) != 0 {
        delete_item(e, this, |e| e.mem.u32(base + 4));
    }
    e.mem.set_u32(base + 4, 0);
    e.set(this, ListBox::usNextIndex, 0);
    e.set(this, ListBox::fTotalListHeight, 0.0);
    if !e.get(this, ListBox::pContainerRect).is_null() {
        let num_filtered = e.call(TEXT_TO_TRAIT, &args![NUM_FILTERED_TEXT]).u32();
        let container = e.get(this, ListBox::pContainerRect);
        e.call(
            TILE_SET_FLOAT,
            &args![container, num_filtered, 0.0f32.to_bits(), 1u32],
        );
    }
    list_box_bgs_perk_set_total_list_height(e, this);
    e.set(this, ListBox::pHighlightedItem, Ptr::NULL);
    list_box_bgs_perk_update_highlight_box_pos(e, this);
}

// Translated from 007871c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSPerk_P>::Sort` (engine map): sorts the embedded list
/// (`0083fd60` on +4 with `compare`), then `Synchronize`.
pub fn list_box_bgs_perk_sort(e: &mut Engine, this: Ptr<ListBox>, compare: u32) {
    e.call(LIST_SORT, &args![this.addr() + 4, compare]);
    list_box_bgs_perk_synchronize(e, this);
}

// Translated from 007871f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSPerk_P>::Highlight` (engine map): when the list is enabled
/// and `tile` is null, or is one of its items (`007242f0`) and
/// `"_enabled"`, makes it `pHighlightedItem`, moves the highlight box and
/// returns true; otherwise false.
pub fn list_box_bgs_perk_highlight(e: &mut Engine, this: Ptr<ListBox>, tile: u32) -> bool {
    if !list_box_bgs_perk_is_enabled(e, this) {
        return false;
    }
    if tile != 0 {
        if !e.call(LIST_BOX_BELONGS_TO, &args![this, tile]).bool() {
            return false;
        }
        let enabled = e
            .call(ADD_USER_TRAIT, &args![ENABLED_TEXT, 0xffff_ffffu32])
            .u32();
        if !e.call(TILE_IS_TRUE, &args![tile, enabled]).bool() {
            return false;
        }
    }
    e.set(this, ListBox::pHighlightedItem, Ptr::new(tile));
    list_box_bgs_perk_update_highlight_box_pos(e, this);
    true
}

// Translated from 00787600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSPerk_P>::RestoreListState` (engine map): with a container
/// and a scroll bar, gives the scroll bar `"_current_value"` (the saved
/// scroll bar value, word 0) and pokes `"_SetInCode"` with `1.0`; takes the
/// tile at index `_ftol(0040ebd0(fSavedHighlightIndex, NumUnfiltered -
/// 1.0))` (vtable +0x14 with 0), makes it the interface's focus target
/// (`00715860` with trait `0xfc3` and `focus`) and highlights it (vtable
/// +0). Unless `004b71d0` says so, also scrolls the container's menu to it
/// (the menu's vtable +0x14 with the tile's index trait `0xfaa` and the
/// tile, when there is a tile) and highlights nothing.
pub fn list_box_bgs_perk_restore_list_state(e: &mut Engine, this: Ptr<ListBox>, focus: u8) {
    if e.get(this, ListBox::pContainerRect).is_null() || e.get(this, ListBox::pScrollBar).is_null()
    {
        return;
    }
    let saved_value = e.mem.u32(this.addr() + 0x28);
    let current_value = e.call(TEXT_TO_TRAIT, &args![CURRENT_VALUE_TEXT]).u32();
    let scroll_bar = e.get(this, ListBox::pScrollBar);
    e.call(
        TILE_SET_FLOAT,
        &args![scroll_bar, current_value, x87_quiet_bits(saved_value), 0u32],
    );
    let set_in_code = e.call(TEXT_TO_TRAIT, &args![SET_IN_CODE_TEXT]).u32();
    let scroll_bar = e.get(this, ListBox::pScrollBar);
    e.call(TILE_POKE, &args![scroll_bar, set_in_code, 1.0f32.to_bits()]);
    let count = list_box_bgs_perk_num_unfiltered(e, this) as u32;
    let one: f64 = e.global(ONE_DOUBLE);
    let last = x87_store32(x87_sub(f64::from(count), one));
    let saved_index = e.mem.u32(this.addr() + 0x24);
    let index = e
        .call(
            FLOAT_PAIR_0040EBD0,
            &args![x87_quiet_bits(saved_index), x87_quiet_bits(last)],
        )
        .f64();
    let index = x87_fistp64_chop(X87Ext::from(index)) as u32;
    let tile = e.vcall(this.addr(), 0x14, &args![index, 0u32]).u32();
    let owner = e.call(FOCUS_OWNER_004B7210, &args![]).u32();
    e.call(
        SET_CURRENT_FOCUS_TARGET,
        &args![owner, tile, 0xfc3u32, u32::from(focus)],
    );
    e.vcall(this.addr(), 0, &args![tile]);
    if e.call(FLAG_004B71D0, &args![]).bool() {
        return;
    }
    if tile != 0 {
        let container = e.get(this, ListBox::pContainerRect);
        let menu_tile = e.call(TILE_GET_MENU_TILE, &args![container]).u32();
        let menu = e.call(TILE_GET_MENU, &args![menu_tile]).u32();
        let value = e.call(TILE_GET_FLOAT, &args![tile, 0xfaau32]).f64();
        let item_index = e.call(FTOL2, &args![value]).u32();
        e.vcall(menu, 0x14, &args![item_index, tile]);
    }
    e.vcall(this.addr(), 0, &args![0u32]);
}

// Translated from 00787760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `00787c40(this, tile, 0)` (select).
pub fn fn_00787760(e: &mut Engine, this: Ptr<ListBox>, tile: u32) {
    fn_00787c40(e, this, tile, 0);
}

// Translated from 00787780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `00787c40(this, tile, 2)` (toggle).
pub fn fn_00787780(e: &mut Engine, this: Ptr<ListBox>, tile: u32) {
    fn_00787c40(e, this, tile, 2);
}

// Translated from 007877a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007877a0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// Unnamed in the engine map: whether `tile` is selected: the
/// `"_selected"` trait id (a function-local static, `011da090`) is non-zero,
/// `tile` is non-null and `Tile::IsTrue(tile, id)`.
pub fn fn_007877a0(e: &mut Engine, this: Ptr<ListBox>, tile: u32) -> bool {
    let _ = this;
    let selected = static_trait(e, SELECTED_TRAIT_007877A0);
    if selected == 0 || tile == 0 {
        return false;
    }
    let selected = e.mem.u32(SELECTED_TRAIT_007877A0.cache);
    e.call(TILE_IS_TRUE, &args![tile, selected]).bool()
}

// Translated from 007878d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the number of items whose tile is selected
/// (`007877a0`), walking the nodes while the node and its item are
/// non-null.
pub fn fn_007878d0(e: &mut Engine, this: Ptr<ListBox>) -> u32 {
    let mut count = 0u32;
    let mut node = first_node(this);
    while node != 0 && node_item(e, node) != 0 {
        let item = node_item(e, node);
        let tile = e.mem.u32(item);
        if fn_007877a0(e, this, tile) {
            count = count.wrapping_add(1);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    count
}

// Translated from 00787950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`BSSimpleArray<EffectItemData>`, elements of
/// 0x14 bytes): takes the next slot (`007881d0`), constructs one element
/// there (`00788260`) and copies the five words at `element` into it.
/// Returns the slot's index.
pub fn fn_00787950(e: &mut Engine, this: Ptr<BSSimpleArray>, element: u32) -> u32 {
    let index = fn_007881d0(e, this);
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    fn_00788260(e, this, index.wrapping_mul(0x14).wrapping_add(buffer), 1);
    let slot = e
        .get(this, BSSimpleArray::pBuffer)
        .wrapping_add(index.wrapping_mul(0x14));
    for word in 0..5 {
        let value = e.mem.u32(element + 4 * word);
        e.mem.set_u32(slot + 4 * word, value);
    }
    index
}

// Translated from 007879f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007879f0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<BGSPerk_P>::NumUnfiltered` (engine map): `00726c40(this)` minus
/// the container's `"_num_filtered"` (a function-local static id; the float
/// truncated by `FISTP qword`).
pub fn list_box_bgs_perk_num_unfiltered(e: &mut Engine, this: Ptr<ListBox>) -> i32 {
    static_trait(e, NUM_FILTERED_TRAIT);
    let count = e.call(LIST_BOX_COUNT_00726C40, &args![this]).u32();
    let num_filtered = e.mem.u32(NUM_FILTERED_TRAIT.cache);
    let container = e.get(this, ListBox::pContainerRect);
    let value = e
        .call(TILE_GET_FLOAT, &args![container, num_filtered])
        .f64();
    let filtered = x87_fistp64_chop(X87Ext::from(value)) as u32;
    count.wrapping_sub(filtered) as i32
}

// Translated from 00787aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSPerk_P>::Synchronize` (engine map): clears `usNextIndex`
/// and `fTotalListHeight`, then for every node with an item whose filtered
/// byte (+8) is clear gives the item's tile the next index (trait `0xfac`)
/// and its position (`InitTilePosition`); then `SetTotalListHeight` and
/// `UpdateHighlightBoxPos`.
pub fn list_box_bgs_perk_synchronize(e: &mut Engine, this: Ptr<ListBox>) {
    e.set(this, ListBox::usNextIndex, 0);
    e.set(this, ListBox::fTotalListHeight, 0.0);
    let mut node = first_node(this);
    while node != 0 && node_item(e, node) != 0 {
        let item = node_item(e, node);
        if e.mem.u8(item + 8) == 0 {
            let index = e.get(this, ListBox::usNextIndex);
            e.set(this, ListBox::usNextIndex, index.wrapping_add(1));
            let item = node_item(e, node);
            let tile = e.mem.u32(item);
            e.call(TILE_SET_INT, &args![tile, 0xfacu32, u32::from(index)]);
            let item = node_item(e, node);
            let tile = e.mem.u32(item);
            e.call(INIT_TILE_POSITION, &args![this, tile]);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    list_box_bgs_perk_set_total_list_height(e, this);
    list_box_bgs_perk_update_highlight_box_pos(e, this);
}

// Translated from 00787b70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00787b70 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<BGSPerk_P>::IsEnabled` (engine map): the container exists and
/// its `"_enabled"` trait (a function-local static id) is true.
pub fn list_box_bgs_perk_is_enabled(e: &mut Engine, this: Ptr<ListBox>) -> bool {
    static_trait(e, ENABLED_TRAIT);
    let container = e.get(this, ListBox::pContainerRect);
    if container.is_null() {
        return false;
    }
    let enabled = e.mem.u32(ENABLED_TRAIT.cache);
    let container = e.get(this, ListBox::pContainerRect);
    e.call(TILE_IS_TRUE, &args![container, enabled]).bool()
}

// Translated from 00787c40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00787c40 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// Unnamed in the engine map: when the list is enabled and `tile` is one of
/// its items, changes the tile's `"_selected"` trait (a function-local
/// static id): `mode` 0 sets it when clear, 1 clears it when set, 2 flips
/// it. Returns true when it was changed; false otherwise (and for any other
/// mode).
pub fn fn_00787c40(e: &mut Engine, this: Ptr<ListBox>, tile: u32, mode: u32) -> bool {
    if !list_box_bgs_perk_is_enabled(e, this) || tile == 0 {
        return false;
    }
    if !e.call(LIST_BOX_BELONGS_TO, &args![this, tile]).bool() {
        return false;
    }
    let selected_trait = static_trait(e, SELECTED_TRAIT_00787C40);
    let selected = e.call(TILE_IS_TRUE, &args![tile, selected_trait]).u8();
    match mode {
        0 => {
            if selected == 0 {
                let id = e.mem.u32(SELECTED_TRAIT_00787C40.cache);
                e.call(TILE_SET_INT, &args![tile, id, 1u32]);
                return true;
            }
            false
        }
        1 => {
            if selected != 0 {
                let id = e.mem.u32(SELECTED_TRAIT_00787C40.cache);
                e.call(TILE_SET_INT, &args![tile, id, 0u32]);
                return true;
            }
            false
        }
        2 => {
            let value = u32::from(selected == 0);
            let id = e.mem.u32(SELECTED_TRAIT_00787C40.cache);
            e.call(TILE_SET_INT, &args![tile, id, value]);
            true
        }
        _ => false,
    }
}

/// The tile's byte (`0071aa60`) set to `flag` and the tile queued for an
/// update with flags 8.
fn mark_tile(e: &mut Engine, tile: u32, flag: u32) {
    e.call(TILE_FLAG_SETTER_0071AA60, &args![tile, flag]);
    e.call(TILE_ADD_NEEDS_UPDATE, &args![tile, 8u32]);
}

// Translated from 00787d80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00787d80 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the statics' look-ups) is not translated.
/// `ListBox<BGSPerk_P>::UpdateHighlightBoxPos` (engine map): looks the
/// `"_highlight_y"` and `"_selected_height"` trait ids up (statics); with a
/// container and no highlighted item, sets them to -1 and 0 on the
/// container; with one, makes both values follow the highlighted tile's
/// traits `0xfa2` and `0xfb0` (action `0x7d0`, then calculated) and gives
/// the container's `"lb_highlight_box"` tile, and its `"top"`, `"bottom"`,
/// `"left"` and `"right"` children when `"top"` exists (the other three are
/// not checked for null), the highlighted tile's byte (`0071aa80`, as 1 or
/// 0) and an update.
pub fn list_box_bgs_perk_update_highlight_box_pos(e: &mut Engine, this: Ptr<ListBox>) {
    static_trait(e, HIGHLIGHT_Y_TRAIT);
    static_trait(e, SELECTED_HEIGHT_TRAIT);
    if e.get(this, ListBox::pContainerRect).is_null() {
        return;
    }
    if e.get(this, ListBox::pHighlightedItem).is_null() {
        let y_trait = e.mem.u32(HIGHLIGHT_Y_TRAIT.cache);
        let container = e.get(this, ListBox::pContainerRect);
        e.call(TILE_SET_INT, &args![container, y_trait, 0xffff_ffffu32]);
        let height_trait = e.mem.u32(SELECTED_HEIGHT_TRAIT.cache);
        let container = e.get(this, ListBox::pContainerRect);
        e.call(TILE_SET_INT, &args![container, height_trait, 0u32]);
        return;
    }
    let y_trait = e.mem.u32(HIGHLIGHT_Y_TRAIT.cache);
    let container = e.get(this, ListBox::pContainerRect);
    let y_value = e.call(TILE_GET_VALUE, &args![container, y_trait]).u32();
    let height_trait = e.mem.u32(SELECTED_HEIGHT_TRAIT.cache);
    let container = e.get(this, ListBox::pContainerRect);
    let height_value = e
        .call(TILE_GET_VALUE, &args![container, height_trait])
        .u32();
    let highlighted = e.get(this, ListBox::pHighlightedItem);
    e.call(
        VALUE_ADD_ACTION_TILE,
        &args![y_value, 0x7d0u32, highlighted, 0xfa2u32],
    );
    e.call(VALUE_CALCULATE, &args![y_value, 0u32]);
    let highlighted = e.get(this, ListBox::pHighlightedItem);
    e.call(
        VALUE_ADD_ACTION_TILE,
        &args![height_value, 0x7d0u32, highlighted, 0xfb0u32],
    );
    e.call(VALUE_CALCULATE, &args![height_value, 0u32]);
    let highlighted = e.get(this, ListBox::pHighlightedItem);
    // The two branches of the code differ only in the byte they pass.
    let flag = u32::from(
        e.call(TILE_FLAG_GETTER_0071AA80, &args![highlighted])
            .bool(),
    );
    let container = e.get(this, ListBox::pContainerRect);
    let box_tile = e
        .call(TILE_FIND_BY_NAME, &args![container, HIGHLIGHT_BOX_TEXT])
        .u32();
    if box_tile == 0 {
        return;
    }
    let top = e
        .call(TILE_GET_CHILD_BY_NAME, &args![box_tile, TOP_TEXT])
        .u32();
    if top != 0 {
        mark_tile(e, top, flag);
        for name in [BOTTOM_TEXT, LEFT_TEXT, RIGHT_TEXT] {
            let edge = e.call(TILE_GET_CHILD_BY_NAME, &args![box_tile, name]).u32();
            mark_tile(e, edge, flag);
        }
    }
    mark_tile(e, box_tile, flag);
}

// Translated from 007880a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSPerk_P>::SetTotalListHeight` (engine map): with a
/// container, its visible height (trait `0xfb0`, stored as a `float`) and
/// the total list height decide the number of scroll steps: 1, unless both
/// are above `0.0` and the visible height is below the total, then
/// `00647b70(_ftol2(004e4430(unfiltered * (total - visible) / total)) + 1,
/// 2)` (the ratio stored as a `float`). With both above `0.0` and more than
/// one step, the container's `"_scroll_delta"` becomes `(total - visible) /
/// (steps - 1)`; with a scroll bar its `"_number_of_items"` becomes the
/// number of steps.
pub fn list_box_bgs_perk_set_total_list_height(e: &mut Engine, this: Ptr<ListBox>) {
    let container = e.get(this, ListBox::pContainerRect);
    if container.is_null() {
        return;
    }
    let visible = x87_store32(e.call(TILE_GET_FLOAT, &args![container, 0xfb0u32]).f64());
    let mut steps = 1i32;
    let zero: f64 = e.global(ZERO_DOUBLE);
    let total_bits = e.mem.u32(this.addr() + 0x20);
    if above(x87_load32(total_bits), zero) && above(x87_load32(visible), zero) {
        // `FLD total; FLD visible; FCOMPP; TEST AH,5; JP`: on only when the
        // visible height is below the total (ordered).
        let total_bits = e.mem.u32(this.addr() + 0x20);
        if x87_load32(visible) < x87_load32(total_bits) {
            let count = list_box_bgs_perk_num_unfiltered(e, this) as u32;
            let total_bits = e.mem.u32(this.addr() + 0x20);
            let difference = x87_sub(x87_load32(total_bits), x87_m32(visible));
            let product = x87_mul(f64::from(count), difference);
            let total_bits = e.mem.u32(this.addr() + 0x20);
            let ratio = x87_store32(x87_div(product, x87_m32(total_bits)));
            let rounded = e.call(ROUND_FLOAT, &args![x87_quiet_bits(ratio)]).f64();
            let whole = e.call(FTOL2, &args![rounded]).i32();
            steps = e
                .call(INT_PAIR_00647B70, &args![whole.wrapping_add(1), 2i32])
                .i32();
        }
        if steps > 1 {
            let total_bits = e.mem.u32(this.addr() + 0x20);
            let difference = x87_sub(x87_load32(total_bits), x87_m32(visible));
            let delta = x87_store32(x87_div(difference, f64::from(steps - 1)));
            let scroll_delta = e.call(TEXT_TO_TRAIT, &args![SCROLL_DELTA_TEXT]).u32();
            let container = e.get(this, ListBox::pContainerRect);
            e.call(
                TILE_SET_FLOAT,
                &args![container, scroll_delta, x87_quiet_bits(delta), 1u32],
            );
        }
    }
    if !e.get(this, ListBox::pScrollBar).is_null() {
        let number_of_items = e.call(TEXT_TO_TRAIT, &args![NUMBER_OF_ITEMS_TEXT]).u32();
        let scroll_bar = e.get(this, ListBox::pScrollBar);
        e.call(TILE_SET_INT, &args![scroll_bar, number_of_items, steps]);
    }
}

// Translated from 007881d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`BSSimpleArray<EffectItemData>`): makes room
/// for one more element when the array is full (`00438b90`): a first
/// buffer of 4 from vtable +4 when nothing is reserved, otherwise the
/// grown size (`009a3910`) through `009a3c50(new size, iSize)`; then
/// counts the element and returns its index (`iSize - 1`).
pub fn fn_007881d0(e: &mut Engine, this: Ptr<BSSimpleArray>) -> u32 {
    if e.call(ARRAY_IS_FULL, &args![this]).bool() {
        if e.get(this, BSSimpleArray::iReservedSize) == 0 {
            let size = 4u32;
            let buffer = e.vcall(this.addr(), 4, &args![size]).u32();
            e.set(this, BSSimpleArray::pBuffer, buffer);
            e.set(this, BSSimpleArray::iReservedSize, size);
        } else {
            let size = e.call(ARRAY_GROWN_SIZE, &args![this]).u32();
            let count = e.get(this, BSSimpleArray::iSize);
            e.call(ARRAY_RESIZE, &args![this, size, count]);
            e.set(this, BSSimpleArray::iReservedSize, size);
        }
    }
    let count = e.get(this, BSSimpleArray::iSize);
    e.set(this, BSSimpleArray::iSize, count.wrapping_add(1));
    e.get(this, BSSimpleArray::iSize).wrapping_sub(1)
}

// Translated from 00788260 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00788260 - the C++ exception frame (FS:[0] chain, unwind state 0 around each element's construction) is not translated.
/// Unnamed in the engine map (`BSSimpleArray<EffectItemData>`): constructs
/// `count` elements of 0x14 bytes from `place` (placement `new`,
/// `006e6da0`, then `00403ba0` on a non-null place).
pub fn fn_00788260(e: &mut Engine, this: Ptr<BSSimpleArray>, place: u32, count: u32) {
    let _ = this;
    let mut index = 0u32;
    while index < count {
        let element = e
            .call(
                PLACEMENT_NEW,
                &args![0x14u32, index.wrapping_mul(0x14).wrapping_add(place)],
            )
            .u32();
        if element != 0 {
            e.call(ELEMENT_CONSTRUCT, &args![element]);
        }
        index += 1;
    }
}

// Translated from 00788330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`BSSimpleArray<EffectItemData>`'s buffer
/// allocation, vtable +4): `count * 0x14` bytes from the allocator.
pub fn fn_00788330(e: &mut Engine, this: Ptr<BSSimpleArray>, count: u32) -> u32 {
    let _ = this;
    e.call(ALLOCATE, &args![count.wrapping_mul(0x14)]).u32()
}

// Translated from 007883b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`BSSimpleArray<EffectItemData>`'s constructor
/// body): clears the buffer, size and reserved size; reserves
/// `max(reserve, size)` elements through vtable +4 when that is non-zero,
/// then constructs `size` elements (`00788260`) and sets the size.
pub fn fn_007883b0(e: &mut Engine, this: Ptr<BSSimpleArray>, reserve: u32, size: u32) {
    e.set(this, BSSimpleArray::pBuffer, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    e.set(this, BSSimpleArray::iReservedSize, 0);
    let mut reserve = reserve;
    if size > reserve {
        reserve = size;
    }
    if reserve > 0 {
        let buffer = e.vcall(this.addr(), 4, &args![reserve]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, reserve);
    }
    if size > 0 {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        fn_00788260(e, this, buffer, size);
        e.set(this, BSSimpleArray::iSize, size);
    }
}

// --- LevelUpMenu (n055) ---
//
// `LevelUpMenu` (Xbox PDB, 0xCC bytes, offsets as the PC code uses them):
// `eCurrentPage` +0x28, ten tiles `pTiles` +0x2C, `iCurrPoints[2]` +0x54,
// `iMaxPoints[2]` +0x5C, `lbSkillsList` (`ListBox<ActorValue::Index>`) +0x64,
// `lbPerksList` (`ListBox<BGSPerk *>`) +0x94 and `AvailablePerks`
// (`BSSimpleList<BGSPerk *>`) +0xC4. `pMe` is the open menu (`011d9fdc`).
// The user trait ids are the class's `iUT_*` statics, set by `00784a40`
// from their names (`_BaseValue` `011d9fe4`, `_ExtraValue` `011d9fec`,
// `_AddedValue` `011d9ff0`, `_CurrentPage` `011d9fe8`, `_CurrPoints`
// `011d9fd8`, `_MaxPoints` `011d9fd4`, `_Rank` `011d9ff8`, `_selected`
// `011d9fe0`, `_TextAlpha` `011d9ff4`).
//
// Notes that apply to these functions:
// - x87: each `FADD`/`FSUB`/`FDIV` goes through the prelude's helpers in the
//   code's operand order; floats pushed as arguments are their bit patterns;
//   a callee's `ST0` result is its `f64`; `_ftol2` (`00ec62c0`) gets `ST0`
//   as a leading `f64` argument.
// - The compiler's stack cookie checks (`00ec408c`) and exception frames
//   (`FS:[0]`) are not translated; the functions that have them carry a
//   `Translation incomplete` marker.

/// `LevelUpMenu::pMe` (Xbox PDB static): the open menu.
const LEVEL_UP_MENU_ME: u32 = 0x011d_9fdc;
/// `LevelUpMenu`'s vtable (stored by the destructor body).
const LEVEL_UP_MENU_VTABLE: u32 = 0x0107_3cdc;
/// The class id `GetClass` answers.
const LEVEL_UP_MENU_CLASS: u32 = 0x403;
/// `LevelUpMenu` fields (Xbox PDB names).
const CURRENT_PAGE: u32 = 0x28;
const MENU_TILES: u32 = 0x2c;
const CURR_POINTS: u32 = 0x54;
const MAX_POINTS: u32 = 0x5c;
const SKILLS_LIST: u32 = 0x64;
const PERKS_LIST: u32 = 0x94;
const AVAILABLE_PERKS: u32 = 0xc4;
/// The `iUT_*` user trait ids (see the section comment).
const UT_MAX_POINTS: u32 = 0x011d_9fd4;
const UT_CURR_POINTS: u32 = 0x011d_9fd8;
const UT_SELECTED: u32 = 0x011d_9fe0;
const UT_BASE_VALUE: u32 = 0x011d_9fe4;
const UT_CURRENT_PAGE: u32 = 0x011d_9fe8;
const UT_EXTRA_VALUE: u32 = 0x011d_9fec;
const UT_ADDED_VALUE: u32 = 0x011d_9ff0;
const UT_TEXT_ALPHA: u32 = 0x011d_9ff4;
const UT_RANK: u32 = 0x011d_9ff8;
/// `LevelUpMenu::PerkEffectList` (`BSSimpleArray<EffectItemData,1024>`, static).
const PERK_EFFECT_LIST: u32 = 0x011d_a050;
/// The player (`PlayerCharacter *`).
const LEVEL_UP_PLAYER: u32 = 0x011d_ea3c;

/// Callees outside this unit, by address (`thiscall` unless noted).
const MENU_TILE_00726070: u32 = 0x0072_6070;
const TILE_GET_STRING: u32 = 0x00a0_11f0;
const TILE_GET_CHILD_BY_ID: u32 = 0x00a0_3eb0;
const TILE_PLAY_TILE_SOUND: u32 = 0x00a0_b110;
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
const TILE_READ_FILE: u32 = 0x00a0_1b00;
const TILE_GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
const INTERFACE_TILE_00586150: u32 = 0x0058_6150;
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
const MENU_START_FADE_OUT: u32 = 0x00a1_d910;
const MENU_DESTROY: u32 = 0x00a1_c520;
const SKILL_LIST_DESTROY_00721150: u32 = 0x0072_1150;
const LIST_BOX_INITIALIZE: u32 = 0x0075_a0d0;
const LIST_BOX_FOR_EACH: u32 = 0x0073_14c0;
const LIST_BOX_SAVE_LIST_STATE: u32 = 0x0073_12e0;
const LIST_BOX_ITEM_DATA_007E1110: u32 = 0x007e_1110;
const SKILL_LIST_ADD_ITEM: u32 = 0x0075_4690;
const SKILL_LIST_NUM_UNFILTERED: u32 = 0x0072_6660;
const TILE_SET_LABEL_00707E10: u32 = 0x0070_7e10;
const TILE_PARENT_0045CD60: u32 = 0x0045_cd60;
const PLAY_MENU_SOUND: u32 = 0x0070_6f30;
const SHOW_TUTORIAL: u32 = 0x0071_8630;
const REPORT_005B5E40: u32 = 0x005b_5e40;
const SETTING_STRING: u32 = 0x0040_3df0;
const SETTING_INT: u32 = 0x0043_d4d0;
const PRINT_FORMATTED: u32 = 0x0040_6d00;
const STRING_COPY: u32 = 0x0040_6d30;
const STRING_LENGTH: u32 = 0x0044_a670;
const FULL_NAME_GET: u32 = 0x0040_8da0;
const PLAYER_LEVEL: u32 = 0x0087_f9f0;
const ACTOR_VALUE_OWNER_00648BC0: u32 = 0x0064_8bc0;
const HANDLE_ENTRY_POINT: u32 = 0x005e_58f0;
const PERK_LIST_FIRST_00460FB0: u32 = 0x0046_0fb0;
const PERK_LIST_HOLDER: u32 = 0x011c_3f2c;
const PERK_LEVEL_006AABF0: u32 = 0x006a_abf0;
const PERK_NUM_RANKS_005D4EE0: u32 = 0x005d_4ee0;
const PERK_FLAG_007A9280: u32 = 0x007a_9280;
const PERK_IS_ATTAINABLE: u32 = 0x005e_ba00;
const PERK_FLAG_005EB5E0: u32 = 0x005e_b5e0;
const PERK_DESCRIPTION: u32 = 0x005e_bac0;
const PERK_ENTRY_005EB900: u32 = 0x005e_b900;
const BYTE_FLAG_00408D60: u32 = 0x0040_8d60;
const BYTE_FLAG_HOLDER: u32 = 0x011d_a034;
const LIST_APPEND_00905820: u32 = 0x0090_5820;
const LIST_COUNT_005AE380: u32 = 0x005a_e380;
const MIN_004A8F20: u32 = 0x004a_8f20;
const ACTOR_VALUE_NAME: u32 = 0x0066_e950;
const ACTOR_VALUE_DESC: u32 = 0x0066_e9b0;
const ACTOR_VALUE_ICON: u32 = 0x0066_e9f0;
const ACTOR_VALUE_BADGE: u32 = 0x0066_ea20;
const ACTOR_VALUE_TO_INDEX: u32 = 0x0066_ec10;
const ACTOR_VALUE_SECTION_COUNT: u32 = 0x0066_ecf0;
const HUD_CLEAR_XP_METER: u32 = 0x0077_da30;
const PROGRESSION_00464E10: u32 = 0x0046_4e10;
const PROGRESSION_008D5270: u32 = 0x008d_5270;
const PROGRESSION_READY_TO_LEVEL_UP: u32 = 0x008d_51f0;
const ACTOR_UPDATE_MAGIC: u32 = 0x008c_3c40;
const STATS_UPDATE_PLAYER_NAME: u32 = 0x007d_f4e0;
const STATS_UPDATE_PLAYER_TITLE: u32 = 0x007d_f5d0;
const SOUND_CHECK_00525430: u32 = 0x0052_5430;
const SOUND_CHECK_HOLDER: u32 = 0x011f_2250;
const FLAG_004A4040: u32 = 0x004a_4040;
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const GET_SOUND_HANDLE_BY_NAME: u32 = 0x00ad_7550;
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
const SOUND_HANDLE_DESTROY: u32 = 0x0048_3710;
const ACTOR_GET_CLASS: u32 = 0x0088_4350;
const CLASS_TAGS_005A5F40: u32 = 0x005a_5f40;
const ARRAY_SIZE_0044DDC0: u32 = 0x0044_ddc0;
const ARRAY_ELEMENT_006B3160: u32 = 0x006b_3160;
const ARRAY_CLEAR_008454F0: u32 = 0x0084_54f0;
const EFFECT_ACTOR_VALUE_00403EA0: u32 = 0x0040_3ea0;
const EFFECT_GET_MAGNITUDE: u32 = 0x0040_4390;
const EFFECT_SETTING_00825C00: u32 = 0x0082_5c00;
const ARRAY_DESTROY_00788390: u32 = 0x0078_8390;
const POINT_PERCENT_004BD510: u32 = 0x004b_d510;
const COMPARE_STRINGS_00408B20: u32 = 0x0040_8b20;
/// The cdecl callbacks this unit hands to `ListBox::ForEach`.
const SET_DISPLAY_STRING_CALLBACK: u32 = 0x0078_6550;
const GIVE_PERK_CALLBACK: u32 = 0x0078_6570;
const TEXT_ALPHA_CALLBACK: u32 = 0x0078_64e0;
const RESET_SKILL_CALLBACK: u32 = 0x0078_6170;
const RESET_PERK_CALLBACK: u32 = 0x0078_6200;
const PERK_UPDATE_CALLBACK: u32 = 0x0078_6240;
const PERK_COMPARE: u32 = 0x0078_65d0;
/// The skill list's compare function (`007da270`, outside this unit).
const SKILL_COMPARE_007DA270: u32 = 0x007d_a270;

/// Data the code reads: names, formats and numbers.
const LEVEL_UP_MENU_FILE: u32 = 0x0107_3d9c;
const LEVEL_UP_CREATE_FAILED: u32 = 0x0107_3d48;
const TITLE_0_TEXT: u32 = 0x0107_3d3c;
const TITLE_1_TEXT: u32 = 0x0107_3d30;
const CURRENT_PAGE_TEXT: u32 = 0x0107_3cc8;
const END_PAGE_TEXT: u32 = 0x0107_3d24;
const SKILL_TEMPLATE_TEXT: u32 = 0x0107_3dcc;
const PERK_TEMPLATE_TEXT: u32 = 0x0107_3db8;
const SUCCESS_SOUND_TEXT: u32 = 0x0107_3de0;
const OVERFLOW_VALUE_TEXT: u32 = 0x0107_3e04;
const MAX_VALUE_TEXT: u32 = 0x0107_3df8;
const MIN_VALUE_TEXT: u32 = 0x0107_3dec;
const DISPLAY_STRING_TEXT: u32 = 0x0107_3e14;
const NAME_RANK_FORMAT: u32 = 0x0106_eb00;
const INTEGER_FORMAT_01020764: u32 = 0x0102_0764;
/// `double`s compared with trait `0xfa7` (6006.0 and 102.0).
const ROOT_VALUE_010704F0: u32 = 0x0107_04f0;
const ROOT_VALUE_010704E8: u32 = 0x0107_04e8;
/// `255.0` (`double`), the alpha of a perk that can be picked.
const FULL_ALPHA_DOUBLE: u32 = 0x0101_e568;
/// `255.0` and `128.0` (`float`s), the perk line alphas.
const FULL_ALPHA_FLOAT: u32 = 0x0102_3cd8;
const HALF_ALPHA_FLOAT: u32 = 0x0101_e704;
/// Game settings (string and int) the code reads.
const LEVEL_FORMAT_SETTING: u32 = 0x011d_4048;
const TITLE_SETTING_011D33D8: u32 = 0x011d_33d8;
const TITLE_SETTING_011D3564: u32 = 0x011d_3564;
const TITLE_SETTING_011D35C4: u32 = 0x011d_35c4;
const TITLE_SETTING_011D1FEC: u32 = 0x011d_1fec;
const PERK_EVERY_LEVELS_SETTING: u32 = 0x011c_d074;
const UNNAMED_PERK_SETTING: u32 = 0x011d_35b8;
const POINTS_FORMAT_SETTING_0: u32 = 0x011d_1d48;
const POINTS_FORMAT_SETTING_1: u32 = 0x011d_2a00;
const ONE_POINT_SETTING_0: u32 = 0x011d_2a24;
const ONE_POINT_SETTING_1: u32 = 0x011d_2e74;
const TAG_SKILL_SETTING: u32 = 0x011c_d6c0;

fn level_up_me(e: &Engine) -> u32 {
    e.mem.u32(LEVEL_UP_MENU_ME)
}

fn level_up_player(e: &Engine) -> u32 {
    e.mem.u32(LEVEL_UP_PLAYER)
}

fn menu_tile(e: &Engine, menu: u32, index: u32) -> u32 {
    e.mem.u32(menu + MENU_TILES + 4 * index)
}

/// `Menu`'s tile (`00726070`, the word at +4).
fn menu_root(e: &mut Engine, menu: u32) -> u32 {
    e.call(MENU_TILE_00726070, &args![menu]).u32()
}

fn page_slot(e: &Engine, this: u32, table: u32) -> u32 {
    let page = e.mem.u32(this + CURRENT_PAGE);
    this.wrapping_add(table).wrapping_add(page.wrapping_mul(4))
}

fn set_int(e: &mut Engine, tile: u32, trait_id: u32, value: u32) {
    e.call(TILE_SET_INT, &args![tile, trait_id, value]);
}

fn set_string(e: &mut Engine, tile: u32, trait_id: u32, text: u32) {
    e.call(TILE_SET_STRING, &args![tile, trait_id, text, 1u32]);
}

fn set_float(e: &mut Engine, tile: u32, trait_id: u32, bits: u32) {
    e.call(TILE_SET_FLOAT, &args![tile, trait_id, bits, 1u32]);
}

fn get_float(e: &mut Engine, tile: u32, trait_id: u32) -> f64 {
    e.call(TILE_GET_FLOAT, &args![tile, trait_id]).f64()
}

fn setting_string(e: &mut Engine, setting: u32) -> u32 {
    e.call(SETTING_STRING, &args![setting]).u32()
}

fn text_to_trait(e: &mut Engine, text: u32) -> u32 {
    e.call(TEXT_TO_TRAIT, &args![text]).u32()
}

fn ftol(e: &mut Engine, value: f64) -> i32 {
    e.call(FTOL2, &args![value]).i32()
}

/// `ListBox::ForEach(list, callback, 0, 0x7fffffff)` (`007314c0`).
fn for_each(e: &mut Engine, list: u32, callback: u32) {
    e.call(
        LIST_BOX_FOR_EACH,
        &args![list, callback, 0u32, 0x7fff_ffffu32],
    );
}

/// The player's actor value `index` (the object at the player's +0xa4, its
/// slot +0x1c).
fn player_actor_value(e: &mut Engine, index: u32) -> u32 {
    let player = level_up_player(e);
    e.vcall(player.wrapping_add(0xa4), 0x1c, &args![index])
        .u32()
}

/// The player's rank in `perk` (player slot +0x4a0 with `(perk, 0)`), its
/// `AL`.
fn perk_rank(e: &mut Engine, perk: u32) -> u32 {
    let player = level_up_player(e);
    e.vcall(player, 0x4a0, &args![perk, 0u32]).u32() & 0xff
}

/// The player's level (`0087f9f0`, a 16-bit word).
fn player_level(e: &mut Engine) -> i32 {
    let player = level_up_player(e);
    i32::from(e.call(PLAYER_LEVEL, &args![player]).u32() as u16)
}

/// A perk's level requirement (`006aabf0`, a signed byte).
fn perk_level(e: &mut Engine, perk: u32) -> i32 {
    i32::from(e.call(PERK_LEVEL_006AABF0, &args![perk]).u32() as u8 as i8)
}

// Translated from 00784b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LevelUpMenu::GetClass` (engine map): the class id `0x403`.
pub fn level_up_menu_get_class(_e: &mut Engine, _this: Ptr) -> u32 {
    LEVEL_UP_MENU_CLASS
}

// Translated from 00784b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LevelUpMenu::AttachTileByID` (engine map): keeps `tile` as `pTiles[id]`
/// for ids 0 to 9; other ids do nothing.
pub fn level_up_menu_attach_tile_by_id(e: &mut Engine, this: Ptr, id: i32, tile: u32) {
    if (0..10).contains(&id) {
        e.mem
            .set_u32(this.addr() + MENU_TILES + 4 * id as u32, tile);
    }
}

// Translated from 00784bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LevelUpMenu` scalar deleting destructor (engine map): the destructor body
/// ([`fn_00784bf0`]), then frees the object (`00401030`) when bit 0 of
/// `flags` is set. Answers `this`.
pub fn level_up_menu_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00784bf0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00784bf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00784bf0 - the C++ exception frame (FS:[0] chain, unwind states 2, 1 and 0 around the members' destructors) is not translated.
/// The `LevelUpMenu` destructor body (unnamed in the engine map): stores the
/// vtable `01073cdc`, clears `pMe`, then destroys `AvailablePerks`
/// (`0046ffb0`), `lbPerksList` ([`fn_00786c80`]), `lbSkillsList`
/// (`00721150`) and the `Menu` base (`00a1c520`).
pub fn fn_00784bf0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, LEVEL_UP_MENU_VTABLE);
    e.mem.set_u32(LEVEL_UP_MENU_ME, 0);
    e.call(LIST_DESTRUCT, &args![base + AVAILABLE_PERKS]);
    fn_00786c80(e, Ptr::new(base + PERKS_LIST));
    e.call(SKILL_LIST_DESTROY_00721150, &args![base + SKILLS_LIST]);
    e.call(MENU_DESTROY, &args![base]);
}

// Translated from 00784c80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00784c80 - the /GS stack cookie check (00ec408c) is not translated.
/// `LevelUpMenu::Create` (engine map; no words): deletes an open level-up
/// menu (`Tile::GetMenuByClass(0x403)`, vtable +0 with 1), reads
/// `levelup_menu.xml` into a tile and makes its menu `pMe`. When that menu
/// is a level-up menu (vtable +0x34 against [`fn_00785180`]) it takes the
/// tile (`Menu::SetMenuTile`) and, without all ten tiles
/// ([`fn_00785190`]), reports the failure and answers 0. Otherwise: trait
/// `0xfa7` of 6006.0 or 102.0 gives trait `0xfad` the maximum depth; the
/// tutorial (`0x10`, the class, `0x200`) is shown; the titles are set (the
/// level through a game setting format, `_Title_0`, `_Title_1`,
/// `_CurrentPage` 1.0 and the last title); the skill points to spend
/// (`00648bc0` on the player's actor values, through entry point 10,
/// truncated) go to `iMaxPoints[0]`; on a level that is a multiple of the
/// perk setting, every perk the player can still rank in, is not hidden
/// (`007a9280`), is attainable and playable (`005eb5e0`), and (when the
/// byte at `00408d60(011da034)` is set) passes its conditions at its level,
/// is added to `AvailablePerks`. Then `iCurrPoints[1]` is 0, `iMaxPoints[1]`
/// 1, `_EndPage` 0 without perks, the lists are built ([`level_up_menu_init_lists`]) and
/// page 0 shown ([`level_up_menu_set_current_page`]). Answers the tile; when the menu is not a
/// level-up menu it is deleted (when it has a tile) and the answer is 0.
pub fn level_up_menu_create(e: &mut Engine) -> u32 {
    let old = e
        .call(TILE_GET_MENU_BY_CLASS, &args![LEVEL_UP_MENU_CLASS])
        .u32();
    let interface = e.call(FOCUS_OWNER_004B7210, &args![]).u32();
    if old != 0 {
        e.vcall(old, 0, &args![1u32]);
    }
    let reader = e.call(INTERFACE_TILE_00586150, &args![interface]).u32();
    let tile = e
        .call(TILE_READ_FILE, &args![reader, LEVEL_UP_MENU_FILE])
        .u32();
    let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
    e.mem.set_u32(LEVEL_UP_MENU_ME, menu);
    if level_up_me(e) != 0 {
        let me = level_up_me(e);
        let class = e.vcall(me, 0x34, &args![]).u32();
        if class == fn_00785180(e) {
            let me = level_up_me(e);
            e.call(MENU_SET_MENU_TILE, &args![me, tile, 0u32]);
            let me = level_up_me(e);
            if !fn_00785190(e, Ptr::new(me)) {
                e.call(REPORT_005B5E40, &args![LEVEL_UP_CREATE_FAILED]);
                return 0;
            }
            level_up_menu_create_body(e, tile);
            return tile;
        }
    }
    if level_up_me(e) != 0 {
        let me = level_up_me(e);
        if menu_root(e, me) != 0 {
            let me = level_up_me(e);
            if me != 0 {
                e.vcall(me, 0, &args![1u32]);
            }
        }
    }
    0
}

/// The part of [`level_up_menu_create`] after the tiles are checked.
fn level_up_menu_create_body(e: &mut Engine, tile: u32) {
    let first: f64 = e.global(ROOT_VALUE_010704F0);
    let second: f64 = e.global(ROOT_VALUE_010704E8);
    let deep = get_float(e, tile, 0xfa7) == first || get_float(e, tile, 0xfa7) == second;
    if deep {
        let depth = e.call(TILE_GET_MAXIMUM_DEPTH, &args![]).f64();
        set_float(e, tile, 0xfad, x87_store32(depth));
    }
    let class = fn_00785180(e);
    let interface = e.call(FOCUS_OWNER_004B7210, &args![]).u32();
    e.call(
        SHOW_TUTORIAL,
        &args![interface.wrapping_add(0x4d4), 0x10u32, class, 0x200u32],
    );
    e.with_stack(0x40, |e, buffer| {
        let level = player_level(e) as u32;
        let format = setting_string(e, LEVEL_FORMAT_SETTING);
        e.call(PRINT_FORMATTED, &args![buffer, 0x40u32, format, level]);
        let me = level_up_me(e);
        let title = menu_tile(e, me, 0);
        set_string(e, title, 0xfc4, buffer.addr());
    });
    let text = setting_string(e, TITLE_SETTING_011D33D8);
    let me = level_up_me(e);
    let target = menu_tile(e, me, 6);
    set_string(e, target, 0xfc4, text);
    let text = setting_string(e, TITLE_SETTING_011D3564);
    let title = text_to_trait(e, TITLE_0_TEXT);
    let me = level_up_me(e);
    let target = menu_tile(e, me, 7);
    set_string(e, target, title, text);
    let text = setting_string(e, TITLE_SETTING_011D35C4);
    let title = text_to_trait(e, TITLE_1_TEXT);
    let me = level_up_me(e);
    let target = menu_tile(e, me, 7);
    set_string(e, target, title, text);
    let page = text_to_trait(e, CURRENT_PAGE_TEXT);
    e.call(TILE_POKE, &args![tile, page, 1.0f32.to_bits()]);
    let text = setting_string(e, TITLE_SETTING_011D1FEC);
    let me = level_up_me(e);
    let target = menu_tile(e, me, 8);
    set_string(e, target, 0xfc4, text);
    let player = level_up_player(e);
    let owner = if player == 0 {
        0
    } else {
        player.wrapping_add(0xa4)
    };
    let points = e.call(ACTOR_VALUE_OWNER_00648BC0, &args![owner]).i32();
    let points = e.with_stack(4, |e, value| {
        e.mem.set_u32(value.addr(), x87_store32(f64::from(points)));
        let player = level_up_player(e);
        e.call(HANDLE_ENTRY_POINT, &args![10u32, player, value]);
        let me = level_up_me(e);
        e.mem.set_u32(me + CURR_POINTS, 0);
        let loaded = x87_m32(e.mem.u32(value.addr()));
        ftol(e, loaded)
    });
    let me = level_up_me(e);
    e.mem.set_u32(me + MAX_POINTS, points as u32);
    let level = player_level(e);
    let every = e.call(SETTING_INT, &args![PERK_EVERY_LEVELS_SETTING]).u32();
    let every = e.mem.i32(every);
    if level % every == 0 {
        collect_available_perks(e);
    }
    let me = level_up_me(e);
    e.mem.set_u32(me + CURR_POINTS + 4, 0);
    let me = level_up_me(e);
    e.mem.set_u32(me + MAX_POINTS + 4, 1);
    let me = level_up_me(e);
    if e.call(LIST_COUNT_005AE380, &args![me + AVAILABLE_PERKS])
        .u32()
        == 0
    {
        let end = text_to_trait(e, END_PAGE_TEXT);
        let me = level_up_me(e);
        let root = menu_root(e, me);
        set_int(e, root, end, 0);
    }
    let me = level_up_me(e);
    level_up_menu_init_lists(e, Ptr::new(me));
    let me = level_up_me(e);
    level_up_menu_set_current_page(e, Ptr::new(me), 0);
}

/// [`level_up_menu_create`]'s walk of the game's perk list (`00460fb0` on
/// `011c3f2c`), appending each perk that qualifies to `AvailablePerks`
/// (`00905820` with the perk's address).
fn collect_available_perks(e: &mut Engine) {
    let holder = e.mem.u32(PERK_LIST_HOLDER);
    let mut node = e.call(PERK_LIST_FIRST_00460FB0, &args![holder]).u32();
    e.with_stack(4, |e, local| {
        while node != 0 && node_item(e, node) != 0 {
            let perk = node_item(e, node);
            e.mem.set_u32(local.addr(), perk);
            if perk_level(e, perk) > 0 && perk_qualifies(e, perk) {
                let me = level_up_me(e);
                e.call(LIST_APPEND_00905820, &args![me + AVAILABLE_PERKS, local]);
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    });
}

/// The rank, flag, attainable and condition tests of
/// [`collect_available_perks`], in the code's order.
fn perk_qualifies(e: &mut Engine, perk: u32) -> bool {
    let rank = perk_rank(e, perk);
    let ranks = e.call(PERK_NUM_RANKS_005D4EE0, &args![perk]).u32() & 0xff;
    if rank >= ranks {
        return false;
    }
    if e.call(PERK_FLAG_007A9280, &args![perk]).bool() {
        return false;
    }
    let player = level_up_player(e);
    if !e.call(PERK_IS_ATTAINABLE, &args![perk, player]).bool() {
        return false;
    }
    if !e.call(PERK_FLAG_005EB5E0, &args![perk]).bool() {
        return false;
    }
    let flag = e.call(BYTE_FLAG_00408D60, &args![BYTE_FLAG_HOLDER]).u32();
    if e.mem.u8(flag) == 0 {
        return true;
    }
    let player = level_up_player(e);
    if !fn_00785150(e, Ptr::new(perk), player) {
        return false;
    }
    let needed = perk_level(e, perk);
    needed <= player_level(e)
}

// Translated from 00785180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no words): the level-up menu's class id
/// `0x403`.
pub fn fn_00785180(_e: &mut Engine) -> u32 {
    LEVEL_UP_MENU_CLASS
}

// Translated from 00785190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: true when all ten tiles (`pTiles[0..10]`) are
/// set.
pub fn fn_00785190(e: &mut Engine, this: Ptr) -> bool {
    (0..10).all(|index| e.mem.u32(this.addr() + MENU_TILES + 4 * index) != 0)
}

// Translated from 007851d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LevelUpMenu::Close` (engine map; no words): clears the XP meter
/// (`HUDMainMenu::ClearXPMeterFinishedMoving`); with an open menu, finishes
/// the level-up (`008d5270` on the player's progression, `00464e10`), gives
/// each selected perk ([`fn_00786570`] over `lbPerksList`), updates the
/// player's magic (`Actor::UpdateMagic(1, 0)`) when still ready to level
/// up, updates the stats menu's name and title, sets trait `0x1772` of the
/// menu's tile to 1, fades the menu out and clears `pMe`.
pub fn level_up_menu_close(e: &mut Engine) {
    e.call(HUD_CLEAR_XP_METER, &args![]);
    if level_up_me(e) == 0 {
        return;
    }
    let player = level_up_player(e);
    let progression = e.call(PROGRESSION_00464E10, &args![player]).u32();
    e.call(PROGRESSION_008D5270, &args![progression]);
    let me = level_up_me(e);
    for_each(e, me + PERKS_LIST, GIVE_PERK_CALLBACK);
    let player = level_up_player(e);
    let progression = e.call(PROGRESSION_00464E10, &args![player]).u32();
    if e.call(PROGRESSION_READY_TO_LEVEL_UP, &args![progression])
        .bool()
    {
        let player = level_up_player(e);
        e.call(ACTOR_UPDATE_MAGIC, &args![player, 1u32, 0u32]);
    }
    e.call(STATS_UPDATE_PLAYER_NAME, &args![]);
    e.call(STATS_UPDATE_PLAYER_TITLE, &args![]);
    let me = level_up_me(e);
    let root = menu_root(e, me);
    set_int(e, root, 0x1772, 1);
    let me = level_up_me(e);
    e.call(MENU_START_FADE_OUT, &args![me]);
    e.mem.set_u32(LEVEL_UP_MENU_ME, 0);
}

// Translated from 00785280 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00785280 - the /GS stack cookie check (00ec408c) is not translated.
/// `LevelUpMenu::DoEnter` (engine map): the pointer entering an item.
/// A skill (id 11): its actor value's icon (trait `0xfcc` of tile 3) and
/// description (`0xfc4` of tile 4), both shown (`0xfa3`), and its badge on
/// tile 9 when it has one (else tile 9 hidden); then menu sound 4. A perk
/// (id 12): its name on tile 3, its description (`BGSPerk::
/// BuildDescriptionString` into a 0x200-byte buffer) on tile 4, tile 3 shown
/// when the name is longer than one character, tile 4 shown, tile 9 hidden,
/// menu sound 4. An arrow (13, 14): the skill list highlights the arrow's
/// parent (`0045cd60`, list vtable +0) and the menu enters it as a skill
/// (vtable +0x10 with 11).
pub fn level_up_menu_do_enter(e: &mut Engine, this: Ptr, id: u32, tile: u32) {
    let base = this.addr();
    match id {
        0xb => {
            let value = e
                .call(
                    LIST_BOX_ITEM_DATA_007E1110,
                    &args![base + SKILLS_LIST, tile],
                )
                .u32();
            let icon = e.call(ACTOR_VALUE_ICON, &args![value]).u32();
            let target = menu_tile(e, base, 3);
            set_string(e, target, 0xfcc, icon);
            let value = e
                .call(
                    LIST_BOX_ITEM_DATA_007E1110,
                    &args![base + SKILLS_LIST, tile],
                )
                .u32();
            let description = e.call(ACTOR_VALUE_DESC, &args![value]).u32();
            let target = menu_tile(e, base, 4);
            set_string(e, target, 0xfc4, description);
            let target = menu_tile(e, base, 3);
            set_int(e, target, 0xfa3, 1);
            let target = menu_tile(e, base, 4);
            set_int(e, target, 0xfa3, 1);
            let value = e
                .call(
                    LIST_BOX_ITEM_DATA_007E1110,
                    &args![base + SKILLS_LIST, tile],
                )
                .u32();
            if e.call(ACTOR_VALUE_BADGE, &args![value]).u32() != 0 {
                let target = menu_tile(e, base, 9);
                set_int(e, target, 0xfa3, 1);
                let value = e
                    .call(
                        LIST_BOX_ITEM_DATA_007E1110,
                        &args![base + SKILLS_LIST, tile],
                    )
                    .u32();
                let badge = e.call(ACTOR_VALUE_BADGE, &args![value]).u32();
                let target = menu_tile(e, base, 9);
                set_string(e, target, 0xfcc, badge);
            } else {
                let target = menu_tile(e, base, 9);
                set_int(e, target, 0xfa3, 0);
            }
            e.call(PLAY_MENU_SOUND, &args![4u32]);
        }
        0xc => {
            let perk = e
                .call(LIST_BOX_ITEM_DATA_007E1110, &args![base + PERKS_LIST, tile])
                .u32();
            let name = e.call(FULL_NAME_GET, &args![perk + 0x2c]).u32();
            let target = menu_tile(e, base, 3);
            set_string(e, target, 0xfcc, name);
            e.with_stack(0x200, |e, buffer| {
                let perk = e
                    .call(LIST_BOX_ITEM_DATA_007E1110, &args![base + PERKS_LIST, tile])
                    .u32();
                e.call(PERK_DESCRIPTION, &args![perk, buffer, 0x200u32]);
                let target = menu_tile(e, base, 4);
                set_string(e, target, 0xfc4, buffer.addr());
            });
            let target = menu_tile(e, base, 3);
            let text = e.call(TILE_GET_STRING, &args![target, 0xfccu32]).u32();
            let length = e.call(STRING_LENGTH, &args![text]).u32();
            let target = menu_tile(e, base, 3);
            set_int(e, target, 0xfa3, u32::from(length > 1));
            let target = menu_tile(e, base, 4);
            set_int(e, target, 0xfa3, 1);
            let target = menu_tile(e, base, 9);
            set_int(e, target, 0xfa3, 0);
            e.call(PLAY_MENU_SOUND, &args![4u32]);
        }
        0xd | 0xe => {
            let parent = e.call(TILE_PARENT_0045CD60, &args![tile]).u32();
            e.vcall(base + SKILLS_LIST, 0, &args![parent]);
            let parent = e.call(TILE_PARENT_0045CD60, &args![tile]).u32();
            e.vcall(base, 0x10, &args![0xbu32, parent]);
        }
        _ => {}
    }
}

// Translated from 00785540 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00785540 - the /GS stack cookie check (00ec408c) is not translated.
/// `LevelUpMenu::InitLists` (engine map). The skill list (tile 1 with
/// `LUM_SkillTemplate`): every actor value of section 2 that is not flagged
/// `0x2000` (`00406d70`) gets a line (`ListBox<ActorValue>::AddItem` with
/// its name), named (`00707e10`), id 11 (`0xfaa`), `_AddedValue` 0.0 and
/// `_BaseValue` the player's value; the lines' values are summed. The list
/// is sorted (`007da270`, vtable +0x20) and `iMaxPoints[0]` becomes
/// `max(min(iMaxPoints[0], 100 * count - sum), 0)`. The perk list (tile 2
/// with `LUM_PerkTemplate`): every perk of `AvailablePerks` gets a line with
/// its name (or the unnamed setting), `"%s (%d)"` with the next rank when it
/// has more than one rank and the player has one, id 12 and `_Rank` the next
/// rank.
pub fn level_up_menu_init_lists(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let mut sum = 0i32;
    let mut count = 0i32;
    let tile = menu_tile(e, base, 1);
    e.call(
        LIST_BOX_INITIALIZE,
        &args![base + SKILLS_LIST, tile, SKILL_TEMPLATE_TEXT],
    );
    let mut index = 0u32;
    loop {
        let total = e.call(ACTOR_VALUE_SECTION_COUNT, &args![2u32]).u32();
        if index >= total {
            break;
        }
        let value = e
            .call(ACTOR_VALUE_TO_INDEX, &args![2u32, index & 0xff])
            .u32();
        if !e.call(FLAG_TEST_00406D70, &args![value, 0x2000u32]).bool() {
            let name = e.call(ACTOR_VALUE_NAME, &args![value]).u32();
            let item = e
                .call(
                    SKILL_LIST_ADD_ITEM,
                    &args![base + SKILLS_LIST, value, name, 0u32, 0u32],
                )
                .u32();
            let name = e.call(ACTOR_VALUE_NAME, &args![value]).u32();
            e.call(TILE_SET_LABEL_00707E10, &args![item, name]);
            set_int(e, item, 0xfaa, 0xb);
            let added = e.mem.u32(UT_ADDED_VALUE);
            set_float(e, item, added, 0.0f32.to_bits());
            let current = player_actor_value(e, value);
            let base_trait = e.mem.u32(UT_BASE_VALUE);
            set_int(e, item, base_trait, current);
            let current = player_actor_value(e, value) as i32;
            sum = current.wrapping_add(sum);
            count = count.wrapping_add(1);
        }
        index = index.wrapping_add(1);
    }
    e.vcall(base + SKILLS_LIST, 0x20, &args![SKILL_COMPARE_007DA270]);
    let max_points = e.mem.u32(base + MAX_POINTS);
    let left = count.wrapping_mul(100).wrapping_sub(sum);
    let low = e.call(MIN_004A8F20, &args![max_points, left]).u32();
    let points = e.call(INT_PAIR_00647B70, &args![low, 0u32]).u32();
    e.mem.set_u32(base + MAX_POINTS, points);
    let tile = menu_tile(e, base, 2);
    e.call(
        LIST_BOX_INITIALIZE,
        &args![base + PERKS_LIST, tile, PERK_TEMPLATE_TEXT],
    );
    e.with_stack(0x40, |e, buffer| {
        let mut node = base + AVAILABLE_PERKS;
        while node != 0 && node_item(e, node) != 0 {
            let perk = node_item(e, node);
            let rank = perk_rank(e, perk) as i32;
            let perk = node_item(e, node);
            let name = e.call(FULL_NAME_GET, &args![perk + 0x18]).u32();
            e.call(STRING_COPY, &args![buffer, 0x40u32, name]);
            if e.call(STRING_LENGTH, &args![buffer]).u32() == 0 {
                let unnamed = setting_string(e, UNNAMED_PERK_SETTING);
                e.call(STRING_COPY, &args![buffer, 0x40u32, unnamed]);
            }
            let perk = node_item(e, node);
            let ranks = e.call(PERK_NUM_RANKS_005D4EE0, &args![perk]).u32() & 0xff;
            if ranks > 1 && rank > 0 {
                e.call(
                    PRINT_FORMATTED,
                    &args![
                        buffer,
                        0x40u32,
                        NAME_RANK_FORMAT,
                        buffer,
                        rank.wrapping_add(1) as u32
                    ],
                );
            }
            let perk = node_item(e, node);
            let item = list_box_bgs_perk_add_item(
                e,
                Ptr::new(base + PERKS_LIST),
                perk,
                buffer.addr(),
                0,
                0,
            );
            set_int(e, item, 0xfaa, 0xc);
            let rank_trait = e.mem.u32(UT_RANK);
            set_int(e, item, rank_trait, rank.wrapping_add(1) as u32);
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    });
}

// Translated from 00785830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LevelUpMenu::SetCurrentPage` (engine map). When the menu tile's
/// `_EndPage` is below `page`, the menu closes ([`level_up_menu_close`]).
/// Otherwise `eCurrentPage` becomes `page`, the tile's `_CurrentPage`,
/// `_CurrPoints` and `_MaxPoints` are set from it, the counter is updated
/// ([`level_up_menu_update_point_counter`]) and the page is prepared: page 0 refreshes each skill
/// line ([`fn_00786550`]), saves the perk list's state and restores the
/// skill list's ([`fn_00786a50`] with 1); page 1 sets each perk line's alpha
/// ([`fn_007864e0`]), sorts the perks ([`fn_007865d0`], vtable +0x20), saves
/// the skill list's state and restores the perk list's.
pub fn level_up_menu_set_current_page(e: &mut Engine, this: Ptr, page: i32) {
    let base = this.addr();
    let wanted = f64::from(page);
    let end = text_to_trait(e, END_PAGE_TEXT);
    let root = menu_root(e, base);
    let last = get_float(e, root, end);
    if last < wanted {
        level_up_menu_close(e);
        return;
    }
    e.mem.set_u32(base + CURRENT_PAGE, page as u32);
    let page_trait = e.mem.u32(UT_CURRENT_PAGE);
    let root = menu_root(e, base);
    set_int(e, root, page_trait, page as u32);
    let slot = base
        .wrapping_add(CURR_POINTS)
        .wrapping_add((page as u32).wrapping_mul(4));
    let points = e.mem.u32(slot);
    let points_trait = e.mem.u32(UT_CURR_POINTS);
    let root = menu_root(e, base);
    set_int(e, root, points_trait, points);
    let slot = base
        .wrapping_add(MAX_POINTS)
        .wrapping_add((page as u32).wrapping_mul(4));
    let points = e.mem.u32(slot);
    let max_trait = e.mem.u32(UT_MAX_POINTS);
    let root = menu_root(e, base);
    set_int(e, root, max_trait, points);
    level_up_menu_update_point_counter(e, this);
    match e.mem.u32(base + CURRENT_PAGE) {
        0 => {
            for_each(e, base + SKILLS_LIST, SET_DISPLAY_STRING_CALLBACK);
            e.call(LIST_BOX_SAVE_LIST_STATE, &args![base + PERKS_LIST]);
            fn_00786a50(e, Ptr::new(base + SKILLS_LIST), 1);
        }
        1 => {
            for_each(e, base + PERKS_LIST, TEXT_ALPHA_CALLBACK);
            e.vcall(base + PERKS_LIST, 0x20, &args![PERK_COMPARE]);
            e.call(LIST_BOX_SAVE_LIST_STATE, &args![base + SKILLS_LIST]);
            list_box_bgs_perk_restore_list_state(e, Ptr::new(base + PERKS_LIST), 1);
        }
        _ => {}
    }
}

// Translated from 00785990 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00785990 - the /GS stack cookie check (00ec408c) is not translated.
/// `LevelUpMenu::UpdatePointCounter` (engine map): the points left on the
/// page (`iMaxPoints - iCurrPoints`) on tile 5: none hides it (`0xfa3` 0);
/// one shows the page's one-point text (game settings `011d2a24` for page
/// 0, `011d2e74` otherwise); more shows the page's format (settings
/// `011d1d48`, `011d2a00`, kept in a stack array next to the count, which
/// the code indexes by page) printed with the count.
pub fn level_up_menu_update_point_counter(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(0x4c, |e, locals| {
        let formats = locals.addr();
        let buffer = formats + 0xc;
        let format = setting_string(e, POINTS_FORMAT_SETTING_0);
        e.mem.set_u32(formats, format);
        let format = setting_string(e, POINTS_FORMAT_SETTING_1);
        e.mem.set_u32(formats + 4, format);
        let max = e.mem.u32(page_slot(e, base, MAX_POINTS));
        let current = e.mem.u32(page_slot(e, base, CURR_POINTS));
        let left = max.wrapping_sub(current);
        e.mem.set_u32(formats + 8, left);
        let counter = menu_tile(e, base, 5);
        if left == 0 {
            set_int(e, counter, 0xfa3, 0);
        } else if left == 1 {
            let text = if e.mem.u32(base + CURRENT_PAGE) == 0 {
                setting_string(e, ONE_POINT_SETTING_0)
            } else {
                setting_string(e, ONE_POINT_SETTING_1)
            };
            let counter = menu_tile(e, base, 5);
            set_string(e, counter, 0xfc4, text);
            let counter = menu_tile(e, base, 5);
            set_int(e, counter, 0xfa3, 1);
        } else {
            let page = e.mem.u32(base + CURRENT_PAGE);
            let format = e.mem.u32(formats.wrapping_add(page.wrapping_mul(4)));
            e.call(PRINT_FORMATTED, &args![buffer, 0x40u32, format, left]);
            let counter = menu_tile(e, base, 5);
            set_string(e, counter, 0xfc4, buffer);
            let counter = menu_tile(e, base, 5);
            set_int(e, counter, 0xfa3, 1);
        }
    });
}

/// Writes the page's `iCurrPoints` to the menu tile's `_CurrPoints`.
fn show_current_points(e: &mut Engine, base: u32) {
    let points = e.mem.u32(page_slot(e, base, CURR_POINTS));
    let points_trait = e.mem.u32(UT_CURR_POINTS);
    let root = menu_root(e, base);
    set_int(e, root, points_trait, points);
}

// Translated from 00785aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LevelUpMenu::DoClick` (engine map). Reset (6): menu sound 1 when points
/// were spent, the page's points cleared and shown, each skill line reset
/// ([`fn_00786170`]) on page 0 or each perk line ([`fn_00786200`]) on page
/// 1, the counter updated. Next (7) and back (8): the page after or before
/// ([`level_up_menu_set_current_page`]) and trait `0xfc3` of tile 7 or 8 set to 0.0. A perk
/// (12): a selected one is unselected ([`fn_00787780`]); an unselected one
/// whose alpha is 255.0 replaces the selection ([`fn_00787850`] with 1,
/// [`fn_00787760`]); either way its effects are applied
/// ([`level_up_menu_apply_perk_effects`]), the page's points become the selected count and the
/// counter is updated. An arrow (13 less, 14 more) that is shown
/// (`0xfa3`): the page's points change by one, the skill's change by one
/// (times the tag skill setting for a tag skill, `005a5f40` on the player's
/// class) in the line's `_AddedValue` and the player's value (slots +0x48c
/// and +0x394), the line's text and the counter are updated.
pub fn level_up_menu_do_click(e: &mut Engine, this: Ptr, id: u32, tile: u32) {
    let base = this.addr();
    match id {
        6 => {
            if (e.mem.u32(page_slot(e, base, CURR_POINTS)) as i32) > 0 {
                e.call(PLAY_MENU_SOUND, &args![1u32]);
            }
            let slot = page_slot(e, base, CURR_POINTS);
            e.mem.set_u32(slot, 0);
            show_current_points(e, base);
            match e.mem.u32(base + CURRENT_PAGE) {
                0 => for_each(e, base + SKILLS_LIST, RESET_SKILL_CALLBACK),
                1 => for_each(e, base + PERKS_LIST, RESET_PERK_CALLBACK),
                _ => {}
            }
            level_up_menu_update_point_counter(e, this);
        }
        7 => {
            let page = e.mem.i32(base + CURRENT_PAGE).wrapping_add(1);
            level_up_menu_set_current_page(e, this, page);
            let target = menu_tile(e, base, 7);
            set_float(e, target, 0xfc3, 0.0f32.to_bits());
        }
        8 => {
            let page = e.mem.i32(base + CURRENT_PAGE).wrapping_sub(1);
            level_up_menu_set_current_page(e, this, page);
            let target = menu_tile(e, base, 8);
            set_float(e, target, 0xfc3, 0.0f32.to_bits());
        }
        0xc => {
            let perks = Ptr::new(base + PERKS_LIST);
            if fn_007877a0(e, perks, tile) {
                fn_00787780(e, perks, tile);
                level_up_menu_apply_perk_effects(e, this, tile);
            } else {
                let alpha_trait = e.mem.u32(UT_TEXT_ALPHA);
                let alpha = get_float(e, tile, alpha_trait);
                let full: f64 = e.global(FULL_ALPHA_DOUBLE);
                if alpha == full {
                    fn_00787850(e, perks, 1);
                    fn_00787760(e, perks, tile);
                    level_up_menu_apply_perk_effects(e, this, tile);
                }
            }
            let selected = fn_007878d0(e, perks);
            let slot = page_slot(e, base, CURR_POINTS);
            e.mem.set_u32(slot, selected);
            show_current_points(e, base);
            level_up_menu_update_point_counter(e, this);
        }
        0xd | 0xe => {
            if !e.call(TILE_IS_TRUE, &args![tile, 0xfa3u32]).bool() {
                return;
            }
            let parent = e.call(TILE_PARENT_0045CD60, &args![tile]).u32();
            let value = e
                .call(
                    LIST_BOX_ITEM_DATA_007E1110,
                    &args![base + SKILLS_LIST, parent],
                )
                .u32();
            let mut step: i32 = if id != 0xd { 1 } else { -1 };
            let slot = page_slot(e, base, CURR_POINTS);
            let points = e.mem.i32(slot).wrapping_add(step);
            e.mem.set_u32(slot, points as u32);
            show_current_points(e, base);
            let player = level_up_player(e);
            let class = e.call(ACTOR_GET_CLASS, &args![player]).u32();
            if e.call(CLASS_TAGS_005A5F40, &args![class, value]).bool() {
                let setting = e.call(SETTING_INT, &args![TAG_SKILL_SETTING]).u32();
                step = step.wrapping_mul(e.mem.i32(setting));
            }
            let added_trait = e.mem.u32(UT_ADDED_VALUE);
            let parent = e.call(TILE_PARENT_0045CD60, &args![tile]).u32();
            let added = get_float(e, parent, added_trait);
            let added = x87_store32(x87_add(added, f64::from(step)));
            let added_trait = e.mem.u32(UT_ADDED_VALUE);
            let parent = e.call(TILE_PARENT_0045CD60, &args![tile]).u32();
            set_float(e, parent, added_trait, added);
            let current = e.with_stack(4, |e, flag| {
                e.mem.set_u8(flag.addr(), 0);
                let player = level_up_player(e);
                e.vcall(player, 0x48c, &args![value, flag]).f64()
            });
            let current = x87_store32(x87_add(current, f64::from(step)));
            let player = level_up_player(e);
            e.vcall(player, 0x394, &args![value, current]);
            let parent = e.call(TILE_PARENT_0045CD60, &args![tile]).u32();
            level_up_menu_set_display_string(e, this, parent);
            level_up_menu_update_point_counter(e, this);
        }
        _ => {}
    }
}

// Translated from 00785e20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00785e20 - the C++ exception frame (FS:[0] chain, unwind state 0 around the sound handle) is not translated.
/// `LevelUpMenu::DoIdle` (engine map): when the fade state (`xFadeState`,
/// +0x24) is 4 and neither `00525430(011f2250)` nor `004a4040` holds, plays
/// `MUSSuccess` (flags `0x901`) and makes the menu visible
/// (`Menu::PrepForVisibility(0)`).
pub fn level_up_menu_do_idle(e: &mut Engine, this: Ptr) {
    if e.mem.u32(this.addr() + 0x24) != 4 {
        return;
    }
    if e.call(SOUND_CHECK_00525430, &args![SOUND_CHECK_HOLDER])
        .bool()
    {
        return;
    }
    if e.call(FLAG_004A4040, &args![]).bool() {
        return;
    }
    e.with_stack(0xc, |e, handle| {
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let found = e
            .call(
                GET_SOUND_HANDLE_BY_NAME,
                &args![audio, handle, SUCCESS_SOUND_TEXT, 0x901u32],
            )
            .u32();
        e.call(SOUND_HANDLE_PLAY, &args![found, 0u32]);
        e.call(SOUND_HANDLE_DESTROY, &args![handle]);
    });
    e.call(MENU_PREP_FOR_VISIBILITY, &args![this, 0u32]);
}

/// [`level_up_menu_do_gamepad`]'s arrow buttons on the skills page: with a
/// highlighted skill (list vtable +4), its arrow child `id` is clicked
/// (tile sound `0xfcb`, vtable +0xc) when shown. Answers whether it was
/// handled.
fn gamepad_arrow(e: &mut Engine, base: u32, id: u32) -> bool {
    if e.mem.u32(base + CURRENT_PAGE) != 0 {
        return false;
    }
    if e.vcall(base + SKILLS_LIST, 4, &args![]).u32() == 0 {
        return false;
    }
    let item = e.vcall(base + SKILLS_LIST, 4, &args![]).u32();
    let arrow = e.call(TILE_GET_CHILD_BY_ID, &args![item, id]).u32();
    if e.call(TILE_IS_TRUE, &args![arrow, 0xfa3u32]).bool() {
        e.call(TILE_PLAY_TILE_SOUND, &args![arrow, 0xfcbu32]);
        e.vcall(base, 0xc, &args![id, arrow]);
    }
    true
}

// Translated from 00785ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LevelUpMenu::DoGamepad` (engine map): button 3 clicks the highlighted
/// skill's "more" arrow (14) and 4 its "less" arrow (13) on the skills
/// page; 9 clicks the highlighted perk (id 12, after tile sound `0xfcb`) on
/// the perks page, or passes `(3, 1.0)` to vtable +0x38 on the skills page;
/// 10 clicks back (8, tile 8's sound) on the perks page, or passes
/// `(4, 1.0)` to vtable +0x38 on the skills page. Answers whether it was
/// handled.
pub fn level_up_menu_do_gamepad(e: &mut Engine, this: Ptr, button: u32) -> u8 {
    let base = this.addr();
    match button {
        3 => u8::from(gamepad_arrow(e, base, 0xe)),
        4 => u8::from(gamepad_arrow(e, base, 0xd)),
        9 => {
            if e.mem.u32(base + CURRENT_PAGE) == 1
                && e.vcall(base + PERKS_LIST, 4, &args![]).u32() != 0
            {
                let item = e.vcall(base + PERKS_LIST, 4, &args![]).u32();
                e.call(TILE_PLAY_TILE_SOUND, &args![item, 0xfcbu32]);
                let item = e.vcall(base + PERKS_LIST, 4, &args![]).u32();
                e.vcall(base, 0xc, &args![0xcu32, item]);
                1
            } else if e.mem.u32(base + CURRENT_PAGE) == 0 {
                e.vcall(base, 0x38, &args![3u32, 1.0f32.to_bits()]).u32() as u8
            } else {
                0
            }
        }
        10 => {
            if e.mem.u32(base + CURRENT_PAGE) == 1 {
                let back = menu_tile(e, base, 8);
                e.call(TILE_PLAY_TILE_SOUND, &args![back, 0xfcbu32]);
                e.vcall(base, 0xc, &args![8u32, 0u32]);
                1
            } else if e.mem.u32(base + CURRENT_PAGE) == 0 {
                e.vcall(base, 0x38, &args![4u32, 1.0f32.to_bits()]).u32() as u8
            } else {
                0
            }
        }
        _ => 0,
    }
}

// Translated from 00786110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LevelUpMenu::DoGamepadUpEvent` (engine map): button 11 with tile 7's
/// trait `0xfaf` true plays menu sound 1 and clicks next (vtable +0xc with
/// 7). Answers whether it was handled.
pub fn level_up_menu_do_gamepad_up_event(
    e: &mut Engine,
    this: Ptr,
    button: u32,
    _unused_1: u32,
) -> u8 {
    let base = this.addr();
    if button != 0xb {
        return 0;
    }
    let next = menu_tile(e, base, 7);
    if !e.call(TILE_IS_TRUE, &args![next, 0xfafu32]).bool() {
        return 0;
    }
    e.call(PLAY_MENU_SOUND, &args![1u32]);
    e.vcall(base, 0xc, &args![7u32, 0u32]);
    1
}

// Translated from 00786170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, a `ListBox::ForEach` callback with
/// the line's tile and actor value): takes the line's `_AddedValue` back
/// off the player's value (slots +0x48c and +0x394), clears it and
/// refreshes the line's text ([`level_up_menu_set_display_string`]).
pub fn fn_00786170(e: &mut Engine, tile: u32, value: u32) {
    let current = e.with_stack(4, |e, flag| {
        e.mem.set_u8(flag.addr(), 0);
        let player = level_up_player(e);
        e.vcall(player, 0x48c, &args![value, flag]).f64()
    });
    let added_trait = e.mem.u32(UT_ADDED_VALUE);
    let added = get_float(e, tile, added_trait);
    let lowered = x87_store32(x87_sub(current, added));
    let player = level_up_player(e);
    e.vcall(player, 0x394, &args![value, lowered]);
    let added_trait = e.mem.u32(UT_ADDED_VALUE);
    set_float(e, tile, added_trait, 0.0f32.to_bits());
    let me = level_up_me(e);
    level_up_menu_set_display_string(e, Ptr::new(me), tile);
}

// Translated from 00786200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LevelUpMenu::ResetPerksFunc` (engine map; `cdecl`, a `ListBox::ForEach`
/// callback with the line's tile and perk): a selected line is unselected
/// (`_selected` 0) and its effects taken back ([`level_up_menu_apply_perk_effects`]).
pub fn level_up_menu_reset_perks_func(e: &mut Engine, tile: u32, _unused_1: u32) {
    let selected = e.mem.u32(UT_SELECTED);
    if !e.call(TILE_IS_TRUE, &args![tile, selected]).bool() {
        return;
    }
    let selected = e.mem.u32(UT_SELECTED);
    set_int(e, tile, selected, 0);
    let me = level_up_me(e);
    level_up_menu_apply_perk_effects(e, Ptr::new(me), tile);
}

// Translated from 00786240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LevelUpMenu::PerkUpdateFunc` (engine map; `cdecl`, a `ListBox::ForEach`
/// callback with the skill line's tile and actor value): for each entry of
/// `PerkEffectList` for that actor value, the line's total (base + extra +
/// added, truncated) is taken; a pending `_OverflowValue` is added back to
/// the entry and cleared; an entry that would take the total above
/// `_MaxValue` or below `_MinValue` is cut to the limit, the cut kept as
/// `_OverflowValue`; the entry is added to the line's `_ExtraValue`.
pub fn level_up_menu_perk_update_func(e: &mut Engine, tile: u32, value: u32) {
    let mut index = 0u32;
    loop {
        let size = e.call(ARRAY_SIZE_0044DDC0, &args![PERK_EFFECT_LIST]).u32();
        if index >= size {
            break;
        }
        let entry = perk_effect(e, index);
        if e.mem.u32(entry + 0x10) == value {
            perk_update_entry(e, tile, index);
        }
        index = index.wrapping_add(1);
    }
}

/// `PerkEffectList`'s element `index` (`006b3160`).
fn perk_effect(e: &mut Engine, index: u32) -> u32 {
    e.call(ARRAY_ELEMENT_006B3160, &args![PERK_EFFECT_LIST, index])
        .u32()
}

/// One matching entry of [`level_up_menu_perk_update_func`].
fn perk_update_entry(e: &mut Engine, tile: u32, index: u32) {
    let base_trait = e.mem.u32(UT_BASE_VALUE);
    let base_value = get_float(e, tile, base_trait);
    let extra_trait = e.mem.u32(UT_EXTRA_VALUE);
    let extra = get_float(e, tile, extra_trait);
    let partial = x87_add(extra, base_value);
    let added_trait = e.mem.u32(UT_ADDED_VALUE);
    let added = get_float(e, tile, added_trait);
    let total = ftol(e, x87_add(added, partial));
    let overflow = text_to_trait(e, OVERFLOW_VALUE_TEXT);
    let pending = get_float(e, tile, overflow);
    let zero: f64 = e.global(ZERO_DOUBLE);
    if pending != zero {
        let entry = perk_effect(e, index);
        let pending = get_float(e, tile, overflow);
        let pending = ftol(e, pending);
        let magnitude = pending.wrapping_add(e.mem.i32(entry));
        e.mem.set_u32(entry, magnitude as u32);
        set_float(e, tile, overflow, 0.0f32.to_bits());
    }
    let entry = perk_effect(e, index);
    let sum = f64::from(total.wrapping_add(e.mem.i32(entry)));
    let max_trait = text_to_trait(e, MAX_VALUE_TEXT);
    let max = get_float(e, tile, max_trait);
    if max < sum {
        let entry = perk_effect(e, index);
        let sum = f64::from(total.wrapping_add(e.mem.i32(entry)));
        let max_trait = text_to_trait(e, MAX_VALUE_TEXT);
        let max = get_float(e, tile, max_trait);
        let cut = ftol(e, x87_sub(sum, max));
        let overflow = text_to_trait(e, OVERFLOW_VALUE_TEXT);
        set_int(e, tile, overflow, cut as u32);
        let entry = perk_effect(e, index);
        let magnitude = e.mem.i32(entry).wrapping_sub(cut);
        e.mem.set_u32(entry, magnitude as u32);
    } else {
        let entry = perk_effect(e, index);
        let sum = f64::from(total.wrapping_add(e.mem.i32(entry)));
        let min_trait = text_to_trait(e, MIN_VALUE_TEXT);
        let min = get_float(e, tile, min_trait);
        if above(min, sum) {
            let entry = perk_effect(e, index);
            let sum = f64::from(total.wrapping_add(e.mem.i32(entry)));
            let min_trait = text_to_trait(e, MIN_VALUE_TEXT);
            let min = get_float(e, tile, min_trait);
            let cut = ftol(e, x87_sub(sum, min));
            let overflow = text_to_trait(e, OVERFLOW_VALUE_TEXT);
            set_int(e, tile, overflow, cut as u32);
            let entry = perk_effect(e, index);
            let magnitude = e.mem.i32(entry).wrapping_sub(cut);
            e.mem.set_u32(entry, magnitude as u32);
        }
    }
    let extra_trait = e.mem.u32(UT_EXTRA_VALUE);
    let extra = get_float(e, tile, extra_trait);
    let entry = perk_effect(e, index);
    let magnitude = f64::from(e.mem.i32(entry));
    let extra = x87_store32(x87_add(magnitude, extra));
    let extra_trait = e.mem.u32(UT_EXTRA_VALUE);
    set_float(e, tile, extra_trait, extra);
}

// Translated from 007864e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, a `ListBox::ForEach` callback with
/// the line's tile and perk): the line's `_TextAlpha` is 255.0 when the
/// perk's conditions pass for the player ([`fn_00785150`]) and its level is
/// not above the player's, else 128.0.
pub fn fn_007864e0(e: &mut Engine, tile: u32, perk: u32) {
    let player = level_up_player(e);
    let full = fn_00785150(e, Ptr::new(perk), player) && {
        let needed = perk_level(e, perk);
        needed <= player_level(e)
    };
    let alpha = if full {
        e.mem.u32(FULL_ALPHA_FLOAT)
    } else {
        e.mem.u32(HALF_ALPHA_FLOAT)
    };
    let alpha_trait = e.mem.u32(UT_TEXT_ALPHA);
    set_float(e, tile, alpha_trait, x87_quiet_bits(alpha));
}

// Translated from 00786550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, a `ListBox::ForEach` callback with
/// the line's tile and item): refreshes the line's text
/// ([`level_up_menu_set_display_string`] on `pMe`).
pub fn fn_00786550(e: &mut Engine, tile: u32, _unused_1: u32) {
    let me = level_up_me(e);
    level_up_menu_set_display_string(e, Ptr::new(me), tile);
}

// Translated from 00786570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, a `ListBox::ForEach` callback with
/// the line's tile and perk): a selected line gives the player the perk's
/// next rank (player slot +0x498 with `(perk, rank + 1, 0)`, the rank from
/// slot +0x4a0).
pub fn fn_00786570(e: &mut Engine, tile: u32, perk: u32) {
    let selected = e.mem.u32(UT_SELECTED);
    if !e.call(TILE_IS_TRUE, &args![tile, selected]).bool() {
        return;
    }
    let rank = perk_rank(e, perk);
    let player = level_up_player(e);
    e.vcall(player, 0x498, &args![perk, rank.wrapping_add(1), 0u32]);
}

// Translated from 007865d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, the perk list's compare function on
/// two items, each a tile and a perk): the brighter line (`_TextAlpha`)
/// first, then the lower perk level, then the text (`0xfc4`, `00408b20`).
pub fn fn_007865d0(e: &mut Engine, first: u32, second: u32) -> i32 {
    let alpha_trait = e.mem.u32(UT_TEXT_ALPHA);
    let first_tile = e.mem.u32(first);
    let a = get_float(e, first_tile, alpha_trait);
    let alpha_trait = e.mem.u32(UT_TEXT_ALPHA);
    let second_tile = e.mem.u32(second);
    let b = get_float(e, second_tile, alpha_trait);
    if above(b, a) {
        return 1;
    }
    let alpha_trait = e.mem.u32(UT_TEXT_ALPHA);
    let first_tile = e.mem.u32(first);
    let a = get_float(e, first_tile, alpha_trait);
    let alpha_trait = e.mem.u32(UT_TEXT_ALPHA);
    let second_tile = e.mem.u32(second);
    let b = get_float(e, second_tile, alpha_trait);
    if b < a {
        return -1;
    }
    let first_perk = e.mem.u32(first + 4);
    let second_perk = e.mem.u32(second + 4);
    if perk_level(e, first_perk) < perk_level(e, second_perk) {
        return -1;
    }
    let first_perk = e.mem.u32(first + 4);
    let second_perk = e.mem.u32(second + 4);
    if perk_level(e, first_perk) > perk_level(e, second_perk) {
        return 1;
    }
    let second_tile = e.mem.u32(second);
    let second_text = e.call(TILE_GET_STRING, &args![second_tile, 0xfc4u32]).u32();
    let first_tile = e.mem.u32(first);
    let first_text = e.call(TILE_GET_STRING, &args![first_tile, 0xfc4u32]).u32();
    e.call(COMPARE_STRINGS_00408B20, &args![first_text, second_text])
        .i32()
}

// Translated from 007866e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007866e0 - the /GS stack cookie check (00ec408c) is not translated.
/// `LevelUpMenu::SetDisplayString` (engine map): the skill line's
/// `_DisplayString` is `"%d"` of base + extra + added (truncated), kept
/// within 0..100 (`00647b70`, `004a8f20`).
pub fn level_up_menu_set_display_string(e: &mut Engine, _this: Ptr, tile: u32) {
    let base_trait = e.mem.u32(UT_BASE_VALUE);
    let base_value = get_float(e, tile, base_trait);
    let extra_trait = e.mem.u32(UT_EXTRA_VALUE);
    let extra = get_float(e, tile, extra_trait);
    let partial = x87_add(extra, base_value);
    let added_trait = e.mem.u32(UT_ADDED_VALUE);
    let added = get_float(e, tile, added_trait);
    let total = ftol(e, x87_add(added, partial)) as u32;
    let low = e.call(INT_PAIR_00647B70, &args![total, 0u32]).u32();
    let shown = e.call(MIN_004A8F20, &args![low, 100u32]).u32();
    e.with_stack(0x10, |e, buffer| {
        e.call(
            PRINT_FORMATTED,
            &args![buffer, 0x10u32, INTEGER_FORMAT_01020764, shown],
        );
        let display = e
            .call(ADD_USER_TRAIT, &args![DISPLAY_STRING_TEXT, 0xffff_ffffu32])
            .u32();
        set_string(e, tile, display, buffer.addr());
    });
}

// Translated from 007867a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LevelUpMenu::ApplyPerkEffects` (engine map): rebuilds `PerkEffectList`
/// from the perk line's rank: the rank's entry (`005eb900`, the line's
/// `_Rank` truncated to a byte) when it is of type 1 (vtable +0x10): each
/// effect of its list (`0044ddc0`, nodes from +0x28) with an actor value
/// (`00403ea0`, not -1) gives an element (`EffectItemData`, 0x14 bytes,
/// constructed by `00403ba0`) of its magnitude less the previous rank's
/// effects on the same actor value, negated when the line is not selected
/// and again when `007869e0` says so for the effect's setting
/// (`00825c00`). Then each skill line is updated
/// ([`level_up_menu_perk_update_func`] over `lbSkillsList`).
pub fn level_up_menu_apply_perk_effects(e: &mut Engine, this: Ptr, tile: u32) {
    let base = this.addr();
    let perk = e
        .call(LIST_BOX_ITEM_DATA_007E1110, &args![base + PERKS_LIST, tile])
        .u32();
    let rank_trait = e.mem.u32(UT_RANK);
    let rank = get_float(e, tile, rank_trait);
    let rank = crate::x87::fistp_chop(rank) as u32 & 0xff;
    let entry = e.call(PERK_ENTRY_005EB900, &args![perk, rank]).u32();
    let previous = if rank > 1 {
        e.call(PERK_ENTRY_005EB900, &args![perk, rank - 1]).u32()
    } else {
        0
    };
    if entry != 0 && e.vcall(entry, 0x10, &args![]).u32() == 1 {
        let effects = e.call(ARRAY_SIZE_0044DDC0, &args![entry]).u32();
        let previous_effects = if previous != 0 {
            e.call(ARRAY_SIZE_0044DDC0, &args![previous]).u32()
        } else {
            0
        };
        e.call(ARRAY_CLEAR_008454F0, &args![PERK_EFFECT_LIST, 1u32]);
        let mut node = if effects != 0 { effects + 0x28 } else { 0 };
        e.with_stack(0x14, |e, data| {
            let data = data.addr();
            while node != 0 && node_item(e, node) != 0 {
                e.call(ELEMENT_CONSTRUCT, &args![data]);
                let effect = node_item(e, node);
                let value = e.call(EFFECT_ACTOR_VALUE_00403EA0, &args![effect]).u32();
                e.mem.set_u32(data + 0x10, value);
                let effect = node_item(e, node);
                let magnitude = e.call(EFFECT_GET_MAGNITUDE, &args![effect]).u32();
                e.mem.set_u32(data, magnitude);
                if e.mem.u32(data + 0x10) != 0xffff_ffff {
                    perk_effect_element(e, tile, node, data, previous_effects);
                }
                node = e.call(NODE_NEXT, &args![node]).u32();
            }
        });
    }
    for_each(e, base + SKILLS_LIST, PERK_UPDATE_CALLBACK);
}

/// [`level_up_menu_apply_perk_effects`]'s element for one effect with an actor value.
fn perk_effect_element(e: &mut Engine, tile: u32, node: u32, data: u32, previous_effects: u32) {
    let mut other = if previous_effects != 0 {
        previous_effects + 0x28
    } else {
        0
    };
    while other != 0 && node_item(e, other) != 0 {
        let effect = node_item(e, other);
        let value = e.call(EFFECT_ACTOR_VALUE_00403EA0, &args![effect]).u32();
        if value == e.mem.u32(data + 0x10) {
            let effect = node_item(e, other);
            let magnitude = e.call(EFFECT_GET_MAGNITUDE, &args![effect]).u32();
            let left = e.mem.u32(data).wrapping_sub(magnitude);
            e.mem.set_u32(data, left);
        }
        other = e.call(NODE_NEXT, &args![other]).u32();
    }
    let selected = e.mem.u32(UT_SELECTED);
    if !e.call(TILE_IS_TRUE, &args![tile, selected]).bool() {
        let negated = e.mem.i32(data).wrapping_mul(-1);
        e.mem.set_u32(data, negated as u32);
    }
    let value = e.mem.i32(data + 0x10);
    let effect = node_item(e, node);
    let setting = e.call(EFFECT_SETTING_00825C00, &args![effect]).u32();
    if fn_007869e0(e, Ptr::new(setting), value) != 0 {
        let negated = e.mem.i32(data).wrapping_mul(-1);
        e.mem.set_u32(data, negated as u32);
    }
    fn_00787950(e, Ptr::new(PERK_EFFECT_LIST), data);
}

// Translated from 00786a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ListBox<ActorValue::Index>`'s restore of its
/// saved state): with a container and a scroll bar, gives the scroll bar
/// `"_current_value"` (the saved scroll bar value, word 0) and pokes
/// `"_SetInCode"` with 1.0; takes the tile at index `_ftol(0040ebd0(
/// fSavedHighlightIndex, NumUnfiltered - 1.0))` (`00726660`, vtable +0x14
/// with 0), makes it the interface's focus target (`00715860` with trait
/// `0xfc3` and `focus`) and highlights it (vtable +0). Unless `004b71d0`
/// says so, also scrolls the container's menu to it (the menu's vtable
/// +0x14 with the tile's index trait `0xfaa` and the tile, when there is a
/// tile) and highlights nothing.
pub fn fn_00786a50(e: &mut Engine, this: Ptr<ListBox>, focus: u8) {
    if e.get(this, ListBox::pContainerRect).is_null() || e.get(this, ListBox::pScrollBar).is_null()
    {
        return;
    }
    let saved_value = e.mem.u32(this.addr() + 0x28);
    let current_value = e.call(TEXT_TO_TRAIT, &args![CURRENT_VALUE_TEXT]).u32();
    let scroll_bar = e.get(this, ListBox::pScrollBar);
    e.call(
        TILE_SET_FLOAT,
        &args![scroll_bar, current_value, x87_quiet_bits(saved_value), 0u32],
    );
    let set_in_code = e.call(TEXT_TO_TRAIT, &args![SET_IN_CODE_TEXT]).u32();
    let scroll_bar = e.get(this, ListBox::pScrollBar);
    e.call(TILE_POKE, &args![scroll_bar, set_in_code, 1.0f32.to_bits()]);
    let count = e.call(SKILL_LIST_NUM_UNFILTERED, &args![this]).u32();
    let one: f64 = e.global(ONE_DOUBLE);
    let last = x87_store32(x87_sub(f64::from(count), one));
    let saved_index = e.mem.u32(this.addr() + 0x24);
    let index = e
        .call(
            FLOAT_PAIR_0040EBD0,
            &args![x87_quiet_bits(saved_index), x87_quiet_bits(last)],
        )
        .f64();
    let index = x87_fistp64_chop(X87Ext::from(index)) as u32;
    let tile = e.vcall(this.addr(), 0x14, &args![index, 0u32]).u32();
    let owner = e.call(FOCUS_OWNER_004B7210, &args![]).u32();
    e.call(
        SET_CURRENT_FOCUS_TARGET,
        &args![owner, tile, 0xfc3u32, u32::from(focus)],
    );
    e.vcall(this.addr(), 0, &args![tile]);
    if e.call(FLAG_004B71D0, &args![]).bool() {
        return;
    }
    if tile != 0 {
        let container = e.get(this, ListBox::pContainerRect);
        let menu_tile = e.call(TILE_GET_MENU_TILE, &args![container]).u32();
        let menu = e.call(TILE_GET_MENU, &args![menu_tile]).u32();
        let value = e.call(TILE_GET_FLOAT, &args![tile, 0xfaau32]).f64();
        let item_index = e.call(FTOL2, &args![value]).u32();
        e.vcall(menu, 0x14, &args![item_index, tile]);
    }
    e.vcall(this.addr(), 0, &args![0u32]);
}

/// [`list_box_bgs_perk_get_highlight_target`]'s step for a scroll: the
/// container's height (`0xfb0`, negated for 15) over its `"_scroll_delta"`,
/// through `004bd510` with 1.0, truncated; 0 without a container.
fn scroll_step(e: &mut Engine, this: Ptr<ListBox>, negate: bool) -> i32 {
    let container = e.get(this, ListBox::pContainerRect);
    if container.is_null() {
        return 0;
    }
    let height = get_float(e, container.addr(), 0xfb0);
    let height = if negate { -height } else { height };
    let delta_trait = text_to_trait(e, SCROLL_DELTA_TEXT);
    let container = e.get(this, ListBox::pContainerRect);
    let delta = get_float(e, container.addr(), delta_trait);
    let quotient = x87_store32(x87_div(height, delta));
    let steps = e
        .call(POINT_PERCENT_004BD510, &args![quotient, 1.0f32.to_bits()])
        .f64();
    ftol(e, steps)
}

/// Whether `tile` is enabled (its `"_enabled"` trait, `Tile::AddUserTrait`
/// each time).
fn tile_enabled(e: &mut Engine, tile: u32) -> bool {
    let enabled = e
        .call(ADD_USER_TRAIT, &args![ENABLED_TEXT, 0xffff_ffffu32])
        .u32();
    e.call(TILE_IS_TRUE, &args![tile, enabled]).bool()
}

// Translated from 00787260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSPerk_P>::GetHighlightTarget` (engine map): the tile a move
/// highlights. The step is -1 for 1, +1 for 2, a page of scroll for 15
/// (up) and 16 (down), 0 otherwise. With an enabled list, a step and
/// items, the index starts from the highlighted tile's (`0xfac`, plus the
/// step, kept within the list) or the first (step down) or last item; the
/// item at that index is searched; a disabled one moves on in the step's
/// direction: past the end, backwards from the item before the last; past
/// the start, forwards from item 1 (vtable +0x14 for each). Answers the
/// tile found, or 0 when it is the highlighted one.
pub fn list_box_bgs_perk_get_highlight_target(
    e: &mut Engine,
    this: Ptr<ListBox>,
    direction: u32,
) -> u32 {
    let mut found = 0u32;
    let step = match direction {
        1 => -1,
        2 => 1,
        15 => scroll_step(e, this, true),
        16 => scroll_step(e, this, false),
        _ => 0,
    };
    if list_box_bgs_perk_is_enabled(e, this) && step != 0 && e.get(this, ListBox::usNextIndex) != 0
    {
        found = highlight_search(e, this, step);
    }
    let highlighted = e.get(this, ListBox::pHighlightedItem).addr();
    if found != highlighted {
        found
    } else {
        0
    }
}

/// The search loop of [`list_box_bgs_perk_get_highlight_target`].
fn highlight_search(e: &mut Engine, this: Ptr<ListBox>, step: i32) -> u32 {
    let last = i32::from(e.get(this, ListBox::usNextIndex)) - 1;
    let highlighted = e.get(this, ListBox::pHighlightedItem);
    let mut index = if !highlighted.is_null() {
        let current = get_float(e, highlighted.addr(), 0xfac);
        let current = ftol(e, current);
        let index = current.wrapping_add(step);
        if index < 0 {
            0
        } else if index > last {
            last
        } else {
            index
        }
    } else if step > 0 {
        0
    } else {
        last
    };
    let mut found = 0u32;
    loop {
        let mut node = first_node(this);
        while found == 0 && node != 0 && node_item(e, node) != 0 {
            let item = node_item(e, node);
            let tile = e.mem.u32(item);
            let value = get_float(e, tile, 0xfac);
            if f64::from(index) == value {
                let item = node_item(e, node);
                found = e.mem.u32(item);
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
        if found == 0 || tile_enabled(e, found) {
            return found;
        }
        found = 0;
        if step > 0 {
            index = index.wrapping_add(1);
            if index > last {
                index = last - 1;
                loop {
                    if found != 0 && tile_enabled(e, found) {
                        return found;
                    }
                    if index < 0 {
                        return found;
                    }
                    found = e.vcall(this.addr(), 0x14, &args![index as u32, 0u32]).u32();
                    index = index.wrapping_sub(1);
                }
            }
        }
        if step < 0 {
            index = index.wrapping_sub(1);
            if index < 0 {
                index = 1;
                loop {
                    if found != 0 && tile_enabled(e, found) {
                        return found;
                    }
                    if index > last {
                        return found;
                    }
                    found = e.vcall(this.addr(), 0x14, &args![index as u32, 0u32]).u32();
                    index = index.wrapping_add(1);
                }
            }
        }
    }
}

// Translated from 00787850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: unselects ([`fn_00787c20`]) each item's tile,
/// only the unfiltered ones (byte +8 clear) when `unfiltered` is set.
pub fn fn_00787850(e: &mut Engine, this: Ptr<ListBox>, unfiltered: u8) {
    let mut node = first_node(this);
    while node != 0 && node_item(e, node) != 0 {
        let skip = unfiltered != 0 && {
            let item = node_item(e, node);
            e.mem.u8(item + 8) != 0
        };
        if !skip {
            let item = node_item(e, node);
            let tile = e.mem.u32(item);
            fn_00787c20(e, this, tile);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 007879c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSPerk_P>` scalar deleting destructor (engine map): the
/// destructor body ([`fn_00786c80`]), then frees the object when bit 0 of
/// `flags` is set. Answers `this`.
pub fn list_box_bgs_perk_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ListBox>,
    flags: u32,
) -> Ptr<ListBox> {
    fn_00786c80(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00787c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: [`fn_00787c40`] with mode 1 (the code leaves
/// its `AL` in `EAX`).
pub fn fn_00787c20(e: &mut Engine, this: Ptr<ListBox>, tile: u32) -> bool {
    fn_00787c40(e, this, tile, 1)
}

// Translated from 00788360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<EffectItemData,1024>` scalar deleting destructor (engine
/// map): the destructor body (`00788390`), then frees the array when bit 0
/// of `flags` is set. Answers `this`.
pub fn fn_00788360(e: &mut Engine, this: Ptr<BSSimpleArray>, flags: u32) -> Ptr<BSSimpleArray> {
    e.call(ARRAY_DESTROY_00788390, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0078_8300, fn_00788300(Ptr) -> Ptr),
        entry!(0x0078_5150, fn_00785150(Ptr, u32) -> bool),
        entry!(0x0078_69e0, fn_007869e0(Ptr, i32) -> u8),
        entry!(0x0078_6bb0, fn_00786bb0(Ptr<ListBox>) -> Ptr<ListBox>),
        entry!(0x0078_6c80, fn_00786c80(Ptr<ListBox>)),
        entry!(
            0x0078_6cf0,
            list_box_bgs_perk_add_item(Ptr<ListBox>, u32, u32, u32, u32) -> u32
        ),
        entry!(0x0078_6f90, list_box_bgs_perk_remove_all(Ptr<ListBox>)),
        entry!(0x0078_71c0, list_box_bgs_perk_sort(Ptr<ListBox>, u32)),
        entry!(
            0x0078_71f0,
            list_box_bgs_perk_highlight(Ptr<ListBox>, u32) -> bool
        ),
        entry!(
            0x0078_7600,
            list_box_bgs_perk_restore_list_state(Ptr<ListBox>, u8)
        ),
        entry!(0x0078_7760, fn_00787760(Ptr<ListBox>, u32)),
        entry!(0x0078_7780, fn_00787780(Ptr<ListBox>, u32)),
        entry!(0x0078_77a0, fn_007877a0(Ptr<ListBox>, u32) -> bool),
        entry!(0x0078_78d0, fn_007878d0(Ptr<ListBox>) -> u32),
        entry!(0x0078_7950, fn_00787950(Ptr<BSSimpleArray>, u32) -> u32),
        entry!(
            0x0078_79f0,
            list_box_bgs_perk_num_unfiltered(Ptr<ListBox>) -> i32
        ),
        entry!(0x0078_7aa0, list_box_bgs_perk_synchronize(Ptr<ListBox>)),
        entry!(
            0x0078_7b70,
            list_box_bgs_perk_is_enabled(Ptr<ListBox>) -> bool
        ),
        entry!(0x0078_7c40, fn_00787c40(Ptr<ListBox>, u32, u32) -> bool),
        entry!(
            0x0078_7d80,
            list_box_bgs_perk_update_highlight_box_pos(Ptr<ListBox>)
        ),
        entry!(
            0x0078_80a0,
            list_box_bgs_perk_set_total_list_height(Ptr<ListBox>)
        ),
        entry!(0x0078_81d0, fn_007881d0(Ptr<BSSimpleArray>) -> u32),
        entry!(0x0078_8260, fn_00788260(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x0078_8330, fn_00788330(Ptr<BSSimpleArray>, u32) -> u32),
        entry!(0x0078_83b0, fn_007883b0(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x0078_4b80, level_up_menu_get_class(Ptr) -> u32),
        entry!(0x0078_4b90, level_up_menu_attach_tile_by_id(Ptr, i32, u32)),
        entry!(0x0078_4bc0, level_up_menu_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0078_4bf0, fn_00784bf0(Ptr)),
        entry!(0x0078_4c80, level_up_menu_create() -> u32),
        entry!(0x0078_5180, fn_00785180() -> u32),
        entry!(0x0078_5190, fn_00785190(Ptr) -> bool),
        entry!(0x0078_51d0, level_up_menu_close()),
        entry!(0x0078_5280, level_up_menu_do_enter(Ptr, u32, u32)),
        entry!(0x0078_5540, level_up_menu_init_lists(Ptr)),
        entry!(0x0078_5830, level_up_menu_set_current_page(Ptr, i32)),
        entry!(0x0078_5990, level_up_menu_update_point_counter(Ptr)),
        entry!(0x0078_5aa0, level_up_menu_do_click(Ptr, u32, u32)),
        entry!(0x0078_5e20, level_up_menu_do_idle(Ptr)),
        entry!(0x0078_5ed0, level_up_menu_do_gamepad(Ptr, u32) -> u8),
        entry!(0x0078_6110, level_up_menu_do_gamepad_up_event(Ptr, u32, u32) -> u8),
        entry!(0x0078_6170, fn_00786170(u32, u32)),
        entry!(0x0078_6200, level_up_menu_reset_perks_func(u32, u32)),
        entry!(0x0078_6240, level_up_menu_perk_update_func(u32, u32)),
        entry!(0x0078_64e0, fn_007864e0(u32, u32)),
        entry!(0x0078_6550, fn_00786550(u32, u32)),
        entry!(0x0078_6570, fn_00786570(u32, u32)),
        entry!(0x0078_65d0, fn_007865d0(u32, u32) -> i32),
        entry!(0x0078_66e0, level_up_menu_set_display_string(Ptr, u32)),
        entry!(0x0078_67a0, level_up_menu_apply_perk_effects(Ptr, u32)),
        entry!(0x0078_6a50, fn_00786a50(Ptr<ListBox>, u8)),
        entry!(
            0x0078_7260,
            list_box_bgs_perk_get_highlight_target(Ptr<ListBox>, u32) -> u32
        ),
        entry!(0x0078_7850, fn_00787850(Ptr<ListBox>, u8)),
        entry!(
            0x0078_79c0,
            list_box_bgs_perk_scalar_deleting_destructor(Ptr<ListBox>, u32) -> Ptr<ListBox>
        ),
        entry!(0x0078_7c20, fn_00787c20(Ptr<ListBox>, u32) -> bool),
        entry!(
            0x0078_8360,
            fn_00788360(Ptr<BSSimpleArray>, u32) -> Ptr<BSSimpleArray>
        ),
    ]
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn stores_the_vtable_before_calling_the_body() {
        let mut e = Engine::new();
        e.register(CONSTRUCTOR_BODY_007883B0, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 8, seen);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_8300, &args![this]).u32(), this);
        assert_eq!(
            calls_to(&mut e, CONSTRUCTOR_BODY_007883B0),
            vec![vec![this, 0, 0]]
        );
        assert_eq!(e.mem.u32(this + 8), VTABLE_00788300);
    }
}

#[cfg(test)]
mod perk_list_box_tests {
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

    fn stub(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// An engine with the statics, the flag byte and the two doubles mapped,
    /// and the list node helpers doubled (`006815c0` returns the node,
    /// `00726070` its next word).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011d_a000, 0x1000);
        e.map(0x011d_8000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.register(NODE_SELF, |_, a| eax(a[0]));
        e.register(NODE_NEXT, |e, a| eax(e.mem.u32(a[0] + 4)));
        e.register(ADD_USER_TRAIT, |_, _| eax(0x66));
        e
    }

    fn list_box(e: &mut Engine) -> Ptr<ListBox> {
        Ptr::new(e.mem.alloc(0x30))
    }

    /// An item (tile, data, filtered byte) in a new node, or in the embedded
    /// first node when `node` is the list box's +4.
    fn item(e: &mut Engine, tile: u32, data: u32, filtered: u8) -> u32 {
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item, tile);
        e.mem.set_u32(item + 4, data);
        e.mem.set_u8(item + 8, filtered);
        item
    }

    #[test]
    fn fn_00785150_tests_the_condition_at_0x40() {
        let mut e = engine();
        e.register(CONDITION_IS_TRUE, |_, _| eax(1));
        let this = e.mem.alloc(0x50);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0078_5150, &args![this, 0x1234u32]).bool());
        let calls = calls_to(&e, CONDITION_IS_TRUE);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][..3], [this + 0x40, 0x1234, 0]);
        assert_ne!(calls[0][3], 0);
        assert_eq!(calls[0][4], 0);
    }

    #[test]
    fn fn_007869e0_is_true_when_exactly_one_flag_is_set() {
        for (first, value, second, expected) in [
            (1u32, -1i32, 1u32, 1u8),
            (0, -1, 1, 0),
            (1, 5, 1, 0),
            (1, 5, 0, 1),
            (0, 5, 1, 1),
            (0, 5, 0, 0),
        ] {
            let mut e = engine();
            e.register_double(FLAG_TEST_00403E60, move |_, a| {
                assert_eq!(a[1], 4);
                eax(first)
            });
            e.register_double(FLAG_TEST_00406D70, move |_, a| {
                assert_eq!(a[1], 0x200);
                eax(second)
            });
            let this = e.mem.alloc(8);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0078_69e0, &args![this, value]).u8(), expected);
            assert_eq!(
                calls_to(&e, FLAG_TEST_00406D70).len(),
                usize::from(value != -1)
            );
        }
    }

    #[test]
    fn fn_00786bb0_builds_the_perk_list_box() {
        let mut e = engine();
        stub(
            &mut e,
            &[
                LIST_BOX_BASE_CONSTRUCT,
                LIST_CONSTRUCT,
                LIST_BOX_INIT_FLAGS,
                LIST_BOX_REGISTER,
            ],
        );
        let this = list_box(&mut e);
        for offset in (0x0c..0x30).step_by(4) {
            e.mem.set_u32(this.addr() + offset, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_6bb0, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), PERK_LIST_BOX_VTABLE);
        for offset in [0x0c, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28] {
            assert_eq!(e.mem.u32(this.addr() + offset), 0, "offset {offset:#x}");
        }
        assert_eq!(e.mem.u16(this.addr() + 0x1c), 0);
        assert_eq!(e.mem.u16(this.addr() + 0x2c), 0);
        assert_eq!(calls_to(&e, LIST_CONSTRUCT), vec![vec![this.addr() + 4]]);
        assert_eq!(
            calls_to(&e, LIST_BOX_INIT_FLAGS),
            vec![vec![this.addr(), 0, 1]]
        );
        assert_eq!(calls_to(&e, LIST_BOX_REGISTER), vec![vec![this.addr()]]);
    }

    #[test]
    fn fn_00786c80_empties_and_destroys_the_list() {
        let mut e = engine();
        stub(
            &mut e,
            &[
                LIST_BOX_UNREGISTER,
                LIST_DESTRUCT,
                TILE_SET_INT,
                TILE_SET_FLOAT,
            ],
        );
        let this = list_box(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0078_6c80, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), PERK_LIST_BOX_VTABLE);
        assert_eq!(calls_to(&e, LIST_BOX_UNREGISTER), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, LIST_DESTRUCT), vec![vec![this.addr() + 4]]);
        // Without a container RemoveAll stops before the traits.
        assert!(calls_to(&e, TILE_SET_FLOAT).is_empty());
    }

    fn add_item_engine() -> Engine {
        let mut e = engine();
        e.register(TILE_GET_MENU, |_, _| eax(0x5000));
        e.register(MENU_RENDER_TEMPLATE, |_, _| eax(0x6000));
        e.register(TILE_GET_VALUE, |_, _| eax(0));
        e.register(ALLOCATE, |e, a| eax(e.mem.alloc(a[0])));
        e.register(ITEM_CONSTRUCT, |e, a| {
            let data = e.mem.u32(a[2]);
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, data);
            eax(a[0])
        });
        e.register(LIST_BOX_HAS_FLAG, |_, _| eax(1));
        e.register(LIST_BOX_COUNT_00726C40, |_, _| eax(1));
        e.register(ADD_USER_TRAIT, |_, _| eax(0x77));
        e.register(TILE_GET_FLOAT, |_, _| st0(3.0));
        e.register(TILE_GET_VALUE_Q, |_, _| eax(0x9000));
        stub(
            &mut e,
            &[
                TILE_SET_INT,
                TILE_SET_STRING,
                LIST_INSERT_SORTED,
                LIST_APPEND,
                INIT_TILE_POSITION,
                VALUE_CLEAR_ACTIONS,
                VALUE_ADD_ACTION_TILE,
                VALUE_ADD_ACTION_FLOAT,
                VALUE_CALCULATE,
                TILE_SET_FLOAT,
                TEXT_TO_TRAIT,
            ],
        );
        e
    }

    #[test]
    fn add_item_needs_a_container_and_a_template() {
        let mut e = add_item_engine();
        let this = list_box(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0078_6cf0, &args![this, 1u32, 0u32, 0u32, 0x4000u32])
                .u32(),
            0
        );
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        assert_eq!(
            e.call(0x0078_6cf0, &args![this, 1u32, 0u32, 0u32, 0u32])
                .u32(),
            0
        );
        assert!(calls_to(&e, MENU_RENDER_TEMPLATE).is_empty());
    }

    #[test]
    fn add_item_appends_and_sizes_the_container() {
        let mut e = add_item_engine();
        let this = list_box(&mut e);
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        e.set(this, ListBox::pTemplateName, Ptr::new(0x4100));
        e.set(this, ListBox::usNextIndex, 5);
        e.call_log = Some(vec![]);
        let tile = e
            .call(0x0078_6cf0, &args![this, 0x42u32, 0x4200u32, 0u32, 0u32])
            .u32();
        assert_eq!(tile, 0x6000);
        assert_eq!(
            calls_to(&e, MENU_RENDER_TEMPLATE),
            vec![vec![0x5000, 0x3000, 0x4100, 0]]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![vec![0x6000, 0xfc4, 0x4200, 1]]
        );
        let appended = calls_to(&e, LIST_APPEND);
        assert_eq!(appended.len(), 1);
        let item = e.mem.u32(appended[0][1]);
        assert_eq!((e.mem.u32(item), e.mem.u32(item + 4)), (0x6000, 0x42));
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![vec![0x6000, 0xfaa, 0xffff_ffff], vec![0x6000, 0xfac, 5]]
        );
        assert_eq!(e.get(this, ListBox::usNextIndex), 6);
        assert_eq!(
            calls_to(&e, INIT_TILE_POSITION),
            vec![vec![this.addr(), 0x6000]]
        );
        assert_eq!(
            calls_to(&e, VALUE_ADD_ACTION_TILE),
            vec![vec![0x9000, 0x7d0, 0x6000, 0xfb0]]
        );
        assert_eq!(
            calls_to(&e, VALUE_ADD_ACTION_FLOAT),
            vec![vec![0x9000, 0x7d3, 3.0f32.to_bits()]]
        );
        assert!(calls_to(&e, LIST_INSERT_SORTED).is_empty());
    }

    #[test]
    fn add_item_inserts_sorted_with_a_compare_function() {
        let mut e = add_item_engine();
        e.register(LIST_BOX_COUNT_00726C40, |_, _| eax(2));
        let this = list_box(&mut e);
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        e.call_log = Some(vec![]);
        let tile = e
            .call(
                0x0078_6cf0,
                &args![this, 0x42u32, 0u32, 0x00ab_cdefu32, 0x4300u32],
            )
            .u32();
        assert_eq!(tile, 0x6000);
        assert_eq!(
            calls_to(&e, MENU_RENDER_TEMPLATE)[0],
            vec![0x5000, 0x3000, 0x4300, 0]
        );
        let inserted = calls_to(&e, LIST_INSERT_SORTED);
        assert_eq!(inserted.len(), 1);
        assert_eq!(inserted[0][0], this.addr() + 4);
        assert_eq!(e.mem.u32(inserted[0][1]), 0x6000);
        assert_eq!(inserted[0][2], 0x00ab_cdef);
        assert!(calls_to(&e, TILE_SET_STRING).is_empty());
        assert!(calls_to(&e, VALUE_ADD_ACTION_TILE).is_empty());
    }

    #[test]
    fn remove_all_deletes_every_item_and_resets() {
        let mut e = engine();
        stub(
            &mut e,
            &[
                FREE,
                NODE_SET_NEXT,
                NODE_DELETE,
                ITEM_DATA_DELETE,
                TILE_SET_FLOAT,
                TILE_SET_INT,
                TILE_GET_VALUE,
                VALUE_ADD_ACTION_TILE,
                VALUE_CALCULATE,
            ],
        );
        e.register(LIST_BOX_HAS_FLAG, |_, a| eax(u32::from(a[1] == 1)));
        e.register(TEXT_TO_TRAIT, |_, _| eax(0x55));
        e.register(ADD_USER_TRAIT, |_, _| eax(0x66));
        e.register(TILE_GET_FLOAT, |_, _| st0(0.0));
        e.register(0x0000_7000, |_, _| Ret::default());
        e.put_vtable(0x0f00_0000, &[0x0000_7000]);
        let this = list_box(&mut e);
        let tile = e.mem.alloc(4);
        e.mem.set_u32(tile, 0x0f00_0000);
        let first = item(&mut e, tile, 0x8000, 0);
        let second = item(&mut e, 0, 0, 0);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, second);
        e.mem.set_u32(this.addr() + 4, first);
        e.mem.set_u32(this.addr() + 8, node);
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        e.set(this, ListBox::pHighlightedItem, Ptr::new(0x3100));
        e.set(this, ListBox::usNextIndex, 4);
        e.set(this, ListBox::fTotalListHeight, 9.0);
        e.call_log = Some(vec![]);
        e.call(0x0078_6f90, &args![this]);
        assert_eq!(calls_to(&e, FREE), vec![vec![second], vec![first]]);
        assert_eq!(calls_to(&e, NODE_SET_NEXT), vec![vec![node, 0]]);
        assert_eq!(calls_to(&e, NODE_DELETE), vec![vec![node, 1]]);
        assert_eq!(calls_to(&e, ITEM_DATA_DELETE), vec![vec![0x8000, 1]]);
        assert_eq!(calls_to(&e, 0x0000_7000), vec![vec![tile, 1]]);
        assert_eq!(e.mem.u32(this.addr() + 4), 0);
        assert_eq!(e.mem.u32(this.addr() + 8), 0);
        assert_eq!(e.get(this, ListBox::usNextIndex), 0);
        assert!(e.get(this, ListBox::pHighlightedItem).is_null());
        assert_eq!(calls_to(&e, TILE_SET_FLOAT), vec![vec![0x3000, 0x55, 0, 1]]);
        // The highlight box gets the "nothing highlighted" values.
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![vec![0x3000, 0x66, 0xffff_ffff], vec![0x3000, 0x66, 0]]
        );
    }

    #[test]
    fn sort_sorts_the_list_then_synchronizes() {
        let mut e = engine();
        stub(&mut e, &[LIST_SORT]);
        let this = list_box(&mut e);
        e.set(this, ListBox::usNextIndex, 3);
        e.call_log = Some(vec![]);
        e.call(0x0078_71c0, &args![this, 0x1234u32]);
        assert_eq!(calls_to(&e, LIST_SORT), vec![vec![this.addr() + 4, 0x1234]]);
        assert_eq!(e.get(this, ListBox::usNextIndex), 0);
    }

    #[test]
    fn highlight_needs_an_enabled_list_and_an_enabled_item() {
        let mut e = engine();
        e.register(ADD_USER_TRAIT, |_, _| eax(0x66));
        e.register_double(TILE_IS_TRUE, |_, a| eax(u32::from(a[0] != 0x6100)));
        e.register(LIST_BOX_BELONGS_TO, |_, a| eax(u32::from(a[1] != 0x6200)));
        stub(
            &mut e,
            &[
                TILE_GET_VALUE,
                VALUE_ADD_ACTION_TILE,
                VALUE_CALCULATE,
                TILE_SET_INT,
            ],
        );
        e.register(TILE_FLAG_GETTER_0071AA80, |_, _| eax(0));
        e.register(TILE_FIND_BY_NAME, |_, _| eax(0));
        let this = list_box(&mut e);
        assert!(!e.call(0x0078_71f0, &args![this, 0x6000u32]).bool());
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        assert!(!e.call(0x0078_71f0, &args![this, 0x6100u32]).bool());
        assert!(!e.call(0x0078_71f0, &args![this, 0x6200u32]).bool());
        assert!(e.get(this, ListBox::pHighlightedItem).is_null());
        assert!(e.call(0x0078_71f0, &args![this, 0x6000u32]).bool());
        assert_eq!(e.get(this, ListBox::pHighlightedItem).addr(), 0x6000);
        assert!(e.call(0x0078_71f0, &args![this, 0u32]).bool());
        assert!(e.get(this, ListBox::pHighlightedItem).is_null());
    }

    #[test]
    fn restore_list_state_focuses_and_scrolls_to_the_saved_item() {
        let mut e = engine();
        e.register(TEXT_TO_TRAIT, |_, a| eax(a[0] & 0xfff));
        e.register(LIST_BOX_COUNT_00726C40, |_, _| eax(10));
        e.register(ADD_USER_TRAIT, |_, _| eax(0x66));
        e.register(TILE_GET_FLOAT, |_, a| {
            st0(if a[1] == 0xfaa { 7.0 } else { 0.0 })
        });
        e.register(FLOAT_PAIR_0040EBD0, |_, a| {
            st0(f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))))
        });
        e.register(FOCUS_OWNER_004B7210, |_, _| eax(0x7700));
        e.register(FLAG_004B71D0, |_, _| eax(0));
        e.register(TILE_GET_MENU_TILE, |_, _| eax(0x5100));
        e.register(TILE_GET_MENU, |_, _| eax(0x5200));
        e.register(FTOL2, |_, a| {
            eax(f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32 as u32)
        });
        stub(
            &mut e,
            &[TILE_SET_FLOAT, TILE_POKE, SET_CURRENT_FOCUS_TARGET],
        );
        e.register(0x0000_7014, |_, _| eax(0x6000));
        e.register(0x0000_7000, |_, _| Ret::default());
        e.register(0x0000_7114, |_, _| Ret::default());
        let mut slots = vec![0u32; 6];
        slots[0] = 0x0000_7000;
        slots[5] = 0x0000_7014;
        e.put_vtable(0x0f00_0000, &slots);
        let mut menu_slots = vec![0u32; 6];
        menu_slots[5] = 0x0000_7114;
        e.put_vtable(0x0f00_1000, &menu_slots);
        e.mem.map(0x5200, 4);
        e.mem.set_u32(0x5200, 0x0f00_1000);
        let this = list_box(&mut e);
        e.mem.set_u32(this.addr(), 0x0f00_0000);
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        e.set(this, ListBox::pScrollBar, Ptr::new(0x3400));
        e.set(this, ListBox::fSavedScrollbarValue, 12.5);
        e.set(this, ListBox::fSavedHighlightIndex, 4.75);
        e.call_log = Some(vec![]);
        e.call(0x0078_7600, &args![this, 1u32]);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x3400, 0x2d8, 12.5f32.to_bits(), 0]]
        );
        assert_eq!(
            calls_to(&e, TILE_POKE),
            vec![vec![0x3400, 0x250, 1.0f32.to_bits()]]
        );
        // 10 items, none filtered: the last index is 9.0.
        assert_eq!(
            calls_to(&e, FLOAT_PAIR_0040EBD0),
            vec![vec![4.75f32.to_bits(), 9.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, 0x0000_7014), vec![vec![this.addr(), 4, 0]]);
        assert_eq!(
            calls_to(&e, SET_CURRENT_FOCUS_TARGET),
            vec![vec![0x7700, 0x6000, 0xfc3, 1]]
        );
        assert_eq!(calls_to(&e, 0x0000_7114), vec![vec![0x5200, 7, 0x6000]]);
        assert_eq!(
            calls_to(&e, 0x0000_7000),
            vec![vec![this.addr(), 0x6000], vec![this.addr(), 0]]
        );

        // When 004b71d0 answers true it stops after the highlight.
        e.register(FLAG_004B71D0, |_, _| eax(1));
        e.call_log = Some(vec![]);
        e.call(0x0078_7600, &args![this, 0u32]);
        assert!(calls_to(&e, 0x0000_7114).is_empty());
        assert_eq!(calls_to(&e, 0x0000_7000), vec![vec![this.addr(), 0x6000]]);

        // Without a scroll bar nothing happens.
        e.set(this, ListBox::pScrollBar, Ptr::NULL);
        e.call_log = Some(vec![]);
        e.call(0x0078_7600, &args![this, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    fn select_engine() -> Engine {
        let mut e = engine();
        e.register(ADD_USER_TRAIT, |_, a| {
            eax(if a[0] == SELECTED_TEXT { 0x88 } else { 0x66 })
        });
        e.register(LIST_BOX_BELONGS_TO, |_, _| eax(1));
        e.register(TILE_IS_TRUE, |e, a| {
            if a[1] == 0x88 {
                eax(e.mem.u32(0x011d_af00))
            } else {
                eax(1)
            }
        });
        stub(&mut e, &[TILE_SET_INT]);
        e
    }

    #[test]
    fn fn_00787760_and_00787780_select_and_toggle() {
        let mut e = select_engine();
        let this = list_box(&mut e);
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        e.call_log = Some(vec![]);
        e.call(0x0078_7760, &args![this, 0x6000u32]);
        e.set_global(0x011d_af00, 1u32);
        e.call(0x0078_7780, &args![this, 0x6000u32]);
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![vec![0x6000, 0x88, 1], vec![0x6000, 0x88, 0]]
        );
    }

    #[test]
    fn fn_00787c40_changes_the_selection_by_mode() {
        let mut e = select_engine();
        let this = list_box(&mut e);
        // Disabled (no container): nothing.
        assert!(!e.call(0x0078_7c40, &args![this, 0x6000u32, 2u32]).bool());
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        assert!(!e.call(0x0078_7c40, &args![this, 0u32, 2u32]).bool());
        e.call_log = Some(vec![]);
        // Not selected: mode 0 selects, mode 1 does nothing, mode 3 neither.
        assert!(e.call(0x0078_7c40, &args![this, 0x6000u32, 0u32]).bool());
        assert!(!e.call(0x0078_7c40, &args![this, 0x6000u32, 1u32]).bool());
        assert!(!e.call(0x0078_7c40, &args![this, 0x6000u32, 3u32]).bool());
        e.set_global(0x011d_af00, 1u32);
        assert!(!e.call(0x0078_7c40, &args![this, 0x6000u32, 0u32]).bool());
        assert!(e.call(0x0078_7c40, &args![this, 0x6000u32, 1u32]).bool());
        assert!(e.call(0x0078_7c40, &args![this, 0x6000u32, 2u32]).bool());
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![
                vec![0x6000, 0x88, 1],
                vec![0x6000, 0x88, 0],
                vec![0x6000, 0x88, 0]
            ]
        );
        // Not one of the list's items.
        e.register(LIST_BOX_BELONGS_TO, |_, _| eax(0));
        assert!(!e.call(0x0078_7c40, &args![this, 0x6000u32, 2u32]).bool());
    }

    #[test]
    fn fn_007877a0_and_007878d0_count_selected_tiles() {
        let mut e = engine();
        e.register(ADD_USER_TRAIT, |_, _| eax(0x88));
        e.register(TILE_IS_TRUE, |_, a| eax(u32::from(a[0] >= 0x6100)));
        let this = list_box(&mut e);
        assert!(!e.call(0x0078_77a0, &args![this, 0u32]).bool());
        assert!(!e.call(0x0078_77a0, &args![this, 0x6000u32]).bool());
        assert!(e.call(0x0078_77a0, &args![this, 0x6100u32]).bool());
        assert_eq!(e.mem.u32(0x011d_a090), 0x88);
        assert_eq!(e.mem.u32(0x011d_a094) & 1, 1);
        let first = item(&mut e, 0x6100, 0, 0);
        let second = item(&mut e, 0x6000, 0, 0);
        let third = item(&mut e, 0x6200, 0, 0);
        let node2 = e.mem.alloc(8);
        let node3 = e.mem.alloc(8);
        e.mem.set_u32(this.addr() + 4, first);
        e.mem.set_u32(this.addr() + 8, node2);
        e.mem.set_u32(node2, second);
        e.mem.set_u32(node2 + 4, node3);
        e.mem.set_u32(node3, third);
        assert_eq!(e.call(0x0078_78d0, &args![this]).u32(), 2);
        // A zero trait id answers false without asking the tile.
        let mut e = engine();
        e.register(ADD_USER_TRAIT, |_, _| eax(0));
        e.call_log = Some(vec![]);
        let this = list_box(&mut e);
        assert!(!e.call(0x0078_77a0, &args![this, 0x6100u32]).bool());
        assert!(calls_to(&e, TILE_IS_TRUE).is_empty());
    }

    #[test]
    fn fn_00787950_appends_one_element() {
        let mut e = engine();
        e.register(ARRAY_IS_FULL, |_, _| eax(0));
        e.register(PLACEMENT_NEW, |_, a| eax(a[1]));
        stub(&mut e, &[ELEMENT_CONSTRUCT]);
        let this: Ptr<BSSimpleArray> = Ptr::new(e.mem.alloc(0x10));
        let buffer = e.mem.alloc(0x14 * 4);
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iSize, 1);
        e.set(this, BSSimpleArray::iReservedSize, 4);
        let element = e.mem.alloc(0x14);
        for word in 0..5 {
            e.mem.set_u32(element + 4 * word, 0x100 + word);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_7950, &args![this, element]).u32(), 1);
        assert_eq!(calls_to(&e, ELEMENT_CONSTRUCT), vec![vec![buffer + 0x14]]);
        for word in 0..5 {
            assert_eq!(e.mem.u32(buffer + 0x14 + 4 * word), 0x100 + word);
        }
        assert_eq!(e.get(this, BSSimpleArray::iSize), 2);
    }

    #[test]
    fn num_unfiltered_subtracts_the_filtered_count() {
        let mut e = engine();
        e.register(ADD_USER_TRAIT, |_, _| eax(0x99));
        e.register(LIST_BOX_COUNT_00726C40, |_, _| eax(7));
        e.register(TILE_GET_FLOAT, |_, a| {
            assert_eq!(a[1], 0x99);
            st0(2.9)
        });
        let this = list_box(&mut e);
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        assert_eq!(e.call(0x0078_79f0, &args![this]).i32(), 5);
        e.register(TILE_GET_FLOAT, |_, _| st0(9.5));
        assert_eq!(e.call(0x0078_79f0, &args![this]).i32(), -2);
    }

    #[test]
    fn synchronize_numbers_the_unfiltered_items() {
        let mut e = engine();
        stub(&mut e, &[TILE_SET_INT, INIT_TILE_POSITION]);
        let this = list_box(&mut e);
        let first = item(&mut e, 0x6100, 0, 0);
        let second = item(&mut e, 0x6200, 0, 1);
        let third = item(&mut e, 0x6300, 0, 0);
        let node2 = e.mem.alloc(8);
        let node3 = e.mem.alloc(8);
        e.mem.set_u32(this.addr() + 4, first);
        e.mem.set_u32(this.addr() + 8, node2);
        e.mem.set_u32(node2, second);
        e.mem.set_u32(node2 + 4, node3);
        e.mem.set_u32(node3, third);
        e.set(this, ListBox::usNextIndex, 9);
        e.set(this, ListBox::fTotalListHeight, 3.0);
        e.call_log = Some(vec![]);
        e.call(0x0078_7aa0, &args![this]);
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![vec![0x6100, 0xfac, 0], vec![0x6300, 0xfac, 1]]
        );
        assert_eq!(
            calls_to(&e, INIT_TILE_POSITION),
            vec![vec![this.addr(), 0x6100], vec![this.addr(), 0x6300]]
        );
        assert_eq!(e.get(this, ListBox::usNextIndex), 2);
        assert_eq!(e.get(this, ListBox::fTotalListHeight), 0.0);
    }

    #[test]
    fn is_enabled_asks_the_container_once_the_static_is_set() {
        let mut e = engine();
        e.register(ADD_USER_TRAIT, |_, _| eax(0x44));
        e.register(TILE_IS_TRUE, |_, a| eax(u32::from(a[1] == 0x44)));
        let this = list_box(&mut e);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0078_7b70, &args![this]).bool());
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        assert!(e.call(0x0078_7b70, &args![this]).bool());
        assert_eq!(
            calls_to(&e, ADD_USER_TRAIT),
            vec![vec![ENABLED_TEXT, 0xffff_ffff]]
        );
        assert_eq!(e.mem.u32(0x011d_a0a0), 0x44);
    }

    #[test]
    fn update_highlight_box_pos_follows_the_highlighted_tile() {
        let mut e = engine();
        e.register(ADD_USER_TRAIT, |_, a| {
            eax(if a[0] == HIGHLIGHT_Y_TEXT { 0x31 } else { 0x32 })
        });
        e.register(TILE_GET_VALUE, |_, a| eax(0x9000 + a[1]));
        e.register(TILE_FLAG_GETTER_0071AA80, |_, _| eax(1));
        e.register(TILE_FIND_BY_NAME, |_, _| eax(0x7100));
        e.register(TILE_GET_CHILD_BY_NAME, |_, a| {
            eax(if a[1] == TOP_TEXT { 0x7200 } else { 0x7300 })
        });
        stub(
            &mut e,
            &[
                VALUE_ADD_ACTION_TILE,
                VALUE_CALCULATE,
                TILE_FLAG_SETTER_0071AA60,
                TILE_ADD_NEEDS_UPDATE,
                TILE_SET_INT,
            ],
        );
        let this = list_box(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0078_7d80, &args![this]);
        assert_eq!(e.mem.u32(0x011d_a0b8) & 3, 3);
        assert!(calls_to(&e, TILE_SET_INT).is_empty());
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        e.call(0x0078_7d80, &args![this]);
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![vec![0x3000, 0x31, 0xffff_ffff], vec![0x3000, 0x32, 0]]
        );
        e.set(this, ListBox::pHighlightedItem, Ptr::new(0x6000));
        e.call_log = Some(vec![]);
        e.call(0x0078_7d80, &args![this]);
        assert_eq!(
            calls_to(&e, VALUE_ADD_ACTION_TILE),
            vec![
                vec![0x9031, 0x7d0, 0x6000, 0xfa2],
                vec![0x9032, 0x7d0, 0x6000, 0xfb0]
            ]
        );
        assert_eq!(
            calls_to(&e, TILE_FLAG_SETTER_0071AA60),
            vec![
                vec![0x7200, 1],
                vec![0x7300, 1],
                vec![0x7300, 1],
                vec![0x7300, 1],
                vec![0x7100, 1]
            ]
        );
        assert_eq!(calls_to(&e, TILE_ADD_NEEDS_UPDATE).len(), 5);
        // Without "top" only the box itself, with the byte 0.
        e.register(TILE_FLAG_GETTER_0071AA80, |_, _| eax(0));
        e.register(TILE_GET_CHILD_BY_NAME, |_, _| eax(0));
        e.call_log = Some(vec![]);
        e.call(0x0078_7d80, &args![this]);
        assert_eq!(
            calls_to(&e, TILE_FLAG_SETTER_0071AA60),
            vec![vec![0x7100, 0]]
        );
    }

    #[test]
    fn set_total_list_height_counts_scroll_steps() {
        let mut e = engine();
        e.register(TILE_GET_FLOAT, |_, a| {
            st0(if a[1] == 0xfb0 { 25.0 } else { 0.0 })
        });
        e.register(ADD_USER_TRAIT, |_, _| eax(0x99));
        e.register(LIST_BOX_COUNT_00726C40, |_, _| eax(8));
        e.register(ROUND_FLOAT, |_, a| {
            st0(f64::from(f32::from_bits(a[0])).ceil())
        });
        e.register(FTOL2, |_, a| {
            eax(f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32 as u32)
        });
        e.register(INT_PAIR_00647B70, |_, a| {
            eax((a[0] as i32).max(a[1] as i32) as u32)
        });
        e.register(TEXT_TO_TRAIT, |_, a| eax(a[0] & 0xfff));
        stub(&mut e, &[TILE_SET_FLOAT, TILE_SET_INT]);
        let this = list_box(&mut e);
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        e.set(this, ListBox::pScrollBar, Ptr::new(0x3400));
        e.set(this, ListBox::fTotalListHeight, 100.0);
        e.call_log = Some(vec![]);
        e.call(0x0078_80a0, &args![this]);
        // 8 * 75 / 100 = 6, + 1 = 7 steps; delta 75 / 6 = 12.5.
        assert_eq!(calls_to(&e, ROUND_FLOAT), vec![vec![6.0f32.to_bits()]]);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x3000, 0x90c, 12.5f32.to_bits(), 1]]
        );
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x3400, 0x8f8, 7]]);
        // Shorter than the visible height: one step, no delta.
        e.set(this, ListBox::fTotalListHeight, 20.0);
        e.call_log = Some(vec![]);
        e.call(0x0078_80a0, &args![this]);
        assert!(calls_to(&e, TILE_SET_FLOAT).is_empty());
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x3400, 0x8f8, 1]]);
        // NaN total: no steps either.
        e.set(this, ListBox::fTotalListHeight, f32::NAN);
        e.call_log = Some(vec![]);
        e.call(0x0078_80a0, &args![this]);
        assert!(calls_to(&e, ROUND_FLOAT).is_empty());
        // No container: nothing.
        e.set(this, ListBox::pContainerRect, Ptr::NULL);
        e.call_log = Some(vec![]);
        e.call(0x0078_80a0, &args![this]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn fn_007881d0_grows_a_full_array() {
        let mut e = engine();
        e.register(ARRAY_IS_FULL, |_, _| eax(1));
        e.register(0x0000_7004, |_, a| eax(0x5000 + a[1]));
        e.register(ARRAY_GROWN_SIZE, |_, _| eax(8));
        stub(&mut e, &[ARRAY_RESIZE]);
        e.put_vtable(0x0f00_0000, &[0, 0x0000_7004]);
        let this: Ptr<BSSimpleArray> = Ptr::new(e.mem.alloc(0x10));
        e.mem.set_u32(this.addr(), 0x0f00_0000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_81d0, &args![this]).u32(), 0);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0x5004);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 4);
        assert_eq!(e.call(0x0078_81d0, &args![this]).u32(), 1);
        assert_eq!(calls_to(&e, ARRAY_RESIZE), vec![vec![this.addr(), 8, 1]]);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 8);
        e.register(ARRAY_IS_FULL, |_, _| eax(0));
        assert_eq!(e.call(0x0078_81d0, &args![this]).u32(), 2);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 3);
    }

    #[test]
    fn fn_00788260_constructs_each_element() {
        let mut e = engine();
        e.register(PLACEMENT_NEW, |_, a| {
            eax(if a[1] == 0x1014 { 0 } else { a[1] })
        });
        stub(&mut e, &[ELEMENT_CONSTRUCT]);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0078_8260, &args![this, 0x1000u32, 3u32]);
        assert_eq!(
            calls_to(&e, PLACEMENT_NEW),
            vec![vec![0x14, 0x1000], vec![0x14, 0x1014], vec![0x14, 0x1028]]
        );
        assert_eq!(
            calls_to(&e, ELEMENT_CONSTRUCT),
            vec![vec![0x1000], vec![0x1028]]
        );
    }

    #[test]
    fn fn_00788330_allocates_0x14_bytes_per_element() {
        let mut e = engine();
        e.register(ALLOCATE, |_, a| eax(0x4000 + a[0]));
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x0078_8330, &args![this, 3u32]).u32(), 0x4000 + 0x3c);
    }

    #[test]
    fn fn_007883b0_reserves_and_sizes() {
        let mut e = engine();
        e.register(0x0000_7004, |_, a| eax(0x5000 + a[1]));
        e.register(PLACEMENT_NEW, |_, a| eax(a[1]));
        stub(&mut e, &[ELEMENT_CONSTRUCT]);
        e.put_vtable(0x0f00_0000, &[0, 0x0000_7004]);
        let this: Ptr<BSSimpleArray> = Ptr::new(e.mem.alloc(0x10));
        e.mem.set_u32(this.addr(), 0x0f00_0000);
        e.call_log = Some(vec![]);
        e.call(0x0078_83b0, &args![this, 2u32, 3u32]);
        assert_eq!(calls_to(&e, 0x0000_7004), vec![vec![this.addr(), 3]]);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0x5003);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 3);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 3);
        assert_eq!(calls_to(&e, ELEMENT_CONSTRUCT).len(), 3);
        e.call_log = Some(vec![]);
        e.call(0x0078_83b0, &args![this, 0u32, 0u32]);
        assert!(calls_to(&e, 0x0000_7004).is_empty());
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 0);
    }
}

#[cfg(test)]
mod level_up_menu_tests {
    use super::*;

    type Calls = Vec<Vec<u32>>;

    fn calls_to(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// The callees outside this unit these tests reach, doubled to do
    /// nothing unless a test says otherwise.
    const OUTSIDE: [u32; 77] = [
        FREE,
        ARRAY_IS_FULL,
        ELEMENT_CONSTRUCT,
        LIST_DESTRUCT,
        LIST_BOX_UNREGISTER,
        SKILL_LIST_DESTROY_00721150,
        MENU_DESTROY,
        TILE_GET_MENU_BY_CLASS,
        FOCUS_OWNER_004B7210,
        INTERFACE_TILE_00586150,
        TILE_READ_FILE,
        TILE_GET_MENU,
        MENU_SET_MENU_TILE,
        REPORT_005B5E40,
        TILE_GET_FLOAT,
        TILE_GET_MAXIMUM_DEPTH,
        TILE_SET_FLOAT,
        TILE_SET_INT,
        TILE_SET_STRING,
        TILE_POKE,
        TEXT_TO_TRAIT,
        SHOW_TUTORIAL,
        SETTING_STRING,
        SETTING_INT,
        PRINT_FORMATTED,
        STRING_COPY,
        STRING_LENGTH,
        PLAYER_LEVEL,
        ACTOR_VALUE_OWNER_00648BC0,
        HANDLE_ENTRY_POINT,
        PERK_LIST_FIRST_00460FB0,
        LIST_COUNT_005AE380,
        LIST_BOX_INITIALIZE,
        LIST_BOX_FOR_EACH,
        LIST_BOX_SAVE_LIST_STATE,
        LIST_BOX_ITEM_DATA_007E1110,
        HUD_CLEAR_XP_METER,
        PROGRESSION_00464E10,
        PROGRESSION_008D5270,
        PROGRESSION_READY_TO_LEVEL_UP,
        ACTOR_UPDATE_MAGIC,
        STATS_UPDATE_PLAYER_NAME,
        STATS_UPDATE_PLAYER_TITLE,
        MENU_START_FADE_OUT,
        ACTOR_VALUE_ICON,
        ACTOR_VALUE_DESC,
        ACTOR_VALUE_BADGE,
        PLAY_MENU_SOUND,
        FULL_NAME_GET,
        PERK_DESCRIPTION,
        TILE_GET_STRING,
        TILE_PARENT_0045CD60,
        TILE_IS_TRUE,
        ADD_USER_TRAIT,
        FTOL2,
        MIN_004A8F20,
        INT_PAIR_00647B70,
        SOUND_CHECK_00525430,
        FLAG_004A4040,
        AUDIO_INSTANCE,
        GET_SOUND_HANDLE_BY_NAME,
        SOUND_HANDLE_PLAY,
        SOUND_HANDLE_DESTROY,
        MENU_PREP_FOR_VISIBILITY,
        TILE_GET_CHILD_BY_ID,
        TILE_PLAY_TILE_SOUND,
        ACTOR_GET_CLASS,
        CLASS_TAGS_005A5F40,
        ARRAY_SIZE_0044DDC0,
        ARRAY_ELEMENT_006B3160,
        ARRAY_CLEAR_008454F0,
        PERK_ENTRY_005EB900,
        COMPARE_STRINGS_00408B20,
        PERK_LEVEL_006AABF0,
        PERK_NUM_RANKS_005D4EE0,
        ARRAY_DESTROY_00788390,
        POINT_PERCENT_004BD510,
    ];

    /// The user trait ids the tests give the statics.
    const TRAITS: [(u32, u32); 9] = [
        (UT_MAX_POINTS, 0x101),
        (UT_CURR_POINTS, 0x102),
        (UT_SELECTED, 0x103),
        (UT_BASE_VALUE, 0x104),
        (UT_CURRENT_PAGE, 0x105),
        (UT_EXTRA_VALUE, 0x106),
        (UT_ADDED_VALUE, 0x107),
        (UT_TEXT_ALPHA, 0x108),
        (UT_RANK, 0x109),
    ];

    const ROOT: u32 = 0x5000;
    const PLAYER_VTABLE: u32 = 0x0f10_0000;
    const PLAYER_VALUES_VTABLE: u32 = 0x0f11_0000;
    const MENU_VTABLE: u32 = 0x0f12_0000;
    const LIST_VTABLE: u32 = 0x0f13_0000;

    fn engine() -> Engine {
        let mut e = Engine::new();
        for &addr in OUTSIDE.iter() {
            e.register(addr, |_, _| Ret::default());
        }
        e.map(0x011d_9000, 0x2000);
        e.map(0x011d_e000, 0x1000);
        e.map(0x011c_3000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.map(0x0101_e000, 0x1000);
        e.map(0x0102_3000, 0x1000);
        e.map(0x0107_0000, 0x1000);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.set_global(FULL_ALPHA_DOUBLE, 255.0f64);
        e.set_global(FULL_ALPHA_FLOAT, 255.0f32);
        e.set_global(HALF_ALPHA_FLOAT, 128.0f32);
        e.set_global(ROOT_VALUE_010704F0, 6006.0f64);
        e.set_global(ROOT_VALUE_010704E8, 102.0f64);
        for (address, id) in TRAITS {
            e.mem.set_u32(address, id);
        }
        e.register(NODE_SELF, |_, a| eax(a[0]));
        e.register(NODE_NEXT, |e, a| eax(e.mem.u32(a[0] + 4)));
        e.register(TEXT_TO_TRAIT, |_, a| eax(a[0] & 0xffff));
        e.register(FTOL2, |_, a| {
            eax(f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32 as u32)
        });
        // The player: slots +0x48c (an actor value as a float), +0x394,
        // +0x4a0 (a rank) and +0x498; its values object at +0xa4 (slot
        // +0x1c).
        let mut slots = vec![0u32; 0x4a4 / 4];
        slots[0x394 / 4] = 0x0000_8394;
        slots[0x48c / 4] = 0x0000_848c;
        slots[0x498 / 4] = 0x0000_8498;
        slots[0x4a0 / 4] = 0x0000_84a0;
        e.put_vtable(PLAYER_VTABLE, &slots);
        let mut values = vec![0u32; 8];
        values[7] = 0x0000_801c;
        e.put_vtable(PLAYER_VALUES_VTABLE, &values);
        let player = e.mem.alloc(0x100);
        e.mem.set_u32(player, PLAYER_VTABLE);
        e.mem.set_u32(player + 0xa4, PLAYER_VALUES_VTABLE);
        e.mem.set_u32(LEVEL_UP_PLAYER, player);
        e.register(0x0000_8394, |_, _| Ret::default());
        e.register(0x0000_848c, |_, _| st0(40.0));
        e.register(0x0000_8498, |_, _| Ret::default());
        e.register(0x0000_84a0, |_, _| eax(0x0000_0102));
        e.register(0x0000_801c, |_, a| eax(a[1] + 10));
        e
    }

    fn player(e: &Engine) -> u32 {
        e.mem.u32(LEVEL_UP_PLAYER)
    }

    /// A level-up menu with its ten tiles (`0x6000 + 0x10 * index`), the menu
    /// tile at +4, the menu vtable and both list boxes' vtables.
    fn menu(e: &mut Engine) -> u32 {
        let menu = e.mem.alloc(0xcc);
        e.mem.set_u32(menu, MENU_VTABLE);
        e.mem.set_u32(menu + 4, ROOT);
        for index in 0..10 {
            e.mem
                .set_u32(menu + MENU_TILES + 4 * index, 0x6000 + 0x10 * index);
        }
        e.mem.set_u32(menu + SKILLS_LIST, LIST_VTABLE);
        e.mem.set_u32(menu + PERKS_LIST, LIST_VTABLE);
        let mut slots = vec![0u32; 0x40 / 4];
        for (index, slot) in slots.iter_mut().enumerate() {
            *slot = 0x0000_9000 + 4 * index as u32;
        }
        e.put_vtable(MENU_VTABLE, &slots);
        let mut slots = vec![0u32; 0x24 / 4];
        for (index, slot) in slots.iter_mut().enumerate() {
            *slot = 0x0000_9100 + 4 * index as u32;
        }
        e.put_vtable(LIST_VTABLE, &slots);
        for index in 0..0x10 {
            e.register(0x0000_9000 + 4 * index, |_, _| Ret::default());
        }
        for index in 0..9 {
            e.register(0x0000_9100 + 4 * index, |_, _| Ret::default());
        }
        menu
    }

    #[test]
    fn get_class_answers_0x403() {
        let mut e = engine();
        let this = menu(&mut e);
        assert_eq!(e.call(0x0078_4b80, &args![this]).u32(), 0x403);
        assert_eq!(e.call(0x0078_5180, &args![]).u32(), 0x403);
    }

    #[test]
    fn attach_tile_by_id_keeps_ids_0_to_9() {
        let mut e = engine();
        let this = e.mem.alloc(0xcc);
        e.call(0x0078_4b90, &args![this, 3u32, 0x1234u32]);
        e.call(0x0078_4b90, &args![this, 10u32, 0x5555u32]);
        e.call(0x0078_4b90, &args![this, 0xffff_ffffu32, 0x6666u32]);
        assert_eq!(e.mem.u32(this + MENU_TILES + 12), 0x1234);
        assert_eq!(e.mem.u32(this + MENU_TILES + 40), 0);
        assert_eq!(e.mem.u32(this + MENU_TILES - 4), 0);
    }

    #[test]
    fn destructor_destroys_the_members_and_frees_with_bit_0() {
        let mut e = engine();
        let this = menu(&mut e);
        e.mem.set_u32(LEVEL_UP_MENU_ME, this);
        e.call_log = Some(vec![]);
        let answer = e.call(0x0078_4bc0, &args![this, 1u32]).u32();
        assert_eq!(answer, this);
        assert_eq!(e.mem.u32(this), LEVEL_UP_MENU_VTABLE);
        assert_eq!(e.mem.u32(LEVEL_UP_MENU_ME), 0);
        assert_eq!(
            calls_to(&e, LIST_DESTRUCT),
            vec![vec![this + AVAILABLE_PERKS], vec![this + PERKS_LIST + 4]]
        );
        assert_eq!(e.mem.u32(this + PERKS_LIST), PERK_LIST_BOX_VTABLE);
        assert_eq!(
            calls_to(&e, SKILL_LIST_DESTROY_00721150),
            vec![vec![this + SKILLS_LIST]]
        );
        assert_eq!(calls_to(&e, MENU_DESTROY), vec![vec![this]]);
        assert_eq!(calls_to(&e, FREE), vec![vec![this]]);
        e.call_log = Some(vec![]);
        e.call(0x0078_4bc0, &args![this, 0u32]);
        assert!(calls_to(&e, FREE).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x0078_4bf0, &args![this]);
        assert_eq!(calls_to(&e, MENU_DESTROY), vec![vec![this]]);
    }

    #[test]
    fn create_builds_the_menu_with_its_perks() {
        let mut e = engine();
        let this = menu(&mut e);
        e.mem.set_u32(this + 0xc4, 0);
        // An open menu to delete first.
        let old = menu(&mut e);
        e.register_double(TILE_GET_MENU_BY_CLASS, move |_, _| eax(old));
        e.register(FOCUS_OWNER_004B7210, |_, _| eax(0x7000));
        e.register(INTERFACE_TILE_00586150, |_, _| eax(0x7100));
        e.register(TILE_READ_FILE, |_, _| eax(0x7200));
        e.register_double(TILE_GET_MENU, move |_, _| eax(this));
        e.register(0x0000_9034, |_, _| eax(0x403));
        e.register(TILE_GET_FLOAT, |_, a| {
            st0(if a[1] == 0xfa7 { 102.0 } else { 0.0 })
        });
        e.register(TILE_GET_MAXIMUM_DEPTH, |_, _| st0(3.0));
        e.register(PLAYER_LEVEL, |_, _| eax(0xffff_0004));
        e.register(SETTING_STRING, |_, a| eax(a[0] & 0xffff));
        e.register(ACTOR_VALUE_OWNER_00648BC0, |_, _| eax(12));
        e.register(HANDLE_ENTRY_POINT, |e, a| {
            let value = e.mem.f32(a[2]);
            e.mem.set_u32(a[2], (value * 1.5).to_bits());
            Ret::default()
        });
        let every = e.mem.alloc(4);
        e.mem.set_u32(every, 2);
        e.register_double(SETTING_INT, move |_, _| eax(every));
        // The game's perk list: two perks; the first qualifies.
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(first, 0x7a00);
        e.mem.set_u32(first + 4, second);
        e.mem.set_u32(second, 0x7b00);
        e.register_double(PERK_LIST_FIRST_00460FB0, move |_, _| eax(first));
        e.register(PERK_LEVEL_006AABF0, |_, a| {
            eax(if a[0] == 0x7a00 { 2 } else { 0 })
        });
        e.register(PERK_NUM_RANKS_005D4EE0, |_, _| eax(3));
        e.register(PERK_FLAG_007A9280, |_, _| eax(0));
        e.register(PERK_IS_ATTAINABLE, |_, _| eax(1));
        e.register(PERK_FLAG_005EB5E0, |_, _| eax(1));
        let flag = e.mem.alloc(4);
        e.register_double(BYTE_FLAG_00408D60, move |_, _| eax(flag));
        e.register(LIST_APPEND_00905820, |e, a| {
            let perk = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], perk);
            Ret::default()
        });
        e.register(LIST_COUNT_005AE380, |_, _| eax(0));
        e.register(ACTOR_VALUE_SECTION_COUNT, |_, _| eax(0));
        e.call_log = Some(vec![]);
        let answer = e.call(0x0078_4c80, &args![]).u32();
        assert_eq!(answer, 0x7200);
        assert_eq!(e.mem.u32(LEVEL_UP_MENU_ME), this);
        assert_eq!(calls_to(&e, 0x0000_9000), vec![vec![old, 1]]);
        assert_eq!(
            calls_to(&e, TILE_READ_FILE),
            vec![vec![0x7100, LEVEL_UP_MENU_FILE]]
        );
        assert_eq!(
            calls_to(&e, MENU_SET_MENU_TILE),
            vec![vec![this, 0x7200, 0]]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT)[0],
            vec![0x7200, 0xfad, 3.0f32.to_bits(), 1]
        );
        assert_eq!(
            calls_to(&e, SHOW_TUTORIAL),
            vec![vec![0x7000 + 0x4d4, 0x10, 0x403, 0x200]]
        );
        let printed = calls_to(&e, PRINT_FORMATTED);
        assert_eq!(&printed[0][1..], &[0x40, 0x4048, 4]);
        assert_eq!(
            calls_to(&e, TILE_POKE),
            vec![vec![0x7200, 0x3cc8, 1.0f32.to_bits()]]
        );
        // 12 points through the entry point (times 1.5): 18, which the
        // skill list (no skills here) then limits.
        assert_eq!(calls_to(&e, MIN_004A8F20), vec![vec![18, 0]]);
        assert_eq!(e.mem.u32(this + CURR_POINTS), 0);
        assert_eq!(e.mem.u32(this + MAX_POINTS + 4), 1);
        assert_eq!(e.mem.u32(this + CURR_POINTS + 4), 0);
        // Level 4 is a multiple of 2: the first perk (rank 2 of 3, level 2)
        // is added; the second has no level.
        assert_eq!(e.mem.u32(this + AVAILABLE_PERKS), 0x7a00);
        // No perks counted: `_EndPage` 0 on the menu tile.
        assert!(calls_to(&e, TILE_SET_INT).contains(&vec![ROOT, 0x3d24, 0]));
        assert_eq!(
            calls_to(&e, LIST_BOX_INITIALIZE)[0],
            vec![this + SKILLS_LIST, 0x6010, SKILL_TEMPLATE_TEXT]
        );
        assert_eq!(e.mem.u32(this + CURRENT_PAGE), 0);

        // Missing a tile: the failure is reported and the answer is 0.
        e.mem.set_u32(this + MENU_TILES + 4 * 9, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_4c80, &args![]).u32(), 0);
        assert_eq!(
            calls_to(&e, REPORT_005B5E40),
            vec![vec![LEVEL_UP_CREATE_FAILED]]
        );

        // Another class: the menu is deleted and the answer is 0.
        e.register(0x0000_9034, |_, _| eax(0x3ea));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_4c80, &args![]).u32(), 0);
        assert_eq!(calls_to(&e, 0x0000_9000), vec![vec![old, 1], vec![this, 1]]);
    }

    #[test]
    fn create_skips_perks_failing_their_conditions() {
        let mut e = engine();
        e.mem.set_u32(LEVEL_UP_MENU_ME, 0);
        let perk = 0x7a00u32;
        // The byte at 00408d60 set: the perk's conditions (00680c60) fail.
        let flag = e.mem.alloc(4);
        e.mem.set_u8(flag, 1);
        e.register_double(BYTE_FLAG_00408D60, move |_, _| eax(flag));
        e.register(CONDITION_IS_TRUE, |_, _| eax(0));
        e.register(PERK_NUM_RANKS_005D4EE0, |_, _| eax(3));
        e.register(PERK_FLAG_007A9280, |_, _| eax(0));
        e.register(PERK_IS_ATTAINABLE, |_, _| eax(1));
        e.register(PERK_FLAG_005EB5E0, |_, _| eax(1));
        assert!(!perk_qualifies(&mut e, perk));
        // The conditions pass but the perk's level (5) is above the
        // player's (4).
        e.register(CONDITION_IS_TRUE, |_, _| eax(1));
        e.register(PERK_LEVEL_006AABF0, |_, _| eax(5));
        e.register(PLAYER_LEVEL, |_, _| eax(4));
        assert!(!perk_qualifies(&mut e, perk));
        e.register(PERK_LEVEL_006AABF0, |_, _| eax(4));
        assert!(perk_qualifies(&mut e, perk));
        // The player's rank (2) reaching the ranks (2).
        e.register(PERK_NUM_RANKS_005D4EE0, |_, _| eax(2));
        assert!(!perk_qualifies(&mut e, perk));
    }

    #[test]
    fn has_all_tiles_needs_the_ten() {
        let mut e = engine();
        let this = menu(&mut e);
        assert!(e.call(0x0078_5190, &args![this]).bool());
        e.mem.set_u32(this + MENU_TILES + 4 * 4, 0);
        assert!(!e.call(0x0078_5190, &args![this]).bool());
    }

    #[test]
    fn close_gives_the_perks_and_fades_out() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x0078_51d0, &args![]);
        assert_eq!(calls_to(&e, HUD_CLEAR_XP_METER).len(), 1);
        assert!(calls_to(&e, MENU_START_FADE_OUT).is_empty());

        let this = menu(&mut e);
        e.mem.set_u32(LEVEL_UP_MENU_ME, this);
        e.register(PROGRESSION_00464E10, |_, _| eax(0x7700));
        e.register(PROGRESSION_READY_TO_LEVEL_UP, |_, _| eax(1));
        e.call_log = Some(vec![]);
        e.call(0x0078_51d0, &args![]);
        let player = player(&e);
        assert_eq!(
            calls_to(&e, PROGRESSION_00464E10),
            vec![vec![player], vec![player]]
        );
        assert_eq!(calls_to(&e, PROGRESSION_008D5270), vec![vec![0x7700]]);
        assert_eq!(
            calls_to(&e, LIST_BOX_FOR_EACH),
            vec![vec![this + PERKS_LIST, GIVE_PERK_CALLBACK, 0, 0x7fff_ffff]]
        );
        assert_eq!(calls_to(&e, ACTOR_UPDATE_MAGIC), vec![vec![player, 1, 0]]);
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![ROOT, 0x1772, 1]]);
        assert_eq!(calls_to(&e, MENU_START_FADE_OUT), vec![vec![this]]);
        assert_eq!(e.mem.u32(LEVEL_UP_MENU_ME), 0);
    }

    #[test]
    fn do_enter_describes_skills_perks_and_arrows() {
        let mut e = engine();
        let this = menu(&mut e);
        e.register(LIST_BOX_ITEM_DATA_007E1110, |_, a| eax(a[1] + 1));
        e.register(ACTOR_VALUE_ICON, |_, a| eax(a[0] + 0x100));
        e.register(ACTOR_VALUE_DESC, |_, a| eax(a[0] + 0x200));
        e.register(ACTOR_VALUE_BADGE, |_, _| eax(0x7300));
        e.call_log = Some(vec![]);
        e.call(0x0078_5280, &args![this, 0xbu32, 0x40u32]);
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![
                vec![0x6030, 0xfcc, 0x141, 1],
                vec![0x6040, 0xfc4, 0x241, 1],
                vec![0x6090, 0xfcc, 0x7300, 1]
            ]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![
                vec![0x6030, 0xfa3, 1],
                vec![0x6040, 0xfa3, 1],
                vec![0x6090, 0xfa3, 1]
            ]
        );
        assert_eq!(calls_to(&e, PLAY_MENU_SOUND), vec![vec![4]]);

        // A perk with a one-character name: tile 3 hidden.
        e.register(FULL_NAME_GET, |_, a| eax(a[0]));
        e.register(STRING_LENGTH, |_, _| eax(1));
        e.call_log = Some(vec![]);
        e.call(0x0078_5280, &args![this, 0xcu32, 0x40u32]);
        assert_eq!(calls_to(&e, FULL_NAME_GET), vec![vec![0x41 + 0x2c]]);
        let description = calls_to(&e, PERK_DESCRIPTION);
        assert_eq!(description[0][0], 0x41);
        assert_eq!(description[0][2], 0x200);
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![
                vec![0x6030, 0xfa3, 0],
                vec![0x6040, 0xfa3, 1],
                vec![0x6090, 0xfa3, 0]
            ]
        );

        // An arrow: the skill list highlights its parent, the menu enters it.
        e.register(TILE_PARENT_0045CD60, |_, _| eax(0x7400));
        e.call_log = Some(vec![]);
        e.call(0x0078_5280, &args![this, 0xdu32, 0x40u32]);
        assert_eq!(
            calls_to(&e, 0x0000_9100),
            vec![vec![this + SKILLS_LIST, 0x7400]]
        );
        assert_eq!(calls_to(&e, 0x0000_9010), vec![vec![this, 0xb, 0x7400]]);
    }

    #[test]
    fn init_lists_fills_both_lists() {
        let mut e = engine();
        let this = menu(&mut e);
        e.mem.set_u32(this + MAX_POINTS, 300);
        e.register(ACTOR_VALUE_SECTION_COUNT, |_, _| eax(3));
        e.register(ACTOR_VALUE_TO_INDEX, |_, a| eax(0x20 + a[1]));
        // The second value (0x21) is flagged 0x2000.
        e.register(FLAG_TEST_00406D70, |_, a| eax(u32::from(a[0] == 0x21)));
        e.register(ACTOR_VALUE_NAME, |_, a| eax(a[0] + 0x300));
        e.register(SKILL_LIST_ADD_ITEM, |_, a| eax(a[1] + 0x1000));
        e.register(TILE_SET_LABEL_00707E10, |_, _| Ret::default());
        e.register(MIN_004A8F20, |_, a| {
            eax((a[0] as i32).min(a[1] as i32) as u32)
        });
        e.register(INT_PAIR_00647B70, |_, a| {
            eax((a[0] as i32).max(a[1] as i32) as u32)
        });
        // One available perk with an empty name and 3 ranks, rank 2.
        e.mem.set_u32(this + AVAILABLE_PERKS, 0x7a00);
        e.mem.set_u32(this + AVAILABLE_PERKS + 4, 0);
        e.register(STRING_LENGTH, |_, _| eax(0));
        e.register(PERK_NUM_RANKS_005D4EE0, |_, _| eax(3));
        e.register(SETTING_STRING, |_, _| eax(0x7500));
        // list_box_bgs_perk_add_item without a container answers 0.
        e.call_log = Some(vec![]);
        e.call(0x0078_5540, &args![this]);
        assert_eq!(
            calls_to(&e, SKILL_LIST_ADD_ITEM),
            vec![
                vec![this + SKILLS_LIST, 0x20, 0x320, 0, 0],
                vec![this + SKILLS_LIST, 0x22, 0x322, 0, 0]
            ]
        );
        assert!(calls_to(&e, TILE_SET_INT).contains(&vec![0x1020, 0x104, 0x2a]));
        assert_eq!(
            calls_to(&e, 0x0000_9120),
            vec![vec![this + SKILLS_LIST, SKILL_COMPARE_007DA270]]
        );
        // Two skills worth 42 and 44: min(300, 200 - 86) = 114.
        assert_eq!(calls_to(&e, MIN_004A8F20), vec![vec![300, 114]]);
        assert_eq!(e.mem.u32(this + MAX_POINTS), 114);
        let copies = calls_to(&e, STRING_COPY);
        assert_eq!(copies.len(), 2);
        assert_eq!(copies[1][2], 0x7500);
        let printed = calls_to(&e, PRINT_FORMATTED);
        assert_eq!(printed.len(), 1);
        assert_eq!(&printed[0][1..3], &[0x40, NAME_RANK_FORMAT]);
        assert_eq!(printed[0][4], 3);
        // The perk's line (0 without a container) gets id 12 and rank 3.
        assert!(calls_to(&e, TILE_SET_INT).contains(&vec![0, 0xfaa, 0xc]));
        assert!(calls_to(&e, TILE_SET_INT).contains(&vec![0, 0x109, 3]));
    }

    #[test]
    fn set_current_page_sets_the_traits_or_closes() {
        let mut e = engine();
        let this = menu(&mut e);
        e.mem.set_u32(this + CURR_POINTS + 4, 1);
        e.mem.set_u32(this + MAX_POINTS + 4, 1);
        e.register(TILE_GET_FLOAT, |_, _| st0(1.0));
        e.call_log = Some(vec![]);
        e.call(0x0078_5830, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this + CURRENT_PAGE), 1);
        assert_eq!(
            calls_to(&e, TILE_SET_INT)[..3].to_vec(),
            vec![
                vec![ROOT, 0x105, 1],
                vec![ROOT, 0x102, 1],
                vec![ROOT, 0x101, 1]
            ]
        );
        assert_eq!(
            calls_to(&e, LIST_BOX_FOR_EACH),
            vec![vec![this + PERKS_LIST, TEXT_ALPHA_CALLBACK, 0, 0x7fff_ffff]]
        );
        assert_eq!(
            calls_to(&e, 0x0000_9120),
            vec![vec![this + PERKS_LIST, PERK_COMPARE]]
        );
        assert_eq!(
            calls_to(&e, LIST_BOX_SAVE_LIST_STATE),
            vec![vec![this + SKILLS_LIST]]
        );

        e.call_log = Some(vec![]);
        e.call(0x0078_5830, &args![this, 0u32]);
        assert_eq!(
            calls_to(&e, LIST_BOX_FOR_EACH),
            vec![vec![
                this + SKILLS_LIST,
                SET_DISPLAY_STRING_CALLBACK,
                0,
                0x7fff_ffff
            ]]
        );
        assert_eq!(
            calls_to(&e, LIST_BOX_SAVE_LIST_STATE),
            vec![vec![this + PERKS_LIST]]
        );

        // Past `_EndPage` the menu closes.
        e.mem.set_u32(LEVEL_UP_MENU_ME, 0);
        e.call_log = Some(vec![]);
        e.call(0x0078_5830, &args![this, 2u32]);
        assert_eq!(calls_to(&e, HUD_CLEAR_XP_METER).len(), 1);
        assert_eq!(e.mem.u32(this + CURRENT_PAGE), 0);
    }

    #[test]
    fn update_point_counter_shows_what_is_left() {
        let mut e = engine();
        let this = menu(&mut e);
        e.register(SETTING_STRING, |_, a| eax(a[0] & 0xffff));
        e.mem.set_u32(this + MAX_POINTS, 5);
        e.mem.set_u32(this + CURR_POINTS, 5);
        e.call_log = Some(vec![]);
        e.call(0x0078_5990, &args![this]);
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x6050, 0xfa3, 0]]);

        e.mem.set_u32(this + CURR_POINTS, 4);
        e.call_log = Some(vec![]);
        e.call(0x0078_5990, &args![this]);
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![vec![0x6050, 0xfc4, 0x2a24, 1]]
        );
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x6050, 0xfa3, 1]]);

        e.mem.set_u32(this + CURRENT_PAGE, 1);
        e.mem.set_u32(this + MAX_POINTS + 4, 9);
        e.call_log = Some(vec![]);
        e.call(0x0078_5990, &args![this]);
        let printed = calls_to(&e, PRINT_FORMATTED);
        assert_eq!(&printed[0][1..], &[0x40, 0x2a00, 9]);
        assert_eq!(
            calls_to(&e, TILE_SET_STRING)[0][0..2].to_vec(),
            vec![0x6050, 0xfc4]
        );
    }

    #[test]
    fn do_click_resets_turns_pages_and_picks() {
        let mut e = engine();
        let this = menu(&mut e);
        e.mem.set_u32(this + CURR_POINTS, 3);
        e.call_log = Some(vec![]);
        e.call(0x0078_5aa0, &args![this, 6u32, 0u32]);
        assert_eq!(calls_to(&e, PLAY_MENU_SOUND), vec![vec![1]]);
        assert_eq!(e.mem.u32(this + CURR_POINTS), 0);
        assert_eq!(calls_to(&e, TILE_SET_INT)[0], vec![ROOT, 0x102, 0]);
        assert_eq!(
            calls_to(&e, LIST_BOX_FOR_EACH),
            vec![vec![
                this + SKILLS_LIST,
                RESET_SKILL_CALLBACK,
                0,
                0x7fff_ffff
            ]]
        );

        // Next page: page 1 then trait 0xfc3 of tile 7.
        e.register(TILE_GET_FLOAT, |_, _| st0(1.0));
        e.call_log = Some(vec![]);
        e.call(0x0078_5aa0, &args![this, 7u32, 0u32]);
        assert_eq!(e.mem.u32(this + CURRENT_PAGE), 1);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT).last().unwrap(),
            &vec![0x6070, 0xfc3, 0, 1]
        );
        e.call_log = Some(vec![]);
        e.call(0x0078_5aa0, &args![this, 8u32, 0u32]);
        assert_eq!(e.mem.u32(this + CURRENT_PAGE), 0);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT).last().unwrap(),
            &vec![0x6080, 0xfc3, 0, 1]
        );

        // A perk that is not selected and bright: everything unselected
        // (an empty list), then it is selected and applied.
        e.mem.set_u32(this + CURRENT_PAGE, 1);
        e.register(TILE_GET_FLOAT, |_, _| st0(255.0));
        e.call_log = Some(vec![]);
        e.call(0x0078_5aa0, &args![this, 0xcu32, 0x40u32]);
        assert_eq!(
            calls_to(&e, LIST_BOX_ITEM_DATA_007E1110),
            vec![vec![this + PERKS_LIST, 0x40]]
        );
        assert_eq!(e.mem.u32(this + CURR_POINTS + 4), 0);
        assert!(calls_to(&e, TILE_SET_INT).contains(&vec![ROOT, 0x102, 0]));
    }

    #[test]
    fn do_click_arrows_move_points_and_the_skill() {
        let mut e = engine();
        let this = menu(&mut e);
        e.mem.set_u32(this + CURR_POINTS, 2);
        e.register(TILE_IS_TRUE, |_, _| eax(1));
        e.register(TILE_PARENT_0045CD60, |_, _| eax(0x7400));
        e.register(LIST_BOX_ITEM_DATA_007E1110, |_, _| eax(0x26));
        e.register(ACTOR_GET_CLASS, |_, _| eax(0x7800));
        e.register(CLASS_TAGS_005A5F40, |_, _| eax(1));
        let tag = e.mem.alloc(4);
        e.mem.set_u32(tag, 2);
        e.register_double(SETTING_INT, move |_, _| eax(tag));
        e.register(TILE_GET_FLOAT, |_, a| {
            st0(if a[1] == 0x107 { 4.0 } else { 0.0 })
        });
        e.call_log = Some(vec![]);
        e.call(0x0078_5aa0, &args![this, 0xeu32, 0x40u32]);
        assert_eq!(e.mem.u32(this + CURR_POINTS), 3);
        assert_eq!(calls_to(&e, CLASS_TAGS_005A5F40), vec![vec![0x7800, 0x26]]);
        // A tag skill moves by 2: `_AddedValue` 4.0 + 2, the player's
        // 40.0 + 2.
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT)[0],
            vec![0x7400, 0x107, 6.0f32.to_bits(), 1]
        );
        let player = player(&e);
        assert_eq!(calls_to(&e, 0x0000_848c)[0][1], 0x26);
        assert_eq!(
            calls_to(&e, 0x0000_8394),
            vec![vec![player, 0x26, 42.0f32.to_bits()]]
        );
        // Not shown: nothing.
        e.register(TILE_IS_TRUE, |_, _| eax(0));
        e.call_log = Some(vec![]);
        e.call(0x0078_5aa0, &args![this, 0xdu32, 0x40u32]);
        assert_eq!(e.mem.u32(this + CURR_POINTS), 3);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn do_idle_plays_the_success_sound_once_faded_in() {
        let mut e = engine();
        let this = menu(&mut e);
        e.register(AUDIO_INSTANCE, |_, _| eax(0x7900));
        e.register(GET_SOUND_HANDLE_BY_NAME, |_, a| eax(a[1]));
        e.call_log = Some(vec![]);
        e.call(0x0078_5e20, &args![this]);
        assert!(calls_to(&e, SOUND_HANDLE_PLAY).is_empty());
        e.mem.set_u32(this + 0x24, 4);
        e.call_log = Some(vec![]);
        e.call(0x0078_5e20, &args![this]);
        let found = calls_to(&e, GET_SOUND_HANDLE_BY_NAME);
        assert_eq!(found[0][0], 0x7900);
        assert_eq!(&found[0][2..], &[SUCCESS_SOUND_TEXT, 0x901]);
        assert_eq!(calls_to(&e, SOUND_HANDLE_PLAY), vec![vec![found[0][1], 0]]);
        assert_eq!(calls_to(&e, MENU_PREP_FOR_VISIBILITY), vec![vec![this, 0]]);
        e.register(FLAG_004A4040, |_, _| eax(1));
        e.call_log = Some(vec![]);
        e.call(0x0078_5e20, &args![this]);
        assert!(calls_to(&e, SOUND_HANDLE_PLAY).is_empty());
    }

    #[test]
    fn do_gamepad_clicks_arrows_perks_and_back() {
        let mut e = engine();
        let this = menu(&mut e);
        e.register(0x0000_9104, |_, _| eax(0x7a00));
        e.register(TILE_GET_CHILD_BY_ID, |_, a| eax(a[0] + a[1]));
        e.register(TILE_IS_TRUE, |_, _| eax(1));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_5ed0, &args![this, 3u32]).u32() as u8, 1);
        assert_eq!(calls_to(&e, TILE_GET_CHILD_BY_ID), vec![vec![0x7a00, 0xe]]);
        assert_eq!(
            calls_to(&e, TILE_PLAY_TILE_SOUND),
            vec![vec![0x7a0e, 0xfcb]]
        );
        assert_eq!(calls_to(&e, 0x0000_900c), vec![vec![this, 0xe, 0x7a0e]]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_5ed0, &args![this, 4u32]).u32() as u8, 1);
        assert_eq!(calls_to(&e, 0x0000_900c), vec![vec![this, 0xd, 0x7a0d]]);
        // Button 9 on the skills page goes to vtable +0x38.
        e.register(0x0000_9038, |_, _| eax(1));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_5ed0, &args![this, 9u32]).u32() as u8, 1);
        assert_eq!(
            calls_to(&e, 0x0000_9038),
            vec![vec![this, 3, 1.0f32.to_bits()]]
        );
        // On the perks page: 9 clicks the highlighted perk, 10 goes back.
        e.mem.set_u32(this + CURRENT_PAGE, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_5ed0, &args![this, 9u32]).u32() as u8, 1);
        assert_eq!(calls_to(&e, 0x0000_900c), vec![vec![this, 0xc, 0x7a00]]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_5ed0, &args![this, 10u32]).u32() as u8, 1);
        assert_eq!(
            calls_to(&e, TILE_PLAY_TILE_SOUND),
            vec![vec![0x6080, 0xfcb]]
        );
        assert_eq!(calls_to(&e, 0x0000_900c), vec![vec![this, 8, 0]]);
        // Arrows on the perks page and other buttons are not handled.
        assert_eq!(e.call(0x0078_5ed0, &args![this, 3u32]).u32() as u8, 0);
        assert_eq!(e.call(0x0078_5ed0, &args![this, 5u32]).u32() as u8, 0);
    }

    #[test]
    fn do_gamepad_up_event_clicks_next() {
        let mut e = engine();
        let this = menu(&mut e);
        assert_eq!(
            e.call(0x0078_6110, &args![this, 0xbu32, 0u32]).u32() as u8,
            0
        );
        e.register(TILE_IS_TRUE, |_, _| eax(1));
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0078_6110, &args![this, 0xbu32, 0u32]).u32() as u8,
            1
        );
        assert_eq!(calls_to(&e, TILE_IS_TRUE), vec![vec![0x6070, 0xfaf]]);
        assert_eq!(calls_to(&e, PLAY_MENU_SOUND), vec![vec![1]]);
        assert_eq!(calls_to(&e, 0x0000_900c), vec![vec![this, 7, 0]]);
        assert_eq!(
            e.call(0x0078_6110, &args![this, 0xau32, 0u32]).u32() as u8,
            0
        );
    }

    #[test]
    fn reset_skill_takes_the_added_value_back() {
        let mut e = engine();
        let this = menu(&mut e);
        e.mem.set_u32(LEVEL_UP_MENU_ME, this);
        e.register(TILE_GET_FLOAT, |_, a| {
            st0(if a[1] == 0x107 { 5.0 } else { 0.0 })
        });
        e.call_log = Some(vec![]);
        e.call(0x0078_6170, &args![0x7400u32, 0x26u32]);
        let player = player(&e);
        assert_eq!(
            calls_to(&e, 0x0000_8394),
            vec![vec![player, 0x26, 35.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, TILE_SET_FLOAT)[0], vec![0x7400, 0x107, 0, 1]);
        // The line's text is refreshed (`_DisplayString`).
        assert_eq!(
            calls_to(&e, ADD_USER_TRAIT),
            vec![vec![DISPLAY_STRING_TEXT, 0xffff_ffff]]
        );
    }

    #[test]
    fn reset_perk_unselects_and_takes_the_effects_back() {
        let mut e = engine();
        let this = menu(&mut e);
        e.mem.set_u32(LEVEL_UP_MENU_ME, this);
        e.call_log = Some(vec![]);
        e.call(0x0078_6200, &args![0x7400u32, 0u32]);
        assert!(calls_to(&e, TILE_SET_INT).is_empty());
        e.register(TILE_IS_TRUE, |_, _| eax(1));
        e.call_log = Some(vec![]);
        e.call(0x0078_6200, &args![0x7400u32, 0u32]);
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x7400, 0x103, 0]]);
        assert_eq!(
            calls_to(&e, LIST_BOX_ITEM_DATA_007E1110),
            vec![vec![this + PERKS_LIST, 0x7400]]
        );
        assert_eq!(
            calls_to(&e, LIST_BOX_FOR_EACH),
            vec![vec![
                this + SKILLS_LIST,
                PERK_UPDATE_CALLBACK,
                0,
                0x7fff_ffff
            ]]
        );
    }

    /// `PerkEffectList` with entries (magnitude, actor value).
    fn perk_effects(e: &mut Engine, entries: &[(i32, u32)]) -> u32 {
        let buffer = e.mem.alloc(0x14 * entries.len() as u32);
        for (index, &(magnitude, value)) in entries.iter().enumerate() {
            let entry = buffer + 0x14 * index as u32;
            e.mem.set_u32(entry, magnitude as u32);
            e.mem.set_u32(entry + 0x10, value);
        }
        let count = entries.len() as u32;
        e.register_double(ARRAY_SIZE_0044DDC0, move |_, _| eax(count));
        e.register_double(
            ARRAY_ELEMENT_006B3160,
            move |_, a| eax(buffer + 0x14 * a[1]),
        );
        buffer
    }

    /// `PerkEffectList` with room for elements: not full (`00438b90`), a
    /// buffer of four, placement `new` at the place given.
    fn effect_array(e: &mut Engine) -> u32 {
        let buffer = e.mem.alloc(0x50);
        e.mem.set_u32(PERK_EFFECT_LIST + 4, buffer);
        e.mem.set_u32(PERK_EFFECT_LIST + 0xc, 4);
        e.register(PLACEMENT_NEW, |_, a| eax(a[1]));
        buffer
    }

    #[test]
    fn perk_update_cuts_entries_at_the_limits() {
        let mut e = engine();
        let buffer = perk_effects(&mut e, &[(30, 0x26), (5, 0x27)]);
        // Base 50, extra 10, added 5 (65); `_MaxValue` 80: 65 + 30 is 15
        // over, kept as `_OverflowValue`.
        e.register(TILE_GET_FLOAT, |_, a| {
            st0(match a[1] {
                0x104 => 50.0,
                0x106 => 10.0,
                0x107 => 5.0,
                0x3df8 => 80.0,
                0x3dec => 0.0,
                _ => 0.0,
            })
        });
        e.call_log = Some(vec![]);
        e.call(0x0078_6240, &args![0x7400u32, 0x26u32]);
        assert_eq!(e.mem.i32(buffer), 15);
        assert_eq!(e.mem.i32(buffer + 0x14), 5);
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x7400, 0x3e04, 15]]);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x7400, 0x106, 25.0f32.to_bits(), 1]]
        );
        // A pending overflow (3) is added back first; below `_MinValue`
        // (70) the entry is cut up to it.
        e.mem.set_u32(buffer, (-10i32) as u32);
        e.register(TILE_GET_FLOAT, |_, a| {
            st0(match a[1] {
                0x104 => 50.0,
                0x106 => 10.0,
                0x107 => 5.0,
                0x3e04 => 3.0,
                0x3df8 => 100.0,
                0x3dec => 70.0,
                _ => 0.0,
            })
        });
        e.call_log = Some(vec![]);
        e.call(0x0078_6240, &args![0x7400u32, 0x26u32]);
        // -10 + 3 = -7; 65 - 7 = 58 is 12 under 70: -7 - (-12) = 5.
        assert_eq!(e.mem.i32(buffer), 5);
        assert_eq!(calls_to(&e, TILE_SET_FLOAT)[0], vec![0x7400, 0x3e04, 0, 1]);
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![vec![0x7400, 0x3e04, (-12i32) as u32]]
        );
    }

    #[test]
    fn text_alpha_follows_the_perk_conditions() {
        let mut e = engine();
        e.register(CONDITION_IS_TRUE, |_, _| eax(1));
        e.register(PERK_LEVEL_006AABF0, |_, _| eax(4));
        e.register(PLAYER_LEVEL, |_, _| eax(4));
        e.call_log = Some(vec![]);
        e.call(0x0078_64e0, &args![0x7400u32, 0x7a00u32]);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x7400, 0x108, 255.0f32.to_bits(), 1]]
        );
        e.register(PERK_LEVEL_006AABF0, |_, _| eax(5));
        e.call_log = Some(vec![]);
        e.call(0x0078_64e0, &args![0x7400u32, 0x7a00u32]);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x7400, 0x108, 128.0f32.to_bits(), 1]]
        );
    }

    #[test]
    fn display_string_callback_refreshes_the_line() {
        let mut e = engine();
        let this = menu(&mut e);
        e.mem.set_u32(LEVEL_UP_MENU_ME, this);
        e.register(TILE_GET_FLOAT, |_, a| {
            st0(match a[1] {
                0x104 => 90.0,
                0x106 => 15.0,
                _ => 2.0,
            })
        });
        e.register(INT_PAIR_00647B70, |_, a| {
            eax((a[0] as i32).max(a[1] as i32) as u32)
        });
        e.register(MIN_004A8F20, |_, a| {
            eax((a[0] as i32).min(a[1] as i32) as u32)
        });
        e.register(ADD_USER_TRAIT, |_, _| eax(0x99));
        e.call_log = Some(vec![]);
        e.call(0x0078_6550, &args![0x7400u32, 0u32]);
        assert_eq!(calls_to(&e, INT_PAIR_00647B70), vec![vec![107, 0]]);
        assert_eq!(calls_to(&e, MIN_004A8F20), vec![vec![107, 100]]);
        let printed = calls_to(&e, PRINT_FORMATTED);
        assert_eq!(&printed[0][1..], &[0x10, INTEGER_FORMAT_01020764, 100]);
        let set = calls_to(&e, TILE_SET_STRING);
        assert_eq!(set[0][0..2].to_vec(), vec![0x7400, 0x99]);
        assert_eq!(set[0][2], printed[0][0]);
        // The method itself, through its address.
        e.call_log = Some(vec![]);
        e.call(0x0078_66e0, &args![this, 0x7400u32]);
        assert_eq!(calls_to(&e, TILE_SET_STRING).len(), 1);
    }

    #[test]
    fn give_perk_raises_the_rank_of_selected_lines() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x0078_6570, &args![0x7400u32, 0x7a00u32]);
        assert!(calls_to(&e, 0x0000_8498).is_empty());
        e.register(TILE_IS_TRUE, |_, _| eax(1));
        e.call_log = Some(vec![]);
        e.call(0x0078_6570, &args![0x7400u32, 0x7a00u32]);
        let player = player(&e);
        assert_eq!(calls_to(&e, 0x0000_84a0), vec![vec![player, 0x7a00, 0]]);
        // The rank's AL is 2: rank 3.
        assert_eq!(calls_to(&e, 0x0000_8498), vec![vec![player, 0x7a00, 3, 0]]);
    }

    #[test]
    fn perk_compare_orders_alpha_level_then_text() {
        let mut e = engine();
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(first, 0x7400);
        e.mem.set_u32(first + 4, 0x7a00);
        e.mem.set_u32(second, 0x7500);
        e.mem.set_u32(second + 4, 0x7b00);
        e.register(TILE_GET_FLOAT, |_, a| {
            st0(if a[0] == 0x7400 { 128.0 } else { 255.0 })
        });
        assert_eq!(e.call(0x0078_65d0, &args![first, second]).i32(), 1);
        e.register(TILE_GET_FLOAT, |_, a| {
            st0(if a[0] == 0x7400 { 255.0 } else { 128.0 })
        });
        assert_eq!(e.call(0x0078_65d0, &args![first, second]).i32(), -1);
        e.register(TILE_GET_FLOAT, |_, _| st0(255.0));
        e.register(PERK_LEVEL_006AABF0, |_, a| {
            eax(if a[0] == 0x7a00 { 2 } else { 4 })
        });
        assert_eq!(e.call(0x0078_65d0, &args![first, second]).i32(), -1);
        e.register(PERK_LEVEL_006AABF0, |_, a| {
            eax(if a[0] == 0x7a00 { 6 } else { 4 })
        });
        assert_eq!(e.call(0x0078_65d0, &args![first, second]).i32(), 1);
        e.register(PERK_LEVEL_006AABF0, |_, _| eax(4));
        e.register(TILE_GET_STRING, |_, a| eax(a[0] + 1));
        e.register(
            COMPARE_STRINGS_00408B20,
            |_, a| eax(a[0].wrapping_sub(a[1])),
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_65d0, &args![first, second]).i32(), -0x100);
        assert_eq!(
            calls_to(&e, TILE_GET_STRING),
            vec![vec![0x7500, 0xfc4], vec![0x7400, 0xfc4]]
        );
    }

    #[test]
    fn apply_perk_effects_rebuilds_the_effect_list() {
        let mut e = engine();
        let this = menu(&mut e);
        e.register(LIST_BOX_ITEM_DATA_007E1110, |_, _| eax(0x7a00));
        e.register(TILE_GET_FLOAT, |_, _| st0(2.9));
        // Rank 2's entry (type 1) and rank 1's.
        let entry = e.mem.alloc(4);
        let previous = e.mem.alloc(4);
        e.put_vtable(0x0f14_0000, &[0, 0, 0, 0, 0x0000_8a10]);
        e.mem.set_u32(entry, 0x0f14_0000);
        e.register(0x0000_8a10, |_, _| eax(1));
        e.register_double(PERK_ENTRY_005EB900, move |_, a| {
            eax(if a[1] == 2 { entry } else { previous })
        });
        // Each entry's effect list: a node at +0x28 (item, next).
        let effects = e.mem.alloc(0x30);
        let previous_effects = e.mem.alloc(0x30);
        e.mem.set_u32(effects + 0x28, 0x7c00);
        e.mem.set_u32(previous_effects + 0x28, 0x7d00);
        e.register_double(ARRAY_SIZE_0044DDC0, move |_, a| {
            eax(if a[0] == entry {
                effects
            } else {
                previous_effects
            })
        });
        e.register(EFFECT_ACTOR_VALUE_00403EA0, |_, _| eax(0x26));
        e.register(EFFECT_GET_MAGNITUDE, |_, a| {
            eax(if a[0] == 0x7c00 { 10 } else { 4 })
        });
        e.register(ELEMENT_CONSTRUCT, |_, _| Ret::default());
        e.register(EFFECT_SETTING_00825C00, |_, _| eax(0x7e00));
        e.register(FLAG_TEST_00403E60, |_, _| eax(0));
        e.register(FLAG_TEST_00406D70, |_, _| eax(0));
        let buffer = effect_array(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0078_67a0, &args![this, 0x7400u32]);
        assert_eq!(
            calls_to(&e, PERK_ENTRY_005EB900),
            vec![vec![0x7a00, 2], vec![0x7a00, 1]]
        );
        assert_eq!(
            calls_to(&e, ARRAY_CLEAR_008454F0),
            vec![vec![PERK_EFFECT_LIST, 1]]
        );
        assert_eq!(
            calls_to(&e, LIST_BOX_FOR_EACH),
            vec![vec![
                this + SKILLS_LIST,
                PERK_UPDATE_CALLBACK,
                0,
                0x7fff_ffff
            ]]
        );
        // One element: 10 less the previous rank's 4, negated (the line is
        // not selected).
        assert_eq!(e.mem.i32(buffer), -6);
        assert_eq!(e.mem.u32(buffer + 0x10), 0x26);
    }

    #[test]
    fn apply_perk_effects_without_an_entry_only_updates() {
        let mut e = engine();
        let this = menu(&mut e);
        e.register(TILE_GET_FLOAT, |_, _| st0(1.0));
        e.call_log = Some(vec![]);
        e.call(0x0078_67a0, &args![this, 0x7400u32]);
        assert_eq!(calls_to(&e, PERK_ENTRY_005EB900), vec![vec![0, 1]]);
        assert!(calls_to(&e, ARRAY_CLEAR_008454F0).is_empty());
        assert_eq!(calls_to(&e, LIST_BOX_FOR_EACH).len(), 1);
    }

    #[test]
    fn element_negates_unselected_and_subtracts_the_previous_rank() {
        let mut e = engine();
        let data = e.mem.alloc(0x14);
        e.mem.set_u32(data, 10);
        e.mem.set_u32(data + 0x10, 0x26);
        let previous = e.mem.alloc(0x30);
        e.mem.set_u32(previous + 0x28, 0x7d00);
        e.register(EFFECT_ACTOR_VALUE_00403EA0, |_, _| eax(0x26));
        e.register(EFFECT_GET_MAGNITUDE, |_, _| eax(4));
        e.register(EFFECT_SETTING_00825C00, |_, _| eax(0x7e00));
        e.register(FLAG_TEST_00403E60, |_, _| eax(0));
        e.register(FLAG_TEST_00406D70, |_, _| eax(0));
        let buffer = effect_array(&mut e);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, 0x7c00);
        perk_effect_element(&mut e, 0x7400, node, data, previous);
        // 10 - 4, negated (not selected): -6.
        assert_eq!(e.mem.i32(data), -6);
        assert_eq!(e.mem.i32(buffer), -6);
    }

    #[test]
    fn restore_skill_list_state_focuses_the_saved_item() {
        let mut e = engine();
        let list = e.mem.alloc(0x30);
        e.put_vtable(0x0f15_0000, &[0x0000_8b00, 0, 0, 0, 0, 0x0000_8b14]);
        e.mem.set_u32(list, 0x0f15_0000);
        e.register(0x0000_8b00, |_, _| Ret::default());
        e.register(0x0000_8b14, |_, _| eax(0x7f00));
        e.register(SKILL_LIST_NUM_UNFILTERED, |_, _| eax(5));
        e.register(FLOAT_PAIR_0040EBD0, |_, a| {
            st0(f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))))
        });
        e.register(FOCUS_OWNER_004B7210, |_, _| eax(0x7700));
        e.register(SET_CURRENT_FOCUS_TARGET, |_, _| Ret::default());
        e.register(FLAG_004B71D0, |_, _| eax(1));
        e.mem.set_u32(list + 0xc, 0x3000);
        e.mem.set_u32(list + 0x14, 0x3400);
        e.mem.set_u32(list + 0x24, 2.5f32.to_bits());
        e.call_log = Some(vec![]);
        e.call(0x0078_6a50, &args![list, 1u32]);
        assert_eq!(
            calls_to(&e, FLOAT_PAIR_0040EBD0),
            vec![vec![2.5f32.to_bits(), 4.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, 0x0000_8b14), vec![vec![list, 2, 0]]);
        assert_eq!(
            calls_to(&e, SET_CURRENT_FOCUS_TARGET),
            vec![vec![0x7700, 0x7f00, 0xfc3, 1]]
        );
        assert_eq!(calls_to(&e, 0x0000_8b00), vec![vec![list, 0x7f00]]);
        e.mem.set_u32(list + 0x14, 0);
        e.call_log = Some(vec![]);
        e.call(0x0078_6a50, &args![list, 1u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    /// A perk list box with items of the given indexes (tile `0x7000 +
    /// index`), its container enabled.
    fn highlight_list(e: &mut Engine, count: u32) -> Ptr<ListBox> {
        let list = Ptr::<ListBox>::new(e.mem.alloc(0x30));
        e.put_vtable(0x0f16_0000, &[0, 0, 0, 0, 0, 0x0000_8c14]);
        e.mem.set_u32(list.addr(), 0x0f16_0000);
        e.register(0x0000_8c14, |_, a| eax(0x7000 + a[1]));
        e.set(list, ListBox::pContainerRect, Ptr::new(0x3000));
        e.set(list, ListBox::usNextIndex, count as u16);
        let mut node = list.addr() + 4;
        for index in 0..count {
            let item = e.mem.alloc(0xc);
            e.mem.set_u32(item, 0x7000 + index);
            e.mem.set_u32(node, item);
            if index + 1 < count {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        e.register(TILE_GET_FLOAT, |_, a| st0(f64::from(a[0] - 0x7000)));
        list
    }

    #[test]
    fn highlight_target_steps_over_disabled_items() {
        let mut e = engine();
        let list = highlight_list(&mut e, 4);
        // Everything enabled; the highlighted item is index 1.
        e.register(TILE_IS_TRUE, |_, _| eax(1));
        e.set(list, ListBox::pHighlightedItem, Ptr::new(0x7001));
        assert_eq!(e.call(0x0078_7260, &args![list, 2u32]).u32(), 0x7002);
        assert_eq!(e.call(0x0078_7260, &args![list, 1u32]).u32(), 0x7000);
        // Other directions do nothing.
        assert_eq!(e.call(0x0078_7260, &args![list, 3u32]).u32(), 0);
        // Nothing highlighted: down starts at the first item, which is the
        // answer.
        e.set(list, ListBox::pHighlightedItem, Ptr::NULL);
        assert_eq!(e.call(0x0078_7260, &args![list, 2u32]).u32(), 0x7000);
        // Item 3 disabled: down from 2 runs past the end, then back from 2.
        e.register(TILE_IS_TRUE, |_, a| {
            eax(u32::from(a[0] != 0x7003 || a[1] != 0x66))
        });
        e.register(ADD_USER_TRAIT, |_, _| eax(0x66));
        e.set(list, ListBox::pHighlightedItem, Ptr::new(0x7002));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_7260, &args![list, 2u32]).u32(), 0);
        assert_eq!(calls_to(&e, 0x0000_8c14), vec![vec![list.addr(), 2, 0]]);
    }

    #[test]
    fn highlight_target_scrolls_a_page() {
        let mut e = engine();
        let list = highlight_list(&mut e, 10);
        e.register(TILE_IS_TRUE, |_, _| eax(1));
        e.register(TILE_GET_FLOAT, |_, a| {
            st0(match a[1] {
                0xfb0 => 300.0,
                0xf90c => 100.0,
                _ => f64::from(a[0] - 0x7000),
            })
        });
        e.register(POINT_PERCENT_004BD510, |_, a| {
            st0(f64::from(f32::from_bits(a[0])))
        });
        e.set(list, ListBox::pHighlightedItem, Ptr::new(0x7005));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_7260, &args![list, 16u32]).u32(), 0x7008);
        assert_eq!(
            calls_to(&e, POINT_PERCENT_004BD510),
            vec![vec![3.0f32.to_bits(), 1.0f32.to_bits()]]
        );
        assert_eq!(e.call(0x0078_7260, &args![list, 15u32]).u32(), 0x7002);
    }

    #[test]
    fn unselect_all_skips_filtered_items_when_asked() {
        let mut e = engine();
        let list = highlight_list(&mut e, 2);
        let second_item = e.mem.u32(e.mem.u32(list.addr() + 8));
        e.mem.set_u8(second_item + 8, 1);
        e.register(LIST_BOX_BELONGS_TO, |_, _| eax(1));
        e.register(TILE_IS_TRUE, |_, _| eax(1));
        e.call_log = Some(vec![]);
        e.call(0x0078_7850, &args![list, 1u32]);
        // Only the first tile's `_selected` is cleared.
        assert_eq!(
            calls_to(&e, LIST_BOX_BELONGS_TO),
            vec![vec![list.addr(), 0x7000]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0078_7850, &args![list, 0u32]);
        assert_eq!(calls_to(&e, LIST_BOX_BELONGS_TO).len(), 2);
        // 00787c20 is 00787c40 with mode 1.
        e.call_log = Some(vec![]);
        assert!(e.call(0x0078_7c20, &args![list, 0x7001u32]).bool());
        assert_eq!(calls_to(&e, TILE_SET_INT).len(), 1);
        assert_eq!(calls_to(&e, TILE_SET_INT)[0][2], 0);
    }

    #[test]
    fn scalar_deleting_destructors_free_with_bit_0() {
        let mut e = engine();
        let list = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_79c0, &args![list, 1u32]).u32(), list);
        assert_eq!(e.mem.u32(list), PERK_LIST_BOX_VTABLE);
        assert_eq!(calls_to(&e, FREE), vec![vec![list]]);
        let array = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_8360, &args![array, 0u32]).u32(), array);
        assert_eq!(calls_to(&e, ARRAY_DESTROY_00788390), vec![vec![array]]);
        assert!(calls_to(&e, FREE).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x0078_8360, &args![array, 1u32]);
        assert_eq!(calls_to(&e, FREE), vec![vec![array]]);
    }
}
