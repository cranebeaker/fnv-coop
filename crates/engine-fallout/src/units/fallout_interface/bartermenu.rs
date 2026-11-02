//! `fallout/interface/menus/bartermenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Most of these are the `ListBox<ItemChange_P>` instance of the list box template. Their
//! code matches the `ListBox<BGSPerk_P>` instance of `levelupmenu.cpp` instruction for
//! instruction (only the vtable, the statics and the callees in `listbox.cpp` differ).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_interface::startmenu::ListBox;

/// `00ec7d40` (`cdecl`, one word): the absolute value of an `int`.
const ABS_00EC7D40: u32 = 0x00ec_7d40;
/// `__strupr_s` (`00ecbc5b`, `cdecl`, two words).
const STRUPR_S: u32 = 0x00ec_bc5b;

/// The `ListBox<ItemChange_P>` vtable (`01070890`).
const ITEM_CHANGE_LIST_BOX_VTABLE: u32 = 0x0107_0890;
/// `ListBoxBase` construction (`007240c0`) and the embedded list's constructor
/// (`0096a2d0`) and destructor (`0046ffb0`).
const LIST_BOX_BASE_CONSTRUCT: u32 = 0x0072_40c0;
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// `007a1950` (`thiscall`, `RET 8`): the list box's flag initialisation.
const LIST_BOX_INIT_FLAGS: u32 = 0x007a_1950;
/// `00721130` / `007211c0` (`cdecl`): the list box handed to the menu manager's list
/// and taken out of it.
const LIST_BOX_REGISTER: u32 = 0x0072_1130;
const LIST_BOX_UNREGISTER: u32 = 0x0072_11c0;
/// `MemoryManager` free (`00401030`, `cdecl`).
const FREE: u32 = 0x0040_1030;
/// The list node helpers: `006815c0` (the node's item slot), `00726070` (the next node),
/// `006ecd40` (sets the next node, `RET 4`), `004702f0` (deletes the node, `RET 4`).
const NODE_SELF: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
const NODE_SET_NEXT: u32 = 0x006e_cd40;
const NODE_DELETE: u32 = 0x0047_02f0;
/// `004459e0` (`thiscall`, `RET 4`): the item data's deleting call, given 1.
const ITEM_DATA_DELETE: u32 = 0x0044_59e0;
/// The byte that keeps item tiles alive when set (`011d8909`).
const KEEP_ITEM_TILES: u32 = 0x011d_8909;
/// `007242c0` (`thiscall`, `RET 4`): whether the list box has a flag (by bit number).
const LIST_BOX_HAS_FLAG: u32 = 0x0072_42c0;
/// `007242f0` (`thiscall`, `RET 4`): whether a tile is one of the list's items.
const LIST_BOX_BELONGS_TO: u32 = 0x0072_42f0;
/// Tile functions (`BSMenu/tile.cpp`): `Tile::AddUserTrait` (`cdecl`), `Tile::TextToTrait`
/// (`cdecl`), the float setter `00a012d0` (`RET 0xc`), `00a01290` (`RET 8`), the float
/// getter `00a011b0` (`RET 4`, `ST0`), `00a01230` (`RET 4`, a `bool`).
const ADD_USER_TRAIT: u32 = 0x00a0_0940;
const TEXT_TO_TRAIT: u32 = 0x00a0_1860;
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
const TILE_POKE: u32 = 0x00a0_1290;
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
const TILE_IS_TRUE: u32 = 0x00a0_1230;
/// `00700320` (`thiscall`, `RET 8`): sets an integer trait of a tile.
const TILE_SET_INT: u32 = 0x0070_0320;
/// `00715c60` (`thiscall`, `RET 8`): sets an integer value of a tile (`interfacemanager.cpp`).
const TILE_SET_INTEGER_VALUE: u32 = 0x0071_5c60;
/// `Tile::GetMenuTile`-like `00a03d40` and `Tile::GetMenu` `00a03c90` (`thiscall`).
const TILE_GET_MENU_TILE: u32 = 0x00a0_3d40;
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// `_ftol2_sse` (`00ec62c0`): the `ST0` value truncated (passed as an `f64`).
const FTOL2: u32 = 0x00ec_62c0;
/// `0040ebd0` (`cdecl`, two floats, `ST0`).
const FLOAT_PAIR_0040EBD0: u32 = 0x0040_ebd0;
/// `004b7210` (the interface manager) and its `00715860` (`thiscall`, `RET 0xc`): the
/// current focus target; `004b71d0` (no arguments, a `bool`).
const FOCUS_OWNER_004B7210: u32 = 0x004b_7210;
const SET_CURRENT_FOCUS_TARGET: u32 = 0x0071_5860;
const FLAG_004B71D0: u32 = 0x004b_71d0;
/// The embedded list's sort (`0083fd60`, `thiscall`, `RET 4`).
const LIST_SORT: u32 = 0x0083_fd60;
/// `ListBox<StartMenu::Option_P>::InitTilePosition` (`007269d0`, `thiscall`, `RET 4`; the
/// template code the instances share).
const INIT_TILE_POSITION: u32 = 0x0072_69d0;
/// This instance's methods in `listbox.cpp`: `Synchronize` (`0071a670`),
/// `UpdateHighlightBoxPos` (`0071a740`), `SetTotalListHeight` (`0071ad30`),
/// `NumUnfiltered` (`0071ae60`).
const ITEM_CHANGE_SYNCHRONIZE: u32 = 0x0071_a670;
const ITEM_CHANGE_UPDATE_HIGHLIGHT_BOX_POS: u32 = 0x0071_a740;
const ITEM_CHANGE_SET_TOTAL_LIST_HEIGHT: u32 = 0x0071_ad30;
const ITEM_CHANGE_NUM_UNFILTERED: u32 = 0x0071_ae60;
/// `007315b0` (`thiscall`; in `bartermenu.cpp`): the tile `Filter` highlights when the
/// remembered index has none.
const ITEM_CHANGE_007315B0: u32 = 0x0073_15b0;

/// `"_num_filtered"`, `"_current_value"`, `"_SetInCode"`, `"_enabled"`.
const NUM_FILTERED_TEXT: u32 = 0x0106_f8d0;
const CURRENT_VALUE_TEXT: u32 = 0x0107_02d8;
const SET_IN_CODE_TEXT: u32 = 0x0107_0250;
const ENABLED_TEXT: u32 = 0x0107_0244;
/// `1.0` (`double`).
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

/// `Filter`'s `"_num_filtered"` id.
const FILTER_TRAIT: StaticTrait = StaticTrait {
    guard: 0x011d_9038,
    bit: 1,
    cache: 0x011d_9034,
    name: NUM_FILTERED_TEXT,
};
/// `IsEnabled`'s `"_enabled"` id.
const ENABLED_TRAIT: StaticTrait = StaticTrait {
    guard: 0x011d_9040,
    bit: 1,
    cache: 0x011d_903c,
    name: ENABLED_TEXT,
};

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

/// The item word of a list node, reached through `006815c0` as the code does.
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let node = e.call(NODE_SELF, &args![node]).u32();
    e.mem.u32(node)
}

/// The first node of the embedded list (`this + 4`; the code guards a null `this`).
fn first_node(this: Ptr<ListBox>) -> u32 {
    if this.is_null() {
        0
    } else {
        this.addr() + 4
    }
}

/// An item tile's index trait (`0xfac`) as the code reads it: the float getter, then
/// `_ftol2`.
fn tile_index(e: &mut Engine, tile: u32) -> i32 {
    let value = e.call(TILE_GET_FLOAT, &args![tile, 0xfacu32]).f64();
    e.call(FTOL2, &args![value]).i32()
}

// Translated from 0072fa60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, one word): `0072fa80(value)`, its `EAX` returned (the decompiler
/// shows no return; `SetFilterModeTitle` and other callers use it).
pub fn fn_0072fa60(e: &mut Engine, value: i32) -> i32 {
    fn_0072fa80(e, value)
}

// Translated from 0072fa80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, one word): `00ec7d40(value)` (the absolute value), its `EAX`
/// returned.
pub fn fn_0072fa80(e: &mut Engine, value: i32) -> i32 {
    e.call(ABS_00EC7D40, &args![value]).i32()
}

// Translated from 00730490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, two words): `__strupr_s(text, size)`, its `EAX` returned.
pub fn fn_00730490(e: &mut Engine, text: u32, size: u32) -> u32 {
    e.call(STRUPR_S, &args![text, size]).u32()
}

// Translated from 00730810 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00730810 - the C++ exception frame (FS:[0] chain, unwind state 0 after the embedded list is built) is not translated.
/// `ListBox<ItemChange_P>::ListBox<ItemChange_P>` (engine map): the `ListBoxBase` base
/// (`007240c0`), the embedded list (+4, `0096a2d0`), the vtable `01070890`, the fields
/// cleared, `007a1950(this, 0, 1)` and the list box handed to `00721130`. Returns `this`.
pub fn list_box_item_change_list_box(e: &mut Engine, this: Ptr<ListBox>) -> Ptr<ListBox> {
    e.call(LIST_BOX_BASE_CONSTRUCT, &args![this]);
    e.call(LIST_CONSTRUCT, &args![this.addr() + 4]);
    e.mem.set_u32(this.addr(), ITEM_CHANGE_LIST_BOX_VTABLE);
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

// Translated from 007308e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007308e0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the body) is not translated.
/// The `ListBox<ItemChange_P>` destructor (unnamed in the engine map): the vtable,
/// `RemoveAll`, the list box handed to `007211c0`, then the embedded list destroyed
/// (`0046ffb0`).
pub fn fn_007308e0(e: &mut Engine, this: Ptr<ListBox>) {
    e.mem.set_u32(this.addr(), ITEM_CHANGE_LIST_BOX_VTABLE);
    list_box_item_change_remove_all(e, this);
    e.call(LIST_BOX_UNREGISTER, &args![this]);
    e.call(LIST_DESTRUCT, &args![this.addr() + 4]);
}

/// Deletes the `ListBoxItem` (`pTile` +0, `data` +4) that `item` reads each time: the
/// tile through its deleting destructor (vtable slot 0, flag 1) unless `KEEP_ITEM_TILES`
/// is set, the data (`004459e0` with 1) when flag bit 1 is set, then the item freed.
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

// Translated from 00730950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<ItemChange_P>::RemoveAll` (engine map): while the second node (+8) and its item
/// are non-null, deletes the item, clears the node's next pointer (`006ecd40` with 0),
/// deletes the node (`004702f0` with 1) and makes the following node the second; then
/// deletes the first item (+4). Clears the first item, `usNextIndex` and
/// `fTotalListHeight`, sets the container's `"_num_filtered"` to `0.0`, then
/// `SetTotalListHeight` (`0071ad30`), clears `pHighlightedItem` and
/// `UpdateHighlightBoxPos` (`0071a740`).
pub fn list_box_item_change_remove_all(e: &mut Engine, this: Ptr<ListBox>) {
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
    e.call(ITEM_CHANGE_SET_TOTAL_LIST_HEIGHT, &args![this]);
    e.set(this, ListBox::pHighlightedItem, Ptr::NULL);
    e.call(ITEM_CHANGE_UPDATE_HIGHLIGHT_BOX_POS, &args![this]);
}

// Translated from 00730b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<ItemChange_P>::Sort` (engine map): sorts the embedded list (`0083fd60` on +4
/// with `compare`), then `Synchronize` (`0071a670`).
pub fn list_box_item_change_sort(e: &mut Engine, this: Ptr<ListBox>, compare: u32) {
    e.call(LIST_SORT, &args![this.addr() + 4, compare]);
    e.call(ITEM_CHANGE_SYNCHRONIZE, &args![this]);
}

// Translated from 00730bb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00730bb0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<ItemChange_P>::Filter` (engine map; `thiscall`, `RET 4`): runs `filter`
/// (`cdecl`, the item's data, a `bool`) over every item. An item for which it returns
/// true is marked filtered out (its byte at +8 is 1, its index trait `0xfac` becomes -1,
/// the count of filtered items grows); the others are numbered again from 0 and
/// positioned (`InitTilePosition`). A filtered-out highlighted item is dropped and its old
/// index remembered; the item that takes that index is highlighted, else the list's tile at
/// that index (vtable +0x14 with 0), else `007315b0`. The container's `"_num_filtered"`
/// (a function-local static id) gets the count (`00715c60`), then `SetTotalListHeight` and
/// `UpdateHighlightBoxPos`. A null `filter` filters nothing out.
pub fn list_box_item_change_filter(e: &mut Engine, this: Ptr<ListBox>, filter: u32) {
    let mut filtered_count = 0i32;
    let mut remembered = -1i32;
    static_trait(e, FILTER_TRAIT);
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
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    let num_filtered = e.mem.u32(FILTER_TRAIT.cache);
    let container = e.get(this, ListBox::pContainerRect);
    e.call(
        TILE_SET_INTEGER_VALUE,
        &args![container, num_filtered, filtered_count],
    );
    e.call(ITEM_CHANGE_SET_TOTAL_LIST_HEIGHT, &args![this]);
    if e.get(this, ListBox::pHighlightedItem).is_null() && remembered >= 0 {
        let tile = e.vcall(this.addr(), 0x14, &args![remembered, 0u32]).u32();
        e.set(this, ListBox::pHighlightedItem, Ptr::new(tile));
    }
    if e.get(this, ListBox::pHighlightedItem).is_null() && remembered >= 0 {
        let tile = e.call(ITEM_CHANGE_007315B0, &args![this]).u32();
        e.set(this, ListBox::pHighlightedItem, Ptr::new(tile));
    }
    e.call(ITEM_CHANGE_UPDATE_HIGHLIGHT_BOX_POS, &args![this]);
}

// Translated from 00730e20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00730e20 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<ItemChange_P>::IsEnabled` (engine map): the container exists and its
/// `"_enabled"` trait (a function-local static id) is true.
pub fn list_box_item_change_is_enabled(e: &mut Engine, this: Ptr<ListBox>) -> bool {
    static_trait(e, ENABLED_TRAIT);
    if e.get(this, ListBox::pContainerRect).is_null() {
        return false;
    }
    let enabled = e.mem.u32(ENABLED_TRAIT.cache);
    let container = e.get(this, ListBox::pContainerRect);
    e.call(TILE_IS_TRUE, &args![container, enabled]).bool()
}

// Translated from 00730ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<ItemChange_P>::Highlight` (engine map): when the list is enabled and `tile` is
/// null, or is one of its items (`007242f0`) and `"_enabled"`, makes it
/// `pHighlightedItem`, moves the highlight box (`0071a740`) and returns true; otherwise
/// false.
pub fn list_box_item_change_highlight(e: &mut Engine, this: Ptr<ListBox>, tile: u32) -> bool {
    if !list_box_item_change_is_enabled(e, this) {
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
    e.call(ITEM_CHANGE_UPDATE_HIGHLIGHT_BOX_POS, &args![this]);
    true
}

// Translated from 00731360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<ItemChange_P>::RestoreListState` (engine map): with a container and a scroll
/// bar, gives the scroll bar `"_current_value"` (the saved scroll bar value, word 0) and
/// pokes `"_SetInCode"` with `1.0`; takes the tile at index `_ftol(0040ebd0(
/// fSavedHighlightIndex, NumUnfiltered - 1.0))` (vtable +0x14 with 0; `NumUnfiltered` is
/// `0071ae60`), makes it the interface's focus target (`00715860` with trait `0xfc3` and
/// `focus`) and highlights it (vtable +0). Unless `004b71d0` says so, also scrolls the
/// container's menu to it (the menu's vtable +0x14 with the tile's index trait `0xfaa` and
/// the tile, when there is a tile) and highlights nothing.
pub fn list_box_item_change_restore_list_state(e: &mut Engine, this: Ptr<ListBox>, focus: u8) {
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
    let count = e.call(ITEM_CHANGE_NUM_UNFILTERED, &args![this]).u32();
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

// Translated from 007314c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 0xc`: a callback and two indices). Walks the items in order
/// while the node and its item are non-null: stops at the first whose index trait (`0xfac`,
/// read as a float and truncated by `_ftol2`) is at least `end`; for those whose index is at
/// least `start`, calls `callback(tile, data)` (`cdecl`). Signed comparisons.
pub fn fn_007314c0(e: &mut Engine, this: Ptr<ListBox>, callback: u32, start: i32, end: i32) {
    let mut node = first_node(this);
    while node != 0 && node_item(e, node) != 0 {
        let item = node_item(e, node);
        let tile = e.mem.u32(item);
        if tile_index(e, tile) >= end {
            return;
        }
        let item = node_item(e, node);
        let tile = e.mem.u32(item);
        if tile_index(e, tile) >= start {
            let item = node_item(e, node);
            let data = e.mem.u32(item + 4);
            let item = node_item(e, node);
            let tile = e.mem.u32(item);
            e.call(callback, &args![tile, data]);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

// ---------------------------------------------------------------------------
// `BarterMenu` (batch n051)

layout! {
    /// `BarterMenu` (Xbox PDB), 0x120 bytes; the PC code uses the same field offsets
    /// (the constructor `0072ceb0` writes them).
    pub struct BarterMenu: 0x120 {
        /// `pVendorRef` (Xbox PDB): `TESObjectREFR *`.
        0x80 pVendorRef: u32,
        /// `fTransactionAmount` (Xbox PDB).
        0x84 fTransactionAmount: f32,
        /// `iDiscount` (Xbox PDB).
        0x88 iDiscount: i32,
        /// `iPlayerCaps` (Xbox PDB).
        0x8C iPlayerCaps: i32,
        /// `iVendorCaps` (Xbox PDB).
        0x90 iVendorCaps: i32,
        /// `fBuyMult` (Xbox PDB).
        0x94 fBuyMult: f32,
        /// `fSellMult` (Xbox PDB).
        0x98 fSellMult: f32,
        /// `ePlayerFilterMode` (Xbox PDB): 0 to 5.
        0x9C ePlayerFilterMode: i32,
        /// `eVendorFilterMode` (Xbox PDB): 0 to 5.
        0xA0 eVendorFilterMode: i32,
        /// `eFilterSound` (Xbox PDB): a `MENU_SOUNDS` value.
        0xA4 eFilterSound: i32,
        /// `plbCurrListbox` (Xbox PDB): the player's (+0xa8) or the vendor's (+0xd8) list.
        0x108 plbCurrListbox: u32,
        /// `bItemsTraded` (Xbox PDB).
        0x11C bItemsTraded: u8,
    }
}

/// `pTiles` (Xbox PDB) starts at +0x28; the code indexes it with tile ids below 0x16.
const BARTER_TILES: u32 = 0x28;
/// `lbPlayerItems` and `lbVendorItems` (Xbox PDB), `ListBox<ItemChange *>`.
const PLAYER_ITEMS: u32 = 0xa8;
const VENDOR_ITEMS: u32 = 0xd8;
/// `ItemsToBuy` and `ItemsToSell` (Xbox PDB), `BSSimpleList<ItemChange *>`.
const ITEMS_TO_BUY: u32 = 0x10c;
const ITEMS_TO_SELL: u32 = 0x114;

/// `BarterMenu::pMe` (Xbox PDB static): the menu singleton.
const BARTER_MENU: u32 = 0x011d_8fa4;
/// `BarterMenu::pCurrItem` (Xbox PDB static): the clicked item.
const BARTER_CURRENT_ITEM: u32 = 0x011d_8fa8;
/// The trait ids the constructor looks up, kept in statics: `"_Title"` (`011d8fac`),
/// `"_Value"` (`011d8fb0`), `"_IsBarterSelected"` (`011d8fb4`),
/// `"_IsContainerListSelected"` (`011d8fb8`), `"_ArrowDirection"` (`011d8fbc`).
const TITLE_TRAIT: u32 = 0x011d_8fac;
const VALUE_TRAIT: u32 = 0x011d_8fb0;
const IS_BARTER_SELECTED_TRAIT: u32 = 0x011d_8fb4;
const IS_CONTAINER_LIST_SELECTED_TRAIT: u32 = 0x011d_8fb8;
const ARROW_DIRECTION_TRAIT: u32 = 0x011d_8fbc;
/// A byte static `DoClick` clears and `DoEnter` sets on the item tiles (its name is not
/// confirmed).
const ITEM_ENTERED_0119FDB0: u32 = 0x0119_fdb0;
/// The player (`PlayerCharacter *` global) and the data handler (`TESDataHandler *`).
const PLAYER: u32 = 0x011d_ea3c;
const DATA_HANDLER: u32 = 0x011c_3f2c;

/// `BarterMenu`'s vtable (`010706ec`) and its class number.
const BARTER_MENU_VTABLE: u32 = 0x0107_06ec;
const BARTER_MENU_CLASS: u32 = 0x41d;

/// `Menu::Menu` (`00a1c4a0`) and the `Menu` destructor body (`00a1c520`).
const MENU_CONSTRUCT: u32 = 0x00a1_c4a0;
const MENU_DESTRUCT: u32 = 0x00a1_c520;
/// `BSSimpleList` clearing (`00470470`, `thiscall`).
const LIST_CLEAR: u32 = 0x0047_0470;
/// `TESDataHandler::ClearBarterContainer` (`0046faf0`) and its set counterpart
/// (`0046f310`, `RET 4`: the vendor).
const CLEAR_BARTER_CONTAINER: u32 = 0x0046_faf0;
const SET_BARTER_CONTAINER_0046F310: u32 = 0x0046_f310;
/// The interface manager singleton (`004b7210`) and its tile reader (`00586150`).
const TILE_READER: u32 = 0x0058_6150;
/// `Tile::ReadFile` (`00a01b00`, `RET 4`), `Tile::GetMenuByClass` (`00a09030`, cdecl),
/// `Tile::GetChildByName` (`00a03da0`, `RET 4`), the string trait setter (`00a01350`,
/// `RET 0xc`), `Tile::GetString` (`00a011f0`, `RET 4`) and `Tile::GetMaximumDepth`
/// (`00a01530`, no arguments, `ST0`).
const TILE_READ_FILE: u32 = 0x00a0_1b00;
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
const TILE_GET_CHILD_BY_NAME: u32 = 0x00a0_3da0;
const TILE_SET_STRING: u32 = 0x00a0_1350;
const TILE_GET_STRING: u32 = 0x00a0_11f0;
const TILE_GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
/// `Menu::SetMenuTile` (`00a1dc70`, `RET 8`), `Menu::PrepForVisibility` (`00a1dc20`,
/// `RET 4`), `Menu::StartFadeOut` (`00a1d910`).
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
const MENU_START_FADE_OUT: u32 = 0x00a1_d910;
/// `00796f90` (`thiscall`, a `bool`): whether the menu's tiles are all present
/// (`mapmenu.cpp` in the engine map; the body is not confirmed here).
const TILES_PRESENT_00796F90: u32 = 0x0079_6f90;
/// The print function a missing vendor or menu is reported with (`005b5e40`, cdecl).
const PRINT_MESSAGE: u32 = 0x005b_5e40;
/// `DialogMenu::ServiceStart` (`00763ff0`) and `ServiceEnd` (`007640a0`, cdecl).
const DIALOG_SERVICE_START: u32 = 0x0076_3ff0;
const DIALOG_SERVICE_END: u32 = 0x0076_40a0;
/// The tutorial message shower (`00718630`, `this` = interface + 0x4d4, `RET 0xc`).
const SHOW_TUTORIAL: u32 = 0x0071_8630;
/// `ListBox<unsigned int>::Initialize` (`0075a0d0`, `RET 8`).
const LIST_BOX_INITIALIZE: u32 = 0x0075_a0d0;
/// A setting's string (`00403df0`) and value address (`0043d4d0`), `thiscall` on the
/// setting.
const SETTING_STRING: u32 = 0x0040_3df0;
const SETTING_VALUE_ADDRESS: u32 = 0x0043_d4d0;
/// `00647b70` and `004a8f20` (cdecl, two words each): the discount is passed through the
/// first with -100 and the second with 100.
const BOUND_00647B70: u32 = 0x0064_7b70;
const BOUND_004A8F20: u32 = 0x004a_8f20;
/// `ActorValueOwner::GetClampedActorValue` (`0066ef20`, `RET 4`) and
/// `GamePlayFormulas::GetBarterMultipliers` (`00649050`, cdecl, three words).
const GET_CLAMPED_ACTOR_VALUE: u32 = 0x0066_ef20;
const GET_BARTER_MULTIPLIERS: u32 = 0x0064_9050;
/// `InterfaceManager::PlayMenuSound` (`00717280`, cdecl) and `Interface::PlayMenuSound`
/// (`00706f30`, cdecl).
const MANAGER_PLAY_MENU_SOUND: u32 = 0x0071_7280;
const PLAY_MENU_SOUND: u32 = 0x0070_6f30;
/// `MiddleHighProcess::GetSavedAcquireObject` (`008d8520`; the player's process, by the
/// engine map's name for the folded code) and `PlayerCharacter::UpdatePlayer3d`
/// (`00950030`).
const ACTOR_PROCESS: u32 = 0x008d_8520;
const UPDATE_PLAYER_3D: u32 = 0x0095_0030;
/// `TESTopic::GetTopic` (`0061a2d0`, cdecl, two words).
const GET_TOPIC: u32 = 0x0061_a2d0;
/// `Interface::GetMessageMenuResult` (`00703fa0`, no arguments).
const GET_MESSAGE_MENU_RESULT: u32 = 0x0070_3fa0;
/// `Interface::IsConsoleVisible` (`00703d50`), the console print (`00703c00`, cdecl,
/// variadic) and the message box (`00703e80`, cdecl, variadic).
const IS_CONSOLE_VISIBLE: u32 = 0x0070_3d50;
const CONSOLE_PRINT: u32 = 0x0070_3c00;
const SHOW_MESSAGE_BOX: u32 = 0x0070_3e80;
/// `ExtraDataList::GetOriginalReference` (`00418630`) and the form id getter
/// `0084e3a0` (`thiscall`).
const GET_ORIGINAL_REFERENCE: u32 = 0x0041_8630;
const FORM_ID_0084E3A0: u32 = 0x0084_e3a0;
/// The first word of an object (`00559450`).
const FIRST_WORD: u32 = 0x0055_9450;
/// `0x00406d90` (cdecl, one `float`): the `float` as an integer.
const FLOAT_TO_INT_00406D90: u32 = 0x0040_6d90;
/// `ListBox::SaveListState` (`007312e0`, the map's name from `mapmenu.cpp`) and
/// `QuantityMenu::Create` (`007aba00`, cdecl, three words).
const LIST_SAVE_STATE: u32 = 0x0073_12e0;
const QUANTITY_MENU_CREATE: u32 = 0x007a_ba00;
/// The list count (`005ae380`, `thiscall`; the map's folded name is `VATS::GetCount`).
const LIST_COUNT_005AE380: u32 = 0x005a_e380;
/// `Actor::GetGoldAmount` (`00891d70`), the data handler's barter container
/// (`00740960`), `InventoryChanges::GetGoldAmount` (`004cb320`).
const ACTOR_GET_GOLD_AMOUNT: u32 = 0x0089_1d70;
const BARTER_CONTAINER_00740960: u32 = 0x0074_0960;
const INVENTORY_GET_GOLD_AMOUNT: u32 = 0x004c_b320;
/// `_snprintf`-like formatter (`00406d00`, cdecl, variadic) and copier (`00406d30`,
/// cdecl, three words).
const FORMAT_TEXT: u32 = 0x0040_6d00;
const COPY_TEXT: u32 = 0x0040_6d30;
/// `InventoryChanges::GetInventoryChanges` (`004bf220`, cdecl),
/// `InventoryChanges::GetItemGroup` (`004d4b00`, `RET 8`).
const GET_INVENTORY_CHANGES: u32 = 0x004b_f220;
const GET_ITEM_GROUP: u32 = 0x004d_4b00;
/// `ListBox<ItemChange *>`'s `RemoveItemGroup` (`0071a200`), `AddItem` (`0071a320`),
/// the inventory fill (`00719ef0`, `RET 0x18`) and the item count (`00726c40`).
const LIST_REMOVE_ITEM_GROUP: u32 = 0x0071_a200;
const LIST_ADD_ITEM: u32 = 0x0071_a320;
const LIST_FILL: u32 = 0x0071_9ef0;
const LIST_ITEM_COUNT: u32 = 0x0072_6c40;
/// `ItemChange::GetFullName` (`004be2d0`), the item's object (`0044ddc0`),
/// `ItemChange::ItemChange` (`004bc550`, `RET 8`), `ItemChange::GetItemValue`
/// (`004bd400`, `ST0`).
const ITEM_FULL_NAME: u32 = 0x004b_e2d0;
const ITEM_OBJECT: u32 = 0x0044_ddc0;
const ITEM_CHANGE_CONSTRUCT: u32 = 0x004b_c550;
const ITEM_GET_VALUE: u32 = 0x004b_d400;
/// `00401000` (`operator new`, cdecl).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `005ae3d0` (`thiscall`, `RET 4`): adds the word at the given address to a list.
const LIST_ADD_005AE3D0: u32 = 0x005a_e3d0;
/// `007d6e90` (`thiscall`): the list's highlighted tile.
const LIST_HIGHLIGHTED_007D6E90: u32 = 0x007d_6e90;
/// `007a1910` (`thiscall`): the list's selected item.
const LIST_SELECTED_007A1910: u32 = 0x007a_1910;
/// `Interface::PopulateItemStatsDisplay` (`00707e30`, cdecl, two words) and `00707e10`
/// (`thiscall`, `RET 4`, `interface.cpp`).
const ITEM_CARD_00707E30: u32 = 0x0070_7e30;
const TILE_SET_ITEM_00707E10: u32 = 0x0070_7e10;
/// `ItemChange::GetIconOne` (`004be200`, `thiscall`, `RET 8`) on the clicked item.
const ITEM_DESCRIPTION_004BE200: u32 = 0x004b_e200;
/// `TESBipedModelForm::GetFormAsBipedModel` (`00480db0`, cdecl) and `GetPlayable`
/// (`00480d90`).
const AS_BIPED_MODEL: u32 = 0x0048_0db0;
const BIPED_IS_PLAYABLE: u32 = 0x0048_0d90;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words) and its two type descriptors.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const CAST_SOURCE_TYPE: u32 = 0x0118_3108;
const CAST_TARGET_TYPE: u32 = 0x0118_3158;
/// A form's type (`00401170`).
const FORM_TYPE: u32 = 0x0040_1170;
/// `TESContainer::IsGold` (`00481f10`, cdecl).
const IS_GOLD: u32 = 0x0048_1f10;
/// `0048cee0` (`thiscall`) on the cast form, `0047bcf0` on a form of type 0x28 and
/// `004c94d0` on a form of type 0x29 (`thiscall`, `bool`).
const CAST_TEST_0048CEE0: u32 = 0x0048_cee0;
const TYPE_28_TEST_0047BCF0: u32 = 0x0047_bcf0;
const TYPE_29_TEST_004C94D0: u32 = 0x004c_94d0;
/// `004181e0` (`thiscall`, plain `RET`, no stack word) on the vendor, then `0047efe0`
/// on the result + 0x90 (`RET 4`: the flag, a `bool`).
const VENDOR_SERVICES_004181E0: u32 = 0x0041_81e0;
const SERVICE_FLAG_0047EFE0: u32 = 0x0047_efe0;
/// `TESValueForm::GetFormValue` (`0048e8a0`, cdecl) and `BGSEquipType::GetEquipType`
/// (`00479430`, cdecl).
const GET_FORM_VALUE: u32 = 0x0048_e8a0;
const GET_EQUIP_TYPE: u32 = 0x0047_9430;
/// `TESObjectREFR`'s constructor (`0055a2f0`) and destructor (`0055a430`), the setters
/// `00567ad0`, `TESObjectREFR::SetGlobal` (`00567b50`), `00567bd0`,
/// `TESObjectREFR::SetObjectReference` (`00575690`), and `TESObjectREFR::IsAnOwner`
/// (`005785e0`, `RET 8`).
const REFERENCE_CONSTRUCT: u32 = 0x0055_a2f0;
const REFERENCE_DESTRUCT: u32 = 0x0055_a430;
const REFERENCE_SET_OWNER_00567AD0: u32 = 0x0056_7ad0;
const REFERENCE_SET_GLOBAL: u32 = 0x0056_7b50;
const REFERENCE_SET_RANK_00567BD0: u32 = 0x0056_7bd0;
const REFERENCE_SET_OBJECT: u32 = 0x0057_5690;
const REFERENCE_IS_AN_OWNER: u32 = 0x0057_85e0;
/// The extra data list's owner (`00418660`), `ExtraDataList::GetGlobal` (`00418690`),
/// `ExtraDataList::GetRank` (`004186c0`), `ExtraDataList::GetCount` (`00418770`, a
/// 16-bit count).
const EXTRA_GET_OWNER_00418660: u32 = 0x0041_8660;
const EXTRA_GET_GLOBAL: u32 = 0x0041_8690;
const EXTRA_GET_RANK: u32 = 0x0041_86c0;
const EXTRA_GET_COUNT: u32 = 0x0041_8770;
/// `00905330` (`thiscall`, `RET 4`) on the item's extra list: removes an entry.
const EXTRA_REMOVE_00905330: u32 = 0x0090_5330;
/// `BGSEntryPoint::HandleEntryPoint` (`005e58f0`, cdecl, variadic).
const HANDLE_ENTRY_POINT: u32 = 0x005e_58f0;
/// `00408860` (cdecl, a `float`, `ST0`), `004bd510` (cdecl, two `float`s, `ST0`),
/// `00404010` (cdecl, two `float`s, `ST0`).
const VALUE_00408860: u32 = 0x0040_8860;
const FLOAT_PAIR_004BD510: u32 = 0x004b_d510;
const FLOAT_PAIR_00404010: u32 = 0x0040_4010;
/// `00469880` (cdecl, two strings): the string comparison `SortFunc` answers.
const STRING_COMPARE_00469880: u32 = 0x0046_9880;
/// `ItemChange::GetWorn` (`004bddd0`, `thiscall`, `RET 4`) on the item (the trait it
/// sets is `"_IsEquipped"`).
const ITEM_IS_EQUIPPED_004BDDD0: u32 = 0x004b_ddd0;
/// `Interface::IsTopMenuID` (`00702450`, cdecl, a menu class).
const MENU_IS_TOP_00702450: u32 = 0x0070_2450;
/// `0070ec70` (`thiscall` on the interface manager): the wheel direction.
const WHEEL_DIRECTION_0070EC70: u32 = 0x0070_ec70;
/// `ListBox<StartMenu::Option_P>::GetTileFor` (`007a22d0`, the template code the
/// instances share; `RET 4`: the address of an item word).
const LIST_GET_TILE_FOR: u32 = 0x007a_22d0;
/// `008aded0` (`thiscall` on the player, `RET 0xc`, `actor.cpp`).
const PLAYER_BARTER_008ADED0: u32 = 0x008a_ded0;
/// `008256d0` (`thiscall` on a `BSSimpleList`, a `bool`): whether the list is empty.
const LIST_IS_EMPTY_008256D0: u32 = 0x0082_56d0;
/// `00406ce0` (cdecl, a `float`, `ST0`).
const VALUE_00406CE0: u32 = 0x0040_6ce0;
/// `00905820` (`thiscall` on a `BSSimpleList`, `RET 4`: the address of an item word):
/// adds the item.
const LIST_APPEND_00905820: u32 = 0x0090_5820;
/// `"%d"` (`01020764`).
const INTEGER_FORMAT: u32 = 0x0102_0764;
/// The amount's alphas: 64.0 (`0101e6ec`) and 255.0 (`01023cd8`).
const DIM_ALPHA: u32 = 0x0101_e6ec;
const FULL_ALPHA: u32 = 0x0102_3cd8;
/// `"_scroll_delta"` (`0106f90c`).
const SCROLL_DELTA_TEXT: u32 = 0x0106_f90c;
/// `BSAudio::QInstance` (`00453a70`), `BSAudio::GetSoundHandleByName` (`00ad7550`,
/// `RET 0x10`), `BSSoundHandle::Play` (`00ad8830`, `RET 4`) and the handle's destructor
/// (`00483710`).
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const AUDIO_SOUND_BY_NAME: u32 = 0x00ad_7550;
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
const SOUND_HANDLE_DESTRUCT: u32 = 0x0048_3710;
/// The security cookie (`.data`).
const SECURITY_COOKIE: u32 = 0x011c_16bc;
/// `005d43c0` (`thiscall`, `tesscriptfunctions.cpp`): the reference's extra data list,
/// and `ExtraDataList::GetMerchantContainer` (`00421400`).
const EXTRA_LIST_005D43C0: u32 = 0x005d_43c0;
const GET_MERCHANT_CONTAINER: u32 = 0x0042_1400;
/// `Script::SetActionFlag` (`005ac750`, cdecl, three words).
const SET_ACTION_FLAG: u32 = 0x005a_c750;
/// `TESDataHandler::RemoveItemBarterContainer` (`0046f640`, `RET 0xc`).
const REMOVE_ITEM_BARTER_CONTAINER: u32 = 0x0046_f640;
/// `MiscStatManager::ModVal` (`004d5e10`, cdecl, two words).
const MISC_STAT_MOD_VAL: u32 = 0x004d_5e10;
/// `004839c0` (cdecl, one word, `tesform.cpp`): the form the caps `ItemChange` is made of.
const CAPS_FORM_004839C0: u32 = 0x0048_39c0;
/// The `ItemChange` destructor (`004bc5f0`).
const ITEM_CHANGE_DESTRUCT: u32 = 0x004b_c5f0;
/// `InventoryChanges::RemoveGold` (`004cb4b0`, `RET 0xc`) and
/// `InventoryChanges::GetObjectCount` (`004c8f30`, `RET 4`).
const INVENTORY_REMOVE_GOLD: u32 = 0x004c_b4b0;
const GET_OBJECT_COUNT: u32 = 0x004c_8f30;
/// `"ITMBottlecapsUp"` (`0107087c`) and `"ITMBottlecapsDown"` (`01070868`).
const CAPS_UP_SOUND: u32 = 0x0107_087c;
const CAPS_DOWN_SOUND: u32 = 0x0107_0868;
/// The language setting (`011c3cb4`), `"ENGLISH"` (`01018348`) and the comparison
/// `00408b20` (cdecl, two strings).
const LANGUAGE_SETTING: u32 = 0x011c_3cb4;
const ENGLISH_TEXT: u32 = 0x0101_8348;
const COMPARE_NAME_00408B20: u32 = 0x0040_8b20;
/// The display name of a reference (`0055d520`, `thiscall`).
const REFERENCE_NAME: u32 = 0x0055_d520;
/// The byte that asks `DoIdle` for a list refresh (`011d8fc0`; its name is not
/// confirmed).
const REFRESH_REQUESTED_011D8FC0: u32 = 0x011d_8fc0;
/// `00719790` (cdecl, five words, `interfacemanager.cpp`): the item stats display.
const SHOW_ITEM_STATS_00719790: u32 = 0x0071_9790;
/// `004be060` and `004be180` (`thiscall` on an object, `ST0`; `inventorychanges.cpp`).
const STAT_004BE060: u32 = 0x004b_e060;
const STAT_004BE180: u32 = 0x004b_e180;

/// `"Barter menu called with NULL vendor!  Oh, noes!"` (`010707d0`),
/// `"Data\Menus\barter_menu.xml"` (`010707b4`), `"MENUS: Barter Menu Creation Failed...
/// Are your menu and art resources up to date?"` (`01070760`), `"BM_CapsLabel"`,
/// `"WeightInfo"`, `"ValueInfo"`, `"CM_list_template"`, `"_NumBartered"`, the two
/// `"Original Container Ref: ..."` formats, `"%s   %d"`, `"_Title"`, `"_Value"`,
/// `"_IsBarterSelected"`, `"_IsContainerListSelected"`, `"_ArrowDirection"`,
/// `"%s (%d)"`, `"_IsEquipped"`, `"--"`, `"%.1f"`, `"%.0f"`.
const NULL_VENDOR_TEXT: u32 = 0x0107_07d0;
const BARTER_MENU_FILE: u32 = 0x0107_07b4;
const CREATION_FAILED_TEXT: u32 = 0x0107_0760;
const CAPS_LABEL_NAME: u32 = 0x0107_074c;
const WEIGHT_INFO_NAME: u32 = 0x0107_0740;
const VALUE_INFO_NAME: u32 = 0x0107_0734;
const LIST_TEMPLATE_NAME: u32 = 0x0107_04d4;
const NUM_BARTERED_TEXT: u32 = 0x0107_0844;
const ORIGINAL_REFERENCE_FORMAT: u32 = 0x0107_0820;
const NO_ORIGINAL_REFERENCE_TEXT: u32 = 0x0107_0800;
const CAPS_FORMAT: u32 = 0x0107_0854;
const TITLE_TEXT: u32 = 0x0106_ed20;
const VALUE_TEXT: u32 = 0x0106_ed18;
const IS_BARTER_SELECTED_TEXT: u32 = 0x0107_06d0;
const IS_CONTAINER_LIST_SELECTED_TEXT: u32 = 0x0107_06b4;
const ARROW_DIRECTION_TEXT: u32 = 0x0107_06a4;
const NAME_COUNT_FORMAT: u32 = 0x0106_eb00;
const IS_EQUIPPED_TEXT: u32 = 0x0107_085c;
const NO_VALUE_TEXT: u32 = 0x0104_7008;
const ONE_DECIMAL_FORMAT: u32 = 0x0102_0768;
const NO_DECIMAL_FORMAT: u32 = 0x0106_ecc8;
/// The doubles 6006.0 (`010704f0`) and 102.0 (`010704e8`) the menu's stacking type is
/// compared with, 0.0 (`01012060`) and 100.0 (`01017a40`); the floats -1.0 (`01012054`)
/// and 0.1 (`0101e2bc`).
const STACKING_TYPE_ONE: u32 = 0x0107_04f0;
const STACKING_TYPE_TWO: u32 = 0x0107_04e8;
const ZERO_DOUBLE: u32 = 0x0101_2060;
const HUNDRED_DOUBLE: u32 = 0x0101_7a40;
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;
const TENTH_FLOAT: u32 = 0x0101_e2bc;

/// The menu singleton's address.
fn barter_menu(e: &Engine) -> Ptr<BarterMenu> {
    Ptr::new(e.global::<u32>(BARTER_MENU))
}

/// The word of `pTiles` at `index` (`this + 0x28 + 4 * index`).
fn barter_tile(e: &Engine, this: Ptr<BarterMenu>, index: u32) -> u32 {
    e.mem
        .u32(this.addr().wrapping_add(BARTER_TILES + 4 * index))
}

/// The item word at a list node (`006815c0`, read), or 0 for a null node or item: the
/// loop condition the code repeats for every `BSSimpleList` walk.
fn walk_item(e: &mut Engine, node: u32) -> u32 {
    if node == 0 {
        0
    } else {
        node_item(e, node)
    }
}

/// The extra data list's first item (`[006815c0(00559450(object))]`), or 0 when the
/// object's first word is null.
fn first_extra(e: &mut Engine, object: u32) -> u32 {
    if e.call(FIRST_WORD, &args![object]).u32() == 0 {
        return 0;
    }
    let list = e.call(FIRST_WORD, &args![object]).u32();
    node_item(e, list)
}

/// [`first_extra`] of a node's item, the item read through the node (`006815c0`) each
/// time, as the code does.
fn node_first_extra(e: &mut Engine, node: u32) -> u32 {
    let item = node_item(e, node);
    if e.call(FIRST_WORD, &args![item]).u32() == 0 {
        return 0;
    }
    let item = node_item(e, node);
    let list = e.call(FIRST_WORD, &args![item]).u32();
    node_item(e, list)
}

/// `00a01350(tile, trait, text, 1)`: sets a string trait.
fn set_string(e: &mut Engine, tile: u32, which: u32, text: u32) {
    e.call(TILE_SET_STRING, &args![tile, which, text, 1u32]);
}

// Translated from 0072ceb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0072ceb0 - the C++ exception-unwinding frame (FS:[0], states 0 to 4 over the members) is not translated.
/// `BarterMenu::BarterMenu` (Xbox PDB): the `Menu` base (`00a1c4a0`), the vtable
/// `010706ec`, both item lists (`0x00730810` on +0xa8 and +0xd8), the buy and sell lists
/// (`0096a2d0` on +0x10c and +0x114); looks up the five trait ids into their statics,
/// clears `pCurrItem`, `plbCurrListbox`, both cap counts, `bItemsTraded` and the filter
/// modes, sets both multipliers to 1.0 and the filter sound to 3. Returns `this`.
pub fn barter_menu_barter_menu(e: &mut Engine, this: Ptr<BarterMenu>) -> Ptr<BarterMenu> {
    let base = this.addr();
    e.call(MENU_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, BARTER_MENU_VTABLE);
    list_box_item_change_list_box(e, Ptr::new(base + PLAYER_ITEMS));
    list_box_item_change_list_box(e, Ptr::new(base + VENDOR_ITEMS));
    e.call(LIST_CONSTRUCT, &args![base + ITEMS_TO_BUY]);
    e.call(LIST_CONSTRUCT, &args![base + ITEMS_TO_SELL]);
    for (name, cache) in [
        (TITLE_TEXT, TITLE_TRAIT),
        (VALUE_TEXT, VALUE_TRAIT),
        (IS_BARTER_SELECTED_TEXT, IS_BARTER_SELECTED_TRAIT),
        (
            IS_CONTAINER_LIST_SELECTED_TEXT,
            IS_CONTAINER_LIST_SELECTED_TRAIT,
        ),
        (ARROW_DIRECTION_TEXT, ARROW_DIRECTION_TRAIT),
    ] {
        let id = e.call(TEXT_TO_TRAIT, &args![name]).u32();
        e.set_global(cache, id);
    }
    e.set_global(BARTER_CURRENT_ITEM, 0u32);
    e.set(this, BarterMenu::plbCurrListbox, 0);
    e.set(this, BarterMenu::iPlayerCaps, 0);
    e.set(this, BarterMenu::iVendorCaps, 0);
    e.mem.set_u32(base + 0x94, x87_store32(1.0));
    e.mem.set_u32(base + 0x98, x87_store32(1.0));
    e.set(this, BarterMenu::bItemsTraded, 0);
    e.set(this, BarterMenu::ePlayerFilterMode, 0);
    e.set(this, BarterMenu::eVendorFilterMode, 0);
    e.set(this, BarterMenu::eFilterSound, 3);
    this
}

// Translated from 0072d030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BarterMenu::GetClass` (Xbox PDB): the class number `0x41d`.
pub fn barter_menu_get_class(_e: &mut Engine, _this: Ptr<BarterMenu>) -> u32 {
    BARTER_MENU_CLASS
}

// Translated from 0072d070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BarterMenu::`scalar deleting destructor'` (Xbox PDB): the destructor body
/// (`0072d0a0`), then the block freed (`00401030`) when bit 0 of `flags` is set.
/// Returns `this`.
pub fn barter_menu_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BarterMenu>,
    flags: u32,
) -> Ptr<BarterMenu> {
    fn_0072d0a0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

/// Walks a `BSSimpleList<ItemChange *>` from `node`, deleting every item
/// (`004459e0` with 1).
fn delete_list_items(e: &mut Engine, mut node: u32) {
    while walk_item(e, node) != 0 {
        let item = node_item(e, node);
        if item != 0 {
            e.call(ITEM_DATA_DELETE, &args![item, 1u32]);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 0072d0a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0072d0a0 - the C++ exception-unwinding frame (FS:[0], states 4 down to 0) is not translated.
/// The `BarterMenu` destructor body (unnamed in the engine map): the vtable; when the
/// singleton is set, deletes every item of its buy and sell lists (`004459e0` with 1)
/// and clears both lists (`00470470`); clears the data handler's barter container
/// (`0046faf0`) and the singleton, destroys the sell and buy lists (`0046ffb0`), the
/// vendor's and the player's item lists (`007308e0`) and runs the `Menu` destructor.
pub fn fn_0072d0a0(e: &mut Engine, this: Ptr<BarterMenu>) {
    let base = this.addr();
    e.mem.set_u32(base, BARTER_MENU_VTABLE);
    if barter_menu(e).addr() != 0 {
        delete_list_items(e, barter_menu(e).addr() + ITEMS_TO_BUY);
        delete_list_items(e, barter_menu(e).addr() + ITEMS_TO_SELL);
        let me = barter_menu(e).addr();
        e.call(LIST_CLEAR, &args![me + ITEMS_TO_BUY]);
        let me = barter_menu(e).addr();
        e.call(LIST_CLEAR, &args![me + ITEMS_TO_SELL]);
    }
    let handler = e.global::<u32>(DATA_HANDLER);
    e.call(CLEAR_BARTER_CONTAINER, &args![handler]);
    e.set_global(BARTER_MENU, 0u32);
    e.call(LIST_DESTRUCT, &args![base + ITEMS_TO_SELL]);
    e.call(LIST_DESTRUCT, &args![base + ITEMS_TO_BUY]);
    fn_007308e0(e, Ptr::new(base + VENDOR_ITEMS));
    fn_007308e0(e, Ptr::new(base + PLAYER_ITEMS));
    e.call(MENU_DESTRUCT, &args![this]);
}

// Translated from 0072d250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BarterMenu::Create` (Xbox PDB, cdecl): opens the barter menu on `vendor` with the
/// discount `discount`. Without a vendor prints `"Barter menu called with NULL
/// vendor!..."` (with 0x12) and answers 0. Deletes an open menu of the class, reads
/// `"Data\Menus\barter_menu.xml"` and, for a menu of this class: attaches the tile;
/// without all its tiles (`00796f90`) prints the creation-failed message and answers 0;
/// sets the depth trait `0xfad` to the maximum depth for the stacking types 6006 and 102
/// (trait `0xfa7`); starts the dialogue service, shows tutorial message 0x17, sets up
/// both item lists (`"CM_list_template"`), labels the title tiles from their settings
/// and the five tab tiles that exist (trait `"_Title"`), stores the vendor, the discount
/// bounded by -100 and 100 (`00647b70`, `004a8f20`), a zero transaction amount and the
/// barter multipliers (actor value 0x20 of the player), plays menu sound 0x24, prepares
/// the menu for showing and fills the lists (`0072dc30` with 0). Returns the root tile.
/// A menu of another class is deleted (when it has a tile) and the answer is 0.
pub fn barter_menu_create(e: &mut Engine, vendor: u32, discount: i32) -> u32 {
    let interface = e.call(FOCUS_OWNER_004B7210, &args![]).u32();
    if vendor == 0 {
        e.call(PRINT_MESSAGE, &args![0x12u32, NULL_VENDOR_TEXT]);
        return 0;
    }
    let class = fn_0072d6c0(e);
    let old = e.call(TILE_GET_MENU_BY_CLASS, &args![class]).u32();
    if old != 0 {
        e.vcall(old, 0, &args![1u32]);
    }
    let reader = e.call(TILE_READER, &args![interface]).u32();
    let root = e
        .call(TILE_READ_FILE, &args![reader, BARTER_MENU_FILE])
        .u32();
    let found = if root != 0 {
        e.call(TILE_GET_MENU, &args![root]).u32()
    } else {
        0
    };
    e.set_global(BARTER_MENU, found);
    if barter_menu(e).addr() != 0 {
        let me = barter_menu(e).addr();
        let class = e.vcall(me, 0x34, &args![]).u32();
        if class == fn_0072d6c0(e) {
            return finish_barter_create(e, root, interface, vendor, discount);
        }
    }
    if barter_menu(e).addr() != 0 {
        let me = barter_menu(e).addr();
        if e.call(NODE_NEXT, &args![me]).u32() != 0 {
            let me = barter_menu(e).addr();
            if me != 0 {
                e.vcall(me, 0, &args![1u32]);
            }
        }
    }
    0
}

/// The part of [`barter_menu_create`] after a menu of the class was read.
fn finish_barter_create(
    e: &mut Engine,
    root: u32,
    interface: u32,
    vendor: u32,
    discount: i32,
) -> u32 {
    let me = barter_menu(e).addr();
    e.call(MENU_SET_MENU_TILE, &args![me, root, 0u32]);
    let me = barter_menu(e).addr();
    if !e.call(TILES_PRESENT_00796F90, &args![me]).bool() {
        e.call(PRINT_MESSAGE, &args![CREATION_FAILED_TEXT]);
        return 0;
    }
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
    e.call(DIALOG_SERVICE_START, &args![]);
    let class = fn_0072d6c0(e);
    e.call(
        SHOW_TUTORIAL,
        &args![interface.wrapping_add(0x4d4), 0x17u32, class, 0x200u32],
    );
    for (list, container) in [(PLAYER_ITEMS, 0x40u32), (VENDOR_ITEMS, 0x54)] {
        let me = barter_menu(e).addr();
        let tile = e.mem.u32(me + container);
        e.call(
            LIST_BOX_INITIALIZE,
            &args![me + list, tile, LIST_TEMPLATE_NAME],
        );
    }
    let text = e.call(SETTING_STRING, &args![0x011d_4378u32]).u32();
    let tile = barter_tile(e, barter_menu(e), 3);
    set_string(e, tile, 0xfc4, text);
    let text = e.call(SETTING_STRING, &args![0x011d_25b0u32]).u32();
    let label = e
        .call(TILE_GET_CHILD_BY_NAME, &args![root, CAPS_LABEL_NAME])
        .u32();
    set_string(e, label, 0xfc4, text);
    let text = e.call(SETTING_STRING, &args![0x011d_26f4u32]).u32();
    let tile = barter_tile(e, barter_menu(e), 0xd);
    set_string(e, tile, 0xfc4, text);
    let text = e.call(SETTING_STRING, &args![0x011d_33fcu32]).u32();
    let tile = barter_tile(e, barter_menu(e), 0x13);
    set_string(e, tile, 0xfc4, text);
    let title = e.call(TEXT_TO_TRAIT, &args![TITLE_TEXT]).u32();
    for (name, setting) in [
        (WEIGHT_INFO_NAME, 0x011d_2aa8u32),
        (VALUE_INFO_NAME, 0x011d_26a0),
    ] {
        let child = e.call(TILE_GET_CHILD_BY_NAME, &args![root, name]).u32();
        if child != 0 {
            let text = e.call(SETTING_STRING, &args![setting]).u32();
            set_string(e, child, title, text);
        }
    }
    for (offset, setting) in [
        (0x60u32, 0x011d_2c7cu32),
        (0x64, 0x011d_412c),
        (0x68, 0x011d_330c),
        (0x6c, 0x011d_20c4),
        (0x70, 0x011d_396c),
    ] {
        if e.mem.u32(barter_menu(e).addr() + offset) != 0 {
            let text = e.call(SETTING_STRING, &args![setting]).u32();
            let tile = e.mem.u32(barter_menu(e).addr() + offset);
            set_string(e, tile, title, text);
        }
    }
    let me = barter_menu(e);
    e.set(me, BarterMenu::pVendorRef, vendor);
    let low = e.call(BOUND_00647B70, &args![discount, -100i32]).u32();
    let bounded = e.call(BOUND_004A8F20, &args![low, 100u32]).i32();
    let me = barter_menu(e);
    e.set(me, BarterMenu::iDiscount, bounded);
    let me = barter_menu(e);
    e.mem.set_u32(me.addr() + 0x84, x87_store32(0.0));
    let me = barter_menu(e).addr();
    let player = e.global::<u32>(PLAYER);
    let value = e
        .call(
            GET_CLAMPED_ACTOR_VALUE,
            &args![player.wrapping_add(0xa4), 0x20u32],
        )
        .u32();
    e.call(GET_BARTER_MULTIPLIERS, &args![value, me + 0x94, me + 0x98]);
    e.call(MANAGER_PLAY_MENU_SOUND, &args![0x24u32]);
    let me = barter_menu(e).addr();
    e.call(MENU_PREP_FOR_VISIBILITY, &args![me, 0u32]);
    let me = barter_menu(e);
    fn_0072dc30(e, me, 0);
    root
}

// Translated from 0072d6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments): the barter menu's class number `0x41d`.
pub fn fn_0072d6c0(_e: &mut Engine) -> u32 {
    BARTER_MENU_CLASS
}

// Translated from 0072d6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments; the barter menu's close): with a menu open, updates the
/// player's 3D when its process asks for it (slot +0x474 with 1), ends the dialogue
/// service with topic `GetTopic(5, 4)` when `bItemsTraded` is set (else none), sets the
/// menu tile's trait `0x1772` to 1 and starts the fade out.
pub fn fn_0072d6d0(e: &mut Engine) {
    if barter_menu(e).addr() == 0 {
        return;
    }
    let player = e.global::<u32>(PLAYER);
    let process = e.call(ACTOR_PROCESS, &args![player]).u32();
    if e.vcall(process, 0x474, &args![1u32]).bool() {
        let player = e.global::<u32>(PLAYER);
        e.call(UPDATE_PLAYER_3D, &args![player]);
    }
    let mut topic = 0;
    if e.get(barter_menu(e), BarterMenu::bItemsTraded) != 0 {
        topic = e.call(GET_TOPIC, &args![5u32, 4u32]).u32();
    }
    e.call(DIALOG_SERVICE_END, &args![topic]);
    let menu_tile = e.mem.u32(barter_menu(e).addr() + 4);
    e.call(TILE_SET_INT, &args![menu_tile, 0x1772u32, 1u32]);
    let me = barter_menu(e).addr();
    e.call(MENU_START_FADE_OUT, &args![me]);
}

/// The filter-tab steps of `DoClick` for one side: plays the filter sound, then moves the
/// side's filter mode by `step` (wrapping 0..=5), retitles (`SetFilterModeTitle`) and
/// filters the list (`FilterModeFunc`) until it shows an item or the mode is 0.
fn step_filter_mode(
    e: &mut Engine,
    this: Ptr<BarterMenu>,
    mode: u32,
    title: u32,
    list: u32,
    step: i32,
) {
    let base = this.addr();
    let sound = e.get(this, BarterMenu::eFilterSound);
    e.call(PLAY_MENU_SOUND, &args![sound]);
    loop {
        let value = e.mem.i32(base + mode).wrapping_add(step);
        e.mem.set_i32(base + mode, value);
        if e.mem.i32(base + mode) < 0 {
            e.mem.set_i32(base + mode, 5);
        } else if e.mem.i32(base + mode) > 5 {
            e.mem.set_i32(base + mode, 0);
        }
        let tile = e.mem.u32(base + title);
        barter_menu_set_filter_mode_title(e, this, tile);
        list_box_item_change_filter(e, Ptr::new(base + list), 0x0073_04b0);
        let shown = e
            .call(ITEM_CHANGE_NUM_UNFILTERED, &args![base + list])
            .u32();
        if shown != 0 || e.mem.i32(base + mode) == 0 {
            break;
        }
    }
}

// Translated from 0072d770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BarterMenu::DoClick` (Xbox PDB): `tile` defaults to `pTiles[id]` for ids below 0x16.
/// Ids 2-4 step the player's filter mode (2 back, 3 and 4 forward), 7-9 the vendor's
/// (7 back); 0xd completes the transaction (`CompleteTransaction`) and refills the
/// lists; 0x13 leaves: with items to buy or sell it asks first (the message box with
/// the three settings' texts and callback `0072db40`), else closes (`0072d6d0`). 0x17
/// (an item, with a current item and a tile) clears the byte at `0119fdb0`; with the
/// console open it prints the current item's original container reference, otherwise,
/// when more of the item remain than the setting at `011d2418`, saves the list state,
/// un-highlights and opens the quantity menu (`ItemSelectCallback`), else selects one
/// (`ItemSelectCallback(1)`). The switch's jump tables (`0072db08`, `0072db20`) are the
/// `match` arms.
pub fn barter_menu_do_click(e: &mut Engine, this: Ptr<BarterMenu>, id: i32, tile: u32) {
    let base = this.addr();
    let mut tile = tile;
    if tile == 0 && (0..0x16).contains(&id) {
        tile = barter_tile(e, this, id as u32);
    }
    match id {
        2..=4 => step_filter_mode(
            e,
            this,
            0x9c,
            0x34,
            PLAYER_ITEMS,
            if id != 2 { 1 } else { -1 },
        ),
        7..=9 => step_filter_mode(
            e,
            this,
            0xa0,
            0x48,
            VENDOR_ITEMS,
            if id != 7 { 1 } else { -1 },
        ),
        0xd => {
            e.set_global(ITEM_ENTERED_0119FDB0, 0u8);
            barter_menu_complete_transaction(e, this);
            fn_0072dc30(e, this, 0);
        }
        0x13 => {
            let buying = e
                .call(LIST_COUNT_005AE380, &args![base + ITEMS_TO_BUY])
                .u32();
            let pending = buying != 0
                || e.call(LIST_COUNT_005AE380, &args![base + ITEMS_TO_SELL])
                    .u32()
                    != 0;
            if pending {
                let no = e.call(SETTING_STRING, &args![0x011d_3684u32]).u32();
                let yes = e.call(SETTING_STRING, &args![0x011d_34f8u32]).u32();
                let text = e.call(SETTING_STRING, &args![0x011d_1fa4u32]).u32();
                e.call(
                    SHOW_MESSAGE_BOX,
                    &args![
                        0u32,
                        0u32,
                        text,
                        0x0072_db40u32,
                        0u32,
                        0x17u32,
                        x87_store32(0.0),
                        x87_store32(0.0),
                        yes,
                        no,
                        0u32
                    ],
                );
            } else {
                fn_0072d6d0(e);
            }
        }
        0x17 => {
            if e.global::<u32>(BARTER_CURRENT_ITEM) == 0 || tile == 0 {
                return;
            }
            e.set_global(ITEM_ENTERED_0119FDB0, 0u8);
            let item = e.global::<u32>(BARTER_CURRENT_ITEM);
            let count = e.call(NODE_NEXT, &args![item]).i32();
            let numbered = e.call(TEXT_TO_TRAIT, &args![NUM_BARTERED_TEXT]).u32();
            let value = e.call(TILE_GET_FLOAT, &args![tile, numbered]).f64();
            let bartered = e
                .call(FLOAT_TO_INT_00406D90, &args![x87_store32(value)])
                .i32();
            let remaining = count.wrapping_sub(bartered);
            if e.call(IS_CONSOLE_VISIBLE, &args![]).bool() {
                let item = e.global::<u32>(BARTER_CURRENT_ITEM);
                let extra = first_extra(e, item);
                let reference = if extra != 0 {
                    let item = e.global::<u32>(BARTER_CURRENT_ITEM);
                    let list = e.call(FIRST_WORD, &args![item]).u32();
                    let first = node_item(e, list);
                    e.call(GET_ORIGINAL_REFERENCE, &args![first]).u32()
                } else {
                    0
                };
                if reference != 0 {
                    let id = e.call(FORM_ID_0084E3A0, &args![reference]).u32();
                    let name = e.vcall(reference, 0x130, &args![]).u32();
                    e.call(CONSOLE_PRINT, &args![ORIGINAL_REFERENCE_FORMAT, name, id]);
                } else {
                    e.call(CONSOLE_PRINT, &args![NO_ORIGINAL_REFERENCE_TEXT]);
                }
            } else {
                let limit = e.call(SETTING_VALUE_ADDRESS, &args![0x011d_2418u32]).u32();
                if remaining > e.mem.i32(limit) {
                    let list = e.get(this, BarterMenu::plbCurrListbox);
                    e.call(LIST_SAVE_STATE, &args![list]);
                    let list = e.get(this, BarterMenu::plbCurrListbox);
                    e.vcall(list, 0, &args![0u32]);
                    e.call(
                        QUANTITY_MENU_CREATE,
                        &args![remaining, 0x0072_f6f0u32, 0x7fff_ffffu32],
                    );
                } else {
                    barter_menu_item_select_callback(e, 1);
                }
            }
        }
        _ => {}
    }
}

// Translated from 0072db40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments; the leave question's callback): closes the menu (`0072d6d0`)
/// unless the message menu answered with a non-zero button
/// (`Interface::GetMessageMenuResult`).
pub fn fn_0072db40(e: &mut Engine) {
    if e.call(GET_MESSAGE_MENU_RESULT, &args![]).u8() == 0 {
        fn_0072d6d0(e);
    }
}

// Translated from 0072db60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0072db60 - the /GS cookie check (00ec408c) on the label buffer is not translated.
/// `BarterMenu::UpdateCapsLabels` (Xbox PDB): reads the player's caps
/// (`Actor::GetGoldAmount`) and the vendor container's (`00740960`,
/// `InventoryChanges::GetGoldAmount`) into `iPlayerCaps` and `iVendorCaps`, and writes
/// `"%s   %d"` of the caps label setting (`011d25b0`) and each count into the string
/// trait `0xfc4` of `pTiles[5]` and `pTiles[0xa]` (a 0x40-byte buffer on the stack).
pub fn barter_menu_update_caps_labels(e: &mut Engine, this: Ptr<BarterMenu>) {
    let base = this.addr();
    let player = e.global::<u32>(PLAYER);
    let caps = e.call(ACTOR_GET_GOLD_AMOUNT, &args![player]).i32();
    e.set(this, BarterMenu::iPlayerCaps, caps);
    let handler = e.global::<u32>(DATA_HANDLER);
    let container = e.call(BARTER_CONTAINER_00740960, &args![handler]).u32();
    let caps = e.call(INVENTORY_GET_GOLD_AMOUNT, &args![container]).i32();
    e.set(this, BarterMenu::iVendorCaps, caps);
    // The frame below `EBP`: the buffer is at `EBP - 0x48`.
    e.with_stack(0x48, |e, frame| {
        let buffer = frame.addr();
        for (count, tile) in [(0x8cu32, 0x3cu32), (0x90, 0x50)] {
            let caps = e.mem.u32(base + count);
            let label = e.call(SETTING_STRING, &args![0x011d_25b0u32]).u32();
            e.call(
                FORMAT_TEXT,
                &args![buffer, 0x40u32, CAPS_FORMAT, label, caps],
            );
            let tile = e.mem.u32(base + tile);
            set_string(e, tile, 0xfc4, buffer);
        }
    });
}

/// One side of `0072dc30`'s refresh for a form: the inventory's items of the group
/// (`GetItemGroup` into the list at `group`), the list's entries for the form removed,
/// then each item the side's first-pass filter keeps added (with its full name) and the
/// others deleted (`004459e0` with 1); the group list is cleared.
fn refresh_group(
    e: &mut Engine,
    this: Ptr<BarterMenu>,
    inventory: u32,
    form: u32,
    group: u32,
    list: u32,
    vendor_side: bool,
) {
    let base = this.addr();
    e.call(GET_ITEM_GROUP, &args![inventory, form, group]);
    e.call(LIST_REMOVE_ITEM_GROUP, &args![base + list, form]);
    let mut node = group;
    while walk_item(e, node) != 0 {
        let item = node_item(e, node);
        let filtered = if vendor_side {
            barter_menu_vendor_first_pass_filter_func(e, item)
        } else {
            barter_menu_player_first_pass_filter_func(e, item)
        };
        if !filtered {
            let item = node_item(e, node);
            let name = e.call(ITEM_FULL_NAME, &args![item]).u32();
            let item = node_item(e, node);
            e.call(LIST_ADD_ITEM, &args![base + list, item, name, 0u32, 0u32]);
        } else {
            let item = node_item(e, node);
            if item != 0 {
                e.call(ITEM_DATA_DELETE, &args![item, 1u32]);
            }
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.call(LIST_CLEAR, &args![group]);
}

/// One transaction list of `0072dc30` (`ItemsToBuy` or `ItemsToSell`): for every item
/// (of `form`, or all with a null form) a new `ItemChange` (`operator new(0xc)`,
/// `ItemChange::ItemChange(object, count)`), the item's first extra data added to it
/// (`005ae3d0` on its list, with the local holding it) when there is one, then added to
/// the list `list` with its name and its `"_IsBarterSelected"` trait set to 1.
fn readd_transaction_items(
    e: &mut Engine,
    this: Ptr<BarterMenu>,
    form: u32,
    items: u32,
    list: u32,
) {
    let base = this.addr();
    let mut node = base + items;
    while walk_item(e, node) != 0 {
        let matches = form == 0 || {
            let item = node_item(e, node);
            e.call(ITEM_OBJECT, &args![item]).u32() == form
        };
        if matches {
            let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
            let change = if block != 0 {
                let item = node_item(e, node);
                let count = e.call(NODE_NEXT, &args![item]).u32();
                let item = node_item(e, node);
                let object = e.call(ITEM_OBJECT, &args![item]).u32();
                e.call(ITEM_CHANGE_CONSTRUCT, &args![block, object, count])
                    .u32()
            } else {
                0
            };
            let extra = node_first_extra(e, node);
            e.with_stack(4, |e, local| {
                e.mem.set_u32(local.addr(), extra);
                if extra != 0 {
                    let list = e.call(FIRST_WORD, &args![change]).u32();
                    e.call(LIST_ADD_005AE3D0, &args![list, local]);
                }
            });
            let name = e.call(ITEM_FULL_NAME, &args![change]).u32();
            let tile = e
                .call(LIST_ADD_ITEM, &args![base + list, change, name, 0u32, 0u32])
                .u32();
            let selected = e.global::<u32>(IS_BARTER_SELECTED_TRAIT);
            e.call(TILE_SET_INT, &args![tile, selected, 1u32]);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 0072dc30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0072dc30 - the C++ exception-unwinding frame (FS:[0]: the local list, the new ItemChange) is not translated.
/// Unnamed (`thiscall`, `RET 4`: a form; the barter menu's list refresh). Saves the
/// current list's state when it has items (its slot +4). With a form, refreshes only the
/// form's items: the player's inventory (`GetInventoryChanges` of the player) and the
/// vendor container (`00740960`), each through a local list (`BSSimpleList`, built and
/// destroyed here) and the side's first-pass filter. Without one, gives the vendor to the
/// data handler (`0046f310`) and fills both lists from scratch (`00719ef0`: the player
/// with `PlayerFirstPassFilterFunc`, the vendor with `VendorFirstPassFilterFunc` and
/// last word 1). Then re-adds the items to buy to the player's list and the items to sell
/// to the vendor's, marks the stolen items (`RemoveStolenItems`, player's list) and
/// initialises both lists (`InitFunc`), filters them (`FilterModeFunc`) and sorts them
/// (slot +0x20 with `SortFunc`). Without a current list and with a gamepad, the vendor's
/// list becomes current. A current list with items gets its state restored; an empty one
/// switches tabs (slot +0xc with 4 or 9, the filter sound off then 3), then restores the
/// state or (when still empty) enters the other side (slot +0x10) and, with a gamepad,
/// focuses its highlighted tile. Ends with `UpdateCapsLabels`.
pub fn fn_0072dc30(e: &mut Engine, this: Ptr<BarterMenu>, form: u32) {
    let base = this.addr();
    let current = e.get(this, BarterMenu::plbCurrListbox);
    if current != 0 {
        let current = e.get(this, BarterMenu::plbCurrListbox);
        if e.vcall(current, 4, &args![]).u32() != 0 {
            let current = e.get(this, BarterMenu::plbCurrListbox);
            e.call(LIST_SAVE_STATE, &args![current]);
        }
    }
    if form != 0 {
        // The frame below `EBP`: the local list is at `EBP - 0x20`.
        e.with_stack(8, |e, group| {
            let group = group.addr();
            e.call(LIST_CONSTRUCT, &args![group]);
            let player = e.global::<u32>(PLAYER);
            let inventory = e.call(GET_INVENTORY_CHANGES, &args![player]).u32();
            let handler = e.global::<u32>(DATA_HANDLER);
            let container = e.call(BARTER_CONTAINER_00740960, &args![handler]).u32();
            if inventory != 0 {
                refresh_group(e, this, inventory, form, group, PLAYER_ITEMS, false);
            }
            if container != 0 {
                refresh_group(e, this, container, form, group, VENDOR_ITEMS, true);
            }
            e.call(LIST_DESTRUCT, &args![group]);
        });
    } else {
        let handler = e.global::<u32>(DATA_HANDLER);
        let vendor = e.get(this, BarterMenu::pVendorRef);
        e.call(SET_BARTER_CONTAINER_0046F310, &args![handler, vendor]);
        let player = e.global::<u32>(PLAYER);
        e.call(
            LIST_FILL,
            &args![
                base + PLAYER_ITEMS,
                player,
                0x0072_e570u32,
                0u32,
                0u32,
                0u32,
                0u32
            ],
        );
        let vendor = e.get(this, BarterMenu::pVendorRef);
        e.call(
            LIST_FILL,
            &args![
                base + VENDOR_ITEMS,
                vendor,
                0x0072_e6c0u32,
                0u32,
                0u32,
                0u32,
                1u32
            ],
        );
    }
    readd_transaction_items(e, this, form, ITEMS_TO_BUY, PLAYER_ITEMS);
    readd_transaction_items(e, this, form, ITEMS_TO_SELL, VENDOR_ITEMS);
    let player_list = Ptr::new(base + PLAYER_ITEMS);
    let vendor_list = Ptr::new(base + VENDOR_ITEMS);
    fn_007314c0(e, player_list, 0x0072_eac0, 0, 0x7fff_ffff);
    fn_007314c0(e, player_list, 0x0072_f210, 0, 0x7fff_ffff);
    list_box_item_change_filter(e, player_list, 0x0073_04b0);
    e.vcall(base + PLAYER_ITEMS, 0x20, &args![0x0072_f070u32]);
    fn_007314c0(e, vendor_list, 0x0072_f210, 0, 0x7fff_ffff);
    list_box_item_change_filter(e, vendor_list, 0x0073_04b0);
    e.vcall(base + VENDOR_ITEMS, 0x20, &args![0x0072_f070u32]);
    if e.get(this, BarterMenu::plbCurrListbox) == 0 && e.call(FLAG_004B71D0, &args![]).bool() {
        e.set(this, BarterMenu::plbCurrListbox, base + VENDOR_ITEMS);
    }
    if e.get(this, BarterMenu::plbCurrListbox) == 0 {
        barter_menu_update_caps_labels(e, this);
        return;
    }
    let current = e.get(this, BarterMenu::plbCurrListbox);
    if e.call(ITEM_CHANGE_NUM_UNFILTERED, &args![current]).u32() != 0 {
        let current = e.get(this, BarterMenu::plbCurrListbox);
        list_box_item_change_restore_list_state(e, Ptr::new(current), 1);
    } else {
        e.set(this, BarterMenu::eFilterSound, -1);
        let tab = if e.get(this, BarterMenu::plbCurrListbox) != base + PLAYER_ITEMS {
            9u32
        } else {
            4
        };
        e.vcall(base, 0xc, &args![tab, 0u32]);
        e.set(this, BarterMenu::eFilterSound, 3);
        let current = e.get(this, BarterMenu::plbCurrListbox);
        if e.call(ITEM_CHANGE_NUM_UNFILTERED, &args![current]).u32() != 0 {
            let current = e.get(this, BarterMenu::plbCurrListbox);
            list_box_item_change_restore_list_state(e, Ptr::new(current), 1);
        } else {
            let player_side =
                u32::from(e.get(this, BarterMenu::plbCurrListbox) == base + PLAYER_ITEMS);
            e.vcall(base, 0x10, &args![player_side, 0u32]);
            if e.call(FLAG_004B71D0, &args![]).bool() {
                let current = e.get(this, BarterMenu::plbCurrListbox);
                let tile = e.call(LIST_HIGHLIGHTED_007D6E90, &args![current]).u32();
                let owner = e.call(FOCUS_OWNER_004B7210, &args![]).u32();
                e.call(
                    SET_CURRENT_FOCUS_TARGET,
                    &args![owner, tile, 0xfc3u32, 1u32],
                );
            }
        }
    }
    barter_menu_update_caps_labels(e, this);
}

// Translated from 0072e370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BarterMenu::DoEnter` (Xbox PDB): `tile` defaults to `pTiles[id]` for ids below
/// 0x16. Id 0 (the player's side): leaves tile 0x17 (slot +0x14), un-highlights the
/// vendor's list, clears the menu tile's `"_IsContainerListSelected"` and makes the
/// player's list current; id 1 the mirror (the trait set to 1, the vendor's list
/// current). Id 0x17 (an item) sets the byte at `0119fdb0`; without a current list
/// enters the side the tile belongs to (slot +0x10: 1 for the vendor's list); the current
/// list's selected item becomes `pCurrItem` (`007a1910`) and, when there is one, shows
/// its card (`00707e30` with `pTiles[0x15]`) and description (`004be200(0, 1)` into
/// `pTiles[0x14]`'s trait `0xfcc`), with both tiles' trait `0xfa3` set to 1.
pub fn barter_menu_do_enter(e: &mut Engine, this: Ptr<BarterMenu>, id: i32, tile: u32) {
    let base = this.addr();
    let mut tile = tile;
    if tile == 0 && (0..0x16).contains(&id) {
        tile = barter_tile(e, this, id as u32);
    }
    match id {
        0 | 1 => {
            let (other, selected, current) = if id == 0 {
                (VENDOR_ITEMS, 0u32, PLAYER_ITEMS)
            } else {
                (PLAYER_ITEMS, 1, VENDOR_ITEMS)
            };
            e.vcall(base, 0x14, &args![0x17u32, 0u32]);
            e.vcall(base + other, 0, &args![0u32]);
            let which = e.global::<u32>(IS_CONTAINER_LIST_SELECTED_TRAIT);
            let menu_tile = e.call(NODE_NEXT, &args![base]).u32();
            e.call(TILE_SET_INT, &args![menu_tile, which, selected]);
            e.set(this, BarterMenu::plbCurrListbox, base + current);
        }
        0x17 => {
            e.set_global(ITEM_ENTERED_0119FDB0, 1u8);
            if e.get(this, BarterMenu::plbCurrListbox) == 0 {
                let vendor_side = e
                    .call(LIST_BOX_BELONGS_TO, &args![base + VENDOR_ITEMS, tile])
                    .u8();
                e.vcall(base, 0x10, &args![u32::from(vendor_side != 0), 0u32]);
            }
            let current = e.get(this, BarterMenu::plbCurrListbox);
            let item = e.call(LIST_SELECTED_007A1910, &args![current]).u32();
            e.set_global(BARTER_CURRENT_ITEM, item);
            if e.global::<u32>(BARTER_CURRENT_ITEM) == 0 {
                return;
            }
            let card = barter_tile(e, this, 0x15);
            let item = e.global::<u32>(BARTER_CURRENT_ITEM);
            e.call(ITEM_CARD_00707E30, &args![item, card]);
            let card = barter_tile(e, this, 0x15);
            e.call(TILE_SET_INT, &args![card, 0xfa3u32, 1u32]);
            let item = e.global::<u32>(BARTER_CURRENT_ITEM);
            let text = e
                .call(ITEM_DESCRIPTION_004BE200, &args![item, 0u32, 1u32])
                .u32();
            let description = barter_tile(e, this, 0x14);
            set_string(e, description, 0xfcc, text);
            let description = barter_tile(e, this, 0x14);
            e.call(TILE_SET_INT, &args![description, 0xfa3u32, 1u32]);
        }
        _ => {}
    }
}

// Translated from 0072e530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BarterMenu::DoLeave` (Xbox PDB, `RET 8`; neither word is read): hides the item card
/// and description (trait `0xfa3` of `pTiles[0x15]` and `pTiles[0x14]` set to 0).
pub fn barter_menu_do_leave(e: &mut Engine, this: Ptr<BarterMenu>, _unused_1: u32, _unused_2: u32) {
    let card = barter_tile(e, this, 0x15);
    e.call(TILE_SET_INT, &args![card, 0xfa3u32, 0u32]);
    let description = barter_tile(e, this, 0x14);
    e.call(TILE_SET_INT, &args![description, 0xfa3u32, 0u32]);
}

/// The tests both first-pass filters end with, on the item's object: a playable biped
/// form (when it is one, `00480d90`), not gold, a cast form (`__RTDynamicCast` from
/// `01183108` to `01183158`) for which `0048cee0` holds, and for objects of type 0x28 and
/// 0x29 their tests (`0047bcf0`, `004c94d0`). Returns the object, the biped form, the
/// cast form and the two typed objects in the exe's order of evaluation.
fn first_pass_inputs(e: &mut Engine, item: u32, object: u32) -> (u32, u32, u32, u32) {
    let biped = e.call(AS_BIPED_MODEL, &args![object]).u32();
    let cast = e
        .call(
            RT_DYNAMIC_CAST,
            &args![object, 0u32, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0u32],
        )
        .u32();
    let typed = |e: &mut Engine, kind: u32| {
        let found = e.call(ITEM_OBJECT, &args![item]).u32();
        if e.call(FORM_TYPE, &args![found]).u32() == kind {
            e.call(ITEM_OBJECT, &args![item]).u32()
        } else {
            0
        }
    };
    let type_28 = typed(e, 0x28);
    let type_29 = typed(e, 0x29);
    (biped, cast, type_28, type_29)
}

// Translated from 0072e570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BarterMenu::PlayerFirstPassFilterFunc` (Xbox PDB, cdecl): true (filtered out) for a
/// null object, an unplayable biped form, an object whose slot +0x94 answers true, gold,
/// a form that does not cast (or whose `0048cee0` is 0), a type-0x28 or type-0x29 object
/// that fails its test, or an object of type 0x2e. The item's object is read again
/// without a null check, as the code does.
pub fn barter_menu_player_first_pass_filter_func(e: &mut Engine, item: u32) -> bool {
    let object = if item != 0 {
        e.call(ITEM_OBJECT, &args![item]).u32()
    } else {
        0
    };
    let (biped, cast, type_28, type_29) = first_pass_inputs(e, item, object);
    if object == 0 {
        return true;
    }
    if biped != 0 && !e.call(BIPED_IS_PLAYABLE, &args![biped]).bool() {
        return true;
    }
    if e.vcall(object, 0x94, &args![]).bool() {
        return true;
    }
    if e.call(IS_GOLD, &args![object]).bool() {
        return true;
    }
    if cast == 0 || e.call(CAST_TEST_0048CEE0, &args![cast]).u32() == 0 {
        return true;
    }
    if type_28 != 0 && !e.call(TYPE_28_TEST_0047BCF0, &args![type_28]).bool() {
        return true;
    }
    if type_29 != 0 && !e.call(TYPE_29_TEST_004C94D0, &args![type_29]).bool() {
        return true;
    }
    e.call(FORM_TYPE, &args![object]).u32() == 0x2e
}

/// Whether the vendor does not take items of a service flag: `!0047efe0(004181e0(vendor)
/// + 0x90, flag)`.
fn vendor_refuses(e: &mut Engine, vendor: u32, flag: u32) -> bool {
    // 0072e783 and the other cases: `004181e0` takes no stack word (plain RET); the
    // pushed flag is the word of `0047efe0` (RET 4).
    let services = e.call(VENDOR_SERVICES_004181E0, &args![vendor]).u32();
    !e.call(
        SERVICE_FLAG_0047EFE0,
        &args![services.wrapping_add(0x90), flag],
    )
    .bool()
}

// Translated from 0072e6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BarterMenu::VendorFirstPassFilterFunc` (Xbox PDB, cdecl): with a vendor that is an
/// actor (slot +0x100), first filters by the item's form type (the jump tables `0072ea3c`
/// and `0072ea5c` as `match` arms): 0x18 needs service flag 2, 0x19 flag 8, 0x1f, 0x32 and
/// 0x67 flag 0x400, 0x28 and 0x29 flag 1, 0x2e is always filtered, 0x31 with a value
/// (`GetFormValue` > 0) flag 0x400, and 0x2f by its equip type (10: 0x20, 11: 0x40, 12:
/// 0x10, 13: 4; jump table `0072eaac`). Then filters (whatever the result so far) a null
/// object, an unplayable biped form, gold, a form that does not cast, and a type-0x28 or
/// type-0x29 object that fails its test.
pub fn barter_menu_vendor_first_pass_filter_func(e: &mut Engine, item: u32) -> bool {
    let mut filtered = false;
    let me = barter_menu(e);
    let mut vendor = 0;
    if e.get(me, BarterMenu::pVendorRef) != 0 {
        let reference = e.get(barter_menu(e), BarterMenu::pVendorRef);
        if e.vcall(reference, 0x100, &args![]).bool() {
            vendor = e.get(barter_menu(e), BarterMenu::pVendorRef);
        }
    }
    let object = if item != 0 {
        e.call(ITEM_OBJECT, &args![item]).u32()
    } else {
        0
    };
    if vendor != 0 {
        let found = e.call(ITEM_OBJECT, &args![item]).u32();
        let kind = e.call(FORM_TYPE, &args![found]).u32();
        match kind {
            0x18 => filtered = vendor_refuses(e, vendor, 2),
            0x19 => filtered = vendor_refuses(e, vendor, 8),
            0x1f | 0x32 | 0x67 => filtered = vendor_refuses(e, vendor, 0x400),
            0x28 | 0x29 => filtered = vendor_refuses(e, vendor, 1),
            0x2e => filtered = true,
            0x31 => {
                filtered = if object != 0 && e.call(GET_FORM_VALUE, &args![object]).i32() > 0 {
                    vendor_refuses(e, vendor, 0x400)
                } else {
                    true
                };
            }
            0x2f => {
                let found = e.call(ITEM_OBJECT, &args![item]).u32();
                match e.call(GET_EQUIP_TYPE, &args![found]).u32() {
                    10 => filtered = vendor_refuses(e, vendor, 0x20),
                    11 => filtered = vendor_refuses(e, vendor, 0x40),
                    12 => filtered = vendor_refuses(e, vendor, 0x10),
                    13 => filtered = vendor_refuses(e, vendor, 4),
                    _ => {}
                }
            }
            _ => {}
        }
    }
    let (biped, cast, type_28, type_29) = first_pass_inputs(e, item, object);
    if object == 0 {
        return true;
    }
    if biped != 0 && !e.call(BIPED_IS_PLAYABLE, &args![biped]).bool() {
        return true;
    }
    if e.call(IS_GOLD, &args![object]).bool() {
        return true;
    }
    if cast == 0 || e.call(CAST_TEST_0048CEE0, &args![cast]).u32() == 0 {
        return true;
    }
    if type_28 != 0 && !e.call(TYPE_28_TEST_0047BCF0, &args![type_28]).bool() {
        return true;
    }
    if type_29 != 0 && !e.call(TYPE_29_TEST_004C94D0, &args![type_29]).bool() {
        return true;
    }
    filtered
}

// Translated from 0072eac0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0072eac0 - the C++ exception-unwinding frame (FS:[0]: the local list and the local reference) and the /GS cookie check (00ec408c) are not translated.
/// `BarterMenu::RemoveStolenItems` (Xbox PDB, cdecl: the tile, which is not read, and
/// the item; a callback of `007314c0`). With a vendor that is an actor (slot +0x100),
/// every extra data entry of the item that has an owner (`00418660`) is tried on a local
/// `TESObjectREFR` (constructed and destroyed here) given that owner, its global, its rank
/// and the item's object: when the vendor is an owner (`IsAnOwner(vendor, 1)`) the entry
/// goes into a local list (`005ae3d0`). Then for each listed entry the item's count is
/// lowered by the entry's count (`ExtraDataList::GetCount`, 16 bits, sign-extended) and
/// the entry removed from the item's extra list (`00905330` with the node's item slot);
/// the local list is cleared and destroyed.
pub fn barter_menu_remove_stolen_items(e: &mut Engine, _unused_1: u32, item: u32) {
    let mut vendor = 0;
    if e.get(barter_menu(e), BarterMenu::pVendorRef) != 0 {
        let reference = e.get(barter_menu(e), BarterMenu::pVendorRef);
        if e.vcall(reference, 0x100, &args![]).bool() {
            vendor = e.get(barter_menu(e), BarterMenu::pVendorRef);
        }
    }
    // The frame below `EBP`: the list at `EBP - 0x14`, the entry at `EBP - 0x20`, the
    // reference at `EBP - 0x8c` (to `EBP - 0x24`, 0x68 bytes: 0055a2f0 writes +0x64).
    e.with_stack(0x74, |e, frame| {
        let list = frame.addr();
        let entry_local = frame.addr() + 8;
        let reference = frame.addr() + 0xc;
        e.call(LIST_CONSTRUCT, &args![list]);
        let mut node = e.call(FIRST_WORD, &args![item]).u32();
        while walk_item(e, node) != 0 {
            let entry = node_item(e, node);
            e.mem.set_u32(entry_local, entry);
            if vendor != 0 && e.call(EXTRA_GET_OWNER_00418660, &args![entry]).u32() != 0 {
                e.call(REFERENCE_CONSTRUCT, &args![reference]);
                let owner = e.call(EXTRA_GET_OWNER_00418660, &args![entry]).u32();
                e.call(REFERENCE_SET_OWNER_00567AD0, &args![reference, owner]);
                let global = e.call(EXTRA_GET_GLOBAL, &args![entry]).u32();
                e.call(REFERENCE_SET_GLOBAL, &args![reference, global]);
                let rank = e.call(EXTRA_GET_RANK, &args![entry]).u32();
                e.call(REFERENCE_SET_RANK_00567BD0, &args![reference, rank]);
                let object = e.call(ITEM_OBJECT, &args![item]).u32();
                e.call(REFERENCE_SET_OBJECT, &args![reference, object]);
                if e.call(REFERENCE_IS_AN_OWNER, &args![reference, vendor, 1u32])
                    .bool()
                {
                    e.call(LIST_ADD_005AE3D0, &args![list, entry_local]);
                }
                e.call(REFERENCE_DESTRUCT, &args![reference]);
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
        let mut node = list;
        while walk_item(e, node) != 0 {
            let count = e.call(NODE_NEXT, &args![item]).i32();
            let entry = node_item(e, node);
            let removed = i32::from(e.call(EXTRA_GET_COUNT, &args![entry]).u16() as i16);
            e.call(NODE_SET_NEXT, &args![item, count.wrapping_sub(removed)]);
            let slot = e.call(NODE_SELF, &args![node]).u32();
            let extras = e.call(FIRST_WORD, &args![item]).u32();
            e.call(EXTRA_REMOVE_00905330, &args![extras, slot]);
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
        e.call(LIST_CLEAR, &args![list]);
        e.call(LIST_DESTRUCT, &args![list]);
    });
}

/// One side of `GetSortValue`: the item's value times the multiplier (1.0 for objects of
/// type 0x6c and 0x74), changed by the discount (`buying`: a positive discount lowers
/// it, else raises it), passed through the barter entry point (0x11, the player, the
/// object, its address), then `004bd510` with 1.0 when `00408860` of it is above 1.0, else
/// with 0.1. Returns the value as stored.
fn sort_side_value(e: &mut Engine, item: u32, value: u32, multiplier: u32, buying: bool) -> u32 {
    let me = barter_menu(e).addr();
    let found = e.call(ITEM_OBJECT, &args![item]).u32();
    let special = e.call(FORM_TYPE, &args![found]).u32() == 0x6c || {
        let found = e.call(ITEM_OBJECT, &args![item]).u32();
        e.call(FORM_TYPE, &args![found]).u32() == 0x74
    };
    let scale = if special {
        x87_store32(1.0)
    } else {
        x87_quiet_bits(e.mem.u32(me + multiplier))
    };
    let mut value = x87_store32(x87_mul(x87_load32(value), x87_m32(scale)));
    let hundred: f64 = e.global(HUNDRED_DOUBLE);
    let one: f64 = e.global(ONE_DOUBLE);
    if e.get(barter_menu(e), BarterMenu::iDiscount) > 0 {
        let discount = e.get(barter_menu(e), BarterMenu::iDiscount);
        let amount = e.call(ABS_00EC7D40, &args![discount]).i32();
        let part = x87_div(f64::from(amount), hundred);
        let factor = if buying {
            x87_sub(1.0, part)
        } else {
            x87_add(part, one)
        };
        value = x87_store32(x87_mul(factor, x87_m32(value)));
    } else if e.get(barter_menu(e), BarterMenu::iDiscount) < 0 {
        let discount = e.get(barter_menu(e), BarterMenu::iDiscount);
        let amount = e.call(ABS_00EC7D40, &args![discount]).i32();
        let part = x87_div(f64::from(amount), hundred);
        let factor = if buying {
            x87_add(part, one)
        } else {
            x87_sub(1.0, part)
        };
        value = x87_store32(x87_mul(factor, x87_m32(value)));
    }
    if buying {
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), value);
            let found = e.call(ITEM_OBJECT, &args![item]).u32();
            let player = e.global::<u32>(PLAYER);
            e.call(
                HANDLE_ENTRY_POINT,
                &args![0x11u32, player, found, local.addr()],
            );
            value = e.mem.u32(local.addr());
        });
    }
    let checked = e.call(VALUE_00408860, &args![x87_quiet_bits(value)]).f64();
    // FCOMP against 1.0; TEST AH,0x41; JNZ: the 1.0 pair only when strictly above.
    let bound = if checked > one {
        x87_store32(1.0)
    } else {
        x87_quiet_bits(e.global::<u32>(TENTH_FLOAT))
    };
    let result = e
        .call(FLOAT_PAIR_004BD510, &args![x87_quiet_bits(value), bound])
        .f64();
    x87_store32(result)
}

// Translated from 0072ed00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BarterMenu::GetSortValue` (Xbox PDB, cdecl: the tile, the item; a `float` in `ST0`):
/// the item's value (`ItemChange::GetItemValue`); -1.0 when it is below 0.0. An item that
/// is on the vendor's side and not selected, or on the player's side and selected (the
/// vendor's list test `007242f0`, `"_IsBarterSelected"`), is valued at the buy price
/// (`fBuyMult`, the barter entry point, then `00404010` with the original value); the
/// others at the sell price (`fSellMult`, no entry point, then `0040ebd0`). The two
/// branches that answer 0.0 cannot be reached (the conditions cover every case) but are
/// kept as the code has them.
pub fn barter_menu_get_sort_value(e: &mut Engine, tile: u32, item: u32) -> f32 {
    let value = e.call(ITEM_GET_VALUE, &args![item]).f64();
    let value = x87_store32(value);
    let original = x87_quiet_bits(value);
    let me = barter_menu(e).addr();
    let vendor_side = e
        .call(LIST_BOX_BELONGS_TO, &args![me + VENDOR_ITEMS, tile])
        .u8()
        != 0;
    let selected_trait = e.global::<u32>(IS_BARTER_SELECTED_TRAIT);
    let selected = e.call(TILE_IS_TRUE, &args![tile, selected_trait]).u8() != 0;
    let mut result = x87_store32(0.0);
    // FCOMP against 0.0; TEST AH,5; JP: -1.0 only when the value is below 0.0.
    if x87_load32(value) < e.global::<f64>(ZERO_DOUBLE) {
        result = x87_quiet_bits(e.global::<u32>(MINUS_ONE_FLOAT));
    } else if (vendor_side && !selected) || (!vendor_side && selected) {
        let value = sort_side_value(e, item, value, 0x94, true);
        result = x87_store32(e.call(FLOAT_PAIR_00404010, &args![value, original]).f64());
    } else if (vendor_side && selected) || (!vendor_side && !selected) {
        let value = sort_side_value(e, item, value, 0x98, false);
        result = x87_store32(e.call(FLOAT_PAIR_0040EBD0, &args![value, original]).f64());
    }
    f32::from_bits(x87_quiet_bits(result))
}

// Translated from 0072f070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BarterMenu::SortFunc` (Xbox PDB, cdecl: two list items, each a tile then its data):
/// selected items (`"_IsBarterSelected"`) first (-1 when only the first is, 1 when only
/// the second is), otherwise `00469880` of the two tiles' strings (trait `0xfc4`; the
/// second's read first). The items' data words are read and not used.
pub fn barter_menu_sort_func(e: &mut Engine, first: u32, second: u32) -> i32 {
    let _first_data = e.mem.u32(first.wrapping_add(4));
    let _second_data = e.mem.u32(second.wrapping_add(4));
    let selected = e.global::<u32>(IS_BARTER_SELECTED_TRAIT);
    let first_tile = e.mem.u32(first);
    let first_selected = e.call(TILE_IS_TRUE, &args![first_tile, selected]).u8() != 0;
    let selected = e.global::<u32>(IS_BARTER_SELECTED_TRAIT);
    let second_tile = e.mem.u32(second);
    let second_selected = e.call(TILE_IS_TRUE, &args![second_tile, selected]).u8() != 0;
    if first_selected && !second_selected {
        return -1;
    }
    if !first_selected && second_selected {
        return 1;
    }
    let second_tile = e.mem.u32(second);
    let second_text = e.call(TILE_GET_STRING, &args![second_tile, 0xfc4u32]).u32();
    let first_tile = e.mem.u32(first);
    let first_text = e.call(TILE_GET_STRING, &args![first_tile, 0xfc4u32]).u32();
    e.call(STRING_COMPARE_00469880, &args![first_text, second_text])
        .i32()
}

// Translated from 0072f120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BarterMenu::GetTransactItem` (Xbox PDB, `thiscall`, `RET 8`: a list node, an item;
/// `this` is not read): the first item of the list with the same object (`0044ddc0`) and
/// the same first extra data as `item`, or null.
pub fn barter_menu_get_transact_item(
    e: &mut Engine,
    _this: Ptr<BarterMenu>,
    list: u32,
    item: u32,
) -> u32 {
    let mut found = 0;
    let mut node = list;
    while found == 0 && walk_item(e, node) != 0 {
        let candidate = node_item(e, node);
        let object = e.call(ITEM_OBJECT, &args![item]).u32();
        if object == e.call(ITEM_OBJECT, &args![candidate]).u32() {
            let wanted = first_extra(e, item);
            let extra = first_extra(e, candidate);
            if wanted == extra {
                found = candidate;
            }
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    found
}

// Translated from 0072f210 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0072f210 - the /GS cookie check (00ec408c) on the text buffer is not translated.
/// `BarterMenu::InitFunc` (Xbox PDB, cdecl: the tile, the item): the count shown is the
/// item's count, less the count of the same item in the buy or sell list (`GetTransactItem`)
/// for an unselected tile, whose `"_NumBartered"` trait gets that transaction count. With
/// more than one, the name trait `0xfc4` reads `"%s (%d)"`. An equipped item
/// (`ItemChange::GetWorn(0)`) on an unselected tile gets `"_IsEquipped"` 1. The value
/// trait (`"_Value"`) reads `"--"` for a value at most 0, `"%.1f"` below 1, else `"%.0f"`
/// (of `GetSortValue`). Then `00707e10` with the item's name and the trait `0xfb0` set to
/// 0.0. A 0x80-byte buffer on the stack.
pub fn barter_menu_init_func(e: &mut Engine, tile: u32, item: u32) {
    let value = barter_menu_get_sort_value(e, tile, item).to_bits();
    let value = x87_quiet_bits(value);
    let selected = e.global::<u32>(IS_BARTER_SELECTED_TRAIT);
    let count = if !e.call(TILE_IS_TRUE, &args![tile, selected]).bool() {
        let me = barter_menu(e);
        let mut transact = barter_menu_get_transact_item(e, me, me.addr() + ITEMS_TO_BUY, item);
        if transact == 0 {
            let me = barter_menu(e);
            transact = barter_menu_get_transact_item(e, me, me.addr() + ITEMS_TO_SELL, item);
        }
        let traded = if transact != 0 {
            e.call(NODE_NEXT, &args![transact]).i32()
        } else {
            0
        };
        let count = e.call(NODE_NEXT, &args![item]).i32().wrapping_sub(traded);
        let traded = if transact != 0 {
            e.call(NODE_NEXT, &args![transact]).u32()
        } else {
            0
        };
        let numbered = e.call(TEXT_TO_TRAIT, &args![NUM_BARTERED_TEXT]).u32();
        e.call(TILE_SET_INT, &args![tile, numbered, traded]);
        count
    } else {
        e.call(NODE_NEXT, &args![item]).i32()
    };
    // The frame below `EBP`: the buffer is at `EBP - 0x88`.
    e.with_stack(0x80, |e, buffer| {
        let buffer = buffer.addr();
        if count > 1 {
            let name = e.call(ITEM_FULL_NAME, &args![item]).u32();
            e.call(
                FORMAT_TEXT,
                &args![buffer, 0x80u32, NAME_COUNT_FORMAT, name, count],
            );
            set_string(e, tile, 0xfc4, buffer);
        }
        if e.call(ITEM_IS_EQUIPPED_004BDDD0, &args![item, 0u32]).bool() {
            let selected = e.global::<u32>(IS_BARTER_SELECTED_TRAIT);
            if !e.call(TILE_IS_TRUE, &args![tile, selected]).bool() {
                let equipped = e
                    .call(ADD_USER_TRAIT, &args![IS_EQUIPPED_TEXT, 0xffff_ffffu32])
                    .u32();
                e.call(TILE_SET_INT, &args![tile, equipped, 1u32]);
            }
        }
        let zero: f64 = e.global(ZERO_DOUBLE);
        let one: f64 = e.global(ONE_DOUBLE);
        let shown = x87_load32(value);
        // FCOMP against 0.0; TEST AH,0x41; JP: "--" for a value at most 0.0 (not NaN).
        if shown <= zero {
            e.call(COPY_TEXT, &args![buffer, 0x80u32, NO_VALUE_TEXT]);
        } else {
            let bits = shown.to_bits();
            // TEST AH,0x41 then TEST AH,5: one decimal only above 0.0 and below 1.0.
            let format = if shown > zero && shown < one {
                ONE_DECIMAL_FORMAT
            } else {
                NO_DECIMAL_FORMAT
            };
            e.call(
                FORMAT_TEXT,
                &args![buffer, 0x80u32, format, bits as u32, (bits >> 32) as u32],
            );
        }
        let value_trait = e.global::<u32>(VALUE_TRAIT);
        set_string(e, tile, value_trait, buffer);
    });
    let name = e.call(ITEM_FULL_NAME, &args![item]).u32();
    e.call(TILE_SET_ITEM_00707E10, &args![tile, name]);
    e.call(
        TILE_SET_FLOAT,
        &args![tile, 0xfb0u32, x87_store32(0.0), 1u32],
    );
}

// Translated from 0072f4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BarterMenu::DoGamepad` (Xbox PDB, `RET 8`; the second word is not read): only while
/// the barter menu is on top (`Interface::IsTopMenuID(0x41d)`). Buttons 3 and 4 switch
/// the current side's filter tab (slot +0xc with 2 or 4 on the player's side, 7 or 9 on
/// the vendor's, after saving that list's state) and restore the list state without
/// focus; 0xd and 0xe enter the player's or the vendor's side when that list is not yet
/// enabled and has items (saving the other list's state, slot +0x10 with 0 or 1, then
/// restoring with focus). Returns whether it handled the button (jump tables
/// `0072f658`, `0072f668`).
pub fn barter_menu_do_gamepad(
    e: &mut Engine,
    this: Ptr<BarterMenu>,
    button: i32,
    _unused_2: u32,
) -> bool {
    let base = this.addr();
    let mut handled = false;
    if !e
        .call(MENU_IS_TOP_00702450, &args![BARTER_MENU_CLASS])
        .bool()
    {
        return handled;
    }
    match button {
        3 | 4 => {
            let forward = u32::from(button != 4);
            if e.get(this, BarterMenu::plbCurrListbox) == base + PLAYER_ITEMS {
                e.call(LIST_SAVE_STATE, &args![base + PLAYER_ITEMS]);
                e.vcall(base, 0xc, &args![forward * 2 + 2, 0u32]);
                list_box_item_change_restore_list_state(e, Ptr::new(base + PLAYER_ITEMS), 0);
            } else {
                e.call(LIST_SAVE_STATE, &args![base + VENDOR_ITEMS]);
                e.vcall(base, 0xc, &args![forward * 2 + 7, 0u32]);
                list_box_item_change_restore_list_state(e, Ptr::new(base + VENDOR_ITEMS), 0);
            }
            handled = true;
        }
        0xd | 0xe => {
            let (list, other, side) = if button == 0xd {
                (PLAYER_ITEMS, VENDOR_ITEMS, 0u32)
            } else {
                (VENDOR_ITEMS, PLAYER_ITEMS, 1)
            };
            if !list_box_item_change_is_enabled(e, Ptr::new(base + list))
                && e.call(LIST_ITEM_COUNT, &args![base + list]).u32() != 0
            {
                e.call(LIST_SAVE_STATE, &args![base + other]);
                e.vcall(base, 0x10, &args![side, 0u32]);
                list_box_item_change_restore_list_state(e, Ptr::new(base + list), 1);
                handled = true;
            }
        }
        _ => {}
    }
    handled
}

// Translated from 0072f680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BarterMenu::DoWheelMove` (Xbox PDB, `RET 8`; the second word is not read): on tile 3
/// or 8 (a side's filter tab) clicks the side's back tab (2 or 7) when the interface
/// manager's wheel direction (`0070ec70`) is negative, else its forward tab (4 or 9).
pub fn barter_menu_do_wheel_move(e: &mut Engine, this: Ptr<BarterMenu>, id: i32, _unused_2: u32) {
    if id != 3 && id != 8 {
        return;
    }
    let manager = e.call(FOCUS_OWNER_004B7210, &args![]).u32();
    let back = if id != 3 { 5u32 } else { 0 };
    if e.call(WHEEL_DIRECTION_0070EC70, &args![manager]).i32() < 0 {
        e.vcall(this.addr(), 0xc, &args![back + 2, 0u32]);
    } else {
        e.vcall(this.addr(), 0xc, &args![back + 4, 0u32]);
    }
}

// Translated from 0072f6f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0072f6f0 - the /GS cookie check (00ec408c) on the amount buffer is not translated.
/// `BarterMenu::ItemSelectCallback` (Xbox PDB, cdecl: the count; also the quantity
/// menu's callback). With a count, a current item and its tile in the current list
/// (`GetTileFor`), adds `GetSortValue * count` (negated on the vendor's side) to
/// `fTransactionAmount`, tells the player (`008aded0` with the object, whether the
/// vendor's list is current, 0), removes the item from the transaction when its tile is
/// selected (`RemoveTransactItem`), else adds it (`AddTransactItem`), and refreshes the
/// lists for its object (`0072dc30`). Unless both transaction lists are empty
/// (`008256d0`; then the amount and the trade button are hidden), shows the amount: the
/// arrow (`"_ArrowDirection"` of the menu tile: 2 when the amount is above 0, else 1),
/// `"%d"` of `0072fa60(004a8f20(int amount, iVendorCaps))` in `pTiles[0xc]`'s `"_Value"`
/// (made visible, trait `0xfa3`), the trade button's trait `0xfaf` (`pTiles[0xd]`) and
/// the amount's alpha (trait `0xfa9`: 255 when affordable and not above the vendor's
/// caps, 64 otherwise). Affordable: an amount above 0, or one at most 0 whose negation is
/// at most the player's caps. A 0x10-byte buffer on the stack.
pub fn barter_menu_item_select_callback(e: &mut Engine, count: i32) {
    let me = barter_menu(e);
    let current = e.get(me, BarterMenu::plbCurrListbox);
    let tile = e
        .call(LIST_GET_TILE_FOR, &args![current, BARTER_CURRENT_ITEM])
        .u32();
    if count == 0 || e.global::<u32>(BARTER_CURRENT_ITEM) == 0 || tile == 0 {
        return;
    }
    let item = e.global::<u32>(BARTER_CURRENT_ITEM);
    let value = barter_menu_get_sort_value(e, tile, item);
    let me = barter_menu(e);
    let sign = if e.get(me, BarterMenu::plbCurrListbox) == me.addr() + PLAYER_ITEMS {
        1
    } else {
        -1
    };
    let amount = x87_mul(x87_load32(value.to_bits()), f64::from(sign));
    let amount = x87_mul(amount, f64::from(count));
    let amount = x87_add(amount, x87_m32(e.mem.u32(me.addr() + 0x84)));
    e.mem
        .set_u32(barter_menu(e).addr() + 0x84, x87_store32(amount));
    let me = barter_menu(e);
    let vendor_side = u32::from(e.get(me, BarterMenu::plbCurrListbox) == me.addr() + VENDOR_ITEMS);
    let item = e.global::<u32>(BARTER_CURRENT_ITEM);
    let object = e.call(ITEM_OBJECT, &args![item]).u32();
    let player = e.global::<u32>(PLAYER);
    e.call(
        PLAYER_BARTER_008ADED0,
        &args![player, object, vendor_side, 0u32],
    );
    let selected = e.global::<u32>(IS_BARTER_SELECTED_TRAIT);
    if e.call(TILE_IS_TRUE, &args![tile, selected]).bool() {
        let me = barter_menu(e);
        barter_menu_remove_transact_item(e, me, count);
    } else {
        let me = barter_menu(e);
        barter_menu_add_transact_item(e, me, count);
    }
    let item = e.global::<u32>(BARTER_CURRENT_ITEM);
    let object = e.call(ITEM_OBJECT, &args![item]).u32();
    let me = barter_menu(e);
    fn_0072dc30(e, me, object);
    let me = barter_menu(e).addr();
    let empty = e
        .call(LIST_IS_EMPTY_008256D0, &args![me + ITEMS_TO_BUY])
        .bool()
        && {
            let me = barter_menu(e).addr();
            e.call(LIST_IS_EMPTY_008256D0, &args![me + ITEMS_TO_SELL])
                .bool()
        };
    let amount_tile = |e: &Engine| barter_tile(e, barter_menu(e), 0xc);
    let trade_tile = |e: &Engine| barter_tile(e, barter_menu(e), 0xd);
    if empty {
        let tile = trade_tile(e);
        e.call(TILE_SET_INT, &args![tile, 0xfafu32, 0u32]);
        let tile = amount_tile(e);
        e.call(TILE_SET_INT, &args![tile, 0xfa3u32, 0u32]);
        return;
    }
    let amount_bits = e.mem.u32(barter_menu(e).addr() + 0x84);
    let rounded = e
        .call(VALUE_00406CE0, &args![x87_quiet_bits(amount_bits)])
        .f64();
    let whole = e
        .call(FLOAT_TO_INT_00406D90, &args![x87_store32(rounded)])
        .i32();
    let zero: f64 = e.global(ZERO_DOUBLE);
    let amount = |e: &Engine| x87_load32(e.mem.u32(barter_menu(e).addr() + 0x84));
    // FCOMP against 0.0; TEST AH,0x41; JNZ: 2 only when the amount is above 0.0.
    let arrow = if amount(e) > zero { 2u32 } else { 1 };
    let which = e.global::<u32>(ARROW_DIRECTION_TRAIT);
    let me = barter_menu(e).addr();
    let menu_tile = e.call(NODE_NEXT, &args![me]).u32();
    e.call(TILE_SET_INT, &args![menu_tile, which, arrow]);
    let vendor_caps = e.get(barter_menu(e), BarterMenu::iVendorCaps);
    let bounded = e.call(BOUND_004A8F20, &args![whole, vendor_caps]).i32();
    let shown = fn_0072fa60(e, bounded);
    // The frame below `EBP`: the buffer is at `EBP - 0x18`.
    e.with_stack(0x10, |e, buffer| {
        let buffer = buffer.addr();
        e.call(FORMAT_TEXT, &args![buffer, 0x10u32, INTEGER_FORMAT, shown]);
        let value_trait = e.global::<u32>(VALUE_TRAIT);
        let tile = amount_tile(e);
        set_string(e, tile, value_trait, buffer);
    });
    let tile = amount_tile(e);
    e.call(TILE_SET_INT, &args![tile, 0xfa3u32, 1u32]);
    let player_caps = |e: &Engine| f64::from(e.get(barter_menu(e), BarterMenu::iPlayerCaps));
    // TEST AH,0x41; JZ: above 0.0 is affordable; else (TEST AH,0x41; JP) NaN is not, and
    // the negated amount must not be above the player's caps (FCOMPP, TEST AH,0x41; JP).
    let affordable = amount(e) > zero || (amount(e) <= zero && -amount(e) <= player_caps(e));
    let dim: u32 = e.global(DIM_ALPHA);
    if affordable {
        let tile = trade_tile(e);
        e.call(TILE_SET_INT, &args![tile, 0xfafu32, 1u32]);
        let vendor_caps = f64::from(e.get(barter_menu(e), BarterMenu::iVendorCaps));
        // Both TEST AH,0x41; JNZ: dimmed only when the amount is above 0.0 and above the
        // vendor's caps.
        let alpha = if amount(e) > zero && amount(e) > vendor_caps {
            dim
        } else {
            e.global(FULL_ALPHA)
        };
        let tile = amount_tile(e);
        e.call(
            TILE_SET_FLOAT,
            &args![tile, 0xfa9u32, x87_quiet_bits(alpha), 1u32],
        );
    } else {
        let tile = trade_tile(e);
        e.call(TILE_SET_INT, &args![tile, 0xfafu32, 0u32]);
        let tile = amount_tile(e);
        e.call(
            TILE_SET_FLOAT,
            &args![tile, 0xfa9u32, x87_quiet_bits(dim), 1u32],
        );
    }
}

/// The transaction list the current side adds to (`ItemsToSell` from the player's
/// list, `ItemsToBuy` from the vendor's) or removes from (the other one).
fn transaction_list(e: &Engine, this: Ptr<BarterMenu>, adding: bool) -> u32 {
    let base = this.addr();
    let player_side = e.get(this, BarterMenu::plbCurrListbox) == base + PLAYER_ITEMS;
    if player_side == adding {
        base + ITEMS_TO_SELL
    } else {
        base + ITEMS_TO_BUY
    }
}

// Translated from 0072faa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0072faa0 - the C++ exception-unwinding frame (FS:[0] around the new ItemChange) is not translated.
/// `BarterMenu::AddTransactItem` (Xbox PDB, `RET 4`: the count): in the transaction
/// list of the current side (`ItemsToSell` for the player's list, else `ItemsToBuy`),
/// adds the count to the current item's entry (`GetTransactItem`, `006ecd40`); without
/// one, makes a new `ItemChange` of the item's object and the count (`operator new(0xc)`,
/// `004bc550`), gives it the item's first extra data (`005ae3d0`, through a local) and
/// adds it to the list (`00905820`, through a local). The entry's own first extra data
/// is read and not used.
pub fn barter_menu_add_transact_item(e: &mut Engine, this: Ptr<BarterMenu>, count: i32) {
    let list = transaction_list(e, this, true);
    let item = e.global::<u32>(BARTER_CURRENT_ITEM);
    let entry = barter_menu_get_transact_item(e, this, list, item);
    let _entry_extra = if entry != 0 { first_extra(e, entry) } else { 0 };
    if entry != 0 {
        let total = e.call(NODE_NEXT, &args![entry]).i32().wrapping_add(count);
        e.call(NODE_SET_NEXT, &args![entry, total]);
        return;
    }
    let item = e.global::<u32>(BARTER_CURRENT_ITEM);
    let extra = if item != 0 { first_extra(e, item) } else { 0 };
    // The frame below `EBP`: the new entry at `EBP - 0x18`, the extra data at `EBP - 0x1c`.
    e.with_stack(8, |e, locals| {
        let entry_local = locals.addr();
        let extra_local = locals.addr() + 4;
        e.mem.set_u32(extra_local, extra);
        let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
        let change = if block != 0 {
            let item = e.global::<u32>(BARTER_CURRENT_ITEM);
            let object = e.call(ITEM_OBJECT, &args![item]).u32();
            e.call(ITEM_CHANGE_CONSTRUCT, &args![block, object, count])
                .u32()
        } else {
            0
        };
        e.mem.set_u32(entry_local, change);
        if e.mem.u32(extra_local) != 0 {
            let change = e.mem.u32(entry_local);
            let extras = e.call(FIRST_WORD, &args![change]).u32();
            e.call(LIST_ADD_005AE3D0, &args![extras, extra_local]);
        }
        e.call(LIST_APPEND_00905820, &args![list, entry_local]);
    });
}

// Translated from 0072fc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BarterMenu::RemoveTransactItem` (Xbox PDB, `RET 4`: the count): in the transaction
/// list the current side's items came from (`ItemsToBuy` for the player's list, else
/// `ItemsToSell`), lowers the current item's entry by the count (`006ecd40`), or when
/// nothing would remain removes it from the list (`00905330`, through a local) and
/// deletes it (`004459e0` with 1). The entry's first extra data is read and not used; the
/// entry is not checked for null.
pub fn barter_menu_remove_transact_item(e: &mut Engine, this: Ptr<BarterMenu>, count: i32) {
    let list = transaction_list(e, this, false);
    let item = e.global::<u32>(BARTER_CURRENT_ITEM);
    let entry = barter_menu_get_transact_item(e, this, list, item);
    let _entry_extra = first_extra(e, entry);
    let left = e.call(NODE_NEXT, &args![entry]).i32().wrapping_sub(count);
    if left > 0 {
        let left = e.call(NODE_NEXT, &args![entry]).i32().wrapping_sub(count);
        e.call(NODE_SET_NEXT, &args![entry, left]);
        return;
    }
    // The frame below `EBP`: the entry at `EBP - 0xc`.
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), entry);
        e.call(EXTRA_REMOVE_00905330, &args![list, local]);
        let entry = e.mem.u32(local.addr());
        if entry != 0 {
            e.call(ITEM_DATA_DELETE, &args![entry, 1u32]);
        }
    });
}

/// Plays a sound by name (`BSAudio::GetSoundHandleByName(QInstance(), handle, name,
/// 0x101)`) and destroys the handle (`00483710`). The 0 pushed first (00730061 /
/// 00730109) is not popped by 00ad7550 (`RET 0xc`, past the no-argument getter
/// 00453a70); it is the word `BSSoundHandle::Play` (00ad8830, `RET 4`) reads.
fn play_transaction_sound(e: &mut Engine, name: u32) {
    // The handle at `EBP - 0x5c` (or `- 0x68`), 0xc bytes.
    e.with_stack(0xc, |e, handle| {
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let sound = e
            .call(AUDIO_SOUND_BY_NAME, &args![audio, handle, name, 0x101u32])
            .u32();
        e.call(SOUND_HANDLE_PLAY, &args![sound, 0u32]);
        e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
    });
}

// Translated from 0072fd10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0072fd10 - the C++ exception-unwinding frame (FS:[0]: the local ItemChange and the sound handles) is not translated.
/// `BarterMenu::CompleteTransaction` (Xbox PDB). The buyer is the vendor's merchant
/// container (`ExtraDataList::GetMerchantContainer` of `005d43c0(vendor)`) or the vendor;
/// the player's current weapon comes from the player's process (slot +0x14c). Each item
/// to buy: its first extra data gets `Script::SetActionFlag(vendor, extra, 0x4000)`, it
/// leaves the barter container (`RemoveItemBarterContainer(item, count, vendor)`) and is
/// deleted. Each item to sell: the same flag with the player, the player's slot +0x17c
/// (object, extra, count, 0, 0, buyer, 0, 0, 1, 0) moves it, and it is deleted. Hides
/// the trade button (`0xfaf`) and the amount (`0xfa3`), counts the caps (misc stat 0x27
/// with the absolute amount); a positive amount takes the caps from the vendor
/// (`RemoveItemBarterContainer` of a local `ItemChange` of the caps form
/// `004839c0(0xf)`, bounded by the vendor's caps) and plays `"ITMBottlecapsUp"`; a
/// negative one removes the gold from the player (`InventoryChanges::RemoveGold(player,
/// -amount, buyer)`) and plays `"ITMBottlecapsDown"`. Clears the amount and both
/// transaction lists, sets `bItemsTraded`, and when the player no longer has the weapon
/// (`InventoryChanges::GetObjectCount` 0) tells the process (slot +0x168 with 0).
pub fn barter_menu_complete_transaction(e: &mut Engine, this: Ptr<BarterMenu>) {
    let base = this.addr();
    // The frame's saved security cookie (pushed at 0072fd2e, the frame's stack
    // top); the frame's `EBP` is modelled by the block below.
    e.with_stack(0xa0, |e, frame| {
        let ebp = frame.addr() + 0xa0;
        let cookie = e.global::<u32>(SECURITY_COOKIE) ^ ebp;
        e.mem.set_u32(ebp - 0x9c, cookie);
        complete_transaction_body(e, this, base);
    });
}

/// The body of [`barter_menu_complete_transaction`].
fn complete_transaction_body(e: &mut Engine, this: Ptr<BarterMenu>, base: u32) {
    let vendor = e.get(this, BarterMenu::pVendorRef);
    let extras = e.call(EXTRA_LIST_005D43C0, &args![vendor]).u32();
    let buyer = if e.call(GET_MERCHANT_CONTAINER, &args![extras]).u32() != 0 {
        let vendor = e.get(this, BarterMenu::pVendorRef);
        let extras = e.call(EXTRA_LIST_005D43C0, &args![vendor]).u32();
        e.call(GET_MERCHANT_CONTAINER, &args![extras]).u32()
    } else {
        e.get(this, BarterMenu::pVendorRef)
    };
    let player = e.global::<u32>(PLAYER);
    let process = e.call(ACTOR_PROCESS, &args![player]).u32();
    let weapon = e.vcall(process, 0x14c, &args![]).u32();
    let mut node = base + ITEMS_TO_BUY;
    while walk_item(e, node) != 0 {
        let item = node_item(e, node);
        let extra = first_extra(e, item);
        if extra != 0 {
            let vendor = e.get(this, BarterMenu::pVendorRef);
            e.call(SET_ACTION_FLAG, &args![vendor, extra, 0x4000u32]);
        }
        let vendor = e.get(this, BarterMenu::pVendorRef);
        let count = e.call(NODE_NEXT, &args![item]).u32();
        let handler = e.global::<u32>(DATA_HANDLER);
        e.call(
            REMOVE_ITEM_BARTER_CONTAINER,
            &args![handler, item, count, vendor],
        );
        if item != 0 {
            e.call(ITEM_DATA_DELETE, &args![item, 1u32]);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    let mut node = base + ITEMS_TO_SELL;
    while walk_item(e, node) != 0 {
        let item = node_item(e, node);
        let extra = first_extra(e, item);
        if extra != 0 {
            let player = e.global::<u32>(PLAYER);
            e.call(SET_ACTION_FLAG, &args![player, extra, 0x4000u32]);
        }
        let count = e.call(NODE_NEXT, &args![item]).u32();
        let object = e.call(ITEM_OBJECT, &args![item]).u32();
        let player = e.global::<u32>(PLAYER);
        e.vcall(
            player,
            0x17c,
            &args![object, extra, count, 0u32, 0u32, buyer, 0u32, 0u32, 1u32, 0u32],
        );
        if item != 0 {
            e.call(ITEM_DATA_DELETE, &args![item, 1u32]);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    let trade = barter_tile(e, barter_menu(e), 0xd);
    e.call(TILE_SET_INT, &args![trade, 0xfafu32, 0u32]);
    let amount_tile = barter_tile(e, barter_menu(e), 0xc);
    e.call(TILE_SET_INT, &args![amount_tile, 0xfa3u32, 0u32]);
    let amount_bits = e.mem.u32(base + 0x84);
    let rounded = e
        .call(VALUE_00406CE0, &args![x87_quiet_bits(amount_bits)])
        .f64();
    let whole = e
        .call(FLOAT_TO_INT_00406D90, &args![x87_store32(rounded)])
        .i32();
    let size = e.call(ABS_00EC7D40, &args![whole]).u32();
    e.call(MISC_STAT_MOD_VAL, &args![0x27u32, size]);
    let zero: f64 = e.global(ZERO_DOUBLE);
    let amount = x87_load32(e.mem.u32(base + 0x84));
    // FCOMP against 0.0: TEST AH,0x41; JNZ, then TEST AH,5; JP.
    if amount > zero {
        // The local `ItemChange` at `EBP - 0x40`.
        e.with_stack(0xc, |e, change| {
            let caps = e.call(CAPS_FORM_004839C0, &args![0xfu32]).u32();
            e.call(ITEM_CHANGE_CONSTRUCT, &args![change, caps, 0u32]);
            let inventory = e.call(GET_INVENTORY_CHANGES, &args![buyer]).u32();
            if inventory != 0 {
                let vendor = e.get(this, BarterMenu::pVendorRef);
                let vendor_caps = e.get(this, BarterMenu::iVendorCaps);
                let bounded = e.call(BOUND_004A8F20, &args![whole, vendor_caps]).u32();
                let handler = e.global::<u32>(DATA_HANDLER);
                e.call(
                    REMOVE_ITEM_BARTER_CONTAINER,
                    &args![handler, change, bounded, vendor],
                );
            }
            play_transaction_sound(e, CAPS_UP_SOUND);
            e.call(ITEM_CHANGE_DESTRUCT, &args![change]);
        });
    } else if amount < zero {
        let player = e.global::<u32>(PLAYER);
        let inventory = e.call(GET_INVENTORY_CHANGES, &args![player]).u32();
        if inventory != 0 {
            let player = e.global::<u32>(PLAYER);
            e.call(
                INVENTORY_REMOVE_GOLD,
                &args![inventory, player, whole.wrapping_neg(), buyer],
            );
        }
        play_transaction_sound(e, CAPS_DOWN_SOUND);
    }
    e.mem.set_u32(base + 0x84, x87_store32(0.0));
    e.call(LIST_CLEAR, &args![base + ITEMS_TO_BUY]);
    e.call(LIST_CLEAR, &args![base + ITEMS_TO_SELL]);
    e.set(this, BarterMenu::bItemsTraded, 1);
    if weapon != 0 {
        let object = e.call(ITEM_OBJECT, &args![weapon]).u32();
        let player = e.global::<u32>(PLAYER);
        let inventory = e.call(GET_INVENTORY_CHANGES, &args![player]).u32();
        if e.call(GET_OBJECT_COUNT, &args![inventory, object]).u32() == 0 {
            let player = e.global::<u32>(PLAYER);
            let process = e.call(ACTOR_PROCESS, &args![player]).u32();
            e.vcall(process, 0x168, &args![0u32]);
        }
    }
}

// Translated from 007301f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007301f0 - the /GS cookie check (00ec408c) on the title buffer is not translated.
/// `BarterMenu::SetFilterModeTitle` (Xbox PDB, `RET 4`: the tab tile): the title of the
/// side's filter mode (`ePlayerFilterMode` for `pTiles[3]`, else `eVendorFilterMode`;
/// jump table `00730478`): mode 0 is the player label setting (`011d4378`) on the
/// player's tab or the vendor's name (`0055d520`), modes 1-5 the settings `011d29b8`,
/// `011d4d50`, `011d40b4`, `011d2acc`, `011d4918`. Copied into a 0x104-byte buffer
/// (`00406d30`) and, except for the vendor's name, upper-cased (`00730490`) unless the
/// language setting (`011c3cb4`) differs from `"ENGLISH"` (`00408b20`). The tile's string
/// trait `0xfc4` gets the buffer (left as the stack held it for a mode above 5).
pub fn barter_menu_set_filter_mode_title(e: &mut Engine, this: Ptr<BarterMenu>, tile: u32) {
    let base = this.addr();
    let mode = if tile == e.mem.u32(base + 0x34) {
        e.get(this, BarterMenu::ePlayerFilterMode)
    } else {
        e.get(this, BarterMenu::eVendorFilterMode)
    };
    let language = e.call(SETTING_STRING, &args![LANGUAGE_SETTING]).u32();
    let foreign = e
        .call(COMPARE_NAME_00408B20, &args![language, ENGLISH_TEXT])
        .u32()
        != 0;
    // The frame below `EBP`: the buffer is at `EBP - 0x110`.
    e.with_stack(0x104, |e, buffer| {
        let buffer = buffer.addr();
        let copy_setting = |e: &mut Engine, setting: u32| {
            let text = e.call(SETTING_STRING, &args![setting]).u32();
            e.call(COPY_TEXT, &args![buffer, 0x104u32, text]);
            if !foreign {
                fn_00730490(e, buffer, 0x104);
            }
        };
        match mode {
            0 => {
                if tile == e.mem.u32(base + 0x34) {
                    copy_setting(e, 0x011d_4378);
                } else {
                    let vendor = e.get(this, BarterMenu::pVendorRef);
                    let name = e.call(REFERENCE_NAME, &args![vendor]).u32();
                    e.call(COPY_TEXT, &args![buffer, 0x104u32, name]);
                }
            }
            1 => copy_setting(e, 0x011d_29b8),
            2 => copy_setting(e, 0x011d_4d50),
            3 => copy_setting(e, 0x011d_40b4),
            4 => copy_setting(e, 0x011d_2acc),
            5 => copy_setting(e, 0x011d_4918),
            _ => {}
        }
        set_string(e, tile, 0xfc4, buffer);
    });
}

// Translated from 007304b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BarterMenu::FilterModeFunc` (Xbox PDB, cdecl: the item): the item's tile in the
/// player's list (`GetTileFor` with the address of the argument) or else the vendor's,
/// and that side's filter mode (jump table `00730678`): mode 1 keeps type 0x28, 2 types
/// 0x18 and 0x1a, 3 types 0x1d, 0x2f and 0x19, 4 filters out the types 0x28, 0x18, 0x1a,
/// 0x1d, 0x2f, 0x29, 0x19, 0x6c, 0x73 and 0x74, 5 keeps type 0x29; other modes keep
/// everything. A kept item on a tile is still filtered out when its count less the tile's
/// `"_NumBartered"` is at most 0.0. True means filtered out.
pub fn barter_menu_filter_mode_func(e: &mut Engine, item: u32) -> bool {
    // The argument's own stack slot, whose address the code passes on.
    e.with_stack(4, |e, argument| {
        let argument = argument.addr();
        e.mem.set_u32(argument, item);
        let mut filtered = false;
        let object = e.call(ITEM_OBJECT, &args![item]).u32();
        let kind = e.call(FORM_TYPE, &args![object]).u32();
        let me = barter_menu(e).addr();
        let mut tile = e
            .call(LIST_GET_TILE_FOR, &args![me + PLAYER_ITEMS, argument])
            .u32();
        let mode = if tile != 0 {
            e.get(barter_menu(e), BarterMenu::ePlayerFilterMode)
        } else {
            let me = barter_menu(e).addr();
            tile = e
                .call(LIST_GET_TILE_FOR, &args![me + VENDOR_ITEMS, argument])
                .u32();
            e.get(barter_menu(e), BarterMenu::eVendorFilterMode)
        };
        match mode {
            1 => filtered = kind != 0x28,
            2 => filtered = kind != 0x18 && kind != 0x1a,
            3 => filtered = kind != 0x1d && kind != 0x2f && kind != 0x19,
            4 => {
                filtered = matches!(
                    kind,
                    0x28 | 0x18 | 0x1a | 0x1d | 0x2f | 0x29 | 0x19 | 0x6c | 0x73 | 0x74
                );
            }
            5 => filtered = kind != 0x29,
            _ => {}
        }
        if tile != 0 && !filtered {
            let item = e.mem.u32(argument);
            let count = e.call(NODE_NEXT, &args![item]).i32();
            let count = f64::from(count);
            let numbered = e.call(TEXT_TO_TRAIT, &args![NUM_BARTERED_TEXT]).u32();
            let bartered = e.call(TILE_GET_FLOAT, &args![tile, numbered]).f64();
            let left = x87_sub(count, bartered);
            // FCOMP against 0.0; TEST AH,0x41; JNP: filtered when at most 0.0 (not NaN).
            filtered = left <= e.global::<f64>(ZERO_DOUBLE);
        }
        filtered
    })
}

// Translated from 00730690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, a `bool`): with true, refreshes the open barter menu's lists
/// (`0072dc30` with 0) and clears the refresh request byte at `011d8fc0`; with false,
/// sets it (the next `DoIdle` refreshes).
pub fn fn_00730690(e: &mut Engine, now: u8) {
    if now != 0 {
        if barter_menu(e).addr() != 0 {
            let me = barter_menu(e);
            fn_0072dc30(e, me, 0);
        }
        e.set_global(REFRESH_REQUESTED_011D8FC0, 0u8);
    } else {
        e.set_global(REFRESH_REQUESTED_011D8FC0, 1u8);
    }
}

// Translated from 007306d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BarterMenu::DoIdle` (Xbox PDB): runs a requested refresh (`00730690(1)` when the
/// byte at `011d8fc0` is 1); while the byte at `0119fdb0` is set, shows the current
/// item's stats (`00719790`, cdecl): for an object of type 0x28 with `pTiles[0xe]`,
/// `pTiles[0xf]` and `pTiles[0x10]`; for type 0x18 with `pTiles[0x11]` when
/// `004be060` of the object is not 0.0 and `pTiles[0x12]` when `004be180` of it is not
/// 0.0 (each else none), then the item and 0.
pub fn barter_menu_do_idle(e: &mut Engine, this: Ptr<BarterMenu>) {
    let base = this.addr();
    if e.global::<u8>(REFRESH_REQUESTED_011D8FC0) == 1 {
        fn_00730690(e, 1);
    }
    if e.global::<u8>(ITEM_ENTERED_0119FDB0) == 0 {
        return;
    }
    let item_kind = |e: &mut Engine| {
        let item = e.global::<u32>(BARTER_CURRENT_ITEM);
        let object = e.call(ITEM_OBJECT, &args![item]).u32();
        e.call(FORM_TYPE, &args![object]).u32()
    };
    if e.global::<u32>(BARTER_CURRENT_ITEM) != 0 && item_kind(e) == 0x28 {
        let item = e.global::<u32>(BARTER_CURRENT_ITEM);
        let tiles = [0x60u32, 0x64, 0x68].map(|offset| e.mem.u32(base + offset));
        e.call(
            SHOW_ITEM_STATS_00719790,
            &args![tiles[0], tiles[1], tiles[2], item, 0u32],
        );
        return;
    }
    if e.global::<u32>(BARTER_CURRENT_ITEM) == 0 || item_kind(e) != 0x18 {
        return;
    }
    let zero: f64 = e.global(ZERO_DOUBLE);
    let stat_tile = |e: &mut Engine, getter: u32, offset: u32| {
        if e.global::<u32>(BARTER_CURRENT_ITEM) == 0 {
            return 0;
        }
        let item = e.global::<u32>(BARTER_CURRENT_ITEM);
        let object = e.call(ITEM_OBJECT, &args![item]).u32();
        let value = e.call(getter, &args![object]).f64();
        // FCOMP against 0.0; TEST AH,0x44; JNP: none only when equal.
        if value != zero {
            e.mem.u32(base + offset)
        } else {
            0
        }
    };
    let second = stat_tile(e, STAT_004BE180, 0x70);
    let first = stat_tile(e, STAT_004BE060, 0x6c);
    let item = e.global::<u32>(BARTER_CURRENT_ITEM);
    e.call(
        SHOW_ITEM_STATS_00719790,
        &args![first, second, 0u32, item, 0u32],
    );
}

/// Whether a tile's `"_enabled"` trait (looked up by `Tile::AddUserTrait` each time) is
/// true.
fn tile_enabled(e: &mut Engine, tile: u32) -> bool {
    let enabled = e
        .call(ADD_USER_TRAIT, &args![ENABLED_TEXT, 0xffff_ffffu32])
        .u32();
    e.call(TILE_IS_TRUE, &args![tile, enabled]).bool()
}

/// The page step of `GetHighlightTarget`: 0 without a container; otherwise the
/// container's trait `0xfb0` (negated for a page up, `FCHS`) divided (`FDIVR`) by its
/// `"_scroll_delta"`, stored as a `float`, passed with 1.0 to `004bd510` and truncated
/// (`_ftol2`).
fn page_step(e: &mut Engine, this: Ptr<ListBox>, up: bool) -> i32 {
    if e.get(this, ListBox::pContainerRect).is_null() {
        return 0;
    }
    let container = e.get(this, ListBox::pContainerRect);
    let extent = e.call(TILE_GET_FLOAT, &args![container, 0xfb0u32]).f64();
    let numerator = if up { -extent } else { extent };
    let scroll_delta = e.call(TEXT_TO_TRAIT, &args![SCROLL_DELTA_TEXT]).u32();
    let container = e.get(this, ListBox::pContainerRect);
    let delta = e
        .call(TILE_GET_FLOAT, &args![container, scroll_delta])
        .f64();
    let step = x87_store32(x87_div(numerator, delta));
    let rounded = e
        .call(FLOAT_PAIR_004BD510, &args![step, x87_store32(1.0)])
        .f64();
    e.call(FTOL2, &args![rounded]).i32()
}

// Translated from 00730f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<ItemChange_P>::GetHighlightTarget` (Xbox PDB, `RET 4`): the tile a move in
/// `direction` highlights (1 up, 2 down, 0xf page up, 0x10 page down, by the jump tables
/// `007312bc`/`007312d0`; others do not move), or 0 when there is none or it is the one
/// already highlighted. With the list enabled, a step and items: the index starts from
/// the highlighted tile's index (`0xfac`) plus the step, kept within the items (else 0
/// for a step down, the last for a step up), and the item tile at that index is taken; a
/// disabled one (`"_enabled"`) moves the index on by one and searches again, and past
/// either end the tiles at the indices from the other side (slot +0x14) are tried until
/// an enabled one is found or the indices run out.
pub fn list_box_item_change_get_highlight_target(
    e: &mut Engine,
    this: Ptr<ListBox>,
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
    let searching = list_box_item_change_is_enabled(e, this)
        && step != 0
        && e.get(this, ListBox::usNextIndex) != 0;
    if searching {
        let last = i32::from(e.get(this, ListBox::usNextIndex)) - 1;
        let mut index = if !e.get(this, ListBox::pHighlightedItem).is_null() {
            let highlighted = e.get(this, ListBox::pHighlightedItem);
            let value = e.call(TILE_GET_FLOAT, &args![highlighted, 0xfacu32]).f64();
            let moved = e.call(FTOL2, &args![value]).i32().wrapping_add(step);
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
            while target == 0 && walk_item(e, node) != 0 {
                let item = node_item(e, node);
                let tile = e.mem.u32(item);
                let value = e.call(TILE_GET_FLOAT, &args![tile, 0xfacu32]).f64();
                // FILD index; FUCOMPP; TEST AH,0x44; JP: taken only when equal.
                if f64::from(index) == value {
                    let item = node_item(e, node);
                    target = e.mem.u32(item);
                }
                node = e.call(NODE_NEXT, &args![node]).u32();
            }
            if target == 0 || tile_enabled(e, target) {
                break;
            }
            target = 0;
            if step > 0 {
                index = index.wrapping_add(1);
                if index > last {
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
            }
            if step < 0 {
                index = index.wrapping_sub(1);
                if index < 0 {
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

// Translated from 00731580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<ItemChange_P>::`scalar deleting destructor'` (Xbox PDB): the destructor body
/// (`007308e0`), then the block freed (`00401030`) when bit 0 of `flags` is set. Returns
/// `this`.
pub fn list_box_item_change_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ListBox>,
    flags: u32,
) -> Ptr<ListBox> {
    fn_007308e0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 007315b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on a `ListBox<ItemChange *>`): the tile of its last unfiltered
/// item (slot +0x14 with `NumUnfiltered - 1` and 0).
pub fn fn_007315b0(e: &mut Engine, this: Ptr<ListBox>) -> u32 {
    let count = e.call(ITEM_CHANGE_NUM_UNFILTERED, &args![this]).u32();
    e.vcall(this.addr(), 0x14, &args![count.wrapping_sub(1), 0u32])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0072_fa60, fn_0072fa60(i32) -> i32),
        entry!(0x0072_fa80, fn_0072fa80(i32) -> i32),
        entry!(0x0073_0490, fn_00730490(u32, u32) -> u32),
        entry!(
            0x0073_0810,
            list_box_item_change_list_box(Ptr<ListBox>) -> Ptr<ListBox>
        ),
        entry!(0x0073_08e0, fn_007308e0(Ptr<ListBox>)),
        entry!(0x0073_0950, list_box_item_change_remove_all(Ptr<ListBox>)),
        entry!(0x0073_0b80, list_box_item_change_sort(Ptr<ListBox>, u32)),
        entry!(0x0073_0bb0, list_box_item_change_filter(Ptr<ListBox>, u32)),
        entry!(
            0x0073_0e20,
            list_box_item_change_is_enabled(Ptr<ListBox>) -> bool
        ),
        entry!(
            0x0073_0ed0,
            list_box_item_change_highlight(Ptr<ListBox>, u32) -> bool
        ),
        entry!(
            0x0073_1360,
            list_box_item_change_restore_list_state(Ptr<ListBox>, u8)
        ),
        entry!(0x0073_14c0, fn_007314c0(Ptr<ListBox>, u32, i32, i32)),
        entry!(
            0x0072_ceb0,
            barter_menu_barter_menu(Ptr<BarterMenu>) -> Ptr<BarterMenu>
        ),
        entry!(0x0072_d030, barter_menu_get_class(Ptr<BarterMenu>) -> u32),
        entry!(
            0x0072_d070,
            barter_menu_scalar_deleting_destructor(Ptr<BarterMenu>, u32) -> Ptr<BarterMenu>
        ),
        entry!(0x0072_d0a0, fn_0072d0a0(Ptr<BarterMenu>)),
        entry!(0x0072_d250, barter_menu_create(u32, i32) -> u32),
        entry!(0x0072_d6c0, fn_0072d6c0() -> u32),
        entry!(0x0072_d6d0, fn_0072d6d0()),
        entry!(0x0072_d770, barter_menu_do_click(Ptr<BarterMenu>, i32, u32)),
        entry!(0x0072_db40, fn_0072db40()),
        entry!(0x0072_db60, barter_menu_update_caps_labels(Ptr<BarterMenu>)),
        entry!(0x0072_dc30, fn_0072dc30(Ptr<BarterMenu>, u32)),
        entry!(0x0072_e370, barter_menu_do_enter(Ptr<BarterMenu>, i32, u32)),
        entry!(0x0072_e530, barter_menu_do_leave(Ptr<BarterMenu>, u32, u32)),
        entry!(0x0072_e570, barter_menu_player_first_pass_filter_func(u32) -> bool),
        entry!(0x0072_e6c0, barter_menu_vendor_first_pass_filter_func(u32) -> bool),
        entry!(0x0072_eac0, barter_menu_remove_stolen_items(u32, u32)),
        entry!(0x0072_ed00, barter_menu_get_sort_value(u32, u32) -> f32),
        entry!(0x0072_f070, barter_menu_sort_func(u32, u32) -> i32),
        entry!(
            0x0072_f120,
            barter_menu_get_transact_item(Ptr<BarterMenu>, u32, u32) -> u32
        ),
        entry!(0x0072_f210, barter_menu_init_func(u32, u32)),
        entry!(
            0x0072_f4b0,
            barter_menu_do_gamepad(Ptr<BarterMenu>, i32, u32) -> bool
        ),
        entry!(
            0x0072_f680,
            barter_menu_do_wheel_move(Ptr<BarterMenu>, i32, u32)
        ),
        entry!(0x0072_f6f0, barter_menu_item_select_callback(i32)),
        entry!(
            0x0072_faa0,
            barter_menu_add_transact_item(Ptr<BarterMenu>, i32)
        ),
        entry!(
            0x0072_fc30,
            barter_menu_remove_transact_item(Ptr<BarterMenu>, i32)
        ),
        entry!(
            0x0072_fd10,
            barter_menu_complete_transaction(Ptr<BarterMenu>)
        ),
        entry!(
            0x0073_01f0,
            barter_menu_set_filter_mode_title(Ptr<BarterMenu>, u32)
        ),
        entry!(0x0073_04b0, barter_menu_filter_mode_func(u32) -> bool),
        entry!(0x0073_0690, fn_00730690(u8)),
        entry!(0x0073_06d0, barter_menu_do_idle(Ptr<BarterMenu>)),
        entry!(
            0x0073_0f40,
            list_box_item_change_get_highlight_target(Ptr<ListBox>, u32) -> u32
        ),
        entry!(
            0x0073_1580,
            list_box_item_change_scalar_deleting_destructor(Ptr<ListBox>, u32) -> Ptr<ListBox>
        ),
        entry!(0x0073_15b0, fn_007315b0(Ptr<ListBox>) -> u32),
    ]
}

#[cfg(test)]
mod tests {
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

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// An engine with the statics, the flag byte and the doubles mapped, and the list
    /// node helpers doubled (`006815c0` returns the node, `00726070` its next word).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x011d_8000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.register(NODE_SELF, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(ADD_USER_TRAIT, |_, _| 0x66u32.into_ret());
        e.register(FTOL2, |_, a| {
            (f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32 as u32).into_ret()
        });
        stub(
            &mut e,
            &[
                ITEM_CHANGE_SYNCHRONIZE,
                ITEM_CHANGE_UPDATE_HIGHLIGHT_BOX_POS,
                ITEM_CHANGE_SET_TOTAL_LIST_HEIGHT,
            ],
        );
        e
    }

    fn list_box(e: &mut Engine) -> Ptr<ListBox> {
        Ptr::new(e.mem.alloc(0x30))
    }

    /// An item (tile, data, filtered byte).
    fn item(e: &mut Engine, tile: u32, data: u32, filtered: u8) -> u32 {
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item, tile);
        e.mem.set_u32(item + 4, data);
        e.mem.set_u8(item + 8, filtered);
        item
    }

    /// Three items in the list box: the first in the embedded node, two more in nodes.
    fn three_items(e: &mut Engine, this: Ptr<ListBox>, items: [u32; 3]) {
        let node2 = e.mem.alloc(8);
        let node3 = e.mem.alloc(8);
        e.mem.set_u32(this.addr() + 4, items[0]);
        e.mem.set_u32(this.addr() + 8, node2);
        e.mem.set_u32(node2, items[1]);
        e.mem.set_u32(node2 + 4, node3);
        e.mem.set_u32(node3, items[2]);
    }

    #[test]
    fn the_forwarders_return_their_callees_results() {
        let mut e = engine();
        e.register(ABS_00EC7D40, |_, a| (a[0] as i32).wrapping_abs().into_ret());
        e.register(STRUPR_S, |_, a| (a[0] + a[1]).into_ret());
        assert_eq!(e.call(0x0072_fa60, &args![-7i32]).i32(), 7);
        assert_eq!(e.call(0x0072_fa80, &args![9i32]).i32(), 9);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0073_0490, &args![0x100u32, 0x20u32]).u32(), 0x120);
        assert_eq!(calls_to(&e, STRUPR_S), vec![vec![0x100, 0x20]]);
    }

    #[test]
    fn the_constructor_builds_the_item_change_list_box() {
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
        assert_eq!(e.call(0x0073_0810, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), ITEM_CHANGE_LIST_BOX_VTABLE);
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
    fn the_destructor_empties_and_destroys_the_list() {
        let mut e = engine();
        stub(
            &mut e,
            &[LIST_BOX_UNREGISTER, LIST_DESTRUCT, TILE_SET_FLOAT],
        );
        let this = list_box(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0073_08e0, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), ITEM_CHANGE_LIST_BOX_VTABLE);
        assert_eq!(calls_to(&e, LIST_BOX_UNREGISTER), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, LIST_DESTRUCT), vec![vec![this.addr() + 4]]);
        assert_eq!(
            calls_to(&e, ITEM_CHANGE_SET_TOTAL_LIST_HEIGHT),
            vec![vec![this.addr()]]
        );
        assert!(calls_to(&e, TILE_SET_FLOAT).is_empty());
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
            ],
        );
        e.register(LIST_BOX_HAS_FLAG, |_, a| u32::from(a[1] == 1).into_ret());
        e.register(TEXT_TO_TRAIT, |_, _| 0x55u32.into_ret());
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
        e.call(0x0073_0950, &args![this]);
        assert_eq!(calls_to(&e, FREE), vec![vec![second], vec![first]]);
        assert_eq!(calls_to(&e, NODE_SET_NEXT), vec![vec![node, 0]]);
        assert_eq!(calls_to(&e, NODE_DELETE), vec![vec![node, 1]]);
        assert_eq!(calls_to(&e, ITEM_DATA_DELETE), vec![vec![0x8000, 1]]);
        assert_eq!(calls_to(&e, 0x0000_7000), vec![vec![tile, 1]]);
        assert_eq!(e.mem.u32(this.addr() + 4), 0);
        assert_eq!(e.mem.u32(this.addr() + 8), 0);
        assert_eq!(e.get(this, ListBox::usNextIndex), 0);
        assert_eq!(e.get(this, ListBox::fTotalListHeight), 0.0);
        assert!(e.get(this, ListBox::pHighlightedItem).is_null());
        assert_eq!(calls_to(&e, TILE_SET_FLOAT), vec![vec![0x3000, 0x55, 0, 1]]);
        assert_eq!(
            calls_to(&e, ITEM_CHANGE_UPDATE_HIGHLIGHT_BOX_POS),
            vec![vec![this.addr()]]
        );
        // With the keep-tiles byte set the tile is not deleted.
        e.set_global(KEEP_ITEM_TILES, 1u8);
        let first = item(&mut e, tile, 0, 0);
        e.mem.set_u32(this.addr() + 4, first);
        e.call_log = Some(vec![]);
        e.call(0x0073_0950, &args![this]);
        assert!(calls_to(&e, 0x0000_7000).is_empty());
        assert_eq!(calls_to(&e, FREE), vec![vec![first]]);
    }

    #[test]
    fn sort_sorts_the_list_then_synchronizes() {
        let mut e = engine();
        stub(&mut e, &[LIST_SORT]);
        let this = list_box(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0073_0b80, &args![this, 0x1234u32]);
        assert_eq!(calls_to(&e, LIST_SORT), vec![vec![this.addr() + 4, 0x1234]]);
        assert_eq!(
            calls_to(&e, ITEM_CHANGE_SYNCHRONIZE),
            vec![vec![this.addr()]]
        );
    }

    #[test]
    fn filter_marks_numbers_and_moves_the_highlight() {
        let mut e = engine();
        stub(
            &mut e,
            &[TILE_SET_INT, INIT_TILE_POSITION, TILE_SET_INTEGER_VALUE],
        );
        // The filter drops data 0xd2.
        e.register(0x00ab_0000, |_, a| u32::from(a[0] == 0xd2).into_ret());
        e.register(TILE_GET_FLOAT, |_, _| st0(1.0));
        let this = list_box(&mut e);
        let items = [
            item(&mut e, 0x6100, 0xd1, 1),
            item(&mut e, 0x6200, 0xd2, 0),
            item(&mut e, 0x6300, 0xd3, 1),
        ];
        three_items(&mut e, this, items);
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        e.set(this, ListBox::pHighlightedItem, Ptr::new(0x6200));
        e.call_log = Some(vec![]);
        e.call(0x0073_0bb0, &args![this, 0x00ab_0000u32]);
        assert_eq!(
            [items[0], items[1], items[2]].map(|i| e.mem.u8(i + 8)),
            [0, 1, 0]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![
                vec![0x6100, 0xfac, 0],
                vec![0x6200, 0xfac, 0xffff_ffff],
                vec![0x6300, 0xfac, 1]
            ]
        );
        // The highlighted tile was filtered out at index 1: the next unfiltered item takes
        // index 1 and the highlight.
        assert_eq!(e.get(this, ListBox::pHighlightedItem).addr(), 0x6300);
        assert_eq!(
            calls_to(&e, TILE_SET_INTEGER_VALUE),
            vec![vec![0x3000, 0x66, 1]]
        );
        assert_eq!(e.mem.u32(0x011d_9038) & 1, 1);
        assert_eq!(e.get(this, ListBox::usNextIndex), 2);
    }

    #[test]
    fn filter_falls_back_to_the_tile_at_the_index_then_007315b0() {
        let mut e = engine();
        stub(
            &mut e,
            &[TILE_SET_INT, INIT_TILE_POSITION, TILE_SET_INTEGER_VALUE],
        );
        e.register(0x00ab_0000, |_, _| 1u32.into_ret());
        e.register(TILE_GET_FLOAT, |_, _| st0(2.0));
        e.register(0x0000_7014, |_, _| Ret::default());
        e.register(ITEM_CHANGE_007315B0, |_, _| 0x6900u32.into_ret());
        let mut slots = vec![0u32; 6];
        slots[5] = 0x0000_7014;
        e.put_vtable(0x0f00_0000, &slots);
        let this = list_box(&mut e);
        e.mem.set_u32(this.addr(), 0x0f00_0000);
        let first = item(&mut e, 0x6100, 0, 0);
        e.mem.set_u32(this.addr() + 4, first);
        e.set(this, ListBox::pHighlightedItem, Ptr::new(0x6100));
        e.call_log = Some(vec![]);
        e.call(0x0073_0bb0, &args![this, 0x00ab_0000u32]);
        assert_eq!(calls_to(&e, 0x0000_7014), vec![vec![this.addr(), 2, 0]]);
        assert_eq!(calls_to(&e, ITEM_CHANGE_007315B0), vec![vec![this.addr()]]);
        assert_eq!(e.get(this, ListBox::pHighlightedItem).addr(), 0x6900);
        // A null filter filters nothing out and keeps the highlight.
        e.mem.set_u8(first + 8, 1);
        e.set(this, ListBox::pHighlightedItem, Ptr::new(0x6100));
        e.call_log = Some(vec![]);
        e.call(0x0073_0bb0, &args![this, 0u32]);
        assert_eq!(e.mem.u8(first + 8), 0);
        assert_eq!(e.get(this, ListBox::pHighlightedItem).addr(), 0x6100);
        assert!(calls_to(&e, ITEM_CHANGE_007315B0).is_empty());
    }

    #[test]
    fn is_enabled_and_highlight_need_an_enabled_container() {
        let mut e = engine();
        e.register(ADD_USER_TRAIT, |_, _| 0x44u32.into_ret());
        e.register(TILE_IS_TRUE, |_, a| u32::from(a[0] != 0x6100).into_ret());
        e.register(LIST_BOX_BELONGS_TO, |_, a| {
            u32::from(a[1] != 0x6200).into_ret()
        });
        let this = list_box(&mut e);
        assert!(!e.call(0x0073_0e20, &args![this]).bool());
        assert_eq!(e.mem.u32(0x011d_903c), 0x44);
        assert!(!e.call(0x0073_0ed0, &args![this, 0x6000u32]).bool());
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        assert!(e.call(0x0073_0e20, &args![this]).bool());
        assert!(!e.call(0x0073_0ed0, &args![this, 0x6100u32]).bool());
        assert!(!e.call(0x0073_0ed0, &args![this, 0x6200u32]).bool());
        assert!(e.get(this, ListBox::pHighlightedItem).is_null());
        e.call_log = Some(vec![]);
        assert!(e.call(0x0073_0ed0, &args![this, 0x6000u32]).bool());
        assert_eq!(e.get(this, ListBox::pHighlightedItem).addr(), 0x6000);
        assert_eq!(
            calls_to(&e, ITEM_CHANGE_UPDATE_HIGHLIGHT_BOX_POS),
            vec![vec![this.addr()]]
        );
        assert!(e.call(0x0073_0ed0, &args![this, 0u32]).bool());
        assert!(e.get(this, ListBox::pHighlightedItem).is_null());
    }

    #[test]
    fn restore_list_state_focuses_and_scrolls_to_the_saved_item() {
        let mut e = engine();
        e.register(TEXT_TO_TRAIT, |_, a| (a[0] & 0xfff).into_ret());
        e.register(ITEM_CHANGE_NUM_UNFILTERED, |_, _| 10u32.into_ret());
        e.register(TILE_GET_FLOAT, |_, a| {
            st0(if a[1] == 0xfaa { 7.0 } else { 0.0 })
        });
        e.register(FLOAT_PAIR_0040EBD0, |_, a| {
            st0(f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))))
        });
        e.register(FOCUS_OWNER_004B7210, |_, _| 0x7700u32.into_ret());
        e.register(FLAG_004B71D0, |_, _| Ret::default());
        e.register(TILE_GET_MENU_TILE, |_, _| 0x5100u32.into_ret());
        e.register(TILE_GET_MENU, |_, _| 0x5200u32.into_ret());
        stub(
            &mut e,
            &[TILE_SET_FLOAT, TILE_POKE, SET_CURRENT_FOCUS_TARGET],
        );
        e.register(0x0000_7014, |_, _| 0x6000u32.into_ret());
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
        e.call(0x0073_1360, &args![this, 1u32]);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x3400, 0x2d8, 12.5f32.to_bits(), 0]]
        );
        assert_eq!(
            calls_to(&e, TILE_POKE),
            vec![vec![0x3400, 0x250, 1.0f32.to_bits()]]
        );
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
        e.register(FLAG_004B71D0, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0073_1360, &args![this, 0u32]);
        assert!(calls_to(&e, 0x0000_7114).is_empty());
        e.set(this, ListBox::pScrollBar, Ptr::NULL);
        e.call_log = Some(vec![]);
        e.call(0x0073_1360, &args![this, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn fn_007314c0_calls_back_for_the_index_range() {
        let mut e = engine();
        e.register(TILE_GET_FLOAT, |_, a| st0(f64::from(a[0] - 0x6100) / 256.0));
        e.register(0x00ab_0000, |_, _| Ret::default());
        let this = list_box(&mut e);
        let items = [
            item(&mut e, 0x6100, 0xd0, 0),
            item(&mut e, 0x6200, 0xd1, 0),
            item(&mut e, 0x6300, 0xd2, 0),
        ];
        three_items(&mut e, this, items);
        e.call_log = Some(vec![]);
        e.call(0x0073_14c0, &args![this, 0x00ab_0000u32, 1i32, 2i32]);
        assert_eq!(calls_to(&e, 0x00ab_0000), vec![vec![0x6200, 0xd1]]);
        e.call_log = Some(vec![]);
        e.call(0x0073_14c0, &args![this, 0x00ab_0000u32, 0i32, 9i32]);
        assert_eq!(calls_to(&e, 0x00ab_0000).len(), 3);
    }
}

#[cfg(test)]
mod n051_tests {
    use super::*;

    /// Every callee outside the file the barter functions reach, doubled to do nothing.
    const STUBBED: &[u32] = &[
        ABS_00EC7D40,
        STRUPR_S,
        LIST_BOX_BASE_CONSTRUCT,
        LIST_CONSTRUCT,
        LIST_DESTRUCT,
        LIST_BOX_INIT_FLAGS,
        LIST_BOX_REGISTER,
        LIST_BOX_UNREGISTER,
        FREE,
        NODE_DELETE,
        ITEM_DATA_DELETE,
        LIST_BOX_HAS_FLAG,
        LIST_BOX_BELONGS_TO,
        ADD_USER_TRAIT,
        TEXT_TO_TRAIT,
        TILE_SET_FLOAT,
        TILE_POKE,
        TILE_GET_FLOAT,
        TILE_IS_TRUE,
        TILE_SET_INT,
        TILE_SET_INTEGER_VALUE,
        TILE_GET_MENU_TILE,
        TILE_GET_MENU,
        FTOL2,
        FLOAT_PAIR_0040EBD0,
        FOCUS_OWNER_004B7210,
        SET_CURRENT_FOCUS_TARGET,
        FLAG_004B71D0,
        LIST_SORT,
        INIT_TILE_POSITION,
        ITEM_CHANGE_SYNCHRONIZE,
        ITEM_CHANGE_UPDATE_HIGHLIGHT_BOX_POS,
        ITEM_CHANGE_SET_TOTAL_LIST_HEIGHT,
        ITEM_CHANGE_NUM_UNFILTERED,
        MENU_CONSTRUCT,
        MENU_DESTRUCT,
        LIST_CLEAR,
        CLEAR_BARTER_CONTAINER,
        SET_BARTER_CONTAINER_0046F310,
        TILE_READER,
        TILE_READ_FILE,
        TILE_GET_MENU_BY_CLASS,
        TILE_GET_CHILD_BY_NAME,
        TILE_SET_STRING,
        TILE_GET_STRING,
        TILE_GET_MAXIMUM_DEPTH,
        MENU_SET_MENU_TILE,
        MENU_PREP_FOR_VISIBILITY,
        MENU_START_FADE_OUT,
        TILES_PRESENT_00796F90,
        PRINT_MESSAGE,
        DIALOG_SERVICE_START,
        DIALOG_SERVICE_END,
        SHOW_TUTORIAL,
        LIST_BOX_INITIALIZE,
        SETTING_STRING,
        SETTING_VALUE_ADDRESS,
        BOUND_00647B70,
        BOUND_004A8F20,
        GET_CLAMPED_ACTOR_VALUE,
        GET_BARTER_MULTIPLIERS,
        MANAGER_PLAY_MENU_SOUND,
        PLAY_MENU_SOUND,
        ACTOR_PROCESS,
        UPDATE_PLAYER_3D,
        GET_TOPIC,
        GET_MESSAGE_MENU_RESULT,
        IS_CONSOLE_VISIBLE,
        CONSOLE_PRINT,
        SHOW_MESSAGE_BOX,
        GET_ORIGINAL_REFERENCE,
        FORM_ID_0084E3A0,
        FLOAT_TO_INT_00406D90,
        LIST_SAVE_STATE,
        QUANTITY_MENU_CREATE,
        LIST_COUNT_005AE380,
        ACTOR_GET_GOLD_AMOUNT,
        BARTER_CONTAINER_00740960,
        INVENTORY_GET_GOLD_AMOUNT,
        FORMAT_TEXT,
        COPY_TEXT,
        GET_INVENTORY_CHANGES,
        GET_ITEM_GROUP,
        LIST_REMOVE_ITEM_GROUP,
        LIST_ADD_ITEM,
        LIST_FILL,
        LIST_ITEM_COUNT,
        ITEM_FULL_NAME,
        ITEM_CHANGE_CONSTRUCT,
        ITEM_GET_VALUE,
        OPERATOR_NEW,
        LIST_ADD_005AE3D0,
        LIST_HIGHLIGHTED_007D6E90,
        LIST_SELECTED_007A1910,
        ITEM_CARD_00707E30,
        TILE_SET_ITEM_00707E10,
        ITEM_DESCRIPTION_004BE200,
        AS_BIPED_MODEL,
        BIPED_IS_PLAYABLE,
        RT_DYNAMIC_CAST,
        FORM_TYPE,
        IS_GOLD,
        CAST_TEST_0048CEE0,
        TYPE_28_TEST_0047BCF0,
        TYPE_29_TEST_004C94D0,
        VENDOR_SERVICES_004181E0,
        SERVICE_FLAG_0047EFE0,
        GET_FORM_VALUE,
        GET_EQUIP_TYPE,
        REFERENCE_CONSTRUCT,
        REFERENCE_DESTRUCT,
        REFERENCE_SET_OWNER_00567AD0,
        REFERENCE_SET_GLOBAL,
        REFERENCE_SET_RANK_00567BD0,
        REFERENCE_SET_OBJECT,
        REFERENCE_IS_AN_OWNER,
        EXTRA_GET_OWNER_00418660,
        EXTRA_GET_GLOBAL,
        EXTRA_GET_RANK,
        EXTRA_GET_COUNT,
        EXTRA_REMOVE_00905330,
        HANDLE_ENTRY_POINT,
        VALUE_00408860,
        FLOAT_PAIR_004BD510,
        FLOAT_PAIR_00404010,
        STRING_COMPARE_00469880,
        ITEM_IS_EQUIPPED_004BDDD0,
        MENU_IS_TOP_00702450,
        WHEEL_DIRECTION_0070EC70,
        LIST_GET_TILE_FOR,
        PLAYER_BARTER_008ADED0,
        LIST_IS_EMPTY_008256D0,
        VALUE_00406CE0,
        LIST_APPEND_00905820,
        AUDIO_INSTANCE,
        AUDIO_SOUND_BY_NAME,
        SOUND_HANDLE_PLAY,
        SOUND_HANDLE_DESTRUCT,
        EXTRA_LIST_005D43C0,
        GET_MERCHANT_CONTAINER,
        SET_ACTION_FLAG,
        REMOVE_ITEM_BARTER_CONTAINER,
        MISC_STAT_MOD_VAL,
        CAPS_FORM_004839C0,
        ITEM_CHANGE_DESTRUCT,
        INVENTORY_REMOVE_GOLD,
        GET_OBJECT_COUNT,
        COMPARE_NAME_00408B20,
        REFERENCE_NAME,
        SHOW_ITEM_STATS_00719790,
        STAT_004BE060,
        STAT_004BE180,
    ];

    /// The shared test vtable and the address its slot `offset` calls.
    const VTABLE: u32 = 0x0f00_0000;
    fn slot(offset: u32) -> u32 {
        0x0e00_0000 + offset
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

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn answer(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| value.into_ret());
    }

    /// An engine with the globals mapped, every callee doubled, the list helpers
    /// working on `[item, next]` nodes and `[extras, count, object]` items, and the test
    /// vtable (0x180 slots, each a do-nothing double).
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x011d_8000,
            0x011d_9000,
            0x011d_e000,
            0x0119_f000,
            0x011c_1000,
            0x011c_3000,
            0x0101_2000,
            0x0101_7000,
            0x0101_e000,
            0x0102_3000,
            0x0107_0000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(HUNDRED_DOUBLE, 100.0f64);
        e.set_global(STACKING_TYPE_ONE, 6006.0f64);
        e.set_global(STACKING_TYPE_TWO, 102.0f64);
        e.set_global(MINUS_ONE_FLOAT, -1.0f32);
        e.set_global(TENTH_FLOAT, 0.1f32);
        e.set_global(DIM_ALPHA, 64.0f32);
        e.set_global(FULL_ALPHA, 255.0f32);
        for &addr in STUBBED {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(NODE_SELF, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(NODE_SET_NEXT, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(ITEM_OBJECT, |e, a| {
            if a[0] == 0 {
                Ret::default()
            } else {
                e.mem.u32(a[0] + 8).into_ret()
            }
        });
        let slots: Vec<u32> = (0..0x180).map(|i| slot(4 * i)).collect();
        for &s in &slots {
            e.register(s, |_, _| Ret::default());
        }
        e.put_vtable(VTABLE, &slots);
        e
    }

    /// A barter menu (made the singleton) with the test vtable on it and both lists, a sell
    /// multiplier of 1.0 and the player's process (an object with the test vtable).
    fn menu(e: &mut Engine) -> Ptr<BarterMenu> {
        let this = Ptr::new(e.mem.alloc(0x120));
        let process = e.mem.alloc(4);
        e.mem.set_u32(process, VTABLE);
        answer(e, ACTOR_PROCESS, process);
        e.mem.set_u32(this.addr(), VTABLE);
        e.mem.set_u32(this.addr() + PLAYER_ITEMS, VTABLE);
        e.mem.set_u32(this.addr() + VENDOR_ITEMS, VTABLE);
        e.set(this, BarterMenu::fSellMult, 1.0);
        e.set_global(BARTER_MENU, this.addr());
        this
    }

    /// An item: its extras list, count and object.
    fn item(e: &mut Engine, extras: u32, count: u32, object: u32) -> u32 {
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item, extras);
        e.mem.set_u32(item + 4, count);
        e.mem.set_u32(item + 8, object);
        item
    }

    /// An object with the test vtable.
    fn object(e: &mut Engine) -> u32 {
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, VTABLE);
        object
    }

    #[test]
    fn the_constructor_builds_the_barter_menu() {
        let mut e = engine();
        e.register(TEXT_TO_TRAIT, |_, a| (a[0] & 0xffff).into_ret());
        let this = Ptr::<BarterMenu>::new(e.mem.alloc(0x120));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0072_ceb0, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), BARTER_MENU_VTABLE);
        assert_eq!(e.global::<u32>(TITLE_TRAIT), 0xed20);
        assert_eq!(e.global::<u32>(ARROW_DIRECTION_TRAIT), 0x06a4);
        assert_eq!(e.get(this, BarterMenu::eFilterSound), 3);
        assert_eq!(e.get(this, BarterMenu::fBuyMult), 1.0);
        assert_eq!(e.get(this, BarterMenu::fSellMult), 1.0);
        assert_eq!(
            calls_to(&e, LIST_CONSTRUCT),
            vec![
                vec![this.addr() + 0xac],
                vec![this.addr() + 0xdc],
                vec![this.addr() + 0x10c],
                vec![this.addr() + 0x114]
            ]
        );
        assert_eq!(calls_to(&e, MENU_CONSTRUCT), vec![vec![this.addr()]]);
    }

    #[test]
    fn the_class_is_0x41d() {
        let mut e = engine();
        let this = menu(&mut e);
        assert_eq!(e.call(0x0072_d030, &args![this]).u32(), 0x41d);
        assert_eq!(e.call(0x0072_d6c0, &args![]).u32(), 0x41d);
    }

    #[test]
    fn the_deleting_destructor_frees_only_with_the_flag() {
        let mut e = engine();
        let this = menu(&mut e);
        e.set_global(BARTER_MENU, 0u32);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0072_d070, &args![this, 0u32]).u32(), this.addr());
        assert!(calls_to(&e, FREE).is_empty());
        assert_eq!(calls_to(&e, MENU_DESTRUCT), vec![vec![this.addr()]]);
        e.call_log = Some(vec![]);
        e.call(0x0072_d070, &args![this, 1u32]);
        assert_eq!(calls_to(&e, FREE), vec![vec![this.addr()]]);
    }

    #[test]
    fn the_destructor_deletes_the_transaction_items() {
        let mut e = engine();
        let this = menu(&mut e);
        let bought = item(&mut e, 0, 1, 0);
        e.mem.set_u32(this.addr() + ITEMS_TO_BUY, bought);
        e.call_log = Some(vec![]);
        e.call(0x0072_d0a0, &args![this]);
        assert_eq!(calls_to(&e, ITEM_DATA_DELETE), vec![vec![bought, 1]]);
        assert_eq!(
            calls_to(&e, LIST_CLEAR),
            vec![vec![this.addr() + 0x10c], vec![this.addr() + 0x114]]
        );
        assert_eq!(e.global::<u32>(BARTER_MENU), 0);
        assert_eq!(
            calls_to(&e, LIST_DESTRUCT),
            vec![
                vec![this.addr() + 0x114],
                vec![this.addr() + 0x10c],
                vec![this.addr() + 0xdc],
                vec![this.addr() + 0xac]
            ]
        );
        assert_eq!(calls_to(&e, MENU_DESTRUCT), vec![vec![this.addr()]]);
        // Without a singleton nothing is deleted.
        e.call_log = Some(vec![]);
        e.call(0x0072_d0a0, &args![this]);
        assert!(calls_to(&e, ITEM_DATA_DELETE).is_empty());
    }

    #[test]
    fn create_needs_a_vendor_and_a_menu_of_the_class() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0072_d250, &args![0u32, 0i32]).u32(), 0);
        assert_eq!(
            calls_to(&e, PRINT_MESSAGE),
            vec![vec![0x12, NULL_VENDOR_TEXT]]
        );
        let this = menu(&mut e);
        e.set_global(BARTER_MENU, 0u32);
        answer(&mut e, TILE_READ_FILE, 0x5000);
        answer(&mut e, TILE_GET_MENU, this.addr());
        answer(&mut e, NODE_NEXT, 0x5000);
        // Another class: the menu read is deleted.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0072_d250, &args![0x9000u32, 0i32]).u32(), 0);
        assert_eq!(calls_to(&e, slot(0)), vec![vec![this.addr(), 1]]);
        // The class, without its tiles.
        e.register(slot(0x34), |_, _| 0x41du32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0072_d250, &args![0x9000u32, 0i32]).u32(), 0);
        assert_eq!(
            calls_to(&e, PRINT_MESSAGE),
            vec![vec![CREATION_FAILED_TEXT]]
        );
        // The full start.
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        answer(&mut e, TILES_PRESENT_00796F90, 1);
        e.register(TILE_GET_FLOAT, |_, _| st0(102.0));
        e.register(TILE_GET_MAXIMUM_DEPTH, |_, _| st0(3.0));
        e.register(BOUND_00647B70, |_, a| {
            (a[0] as i32).max(a[1] as i32).into_ret()
        });
        e.register(BOUND_004A8F20, |_, a| {
            (a[0] as i32).min(a[1] as i32).into_ret()
        });
        answer(&mut e, TEXT_TO_TRAIT, 0x31);
        answer(&mut e, TILE_GET_CHILD_BY_NAME, 0x5100);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0072_d250, &args![0x9000u32, 150i32]).u32(), 0x5000);
        assert_eq!(e.get(this, BarterMenu::pVendorRef), 0x9000);
        assert_eq!(e.get(this, BarterMenu::iDiscount), 100);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT)[0],
            vec![0x5000, 0xfad, 3.0f32.to_bits(), 1]
        );
        assert_eq!(
            calls_to(&e, SHOW_TUTORIAL),
            vec![vec![0x4d4, 0x17, 0x41d, 0x200]]
        );
        assert_eq!(
            calls_to(&e, GET_BARTER_MULTIPLIERS),
            vec![vec![0, this.addr() + 0x94, this.addr() + 0x98]]
        );
        assert_eq!(calls_to(&e, MANAGER_PLAY_MENU_SOUND), vec![vec![0x24]]);
        assert_eq!(
            calls_to(&e, MENU_PREP_FOR_VISIBILITY),
            vec![vec![this.addr(), 0]]
        );
        // The lists were filled (`0072dc30` with no form).
        assert_eq!(calls_to(&e, LIST_FILL).len(), 2);
    }

    #[test]
    fn closing_ends_the_service_with_the_traded_topic() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x0072_d6d0, &args![]);
        assert!(calls_to(&e, MENU_START_FADE_OUT).is_empty());
        let this = menu(&mut e);
        e.mem.set_u32(this.addr() + 4, 0x5200);
        e.set(this, BarterMenu::bItemsTraded, 1);
        let process = object(&mut e);
        answer(&mut e, ACTOR_PROCESS, process);
        e.register(slot(0x474), |_, _| 1u32.into_ret());
        answer(&mut e, GET_TOPIC, 0x77);
        e.call_log = Some(vec![]);
        e.call(0x0072_d6d0, &args![]);
        assert_eq!(calls_to(&e, UPDATE_PLAYER_3D).len(), 1);
        assert_eq!(calls_to(&e, GET_TOPIC), vec![vec![5, 4]]);
        assert_eq!(calls_to(&e, DIALOG_SERVICE_END), vec![vec![0x77]]);
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x5200, 0x1772, 1]]);
        assert_eq!(calls_to(&e, MENU_START_FADE_OUT), vec![vec![this.addr()]]);
    }

    #[test]
    fn do_click_steps_filters_leaves_and_picks_items() {
        let mut e = engine();
        let this = menu(&mut e);
        answer(&mut e, ITEM_CHANGE_NUM_UNFILTERED, 1);
        e.call_log = Some(vec![]);
        e.call(0x0072_d770, &args![this, 2i32, 0u32]);
        assert_eq!(e.get(this, BarterMenu::ePlayerFilterMode), 5);
        assert_eq!(calls_to(&e, PLAY_MENU_SOUND), vec![vec![0]]);
        e.call(0x0072_d770, &args![this, 9i32, 0u32]);
        assert_eq!(e.get(this, BarterMenu::eVendorFilterMode), 1);
        // Leaving with nothing pending closes; with items asks first.
        e.call_log = Some(vec![]);
        e.call(0x0072_d770, &args![this, 0x13i32, 0u32]);
        assert_eq!(calls_to(&e, MENU_START_FADE_OUT).len(), 1);
        answer(&mut e, LIST_COUNT_005AE380, 1);
        answer(&mut e, SETTING_STRING, 0x7700);
        e.call_log = Some(vec![]);
        e.call(0x0072_d770, &args![this, 0x13i32, 0u32]);
        let asked = calls_to(&e, SHOW_MESSAGE_BOX);
        assert_eq!(asked.len(), 1);
        assert_eq!(asked[0].len(), 11);
        assert_eq!(asked[0][3], 0x0072_db40);
        assert_eq!(asked[0][5], 0x17);
        // An item with more left than the setting opens the quantity menu.
        let current = item(&mut e, 0, 5, 0);
        e.set_global(BARTER_CURRENT_ITEM, current);
        e.set_global(ITEM_ENTERED_0119FDB0, 1u8);
        e.register(TILE_GET_FLOAT, |_, _| st0(1.0));
        e.register(FLOAT_TO_INT_00406D90, |_, a| {
            (f32::from_bits(a[0]) as i32).into_ret()
        });
        let limit = e.mem.alloc(4);
        e.mem.set_i32(limit, 1);
        answer(&mut e, SETTING_VALUE_ADDRESS, limit);
        e.set(this, BarterMenu::plbCurrListbox, this.addr() + 0xa8);
        e.call_log = Some(vec![]);
        e.call(0x0072_d770, &args![this, 0x17i32, 0x6100u32]);
        assert_eq!(e.global::<u8>(ITEM_ENTERED_0119FDB0), 0);
        assert_eq!(
            calls_to(&e, QUANTITY_MENU_CREATE),
            vec![vec![4, 0x0072_f6f0, 0x7fff_ffff]]
        );
        // Without a tile nothing happens.
        e.call_log = Some(vec![]);
        e.call(0x0072_d770, &args![this, 0x17i32, 0u32]);
        assert!(calls_to(&e, QUANTITY_MENU_CREATE).is_empty());
    }

    #[test]
    fn the_leave_callback_closes_on_the_first_button() {
        let mut e = engine();
        let _this = menu(&mut e);
        answer(&mut e, GET_MESSAGE_MENU_RESULT, 1);
        e.call_log = Some(vec![]);
        e.call(0x0072_db40, &args![]);
        assert!(calls_to(&e, MENU_START_FADE_OUT).is_empty());
        answer(&mut e, GET_MESSAGE_MENU_RESULT, 0x100);
        e.call(0x0072_db40, &args![]);
        assert_eq!(calls_to(&e, MENU_START_FADE_OUT).len(), 1);
    }

    #[test]
    fn caps_labels_show_both_counts() {
        let mut e = engine();
        let this = menu(&mut e);
        e.mem.set_u32(this.addr() + 0x3c, 0x5300);
        e.mem.set_u32(this.addr() + 0x50, 0x5400);
        answer(&mut e, ACTOR_GET_GOLD_AMOUNT, 50);
        answer(&mut e, INVENTORY_GET_GOLD_AMOUNT, 70);
        answer(&mut e, SETTING_STRING, 0x7700);
        e.call_log = Some(vec![]);
        e.call(0x0072_db60, &args![this]);
        assert_eq!(e.get(this, BarterMenu::iPlayerCaps), 50);
        assert_eq!(e.get(this, BarterMenu::iVendorCaps), 70);
        let formats = calls_to(&e, FORMAT_TEXT);
        assert_eq!(formats.len(), 2);
        assert_eq!(formats[0][1..], [0x40, CAPS_FORMAT, 0x7700, 50]);
        assert_eq!(formats[1][4], 70);
        let strings = calls_to(&e, TILE_SET_STRING);
        assert_eq!(strings[0][..2], [0x5300, 0xfc4]);
        assert_eq!(strings[1][..2], [0x5400, 0xfc4]);
    }

    #[test]
    fn the_refresh_adds_kept_items_and_the_transactions() {
        let mut e = engine();
        let this = menu(&mut e);
        let form = object(&mut e);
        let kept = item(&mut e, 0, 1, form);
        answer(&mut e, GET_INVENTORY_CHANGES, 0x8800);
        e.register_double(GET_ITEM_GROUP, move |e, a| {
            e.mem.set_u32(a[2], kept);
            Ret::default()
        });
        answer(&mut e, RT_DYNAMIC_CAST, 0x8900);
        answer(&mut e, CAST_TEST_0048CEE0, 1);
        answer(&mut e, FORM_TYPE, 0x1f);
        let bought = item(&mut e, 0, 2, form);
        e.mem.set_u32(this.addr() + ITEMS_TO_BUY, bought);
        let block = e.mem.alloc(0xc);
        answer(&mut e, OPERATOR_NEW, block);
        answer(&mut e, ITEM_CHANGE_CONSTRUCT, block);
        answer(&mut e, LIST_ADD_ITEM, 0x6600);
        e.set_global(IS_BARTER_SELECTED_TRAIT, 0x44u32);
        e.call_log = Some(vec![]);
        e.call(0x0072_dc30, &args![this, form]);
        assert_eq!(
            calls_to(&e, LIST_REMOVE_ITEM_GROUP),
            vec![vec![this.addr() + 0xa8, form]]
        );
        let added = calls_to(&e, LIST_ADD_ITEM);
        assert_eq!(added.len(), 2);
        assert_eq!(added[0][..2], [this.addr() + 0xa8, kept]);
        assert_eq!(added[1][..2], [this.addr() + 0xa8, block]);
        assert_eq!(
            calls_to(&e, ITEM_CHANGE_CONSTRUCT),
            vec![vec![block, form, 2]]
        );
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x6600, 0x44, 1]]);
        assert_eq!(calls_to(&e, slot(0x20)).len(), 2);
        // No form: the lists are filled; an empty current list switches tabs.
        e.mem.set_u32(this.addr() + ITEMS_TO_BUY, 0);
        e.set(this, BarterMenu::pVendorRef, 0x9000);
        e.set(this, BarterMenu::plbCurrListbox, this.addr() + 0xa8);
        e.call_log = Some(vec![]);
        e.call(0x0072_dc30, &args![this, 0u32]);
        assert_eq!(
            calls_to(&e, LIST_FILL),
            vec![
                vec![this.addr() + 0xa8, 0, 0x0072_e570, 0, 0, 0, 0],
                vec![this.addr() + 0xd8, 0x9000, 0x0072_e6c0, 0, 0, 0, 1]
            ]
        );
        assert_eq!(calls_to(&e, slot(0xc)), vec![vec![this.addr(), 4, 0]]);
        assert_eq!(calls_to(&e, slot(0x10)), vec![vec![this.addr(), 1, 0]]);
        assert_eq!(e.get(this, BarterMenu::eFilterSound), 3);
    }

    #[test]
    fn do_enter_switches_sides_and_shows_the_item() {
        let mut e = engine();
        let this = menu(&mut e);
        e.mem.set_u32(this.addr() + 4, 0x5200);
        e.set_global(IS_CONTAINER_LIST_SELECTED_TRAIT, 0x45u32);
        e.call_log = Some(vec![]);
        e.call(0x0072_e370, &args![this, 1i32, 0u32]);
        assert_eq!(calls_to(&e, slot(0x14)), vec![vec![this.addr(), 0x17, 0]]);
        assert_eq!(calls_to(&e, slot(0)), vec![vec![this.addr() + 0xa8, 0]]);
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x5200, 0x45, 1]]);
        assert_eq!(e.get(this, BarterMenu::plbCurrListbox), this.addr() + 0xd8);
        e.mem.set_u32(this.addr() + 0x7c, 0x5500);
        e.mem.set_u32(this.addr() + 0x78, 0x5600);
        answer(&mut e, LIST_SELECTED_007A1910, 0x7100);
        answer(&mut e, ITEM_DESCRIPTION_004BE200, 0x7200);
        e.call_log = Some(vec![]);
        e.call(0x0072_e370, &args![this, 0x17i32, 0x6100u32]);
        assert_eq!(e.global::<u8>(ITEM_ENTERED_0119FDB0), 1);
        assert_eq!(e.global::<u32>(BARTER_CURRENT_ITEM), 0x7100);
        assert_eq!(calls_to(&e, ITEM_CARD_00707E30), vec![vec![0x7100, 0x5500]]);
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![vec![0x5600, 0xfcc, 0x7200, 1]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0072_e530, &args![this, 0u32, 0u32]);
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![vec![0x5500, 0xfa3, 0], vec![0x5600, 0xfa3, 0]]
        );
    }

    #[test]
    fn the_player_filter_keeps_only_tradeable_objects() {
        let mut e = engine();
        assert!(e.call(0x0072_e570, &args![0u32]).bool());
        let form = object(&mut e);
        let it = item(&mut e, 0, 1, form);
        answer(&mut e, RT_DYNAMIC_CAST, 0x8900);
        answer(&mut e, CAST_TEST_0048CEE0, 1);
        answer(&mut e, FORM_TYPE, 0x1f);
        assert!(!e.call(0x0072_e570, &args![it]).bool());
        answer(&mut e, FORM_TYPE, 0x2e);
        assert!(e.call(0x0072_e570, &args![it]).bool());
        answer(&mut e, FORM_TYPE, 0x28);
        assert!(e.call(0x0072_e570, &args![it]).bool());
        answer(&mut e, TYPE_28_TEST_0047BCF0, 1);
        assert!(!e.call(0x0072_e570, &args![it]).bool());
        answer(&mut e, IS_GOLD, 1);
        assert!(e.call(0x0072_e570, &args![it]).bool());
    }

    #[test]
    fn the_vendor_filter_checks_the_vendors_services() {
        let mut e = engine();
        let this = menu(&mut e);
        let vendor = object(&mut e);
        e.set(this, BarterMenu::pVendorRef, vendor);
        e.register(slot(0x100), |_, _| 1u32.into_ret());
        let form = object(&mut e);
        let it = item(&mut e, 0, 1, form);
        answer(&mut e, RT_DYNAMIC_CAST, 0x8900);
        answer(&mut e, CAST_TEST_0048CEE0, 1);
        answer(&mut e, FORM_TYPE, 0x18);
        answer(&mut e, VENDOR_SERVICES_004181E0, 0x100);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0072_e6c0, &args![it]).bool());
        assert_eq!(calls_to(&e, VENDOR_SERVICES_004181E0), vec![vec![vendor]]);
        assert_eq!(calls_to(&e, SERVICE_FLAG_0047EFE0), vec![vec![0x190, 2]]);
        answer(&mut e, SERVICE_FLAG_0047EFE0, 1);
        assert!(!e.call(0x0072_e6c0, &args![it]).bool());
        answer(&mut e, FORM_TYPE, 0x2f);
        answer(&mut e, GET_EQUIP_TYPE, 11);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0072_e6c0, &args![it]).bool());
        assert_eq!(calls_to(&e, VENDOR_SERVICES_004181E0), vec![vec![vendor]]);
        assert_eq!(calls_to(&e, SERVICE_FLAG_0047EFE0), vec![vec![0x190, 0x40]]);
        answer(&mut e, FORM_TYPE, 0x2e);
        assert!(e.call(0x0072_e6c0, &args![it]).bool());
    }

    #[test]
    fn stolen_entries_owned_by_the_vendor_are_removed() {
        let mut e = engine();
        let this = menu(&mut e);
        let vendor = object(&mut e);
        e.set(this, BarterMenu::pVendorRef, vendor);
        e.register(slot(0x100), |_, _| 1u32.into_ret());
        let extras = e.mem.alloc(8);
        e.mem.set_u32(extras, 0x7300);
        let it = item(&mut e, extras, 5, 0x8000);
        answer(&mut e, EXTRA_GET_OWNER_00418660, 0x7400);
        answer(&mut e, REFERENCE_IS_AN_OWNER, 1);
        answer(&mut e, EXTRA_GET_COUNT, 0xfffe);
        e.register(LIST_ADD_005AE3D0, |e, a| {
            let word = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], word);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0072_eac0, &args![0x6100u32, it]);
        let checks = calls_to(&e, REFERENCE_IS_AN_OWNER);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0][1..], [vendor, 1]);
        assert_eq!(calls_to(&e, REFERENCE_SET_OBJECT)[0][1], 0x8000);
        // The count goes up by the sign-extended 16-bit count -2.
        assert_eq!(e.mem.u32(it + 4), 7);
        assert_eq!(calls_to(&e, EXTRA_REMOVE_00905330).len(), 1);
        // Without a vendor nothing is listed.
        e.register(slot(0x100), |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0072_eac0, &args![0x6100u32, it]);
        assert!(calls_to(&e, REFERENCE_CONSTRUCT).is_empty());
    }

    #[test]
    fn sort_values_take_the_buy_or_sell_side() {
        let mut e = engine();
        let this = menu(&mut e);
        let it = item(&mut e, 0, 1, 0x8000);
        e.register(ITEM_GET_VALUE, |_, _| st0(-3.0));
        assert_eq!(e.call(0x0072_ed00, &args![0x6100u32, it]).f32(), -1.0);
        e.register(ITEM_GET_VALUE, |_, _| st0(10.0));
        e.set(this, BarterMenu::fBuyMult, 2.0);
        e.set(this, BarterMenu::iDiscount, 10);
        e.register(ABS_00EC7D40, |_, a| (a[0] as i32).wrapping_abs().into_ret());
        e.register(VALUE_00408860, |_, a| st0(f64::from(f32::from_bits(a[0]))));
        e.register(FLOAT_PAIR_004BD510, |_, a| {
            st0(f64::from(f32::from_bits(a[0])))
        });
        e.register(FLOAT_PAIR_00404010, |_, a| {
            st0(f64::from(f32::from_bits(a[0])) + 0.5)
        });
        // On the vendor's side and not selected: the buy price, 10 * 2 * 0.9.
        answer(&mut e, LIST_BOX_BELONGS_TO, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0072_ed00, &args![0x6100u32, it]).f32(), 18.5);
        assert_eq!(calls_to(&e, HANDLE_ENTRY_POINT).len(), 1);
        assert_eq!(calls_to(&e, FLOAT_PAIR_004BD510)[0][1], 1.0f32.to_bits());
        assert_eq!(
            calls_to(&e, FLOAT_PAIR_00404010),
            vec![vec![18.0f32.to_bits(), 10.0f32.to_bits()]]
        );
        // Selected on the vendor's side: the sell price, 10 * 0.5 * 1.1.
        e.set(this, BarterMenu::fSellMult, 0.5);
        answer(&mut e, TILE_IS_TRUE, 1);
        e.register(FLOAT_PAIR_0040EBD0, |_, a| {
            st0(f64::from(f32::from_bits(a[0])))
        });
        e.register(VALUE_00408860, |_, _| st0(0.5));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0072_ed00, &args![0x6100u32, it]).f32(), 5.5);
        assert!(calls_to(&e, HANDLE_ENTRY_POINT).is_empty());
        assert_eq!(calls_to(&e, FLOAT_PAIR_004BD510)[0][1], 0.1f32.to_bits());
    }

    #[test]
    fn the_sort_puts_selected_items_first() {
        let mut e = engine();
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(first, 0x6100);
        e.mem.set_u32(second, 0x6200);
        e.register(TILE_IS_TRUE, |_, a| u32::from(a[0] == 0x6100).into_ret());
        assert_eq!(e.call(0x0072_f070, &args![first, second]).i32(), -1);
        assert_eq!(e.call(0x0072_f070, &args![second, first]).i32(), 1);
        e.register(TILE_IS_TRUE, |_, _| Ret::default());
        e.register(TILE_GET_STRING, |_, a| (a[0] + 1).into_ret());
        e.register(STRING_COMPARE_00469880, |_, a| {
            (a[0] as i32 - a[1] as i32).into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0072_f070, &args![first, second]).i32(), -0x100);
        assert_eq!(
            calls_to(&e, TILE_GET_STRING),
            vec![vec![0x6200, 0xfc4], vec![0x6100, 0xfc4]]
        );
    }

    #[test]
    fn the_transact_item_matches_object_and_extra_data() {
        let mut e = engine();
        let this = menu(&mut e);
        let extras_a = e.mem.alloc(8);
        e.mem.set_u32(extras_a, 0x7300);
        let extras_b = e.mem.alloc(8);
        e.mem.set_u32(extras_b, 0x7400);
        let wanted = item(&mut e, extras_b, 1, 0x8000);
        let other = item(&mut e, extras_a, 1, 0x8000);
        let same = item(&mut e, extras_b, 3, 0x8000);
        let list = e.mem.alloc(8);
        let node = e.mem.alloc(8);
        e.mem.set_u32(list, other);
        e.mem.set_u32(list + 4, node);
        e.mem.set_u32(node, same);
        assert_eq!(e.call(0x0072_f120, &args![this, list, wanted]).u32(), same);
        let lone = item(&mut e, 0, 1, 0x8100);
        assert_eq!(e.call(0x0072_f120, &args![this, list, lone]).u32(), 0);
    }

    #[test]
    fn init_func_labels_the_tile() {
        let mut e = engine();
        let _this = menu(&mut e);
        let it = item(&mut e, 0, 3, 0x8000);
        e.register(ITEM_GET_VALUE, |_, _| st0(0.5));
        e.register(FLOAT_PAIR_0040EBD0, |_, a| {
            st0(f64::from(f32::from_bits(a[0])))
        });
        e.register(FLOAT_PAIR_004BD510, |_, a| {
            st0(f64::from(f32::from_bits(a[0])))
        });
        answer(&mut e, ITEM_FULL_NAME, 0x7500);
        answer(&mut e, TEXT_TO_TRAIT, 0x33);
        e.set_global(VALUE_TRAIT, 0x34u32);
        e.call_log = Some(vec![]);
        e.call(0x0072_f210, &args![0x6100u32, it]);
        let formats = calls_to(&e, FORMAT_TEXT);
        assert_eq!(formats[0][1..], [0x80, NAME_COUNT_FORMAT, 0x7500, 3]);
        let half = 0.5f64.to_bits();
        assert_eq!(
            formats[1][1..],
            [0x80, ONE_DECIMAL_FORMAT, half as u32, (half >> 32) as u32]
        );
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x6100, 0x33, 0]]);
        assert_eq!(calls_to(&e, TILE_SET_STRING)[1][..2], [0x6100, 0x34]);
        assert_eq!(
            calls_to(&e, TILE_SET_ITEM_00707E10),
            vec![vec![0x6100, 0x7500]]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x6100, 0xfb0, 0, 1]]
        );
        // A value of at most 0 shows "--".
        e.register(ITEM_GET_VALUE, |_, _| st0(0.0));
        e.call_log = Some(vec![]);
        e.call(0x0072_f210, &args![0x6100u32, it]);
        assert_eq!(calls_to(&e, COPY_TEXT)[0][1..], [0x80, NO_VALUE_TEXT]);
    }

    #[test]
    fn the_gamepad_switches_tabs_and_sides() {
        let mut e = engine();
        let this = menu(&mut e);
        assert!(!e.call(0x0072_f4b0, &args![this, 3i32, 0u32]).bool());
        answer(&mut e, MENU_IS_TOP_00702450, 1);
        e.set(this, BarterMenu::plbCurrListbox, this.addr() + 0xa8);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0072_f4b0, &args![this, 3i32, 0u32]).bool());
        assert_eq!(calls_to(&e, slot(0xc)), vec![vec![this.addr(), 4, 0]]);
        assert_eq!(
            calls_to(&e, LIST_SAVE_STATE),
            vec![vec![this.addr() + 0xa8]]
        );
        e.set(this, BarterMenu::plbCurrListbox, 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0072_f4b0, &args![this, 4i32, 0u32]).bool());
        assert_eq!(calls_to(&e, slot(0xc)), vec![vec![this.addr(), 7, 0]]);
        assert!(!e.call(0x0072_f4b0, &args![this, 0xei32, 0u32]).bool());
        answer(&mut e, LIST_ITEM_COUNT, 2);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0072_f4b0, &args![this, 0xei32, 0u32]).bool());
        assert_eq!(calls_to(&e, slot(0x10)), vec![vec![this.addr(), 1, 0]]);
        assert_eq!(
            calls_to(&e, LIST_SAVE_STATE),
            vec![vec![this.addr() + 0xa8]]
        );
        assert!(!e.call(0x0072_f4b0, &args![this, 5i32, 0u32]).bool());
    }

    #[test]
    fn the_wheel_clicks_a_filter_tab() {
        let mut e = engine();
        let this = menu(&mut e);
        e.register(WHEEL_DIRECTION_0070EC70, |_, _| (-1i32).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0072_f680, &args![this, 8i32, 0u32]);
        e.call(0x0072_f680, &args![this, 5i32, 0u32]);
        assert_eq!(calls_to(&e, slot(0xc)), vec![vec![this.addr(), 7, 0]]);
        answer(&mut e, WHEEL_DIRECTION_0070EC70, 1);
        e.call_log = Some(vec![]);
        e.call(0x0072_f680, &args![this, 3i32, 0u32]);
        assert_eq!(calls_to(&e, slot(0xc)), vec![vec![this.addr(), 4, 0]]);
    }

    #[test]
    fn selecting_an_item_moves_the_amount_and_shows_it() {
        let mut e = engine();
        let this = menu(&mut e);
        e.set(this, BarterMenu::plbCurrListbox, this.addr() + 0xa8);
        answer(&mut e, ACTOR_GET_GOLD_AMOUNT, 100);
        answer(&mut e, INVENTORY_GET_GOLD_AMOUNT, 1000);
        let it = item(&mut e, 0, 4, 0x8000);
        e.set_global(BARTER_CURRENT_ITEM, it);
        answer(&mut e, LIST_GET_TILE_FOR, 0x6100);
        e.register(ITEM_GET_VALUE, |_, _| st0(3.0));
        e.register(FLOAT_PAIR_0040EBD0, |_, a| {
            st0(f64::from(f32::from_bits(a[0])))
        });
        e.register(FLOAT_PAIR_004BD510, |_, a| {
            st0(f64::from(f32::from_bits(a[0])))
        });
        answer(&mut e, LIST_IS_EMPTY_008256D0, 1);
        let block = e.mem.alloc(0xc);
        answer(&mut e, OPERATOR_NEW, block);
        answer(&mut e, ITEM_CHANGE_CONSTRUCT, block);
        e.mem.set_u32(this.addr() + 0x5c, 0x5500);
        e.mem.set_u32(this.addr() + 0x58, 0x5600);
        e.call_log = Some(vec![]);
        e.call(0x0072_f6f0, &args![2i32]);
        assert_eq!(e.get(this, BarterMenu::fTransactionAmount), 6.0);
        assert_eq!(
            calls_to(&e, PLAYER_BARTER_008ADED0),
            vec![vec![0, 0x8000, 0, 0]]
        );
        assert_eq!(calls_to(&e, LIST_APPEND_00905820).len(), 1);
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![vec![0x5500, 0xfaf, 0], vec![0x5600, 0xfa3, 0]]
        );
        // With transactions the amount shows: 6 + 6, affordable, below the vendor's caps.
        answer(&mut e, LIST_IS_EMPTY_008256D0, 0);
        e.register(VALUE_00406CE0, |_, a| st0(f64::from(f32::from_bits(a[0]))));
        e.register(FLOAT_TO_INT_00406D90, |_, a| {
            (f32::from_bits(a[0]) as i32).into_ret()
        });
        e.register(BOUND_004A8F20, |_, a| {
            (a[0] as i32).min(a[1] as i32).into_ret()
        });
        e.register(ABS_00EC7D40, |_, a| (a[0] as i32).wrapping_abs().into_ret());
        e.set_global(ARROW_DIRECTION_TRAIT, 0x46u32);
        e.mem.set_u32(this.addr() + 4, 0x5200);
        e.call_log = Some(vec![]);
        e.call(0x0072_f6f0, &args![2i32]);
        assert_eq!(e.get(this, BarterMenu::fTransactionAmount), 12.0);
        let ints = calls_to(&e, TILE_SET_INT);
        assert_eq!(ints[0], vec![0x5200, 0x46, 2]);
        assert_eq!(ints[2], vec![0x5500, 0xfaf, 1]);
        assert_eq!(
            calls_to(&e, FORMAT_TEXT).last().unwrap()[1..],
            [0x10, INTEGER_FORMAT, 12]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x5600, 0xfa9, 255.0f32.to_bits(), 1]]
        );
        // A count of 0 does nothing.
        e.call_log = Some(vec![]);
        e.call(0x0072_f6f0, &args![0i32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn adding_a_transaction_item_grows_or_creates_the_entry() {
        let mut e = engine();
        let this = menu(&mut e);
        e.set(this, BarterMenu::plbCurrListbox, this.addr() + 0xa8);
        let it = item(&mut e, 0, 4, 0x8000);
        e.set_global(BARTER_CURRENT_ITEM, it);
        let entry = item(&mut e, 0, 2, 0x8000);
        e.mem.set_u32(this.addr() + ITEMS_TO_SELL, entry);
        e.call(0x0072_faa0, &args![this, 3i32]);
        assert_eq!(e.mem.u32(entry + 4), 5);
        e.mem.set_u32(this.addr() + ITEMS_TO_SELL, 0);
        answer(&mut e, OPERATOR_NEW, 0x7600);
        answer(&mut e, ITEM_CHANGE_CONSTRUCT, 0x7600);
        e.call_log = Some(vec![]);
        e.call(0x0072_faa0, &args![this, 3i32]);
        assert_eq!(
            calls_to(&e, ITEM_CHANGE_CONSTRUCT),
            vec![vec![0x7600, 0x8000, 3]]
        );
        assert_eq!(
            calls_to(&e, LIST_APPEND_00905820)[0][0],
            this.addr() + ITEMS_TO_SELL
        );
    }

    #[test]
    fn removing_a_transaction_item_lowers_or_deletes_the_entry() {
        let mut e = engine();
        let this = menu(&mut e);
        e.set(this, BarterMenu::plbCurrListbox, this.addr() + 0xa8);
        let it = item(&mut e, 0, 4, 0x8000);
        e.set_global(BARTER_CURRENT_ITEM, it);
        let entry = item(&mut e, 0, 5, 0x8000);
        e.mem.set_u32(this.addr() + ITEMS_TO_BUY, entry);
        e.call(0x0072_fc30, &args![this, 2i32]);
        assert_eq!(e.mem.u32(entry + 4), 3);
        e.call_log = Some(vec![]);
        e.call(0x0072_fc30, &args![this, 3i32]);
        assert_eq!(
            calls_to(&e, EXTRA_REMOVE_00905330)[0][0],
            this.addr() + ITEMS_TO_BUY
        );
        assert_eq!(calls_to(&e, ITEM_DATA_DELETE), vec![vec![entry, 1]]);
    }

    #[test]
    fn completing_moves_the_items_and_the_caps() {
        let mut e = engine();
        let this = menu(&mut e);
        e.set(this, BarterMenu::pVendorRef, 0x9000);
        e.set(this, BarterMenu::iVendorCaps, 40);
        e.set(this, BarterMenu::fTransactionAmount, 25.0);
        e.set_global(SECURITY_COOKIE, 0x1234_5678u32);
        let process = object(&mut e);
        answer(&mut e, ACTOR_PROCESS, process);
        let bought = item(&mut e, 0, 2, 0x8000);
        e.mem.set_u32(this.addr() + ITEMS_TO_BUY, bought);
        let player = object(&mut e);
        e.set_global(PLAYER, player);
        let sold = item(&mut e, 0, 1, 0x8100);
        e.mem.set_u32(this.addr() + ITEMS_TO_SELL, sold);
        e.register(VALUE_00406CE0, |_, a| st0(f64::from(f32::from_bits(a[0]))));
        e.register(FLOAT_TO_INT_00406D90, |_, a| {
            (f32::from_bits(a[0]) as i32).into_ret()
        });
        e.register(BOUND_004A8F20, |_, a| {
            (a[0] as i32).min(a[1] as i32).into_ret()
        });
        e.register(ABS_00EC7D40, |_, a| (a[0] as i32).wrapping_abs().into_ret());
        answer(&mut e, GET_INVENTORY_CHANGES, 0x8800);
        answer(&mut e, CAPS_FORM_004839C0, 0x0f0f);
        answer(&mut e, AUDIO_SOUND_BY_NAME, 0x7700);
        e.call_log = Some(vec![]);
        e.call(0x0072_fd10, &args![this]);
        let removed = calls_to(&e, REMOVE_ITEM_BARTER_CONTAINER);
        assert_eq!(removed[0], vec![0, bought, 2, 0x9000]);
        assert_eq!(removed[1][2..], [25, 0x9000]);
        assert_eq!(
            calls_to(&e, slot(0x17c)),
            vec![vec![player, 0x8100, 0, 1, 0, 0, 0x9000, 0, 0, 1, 0]]
        );
        assert_eq!(calls_to(&e, MISC_STAT_MOD_VAL), vec![vec![0x27, 25]]);
        assert_eq!(calls_to(&e, AUDIO_SOUND_BY_NAME)[0][2], CAPS_UP_SOUND);
        let played = calls_to(&e, SOUND_HANDLE_PLAY);
        assert_eq!(played.len(), 1);
        assert_eq!(played[0], vec![0x7700, 0]);
        assert_eq!(calls_to(&e, AUDIO_SOUND_BY_NAME)[0].len(), 4);
        assert_eq!(e.get(this, BarterMenu::fTransactionAmount), 0.0);
        assert_eq!(e.get(this, BarterMenu::bItemsTraded), 1);
        assert_eq!(calls_to(&e, ITEM_DATA_DELETE).len(), 2);
        // A negative amount takes the player's gold.
        e.set(this, BarterMenu::fTransactionAmount, -7.0);
        e.call_log = Some(vec![]);
        e.call(0x0072_fd10, &args![this]);
        assert_eq!(
            calls_to(&e, INVENTORY_REMOVE_GOLD),
            vec![vec![0x8800, player, 7, 0x9000]]
        );
        assert_eq!(calls_to(&e, AUDIO_SOUND_BY_NAME)[0][2], CAPS_DOWN_SOUND);
    }

    #[test]
    fn filter_mode_titles_come_from_the_settings() {
        let mut e = engine();
        let this = menu(&mut e);
        e.mem.set_u32(this.addr() + 0x34, 0x5300);
        e.set(this, BarterMenu::ePlayerFilterMode, 2);
        e.register(SETTING_STRING, |_, a| (a[0] & 0xffff).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0073_01f0, &args![this, 0x5300u32]);
        assert_eq!(calls_to(&e, COPY_TEXT)[0][1..], [0x104, 0x4d50]);
        assert_eq!(calls_to(&e, STRUPR_S).len(), 1);
        assert_eq!(calls_to(&e, TILE_SET_STRING)[0][..2], [0x5300, 0xfc4]);
        // The vendor's tab in mode 0: the vendor's name, never upper-cased.
        answer(&mut e, REFERENCE_NAME, 0x7800);
        e.call_log = Some(vec![]);
        e.call(0x0073_01f0, &args![this, 0x5400u32]);
        assert_eq!(calls_to(&e, COPY_TEXT)[0][1..], [0x104, 0x7800]);
        assert!(calls_to(&e, STRUPR_S).is_empty());
    }

    #[test]
    fn filter_modes_pick_item_types() {
        let mut e = engine();
        let this = menu(&mut e);
        let it = item(&mut e, 0, 2, 0x8000);
        answer(&mut e, LIST_GET_TILE_FOR, 0x6100);
        e.set(this, BarterMenu::ePlayerFilterMode, 1);
        answer(&mut e, FORM_TYPE, 0x18);
        assert!(e.call(0x0073_04b0, &args![it]).bool());
        answer(&mut e, FORM_TYPE, 0x28);
        e.register(TILE_GET_FLOAT, |_, _| st0(1.0));
        assert!(!e.call(0x0073_04b0, &args![it]).bool());
        e.register(TILE_GET_FLOAT, |_, _| st0(2.0));
        assert!(e.call(0x0073_04b0, &args![it]).bool());
        e.set(this, BarterMenu::ePlayerFilterMode, 4);
        assert!(e.call(0x0073_04b0, &args![it]).bool());
        // An item of no list keeps the vendor's mode and no tile.
        answer(&mut e, LIST_GET_TILE_FOR, 0);
        e.set(this, BarterMenu::eVendorFilterMode, 0);
        assert!(!e.call(0x0073_04b0, &args![it]).bool());
    }

    #[test]
    fn refresh_requests_wait_for_do_idle() {
        let mut e = engine();
        let this = menu(&mut e);
        e.call(0x0073_0690, &args![0u32]);
        assert_eq!(e.global::<u8>(REFRESH_REQUESTED_011D8FC0), 1);
        e.call_log = Some(vec![]);
        e.call(0x0073_06d0, &args![this]);
        assert_eq!(e.global::<u8>(REFRESH_REQUESTED_011D8FC0), 0);
        assert_eq!(calls_to(&e, LIST_FILL).len(), 2);
    }

    #[test]
    fn do_idle_shows_the_entered_items_stats() {
        let mut e = engine();
        let this = menu(&mut e);
        for offset in [0x60u32, 0x64, 0x68, 0x6c, 0x70] {
            e.mem.set_u32(this.addr() + offset, offset);
        }
        let it = item(&mut e, 0, 1, 0x8000);
        e.set_global(BARTER_CURRENT_ITEM, it);
        e.set_global(ITEM_ENTERED_0119FDB0, 1u8);
        answer(&mut e, FORM_TYPE, 0x28);
        e.call_log = Some(vec![]);
        e.call(0x0073_06d0, &args![this]);
        assert_eq!(
            calls_to(&e, SHOW_ITEM_STATS_00719790),
            vec![vec![0x60, 0x64, 0x68, it, 0]]
        );
        answer(&mut e, FORM_TYPE, 0x18);
        e.register(STAT_004BE060, |_, _| st0(2.0));
        e.register(STAT_004BE180, |_, _| st0(0.0));
        e.call_log = Some(vec![]);
        e.call(0x0073_06d0, &args![this]);
        assert_eq!(
            calls_to(&e, SHOW_ITEM_STATS_00719790),
            vec![vec![0x6c, 0, 0, it, 0]]
        );
        e.set_global(ITEM_ENTERED_0119FDB0, 0u8);
        e.call_log = Some(vec![]);
        e.call(0x0073_06d0, &args![this]);
        assert!(calls_to(&e, SHOW_ITEM_STATS_00719790).is_empty());
    }

    #[test]
    fn the_highlight_target_skips_disabled_items() {
        let mut e = engine();
        let list = Ptr::<ListBox>::new(e.mem.alloc(0x30));
        e.mem.set_u32(list.addr(), VTABLE);
        e.set(list, ListBox::pContainerRect, Ptr::new(0x3000));
        e.set(list, ListBox::usNextIndex, 3);
        // Tiles 0x6100, 0x6200, 0x6300 at indices 0, 1, 2.
        let tiles = [0x6100u32, 0x6200, 0x6300];
        let items = tiles.map(|tile| {
            let it = e.mem.alloc(0xc);
            e.mem.set_u32(it, tile);
            it
        });
        let node2 = e.mem.alloc(8);
        let node3 = e.mem.alloc(8);
        e.mem.set_u32(list.addr() + 4, items[0]);
        e.mem.set_u32(list.addr() + 8, node2);
        e.mem.set_u32(node2, items[1]);
        e.mem.set_u32(node2 + 4, node3);
        e.mem.set_u32(node3, items[2]);
        e.register(TILE_GET_FLOAT, |_, a| {
            st0(f64::from((a[0].wrapping_sub(0x6100)) / 0x100))
        });
        e.register(FTOL2, |_, a| {
            (f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32).into_ret()
        });
        // Every tile is enabled but 0x6200.
        e.register(TILE_IS_TRUE, |_, a| u32::from(a[0] != 0x6200).into_ret());
        e.set(list, ListBox::pHighlightedItem, Ptr::new(0x6100));
        assert_eq!(e.call(0x0073_0f40, &args![list, 2u32]).u32(), 0x6300);
        e.set(list, ListBox::pHighlightedItem, Ptr::new(0x6300));
        assert_eq!(e.call(0x0073_0f40, &args![list, 1u32]).u32(), 0x6100);
        assert_eq!(e.call(0x0073_0f40, &args![list, 2u32]).u32(), 0);
        assert_eq!(e.call(0x0073_0f40, &args![list, 5u32]).u32(), 0);
        // A page down by the container's extent over its scroll delta.
        e.register(FLOAT_PAIR_004BD510, |_, a| {
            st0(f64::from(f32::from_bits(a[0])))
        });
        e.set(list, ListBox::pHighlightedItem, Ptr::NULL);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0073_0f40, &args![list, 0x10u32]).u32(), 0x6100);
        assert_eq!(calls_to(&e, FLOAT_PAIR_004BD510)[0][1], 1.0f32.to_bits());
    }

    #[test]
    fn the_list_deleting_destructor_and_last_tile() {
        let mut e = engine();
        let list = Ptr::<ListBox>::new(e.mem.alloc(0x30));
        e.mem.set_u32(list.addr(), VTABLE);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0073_1580, &args![list, 1u32]).u32(), list.addr());
        assert_eq!(calls_to(&e, FREE), vec![vec![list.addr()]]);
        assert_eq!(calls_to(&e, LIST_BOX_UNREGISTER), vec![vec![list.addr()]]);
        e.mem.set_u32(list.addr(), VTABLE);
        answer(&mut e, ITEM_CHANGE_NUM_UNFILTERED, 4);
        e.register(slot(0x14), |_, a| (a[1] + 0x100).into_ret());
        assert_eq!(e.call(0x0073_15b0, &args![list]).u32(), 0x103);
    }
}
