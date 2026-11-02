//! `fallout/interface/menus/recipemenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0072ba80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: walks `count` elements from `address` and does
/// nothing to them (the loop body is empty in the exe). `this` and
/// `address` are not read.
pub fn fn_0072ba80(_e: &mut Engine, _this: Ptr, _address: u32, _count: u32) {}

/// The constructor body `006b3eb0` (`this` plus two stack words, `RET 8`).
const CONSTRUCTOR_BODY_006B3EB0: u32 = 0x006b_3eb0;
/// The vtable stored at `+0` (RTTI class `BSSimpleArray<TESRecipeCategory*,...>`).
const VTABLE_0072CE30: u32 = 0x0107_0660;

// Translated from 0072ce30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `recipemenu.cpp`): stores the vtable `01070660` (RTTI
/// class `BSSimpleArray<TESRecipeCategory*,...>`), then calls `006b3eb0(this, 0, 0)`.
/// Returns `this`.
pub fn fn_0072ce30(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_0072CE30);
    e.call(CONSTRUCTOR_BODY_006B3EB0, &args![this, 0u32, 0u32]);
    this
}

// Translated from 007273f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether all fourteen tile pointers from +0x28 of the menu (`this`) are
/// set (the recipe menu's tile table; the class layout is not declared
/// here).
pub fn fn_007273f0(e: &mut Engine, this: Ptr) -> bool {
    (0..0xe).all(|i| e.mem.u32(this.addr().wrapping_add(0x28 + 4 * i)) != 0)
}

/// `qsort` (cdecl: base, count, element size, compare).
const QSORT: u32 = 0x00ec_6f20;

// Translated from 00729970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sorts the array of `this` (a `BSSimpleArray`-shaped object: data pointer
/// at +4, count at +8) with `qsort` and the compare function `compare`,
/// four-byte elements.
pub fn fn_00729970(e: &mut Engine, this: Ptr, compare: u32) {
    let data = e.mem.u32(this.addr().wrapping_add(4));
    let count = e.mem.u32(this.addr().wrapping_add(8));
    e.call(QSORT, &args![data, count, 4u32, compare]);
}

// ---------------------------------------------------------------------------
// The `ListBox<TESRecipe *>`, `ListBox<TESRecipeComponent *>` and
// `ListBox<TESConditionItem *>` instances of the list box template. Their code matches the
// `ListBox<int>` instance of `dialogmenu.cpp` (and, for `Sort`, `GetHighlightTarget`,
// `Filter` and `RestoreListState`, the `ListBox<ActorValue::Index>` and
// `ListBox<ItemChange_P>` instances) instruction for instruction: only the vtables, the
// function-local statics and the instance's own methods differ.

use crate::units::fallout_interface::startmenu::ListBox;

/// `ListBoxBase` construction (`007240c0`), the embedded list's constructor (`0096a2d0`)
/// and destructor (`0046ffb0`).
const LIST_BOX_BASE_CONSTRUCT: u32 = 0x0072_40c0;
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// `007a1950` (`thiscall`, `RET 8`): the list box's flag initialisation.
const LIST_BOX_INIT_FLAGS: u32 = 0x007a_1950;
/// `00721130` / `007211c0` (`cdecl`, one word): the list box handed to the menu manager's
/// list and taken out of it.
const LIST_BOX_REGISTER: u32 = 0x0072_1130;
const LIST_BOX_UNREGISTER: u32 = 0x0072_11c0;
/// `MemoryManager` allocate (`00401000`) and free (`00401030`), `cdecl`.
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// The list node helpers: `006815c0` (the node's item slot), `00726070` (the word at +4:
/// the next node), `006ecd40` (sets the next node, `RET 4`), `004702f0` (deletes the
/// node, `RET 4`).
const NODE_SELF: u32 = 0x0068_15c0;
const WORD_AT_FOUR: u32 = 0x0072_6070;
const NODE_SET_NEXT: u32 = 0x006e_cd40;
const NODE_DELETE: u32 = 0x0047_02f0;
/// `004459e0` (`thiscall`, `RET 4`): the item data's deleting call, given 1.
const ITEM_DATA_DELETE: u32 = 0x0044_59e0;
/// The byte that keeps item tiles alive when set (`011d8909`).
const KEEP_ITEM_TILES: u32 = 0x011d_8909;
/// `007242c0` (`thiscall`, `RET 4`): whether the list box has a flag (by bit number);
/// `007242f0` (`thiscall`, `RET 4`): whether a tile is one of the list's items.
const LIST_BOX_HAS_FLAG: u32 = 0x0072_42c0;
const LIST_BOX_BELONGS_TO: u32 = 0x0072_42f0;
/// Tile functions: `Tile::AddUserTrait` (`cdecl`), `Tile::TextToTrait` (`cdecl`), the float
/// setter `00a012d0` (`RET 0xc`), `Tile::Poke` (`00a01290`, `RET 8`), the float getter
/// `00a011b0` (`RET 4`, `ST0`), `Tile::IsTrue` (`00a01230`, `RET 4`), the integer setter
/// `00700320` (`RET 8`), the string setter `00a01350` (`RET 0xc`), `Tile::GetValue`
/// (`00a00f30`, `RET 4`), `Tile::GetValueQ` (`00a00e90`, `RET 4`), the named descendant
/// look-up `00a08b20` (`cdecl`), `Tile::GetChildByName` (`00a03da0`, `RET 4`),
/// `Tile::AddNeedsUpdate` (`00a07530`, `RET 4`), the byte getter `0071aa80` and setter
/// `0071aa60` (`RET 4`), `Tile::GetMenu` (`00a03c90`), `Tile::GetMenuTile` (`00a03d40`).
const ADD_USER_TRAIT: u32 = 0x00a0_0940;
const TEXT_TO_TRAIT: u32 = 0x00a0_1860;
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
const TILE_POKE: u32 = 0x00a0_1290;
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
const TILE_IS_TRUE: u32 = 0x00a0_1230;
const TILE_SET_INT: u32 = 0x0070_0320;
const TILE_SET_STRING: u32 = 0x00a0_1350;
const TILE_GET_VALUE: u32 = 0x00a0_0f30;
const TILE_GET_VALUE_Q: u32 = 0x00a0_0e90;
const TILE_FIND_BY_NAME: u32 = 0x00a0_8b20;
const TILE_GET_CHILD_BY_NAME: u32 = 0x00a0_3da0;
const TILE_ADD_NEEDS_UPDATE: u32 = 0x00a0_7530;
const TILE_FLAG_GETTER_0071AA80: u32 = 0x0071_aa80;
const TILE_FLAG_SETTER_0071AA60: u32 = 0x0071_aa60;
const TILE_GET_MENU: u32 = 0x00a0_3c90;
const TILE_GET_MENU_TILE: u32 = 0x00a0_3d40;
/// `00715c60` (`thiscall`, `RET 8`): sets an integer value of a tile.
const TILE_SET_INTEGER_VALUE: u32 = 0x0071_5c60;
/// `Menu::RenderTemplate` (`00a1ddb0`, `RET 0xc`).
const MENU_RENDER_TEMPLATE: u32 = 0x00a1_ddb0;
/// The item constructor (`007a62e0`, `RET 8`), the sorted insert (`007a7eb0`, `RET 8`),
/// the append (`00905820`, `RET 4`), `InitTilePosition` (`007269d0`, `RET 4`), the item
/// count (`00726c40`), the embedded list's sort (`0083fd60`, `RET 4`).
const ITEM_CONSTRUCT: u32 = 0x007a_62e0;
const LIST_INSERT_SORTED: u32 = 0x007a_7eb0;
const LIST_APPEND: u32 = 0x0090_5820;
const INIT_TILE_POSITION: u32 = 0x0072_69d0;
const LIST_BOX_COUNT_00726C40: u32 = 0x0072_6c40;
const LIST_SORT: u32 = 0x0083_fd60;
/// The tile value's actions (`00a09200`, `00a09130`, `00a09080`, `00a09410`).
const VALUE_CLEAR_ACTIONS: u32 = 0x00a0_9200;
const VALUE_ADD_ACTION_TILE: u32 = 0x00a0_9130;
const VALUE_ADD_ACTION_FLOAT: u32 = 0x00a0_9080;
const VALUE_CALCULATE: u32 = 0x00a0_9410;
/// `_ftol2_sse` (`00ec62c0`), `004e4430` (`cdecl`, a `float`, `ST0`), `00647b70` (`cdecl`,
/// two `int`s), `004bd510` (`cdecl`, two `float`s, `ST0`), `0040ebd0` (`cdecl`, two
/// `float`s, `ST0`).
const FTOL2: u32 = 0x00ec_62c0;
const ROUND_FLOAT: u32 = 0x004e_4430;
const INT_PAIR_00647B70: u32 = 0x0064_7b70;
const SCROLL_ROUND_004BD510: u32 = 0x004b_d510;
const FLOAT_PAIR_0040EBD0: u32 = 0x0040_ebd0;
/// `004b7210` (the interface manager), its `00715860` (`thiscall`, `RET 0xc`: the current
/// focus target) and `004b71d0` (no arguments, a `bool`).
const INTERFACE_MANAGER: u32 = 0x004b_7210;
const SET_CURRENT_FOCUS_TARGET: u32 = 0x0071_5860;
const FLAG_004B71D0: u32 = 0x004b_71d0;
/// `"_number_of_visible_items"`, `"_num_filtered"`, `"_enabled"`, `"_highlight_y"`,
/// `"_selected_height"`, `"lb_highlight_box"`, `"top"`, `"bottom"`, `"left"`, `"right"`,
/// `"_scroll_delta"`, `"_number_of_items"`, `"_current_value"`, `"_SetInCode"`.
const NUMBER_OF_VISIBLE_ITEMS_TEXT: u32 = 0x0106_f860;
const NUM_FILTERED_TEXT: u32 = 0x0106_f8d0;
const ENABLED_TEXT: u32 = 0x0107_0244;
const HIGHLIGHT_Y_TEXT: u32 = 0x0106_f8c0;
const SELECTED_HEIGHT_TEXT: u32 = 0x0106_f8ac;
const HIGHLIGHT_BOX_TEXT: u32 = 0x0106_f898;
const TOP_TEXT: u32 = 0x0106_f894;
const BOTTOM_TEXT: u32 = 0x0106_f88c;
const LEFT_TEXT: u32 = 0x0106_f884;
const RIGHT_TEXT: u32 = 0x0106_f87c;
const SCROLL_DELTA_TEXT: u32 = 0x0106_f90c;
const NUMBER_OF_ITEMS_TEXT: u32 = 0x0106_f8f8;
const CURRENT_VALUE_TEXT: u32 = 0x0107_02d8;
const SET_IN_CODE_TEXT: u32 = 0x0107_0250;
/// `0.0` and `1.0` (`double`s).
const ZERO_DOUBLE: u32 = 0x0101_2060;
const ONE_DOUBLE: u32 = 0x0101_2070;

/// A trait id kept in a function-local static: the guard word (bit `bit` says it has been
/// looked up) and the id.
#[derive(Clone, Copy)]
struct StaticTrait {
    guard: u32,
    bit: u32,
    cache: u32,
    name: u32,
}

/// Looks the static trait id up the first time (the guard bit is set before
/// `Tile::AddUserTrait(name, -1)` runs, as the code does); returns the id read back.
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

type ListBoxMethod = fn(&mut Engine, Ptr<ListBox>);
type ListBoxTest = fn(&mut Engine, Ptr<ListBox>) -> bool;
type ListBoxCount = fn(&mut Engine, Ptr<ListBox>) -> i32;

/// One of this unit's list box instances: its vtable, its own methods (direct calls in
/// this file) and the function-local statics they keep trait ids in.
#[derive(Clone, Copy)]
struct RecipeListBox {
    vtable: u32,
    remove_all: ListBoxMethod,
    synchronize: ListBoxMethod,
    is_enabled: ListBoxTest,
    update_highlight_box_pos: ListBoxMethod,
    set_total_list_height: ListBoxMethod,
    num_unfiltered: ListBoxCount,
    /// `IsEnabled`'s `"_enabled"` id.
    enabled_trait: StaticTrait,
    /// `UpdateHighlightBoxPos`'s `"_highlight_y"` (bit 1) and `"_selected_height"` (bit 2)
    /// ids.
    highlight_y_trait: StaticTrait,
    selected_height_trait: StaticTrait,
    /// `NumUnfiltered`'s `"_num_filtered"` id.
    num_filtered_trait: StaticTrait,
}

/// The statics of an instance from its three guard words (each id just below its guard;
/// `UpdateHighlightBoxPos`'s `"_highlight_y"` 4 below, `"_selected_height"` 8 below), as
/// the methods' disassembly uses them.
const fn statics([enabled, highlight, num_filtered]: [u32; 3]) -> [StaticTrait; 4] {
    [
        StaticTrait {
            guard: enabled,
            bit: 1,
            cache: enabled - 4,
            name: ENABLED_TEXT,
        },
        StaticTrait {
            guard: highlight,
            bit: 1,
            cache: highlight - 4,
            name: HIGHLIGHT_Y_TEXT,
        },
        StaticTrait {
            guard: highlight,
            bit: 2,
            cache: highlight - 8,
            name: SELECTED_HEIGHT_TEXT,
        },
        StaticTrait {
            guard: num_filtered,
            bit: 1,
            cache: num_filtered - 4,
            name: NUM_FILTERED_TEXT,
        },
    ]
}

const RECIPE_STATICS: [StaticTrait; 4] = statics([0x011d_8f58, 0x011d_8f64, 0x011d_8f50]);
const COMPONENT_STATICS: [StaticTrait; 4] = statics([0x011d_8f6c, 0x011d_8f78, 0x011d_8f94]);
const CONDITION_STATICS: [StaticTrait; 4] = statics([0x011d_8f80, 0x011d_8f8c, 0x011d_8f9c]);

/// `ListBox<TESRecipe *>` (vtable `010705d4`).
const RECIPE_LIST_BOX: RecipeListBox = RecipeListBox {
    vtable: 0x0107_05d4,
    remove_all: list_box_tes_recipe_remove_all,
    synchronize: list_box_tes_recipe_synchronize,
    is_enabled: list_box_tes_recipe_is_enabled,
    update_highlight_box_pos: list_box_tes_recipe_update_highlight_box_pos,
    set_total_list_height: list_box_tes_recipe_set_total_list_height,
    num_unfiltered: fn_0072bab0,
    enabled_trait: RECIPE_STATICS[0],
    highlight_y_trait: RECIPE_STATICS[1],
    selected_height_trait: RECIPE_STATICS[2],
    num_filtered_trait: RECIPE_STATICS[3],
};
/// `ListBox<TESRecipeComponent *>` (vtable `01070610`).
const COMPONENT_LIST_BOX: RecipeListBox = RecipeListBox {
    vtable: 0x0107_0610,
    remove_all: list_box_tes_recipe_component_remove_all,
    synchronize: list_box_tes_recipe_component_synchronize,
    is_enabled: list_box_tes_recipe_component_is_enabled,
    update_highlight_box_pos: list_box_tes_recipe_component_update_highlight_box_pos,
    set_total_list_height: list_box_tes_recipe_component_set_total_list_height,
    num_unfiltered: list_box_tes_recipe_component_num_unfiltered,
    enabled_trait: COMPONENT_STATICS[0],
    highlight_y_trait: COMPONENT_STATICS[1],
    selected_height_trait: COMPONENT_STATICS[2],
    num_filtered_trait: COMPONENT_STATICS[3],
};
/// `ListBox<TESConditionItem *>` (vtable `01070638`).
const CONDITION_LIST_BOX: RecipeListBox = RecipeListBox {
    vtable: 0x0107_0638,
    remove_all: list_box_tes_condition_item_remove_all,
    synchronize: list_box_tes_condition_item_synchronize,
    is_enabled: list_box_tes_condition_item_is_enabled,
    update_highlight_box_pos: list_box_tes_condition_item_update_highlight_box_pos,
    set_total_list_height: list_box_tes_condition_item_set_total_list_height,
    num_unfiltered: list_box_tes_condition_item_num_unfiltered,
    enabled_trait: CONDITION_STATICS[0],
    highlight_y_trait: CONDITION_STATICS[1],
    selected_height_trait: CONDITION_STATICS[2],
    num_filtered_trait: CONDITION_STATICS[3],
};

/// `ListBox<TESRecipe *>::Filter`'s `"_num_filtered"` id.
const RECIPE_FILTER_TRAIT: StaticTrait = StaticTrait {
    guard: 0x011d_8f48,
    bit: 1,
    cache: 0x011d_8f44,
    name: NUM_FILTERED_TEXT,
};

/// The item word of a list node, reached through `006815c0` as the code does.
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let node = e.call(NODE_SELF, &args![node]).u32();
    e.mem.u32(node)
}

/// The next node (`00726070`, the word at +4).
fn node_next(e: &mut Engine, node: u32) -> u32 {
    e.call(WORD_AT_FOUR, &args![node]).u32()
}

/// The first node of the embedded list (`this + 4`; the code guards a null `this`).
fn first_node(this: Ptr<ListBox>) -> u32 {
    if this.is_null() {
        0
    } else {
        this.addr() + 4
    }
}

/// `FCOMP; FNSTSW AX; TEST AH,0x41; JNZ`: the code goes on only when the value is strictly
/// above the other (not less, equal or unordered).
fn above(a: f64, b: f64) -> bool {
    a.partial_cmp(&b) == Some(std::cmp::Ordering::Greater)
}

/// An item tile's index trait (`0xfac`) as the code reads it: the float getter, then
/// `_ftol2`.
fn tile_index(e: &mut Engine, tile: u32) -> i32 {
    let value = e.call(TILE_GET_FLOAT, &args![tile, 0xfacu32]).f64();
    e.call(FTOL2, &args![value]).i32()
}

/// `Tile::IsTrue(tile, Tile::AddUserTrait("_enabled", -1))`.
fn tile_enabled(e: &mut Engine, tile: u32) -> bool {
    let enabled = e
        .call(ADD_USER_TRAIT, &args![ENABLED_TEXT, 0xffff_ffffu32])
        .u32();
    e.call(TILE_IS_TRUE, &args![tile, enabled]).bool()
}

/// The constructor: the `ListBoxBase` base (`007240c0`), the embedded list (+4,
/// `0096a2d0`), the vtable, the fields cleared, `007a1950(this, 0, 1)` and the list box
/// handed to `00721130`. Returns `this`.
fn list_box_construct(e: &mut Engine, this: Ptr<ListBox>, kind: RecipeListBox) -> Ptr<ListBox> {
    e.call(LIST_BOX_BASE_CONSTRUCT, &args![this]);
    e.call(LIST_CONSTRUCT, &args![this.addr() + 4]);
    e.mem.set_u32(this.addr(), kind.vtable);
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

/// The destructor: the vtable, `RemoveAll`, the list box handed to `007211c0`, then the
/// embedded list destroyed (`0046ffb0`).
fn list_box_destruct(e: &mut Engine, this: Ptr<ListBox>, kind: RecipeListBox) {
    e.mem.set_u32(this.addr(), kind.vtable);
    (kind.remove_all)(e, this);
    e.call(LIST_BOX_UNREGISTER, &args![this]);
    e.call(LIST_DESTRUCT, &args![this.addr() + 4]);
}

/// The deleting destructor: the destructor (`destruct`), then the object freed when bit 0
/// of `flags` is set. Returns `this`.
fn list_box_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ListBox>,
    flags: u32,
    destruct: ListBoxMethod,
) -> Ptr<ListBox> {
    destruct(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

/// `AddItem`: with a container and a template name (`template`, else `pTemplateName`),
/// renders the template in the container's menu (`Menu::RenderTemplate`), gives the new
/// tile trait `0xfaa` -1 when it has none and the text `text` (trait `0xfc4`) when there
/// is one, and adds an item (tile, `data`): sorted with `compare` (then `Synchronize` when
/// flag 0 is set) or appended (then, with flag 0, `InitTilePosition` and
/// `SetTotalListHeight`, and the tile gets the next index as trait `0xfac`). When the list
/// then counts 1 (`00726c40`) and the container's `"_number_of_visible_items"` is above
/// `0.0`, the container's height value (`0xfb0`) is rebuilt from the tile's height times
/// that number. Returns the tile, or 0 without a container or template.
fn list_box_add_item(
    e: &mut Engine,
    this: Ptr<ListBox>,
    kind: RecipeListBox,
    [data, text, compare, template]: [u32; 4],
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
    // The item is built with the address of the `data` argument's stack slot.
    let build_item = |e: &mut Engine| {
        e.with_stack(4, |e, data_slot| {
            e.mem.set_u32(data_slot.addr(), data);
            let block = e.call(ALLOCATE, &args![0xcu32]).u32();
            if block != 0 {
                e.call(ITEM_CONSTRUCT, &args![block, tile, data_slot]).u32()
            } else {
                0
            }
        })
    };
    if compare != 0 {
        let item = build_item(e);
        e.call(LIST_INSERT_SORTED, &args![this.addr() + 4, item, compare]);
        if e.call(LIST_BOX_HAS_FLAG, &args![this, 0u32]).bool() {
            (kind.synchronize)(e, this);
        }
    } else {
        let item = build_item(e);
        e.with_stack(4, |e, item_slot| {
            e.mem.set_u32(item_slot.addr(), item);
            e.call(LIST_APPEND, &args![this.addr() + 4, item_slot]);
        });
        if e.call(LIST_BOX_HAS_FLAG, &args![this, 0u32]).bool() {
            e.call(INIT_TILE_POSITION, &args![this, tile]);
            (kind.set_total_list_height)(e, this);
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

/// Deletes the `ListBoxItem` (`pTile` +0, `data` +4) that `item` reads each time: the tile
/// through its deleting destructor (vtable slot 0, flag 1) unless `KEEP_ITEM_TILES` is set,
/// the data (`004459e0` with 1) when flag bit 1 is set, then the item freed.
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

/// `RemoveAll`: while the second node (+8) and its item are non-null, deletes the item,
/// clears the node's next pointer (`006ecd40` with 0), deletes the node (`004702f0` with
/// 1) and makes the following node the second; then deletes the first item (+4). Clears
/// the first item, `usNextIndex` and `fTotalListHeight`, sets the container's
/// `"_num_filtered"` to `0.0`, then `SetTotalListHeight`, clears `pHighlightedItem` and
/// `UpdateHighlightBoxPos`.
fn list_box_remove_all(e: &mut Engine, this: Ptr<ListBox>, kind: RecipeListBox) {
    let base = this.addr();
    loop {
        let second = e.mem.u32(base + 8);
        if second == 0 || node_item(e, second) == 0 {
            break;
        }
        let second = e.mem.u32(base + 8);
        let next = node_next(e, second);
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
    (kind.set_total_list_height)(e, this);
    e.set(this, ListBox::pHighlightedItem, Ptr::NULL);
    (kind.update_highlight_box_pos)(e, this);
}

/// `Sort`: sorts the embedded list (`0083fd60` on +4 with `compare`), then `Synchronize`.
fn list_box_sort(e: &mut Engine, this: Ptr<ListBox>, kind: RecipeListBox, compare: u32) {
    e.call(LIST_SORT, &args![this.addr() + 4, compare]);
    (kind.synchronize)(e, this);
}

/// `Highlight`: when the list is enabled and `tile` is null, or is one of its items
/// (`007242f0`) and `"_enabled"`, makes it `pHighlightedItem`, moves the highlight box and
/// returns true; otherwise false.
fn list_box_highlight(e: &mut Engine, this: Ptr<ListBox>, kind: RecipeListBox, tile: u32) -> bool {
    if !(kind.is_enabled)(e, this) {
        return false;
    }
    if tile != 0 {
        if !e.call(LIST_BOX_BELONGS_TO, &args![this, tile]).bool() {
            return false;
        }
        if !tile_enabled(e, tile) {
            return false;
        }
    }
    e.set(this, ListBox::pHighlightedItem, Ptr::new(tile));
    (kind.update_highlight_box_pos)(e, this);
    true
}

/// The step of a page scroll in `GetHighlightTarget`: 0 without a container, otherwise the
/// container's trait `0xfb0` (negated for a page up, kept as a `double`) divided by its
/// `"_scroll_delta"` trait (`FDIVR`), stored as a `float`, passed with `1.0` to `004bd510`
/// and converted with `_ftol2`.
fn page_step(e: &mut Engine, this: Ptr<ListBox>, negate: bool) -> i32 {
    let container = e.get(this, ListBox::pContainerRect);
    if container.is_null() {
        return 0;
    }
    let extent = e.call(TILE_GET_FLOAT, &args![container, 0xfb0u32]).f64();
    let numerator = if negate { -extent } else { extent };
    let scroll_delta = e.call(TEXT_TO_TRAIT, &args![SCROLL_DELTA_TEXT]).u32();
    let container = e.get(this, ListBox::pContainerRect);
    let delta = e
        .call(TILE_GET_FLOAT, &args![container, scroll_delta])
        .f64();
    let step = x87_store32(x87_div(numerator, delta));
    let rounded = e
        .call(
            SCROLL_ROUND_004BD510,
            &args![x87_quiet_bits(step), 1.0f32.to_bits()],
        )
        .f64();
    e.call(FTOL2, &args![rounded]).i32()
}

/// `GetHighlightTarget`: the tile a move in `direction` highlights (1 up, 2 down, 0xF page
/// up, 0x10 page down; anything else does not move), or 0 when there is none or it is the
/// one already highlighted. From the highlighted item's index plus the step (clamped to
/// the list), or the first or last index, it looks for the item whose index trait matches;
/// a disabled one is skipped in the direction of the move, and past either end the search
/// turns back through the list's tile look-up (vtable +0x14 with 0).
fn list_box_get_highlight_target(
    e: &mut Engine,
    this: Ptr<ListBox>,
    kind: RecipeListBox,
    direction: u32,
) -> u32 {
    let mut target = 0u32;
    let step = match direction {
        1 => -1,
        2 => 1,
        0xf => page_step(e, this, true),
        0x10 => page_step(e, this, false),
        _ => 0,
    };
    if (kind.is_enabled)(e, this) && step != 0 && e.get(this, ListBox::usNextIndex) != 0 {
        let last = i32::from(e.get(this, ListBox::usNextIndex)) - 1;
        let mut index = if !e.get(this, ListBox::pHighlightedItem).is_null() {
            let highlighted = e.get(this, ListBox::pHighlightedItem).addr();
            let moved = tile_index(e, highlighted).wrapping_add(step);
            if moved < 0 {
                0
            } else if moved > last {
                last
            } else {
                moved
            }
        } else if step > 0 {
            0
        } else {
            last
        };
        'search: loop {
            let mut node = first_node(this);
            while target == 0 && node != 0 && node_item(e, node) != 0 {
                let item = node_item(e, node);
                let tile = e.mem.u32(item);
                let value = e.call(TILE_GET_FLOAT, &args![tile, 0xfacu32]).f64();
                // `FILD index; FUCOMPP; TEST AH,0x44; JP`: equal and ordered.
                if f64::from(index) == value {
                    let item = node_item(e, node);
                    target = e.mem.u32(item);
                }
                node = node_next(e, node);
            }
            if target == 0 || tile_enabled(e, target) {
                break;
            }
            target = 0;
            if step > 0 {
                index = index.wrapping_add(1);
                if index > last {
                    // Past the end: back from the one before the last.
                    index = last - 1;
                    loop {
                        if target != 0 && tile_enabled(e, target) {
                            break 'search;
                        }
                        if index < 0 {
                            break 'search;
                        }
                        target = e.vcall(this.addr(), 0x14, &args![index, 0u32]).u32();
                        index -= 1;
                    }
                }
            } else if step < 0 {
                index = index.wrapping_sub(1);
                if index < 0 {
                    // Before the start: on from index 1.
                    index = 1;
                    loop {
                        if target != 0 && tile_enabled(e, target) {
                            break 'search;
                        }
                        if index > last {
                            break 'search;
                        }
                        target = e.vcall(this.addr(), 0x14, &args![index, 0u32]).u32();
                        index += 1;
                    }
                }
            }
        }
    }
    if target != e.get(this, ListBox::pHighlightedItem).addr() {
        target
    } else {
        0
    }
}

/// `Synchronize`: clears `usNextIndex` and `fTotalListHeight`, then for every node with an
/// item whose filtered byte (+8) is clear gives the item's tile the next index (trait
/// `0xfac`) and its position (`InitTilePosition`); then `SetTotalListHeight` and
/// `UpdateHighlightBoxPos`.
fn list_box_synchronize(e: &mut Engine, this: Ptr<ListBox>, kind: RecipeListBox) {
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
        node = node_next(e, node);
    }
    (kind.set_total_list_height)(e, this);
    (kind.update_highlight_box_pos)(e, this);
}

/// `IsEnabled`: the container exists and its `"_enabled"` trait (a function-local static
/// id) is true.
fn list_box_is_enabled(e: &mut Engine, this: Ptr<ListBox>, kind: RecipeListBox) -> bool {
    static_trait(e, kind.enabled_trait);
    if e.get(this, ListBox::pContainerRect).is_null() {
        return false;
    }
    let enabled = e.mem.u32(kind.enabled_trait.cache);
    let container = e.get(this, ListBox::pContainerRect);
    e.call(TILE_IS_TRUE, &args![container, enabled]).bool()
}

/// The tile's byte (`0071aa60`) set to `flag` and the tile queued for an update with
/// flags 8.
fn mark_tile(e: &mut Engine, tile: u32, flag: u32) {
    e.call(TILE_FLAG_SETTER_0071AA60, &args![tile, flag]);
    e.call(TILE_ADD_NEEDS_UPDATE, &args![tile, 8u32]);
}

/// `UpdateHighlightBoxPos`: looks the `"_highlight_y"` and `"_selected_height"` trait ids
/// up (statics); with a container and no highlighted item, sets them to -1 and 0 on the
/// container; with one, makes both values follow the highlighted tile's traits `0xfa2` and
/// `0xfb0` (action `0x7d0`, then calculated) and gives the container's
/// `"lb_highlight_box"` tile, and its `"top"`, `"bottom"`, `"left"` and `"right"` children
/// when `"top"` exists (the other three are not checked for null), the highlighted tile's
/// byte (`0071aa80`, as 1 or 0) and an update.
fn list_box_update_highlight_box_pos(e: &mut Engine, this: Ptr<ListBox>, kind: RecipeListBox) {
    static_trait(e, kind.highlight_y_trait);
    static_trait(e, kind.selected_height_trait);
    if e.get(this, ListBox::pContainerRect).is_null() {
        return;
    }
    if e.get(this, ListBox::pHighlightedItem).is_null() {
        let y_trait = e.mem.u32(kind.highlight_y_trait.cache);
        let container = e.get(this, ListBox::pContainerRect);
        e.call(TILE_SET_INT, &args![container, y_trait, 0xffff_ffffu32]);
        let height_trait = e.mem.u32(kind.selected_height_trait.cache);
        let container = e.get(this, ListBox::pContainerRect);
        e.call(TILE_SET_INT, &args![container, height_trait, 0u32]);
        return;
    }
    let y_trait = e.mem.u32(kind.highlight_y_trait.cache);
    let container = e.get(this, ListBox::pContainerRect);
    let y_value = e.call(TILE_GET_VALUE, &args![container, y_trait]).u32();
    let height_trait = e.mem.u32(kind.selected_height_trait.cache);
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

/// `SetTotalListHeight`: with a container, its visible height (trait `0xfb0`, stored as a
/// `float`) and the total list height decide the number of scroll steps: 1, unless both
/// are above `0.0` and the visible height is below the total, then
/// `00647b70(_ftol2(004e4430(unfiltered * (total - visible) / total)) + 1, 2)` (the ratio
/// stored as a `float`). With both above `0.0` and more than one step, the container's
/// `"_scroll_delta"` becomes `(total - visible) / (steps - 1)`; with a scroll bar its
/// `"_number_of_items"` becomes the number of steps.
fn list_box_set_total_list_height(e: &mut Engine, this: Ptr<ListBox>, kind: RecipeListBox) {
    let container = e.get(this, ListBox::pContainerRect);
    if container.is_null() {
        return;
    }
    let visible = x87_store32(e.call(TILE_GET_FLOAT, &args![container, 0xfb0u32]).f64());
    let mut steps = 1i32;
    let zero: f64 = e.global(ZERO_DOUBLE);
    let total_bits = e.mem.u32(this.addr() + 0x20);
    if above(x87_load32(total_bits), zero) && above(x87_load32(visible), zero) {
        // `FLD total; FLD visible; FCOMPP; TEST AH,5; JP`: on only when the visible height
        // is below the total (ordered).
        let total_bits = e.mem.u32(this.addr() + 0x20);
        if x87_load32(visible) < x87_load32(total_bits) {
            let count = (kind.num_unfiltered)(e, this) as u32;
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

/// `NumUnfiltered`: `00726c40(this)` minus the container's `"_num_filtered"` (a
/// function-local static id; the float truncated by `FISTP qword`).
fn list_box_num_unfiltered(e: &mut Engine, this: Ptr<ListBox>, kind: RecipeListBox) -> i32 {
    static_trait(e, kind.num_filtered_trait);
    let count = e.call(LIST_BOX_COUNT_00726C40, &args![this]).u32();
    let num_filtered = e.mem.u32(kind.num_filtered_trait.cache);
    let container = e.get(this, ListBox::pContainerRect);
    let value = e
        .call(TILE_GET_FLOAT, &args![container, num_filtered])
        .f64();
    let filtered = x87_fistp64_chop(X87Ext::from(value)) as u32;
    count.wrapping_sub(filtered) as i32
}

// Translated from 007299a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007299a0 - the C++ exception frame (FS:[0] chain, unwind state 0 after the embedded list is built) is not translated.
/// The `ListBox<TESRecipe *>` constructor (unnamed in the engine map; vtable `010705d4`):
/// `list_box_construct`. Returns `this`.
pub fn fn_007299a0(e: &mut Engine, this: Ptr<ListBox>) -> Ptr<ListBox> {
    list_box_construct(e, this, RECIPE_LIST_BOX)
}

// Translated from 00729a70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00729a70 - the C++ exception frame (FS:[0] chain, unwind state 0 around the body) is not translated.
/// The `ListBox<TESRecipe *>` destructor (unnamed in the engine map): `list_box_destruct`.
pub fn fn_00729a70(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_destruct(e, this, RECIPE_LIST_BOX);
}

// Translated from 00729ae0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00729ae0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the item allocations) is not translated.
/// `ListBox<TESRecipe *>::AddItem` (unnamed in the engine map; `RET 0x10`: data, text,
/// compare function, template name): `list_box_add_item`. Returns the tile.
pub fn fn_00729ae0(
    e: &mut Engine,
    this: Ptr<ListBox>,
    data: u32,
    text: u32,
    compare: u32,
    template: u32,
) -> u32 {
    list_box_add_item(e, this, RECIPE_LIST_BOX, [data, text, compare, template])
}

// Translated from 00729d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESRecipe_P>::RemoveAll` (engine map): `list_box_remove_all`.
pub fn list_box_tes_recipe_remove_all(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_remove_all(e, this, RECIPE_LIST_BOX);
}

// Translated from 00729fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESRecipe_P>::Sort` (engine map): `list_box_sort`.
pub fn list_box_tes_recipe_sort(e: &mut Engine, this: Ptr<ListBox>, compare: u32) {
    list_box_sort(e, this, RECIPE_LIST_BOX, compare);
}

// Translated from 00729fe0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00729fe0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<TESRecipe_P>::Filter` (engine map; `RET 4`): runs `filter` (`cdecl`, the
/// item's data, a `bool`) over every item. An item for which it returns true is marked
/// filtered out (its byte at +8 is 1, its index trait `0xfac` becomes -1, the count of
/// filtered items grows); the others are numbered again from 0 and positioned
/// (`InitTilePosition`). A filtered-out highlighted item is dropped and its old index
/// remembered; the item that takes that index is highlighted, else the list's tile at that
/// index (vtable +0x14 with 0), else `fn_0072ba50`'s tile. The container's
/// `"_num_filtered"` (a function-local static id) gets the count (`00715c60`), then
/// `SetTotalListHeight` and `UpdateHighlightBoxPos`. A null `filter` filters nothing out.
pub fn list_box_tes_recipe_filter(e: &mut Engine, this: Ptr<ListBox>, filter: u32) {
    let mut filtered_count = 0i32;
    let mut remembered = -1i32;
    static_trait(e, RECIPE_FILTER_TRAIT);
    e.set(this, ListBox::usNextIndex, 0);
    e.set(this, ListBox::fTotalListHeight, 0.0);
    let mut node = first_node(this);
    while node != 0 && node_item(e, node) != 0 {
        let filtered = filter != 0 && {
            let item = node_item(e, node);
            let data = e.mem.u32(item + 4);
            e.call(filter, &args![data]).bool()
        };
        let item = node_item(e, node);
        e.mem.set_u8(item + 8, u8::from(filtered));
        let item = node_item(e, node);
        if e.mem.u8(item + 8) == 0 {
            if remembered >= 0 && i32::from(e.get(this, ListBox::usNextIndex)) == remembered {
                let item = node_item(e, node);
                let tile = e.mem.u32(item);
                e.set(this, ListBox::pHighlightedItem, Ptr::new(tile));
            }
            let index = e.get(this, ListBox::usNextIndex);
            e.set(this, ListBox::usNextIndex, index.wrapping_add(1));
            let item = node_item(e, node);
            let tile = e.mem.u32(item);
            e.call(TILE_SET_INT, &args![tile, 0xfacu32, u32::from(index)]);
            let item = node_item(e, node);
            let tile = e.mem.u32(item);
            e.call(INIT_TILE_POSITION, &args![this, tile]);
        } else {
            let item = node_item(e, node);
            let tile = e.mem.u32(item);
            if tile == e.get(this, ListBox::pHighlightedItem).addr() {
                let item = node_item(e, node);
                let tile = e.mem.u32(item);
                remembered = tile_index(e, tile);
                e.set(this, ListBox::pHighlightedItem, Ptr::NULL);
            }
            let item = node_item(e, node);
            let tile = e.mem.u32(item);
            e.call(TILE_SET_INT, &args![tile, 0xfacu32, 0xffff_ffffu32]);
            filtered_count += 1;
        }
        node = node_next(e, node);
    }
    let num_filtered = e.mem.u32(RECIPE_FILTER_TRAIT.cache);
    let container = e.get(this, ListBox::pContainerRect);
    e.call(
        TILE_SET_INTEGER_VALUE,
        &args![container, num_filtered, filtered_count],
    );
    list_box_tes_recipe_set_total_list_height(e, this);
    if e.get(this, ListBox::pHighlightedItem).is_null() && remembered >= 0 {
        let tile = e.vcall(this.addr(), 0x14, &args![remembered, 0u32]).u32();
        e.set(this, ListBox::pHighlightedItem, Ptr::new(tile));
    }
    if e.get(this, ListBox::pHighlightedItem).is_null() && remembered >= 0 {
        let tile = fn_0072ba50(e, this);
        e.set(this, ListBox::pHighlightedItem, Ptr::new(tile));
    }
    list_box_tes_recipe_update_highlight_box_pos(e, this);
}

// Translated from 0072a250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESRecipe_P>::Highlight` (engine map): `list_box_highlight`.
pub fn list_box_tes_recipe_highlight(e: &mut Engine, this: Ptr<ListBox>, tile: u32) -> bool {
    list_box_highlight(e, this, RECIPE_LIST_BOX, tile)
}

// Translated from 0072a2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESRecipe_P>::GetHighlightTarget` (engine map): `list_box_get_highlight_target`
/// (the switch's byte map `0072a650` and table `0072a63c` give 1, 2, 0xF and 0x10).
pub fn list_box_tes_recipe_get_highlight_target(
    e: &mut Engine,
    this: Ptr<ListBox>,
    direction: u32,
) -> u32 {
    list_box_get_highlight_target(e, this, RECIPE_LIST_BOX, direction)
}

// Translated from 0072a660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESRecipe_P>::RestoreListState` (engine map; `RET 4`): with a container and a
/// scroll bar, gives the scroll bar `"_current_value"` (the saved scroll bar value, flag 0)
/// and pokes `"_SetInCode"` with `1.0`; takes the tile at index `FISTP(0040ebd0(
/// fSavedHighlightIndex, NumUnfiltered - 1.0))` (vtable +0x14 with 0), makes it the
/// interface's focus target (`00715860` with trait `0xfc3` and `focus`) and highlights it
/// (vtable +0). Unless `004b71d0` says so, also scrolls the container's menu to it (the
/// menu's vtable +0x14 with the tile's index trait `0xfaa` and the tile, when there is a
/// tile) and highlights nothing.
pub fn list_box_tes_recipe_restore_list_state(e: &mut Engine, this: Ptr<ListBox>, focus: u8) {
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
    let count = fn_0072bab0(e, this) as u32;
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
    let owner = e.call(INTERFACE_MANAGER, &args![]).u32();
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

// Translated from 0072a7c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0072a7c0 - the C++ exception frame (FS:[0] chain, unwind state 0 after the embedded list is built) is not translated.
/// The `ListBox<TESRecipeComponent *>` constructor (unnamed in the engine map; vtable
/// `01070610`): `list_box_construct`. Returns `this`.
pub fn fn_0072a7c0(e: &mut Engine, this: Ptr<ListBox>) -> Ptr<ListBox> {
    list_box_construct(e, this, COMPONENT_LIST_BOX)
}

// Translated from 0072a890 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0072a890 - the C++ exception frame (FS:[0] chain, unwind state 0 around the body) is not translated.
/// The `ListBox<TESRecipeComponent *>` destructor (unnamed in the engine map):
/// `list_box_destruct`.
pub fn fn_0072a890(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_destruct(e, this, COMPONENT_LIST_BOX);
}

// Translated from 0072a900 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0072a900 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the item allocations) is not translated.
/// `ListBox<TESRecipeComponent_P>::AddItem` (engine map; `RET 0x10`): `list_box_add_item`.
pub fn list_box_tes_recipe_component_add_item(
    e: &mut Engine,
    this: Ptr<ListBox>,
    data: u32,
    text: u32,
    compare: u32,
    template: u32,
) -> u32 {
    list_box_add_item(e, this, COMPONENT_LIST_BOX, [data, text, compare, template])
}

// Translated from 0072aba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESRecipeComponent_P>::RemoveAll` (engine map): `list_box_remove_all`.
pub fn list_box_tes_recipe_component_remove_all(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_remove_all(e, this, COMPONENT_LIST_BOX);
}

// Translated from 0072add0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESRecipeComponent_P>::Sort` (engine map): `list_box_sort`.
pub fn list_box_tes_recipe_component_sort(e: &mut Engine, this: Ptr<ListBox>, compare: u32) {
    list_box_sort(e, this, COMPONENT_LIST_BOX, compare);
}

// Translated from 0072ae00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESRecipeComponent_P>::Highlight` (engine map): `list_box_highlight`.
pub fn list_box_tes_recipe_component_highlight(
    e: &mut Engine,
    this: Ptr<ListBox>,
    tile: u32,
) -> bool {
    list_box_highlight(e, this, COMPONENT_LIST_BOX, tile)
}

// Translated from 0072ae70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESRecipeComponent_P>::GetHighlightTarget` (engine map):
/// `list_box_get_highlight_target`.
pub fn list_box_tes_recipe_component_get_highlight_target(
    e: &mut Engine,
    this: Ptr<ListBox>,
    direction: u32,
) -> u32 {
    list_box_get_highlight_target(e, this, COMPONENT_LIST_BOX, direction)
}

// Translated from 0072b210 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0072b210 - the C++ exception frame (FS:[0] chain, unwind state 0 after the embedded list is built) is not translated.
/// The `ListBox<TESConditionItem *>` constructor (unnamed in the engine map; vtable
/// `01070638`): `list_box_construct`. Returns `this`.
pub fn fn_0072b210(e: &mut Engine, this: Ptr<ListBox>) -> Ptr<ListBox> {
    list_box_construct(e, this, CONDITION_LIST_BOX)
}

// Translated from 0072b2e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0072b2e0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the body) is not translated.
/// The `ListBox<TESConditionItem *>` destructor (unnamed in the engine map):
/// `list_box_destruct`.
pub fn fn_0072b2e0(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_destruct(e, this, CONDITION_LIST_BOX);
}

// Translated from 0072b350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESConditionItem_P>::RemoveAll` (engine map): `list_box_remove_all`.
pub fn list_box_tes_condition_item_remove_all(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_remove_all(e, this, CONDITION_LIST_BOX);
}

// Translated from 0072b580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESConditionItem_P>::Sort` (engine map): `list_box_sort`.
pub fn list_box_tes_condition_item_sort(e: &mut Engine, this: Ptr<ListBox>, compare: u32) {
    list_box_sort(e, this, CONDITION_LIST_BOX, compare);
}

// Translated from 0072b5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESConditionItem_P>::Highlight` (engine map): `list_box_highlight`.
pub fn list_box_tes_condition_item_highlight(
    e: &mut Engine,
    this: Ptr<ListBox>,
    tile: u32,
) -> bool {
    list_box_highlight(e, this, CONDITION_LIST_BOX, tile)
}

// Translated from 0072b620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESConditionItem_P>::GetHighlightTarget` (engine map):
/// `list_box_get_highlight_target`.
pub fn list_box_tes_condition_item_get_highlight_target(
    e: &mut Engine,
    this: Ptr<ListBox>,
    direction: u32,
) -> u32 {
    list_box_get_highlight_target(e, this, CONDITION_LIST_BOX, direction)
}

// Translated from 0072b9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESRecipe_P>::_scalar_deleting_destructor_` (engine map; `RET 4`):
/// `list_box_deleting_destructor` with `fn_00729a70`. Returns `this`.
pub fn list_box_tes_recipe_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ListBox>,
    flags: u32,
) -> Ptr<ListBox> {
    list_box_deleting_destructor(e, this, flags, fn_00729a70)
}

// Translated from 0072b9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESRecipeComponent_P>::_scalar_deleting_destructor_` (engine map; `RET 4`):
/// `list_box_deleting_destructor` with `fn_0072a890`. Returns `this`.
pub fn list_box_tes_recipe_component_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ListBox>,
    flags: u32,
) -> Ptr<ListBox> {
    list_box_deleting_destructor(e, this, flags, fn_0072a890)
}

// Translated from 0072ba20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESConditionItem_P>::_scalar_deleting_destructor_` (engine map; `RET 4`):
/// `list_box_deleting_destructor` with `fn_0072b2e0`. Returns `this`.
pub fn list_box_tes_condition_item_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ListBox>,
    flags: u32,
) -> Ptr<ListBox> {
    list_box_deleting_destructor(e, this, flags, fn_0072b2e0)
}

// Translated from 0072ba50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Named `ListBox<TESRecipe_P>::NumUnfiltered` in the engine map, but the body is another
/// method (folded name): the list's tile at index `NumUnfiltered - 1` (`fn_0072bab0`, then
/// vtable +0x14 with 0), its `EAX` returned.
pub fn fn_0072ba50(e: &mut Engine, this: Ptr<ListBox>) -> u32 {
    let last = fn_0072bab0(e, this).wrapping_sub(1);
    e.vcall(this.addr(), 0x14, &args![last, 0u32]).u32()
}

// Translated from 0072bab0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0072bab0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<TESRecipe *>::NumUnfiltered` (unnamed in the engine map; the body matches the
/// other instances' `NumUnfiltered`): `list_box_num_unfiltered`.
pub fn fn_0072bab0(e: &mut Engine, this: Ptr<ListBox>) -> i32 {
    list_box_num_unfiltered(e, this, RECIPE_LIST_BOX)
}

// Translated from 0072bb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESRecipe_P>::Synchronize` (engine map): `list_box_synchronize`.
pub fn list_box_tes_recipe_synchronize(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_synchronize(e, this, RECIPE_LIST_BOX);
}

// Translated from 0072bc30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0072bc30 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<TESRecipe_P>::IsEnabled` (engine map): `list_box_is_enabled`.
pub fn list_box_tes_recipe_is_enabled(e: &mut Engine, this: Ptr<ListBox>) -> bool {
    list_box_is_enabled(e, this, RECIPE_LIST_BOX)
}

// Translated from 0072bce0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0072bce0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the statics' look-ups) is not translated.
/// `ListBox<TESRecipe_P>::UpdateHighlightBoxPos` (engine map):
/// `list_box_update_highlight_box_pos`.
pub fn list_box_tes_recipe_update_highlight_box_pos(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_update_highlight_box_pos(e, this, RECIPE_LIST_BOX);
}

// Translated from 0072c000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESRecipe_P>::SetTotalListHeight` (engine map): `list_box_set_total_list_height`.
pub fn list_box_tes_recipe_set_total_list_height(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_set_total_list_height(e, this, RECIPE_LIST_BOX);
}

// Translated from 0072c130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESRecipeComponent_P>::Synchronize` (engine map): `list_box_synchronize`.
pub fn list_box_tes_recipe_component_synchronize(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_synchronize(e, this, COMPONENT_LIST_BOX);
}

// Translated from 0072c200 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0072c200 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<TESRecipeComponent_P>::IsEnabled` (engine map): `list_box_is_enabled`.
pub fn list_box_tes_recipe_component_is_enabled(e: &mut Engine, this: Ptr<ListBox>) -> bool {
    list_box_is_enabled(e, this, COMPONENT_LIST_BOX)
}

// Translated from 0072c2b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0072c2b0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the statics' look-ups) is not translated.
/// `ListBox<TESRecipeComponent_P>::UpdateHighlightBoxPos` (engine map):
/// `list_box_update_highlight_box_pos`.
pub fn list_box_tes_recipe_component_update_highlight_box_pos(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_update_highlight_box_pos(e, this, COMPONENT_LIST_BOX);
}

// Translated from 0072c5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESRecipeComponent_P>::SetTotalListHeight` (engine map):
/// `list_box_set_total_list_height`.
pub fn list_box_tes_recipe_component_set_total_list_height(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_set_total_list_height(e, this, COMPONENT_LIST_BOX);
}

// Translated from 0072c700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESConditionItem_P>::Synchronize` (engine map): `list_box_synchronize`.
pub fn list_box_tes_condition_item_synchronize(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_synchronize(e, this, CONDITION_LIST_BOX);
}

// Translated from 0072c7d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0072c7d0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<TESConditionItem_P>::IsEnabled` (engine map): `list_box_is_enabled`.
pub fn list_box_tes_condition_item_is_enabled(e: &mut Engine, this: Ptr<ListBox>) -> bool {
    list_box_is_enabled(e, this, CONDITION_LIST_BOX)
}

// Translated from 0072c880 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0072c880 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the statics' look-ups) is not translated.
/// `ListBox<TESConditionItem_P>::UpdateHighlightBoxPos` (engine map):
/// `list_box_update_highlight_box_pos`.
pub fn list_box_tes_condition_item_update_highlight_box_pos(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_update_highlight_box_pos(e, this, CONDITION_LIST_BOX);
}

// Translated from 0072cba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESConditionItem_P>::SetTotalListHeight` (engine map):
/// `list_box_set_total_list_height`.
pub fn list_box_tes_condition_item_set_total_list_height(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_set_total_list_height(e, this, CONDITION_LIST_BOX);
}

// Translated from 0072ccd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0072ccd0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<TESRecipeComponent_P>::NumUnfiltered` (engine map): `list_box_num_unfiltered`.
pub fn list_box_tes_recipe_component_num_unfiltered(e: &mut Engine, this: Ptr<ListBox>) -> i32 {
    list_box_num_unfiltered(e, this, COMPONENT_LIST_BOX)
}

// Translated from 0072cd80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0072cd80 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<TESConditionItem_P>::NumUnfiltered` (engine map): `list_box_num_unfiltered`.
pub fn list_box_tes_condition_item_num_unfiltered(e: &mut Engine, this: Ptr<ListBox>) -> i32 {
    list_box_num_unfiltered(e, this, CONDITION_LIST_BOX)
}

/// The `BSSimpleArray<TESRecipeCategory *,1024>` destructor (`0072ce90`, `thiscall`).
const CATEGORY_ARRAY_DESTRUCTOR: u32 = 0x0072_ce90;

// Translated from 0072ce60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<TESRecipeCategory_P_1024>::_scalar_deleting_destructor_` (engine map;
/// `RET 4`): the destructor `0072ce90`, then the object freed when bit 0 of `flags` is
/// set. Returns `this`.
pub fn bs_simple_array_tes_recipe_category_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(CATEGORY_ARRAY_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}
// ---------------------------------------------------------------------------
// `RecipeMenu`.

/// `RecipeMenu::pMe` (Xbox PDB static): the open menu (`Create` stores the menu it reads,
/// the destructor clears it).
const RECIPE_MENU: u32 = 0x011d_8e90;
/// The recipe `DoEnter` selects (the recipe list's current item, `007a1910`), and the trait
/// ids the constructor looks up with `"_Title"`, `"_Value"` and `"_IsRecipeSelected"`.
const SELECTED_RECIPE_011D8E94: u32 = 0x011d_8e94;
const TITLE_TRAIT_011D8E98: u32 = 0x011d_8e98;
const VALUE_TRAIT_011D8E9C: u32 = 0x011d_8e9c;
const IS_RECIPE_SELECTED_TRAIT_011D8EA0: u32 = 0x011d_8ea0;
/// Bytes: `DoIdle` refills the list when `011d8ea4` is 1 (`fn_00728a30` sets and clears
/// it); `011d8ea5` is set while the ingredient list has the focus; `011d8ea6` is the
/// direction of `DoIdle`'s fade.
const REFRESH_011D8EA4: u32 = 0x011d_8ea4;
const INGREDIENTS_FOCUSED_011D8EA5: u32 = 0x011d_8ea5;
const FADE_DOWN_011D8EA6: u32 = 0x011d_8ea6;
/// The number of the selected recipe that can be made (`fn_00727920` computes it; the
/// constructor clears it; `DoClick`'s quantity menu offers it).
const CAN_MAKE_011D8EA8: u32 = 0x011d_8ea8;
/// Two tiles `DoIdle` fades (`fn_00728da0` stores them), a third tile it stores, the faded
/// value (`float`) and the last time read (`00825c00`).
const FADE_TILE_011D8EAC: u32 = 0x011d_8eac;
const FADE_TILE_011D8EB0: u32 = 0x011d_8eb0;
const TILE_011D8EB4: u32 = 0x011d_8eb4;
const FADE_VALUE_011D8EB8: u32 = 0x011d_8eb8;
const LAST_TIME_011D8EBC: u32 = 0x011d_8ebc;
/// `DoIdle` fades only while this byte is set.
const FADING_0119FBF0: u32 = 0x0119_fbf0;
/// The category filter's index into the category array (-1: all categories).
const FILTER_INDEX_0119FBF4: u32 = 0x0119_fbf4;
/// The `BSSimpleArray<TESRecipeCategory *,1024>` the categories are kept in
/// (`fn_00726f10` returns it).
const CATEGORY_ARRAY: u32 = 0x011d_8f08;

/// `RecipeMenu`'s vtable and class number.
const RECIPE_MENU_VTABLE: u32 = 0x0107_048c;
const RECIPE_MENU_CLASS: u32 = 0x435;

/// `RecipeMenu` fields (Xbox PDB names; the PC code uses these offsets): `pTiles` (+0x28,
/// fourteen `Tile*`), `pCrafterRef`, `pCategory`, `eFilterSound`, `lbRecipeList`,
/// `plbCurrListbox`, `lbIngredientList`, `lbConditionalList`, `bTutorial`.
const TILES: u32 = 0x28;
const CRAFTER_REF: u32 = 0x60;
const CATEGORY: u32 = 0x64;
const FILTER_SOUND: u32 = 0x68;
const RECIPE_LIST: u32 = 0x6c;
const CURRENT_LIST: u32 = 0x9c;
const INGREDIENT_LIST: u32 = 0xa0;
const CONDITIONAL_LIST: u32 = 0xd0;
const TUTORIAL: u32 = 0x100;

/// `Menu::Menu` (`00a1c4a0`) and the `Menu` destructor body (`00a1c520`).
const MENU_CONSTRUCTOR: u32 = 0x00a1_c4a0;
const MENU_DESTRUCTOR: u32 = 0x00a1_c520;
/// `0046fb20` (`thiscall` on the word at `011c3f2c`), `008454f0` (`thiscall`, `RET 4`: on
/// the category array, with 1).
const OWNER_011C3F2C: u32 = 0x011c_3f2c;
const DESTRUCTOR_STEP_0046FB20: u32 = 0x0046_fb20;
const ARRAY_RESET_008454F0: u32 = 0x0084_54f0;
/// `Tile::GetMenuByClass` (`cdecl`), the tile reader of the interface manager (`00586150`),
/// `Tile::ReadFile` (`RET 4`), `Menu::SetMenuTile` (`RET 8`), `Tile::GetMaximumDepth` (no
/// arguments, `ST0`), `Menu::PrepForVisibility` (`RET 4`), `Menu::StartFadeOut`.
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
const TILE_READER_00586150: u32 = 0x0058_6150;
const TILE_READ_FILE: u32 = 0x00a0_1b00;
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
const TILE_GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
const MENU_START_FADE_OUT: u32 = 0x00a1_d910;
/// The print function the menu reports problems with (`cdecl`, variable).
const PRINT_MESSAGE: u32 = 0x005b_5e40;
/// `DialogMenu::ServiceStart` (`cdecl`, none) and `DialogMenu::ServiceEnd` (`cdecl`, one
/// word).
const SERVICE_START: u32 = 0x0076_3ff0;
const SERVICE_END: u32 = 0x0076_40a0;
/// `ListBox<unsigned int>::Initialize` (`RET 8`: container, template).
const LIST_BOX_INITIALIZE: u32 = 0x0075_a0d0;
/// `00403df0` (`thiscall` on a game setting): the setting's string.
const SETTING_STRING: u32 = 0x0040_3df0;
/// `InterfaceManager::PlayMenuSound` (`cdecl`, one word), `Interface::PlayMenuSound`
/// (`00706f30`, `cdecl`, one word), `Interface::IsTopMenuID` (`cdecl`, one word).
const INTERFACE_MANAGER_PLAY_MENU_SOUND: u32 = 0x0071_7280;
const INTERFACE_PLAY_MENU_SOUND: u32 = 0x0070_6f30;
const INTERFACE_IS_TOP_MENU_ID: u32 = 0x0070_2450;
/// The tutorial manager at +0x4d4 of the interface manager: `00718840` (`RET 4`, a `bool`:
/// the tutorial was shown) and `007185e0` (`RET 4`).
const TUTORIAL_MANAGER_OFFSET: u32 = 0x4d4;
const TUTORIAL_SHOWN_00718840: u32 = 0x0071_8840;
const TUTORIAL_MARK_007185E0: u32 = 0x0071_85e0;
/// `TESForm::GetFormByEditorID` (`cdecl`, one word), `__RTDynamicCast` (`cdecl`, five
/// words), `TutorialMenu::Create` (`cdecl`, two words, `AL`).
const GET_FORM_BY_EDITOR_ID: u32 = 0x0048_3a00;
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TUTORIAL_MENU_CREATE: u32 = 0x007e_8890;
/// The RTTI descriptors the casts use: the source (`01183028`) and the targets.
const RTTI_SOURCE: u32 = 0x0118_3028;
const RTTI_01186130: u32 = 0x0118_6130;
const RTTI_01183108: u32 = 0x0118_3108;
const RTTI_01183158: u32 = 0x0118_3158;
const RTTI_01183200: u32 = 0x0118_3200;
const RTTI_01183A34: u32 = 0x0118_3a34;
/// `QuantityMenu::Create` (`cdecl`: count, callback, maximum).
const QUANTITY_MENU_CREATE: u32 = 0x007a_ba00;
/// The player (`011dea3c`), `MiddleHighProcess::GetSavedAcquireObject`-named `008d8520`
/// (the reference's process), `PlayerCharacter::UpdatePlayer3d` (`00950030`).
const PLAYER: u32 = 0x011d_ea3c;
const GET_PROCESS: u32 = 0x008d_8520;
const PLAYER_UPDATE_3D: u32 = 0x0095_0030;
/// The category array's methods: the count (`0044ddc0`), the element address (`006a7ad0`,
/// `RET 4`), the index of a value (`00719b20`, `RET 0xc`), the append (`007cb2e0`,
/// `RET 4`).
const ARRAY_COUNT: u32 = 0x0044_ddc0;
const ARRAY_AT: u32 = 0x006a_7ad0;
const ARRAY_FIND_00719B20: u32 = 0x0071_9b20;
const ARRAY_APPEND_007CB2E0: u32 = 0x007c_b2e0;
/// The compare function `00719b20` is given (`009a3830`).
const COMPARE_009A3830: u32 = 0x009a_3830;
/// A string object's text (`00408da0`, `thiscall`), the word at +0 (`00559450`).
const STRING_TEXT: u32 = 0x0040_8da0;
const WORD_AT_ZERO: u32 = 0x0055_9450;
/// The string functions (`cdecl`): `00406d00` (format: buffer, size, format, words),
/// `00406d30` (copy: buffer, size, source), `00469880` (compare two strings),
/// `0044a670` (length), `00406f60` (format into a `BSString`).
const FORMAT: u32 = 0x0040_6d00;
const COPY: u32 = 0x0040_6d30;
const COMPARE_STRINGS: u32 = 0x0046_9880;
const STRING_LENGTH: u32 = 0x0044_a670;
const FORMAT_STRING_OBJECT: u32 = 0x0040_6f60;
/// The `BSString` constructor and destructor (`004037b0`, `004037d0`).
const STRING_OBJECT_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_OBJECT_DESTRUCT: u32 = 0x0040_37d0;
/// The recipe's methods (`thiscall`): the head of the recipe list of the object
/// (`004077e0` on the word at `011c3f2c`), the node's end test (`008256d0`, a `bool`),
/// `006a1b60` (a form the code compares with `pCategory`), `006286d0` (a form the code
/// keeps in the category array and names through its +0x18), `006a9540` (the conditions
/// given to `TESCondition::IsTrue`, `00680c30`, `RET 8`), `005d43c0` and `005a8080` (the
/// lists the code treats as outputs and inputs), `0059bb30` (an actor value, -1 for none),
/// `0045cd60` (the level that actor value needs),
/// `VATS::GetCount`-named `005ae380` (a list's count).
const RECIPE_LIST_HEAD_004077E0: u32 = 0x0040_77e0;
const NODE_AT_END_008256D0: u32 = 0x0082_56d0;
const RECIPE_CATEGORY_006A1B60: u32 = 0x006a_1b60;
const RECIPE_CATEGORY_KEY_006286D0: u32 = 0x0062_86d0;
const RECIPE_CONDITIONS_006A9540: u32 = 0x006a_9540;
const CONDITION_IS_TRUE: u32 = 0x0068_0c30;
const RECIPE_OUTPUTS_005D43C0: u32 = 0x005d_43c0;
const RECIPE_INPUTS_005A8080: u32 = 0x005a_8080;
const RECIPE_SKILL_0059BB30: u32 = 0x0059_bb30;
const RECIPE_SKILL_LEVEL_0045CD60: u32 = 0x0045_cd60;
const LIST_COUNT_005AE380: u32 = 0x005a_e380;
/// `ActorValueOwner::GetClampedActorValue` (`RET 4`, on +0xa4 of an actor),
/// `ActorValue::GetActorValueName` (`cdecl`).
const GET_CLAMPED_ACTOR_VALUE: u32 = 0x0066_ef20;
const GET_ACTOR_VALUE_NAME: u32 = 0x0066_e950;
/// `TESObjectREFR::GetInventoryItem` (`RET 8`).
const GET_INVENTORY_ITEM: u32 = 0x0057_6260;
/// `007b5370` (`thiscall`, in `repairmenu.cpp`): the item's tile slot.
const ITEM_TILE_007B5370: u32 = 0x007b_5370;
/// `ListBox<StartMenu::Option_P>::GetTileFor` (`007a22d0`, `RET 4`), the list box's current
/// item (`007a1910`), `ListBox<BGSNote_P>::SaveListState` (`007312e0`; template code).
const LIST_BOX_GET_TILE_FOR: u32 = 0x007a_22d0;
const LIST_BOX_CURRENT_ITEM: u32 = 0x007a_1910;
const LIST_BOX_SAVE_LIST_STATE: u32 = 0x0073_12e0;
/// `00464f30` (`cdecl`, two words): the error item's data.
const ERROR_ITEM_00464F30: u32 = 0x0046_4f30;
/// `0048cee0` (`thiscall`), `00480f80` (`cdecl`, two words), `0087f4c0` (`thiscall` on the
/// player), `00481280` (`RET 4`): the item text of a selected recipe's output.
const FORM_0048CEE0: u32 = 0x0048_cee0;
const FORM_00480F80: u32 = 0x0048_0f80;
const PLAYER_0087F4C0: u32 = 0x0087_f4c0;
const FORM_00481280: u32 = 0x0048_1280;

/// `"Recipe menu called with NULL vendor!  Oh, noes!"`, the menu file, the creation
/// failure message, `"CM_list_template"`, `"HelpCrafting"`, the missing category text,
/// `"ERROR WITH CAST"`, `"%s (%d/%d)"`, `"%s (%d)"`, `"%s"`.
const NULL_VENDOR_TEXT: u32 = 0x0107_0568;
const MENU_FILE_TEXT: u32 = 0x0107_054c;
const CREATION_FAILED_TEXT: u32 = 0x0107_04f8;
const LIST_TEMPLATE_TEXT: u32 = 0x0107_04d4;
const HELP_CRAFTING_TEXT: u32 = 0x0101_8d5c;
const MISSING_CATEGORY_TEXT: u32 = 0x0107_05b4;
const ERROR_WITH_CAST_TEXT: u32 = 0x0107_0598;
const HAVE_OF_NEED_FORMAT: u32 = 0x0107_05a8;
const NAME_COUNT_FORMAT: u32 = 0x0106_eb00;
const ONE_STRING_FORMAT: u32 = 0x0101_9f08;
/// `"_Title"`, `"_Value"`, `"_IsRecipeSelected"`.
const TITLE_TEXT: u32 = 0x0106_ed20;
const VALUE_TEXT: u32 = 0x0106_ed18;
const IS_RECIPE_SELECTED_TEXT: u32 = 0x0107_0474;
/// The `double`s `6006.0` and `102.0` (the stacking types the depth is set for), `20.0`.
const STACKING_TYPE_ONE: u32 = 0x0107_04f0;
const STACKING_TYPE_TWO: u32 = 0x0107_04e8;
const TWENTY_DOUBLE: u32 = 0x0102_fc70;

/// The game settings whose strings the menu shows (`00403df0` on each).
const SETTING_011D2088: u32 = 0x011d_2088;
const SETTING_011D2B08: u32 = 0x011d_2b08;
const SETTING_011D3090: u32 = 0x011d_3090;
const SETTING_011D50F8: u32 = 0x011d_50f8;
const SETTING_011D26F4: u32 = 0x011d_26f4;
const SETTING_011D33FC: u32 = 0x011d_33fc;

/// The open menu.
fn me(e: &Engine) -> u32 {
    e.global::<u32>(RECIPE_MENU)
}

/// `pTiles[index]` of the open menu, read the way the code reads it.
fn me_tile(e: &Engine, offset: u32) -> u32 {
    e.mem.u32(me(e).wrapping_add(offset))
}

/// The open menu's crafter when it is one the code accepts (vtable +0x100 true), else 0.
fn accepted_crafter(e: &mut Engine) -> u32 {
    if e.mem.u32(me(e).wrapping_add(CRAFTER_REF)) != 0 {
        let crafter = e.mem.u32(me(e).wrapping_add(CRAFTER_REF));
        if e.vcall(crafter, 0x100, &args![]).bool() {
            return e.mem.u32(me(e).wrapping_add(CRAFTER_REF));
        }
    }
    0
}

/// The setting's string set as trait `0xfc4` of `tile` (flag 1).
fn set_setting_text(e: &mut Engine, tile: u32, setting: u32) {
    let text = e.call(SETTING_STRING, &args![setting]).u32();
    e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, text, 1u32]);
}

/// `__RTDynamicCast(object, 0, 01183028, target, 0)`.
fn dynamic_cast(e: &mut Engine, object: u32, target: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![object, 0u32, RTTI_SOURCE, target, 0u32],
    )
    .u32()
}

// Translated from 00726df0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00726df0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 3 around the members' construction) is not translated.
/// `RecipeMenu::RecipeMenu` (Xbox PDB): the `Menu` base, the vtable `0107048c`, the three
/// list boxes (+0x6c recipes, +0xa0 ingredients, +0xd0 conditions), the trait ids of
/// `"_Title"`, `"_Value"` and `"_IsRecipeSelected"`, the selected recipe cleared,
/// `plbCurrListbox` set to the recipe list, the count cleared, the category array reset
/// (`008454f0` with 1), `eFilterSound` 3 and `bTutorial` cleared. Returns `this`.
pub fn recipe_menu_recipe_menu(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(MENU_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), RECIPE_MENU_VTABLE);
    fn_007299a0(e, Ptr::new(this.addr() + RECIPE_LIST));
    fn_0072a7c0(e, Ptr::new(this.addr() + INGREDIENT_LIST));
    fn_0072b210(e, Ptr::new(this.addr() + CONDITIONAL_LIST));
    for (name, slot) in [
        (TITLE_TEXT, TITLE_TRAIT_011D8E98),
        (VALUE_TEXT, VALUE_TRAIT_011D8E9C),
        (IS_RECIPE_SELECTED_TEXT, IS_RECIPE_SELECTED_TRAIT_011D8EA0),
    ] {
        let id = e.call(TEXT_TO_TRAIT, &args![name]).u32();
        e.set_global(slot, id);
    }
    e.set_global(SELECTED_RECIPE_011D8E94, 0u32);
    e.mem
        .set_u32(this.addr() + CURRENT_LIST, this.addr() + RECIPE_LIST);
    e.set_global(CAN_MAKE_011D8EA8, 0u32);
    let array = fn_00726f10(e);
    e.call(ARRAY_RESET_008454F0, &args![array, 1u32]);
    e.mem.set_u32(this.addr() + FILTER_SOUND, 3);
    e.mem.set_u8(this.addr() + TUTORIAL, 0);
    this
}

// Translated from 00726f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RecipeMenu::GetClass` (Xbox PDB): the class number `0x435`.
pub fn recipe_menu_get_class(_e: &mut Engine, _this: Ptr) -> u32 {
    RECIPE_MENU_CLASS
}

// Translated from 00726f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments): the address of the category array `011d8f08`.
pub fn fn_00726f10(_e: &mut Engine) -> u32 {
    CATEGORY_ARRAY
}

// Translated from 00726f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RecipeMenu::_scalar_deleting_destructor_` (Xbox PDB; `RET 4`): the destructor, then the
/// object freed when bit 0 of `flags` is set. Returns `this`.
pub fn recipe_menu_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00726f50(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00726f50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00726f50 - the C++ exception frame (FS:[0] chain, unwind states 0 to 3 around the members' destruction) is not translated.
/// The `RecipeMenu` destructor (unnamed in the engine map): the vtable, `0046fb20` on the
/// word at `011c3f2c`, `pMe` cleared, the three list boxes destroyed (conditions,
/// ingredients, recipes) and the `Menu` destructor.
pub fn fn_00726f50(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), RECIPE_MENU_VTABLE);
    let owner = e.global::<u32>(OWNER_011C3F2C);
    e.call(DESTRUCTOR_STEP_0046FB20, &args![owner]);
    e.set_global(RECIPE_MENU, 0u32);
    fn_0072b2e0(e, Ptr::new(this.addr() + CONDITIONAL_LIST));
    fn_0072a890(e, Ptr::new(this.addr() + INGREDIENT_LIST));
    fn_00729a70(e, Ptr::new(this.addr() + RECIPE_LIST));
    e.call(MENU_DESTRUCTOR, &args![this]);
}

// Translated from 00726ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RecipeMenu::Create` (Xbox PDB, `cdecl`: crafter, category). Without a crafter, prints
/// `"Recipe menu called with NULL vendor!..."` (with 0x12) and answers 0. Otherwise deletes
/// an open menu of the class, reads the menu file and takes its menu as `pMe` (clearing the
/// three tiles `011d8eac`, `011d8eb0`, `011d8eb4`). For a menu of this class: attaches the
/// tile, reports missing tiles (and goes on), sets the depth for the two stacking types,
/// starts the service (`DialogMenu::ServiceStart`), sets up the recipe and ingredient lists
/// (`"CM_list_template"` in the tiles at +0x34 and +0x40), gives six tiles their setting
/// strings, lowers the tiles at +0x44 and +0x48 by 20 (trait `0xfa1`), stores the crafter
/// and the category, plays sound `0x24`, shows the `"HelpCrafting"` tutorial once (the
/// result kept in `bTutorial`), prepares the menu for showing and fills the recipe list
/// (`fn_00727680`). Returns the root tile. A menu of another class is deleted (when it has
/// a tile) and the result is 0.
pub fn recipe_menu_create(e: &mut Engine, crafter: u32, category: u32) -> u32 {
    let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
    if crafter == 0 {
        e.call(PRINT_MESSAGE, &args![0x12u32, NULL_VENDOR_TEXT]);
        return 0;
    }
    let class = fn_007273e0(e);
    let old = e.call(TILE_GET_MENU_BY_CLASS, &args![class]).u32();
    if old != 0 {
        e.vcall(old, 0, &args![1u32]);
    }
    let reader = e.call(TILE_READER_00586150, &args![interface]).u32();
    let root = e.call(TILE_READ_FILE, &args![reader, MENU_FILE_TEXT]).u32();
    let menu = if root != 0 {
        e.call(TILE_GET_MENU, &args![root]).u32()
    } else {
        0
    };
    e.set_global(RECIPE_MENU, menu);
    e.set_global(FADE_TILE_011D8EAC, 0u32);
    e.set_global(FADE_TILE_011D8EB0, 0u32);
    e.set_global(TILE_011D8EB4, 0u32);
    if me(e) != 0 {
        let class = e.vcall(me(e), 0x34, &args![]).u32();
        if class == fn_007273e0(e) {
            return finish_create(e, root, crafter, category);
        }
    }
    if me(e) != 0 && e.call(WORD_AT_FOUR, &args![me(e)]).u32() != 0 && me(e) != 0 {
        let menu = me(e);
        e.vcall(menu, 0, &args![1u32]);
    }
    0
}

/// The part of [`recipe_menu_create`] after the menu of the class was read.
fn finish_create(e: &mut Engine, root: u32, crafter: u32, category: u32) -> u32 {
    let menu = me(e);
    e.call(MENU_SET_MENU_TILE, &args![menu, root, 0u32]);
    if !fn_007273f0(e, Ptr::new(me(e))) {
        e.call(PRINT_MESSAGE, &args![CREATION_FAILED_TEXT]);
    }
    // `FCOMP double; TEST AH,0x44`: equal (and ordered) to either stacking type.
    let kind = e.call(TILE_GET_FLOAT, &args![root, 0xfa7u32]).f64();
    let matches = kind == e.global::<f64>(STACKING_TYPE_ONE) || {
        let again = e.call(TILE_GET_FLOAT, &args![root, 0xfa7u32]).f64();
        again == e.global::<f64>(STACKING_TYPE_TWO)
    };
    if matches {
        let depth = e.call(TILE_GET_MAXIMUM_DEPTH, &args![]).f64();
        e.call(
            TILE_SET_FLOAT,
            &args![root, 0xfadu32, x87_store32(depth), 1u32],
        );
    }
    e.call(SERVICE_START, &args![]);
    for (list, container) in [(RECIPE_LIST, 0x34u32), (INGREDIENT_LIST, 0x40)] {
        let container = me_tile(e, container);
        let list = me(e).wrapping_add(list);
        e.call(
            LIST_BOX_INITIALIZE,
            &args![list, container, LIST_TEMPLATE_TEXT],
        );
    }
    for (setting, offset) in [
        (SETTING_011D2088, 0x2cu32),
        (SETTING_011D2B08, 0x5c),
        (SETTING_011D3090, 0x54),
        (SETTING_011D50F8, 0x58),
        (SETTING_011D26F4, 0x44),
        (SETTING_011D33FC, 0x48),
    ] {
        let text = e.call(SETTING_STRING, &args![setting]).u32();
        let tile = me_tile(e, offset);
        e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, text, 1u32]);
    }
    for offset in [0x44u32, 0x48] {
        let tile = me_tile(e, offset);
        let value = e.call(TILE_GET_FLOAT, &args![tile, 0xfa1u32]).f64();
        let lowered = x87_store32(x87_sub(value, e.global::<f64>(TWENTY_DOUBLE)));
        let tile = me_tile(e, offset);
        e.call(
            TILE_SET_FLOAT,
            &args![tile, 0xfa1u32, x87_quiet_bits(lowered), 1u32],
        );
    }
    let menu = me(e);
    e.mem.set_u32(menu + CRAFTER_REF, crafter);
    let menu = me(e);
    e.mem.set_u32(menu + CATEGORY, category);
    e.call(INTERFACE_MANAGER_PLAY_MENU_SOUND, &args![0x24u32]);
    let manager = e.call(INTERFACE_MANAGER, &args![]).u32() + TUTORIAL_MANAGER_OFFSET;
    if !e
        .call(TUTORIAL_SHOWN_00718840, &args![manager, 0x26u32])
        .bool()
    {
        let form = e
            .call(GET_FORM_BY_EDITOR_ID, &args![HELP_CRAFTING_TEXT])
            .u32();
        let tutorial = dynamic_cast(e, form, RTTI_01186130);
        if tutorial != 0 {
            let shown = e.call(TUTORIAL_MENU_CREATE, &args![tutorial, 0u32]).u8();
            let menu = me(e);
            e.mem.set_u8(menu + TUTORIAL, shown);
            let manager = e.call(INTERFACE_MANAGER, &args![]).u32() + TUTORIAL_MANAGER_OFFSET;
            e.call(TUTORIAL_MARK_007185E0, &args![manager, 0x26u32]);
        }
    }
    let menu = me(e);
    e.call(MENU_PREP_FOR_VISIBILITY, &args![menu, 0u32]);
    let menu = me(e);
    fn_00727680(e, Ptr::new(menu), 0, category);
    root
}

// Translated from 007273e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The menu class number `0x435`, without an object (no `this`).
pub fn fn_007273e0(_e: &mut Engine) -> u32 {
    RECIPE_MENU_CLASS
}

// Translated from 00727430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RecipeMenu::Close` (Xbox PDB, no arguments): with an open menu, updates the player's 3D
/// when the player's process says so (vtable +0x474 with 1), ends the service
/// (`DialogMenu::ServiceEnd(0)`), gives the menu tile trait `0x1772` the value 1 and starts
/// the fade-out.
pub fn recipe_menu_close(e: &mut Engine) {
    if me(e) == 0 {
        return;
    }
    let player = e.global::<u32>(PLAYER);
    let process = e.call(GET_PROCESS, &args![player]).u32();
    if e.vcall(process, 0x474, &args![1u32]).bool() {
        let player = e.global::<u32>(PLAYER);
        e.call(PLAYER_UPDATE_3D, &args![player]);
    }
    e.call(SERVICE_END, &args![0u32]);
    let tile = me_tile(e, 4);
    e.call(TILE_SET_INT, &args![tile, 0x1772u32, 1u32]);
    let menu = me(e);
    e.call(MENU_START_FADE_OUT, &args![menu]);
}

// Translated from 007274b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RecipeMenu::DoClick` (Xbox PDB; `RET 8`: id, tile). Without a tile, an id below 14
/// reads `pTiles[id]` (the code keeps it but does not use it). The switch (byte map
/// `0072766c`, table `0072765c`): ids 0 to 2, unless the ingredient list has the focus,
/// play the filter sound and move the category filter (id 0 back, 1 and 2 on; below -1 it
/// wraps to the last category, at or past the count (unsigned) to -1), show the
/// category's name (or the setting `011d2088`'s string for -1) in the tile at +0x2c,
/// highlight the recipe list's first tile, refresh it (vtable +0x10), save its state,
/// filter it with `fn_00728d60` and restore its state with the focus. Id 7 opens the
/// quantity menu (`CAN_MAKE_011D8EA8` items, callback `fn_007284f0`, maximum
/// `0x7fffffff`); id 8 clears the ingredient focus and closes the menu.
pub fn recipe_menu_do_click(e: &mut Engine, this: Ptr, id: i32, tile: u32) {
    let _tile = if tile == 0 && (0..0xe).contains(&id) {
        e.mem.u32(this.addr().wrapping_add(TILES + 4 * id as u32))
    } else {
        tile
    };
    match id {
        0..=2 => {
            if e.global::<u8>(INGREDIENTS_FOCUSED_011D8EA5) != 0 {
                return;
            }
            let sound = e.mem.u32(this.addr() + FILTER_SOUND);
            e.call(INTERFACE_PLAY_MENU_SOUND, &args![sound]);
            let step = if id != 0 { 1u32 } else { 0xffff_ffff };
            let index = e.global::<u32>(FILTER_INDEX_0119FBF4).wrapping_add(step);
            e.set_global(FILTER_INDEX_0119FBF4, index);
            if (e.global::<u32>(FILTER_INDEX_0119FBF4) as i32) < -1 {
                let array = fn_00726f10(e);
                let count = e.call(ARRAY_COUNT, &args![array]).u32();
                e.set_global(FILTER_INDEX_0119FBF4, count.wrapping_sub(1));
            } else {
                let array = fn_00726f10(e);
                let count = e.call(ARRAY_COUNT, &args![array]).u32();
                if e.global::<u32>(FILTER_INDEX_0119FBF4) >= count {
                    e.set_global(FILTER_INDEX_0119FBF4, 0xffff_ffffu32);
                }
            }
            if e.global::<u32>(FILTER_INDEX_0119FBF4) == 0xffff_ffff {
                let text = e.call(SETTING_STRING, &args![SETTING_011D2088]).u32();
                let tile = e.mem.u32(this.addr() + 0x2c);
                e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, text, 1u32]);
            } else {
                let index = e.global::<u32>(FILTER_INDEX_0119FBF4);
                let array = fn_00726f10(e);
                let entry = e.call(ARRAY_AT, &args![array, index]).u32();
                let category = e.mem.u32(entry);
                let text = e.call(STRING_TEXT, &args![category + 0x18]).u32();
                let tile = e.mem.u32(this.addr() + 0x2c);
                e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, text, 1u32]);
            }
            let list = this.addr() + RECIPE_LIST;
            let first = e.vcall(list, 0x14, &args![0u32, 0u32]).u32();
            e.vcall(list, 0, &args![first]);
            e.vcall(list, 0x10, &args![]);
            e.call(LIST_BOX_SAVE_LIST_STATE, &args![list]);
            list_box_tes_recipe_filter(e, Ptr::new(list), FILTER_00728D60);
            list_box_tes_recipe_restore_list_state(e, Ptr::new(list), 1);
        }
        7 => {
            let count = e.global::<u32>(CAN_MAKE_011D8EA8);
            e.call(
                QUANTITY_MENU_CREATE,
                &args![count, MAKE_CALLBACK_007284F0, 0x7fff_ffffu32],
            );
        }
        8 => {
            e.set_global(INGREDIENTS_FOCUSED_011D8EA5, 0u8);
            recipe_menu_close(e);
        }
        _ => {}
    }
}

/// `fn_00728d60`'s and `fn_007284f0`'s addresses, as the code passes them.
const FILTER_00728D60: u32 = 0x0072_8d60;
const MAKE_CALLBACK_007284F0: u32 = 0x0072_84f0;
/// `fn_00728c10`'s, `fn_007278c0`'s and `fn_00728cd0`'s addresses, as the code passes them.
const COMPARE_00728C10: u32 = 0x0072_8c10;
const COMPARE_007278C0: u32 = 0x0072_78c0;
const COMPARE_00728CD0: u32 = 0x0072_8cd0;

// Translated from 00727680 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00727680 - the /GS security cookie check (00ec408c) is not translated
/// Unnamed (`thiscall`, `RET 8`: an unused word, a category; the recipe menu): fills the
/// recipe list. For each recipe of the data handler's list (`004077e0` on the word at
/// `011c3f2c`, until the node's end test) of `category` (all when 0) whose conditions are
/// true for the crafter: when its category word (`006286d0`) is set and not yet in the
/// category array (`00719b20` with `009a3830`), appends it (`007cb2e0`); adds the recipe
/// to the list with its name, or `"%s (%d)"` with the output count when that is above 1,
/// formatted into a 0x100-byte stack buffer; and gives the item's tile alpha `0xfa9` 0xff
/// when `fn_00727920` says some can be made, else 0x7f. Then filters the list with
/// `fn_00728c10` (vtable +0x20) and sorts the category array with `fn_007278c0`.
pub fn fn_00727680(e: &mut Engine, this: Ptr, _unused_1: u32, category: u32) {
    let owner = e.global::<u32>(OWNER_011C3F2C);
    let mut node = e.call(RECIPE_LIST_HEAD_004077E0, &args![owner]).u32();
    while node != 0 && !e.call(NODE_AT_END_008256D0, &args![node]).bool() {
        let recipe = node_item(e, node);
        node = node_next(e, node);
        let recipe_category = e.call(RECIPE_CATEGORY_006A1B60, &args![recipe]).u32();
        if category != 0 && category != recipe_category {
            continue;
        }
        let conditions = e.call(RECIPE_CONDITIONS_006A9540, &args![recipe]).u32();
        let crafter = me_tile(e, CRAFTER_REF);
        if !e
            .call(CONDITION_IS_TRUE, &args![conditions, crafter, 0u32])
            .bool()
        {
            continue;
        }
        let word = e.call(RECIPE_CATEGORY_KEY_006286D0, &args![recipe]).u32();
        if word != 0 {
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), word);
                let array = fn_00726f10(e);
                let index = e
                    .call(
                        ARRAY_FIND_00719B20,
                        &args![array, slot, 0u32, COMPARE_009A3830],
                    )
                    .u32();
                let array = fn_00726f10(e);
                let count = e.call(ARRAY_COUNT, &args![array]).u32();
                if index >= count {
                    let array = fn_00726f10(e);
                    e.call(ARRAY_APPEND_007CB2E0, &args![array, slot]);
                }
            });
        }
        e.with_stack(0x100, |e, buffer| {
            let outputs = e.call(RECIPE_OUTPUTS_005D43C0, &args![recipe]).u32();
            let output_count = |e: &mut Engine| {
                let outputs = e.call(RECIPE_OUTPUTS_005D43C0, &args![recipe]).u32();
                let first = node_item(e, outputs);
                e.call(WORD_AT_ZERO, &args![first]).u32()
            };
            if node_item(e, outputs) != 0 && output_count(e) > 1 {
                let count = output_count(e);
                let name = e.call(STRING_TEXT, &args![recipe + 0x18]).u32();
                e.call(
                    FORMAT,
                    &args![buffer, 0x100u32, NAME_COUNT_FORMAT, name, count],
                );
            } else {
                let name = e.call(STRING_TEXT, &args![recipe + 0x18]).u32();
                e.call(FORMAT, &args![buffer, 0x100u32, ONE_STRING_FORMAT, name]);
            }
            let list = Ptr::new(this.addr() + RECIPE_LIST);
            let tile = fn_00729ae0(e, list, recipe, buffer.addr(), 0, 0);
            let alpha = if fn_00727920(e, recipe) == 0 {
                0x7fu32
            } else {
                0xff
            };
            set_item_alpha(e, tile, alpha);
        });
    }
    let list = this.addr() + RECIPE_LIST;
    e.vcall(list, 0x20, &args![COMPARE_00728C10]);
    fn_00729970(e, Ptr::new(CATEGORY_ARRAY), COMPARE_007278C0);
}

// Translated from 007278c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`: two pointers to categories): compares the categories' names (+0x18,
/// through `00408da0`) with `00469880`.
pub fn fn_007278c0(e: &mut Engine, first: u32, second: u32) -> i32 {
    let a = e.mem.u32(first);
    let b = e.mem.u32(second);
    let a_name = e.call(STRING_TEXT, &args![a + 0x18]).u32();
    let b_name = e.call(STRING_TEXT, &args![b + 0x18]).u32();
    e.call(COMPARE_STRINGS, &args![a_name, b_name]).i32()
}

// Translated from 00727920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, a recipe): how many times the open menu's crafter can make it, also
/// kept in `CAN_MAKE_011D8EA8` (set to 100 first). 0 without a category or a category
/// word; 100 without an accepted crafter (vtable +0x100); 0 when the crafter's clamped
/// skill (`0059bb30`, when not -1) is below the required level or the recipe is not of the
/// menu's category; otherwise, for each input with a form of the cast's type, the
/// crafter's count of it (`GetInventoryItem`, the entry deleted) divided by the needed
/// count (unsigned), the smallest kept (signed); 0 when any count is short.
pub fn fn_00727920(e: &mut Engine, recipe: u32) -> u32 {
    let crafter = accepted_crafter(e);
    e.set_global(CAN_MAKE_011D8EA8, 100u32);
    if e.call(RECIPE_CATEGORY_006A1B60, &args![recipe]).u32() == 0
        || e.call(RECIPE_CATEGORY_KEY_006286D0, &args![recipe]).u32() == 0
    {
        return 0;
    }
    if crafter == 0 {
        return e.global::<u32>(CAN_MAKE_011D8EA8);
    }
    if e.call(RECIPE_SKILL_0059BB30, &args![recipe]).i32() > -1 {
        let skill = e.call(RECIPE_SKILL_0059BB30, &args![recipe]).u32();
        let value = e
            .call(GET_CLAMPED_ACTOR_VALUE, &args![crafter + 0xa4, skill])
            .i32();
        let level = e.call(RECIPE_SKILL_LEVEL_0045CD60, &args![recipe]).i32();
        if value < level {
            return 0;
        }
    }
    let recipe_category = e.call(RECIPE_CATEGORY_006A1B60, &args![recipe]).u32();
    if me_tile(e, CATEGORY) != recipe_category {
        return 0;
    }
    let mut node = e.call(RECIPE_INPUTS_005A8080, &args![recipe]).u32();
    while node != 0 && !e.call(NODE_AT_END_008256D0, &args![node]).bool() {
        let input = node_item(e, node);
        let item = node_item(e, node);
        let form = e.call(WORD_AT_FOUR, &args![item]).u32();
        let object = dynamic_cast(e, form, RTTI_01183108);
        if object != 0 {
            let entry = e
                .call(GET_INVENTORY_ITEM, &args![crafter, object, 0u32])
                .u32();
            let have = if entry != 0 {
                e.call(WORD_AT_FOUR, &args![entry]).u32()
            } else {
                0
            };
            if entry != 0 {
                e.call(ITEM_DATA_DELETE, &args![entry, 1u32]);
            }
            let need = e.call(WORD_AT_ZERO, &args![input]).u32();
            if have < need {
                return 0;
            }
            let need = e.call(WORD_AT_ZERO, &args![input]).u32();
            let times = have / need;
            if (times as i32) < e.global::<i32>(CAN_MAKE_011D8EA8) {
                e.set_global(CAN_MAKE_011D8EA8, times);
            }
        }
        node = node_next(e, node);
    }
    e.global::<u32>(CAN_MAKE_011D8EA8)
}

/// Sets trait `0xfa9` (the alpha) of the tile `fn_00727680`'s and `DoEnter`'s items keep at
/// `007b5370(tile + 4)`.
fn set_item_alpha(e: &mut Engine, tile: u32, alpha: u32) {
    let slot = e.call(ITEM_TILE_007B5370, &args![tile + 4]).u32();
    let item_tile = e.mem.u32(slot);
    e.call(TILE_SET_INT, &args![item_tile, 0xfa9u32, alpha]);
}

// Translated from 00727b10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00727b10 - the /GS security cookie check (00ec408c) is not translated
/// `RecipeMenu::DoEnter` (Xbox PDB; `RET 8`: id, tile; without a tile an id below 14 reads
/// `pTiles[id]`, unused). Id 3 moves the focus to tile 6 (vtable +0x14), saves the recipe
/// list's state, highlights nothing in the ingredient list and restores the recipe list.
/// Id 6 moves the focus to tile 3, highlights nothing in the recipe list, (with
/// `004b71d0`) highlights the ingredient list's first tile, marks the ingredient focus and
/// enters (vtable +0x10, id 0xF) the ingredient list's highlighted tile. Id 0xF shows the
/// selected item: with the ingredient focus, the ingredient list's current item; otherwise
/// it hides two tiles, takes the recipe list's current item as the selected recipe and, for
/// one, shows its category (or the missing category text), the inputs with the counts the
/// crafter has (or `"ERROR WITH CAST"`), the skill line, whether it can be made, and the
/// first output. The item shown goes to `fn_00728da0` with its form.
pub fn recipe_menu_do_enter(e: &mut Engine, this: Ptr, id: i32, tile: u32) {
    let _tile = if tile == 0 && (0..0xe).contains(&id) {
        e.mem.u32(this.addr().wrapping_add(TILES + 4 * id as u32))
    } else {
        tile
    };
    let base = this.addr();
    match id {
        3 => {
            e.vcall(base, 0x14, &args![6u32, 0u32]);
            e.call(LIST_BOX_SAVE_LIST_STATE, &args![base + RECIPE_LIST]);
            e.vcall(base + INGREDIENT_LIST, 0, &args![0u32]);
            list_box_tes_recipe_restore_list_state(e, Ptr::new(base + RECIPE_LIST), 0);
        }
        6 => {
            e.vcall(base, 0x14, &args![3u32, 0u32]);
            e.vcall(base + RECIPE_LIST, 0, &args![0u32]);
            e.call(INTERFACE_MANAGER, &args![]);
            if e.call(FLAG_004B71D0, &args![]).bool() {
                let first = e
                    .vcall(base + INGREDIENT_LIST, 0x14, &args![0u32, 0u32])
                    .u32();
                e.vcall(base + INGREDIENT_LIST, 0, &args![first]);
            }
            e.set_global(INGREDIENTS_FOCUSED_011D8EA5, 1u8);
            let highlighted = e.vcall(base + INGREDIENT_LIST, 4, &args![]).u32();
            e.vcall(base, 0x10, &args![0xfu32, highlighted]);
        }
        0xf => show_selected(e, this),
        _ => {}
    }
}

/// `DoEnter`'s id 0xF.
fn show_selected(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let mut selected = 0u32;
    if e.global::<u8>(INGREDIENTS_FOCUSED_011D8EA5) != 0 {
        if e.call(LIST_BOX_CURRENT_ITEM, &args![base + INGREDIENT_LIST])
            .u32()
            != 0
        {
            selected = e
                .call(LIST_BOX_CURRENT_ITEM, &args![base + INGREDIENT_LIST])
                .u32();
        }
    } else {
        for offset in [0x50u32, 0x4c] {
            let tile = e.mem.u32(base + offset);
            e.call(TILE_SET_INT, &args![tile, 0xfa3u32, 0u32]);
        }
        let current = e.mem.u32(base + CURRENT_LIST);
        if e.call(LIST_BOX_CURRENT_ITEM, &args![current]).u32() != 0 {
            let current = e.mem.u32(base + CURRENT_LIST);
            let item = e.call(LIST_BOX_CURRENT_ITEM, &args![current]).u32();
            e.set_global(SELECTED_RECIPE_011D8E94, item);
        }
        let crafter = accepted_crafter(e);
        if e.global::<u32>(SELECTED_RECIPE_011D8E94) != 0 {
            show_recipe(e, this, crafter, &mut selected);
        }
    }
    if selected == 0 {
        let tile = e.mem.u32(base + 0x4c);
        e.call(TILE_SET_INT, &args![tile, 0xfa3u32, 0u32]);
        return;
    }
    let holder = e.mem.u32(base + 0x50);
    let form = e.call(WORD_AT_FOUR, &args![selected]).u32();
    fn_00728da0(e, this, form, holder);
    let tile = e.mem.u32(base + 0x50);
    e.call(TILE_SET_INT, &args![tile, 0xfa3u32, 1u32]);
    let form = e.call(WORD_AT_FOUR, &args![selected]).u32();
    let first = dynamic_cast(e, form, RTTI_01183200);
    if first != 0 && e.call(FORM_0048CEE0, &args![first]).u32() != 0 {
        let name = e.call(STRING_TEXT, &args![first]).u32();
        let tile = e.mem.u32(base + 0x4c);
        e.call(TILE_SET_STRING, &args![tile, 0xfccu32, name, 1u32]);
        let tile = e.mem.u32(base + 0x4c);
        e.call(TILE_SET_INT, &args![tile, 0xfa3u32, 1u32]);
        return;
    }
    let form = e.call(WORD_AT_FOUR, &args![selected]).u32();
    let second = dynamic_cast(e, form, RTTI_01183A34);
    let player = e.global::<u32>(PLAYER);
    if second != 0 && e.call(FORM_00480F80, &args![second, player]).u32() != 0 {
        let player = e.global::<u32>(PLAYER);
        let value = e.call(PLAYER_0087F4C0, &args![player]).u32();
        let text = e.call(FORM_00481280, &args![second + 0x70, value]).u32();
        let tile = e.mem.u32(base + 0x4c);
        e.call(TILE_SET_STRING, &args![tile, 0xfccu32, text, 1u32]);
        let tile = e.mem.u32(base + 0x4c);
        e.call(TILE_SET_INT, &args![tile, 0xfa3u32, 1u32]);
    } else {
        let tile = e.mem.u32(base + 0x4c);
        e.call(TILE_SET_INT, &args![tile, 0xfa3u32, 0u32]);
    }
}

/// `show_selected` for a selected recipe (the menu's tiles through `pMe`).
fn show_recipe(e: &mut Engine, this: Ptr, crafter: u32, selected: &mut u32) {
    let base = this.addr();
    let mut single_output = false;
    let recipe = e.global::<u32>(SELECTED_RECIPE_011D8E94);
    let mut text = MISSING_CATEGORY_TEXT;
    if e.call(RECIPE_CATEGORY_006A1B60, &args![recipe]).u32() != 0 {
        let category = e.call(RECIPE_CATEGORY_006A1B60, &args![recipe]).u32();
        text = e.call(STRING_TEXT, &args![category + 0x18]).u32();
    }
    let tile = me_tile(e, 0x38);
    e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, text, 1u32]);
    let recipe = e.global::<u32>(SELECTED_RECIPE_011D8E94);
    let category = e.call(RECIPE_CATEGORY_006A1B60, &args![recipe]).u32();
    let alpha = if me_tile(e, CATEGORY) == category {
        0xffu32
    } else {
        0x7f
    };
    let tile = me_tile(e, 0x38);
    e.call(TILE_SET_INT, &args![tile, 0xfa9u32, alpha]);
    let recipe = e.global::<u32>(SELECTED_RECIPE_011D8E94);
    let mut node = e.call(RECIPE_INPUTS_005A8080, &args![recipe]).u32();
    e.vcall(base + INGREDIENT_LIST, 0x1c, &args![]);
    let recipe = e.global::<u32>(SELECTED_RECIPE_011D8E94);
    let outputs = e.call(RECIPE_OUTPUTS_005D43C0, &args![recipe]).u32();
    *selected = node_item(e, outputs);
    let recipe = e.global::<u32>(SELECTED_RECIPE_011D8E94);
    let outputs = e.call(RECIPE_OUTPUTS_005D43C0, &args![recipe]).u32();
    let shows_output = e.call(LIST_COUNT_005AE380, &args![outputs]).u32() > 1 && {
        let recipe = e.global::<u32>(SELECTED_RECIPE_011D8E94);
        let inputs = e.call(RECIPE_INPUTS_005A8080, &args![recipe]).u32();
        e.call(LIST_COUNT_005AE380, &args![inputs]).u32() == 1
    };
    if shows_output {
        let recipe = e.global::<u32>(SELECTED_RECIPE_011D8E94);
        node = e.call(RECIPE_OUTPUTS_005D43C0, &args![recipe]).u32();
        let recipe = e.global::<u32>(SELECTED_RECIPE_011D8E94);
        let word = e.call(RECIPE_CATEGORY_KEY_006286D0, &args![recipe]).u32();
        let name = e.call(STRING_TEXT, &args![word + 0x18]).u32();
        let tile = me_tile(e, 0x54);
        e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, name, 1u32]);
        single_output = true;
    } else {
        set_setting_text(e, me_tile(e, 0x54), SETTING_011D3090);
    }
    e.with_stack(0x100, |e, buffer| {
        while node != 0 && !e.call(NODE_AT_END_008256D0, &args![node]).bool() {
            let item = node_item(e, node);
            let form = e.call(WORD_AT_FOUR, &args![item]).u32();
            let object = dynamic_cast(e, form, RTTI_01183158);
            if object != 0 && crafter != 0 {
                let item = node_item(e, node);
                let form = e.call(WORD_AT_FOUR, &args![item]).u32();
                let bound = dynamic_cast(e, form, RTTI_01183108);
                if bound != 0 {
                    let entry = e
                        .call(GET_INVENTORY_ITEM, &args![crafter, bound, 0u32])
                        .u32();
                    let have = if entry != 0 {
                        e.call(WORD_AT_FOUR, &args![entry]).u32()
                    } else {
                        0
                    };
                    if entry != 0 {
                        e.call(ITEM_DATA_DELETE, &args![entry, 1u32]);
                    }
                    if single_output {
                        let item = node_item(e, node);
                        let count = e.call(WORD_AT_ZERO, &args![item]).u32();
                        let name = e.call(STRING_TEXT, &args![object]).u32();
                        e.call(
                            FORMAT,
                            &args![buffer, 0x100u32, NAME_COUNT_FORMAT, name, count],
                        );
                    } else {
                        let item = node_item(e, node);
                        let count = e.call(WORD_AT_ZERO, &args![item]).u32();
                        let name = e.call(STRING_TEXT, &args![object]).u32();
                        e.call(
                            FORMAT,
                            &args![buffer, 0x100u32, HAVE_OF_NEED_FORMAT, name, have, count],
                        );
                    }
                    let item = node_item(e, node);
                    let tile = list_box_tes_recipe_component_add_item(
                        e,
                        Ptr::new(base + INGREDIENT_LIST),
                        item,
                        buffer.addr(),
                        0,
                        0,
                    );
                    let item = node_item(e, node);
                    let count = e.call(WORD_AT_ZERO, &args![item]).u32();
                    if have < count && !single_output {
                        set_item_alpha(e, tile, 0x7f);
                    } else {
                        set_item_alpha(e, tile, 0xff);
                    }
                }
            } else {
                let error_text = e
                    .call(ERROR_ITEM_00464F30, &args![ERROR_WITH_CAST_TEXT, 0u32])
                    .u32();
                let item = node_item(e, node);
                list_box_tes_recipe_component_add_item(
                    e,
                    Ptr::new(base + INGREDIENT_LIST),
                    item,
                    error_text,
                    0,
                    0,
                );
            }
            node = node_next(e, node);
        }
    });
    e.vcall(base + INGREDIENT_LIST, 0x20, &args![COMPARE_00728CD0]);
    if crafter != 0 {
        e.with_stack(0x100, |e, buffer| {
            e.mem.set_u8(buffer.addr(), 0);
            let recipe = e.global::<u32>(SELECTED_RECIPE_011D8E94);
            if e.call(RECIPE_SKILL_0059BB30, &args![recipe]).i32() > -1 {
                let recipe = e.global::<u32>(SELECTED_RECIPE_011D8E94);
                let level = e.call(RECIPE_SKILL_LEVEL_0045CD60, &args![recipe]).u32();
                let recipe = e.global::<u32>(SELECTED_RECIPE_011D8E94);
                let skill = e.call(RECIPE_SKILL_0059BB30, &args![recipe]).u32();
                let value = e
                    .call(GET_CLAMPED_ACTOR_VALUE, &args![crafter + 0xa4, skill])
                    .u32();
                let recipe = e.global::<u32>(SELECTED_RECIPE_011D8E94);
                let skill = e.call(RECIPE_SKILL_0059BB30, &args![recipe]).u32();
                let name = e.call(GET_ACTOR_VALUE_NAME, &args![skill]).u32();
                e.call(
                    FORMAT,
                    &args![buffer, 0x100u32, HAVE_OF_NEED_FORMAT, name, value, level],
                );
            }
            let tile = me_tile(e, 0x3c);
            e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, buffer, 1u32]);
        });
        let recipe = e.global::<u32>(SELECTED_RECIPE_011D8E94);
        let short = e.call(RECIPE_SKILL_0059BB30, &args![recipe]).i32() > -1 && {
            let recipe = e.global::<u32>(SELECTED_RECIPE_011D8E94);
            let skill = e.call(RECIPE_SKILL_0059BB30, &args![recipe]).u32();
            let value = e
                .call(GET_CLAMPED_ACTOR_VALUE, &args![crafter + 0xa4, skill])
                .i32();
            let recipe = e.global::<u32>(SELECTED_RECIPE_011D8E94);
            let level = e.call(RECIPE_SKILL_LEVEL_0045CD60, &args![recipe]).i32();
            value < level
        };
        let alpha = if short { 0x7fu32 } else { 0xff };
        let tile = me_tile(e, 0x3c);
        e.call(TILE_SET_INT, &args![tile, 0xfa9u32, alpha]);
    }
    let recipe = e.global::<u32>(SELECTED_RECIPE_011D8E94);
    let can_make = u32::from(fn_00727920(e, recipe) != 0);
    let tile = me_tile(e, 0x44);
    e.call(TILE_SET_INT, &args![tile, 0xfafu32, can_make]);
}

// Translated from 00728420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RecipeMenu::DoGamepad` (Xbox PDB; `RET 8`: a button code and an unused word). Only while
/// the menu is the top menu (`Interface::IsTopMenuID(0x435)`); the switch (byte map
/// `007284e4`, table `007284d4`): codes 3 and 4 click (vtable +0xc) id 0 for 4 and id 2 for
/// 3; code 13 enters (vtable +0x10) id 3 and clears the ingredient focus; code 14 enters
/// id 6 and sets it. Returns whether the code was handled.
pub fn recipe_menu_do_gamepad(e: &mut Engine, this: Ptr, code: u32, _unused_2: u32) -> bool {
    if !e
        .call(INTERFACE_IS_TOP_MENU_ID, &args![RECIPE_MENU_CLASS])
        .bool()
    {
        return false;
    }
    match code {
        3 | 4 => {
            let id = if code != 4 { 2u32 } else { 0 };
            e.vcall(this.addr(), 0xc, &args![id, 0u32]);
            true
        }
        13 => {
            e.vcall(this.addr(), 0x10, &args![3u32, 0u32]);
            e.set_global(INGREDIENTS_FOCUSED_011D8EA5, 0u8);
            true
        }
        14 => {
            e.vcall(this.addr(), 0x10, &args![6u32, 0u32]);
            e.set_global(INGREDIENTS_FOCUSED_011D8EA5, 1u8);
            true
        }
        _ => false,
    }
}

/// `MiscStatManager::ModVal` (`cdecl`, two words), `005f5950` (`cdecl`, seven words), the
/// form type (`00401170`), `Actor::GetCurrentWeapon` (`008a1710`),
/// `TESObjectWEAP::GetCurrentAmmo` (`00525980`, `RET 4`),
/// `Actor::GetItemCountinContainer` (`00575610`, `RET 4`), `Actor::UnEquipObject`
/// (`0088d7d0`, `RET 0x18`), `006ecd40` (`RET 4`), `004c0bf0` (`thiscall`, a `bool`),
/// `TESContainer::TESContainer` (`00481610`), its destructor (`00481680`), `004818e0`
/// (`RET 0xc`), `00482090` (`RET 4`, a `float`), `004821a0` (`RET 8`), `00574fa0`
/// (`RET 0xc`), `TESFullName::GetFullName` (`00482720`, `cdecl`), `Actor::GetPickUpSoundName`
/// (`008adcf0`, `RET 0xc`), `007052f0` (`cdecl`, six words: the message).
const MISC_STAT_MOD_VAL: u32 = 0x004d_5e10;
const RECIPE_EVENT_005F5950: u32 = 0x005f_5950;
const FORM_TYPE: u32 = 0x0040_1170;
const GET_CURRENT_WEAPON: u32 = 0x008a_1710;
const GET_CURRENT_AMMO: u32 = 0x0052_5980;
const GET_ITEM_COUNT_IN_CONTAINER: u32 = 0x0057_5610;
const UNEQUIP_OBJECT: u32 = 0x0088_d7d0;
const FORM_004C0BF0: u32 = 0x004c_0bf0;
const CONTAINER_CONSTRUCT: u32 = 0x0048_1610;
const CONTAINER_DESTRUCT: u32 = 0x0048_1680;
const CONTAINER_ADD_004818E0: u32 = 0x0048_18e0;
const CONTAINER_SCALE_00482090: u32 = 0x0048_2090;
const CONTAINER_GIVE_004821A0: u32 = 0x0048_21a0;
const ADD_ITEM_00574FA0: u32 = 0x0057_4fa0;
const GET_FULL_NAME: u32 = 0x0048_2720;
const GET_PICK_UP_SOUND_NAME: u32 = 0x008a_dcf0;
const SHOW_MESSAGE_007052F0: u32 = 0x0070_52f0;
/// The `float`s `01030ff0` (given to `00482090`) and `010162c0` (the message's
/// time), the settings `011d4e7c` and `011d3114`, the formats `"%i %s%s %s"` and
/// `"%s %s"`, the gift box icon path.
const CONTAINER_SCALE_01030FF0: u32 = 0x0103_0ff0;
const MESSAGE_TIME_010162C0: u32 = 0x0101_62c0;
const SETTING_011D4E7C: u32 = 0x011d_4e7c;
const SETTING_011D3114: u32 = 0x011d_3114;
const MANY_ADDED_FORMAT: u32 = 0x0101_c178;
const TWO_STRINGS_FORMAT: u32 = 0x0101_2058;
const GIFT_ICON_TEXT: u32 = 0x0101_c140;

// Translated from 007284f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007284f0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 around the TESContainer and BSString locals) is not translated.
/// Unnamed (`cdecl`, a count; the quantity menu's callback for the recipe menu): makes the
/// selected recipe `count` times. Does nothing for 0. Takes away the inputs: the first
/// input's form goes to `005f5950(0xc, 1, form, 0, 0, 0, 0)` (a form of type 0x6c skips the
/// stat below); for each input, the last form that is one (vtable +0xe4) is removed from
/// the crafter (vtable +0x17c, its count times `count`), unequipping a single equipped
/// weapon first and, when the crafter's weapon's ammo was the form, restarting its
/// process's attack (`006ecd40`). Then `MiscStatManager::ModVal(0x20, count)` and, for
/// each output, the form is given to the crafter (through a `TESContainer` for weapons not
/// of `004c0bf0` and for armour, types 0x18 and 0x1a; else `00574fa0`) and a message with
/// the gift box icon is shown. Closes the menu.
pub fn fn_007284f0(e: &mut Engine, count: i32) {
    let current = me_tile(e, CURRENT_LIST);
    e.call(
        LIST_BOX_GET_TILE_FOR,
        &args![current, SELECTED_RECIPE_011D8E94],
    );
    let recipe = e.global::<u32>(SELECTED_RECIPE_011D8E94);
    let mut node = e.call(RECIPE_INPUTS_005A8080, &args![recipe]).u32();
    let mut chosen = 0u32;
    if count == 0 {
        return;
    }
    let mut seen_first = false;
    let mut skip_stat = false;
    while node != 0 && !e.call(NODE_AT_END_008256D0, &args![node]).bool() {
        let item = node_item(e, node);
        let form = e.call(WORD_AT_FOUR, &args![item]).u32();
        if !seen_first {
            e.call(
                RECIPE_EVENT_005F5950,
                &args![0xcu32, 1u32, form, 0u32, 0u32, 0u32, 0u32],
            );
            if e.call(FORM_TYPE, &args![form]).u32() == 0x6c {
                skip_stat = true;
            }
        }
        seen_first = true;
        let item = node_item(e, node);
        let needed = e.call(WORD_AT_ZERO, &args![item]).i32();
        if form != 0 && e.vcall(form, 0xe4, &args![]).bool() {
            chosen = form;
        }
        if chosen != 0 {
            let crafter = accepted_crafter(e);
            let mut was_ammo = false;
            if crafter != 0 && e.call(GET_CURRENT_WEAPON, &args![crafter]).u32() != 0 {
                let player = e.global::<u32>(PLAYER);
                let weapon = e.call(GET_CURRENT_WEAPON, &args![crafter]).u32();
                let ammo = e.call(GET_CURRENT_AMMO, &args![weapon, player]).u32();
                let item = node_item(e, node);
                let form = e.call(WORD_AT_FOUR, &args![item]).u32();
                was_ammo = ammo == form;
            }
            if e.call(FORM_TYPE, &args![chosen]).u32() == 0x28
                && e.call(GET_ITEM_COUNT_IN_CONTAINER, &args![crafter, chosen])
                    .u32()
                    == 1
                && chosen == e.call(GET_CURRENT_WEAPON, &args![crafter]).u32()
            {
                e.call(
                    UNEQUIP_OBJECT,
                    &args![crafter, chosen, 1u32, 0u32, 0u32, 0u32, 1u32],
                );
            }
            let owner = me_tile(e, CRAFTER_REF);
            e.vcall(
                owner,
                0x17c,
                &args![
                    chosen,
                    0u32,
                    needed.wrapping_mul(count),
                    0u32,
                    0u32,
                    0u32,
                    0u32,
                    0u32,
                    1u32,
                    0u32
                ],
            );
            if was_ammo {
                let process = e.call(GET_PROCESS, &args![crafter]).u32();
                let target = e.vcall(process, 0x14c, &args![]).u32();
                e.call(NODE_SET_NEXT, &args![target, 0u32]);
            }
        }
        node = node_next(e, node);
    }
    if !skip_stat {
        e.call(MISC_STAT_MOD_VAL, &args![0x20u32, count]);
    }
    let recipe = e.global::<u32>(SELECTED_RECIPE_011D8E94);
    let mut node = e.call(RECIPE_OUTPUTS_005D43C0, &args![recipe]).u32();
    let mut chosen = 0u32;
    while node != 0 && !e.call(NODE_AT_END_008256D0, &args![node]).bool() {
        let item = node_item(e, node);
        let form = e.call(WORD_AT_FOUR, &args![item]).u32();
        let item = node_item(e, node);
        let made = e.call(WORD_AT_ZERO, &args![item]).i32();
        if form != 0 && e.vcall(form, 0xe4, &args![]).bool() {
            chosen = form;
        }
        if chosen != 0 {
            let total = made.wrapping_mul(count);
            let kind = e.call(FORM_TYPE, &args![chosen]).u32();
            let through_container = if kind == 0x28 {
                !e.call(FORM_004C0BF0, &args![chosen]).bool()
            } else {
                e.call(FORM_TYPE, &args![chosen]).u32() == 0x18
                    || e.call(FORM_TYPE, &args![chosen]).u32() == 0x1a
            };
            if through_container {
                e.with_stack(0xc, |e, container| {
                    e.call(CONTAINER_CONSTRUCT, &args![container]);
                    if chosen != 0 {
                        e.call(
                            CONTAINER_ADD_004818E0,
                            &args![container, chosen, total, 0u32],
                        );
                    }
                    let value = x87_quiet_bits(e.global::<u32>(CONTAINER_SCALE_01030FF0));
                    e.call(CONTAINER_SCALE_00482090, &args![container, value]);
                    let crafter = me_tile(e, CRAFTER_REF);
                    e.call(CONTAINER_GIVE_004821A0, &args![container, crafter, 0u32]);
                    e.call(CONTAINER_DESTRUCT, &args![container]);
                });
            } else {
                let crafter = me_tile(e, CRAFTER_REF);
                e.call(ADD_ITEM_00574FA0, &args![crafter, chosen, 0u32, total]);
            }
            e.with_stack(8, |e, text| {
                e.call(STRING_OBJECT_CONSTRUCT, &args![text]);
                if total > 1 {
                    let first = e.call(SETTING_STRING, &args![SETTING_011D4E7C]).u32();
                    let second = e.call(SETTING_STRING, &args![SETTING_011D3114]).u32();
                    let name = e.call(GET_FULL_NAME, &args![chosen]).u32();
                    e.call(
                        FORMAT_STRING_OBJECT,
                        &args![text, MANY_ADDED_FORMAT, total, name, second, first],
                    );
                } else {
                    let first = e.call(SETTING_STRING, &args![SETTING_011D4E7C]).u32();
                    let name = e.call(GET_FULL_NAME, &args![chosen]).u32();
                    e.call(
                        FORMAT_STRING_OBJECT,
                        &args![text, TWO_STRINGS_FORMAT, name, first],
                    );
                }
                let time = x87_quiet_bits(e.global::<u32>(MESSAGE_TIME_010162C0));
                let player = e.global::<u32>(PLAYER);
                let sound = e
                    .call(GET_PICK_UP_SOUND_NAME, &args![player, chosen, 1u32, 0u32])
                    .u32();
                let message = e.call(WORD_AT_ZERO, &args![text]).u32();
                e.call(
                    SHOW_MESSAGE_007052F0,
                    &args![message, 0u32, GIFT_ICON_TEXT, sound, time, 0u32],
                );
                e.call(STRING_OBJECT_DESTRUCT, &args![text]);
            });
        }
        node = node_next(e, node);
    }
    recipe_menu_close(e);
}

// Translated from 00728a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, a byte): with the byte set, refills the open menu's recipe list
/// (`fn_00727680(pMe, 0, 0)`) and clears the refresh request `011d8ea4`; otherwise sets
/// the request.
pub fn fn_00728a30(e: &mut Engine, now: u8) {
    if now != 0 {
        if me(e) != 0 {
            let menu = me(e);
            fn_00727680(e, Ptr::new(menu), 0, 0);
        }
        e.set_global(REFRESH_011D8EA4, 0u8);
    } else {
        e.set_global(REFRESH_011D8EA4, 1u8);
    }
}

/// `005330e0` (`cdecl`: a `float` pointer and two `float`s): keeps the value between them.
const CLAMP_005330E0: u32 = 0x0053_30e0;
/// `00825c00` (`thiscall` on `011f6394`): the current time.
const TIMER_011F6394: u32 = 0x011f_6394;
const TIMER_NOW_00825C00: u32 = 0x0082_5c00;
/// `DoIdle`'s `float`s: the fade's upper (`0106f49c`) and lower (`0106f498`) limits, its
/// rate (`0101622c`), the clamp limit `01023cd8`, and the `double` `0101e568`.
const FADE_UPPER: u32 = 0x0106_f49c;
const FADE_LOWER: u32 = 0x0106_f498;
const FADE_RATE: u32 = 0x0101_622c;
const ALPHA_LIMIT_01023CD8: u32 = 0x0102_3cd8;
const ALPHA_TOTAL_0101E568: u32 = 0x0101_e568;

// Translated from 00728a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RecipeMenu::DoIdle` (Xbox PDB): a pending refresh request runs (`fn_00728a30(1)`); while
/// `0119fbf0` is set, cross-fades the two tiles `011d8eb0` and `011d8eac` (alpha `0xfa9`:
/// the value clamped to `0..01023cd8`, and `0101e568` minus it, clamped the same way),
/// then moves the value by the elapsed time (`00825c00` minus the last time) times the
/// rate, down or up as `011d8ea6` says, clamps it to the limits and turns the direction
/// down at or above the upper limit and up at or below the lower one.
pub fn recipe_menu_do_idle(e: &mut Engine, _this: Ptr) {
    if e.global::<u8>(REFRESH_011D8EA4) == 1 {
        fn_00728a30(e, 1);
    }
    if e.global::<u8>(FADING_0119FBF0) == 0 {
        return;
    }
    let upper = x87_quiet_bits(e.global::<u32>(FADE_UPPER));
    let lower = x87_quiet_bits(e.global::<u32>(FADE_LOWER));
    let rate = x87_quiet_bits(e.global::<u32>(FADE_RATE));
    let value = x87_quiet_bits(e.global::<u32>(FADE_VALUE_011D8EB8));
    let (first, second) = e.with_stack(8, |e, slots| {
        let limit = x87_quiet_bits(e.global::<u32>(ALPHA_LIMIT_01023CD8));
        e.mem.set_u32(slots.addr(), value);
        e.call(CLAMP_005330E0, &args![slots, 0.0f32.to_bits(), limit]);
        let first = e.mem.u32(slots.addr());
        let total: f64 = e.global(ALPHA_TOTAL_0101E568);
        let other = x87_store32(x87_sub(total, x87_load32(first)));
        e.mem.set_u32(slots.addr() + 4, other);
        let limit = x87_quiet_bits(e.global::<u32>(ALPHA_LIMIT_01023CD8));
        e.call(
            CLAMP_005330E0,
            &args![slots.addr() + 4, 0.0f32.to_bits(), limit],
        );
        (first, e.mem.u32(slots.addr() + 4))
    });
    if e.global::<u32>(FADE_TILE_011D8EB0) != 0 && e.global::<u32>(FADE_TILE_011D8EAC) != 0 {
        let tile = e.global::<u32>(FADE_TILE_011D8EB0);
        e.call(
            TILE_SET_FLOAT,
            &args![tile, 0xfa9u32, x87_quiet_bits(second), 1u32],
        );
        let tile = e.global::<u32>(FADE_TILE_011D8EAC);
        e.call(
            TILE_SET_FLOAT,
            &args![tile, 0xfa9u32, x87_quiet_bits(first), 1u32],
        );
    }
    let now = e.call(TIMER_NOW_00825C00, &args![TIMER_011F6394]).u32();
    let elapsed = now.wrapping_sub(e.global::<u32>(LAST_TIME_011D8EBC));
    e.set_global(LAST_TIME_011D8EBC, now);
    let step = x87_mul(f64::from(elapsed), x87_m32(rate));
    let current = x87_m32(e.global::<u32>(FADE_VALUE_011D8EB8));
    let moved = if e.global::<u8>(FADE_DOWN_011D8EA6) != 0 {
        x87_sub(current, step)
    } else {
        x87_add(step, current)
    };
    e.set_global(FADE_VALUE_011D8EB8, x87_store32(moved));
    e.call(CLAMP_005330E0, &args![FADE_VALUE_011D8EB8, lower, upper]);
    // `FLD value; FLD upper; FCOMPP; TEST AH,0x41; JP`: on when the upper limit is at or
    // below the value (ordered).
    let value = x87_load32(e.global::<u32>(FADE_VALUE_011D8EB8));
    if x87_load32(upper) <= value {
        e.set_global(FADE_DOWN_011D8EA6, 1u8);
    }
    // `FLD value; FLD lower; FCOMPP; TEST AH,1; JNZ`: on when the lower limit is not below
    // the value and they are ordered.
    let value = x87_load32(e.global::<u32>(FADE_VALUE_011D8EB8));
    if x87_load32(lower) >= value {
        e.set_global(FADE_DOWN_011D8EA6, 0u8);
    }
}

// Translated from 00728c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`: two list items; the recipe list's sort order): recipes that can be
/// made (`fn_00727920` above 0) come first; within each group, by name (+0x18 of the item's
/// data, `00469880`).
pub fn fn_00728c10(e: &mut Engine, first: u32, second: u32) -> i32 {
    let a = e.mem.u32(first + 4);
    let a_name = e.call(STRING_TEXT, &args![a + 0x18]).u32();
    let b = e.mem.u32(second + 4);
    let b_name = e.call(STRING_TEXT, &args![b + 0x18]).u32();
    let a = e.mem.u32(first + 4);
    let a_count = fn_00727920(e, a) as i32;
    let b = e.mem.u32(second + 4);
    let b_count = fn_00727920(e, b) as i32;
    if (a_count > 0 && b_count > 0) || (a_count <= 0 && b_count <= 0) {
        e.call(COMPARE_STRINGS, &args![a_name, b_name]).i32()
    } else if a_count > 0 {
        -1
    } else if b_count > 0 {
        1
    } else {
        // Not reachable (one of the two counts is above 0 here); the code compares again.
        e.call(COMPARE_STRINGS, &args![a_name, b_name]).i32()
    }
}

// Translated from 00728cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`: two list items; the ingredient list's sort order): the names of the
/// items' forms (the word at +4 of each item's data, cast with `01183158`), compared with
/// `00469880`.
pub fn fn_00728cd0(e: &mut Engine, first: u32, second: u32) -> i32 {
    let a = e.mem.u32(first + 4);
    let a = e.call(WORD_AT_FOUR, &args![a]).u32();
    let a = dynamic_cast(e, a, RTTI_01183158);
    let b = e.mem.u32(second + 4);
    let b = e.call(WORD_AT_FOUR, &args![b]).u32();
    let b = dynamic_cast(e, b, RTTI_01183158);
    let a_name = e.call(STRING_TEXT, &args![a]).u32();
    let b_name = e.call(STRING_TEXT, &args![b]).u32();
    e.call(COMPARE_STRINGS, &args![a_name, b_name]).i32()
}

// Translated from 00728d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, a recipe; the recipe list's filter): false while the category filter
/// is -1; otherwise whether the recipe's category word (`006286d0`) differs from the
/// filter's category (the category array's element).
pub fn fn_00728d60(e: &mut Engine, recipe: u32) -> bool {
    if e.global::<u32>(FILTER_INDEX_0119FBF4) == 0xffff_ffff {
        return false;
    }
    let word = e.call(RECIPE_CATEGORY_KEY_006286D0, &args![recipe]).u32();
    let index = e.global::<u32>(FILTER_INDEX_0119FBF4);
    let array = fn_00726f10(e);
    let entry = e.call(ARRAY_AT, &args![array, index]).u32();
    word != e.mem.u32(entry)
}
/// `TESValueForm::GetFormValue` (`0048e8a0`, `cdecl`, one word), `00647c00` (`cdecl`: a
/// `float` and `1.0`, `ST0`), `004d1360` (`thiscall` on the player, `AL`),
/// `TESWeightForm::GetFormWeight` (`0048ebc0`, `cdecl`, two words, `ST0`), the tile array
/// iterator `00683520` (`RET 4`), `00663b60` (`thiscall`), `00645380` (`cdecl`, thirteen
/// words, `ST0`), `CombatFormulas::CalcWeaponDamageForDisplay` (`006450f0`, `cdecl`, four
/// words, `ST0`), `TESObjectWEAP::GetNumProjectiles` (`00525b20`, `RET 0xc`, `AL`),
/// `00663b40` (`thiscall`), `InventoryChanges::GetInventoryChanges` (`004bf220`, `cdecl`),
/// `InventoryChanges::GetObjectCount` (`004c8f30`, `RET 4`),
/// `TESObjectWEAP::GetAmmoRegenRate` (`00709430`, `RET 4`, `ST0`), `004be060` (`thiscall`,
/// `ST0`), `CombatFormulas::CalcArmorRating` (`00646360`, `cdecl`: a word and a `float`,
/// `ST0`), `00476b20` (`cdecl`, a `float`, `ST0`), `TESEnchantableForm::GetFormEnchanting`
/// (`004be330`, `cdecl`), `0040e210` (`RET 4`, a `bool`),
/// `EffectItemList::BuildMenuString` (`00406620`, `RET 8`).
const GET_FORM_VALUE: u32 = 0x0048_e8a0;
const VALUE_00647C00: u32 = 0x0064_7c00;
const PLAYER_004D1360: u32 = 0x004d_1360;
const GET_FORM_WEIGHT: u32 = 0x0048_ebc0;
const TILE_ARRAY_STEP_00683520: u32 = 0x0068_3520;
const WEAPON_00663B60: u32 = 0x0066_3b60;
const WEAPON_DAMAGE_00645380: u32 = 0x0064_5380;
const CALC_WEAPON_DAMAGE_FOR_DISPLAY: u32 = 0x0064_50f0;
const GET_NUM_PROJECTILES: u32 = 0x0052_5b20;
const AMMO_NAME_00663B40: u32 = 0x0066_3b40;
const GET_INVENTORY_CHANGES: u32 = 0x004b_f220;
const GET_OBJECT_COUNT: u32 = 0x004c_8f30;
const GET_AMMO_REGEN_RATE: u32 = 0x0070_9430;
const ARMOR_004BE060: u32 = 0x004b_e060;
const CALC_ARMOR_RATING: u32 = 0x0064_6360;
const ROUND_00476B20: u32 = 0x0047_6b20;
const GET_FORM_ENCHANTING: u32 = 0x004b_e330;
const FORM_FLAG_0040E210: u32 = 0x0040_e210;
const BUILD_MENU_STRING: u32 = 0x0040_6620;
/// `"%d"`, `"%.1f"`, `"%.0f"`, `"%.1fx%d"`, `"%s (%i/%i)"`, `"--"`.
const INT_FORMAT: u32 = 0x0102_0764;
const ONE_DECIMAL_FORMAT: u32 = 0x0102_0768;
const NO_DECIMAL_FORMAT: u32 = 0x0106_ecc8;
const PER_PROJECTILE_FORMAT: u32 = 0x0106_ecf4;
const AMMO_COUNT_FORMAT: u32 = 0x0106_ece8;
const DASHES_TEXT: u32 = 0x0104_7008;
/// The `double`s `0.0`, `1.0` and `2.0`.
const TWO_DOUBLE: u32 = 0x0101_1590;
/// The settings whose strings title the item tiles.
const ITEM_TITLE_SETTINGS: [(usize, u32); 9] = [
    (0, 0x011d_20c4),
    (1, 0x011d_412c),
    (11, 0x011d_2c7c),
    (2, 0x011d_2aa8),
    (3, 0x011d_26a0),
    (4, 0x011d_4da4),
    (6, 0x011d_4edc),
    (7, 0x011d_4024),
    (10, 0x011d_330c),
];
/// The tiles whose visibility (trait `0xfa3`) follows a bit of the flags, in the code's
/// order.
const ITEM_TILE_BITS: [(usize, u32); 11] = [
    (0, 0x1),
    (1, 0x2),
    (11, 0x800),
    (2, 0x4),
    (3, 0x8),
    (4, 0x10),
    (5, 0x20),
    (6, 0x40),
    (7, 0x80),
    (8, 0x100),
    (9, 0x200),
];

/// A `float` (its bits) passed as a `double` (`FLD float; FSTP double`): the two words.
fn double_words(bits: u32) -> [u32; 2] {
    let value = x87_load32(bits).to_bits();
    [value as u32, (value >> 32) as u32]
}

/// `fn_00728da0`'s value and weight text: `"--"` at or below `0.0` (or NaN), `"%.1f"` below
/// `1.0`, else `"%.0f"`, into the 16-byte buffer, then shown in `tile` (the value trait).
fn show_amount(e: &mut Engine, buffer: Ptr, bits: u32, tile: u32, value_trait: u32) {
    let value = x87_load32(bits);
    let zero: f64 = e.global(ZERO_DOUBLE);
    let one: f64 = e.global(ONE_DOUBLE);
    if !above(value, zero) {
        e.call(COPY, &args![buffer, 0x10u32, DASHES_TEXT]);
    } else if matches!(
        value.partial_cmp(&one),
        Some(std::cmp::Ordering::Less) | None
    ) {
        let [low, high] = double_words(bits);
        e.call(
            FORMAT,
            &args![buffer, 0x10u32, ONE_DECIMAL_FORMAT, low, high],
        );
    } else {
        let [low, high] = double_words(bits);
        e.call(
            FORMAT,
            &args![buffer, 0x10u32, NO_DECIMAL_FORMAT, low, high],
        );
    }
    e.call(TILE_SET_STRING, &args![tile, value_trait, buffer, 1u32]);
}

// Translated from 00728da0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00728da0 - the /GS security cookie check (00ec408c) is not translated
/// Unnamed (`thiscall`, `RET 8`: a form and the item tile holder; the recipe menu): fills
/// the item information tiles. Looks up `"_Title"` and `"_Value"`, computes the form's value
/// (`00647c00(GetFormValue, 1.0)`) and weight. With a form and a holder: takes up to twelve
/// tiles from the holder's array (+4; the rest 0). Armour (types 0x18 and 0x1a): the armour
/// rating in tile 0 (flags 0x1d). Weapons (0x28): the damage per second in tile 1 (kept in
/// `011d8eac`), the damage (or per projectile with `"%.1fx%d"`) in tile 11 (kept in
/// `011d8eb0`), `0040ebd0(1.0, 1.0)` as trait `0x1009` of tile 4, `00663b60`'s number in tile
/// 10 when there is one (kept in `011d8eb4`), the ammunition (`"--"`, its name, or
/// `"%s (%i/%i)"` with the player's count) as the title of tile 5 (flags 0xc3e). Anything
/// else: flags 0xc. The effects text (an enchantment's, an ingestible's or a book-like
/// form's list) goes to tile 6 (flag 0x40). The value and weight go to tiles 3 and 2; nine
/// tiles get their titles; eleven (twelve with tile 10, which also gets alpha 255) are shown
/// by the flags; the four tiles from 6 get their y (the holder's height / 2 - 20, doubled
/// with flag 0x10 or 0x20).
pub fn fn_00728da0(e: &mut Engine, _this: Ptr, form: u32, holder: u32) {
    let title_trait = e.call(TEXT_TO_TRAIT, &args![TITLE_TEXT]).u32();
    let value_trait = e.call(TEXT_TO_TRAIT, &args![VALUE_TEXT]).u32();
    let damage_trait = 0x1009u32;
    let offset_a = 0.0f32.to_bits();
    let offset_b = 0.0f32.to_bits();
    let price = e.call(GET_FORM_VALUE, &args![form]).i32();
    let value = e
        .call(
            VALUE_00647C00,
            &args![x87_store32(f64::from(price)), 1.0f32.to_bits()],
        )
        .f64();
    let value = x87_store32(value);
    let player = e.global::<u32>(PLAYER);
    let flag = e.call(PLAYER_004D1360, &args![player]).u8();
    let weight = e.call(GET_FORM_WEIGHT, &args![form, u32::from(flag)]).f64();
    let weight = x87_store32(weight);
    if form == 0 || holder == 0 {
        return;
    }
    let mut tiles = [0u32; 12];
    let mut index = 0usize;
    e.with_stack(4, |e, cursor| {
        let first = e.call(WORD_AT_FOUR, &args![holder + 4]).u32();
        e.mem.set_u32(cursor.addr(), first);
        while e.mem.u32(cursor.addr()) != 0 && index < 0xc {
            let slot = e
                .call(TILE_ARRAY_STEP_00683520, &args![holder + 4, cursor])
                .u32();
            tiles[index] = e.mem.u32(slot);
            index += 1;
        }
    });
    let kind = e.call(FORM_TYPE, &args![form]).u32();
    let mut flags = if kind == 0x18 || kind == 0x1a {
        let base = e.call(ARMOR_004BE060, &args![form]).f64();
        let base = e.call(FTOL2, &args![base]).u32();
        let rating = e
            .call(CALC_ARMOR_RATING, &args![base, 1.0f32.to_bits()])
            .f64();
        let rounded = e.call(ROUND_00476B20, &args![x87_store32(rating)]).f64();
        let whole = e.call(FTOL2, &args![rounded]).u32();
        e.with_stack(0x80, |e, buffer| {
            e.call(FORMAT, &args![buffer, 0x80u32, INT_FORMAT, whole]);
            e.call(TILE_SET_STRING, &args![tiles[0], value_trait, buffer, 1u32]);
        });
        0x1d
    } else if kind == 0x28 {
        show_weapon(e, form, &tiles, [title_trait, value_trait, damage_trait]);
        0xc3e
    } else {
        0xc
    };
    let mut effects = 0u32;
    let object = dynamic_cast(e, form, RTTI_01183108);
    let enchantment = e.call(GET_FORM_ENCHANTING, &args![object]).u32();
    if enchantment != 0 && !e.call(FORM_FLAG_0040E210, &args![enchantment, 4u32]).bool() {
        effects = if enchantment != 0 {
            enchantment + 0x24
        } else {
            0
        };
    }
    if e.call(FORM_TYPE, &args![form]).u32() == 0x1d {
        effects = if form != 0 { form + 0x3c } else { 0 };
    } else if e.call(FORM_TYPE, &args![form]).u32() == 0x2f {
        effects = if object != 0 { object + 0x3c } else { 0 };
    } else {
        let enchantment = e.call(GET_FORM_ENCHANTING, &args![form]).u32();
        if enchantment != 0 && !e.call(FORM_FLAG_0040E210, &args![enchantment, 4u32]).bool() {
            effects = if enchantment != 0 {
                enchantment + 0x24
            } else {
                0
            };
        }
    }
    if effects != 0 && node_item(e, effects + 4) != 0 {
        e.with_stack(0x100, |e, buffer| {
            e.call(BUILD_MENU_STRING, &args![effects, buffer, 0x100u32]);
            if e.call(STRING_LENGTH, &args![buffer]).u32() != 0 {
                e.call(TILE_SET_STRING, &args![tiles[6], value_trait, buffer, 1u32]);
                flags |= 0x40;
            } else {
                flags &= !0x40;
            }
        });
    } else {
        flags &= !0x40;
    }
    e.with_stack(0x10, |e, buffer| {
        show_amount(e, buffer, value, tiles[3], value_trait);
        show_amount(e, buffer, weight, tiles[2], value_trait);
    });
    for (tile, setting) in ITEM_TITLE_SETTINGS {
        let text = e.call(SETTING_STRING, &args![setting]).u32();
        e.call(
            TILE_SET_STRING,
            &args![tiles[tile], title_trait, text, 1u32],
        );
    }
    for (tile, bit) in ITEM_TILE_BITS {
        e.call(
            TILE_SET_INT,
            &args![tiles[tile], 0xfa3u32, u32::from(flags & bit != 0)],
        );
    }
    if tiles[10] != 0 {
        e.call(
            TILE_SET_INT,
            &args![tiles[10], 0xfa3u32, u32::from(flags & 0x400 != 0)],
        );
        let alpha = x87_quiet_bits(e.global::<u32>(ALPHA_LIMIT_01023CD8));
        e.call(TILE_SET_FLOAT, &args![tiles[10], 0xfa9u32, alpha, 1u32]);
    }
    let height = e.call(TILE_GET_FLOAT, &args![holder, 0xfb0u32]).f64();
    let half = x87_div(height, e.global::<f64>(TWO_DOUBLE));
    let mut y = x87_store32(x87_sub(half, e.global::<f64>(TWENTY_DOUBLE)));
    if flags & 0x10 != 0 || flags & 0x20 != 0 {
        y = x87_store32(x87_add(x87_load32(y), x87_load32(y)));
    }
    for tile in [tiles[6], tiles[7]] {
        e.call(
            TILE_SET_FLOAT,
            &args![tile, 0xfa2u32, x87_quiet_bits(y), 1u32],
        );
    }
    for (tile, offset) in [(tiles[8], offset_a), (tiles[9], offset_b)] {
        let moved = x87_store32(x87_add(x87_load32(y), x87_m32(offset)));
        e.call(
            TILE_SET_FLOAT,
            &args![tile, 0xfa2u32, x87_quiet_bits(moved), 1u32],
        );
    }
}

/// `fn_00728da0`'s weapon part (`traits`: title, value, the `0x1009` damage trait).
fn show_weapon(e: &mut Engine, weapon: u32, tiles: &[u32; 12], traits: [u32; 3]) {
    let [title_trait, value_trait, damage_trait] = traits;
    let number = e.call(WEAPON_00663B60, &args![weapon]).u32();
    let multiplier = 1.0f32.to_bits();
    let player = e.global::<u32>(PLAYER);
    let owner = if player != 0 { player + 0xa4 } else { 0 };
    let per_second = e
        .call(
            WEAPON_DAMAGE_00645380,
            &args![
                owner,
                weapon,
                multiplier,
                1u32,
                0u32,
                0u32,
                0u32,
                0xffff_ffffu32,
                0.0f32.to_bits(),
                0.0f32.to_bits(),
                0u32,
                0u32,
                0u32
            ],
        )
        .f64();
    let per_second = x87_store32(per_second);
    // `FCOMP double 0.0; TEST AH,0x41; JP`: the damage only when the multiplier is above
    // `0.0` (or NaN).
    let zero: f64 = e.global(ZERO_DOUBLE);
    let damage = if matches!(
        x87_load32(multiplier).partial_cmp(&zero),
        Some(std::cmp::Ordering::Greater) | None
    ) {
        let damage = e
            .call(
                CALC_WEAPON_DAMAGE_FOR_DISPLAY,
                &args![weapon, multiplier, 0u32, 0u32],
            )
            .f64();
        x87_store32(damage)
    } else {
        0.0f32.to_bits()
    };
    e.with_stack(0x80, |e, buffer| {
        let rounded = e
            .call(
                SCROLL_ROUND_004BD510,
                &args![x87_quiet_bits(per_second), 1.0f32.to_bits()],
            )
            .f64();
        let whole = e.call(FTOL2, &args![rounded]).u32();
        e.call(FORMAT, &args![buffer, 0x80u32, INT_FORMAT, whole]);
        e.call(TILE_SET_STRING, &args![tiles[1], value_trait, buffer, 1u32]);
        e.set_global(FADE_TILE_011D8EAC, tiles[1]);
        let projectiles = u32::from(
            e.call(GET_NUM_PROJECTILES, &args![weapon, 0u32, 0u32, 0u32])
                .u8(),
        );
        if projectiles > 1 {
            let each = x87_store32(x87_div(x87_m32(damage), f64::from(projectiles)));
            let [low, high] = double_words(each);
            e.call(
                FORMAT,
                &args![
                    buffer,
                    0x80u32,
                    PER_PROJECTILE_FORMAT,
                    low,
                    high,
                    projectiles
                ],
            );
        } else {
            let rounded = e
                .call(
                    SCROLL_ROUND_004BD510,
                    &args![x87_quiet_bits(damage), 1.0f32.to_bits()],
                )
                .f64();
            let whole = e.call(FTOL2, &args![rounded]).u32();
            e.call(FORMAT, &args![buffer, 0x80u32, INT_FORMAT, whole]);
        }
        e.call(
            TILE_SET_STRING,
            &args![tiles[11], value_trait, buffer, 1u32],
        );
        e.set_global(FADE_TILE_011D8EB0, tiles[11]);
        let scale = e
            .call(
                FLOAT_PAIR_0040EBD0,
                &args![x87_quiet_bits(multiplier), 1.0f32.to_bits()],
            )
            .f64();
        e.call(
            TILE_SET_FLOAT,
            &args![tiles[4], damage_trait, x87_store32(scale), 1u32],
        );
        e.call(FORMAT, &args![buffer, 0x80u32, INT_FORMAT, number]);
        if tiles[10] != 0 {
            e.call(
                TILE_SET_STRING,
                &args![tiles[10], value_trait, buffer, 1u32],
            );
        }
        e.set_global(TILE_011D8EB4, tiles[10]);
        if e.call(GET_CURRENT_AMMO, &args![weapon, 0u32]).u32() != 0 {
            let player = e.global::<u32>(PLAYER);
            let process = e.call(GET_PROCESS, &args![player]).u32();
            e.vcall(process, 0x148, &args![]);
            let player = e.global::<u32>(PLAYER);
            let process = e.call(GET_PROCESS, &args![player]).u32();
            e.vcall(process, 0x14c, &args![]);
            let ammo = e.call(GET_CURRENT_AMMO, &args![weapon, 0u32]).u32();
            let mut name = e.call(AMMO_NAME_00663B40, &args![ammo]).u32();
            let ammo = e.call(GET_CURRENT_AMMO, &args![weapon, 0u32]).u32();
            let player = e.global::<u32>(PLAYER);
            let changes = e.call(GET_INVENTORY_CHANGES, &args![player]).u32();
            let count = e.call(GET_OBJECT_COUNT, &args![changes, ammo]).u32();
            let loaded = 0u32;
            if name == 0 || e.mem.u8(name) == 0 {
                let ammo = e.call(GET_CURRENT_AMMO, &args![weapon, 0u32]).u32();
                name = e.call(STRING_TEXT, &args![ammo + 0x30]).u32();
            }
            let rate = e.call(GET_AMMO_REGEN_RATE, &args![weapon, 0u32]).f64();
            // `FCOMP double 0.0; TEST AH,0x44; JP`: regenerating unless equal to `0.0`.
            if rate != zero {
                e.call(FORMAT, &args![buffer, 0x80u32, ONE_STRING_FORMAT, name]);
            } else {
                e.call(
                    FORMAT,
                    &args![
                        buffer,
                        0x80u32,
                        AMMO_COUNT_FORMAT,
                        name,
                        loaded,
                        count.wrapping_sub(loaded)
                    ],
                );
            }
        } else {
            e.call(COPY, &args![buffer, 0x80u32, DASHES_TEXT]);
        }
        e.call(TILE_SET_STRING, &args![tiles[5], title_trait, buffer, 1u32]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0072ba80, fn_0072ba80(Ptr, u32, u32)),
        entry!(0x0072_ce30, fn_0072ce30(Ptr) -> Ptr),
        entry!(0x0072_73f0, fn_007273f0(Ptr) -> bool),
        entry!(0x0072_9970, fn_00729970(Ptr, u32)),
        entry!(0x0072_99a0, fn_007299a0(Ptr<ListBox>) -> Ptr<ListBox>),
        entry!(0x0072_9a70, fn_00729a70(Ptr<ListBox>)),
        entry!(
            0x0072_9ae0,
            fn_00729ae0(Ptr<ListBox>, u32, u32, u32, u32) -> u32
        ),
        entry!(0x0072_9d80, list_box_tes_recipe_remove_all(Ptr<ListBox>)),
        entry!(0x0072_9fb0, list_box_tes_recipe_sort(Ptr<ListBox>, u32)),
        entry!(0x0072_9fe0, list_box_tes_recipe_filter(Ptr<ListBox>, u32)),
        entry!(
            0x0072_a250,
            list_box_tes_recipe_highlight(Ptr<ListBox>, u32) -> bool
        ),
        entry!(
            0x0072_a2c0,
            list_box_tes_recipe_get_highlight_target(Ptr<ListBox>, u32) -> u32
        ),
        entry!(
            0x0072_a660,
            list_box_tes_recipe_restore_list_state(Ptr<ListBox>, u8)
        ),
        entry!(0x0072_a7c0, fn_0072a7c0(Ptr<ListBox>) -> Ptr<ListBox>),
        entry!(0x0072_a890, fn_0072a890(Ptr<ListBox>)),
        entry!(
            0x0072_a900,
            list_box_tes_recipe_component_add_item(Ptr<ListBox>, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x0072_aba0,
            list_box_tes_recipe_component_remove_all(Ptr<ListBox>)
        ),
        entry!(
            0x0072_add0,
            list_box_tes_recipe_component_sort(Ptr<ListBox>, u32)
        ),
        entry!(
            0x0072_ae00,
            list_box_tes_recipe_component_highlight(Ptr<ListBox>, u32) -> bool
        ),
        entry!(
            0x0072_ae70,
            list_box_tes_recipe_component_get_highlight_target(Ptr<ListBox>, u32) -> u32
        ),
        entry!(0x0072_b210, fn_0072b210(Ptr<ListBox>) -> Ptr<ListBox>),
        entry!(0x0072_b2e0, fn_0072b2e0(Ptr<ListBox>)),
        entry!(
            0x0072_b350,
            list_box_tes_condition_item_remove_all(Ptr<ListBox>)
        ),
        entry!(
            0x0072_b580,
            list_box_tes_condition_item_sort(Ptr<ListBox>, u32)
        ),
        entry!(
            0x0072_b5b0,
            list_box_tes_condition_item_highlight(Ptr<ListBox>, u32) -> bool
        ),
        entry!(
            0x0072_b620,
            list_box_tes_condition_item_get_highlight_target(Ptr<ListBox>, u32) -> u32
        ),
        entry!(
            0x0072_b9c0,
            list_box_tes_recipe_scalar_deleting_destructor(Ptr<ListBox>, u32) -> Ptr<ListBox>
        ),
        entry!(
            0x0072_b9f0,
            list_box_tes_recipe_component_scalar_deleting_destructor(
                Ptr<ListBox>,
                u32,
            ) -> Ptr<ListBox>
        ),
        entry!(
            0x0072_ba20,
            list_box_tes_condition_item_scalar_deleting_destructor(
                Ptr<ListBox>,
                u32,
            ) -> Ptr<ListBox>
        ),
        entry!(0x0072_ba50, fn_0072ba50(Ptr<ListBox>) -> u32),
        entry!(0x0072_bab0, fn_0072bab0(Ptr<ListBox>) -> i32),
        entry!(0x0072_bb60, list_box_tes_recipe_synchronize(Ptr<ListBox>)),
        entry!(
            0x0072_bc30,
            list_box_tes_recipe_is_enabled(Ptr<ListBox>) -> bool
        ),
        entry!(
            0x0072_bce0,
            list_box_tes_recipe_update_highlight_box_pos(Ptr<ListBox>)
        ),
        entry!(
            0x0072_c000,
            list_box_tes_recipe_set_total_list_height(Ptr<ListBox>)
        ),
        entry!(
            0x0072_c130,
            list_box_tes_recipe_component_synchronize(Ptr<ListBox>)
        ),
        entry!(
            0x0072_c200,
            list_box_tes_recipe_component_is_enabled(Ptr<ListBox>) -> bool
        ),
        entry!(
            0x0072_c2b0,
            list_box_tes_recipe_component_update_highlight_box_pos(Ptr<ListBox>)
        ),
        entry!(
            0x0072_c5d0,
            list_box_tes_recipe_component_set_total_list_height(Ptr<ListBox>)
        ),
        entry!(
            0x0072_c700,
            list_box_tes_condition_item_synchronize(Ptr<ListBox>)
        ),
        entry!(
            0x0072_c7d0,
            list_box_tes_condition_item_is_enabled(Ptr<ListBox>) -> bool
        ),
        entry!(
            0x0072_c880,
            list_box_tes_condition_item_update_highlight_box_pos(Ptr<ListBox>)
        ),
        entry!(
            0x0072_cba0,
            list_box_tes_condition_item_set_total_list_height(Ptr<ListBox>)
        ),
        entry!(
            0x0072_ccd0,
            list_box_tes_recipe_component_num_unfiltered(Ptr<ListBox>) -> i32
        ),
        entry!(
            0x0072_cd80,
            list_box_tes_condition_item_num_unfiltered(Ptr<ListBox>) -> i32
        ),
        entry!(0x0072_ce60, bs_simple_array_tes_recipe_category_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0072_6df0, recipe_menu_recipe_menu(Ptr) -> Ptr),
        entry!(0x0072_6f00, recipe_menu_get_class(Ptr) -> u32),
        entry!(0x0072_6f10, fn_00726f10() -> u32),
        entry!(0x0072_6f20, recipe_menu_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0072_6f50, fn_00726f50(Ptr)),
        entry!(0x0072_6ff0, recipe_menu_create(u32, u32) -> u32),
        entry!(0x0072_73e0, fn_007273e0() -> u32),
        entry!(0x0072_7430, recipe_menu_close()),
        entry!(0x0072_74b0, recipe_menu_do_click(Ptr, i32, u32)),
        entry!(0x0072_7680, fn_00727680(Ptr, u32, u32)),
        entry!(0x0072_78c0, fn_007278c0(u32, u32) -> i32),
        entry!(0x0072_7920, fn_00727920(u32) -> u32),
        entry!(0x0072_7b10, recipe_menu_do_enter(Ptr, i32, u32)),
        entry!(0x0072_8420, recipe_menu_do_gamepad(Ptr, u32, u32) -> bool),
        entry!(0x0072_84f0, fn_007284f0(i32)),
        entry!(0x0072_8a30, fn_00728a30(u8)),
        entry!(0x0072_8a70, recipe_menu_do_idle(Ptr)),
        entry!(0x0072_8c10, fn_00728c10(u32, u32) -> i32),
        entry!(0x0072_8cd0, fn_00728cd0(u32, u32) -> i32),
        entry!(0x0072_8d60, fn_00728d60(u32) -> bool),
        entry!(0x0072_8da0, fn_00728da0(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod b070_tests {
    use super::*;

    #[test]
    fn all_fourteen_tiles_must_be_set() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x60);
        for i in 0..14 {
            e.mem.set_u32(this + 0x28 + 4 * i, 1 + i);
        }
        assert!(e.call(0x0072_73f0, &args![this]).bool());
        e.mem.set_u32(this + 0x28 + 4 * 13, 0);
        assert!(!e.call(0x0072_73f0, &args![this]).bool());
    }

    #[test]
    fn sorts_the_array_with_qsort() {
        let mut e = Engine::new();
        e.register(QSORT, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0x5000);
        e.mem.set_u32(this + 8, 7);
        e.call_log = Some(vec![]);
        e.call(0x0072_9970, &args![this, 0x0072_0000u32]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|c| c.0 == QSORT).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x5000, 7, 4, 0x0072_0000]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touches_nothing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let before = e.mem.alloc(8);
        e.mem.set_u32(before, 0x55);
        e.call(0x0072ba80, &args![0u32, before, 2u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1, "no other call");
        assert_eq!(e.mem.u32(before), 0x55);
    }
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
        e.register(CONSTRUCTOR_BODY_006B3EB0, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 8, seen);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0072_ce30, &args![this]).u32(), this);
        assert_eq!(
            calls_to(&mut e, CONSTRUCTOR_BODY_006B3EB0),
            vec![vec![this, 0, 0]]
        );
        assert_eq!(e.mem.u32(this + 8), VTABLE_0072CE30);
    }
}

#[cfg(test)]
mod n057_tests {
    use super::*;

    /// Every callee outside this file the translations above reach.
    const EXTERNAL: &[u32] = &[
        0x00401000, 0x00401030, 0x00401170, 0x004037b0, 0x004037d0, 0x00403df0, 0x00406620,
        0x00406d00, 0x00406d30, 0x00406f60, 0x004077e0, 0x00408da0, 0x0040e210, 0x0040ebd0,
        0x004459e0, 0x0044a670, 0x0044ddc0, 0x0045cd60, 0x00464f30, 0x00469880, 0x0046fb20,
        0x0046ffb0, 0x004702f0, 0x00476b20, 0x00480f80, 0x00481280, 0x00481610, 0x00481680,
        0x004818e0, 0x00482090, 0x004821a0, 0x00482720, 0x00483a00, 0x0048cee0, 0x0048e8a0,
        0x0048ebc0, 0x004b71d0, 0x004b7210, 0x004bd510, 0x004be060, 0x004be330, 0x004bf220,
        0x004c0bf0, 0x004c8f30, 0x004d1360, 0x004d5e10, 0x004e4430, 0x00525980, 0x00525b20,
        0x005330e0, 0x00559450, 0x00574fa0, 0x00575610, 0x00576260, 0x00586150, 0x0059bb30,
        0x005a8080, 0x005ae380, 0x005b5e40, 0x005d43c0, 0x005f5950, 0x006286d0, 0x006450f0,
        0x00645380, 0x00646360, 0x00647b70, 0x00647c00, 0x00663b40, 0x00663b60, 0x0066e950,
        0x0066ef20, 0x00680c30, 0x006815c0, 0x00683520, 0x006a1b60, 0x006a7ad0, 0x006a9540,
        0x006ecd40, 0x00700320, 0x00702450, 0x007052f0, 0x00706f30, 0x00709430, 0x00715860,
        0x00715c60, 0x00717280, 0x007185e0, 0x00718840, 0x00719b20, 0x0071aa60, 0x0071aa80,
        0x00721130, 0x007211c0, 0x007240c0, 0x007242c0, 0x007242f0, 0x00726070, 0x007269d0,
        0x00726c40, 0x0072ce90, 0x007312e0, 0x0075a0d0, 0x00763ff0, 0x007640a0, 0x007a1910,
        0x007a1950, 0x007a22d0, 0x007a62e0, 0x007a7eb0, 0x007aba00, 0x007b5370, 0x007cb2e0,
        0x007e8890, 0x008256d0, 0x00825c00, 0x0083fd60, 0x008454f0, 0x0087f4c0, 0x0088d7d0,
        0x008a1710, 0x008adcf0, 0x008d8520, 0x00905820, 0x00950030, 0x0096a2d0, 0x00a00940,
        0x00a00e90, 0x00a00f30, 0x00a011b0, 0x00a01230, 0x00a01290, 0x00a012d0, 0x00a01350,
        0x00a01530, 0x00a01860, 0x00a01b00, 0x00a03c90, 0x00a03d40, 0x00a03da0, 0x00a07530,
        0x00a08b20, 0x00a09030, 0x00a09080, 0x00a09130, 0x00a09200, 0x00a09410, 0x00a1c4a0,
        0x00a1c520, 0x00a1d910, 0x00a1dc20, 0x00a1dc70, 0x00a1ddb0, 0x00ec408c, 0x00ec43fb,
        0x00ec62c0,
    ];

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn answer(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| eax.into_ret());
    }

    fn st0(e: &mut Engine, addr: u32, value: f64) {
        e.register_double(addr, move |_, _| Ret {
            st0: value,
            ..Ret::default()
        });
    }

    /// An engine with every outside callee doubled (the node helpers read the node, the
    /// words at +0 and +4 read memory), the statics' pages and the `double`s mapped.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for &addr in EXTERNAL {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(NODE_SELF, |_, a| a[0].into_ret());
        e.register(WORD_AT_FOUR, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(WORD_AT_ZERO, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(QSORT, |_, _| Ret::default());
        e.register(FTOL2, |_, a| {
            (f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32).into_ret()
        });
        for page in [
            0x011d_8000,
            0x011d_9000,
            0x0119_f000,
            0x011d_e000,
            0x011c_3000,
        ] {
            e.map(page, 0x1000);
        }
        for page in [
            0x0101_1000,
            0x0101_2000,
            0x0107_0000,
            0x0102_f000,
            0x0102_3000,
            0x0106_f000,
            0x0101_6000,
            0x0101_e000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.set_global(TWO_DOUBLE, 2.0f64);
        e
    }

    fn list_box(e: &mut Engine) -> Ptr<ListBox> {
        Ptr::new(e.mem.alloc(0x30))
    }

    /// A list node (item, next) and its `ListBoxItem` (tile, data, filtered byte).
    fn item(e: &mut Engine, tile: u32, data: u32, filtered: u8) -> u32 {
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item, tile);
        e.mem.set_u32(item + 4, data);
        e.mem.set_u8(item + 8, filtered);
        item
    }

    /// A list box with the given items (the first in the embedded node, the rest chained).
    fn list_with(e: &mut Engine, items: &[u32]) -> Ptr<ListBox> {
        let this = list_box(e);
        let mut node = this.addr() + 4;
        for (i, &entry) in items.iter().enumerate() {
            e.mem.set_u32(node, entry);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        this
    }

    const INSTANCES: [(usize, RecipeListBox); 3] = [
        (0, RECIPE_LIST_BOX),
        (1, COMPONENT_LIST_BOX),
        (2, CONDITION_LIST_BOX),
    ];

    #[test]
    fn list_box_constructors_and_destructors() {
        let ctors = [0x0072_99a0u32, 0x0072_a7c0, 0x0072_b210];
        let dtors = [0x0072_9a70u32, 0x0072_a890, 0x0072_b2e0];
        let deleting = [0x0072_b9c0u32, 0x0072_b9f0, 0x0072_ba20];
        for (i, kind) in INSTANCES {
            let mut e = engine();
            let this = list_box(&mut e);
            for offset in (0x0c..0x30).step_by(4) {
                e.mem.set_u32(this.addr() + offset, 0xffff_ffff);
            }
            e.call_log = Some(vec![]);
            assert_eq!(e.call(ctors[i], &args![this]).u32(), this.addr());
            assert_eq!(e.mem.u32(this.addr()), kind.vtable);
            assert_eq!(e.mem.u32(this.addr() + 0x0c), 0);
            assert_eq!(e.mem.u16(this.addr() + 0x2c), 0);
            assert_eq!(
                calls_to(&e, LIST_BOX_INIT_FLAGS),
                vec![vec![this.addr(), 0, 1]]
            );
            assert_eq!(calls_to(&e, LIST_BOX_REGISTER), vec![vec![this.addr()]]);
            e.mem.set_u32(this.addr(), 0);
            e.call(dtors[i], &args![this]);
            assert_eq!(e.mem.u32(this.addr()), kind.vtable);
            assert_eq!(calls_to(&e, LIST_BOX_UNREGISTER), vec![vec![this.addr()]]);
            assert_eq!(calls_to(&e, LIST_DESTRUCT), vec![vec![this.addr() + 4]]);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(deleting[i], &args![this, 1u32]).u32(), this.addr());
            assert_eq!(calls_to(&e, FREE), vec![vec![this.addr()]]);
            e.call_log = Some(vec![]);
            e.call(deleting[i], &args![this, 0u32]);
            assert!(calls_to(&e, FREE).is_empty());
        }
    }

    #[test]
    fn category_array_deleting_destructor() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0072_ce60, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, CATEGORY_ARRAY_DESTRUCTOR), vec![vec![this]]);
        assert_eq!(calls_to(&e, FREE), vec![vec![this]]);
    }

    fn add_item_engine() -> Engine {
        let mut e = engine();
        answer(&mut e, TILE_GET_MENU, 0x5000);
        answer(&mut e, MENU_RENDER_TEMPLATE, 0x6000);
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ITEM_CONSTRUCT, |e, a| {
            let data = e.mem.u32(a[2]);
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, data);
            a[0].into_ret()
        });
        answer(&mut e, LIST_BOX_HAS_FLAG, 1);
        answer(&mut e, LIST_BOX_COUNT_00726C40, 1);
        answer(&mut e, ADD_USER_TRAIT, 0x77);
        st0(&mut e, TILE_GET_FLOAT, 3.0);
        answer(&mut e, TILE_GET_VALUE_Q, 0x9000);
        e
    }

    #[test]
    fn add_item_appends_or_inserts_sorted() {
        for addr in [0x0072_9ae0u32, 0x0072_a900] {
            let mut e = add_item_engine();
            let this = list_box(&mut e);
            assert_eq!(
                e.call(addr, &args![this, 1u32, 0u32, 0u32, 0x10u32]).u32(),
                0
            );
            e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
            e.set(this, ListBox::pTemplateName, Ptr::new(0x4100));
            e.set(this, ListBox::usNextIndex, 5);
            e.call_log = Some(vec![]);
            let tile = e
                .call(addr, &args![this, 0x42u32, 0x4200u32, 0u32, 0u32])
                .u32();
            assert_eq!(tile, 0x6000);
            assert_eq!(
                calls_to(&e, MENU_RENDER_TEMPLATE),
                vec![vec![0x5000, 0x3000, 0x4100, 0]]
            );
            let appended = calls_to(&e, LIST_APPEND);
            assert_eq!(appended.len(), 1);
            let entry = e.mem.u32(appended[0][1]);
            assert_eq!((e.mem.u32(entry), e.mem.u32(entry + 4)), (0x6000, 0x42));
            assert_eq!(
                calls_to(&e, TILE_SET_INT),
                vec![vec![0x6000, 0xfaa, 0xffff_ffff], vec![0x6000, 0xfac, 5]]
            );
            assert_eq!(
                calls_to(&e, VALUE_ADD_ACTION_FLOAT),
                vec![vec![0x9000, 0x7d3, 3.0f32.to_bits()]]
            );
            e.call_log = Some(vec![]);
            e.call(addr, &args![this, 0x43u32, 0u32, 0x00ab_cdefu32, 0u32]);
            let inserted = calls_to(&e, LIST_INSERT_SORTED);
            assert_eq!(inserted.len(), 1);
            assert_eq!(inserted[0][2], 0x00ab_cdef);
            assert!(calls_to(&e, LIST_APPEND).is_empty());
        }
    }

    #[test]
    fn remove_all_deletes_items_and_resets() {
        let removers = [0x0072_9d80u32, 0x0072_aba0, 0x0072_b350];
        for (i, kind) in INSTANCES {
            let mut e = engine();
            e.register(LIST_BOX_HAS_FLAG, |_, a| u32::from(a[1] == 1).into_ret());
            answer(&mut e, TEXT_TO_TRAIT, 0x55);
            answer(&mut e, ADD_USER_TRAIT, 0x66);
            e.put_vtable(0x0f00_0000, &[0x0000_7000]);
            e.register(0x0000_7000, |_, _| Ret::default());
            let tile = e.mem.alloc(4);
            e.mem.set_u32(tile, 0x0f00_0000);
            let first = item(&mut e, tile, 0x8000, 0);
            let second = item(&mut e, 0, 0, 0);
            let this = list_with(&mut e, &[first, second]);
            let node = e.mem.u32(this.addr() + 8);
            e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
            e.set(this, ListBox::pHighlightedItem, Ptr::new(0x3100));
            e.call_log = Some(vec![]);
            e.call(removers[i], &args![this]);
            assert_eq!(calls_to(&e, FREE), vec![vec![second], vec![first]]);
            assert_eq!(calls_to(&e, NODE_DELETE), vec![vec![node, 1]]);
            assert_eq!(calls_to(&e, ITEM_DATA_DELETE), vec![vec![0x8000, 1]]);
            assert_eq!(calls_to(&e, 0x0000_7000), vec![vec![tile, 1]]);
            assert!(e.get(this, ListBox::pHighlightedItem).is_null());
            assert_eq!(calls_to(&e, TILE_SET_FLOAT), vec![vec![0x3000, 0x55, 0, 1]]);
            assert_eq!(e.mem.u32(kind.highlight_y_trait.guard) & 3, 3);
        }
    }

    #[test]
    fn sort_synchronizes() {
        let sorts = [0x0072_9fb0u32, 0x0072_add0, 0x0072_b580];
        for (i, _) in INSTANCES {
            let mut e = engine();
            let first = item(&mut e, 0x6100, 0, 0);
            let this = list_with(&mut e, &[first]);
            e.call_log = Some(vec![]);
            e.call(sorts[i], &args![this, 0x1234u32]);
            assert_eq!(calls_to(&e, LIST_SORT), vec![vec![this.addr() + 4, 0x1234]]);
            assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x6100, 0xfac, 0]]);
        }
    }

    #[test]
    fn synchronize_numbers_unfiltered_items() {
        let syncs = [0x0072_bb60u32, 0x0072_c130, 0x0072_c700];
        for (i, _) in INSTANCES {
            let mut e = engine();
            let first = item(&mut e, 0x6100, 0, 0);
            let second = item(&mut e, 0x6200, 0, 1);
            let third = item(&mut e, 0x6300, 0, 0);
            let this = list_with(&mut e, &[first, second, third]);
            e.set(this, ListBox::usNextIndex, 9);
            e.call_log = Some(vec![]);
            e.call(syncs[i], &args![this]);
            assert_eq!(
                calls_to(&e, TILE_SET_INT),
                vec![vec![0x6100, 0xfac, 0], vec![0x6300, 0xfac, 1]]
            );
            assert_eq!(
                calls_to(&e, INIT_TILE_POSITION),
                vec![vec![this.addr(), 0x6100], vec![this.addr(), 0x6300]]
            );
            assert_eq!(e.get(this, ListBox::usNextIndex), 2);
        }
    }

    #[test]
    fn highlight_and_is_enabled() {
        let highlights = [0x0072_a250u32, 0x0072_ae00, 0x0072_b5b0];
        let enabled = [0x0072_bc30u32, 0x0072_c200, 0x0072_c7d0];
        for (i, kind) in INSTANCES {
            let mut e = engine();
            answer(&mut e, ADD_USER_TRAIT, 0x44);
            e.register(TILE_IS_TRUE, |_, a| u32::from(a[0] != 0x6100).into_ret());
            e.register(LIST_BOX_BELONGS_TO, |_, a| {
                u32::from(a[1] != 0x6200).into_ret()
            });
            let this = list_box(&mut e);
            assert!(!e.call(enabled[i], &args![this]).bool());
            assert_eq!(e.mem.u32(kind.enabled_trait.cache), 0x44);
            assert!(!e.call(highlights[i], &args![this, 0x6000u32]).bool());
            e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
            assert!(e.call(enabled[i], &args![this]).bool());
            assert!(!e.call(highlights[i], &args![this, 0x6100u32]).bool());
            assert!(!e.call(highlights[i], &args![this, 0x6200u32]).bool());
            assert!(e.call(highlights[i], &args![this, 0x6000u32]).bool());
            assert_eq!(e.get(this, ListBox::pHighlightedItem).addr(), 0x6000);
        }
    }

    #[test]
    fn get_highlight_target_moves_and_skips_disabled_tiles() {
        let targets = [0x0072_a2c0u32, 0x0072_ae70, 0x0072_b620];
        for (i, _) in INSTANCES {
            let mut e = engine();
            answer(&mut e, TILE_IS_TRUE, 1);
            // The tile's index trait is its address' low byte.
            e.register(TILE_GET_FLOAT, |_, a| Ret {
                st0: f64::from(a[0] & 0xff),
                ..Ret::default()
            });
            let first = item(&mut e, 0x6000, 0, 0);
            let second = item(&mut e, 0x6001, 0, 0);
            let third = item(&mut e, 0x6002, 0, 0);
            let this = list_with(&mut e, &[first, second, third]);
            e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
            e.set(this, ListBox::usNextIndex, 3);
            e.set(this, ListBox::pHighlightedItem, Ptr::new(0x6000));
            assert_eq!(e.call(targets[i], &args![this, 2u32]).u32(), 0x6001);
            // Up from the first: clamped to it, which is highlighted: 0.
            assert_eq!(e.call(targets[i], &args![this, 1u32]).u32(), 0);
            assert_eq!(e.call(targets[i], &args![this, 7u32]).u32(), 0);
            // Disabled second tile: skipped.
            e.register(TILE_IS_TRUE, |_, a| u32::from(a[0] != 0x6001).into_ret());
            assert_eq!(e.call(targets[i], &args![this, 2u32]).u32(), 0x6002);
        }
    }

    #[test]
    fn update_highlight_box_pos_follows_the_tile() {
        let updates = [0x0072_bce0u32, 0x0072_c2b0, 0x0072_c880];
        for (i, kind) in INSTANCES {
            let mut e = engine();
            e.register(ADD_USER_TRAIT, |_, a| {
                if a[0] == HIGHLIGHT_Y_TEXT {
                    0x31u32.into_ret()
                } else {
                    0x32u32.into_ret()
                }
            });
            e.register(TILE_GET_VALUE, |_, a| (0x9000 + a[1]).into_ret());
            answer(&mut e, TILE_FLAG_GETTER_0071AA80, 1);
            answer(&mut e, TILE_FIND_BY_NAME, 0x7100);
            answer(&mut e, TILE_GET_CHILD_BY_NAME, 0x7200);
            let this = list_box(&mut e);
            e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
            e.call_log = Some(vec![]);
            e.call(updates[i], &args![this]);
            assert_eq!(
                calls_to(&e, TILE_SET_INT),
                vec![vec![0x3000, 0x31, 0xffff_ffff], vec![0x3000, 0x32, 0]]
            );
            e.set(this, ListBox::pHighlightedItem, Ptr::new(0x6000));
            e.call_log = Some(vec![]);
            e.call(updates[i], &args![this]);
            assert_eq!(e.mem.u32(kind.highlight_y_trait.cache), 0x31);
            assert_eq!(
                calls_to(&e, VALUE_ADD_ACTION_TILE),
                vec![
                    vec![0x9031, 0x7d0, 0x6000, 0xfa2],
                    vec![0x9032, 0x7d0, 0x6000, 0xfb0]
                ]
            );
            assert_eq!(calls_to(&e, TILE_FLAG_SETTER_0071AA60).len(), 5);
        }
    }

    #[test]
    fn set_total_list_height_counts_scroll_steps() {
        let setters = [0x0072_c000u32, 0x0072_c5d0, 0x0072_cba0];
        for (i, _) in INSTANCES {
            let mut e = engine();
            e.register(TILE_GET_FLOAT, |_, a| Ret {
                st0: if a[1] == 0xfb0 { 25.0 } else { 0.0 },
                ..Ret::default()
            });
            answer(&mut e, LIST_BOX_COUNT_00726C40, 8);
            e.register(ROUND_FLOAT, |_, a| Ret {
                st0: f64::from(f32::from_bits(a[0])).ceil(),
                ..Ret::default()
            });
            e.register(INT_PAIR_00647B70, |_, a| {
                (a[0] as i32).max(a[1] as i32).into_ret()
            });
            e.register(TEXT_TO_TRAIT, |_, a| (a[0] & 0xfff).into_ret());
            let this = list_box(&mut e);
            e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
            e.set(this, ListBox::pScrollBar, Ptr::new(0x3400));
            e.set(this, ListBox::fTotalListHeight, 100.0);
            e.call_log = Some(vec![]);
            e.call(setters[i], &args![this]);
            assert_eq!(calls_to(&e, ROUND_FLOAT), vec![vec![6.0f32.to_bits()]]);
            assert_eq!(
                calls_to(&e, TILE_SET_FLOAT),
                vec![vec![0x3000, 0x90c, 12.5f32.to_bits(), 1]]
            );
            assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x3400, 0x8f8, 7]]);
        }
    }

    #[test]
    fn num_unfiltered_and_the_last_tile() {
        let counters = [0x0072_bab0u32, 0x0072_ccd0, 0x0072_cd80];
        for (i, kind) in INSTANCES {
            let mut e = engine();
            answer(&mut e, ADD_USER_TRAIT, 0x99);
            answer(&mut e, LIST_BOX_COUNT_00726C40, 7);
            st0(&mut e, TILE_GET_FLOAT, 2.9);
            let this = list_box(&mut e);
            e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
            assert_eq!(e.call(counters[i], &args![this]).i32(), 5);
            assert_eq!(e.mem.u32(kind.num_filtered_trait.cache), 0x99);
            if i == 0 {
                e.put_vtable(0x0f00_0000, &[0, 0, 0, 0, 0, 0x0000_7014]);
                e.register(0x0000_7014, |_, a| (0x100 + a[1]).into_ret());
                e.mem.set_u32(this.addr(), 0x0f00_0000);
                assert_eq!(e.call(0x0072_ba50, &args![this]).u32(), 0x104);
            }
        }
    }

    #[test]
    fn filter_marks_items_and_keeps_the_highlight_index() {
        let mut e = engine();
        answer(&mut e, ADD_USER_TRAIT, 0x51);
        e.register(0x0000_7100, |_, a| u32::from(a[0] == 0x22).into_ret());
        e.register(TILE_GET_FLOAT, |_, a| Ret {
            st0: if a[0] == 0x6200 { 1.0 } else { 0.0 },
            ..Ret::default()
        });
        let first = item(&mut e, 0x6100, 0x11, 0);
        let second = item(&mut e, 0x6200, 0x22, 0);
        let third = item(&mut e, 0x6300, 0x33, 0);
        let this = list_with(&mut e, &[first, second, third]);
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        e.set(this, ListBox::pHighlightedItem, Ptr::new(0x6200));
        e.call_log = Some(vec![]);
        e.call(0x0072_9fe0, &args![this, 0x0000_7100u32]);
        assert_eq!(e.mem.u8(second + 8), 1);
        assert_eq!(e.mem.u8(third + 8), 0);
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![
                vec![0x6100, 0xfac, 0],
                vec![0x6200, 0xfac, 0xffff_ffff],
                vec![0x6300, 0xfac, 1]
            ]
        );
        assert_eq!(e.get(this, ListBox::pHighlightedItem).addr(), 0x6300);
        assert_eq!(
            calls_to(&e, TILE_SET_INTEGER_VALUE),
            vec![vec![0x3000, 0x51, 1]]
        );
    }

    #[test]
    fn restore_list_state_focuses_the_saved_tile() {
        let mut e = engine();
        answer(&mut e, TEXT_TO_TRAIT, 0x52);
        answer(&mut e, INTERFACE_MANAGER, 0x8800);
        answer(&mut e, LIST_BOX_COUNT_00726C40, 4);
        st0(&mut e, FLOAT_PAIR_0040EBD0, 2.0);
        e.put_vtable(0x0f00_0000, &[0x0000_7000, 0, 0, 0, 0, 0x0000_7014]);
        e.register(0x0000_7000, |_, _| Ret::default());
        e.register(0x0000_7014, |_, a| (0x100 + a[1]).into_ret());
        let this = list_box(&mut e);
        e.mem.set_u32(this.addr(), 0x0f00_0000);
        e.call_log = Some(vec![]);
        e.call(0x0072_a660, &args![this, 1u8]);
        assert!(calls_to(&e, SET_CURRENT_FOCUS_TARGET).is_empty());
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        e.set(this, ListBox::pScrollBar, Ptr::new(0x3400));
        e.set(this, ListBox::fSavedScrollbarValue, 3.5);
        answer(&mut e, FLAG_004B71D0, 1);
        e.call(0x0072_a660, &args![this, 1u8]);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x3400, 0x52, 3.5f32.to_bits(), 0]]
        );
        assert_eq!(
            calls_to(&e, SET_CURRENT_FOCUS_TARGET),
            vec![vec![0x8800, 0x102, 0xfc3, 1]]
        );
        assert_eq!(calls_to(&e, 0x0000_7000), vec![vec![this.addr(), 0x102]]);
    }

    /// A recipe menu object with a vtable whose slots are doubles.
    fn recipe_menu(e: &mut Engine) -> u32 {
        let menu = e.mem.alloc(0x104);
        e.put_vtable(0x0e00_0000, &[0x0000_8000; 0x20]);
        e.register(0x0000_8000, |_, _| Ret::default());
        e.mem.set_u32(menu, 0x0e00_0000);
        for list in [RECIPE_LIST, INGREDIENT_LIST, CONDITIONAL_LIST] {
            e.mem.set_u32(menu + list, 0x0e00_0000);
        }
        for i in 0..14 {
            e.mem.set_u32(menu + TILES + 4 * i, 0x2000 + 4 * i);
        }
        menu
    }

    #[test]
    fn constructor_destructor_and_class() {
        let mut e = engine();
        answer(&mut e, TEXT_TO_TRAIT, 0x71);
        let this = e.mem.alloc(0x104);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0072_6df0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), RECIPE_MENU_VTABLE);
        assert_eq!(e.mem.u32(this + RECIPE_LIST), RECIPE_LIST_BOX.vtable);
        assert_eq!(e.mem.u32(this + INGREDIENT_LIST), COMPONENT_LIST_BOX.vtable);
        assert_eq!(
            e.mem.u32(this + CONDITIONAL_LIST),
            CONDITION_LIST_BOX.vtable
        );
        assert_eq!(e.mem.u32(this + CURRENT_LIST), this + RECIPE_LIST);
        assert_eq!(e.mem.u32(this + FILTER_SOUND), 3);
        assert_eq!(e.global::<u32>(TITLE_TRAIT_011D8E98), 0x71);
        assert_eq!(
            calls_to(&e, ARRAY_RESET_008454F0),
            vec![vec![CATEGORY_ARRAY, 1]]
        );
        assert_eq!(e.call(0x0072_6f00, &args![this]).u32(), 0x435);
        assert_eq!(e.call(0x0072_73e0, &args![]).u32(), 0x435);
        assert_eq!(e.call(0x0072_6f10, &args![]).u32(), 0x011d_8f08);
        e.set_global(RECIPE_MENU, this);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0072_6f20, &args![this, 1u32]).u32(), this);
        assert_eq!(e.global::<u32>(RECIPE_MENU), 0);
        assert_eq!(calls_to(&e, MENU_DESTRUCTOR), vec![vec![this]]);
        assert_eq!(calls_to(&e, FREE), vec![vec![this]]);
        assert_eq!(
            calls_to(&e, LIST_DESTRUCT),
            vec![
                vec![this + CONDITIONAL_LIST + 4],
                vec![this + INGREDIENT_LIST + 4],
                vec![this + RECIPE_LIST + 4]
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x0072_6f50, &args![this]);
        assert!(calls_to(&e, FREE).is_empty());
    }

    #[test]
    fn create_needs_a_crafter_and_sets_the_menu_up() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0072_6ff0, &args![0u32, 0u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, PRINT_MESSAGE),
            vec![vec![0x12, NULL_VENDOR_TEXT]]
        );
        let menu = recipe_menu(&mut e);
        e.mem.set_u32(0x0e00_0000 + 0x34, 0x0000_8034);
        answer(&mut e, 0x0000_8034, 0x435);
        answer(&mut e, TILE_READ_FILE, 0x4444);
        answer(&mut e, TILE_GET_MENU, menu);
        e.register(TILE_GET_FLOAT, |_, a| Ret {
            st0: if a[1] == 0xfa7 { 6006.0 } else { 6.0 },
            ..Ret::default()
        });
        e.set_global(STACKING_TYPE_ONE, 6006.0f64);
        e.set_global(TWENTY_DOUBLE, 20.0f64);
        st0(&mut e, TILE_GET_MAXIMUM_DEPTH, 9.0);
        answer(&mut e, INTERFACE_MANAGER, 0x1000);
        answer(&mut e, TUTORIAL_SHOWN_00718840, 1);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0072_6ff0, &args![0x7700u32, 0x7800u32]).u32(),
            0x4444
        );
        assert_eq!(e.mem.u32(menu + CRAFTER_REF), 0x7700);
        assert_eq!(e.mem.u32(menu + CATEGORY), 0x7800);
        assert!(calls_to(&e, PRINT_MESSAGE).is_empty());
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT)[0],
            vec![0x4444, 0xfad, 9.0f32.to_bits(), 1]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT)[1],
            vec![0x2000 + 0x1c, 0xfa1, (-14.0f32).to_bits(), 1]
        );
        assert_eq!(
            calls_to(&e, LIST_BOX_INITIALIZE),
            vec![
                vec![menu + RECIPE_LIST, 0x2000 + 0xc, LIST_TEMPLATE_TEXT],
                vec![menu + INGREDIENT_LIST, 0x2000 + 0x18, LIST_TEMPLATE_TEXT]
            ]
        );
        assert_eq!(calls_to(&e, TILE_SET_STRING).len(), 6);
        assert_eq!(
            calls_to(&e, TUTORIAL_SHOWN_00718840),
            vec![vec![0x1000 + 0x4d4, 0x26]]
        );
        assert_eq!(calls_to(&e, MENU_PREP_FOR_VISIBILITY), vec![vec![menu, 0]]);
        assert_eq!(calls_to(&e, RECIPE_LIST_HEAD_004077E0).len(), 1);
    }

    #[test]
    fn create_deletes_a_menu_of_another_class() {
        let mut e = engine();
        let menu = recipe_menu(&mut e);
        answer(&mut e, 0x0000_8000, 0x3f0);
        answer(&mut e, TILE_READ_FILE, 0x4444);
        answer(&mut e, TILE_GET_MENU, menu);
        e.mem.set_u32(menu + 4, 0x55);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0072_6ff0, &args![0x7700u32, 0u32]).u32(), 0);
        assert_eq!(calls_to(&e, 0x0000_8000), vec![vec![menu], vec![menu, 1]]);
    }

    #[test]
    fn close_ends_the_service() {
        let mut e = engine();
        e.call(0x0072_7430, &args![]);
        let menu = recipe_menu(&mut e);
        e.mem.set_u32(menu + 4, 0x4400);
        e.set_global(RECIPE_MENU, menu);
        e.put_vtable(0x0d00_0000, &[0x0000_9000; 0x120]);
        answer(&mut e, 0x0000_9000, 1);
        let process = e.mem.alloc(4);
        e.mem.set_u32(process, 0x0d00_0000);
        answer(&mut e, GET_PROCESS, process);
        e.set_global(PLAYER, 0x5500u32);
        e.call_log = Some(vec![]);
        e.call(0x0072_7430, &args![]);
        assert_eq!(calls_to(&e, PLAYER_UPDATE_3D), vec![vec![0x5500]]);
        assert_eq!(calls_to(&e, SERVICE_END), vec![vec![0]]);
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x4400, 0x1772, 1]]);
        assert_eq!(calls_to(&e, MENU_START_FADE_OUT), vec![vec![menu]]);
    }

    #[test]
    fn do_click_cycles_the_filter_and_opens_the_quantity_menu() {
        let mut e = engine();
        let menu = recipe_menu(&mut e);
        e.set_global(FILTER_INDEX_0119FBF4, 0xffff_ffffu32);
        answer(&mut e, ARRAY_COUNT, 3);
        let category = e.mem.alloc(0x20);
        let entry = e.mem.alloc(4);
        e.mem.set_u32(entry, category);
        answer(&mut e, ARRAY_AT, entry);
        answer(&mut e, STRING_TEXT, 0x7171);
        e.call_log = Some(vec![]);
        e.call(0x0072_74b0, &args![menu, 1i32, 0u32]);
        assert_eq!(e.global::<u32>(FILTER_INDEX_0119FBF4), 0);
        assert_eq!(calls_to(&e, STRING_TEXT), vec![vec![category + 0x18]]);
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![vec![0x2004, 0xfc4, 0x7171, 1]]
        );
        assert_eq!(
            calls_to(&e, LIST_BOX_SAVE_LIST_STATE),
            vec![vec![menu + RECIPE_LIST]]
        );
        e.call(0x0072_74b0, &args![menu, 0i32, 0u32]);
        e.call(0x0072_74b0, &args![menu, 0i32, 0u32]);
        assert_eq!(e.global::<u32>(FILTER_INDEX_0119FBF4), 2);
        e.set_global(CAN_MAKE_011D8EA8, 9u32);
        e.call_log = Some(vec![]);
        e.call(0x0072_74b0, &args![menu, 7i32, 0u32]);
        assert_eq!(
            calls_to(&e, QUANTITY_MENU_CREATE),
            vec![vec![9, MAKE_CALLBACK_007284F0, 0x7fff_ffff]]
        );
        e.set_global(INGREDIENTS_FOCUSED_011D8EA5, 1u8);
        e.call_log = Some(vec![]);
        e.call(0x0072_74b0, &args![menu, 1i32, 0u32]);
        assert!(calls_to(&e, INTERFACE_PLAY_MENU_SOUND).is_empty());
        e.call(0x0072_74b0, &args![menu, 8i32, 0u32]);
        assert_eq!(e.global::<u8>(INGREDIENTS_FOCUSED_011D8EA5), 0);
    }

    /// A node chain over the given items (item at +0, next at +4).
    fn chain(e: &mut Engine, items: &[u32]) -> u32 {
        let mut next = 0;
        for &entry in items.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, entry);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    #[test]
    fn fill_adds_the_recipes_of_the_category() {
        let mut e = engine();
        let menu = recipe_menu(&mut e);
        e.set_global(RECIPE_MENU, menu);
        let recipes = chain(&mut e, &[0x7100, 0x7200]);
        let outputs = chain(&mut e, &[0]);
        answer(&mut e, RECIPE_OUTPUTS_005D43C0, outputs);
        answer(&mut e, RECIPE_LIST_HEAD_004077E0, recipes);
        e.register(RECIPE_CATEGORY_006A1B60, |_, a| (a[0] + 1).into_ret());
        answer(&mut e, CONDITION_IS_TRUE, 1);
        answer(&mut e, RECIPE_CATEGORY_KEY_006286D0, 0x55);
        answer(&mut e, ARRAY_FIND_00719B20, 5);
        answer(&mut e, ARRAY_COUNT, 2);
        answer(&mut e, STRING_TEXT, 0x6600);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x2f00);
        answer(&mut e, ITEM_TILE_007B5370, slot);
        answer(&mut e, TILE_GET_MENU, 0x5000);
        answer(&mut e, MENU_RENDER_TEMPLATE, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x0072_7680, &args![menu, 0u32, 0x7201u32]);
        assert_eq!(calls_to(&e, ARRAY_APPEND_007CB2E0).len(), 1);
        assert_eq!(calls_to(&e, ARRAY_APPEND_007CB2E0)[0][0], CATEGORY_ARRAY);
        let formats = calls_to(&e, FORMAT);
        assert_eq!(formats.len(), 1);
        assert_eq!(&formats[0][1..], &[0x100, ONE_STRING_FORMAT, 0x6600]);
        assert_eq!(
            calls_to(&e, TILE_SET_INT).last().unwrap(),
            &vec![0x2f00, 0xfa9, 0xff]
        );
        assert_eq!(
            calls_to(&e, 0x0000_8000),
            vec![vec![menu + RECIPE_LIST, COMPARE_00728C10]]
        );
        assert_eq!(calls_to(&e, QSORT).len(), 1);
    }

    #[test]
    fn category_names_compare() {
        let mut e = engine();
        e.register(STRING_TEXT, |_, a| (a[0] - 0x18).into_ret());
        e.register(COMPARE_STRINGS, |_, a| {
            (a[0] as i32 - a[1] as i32).into_ret()
        });
        let first = e.mem.alloc(4);
        e.mem.set_u32(first, 0x500);
        let second = e.mem.alloc(4);
        e.mem.set_u32(second, 0x480);
        assert_eq!(e.call(0x0072_78c0, &args![first, second]).i32(), 0x80);
    }

    #[test]
    fn can_make_counts_the_inputs() {
        let mut e = engine();
        let menu = recipe_menu(&mut e);
        e.set_global(RECIPE_MENU, menu);
        answer(&mut e, RECIPE_CATEGORY_006A1B60, 0x90);
        answer(&mut e, RECIPE_CATEGORY_KEY_006286D0, 1);
        assert_eq!(e.call(0x0072_7920, &args![0x7100u32]).u32(), 100);
        let crafter = e.mem.alloc(0x200);
        e.put_vtable(0x0c00_0000, &[0x0000_a000; 0x50]);
        answer(&mut e, 0x0000_a000, 1);
        e.mem.set_u32(crafter, 0x0c00_0000);
        e.mem.set_u32(menu + CRAFTER_REF, crafter);
        e.mem.set_u32(menu + CATEGORY, 0x90);
        answer(&mut e, RECIPE_SKILL_0059BB30, 0xffff_ffff);
        let first = e.mem.alloc(8);
        e.mem.set_u32(first, 2);
        e.mem.set_u32(first + 4, 0x3300);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, 3);
        e.mem.set_u32(second + 4, 0x3400);
        let inputs = chain(&mut e, &[first, second]);
        answer(&mut e, RECIPE_INPUTS_005A8080, inputs);
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry + 4, 7);
        answer(&mut e, GET_INVENTORY_ITEM, entry);
        assert_eq!(e.call(0x0072_7920, &args![0x7100u32]).u32(), 2);
        assert_eq!(e.global::<u32>(CAN_MAKE_011D8EA8), 2);
        e.mem.set_u32(second, 8);
        assert_eq!(e.call(0x0072_7920, &args![0x7100u32]).u32(), 0);
        e.mem.set_u32(menu + CATEGORY, 0x91);
        assert_eq!(e.call(0x0072_7920, &args![0x7100u32]).u32(), 0);
    }

    #[test]
    fn do_enter_switches_lists() {
        let mut e = engine();
        let menu = recipe_menu(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0072_7b10, &args![menu, 3i32, 0u32]);
        assert_eq!(
            calls_to(&e, 0x0000_8000),
            vec![vec![menu, 6, 0], vec![menu + INGREDIENT_LIST, 0]]
        );
        assert_eq!(
            calls_to(&e, LIST_BOX_SAVE_LIST_STATE),
            vec![vec![menu + RECIPE_LIST]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0072_7b10, &args![menu, 6i32, 0u32]);
        assert_eq!(e.global::<u8>(INGREDIENTS_FOCUSED_011D8EA5), 1);
        assert_eq!(
            calls_to(&e, 0x0000_8000).last().unwrap(),
            &vec![menu, 0xf, 0]
        );
    }

    #[test]
    fn do_enter_shows_the_selected_recipe() {
        let mut e = engine();
        let menu = recipe_menu(&mut e);
        e.set_global(RECIPE_MENU, menu);
        e.mem.set_u32(menu + CURRENT_LIST, menu + RECIPE_LIST);
        let output = e.mem.alloc(8);
        e.mem.set_u32(output + 4, 0x3500);
        let outputs = chain(&mut e, &[output]);
        let input = e.mem.alloc(8);
        e.mem.set_u32(input, 2);
        e.mem.set_u32(input + 4, 0x3300);
        let inputs = chain(&mut e, &[input]);
        answer(&mut e, LIST_BOX_CURRENT_ITEM, 0x7100);
        answer(&mut e, RECIPE_CATEGORY_006A1B60, 0x90);
        answer(&mut e, RECIPE_OUTPUTS_005D43C0, outputs);
        answer(&mut e, RECIPE_INPUTS_005A8080, inputs);
        answer(&mut e, LIST_COUNT_005AE380, 1);
        answer(&mut e, STRING_TEXT, 0x6600);
        answer(&mut e, ERROR_ITEM_00464F30, 0x6700);
        answer(&mut e, TILE_GET_MENU, 0x5000);
        answer(&mut e, MENU_RENDER_TEMPLATE, 0x6000);
        let holder = e.mem.alloc(0x10);
        e.mem.set_u32(menu + 0x50, holder);
        e.call_log = Some(vec![]);
        e.call(0x0072_7b10, &args![menu, 0xfi32, 0u32]);
        assert_eq!(e.global::<u32>(SELECTED_RECIPE_011D8E94), 0x7100);
        // No crafter: the input is listed as an error.
        assert_eq!(
            calls_to(&e, ERROR_ITEM_00464F30),
            vec![vec![ERROR_WITH_CAST_TEXT, 0]]
        );
        assert_eq!(
            calls_to(&e, 0x0000_8000),
            vec![
                vec![menu + INGREDIENT_LIST],
                vec![menu + INGREDIENT_LIST, COMPARE_00728CD0]
            ]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_INT).last().unwrap(),
            &vec![0x2000 + 0x24, 0xfa3, 0]
        );
        assert!(calls_to(&e, TILE_SET_INT)
            .iter()
            .any(|c| c == &vec![0x2000 + 0x1c, 0xfaf, 0]));
    }

    #[test]
    fn do_gamepad_maps_buttons() {
        let mut e = engine();
        let menu = recipe_menu(&mut e);
        assert!(!e.call(0x0072_8420, &args![menu, 3u32, 0u32]).bool());
        answer(&mut e, INTERFACE_IS_TOP_MENU_ID, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0072_8420, &args![menu, 3u32, 0u32]).bool());
        assert!(e.call(0x0072_8420, &args![menu, 4u32, 0u32]).bool());
        assert!(e.call(0x0072_8420, &args![menu, 14u32, 0u32]).bool());
        assert_eq!(e.global::<u8>(INGREDIENTS_FOCUSED_011D8EA5), 1);
        assert!(e.call(0x0072_8420, &args![menu, 13u32, 0u32]).bool());
        assert_eq!(e.global::<u8>(INGREDIENTS_FOCUSED_011D8EA5), 0);
        assert!(!e.call(0x0072_8420, &args![menu, 5u32, 0u32]).bool());
        assert_eq!(
            calls_to(&e, 0x0000_8000),
            vec![
                vec![menu, 2, 0],
                vec![menu, 0, 0],
                vec![menu, 6, 0],
                vec![menu, 3, 0]
            ]
        );
    }

    #[test]
    fn make_takes_inputs_and_gives_outputs() {
        let mut e = engine();
        let menu = recipe_menu(&mut e);
        e.set_global(RECIPE_MENU, menu);
        let crafter = e.mem.alloc(0x200);
        e.put_vtable(0x0c00_0000, &[0x0000_a000; 0x120]);
        answer(&mut e, 0x0000_a000, 1);
        e.mem.set_u32(crafter, 0x0c00_0000);
        e.mem.set_u32(menu + CRAFTER_REF, crafter);
        answer(&mut e, GET_PROCESS, crafter);
        let form = e.mem.alloc(4);
        e.mem.set_u32(form, 0x0c00_0000);
        let input = e.mem.alloc(8);
        e.mem.set_u32(input, 2);
        e.mem.set_u32(input + 4, form);
        let output = e.mem.alloc(8);
        e.mem.set_u32(output, 1);
        e.mem.set_u32(output + 4, form);
        let inputs = chain(&mut e, &[input]);
        let outputs = chain(&mut e, &[output]);
        answer(&mut e, RECIPE_INPUTS_005A8080, inputs);
        answer(&mut e, RECIPE_OUTPUTS_005D43C0, outputs);
        answer(&mut e, FORM_TYPE, 0x1f);
        e.call_log = Some(vec![]);
        e.call(0x0072_84f0, &args![0i32]);
        assert!(calls_to(&e, RECIPE_EVENT_005F5950).is_empty());
        e.call(0x0072_84f0, &args![3i32]);
        assert_eq!(
            calls_to(&e, RECIPE_EVENT_005F5950),
            vec![vec![0xc, 1, form, 0, 0, 0, 0]]
        );
        let removed: Vec<_> = calls_to(&e, 0x0000_a000)
            .into_iter()
            .filter(|c| c.len() == 11)
            .collect();
        assert_eq!(
            removed,
            vec![vec![crafter, form, 0, 6, 0, 0, 0, 0, 0, 1, 0]]
        );
        assert_eq!(calls_to(&e, MISC_STAT_MOD_VAL), vec![vec![0x20, 3]]);
        assert_eq!(
            calls_to(&e, ADD_ITEM_00574FA0),
            vec![vec![crafter, form, 0, 3]]
        );
        assert_eq!(calls_to(&e, SHOW_MESSAGE_007052F0).len(), 1);
        assert_eq!(calls_to(&e, FORMAT_STRING_OBJECT)[0][1], MANY_ADDED_FORMAT);
        assert_eq!(calls_to(&e, SERVICE_END), vec![vec![0]]);
    }

    #[test]
    fn refresh_request_and_idle_fade() {
        let mut e = engine();
        e.call(0x0072_8a30, &args![0u8]);
        assert_eq!(e.global::<u8>(REFRESH_011D8EA4), 1);
        e.map(0x0106_f000, 0x1000);
        e.map(0x0101_6000, 0x1000);
        e.map(0x0101_e000, 0x1000);
        e.map(0x0102_3000, 0x1000);
        e.set_global(FADE_UPPER, 485.0f32);
        e.set_global(FADE_LOWER, -230.0f32);
        e.set_global(FADE_RATE, 0.25f32);
        e.set_global(ALPHA_LIMIT_01023CD8, 255.0f32);
        e.set_global(ALPHA_TOTAL_0101E568, 255.0f64);
        e.set_global(FADING_0119FBF0, 1u8);
        e.set_global(FADE_VALUE_011D8EB8, 100.0f32);
        e.set_global(FADE_TILE_011D8EAC, 0x3100u32);
        e.set_global(FADE_TILE_011D8EB0, 0x3200u32);
        e.set_global(LAST_TIME_011D8EBC, 1000u32);
        answer(&mut e, TIMER_NOW_00825C00, 1400);
        e.call_log = Some(vec![]);
        e.call(0x0072_8a70, &args![0u32]);
        assert_eq!(e.global::<u8>(REFRESH_011D8EA4), 0);
        assert_eq!(calls_to(&e, RECIPE_LIST_HEAD_004077E0).len(), 0);
        assert_eq!(e.global::<f32>(FADE_VALUE_011D8EB8), 200.0);
        assert_eq!(e.global::<u32>(LAST_TIME_011D8EBC), 1400);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![
                vec![0x3200, 0xfa9, 155.0f32.to_bits(), 1],
                vec![0x3100, 0xfa9, 100.0f32.to_bits(), 1]
            ]
        );
        e.set_global(FADE_VALUE_011D8EB8, 485.0f32);
        e.call(0x0072_8a70, &args![0u32]);
        assert_eq!(e.global::<u8>(FADE_DOWN_011D8EA6), 1);
    }

    #[test]
    fn sort_orders_put_makeable_recipes_first() {
        let mut e = engine();
        let menu = recipe_menu(&mut e);
        e.set_global(RECIPE_MENU, menu);
        e.register(STRING_TEXT, |_, a| a[0].into_ret());
        e.register(COMPARE_STRINGS, |_, a| {
            (a[0] as i32 - a[1] as i32).into_ret()
        });
        let first = e.mem.alloc(8);
        e.mem.set_u32(first + 4, 0x7100);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second + 4, 0x7200);
        // Neither can be made (no category): by name.
        assert_eq!(e.call(0x0072_8c10, &args![first, second]).i32(), -0x100);
        // Only the second can be made (100 without a crafter): it comes first.
        e.register(RECIPE_CATEGORY_006A1B60, |_, a| {
            u32::from(a[0] == 0x7200).into_ret()
        });
        answer(&mut e, RECIPE_CATEGORY_KEY_006286D0, 1);
        assert_eq!(e.call(0x0072_8c10, &args![first, second]).i32(), 1);
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        let component_a = e.mem.alloc(8);
        e.mem.set_u32(component_a + 4, 0x5000);
        let component_b = e.mem.alloc(8);
        e.mem.set_u32(component_b + 4, 0x6000);
        e.mem.set_u32(first + 4, component_a);
        e.mem.set_u32(second + 4, component_b);
        assert_eq!(e.call(0x0072_8cd0, &args![first, second]).i32(), -0x1000);
    }

    #[test]
    fn category_filter() {
        let mut e = engine();
        e.set_global(FILTER_INDEX_0119FBF4, 0xffff_ffffu32);
        assert!(!e.call(0x0072_8d60, &args![0x7100u32]).bool());
        e.set_global(FILTER_INDEX_0119FBF4, 1u32);
        let entry = e.mem.alloc(4);
        e.mem.set_u32(entry, 0x55);
        answer(&mut e, ARRAY_AT, entry);
        answer(&mut e, RECIPE_CATEGORY_KEY_006286D0, 0x55);
        assert!(!e.call(0x0072_8d60, &args![0x7100u32]).bool());
        answer(&mut e, RECIPE_CATEGORY_KEY_006286D0, 0x56);
        assert!(e.call(0x0072_8d60, &args![0x7100u32]).bool());
    }

    #[test]
    fn item_information_for_a_weapon() {
        let mut e = engine();
        let holder = e.mem.alloc(0x10);
        let cursor_start = e.mem.alloc(4);
        e.mem.set_u32(holder + 8, cursor_start);
        let tiles = e.mem.alloc(0x30);
        for i in 0..12 {
            e.mem.set_u32(tiles + 4 * i, 0x2100 + 4 * i);
        }
        // The iterator hands out the tile array element by element.
        e.register_double(TILE_ARRAY_STEP_00683520, move |e, a| {
            let at = e.mem.u32(a[1]);
            let index = (at - cursor_start) / 4;
            let next = if index < 11 { at + 4 } else { 0 };
            e.mem.set_u32(a[1], next);
            (tiles + 4 * index).into_ret()
        });
        answer(&mut e, TEXT_TO_TRAIT, 0x31);
        answer(&mut e, FORM_TYPE, 0x28);
        st0(&mut e, WEAPON_DAMAGE_00645380, 12.0);
        st0(&mut e, CALC_WEAPON_DAMAGE_FOR_DISPLAY, 9.0);
        e.register(SCROLL_ROUND_004BD510, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])),
            ..Ret::default()
        });
        answer(&mut e, GET_NUM_PROJECTILES, 3);
        st0(&mut e, TILE_GET_FLOAT, 200.0);
        e.set_global(TWENTY_DOUBLE, 20.0f64);
        e.map(0x0102_3000, 0x1000);
        e.call_log = Some(vec![]);
        e.call(0x0072_8da0, &args![0u32, 0x7300u32, 0u32]);
        assert!(calls_to(&e, TILE_ARRAY_STEP_00683520).is_empty());
        e.call(0x0072_8da0, &args![0u32, 0x7300u32, holder]);
        assert_eq!(calls_to(&e, TILE_ARRAY_STEP_00683520).len(), 12);
        assert_eq!(e.global::<u32>(FADE_TILE_011D8EAC), 0x2104);
        assert_eq!(e.global::<u32>(FADE_TILE_011D8EB0), 0x2100 + 44);
        let formats = calls_to(&e, FORMAT);
        assert_eq!(formats[0][2..], [INT_FORMAT, 12]);
        assert_eq!(formats[1][2], PER_PROJECTILE_FORMAT);
        assert_eq!(formats[1][5], 3);
        assert_eq!(
            f64::from_bits(u64::from(formats[1][3]) | u64::from(formats[1][4]) << 32),
            3.0
        );
        // 0xc3e: tile 1 shown, tile 0 hidden.
        let shown = calls_to(&e, TILE_SET_INT);
        assert!(shown.contains(&vec![0x2104, 0xfa3, 1]));
        assert!(shown.contains(&vec![0x2100, 0xfa3, 0]));
        let y = calls_to(&e, TILE_SET_FLOAT)
            .into_iter()
            .filter(|c| c[1] == 0xfa2)
            .collect::<Vec<_>>();
        assert_eq!(y.len(), 4);
        assert_eq!(y[0], vec![0x2100 + 24, 0xfa2, 160.0f32.to_bits(), 1]);
    }
}
