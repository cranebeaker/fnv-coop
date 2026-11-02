//! `fallout/interface/listbox.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The `ListBox<ItemChange *>` instance (the inventory-style lists). A list
//! box is a vtable, an embedded `BSSimpleList` of `ListBoxItem *` (first
//! node at +4) and its tiles; the layout is the shared one of
//! `units/fallout_interface/startmenu.rs`. A `ListBoxItem` is the item's
//! tile (+0), its object (+4) and a filtered flag (byte +8). The list's
//! nodes are read through the accessors `006815c0` (address of the node's
//! item) and `00726070` (the next node), as the exe does.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_interface::startmenu::ListBox;

/// The node accessor `006815c0` (`ECX` = node): the address of its item.
const NODE_ITEM: u32 = 0x0068_15c0;
/// The node accessor `00726070` (`ECX` = node): the next node.
const NODE_NEXT: u32 = 0x0072_6070;
/// `Tile::GetMenu` (`ECX` = tile).
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// `Menu::RenderTemplate` (`ECX` = menu; parent tile, template name, 0).
const MENU_RENDER_TEMPLATE: u32 = 0x00a1_ddb0;
/// `Tile::GetValue` (`ECX` = tile; trait): the trait's `Tile::Value *`.
const TILE_GET_VALUE: u32 = 0x00a0_0f30;
/// `Tile::GetValueQ` (`ECX` = tile; trait).
const TILE_GET_VALUE_Q: u32 = 0x00a0_0e90;
/// The tile's float value of a trait (`00a011b0`, `ECX` = tile; trait),
/// in `ST0`.
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
/// The tile's float setter (`00a012d0`, `ECX` = tile; trait, float, 1).
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
/// The tile's integer setter (`00700320`, `ECX` = tile; trait, value).
const TILE_SET_INT: u32 = 0x0070_0320;
/// The tile's string setter (`00a01350`, `ECX` = tile; trait, string, 1).
const TILE_SET_STRING: u32 = 0x00a0_1350;
/// `Tile::AddUserTrait` (cdecl: name, -1): the trait id of a name.
const TILE_ADD_USER_TRAIT: u32 = 0x00a0_0940;
/// `Tile::TextToTrait` (cdecl: name).
const TILE_TEXT_TO_TRAIT: u32 = 0x00a0_1860;
/// `Tile::GetChildByName` (`ECX` = tile; name).
const TILE_GET_CHILD_BY_NAME: u32 = 0x00a0_3da0;
/// `Tile::AddNeedsUpdate` (`ECX` = tile; flags).
const TILE_ADD_NEEDS_UPDATE: u32 = 0x00a0_7530;
/// The search for a named tile under a tile (`00a08b20`, cdecl: tile,
/// name).
const TILE_FIND: u32 = 0x00a0_8b20;
/// `Tile::Value::ClearActions` (`ECX` = value).
const VALUE_CLEAR_ACTIONS: u32 = 0x00a0_9200;
/// `Tile::Value::AddAction` (`ECX` = value; operator, float).
const VALUE_ADD_ACTION: u32 = 0x00a0_9080;
/// `Tile::Value::AddAction` with a tile and trait (`00a09130`, `ECX` =
/// value; operator, tile, trait).
const VALUE_ADD_ACTION_TILE: u32 = 0x00a0_9130;
/// `Tile::Value::CalculateValue` (`ECX` = value; 0).
const VALUE_CALCULATE: u32 = 0x00a0_9410;
/// The game's allocator entry `00401000` (cdecl: size).
const ALLOCATE: u32 = 0x0040_1000;
/// `operator delete` (`00401030`, cdecl).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The `ListBoxItem` constructor `007a62e0` (`this`; tile, address of the
/// object pointer).
const LIST_BOX_ITEM_CONSTRUCT: u32 = 0x007a_62e0;
/// The list's insert after a node (`007a7eb0`, `ECX` = list; item, node).
const LIST_INSERT_AFTER: u32 = 0x007a_7eb0;
/// The list's append of the value at an address (`00905820`, `ECX` =
/// list; address).
const LIST_APPEND: u32 = 0x0090_5820;
/// The list box's flag test (`007242c0`, `this`; flag number).
const LIST_BOX_FLAG: u32 = 0x0072_42c0;
/// `ListBox::InitTilePosition` (`007269d0`, `this`; tile).
const LIST_BOX_INIT_TILE_POSITION: u32 = 0x0072_69d0;
/// The list box's item count (`00726c40`, `this` only).
const LIST_BOX_COUNT: u32 = 0x0072_6c40;
/// The empty test of a `BSSimpleList` (`008256d0`, `ECX` = list).
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// The node's next setter (`006ecd40`, `ECX` = node; next).
const NODE_SET_NEXT: u32 = 0x006e_cd40;
/// The node's deleting destructor (`004702f0`, `ECX` = node; 1).
const NODE_DELETE: u32 = 0x0047_02f0;
/// The method `RemoveItem` runs on the removed item's object with 1
/// (`004459e0`).
const OBJECT_RELEASE: u32 = 0x0044_59e0;
/// The group of an item's object (`0044ddc0`, `ECX` = object).
const OBJECT_GROUP: u32 = 0x0044_ddc0;
/// The local list's constructor, append-by-address, remove-all and
/// destructor (`0096a2d0`, `005ae3d0`, `00470470`, `0046ffb0`).
const LOCAL_LIST_CONSTRUCT: u32 = 0x0096_a2d0;
const LOCAL_LIST_APPEND: u32 = 0x005a_e3d0;
const LOCAL_LIST_REMOVE_ALL: u32 = 0x0047_0470;
const LOCAL_LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// The function returning the interface manager (`004b7210`).
const INTERFACE_MANAGER: u32 = 0x004b_7210;
/// `InterfaceManager::SetCurrentFocusTarget` (`ECX` = manager; tile,
/// trait, 1).
const SET_CURRENT_FOCUS_TARGET: u32 = 0x0071_5860;
/// The float function `SetTotalListHeight` applies to the scroll share
/// (`004e4430`, cdecl: float), result in `ST0`.
const SCROLL_STEPS: u32 = 0x004e_4430;
/// `_ftol2_sse`: truncates `ST0` (passed as an `f64` argument).
const FTOL: u32 = 0x00ec_62c0;
/// The two-integer function `00647b70` (cdecl) `SetTotalListHeight` gives
/// the step count and 2.
const STEP_LIMIT: u32 = 0x0064_7b70;

/// The `double` 0.0 and 1.0 in the exe's data.
const ZERO: u32 = 0x0101_2060;
const ONE: u32 = 0x0101_2070;

/// Trait strings.
const TEXT_NUMBER_OF_VISIBLE_ITEMS: u32 = 0x0106_f860;
const TEXT_NUM_FILTERED: u32 = 0x0106_f8d0;
const TEXT_HIGHLIGHT_Y: u32 = 0x0106_f8c0;
const TEXT_SELECTED_HEIGHT: u32 = 0x0106_f8ac;
const TEXT_SCROLL_DELTA: u32 = 0x0106_f90c;
const TEXT_NUMBER_OF_ITEMS: u32 = 0x0106_f8f8;
/// Tile names.
const TEXT_LB_HIGHLIGHT_BOX: u32 = 0x0106_f898;
const TEXT_TOP: u32 = 0x0106_f894;
const TEXT_BOTTOM: u32 = 0x0106_f88c;
const TEXT_LEFT: u32 = 0x0106_f884;
const TEXT_RIGHT: u32 = 0x0106_f87c;

/// `UpdateHighlightBoxPos`'s function-local statics: the guard bits and
/// the two trait ids.
const HIGHLIGHT_GUARD: u32 = 0x011d_8cd4;
const HIGHLIGHT_Y_TRAIT: u32 = 0x011d_8cd0;
const SELECTED_HEIGHT_TRAIT: u32 = 0x011d_8ccc;
/// `NumUnfiltered`'s function-local static and its guard.
const NUM_FILTERED_GUARD: u32 = 0x011d_8cdc;
const NUM_FILTERED_TRAIT: u32 = 0x011d_8cd8;

/// Trait ids passed as numbers.
const TRAIT_Y: u32 = 0xfa2;
const TRAIT_ID: u32 = 0xfaa;
const TRAIT_LIST_INDEX: u32 = 0xfac;
const TRAIT_HEIGHT: u32 = 0xfb0;
const TRAIT_FOCUS: u32 = 0xfc3;
const TRAIT_STRING: u32 = 0xfc4;
/// Action operators (`Tile::Value` actions): copy (2000) and add (2003).
const ACTION_COPY: u32 = 2000;
const ACTION_ADD: u32 = 0x7d3;

fn item_slot(e: &mut Engine, node: u32) -> u32 {
    e.call(NODE_ITEM, &args![node]).u32()
}

fn next_node(e: &mut Engine, node: u32) -> u32 {
    e.call(NODE_NEXT, &args![node]).u32()
}

/// The list's first node: `this + 4`, or null for a null `this`.
fn first_node(this: Ptr<ListBox>) -> u32 {
    if this.is_null() {
        0
    } else {
        this.addr().wrapping_add(4)
    }
}

// Translated from 0071a200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<ItemChange *>::RemoveItemGroup` (Xbox PDB): collects, in a
/// local list, the items whose object's group (`0044ddc0`) is `group`,
/// then removes each one's tile (`RemoveItem`), and empties and destroys
/// the local list.
// Translation incomplete: 0071a200 - SEH exception frame (FS:[0]) that destroys the local list on unwinding is not modelled.
pub fn list_box_remove_item_group(e: &mut Engine, this: Ptr<ListBox>, group: u32) {
    e.with_stack(8, |e, local| {
        e.call(LOCAL_LIST_CONSTRUCT, &args![local]);
        let mut node = first_node(this);
        while node != 0 {
            let slot = item_slot(e, node);
            if e.mem.u32(slot) == 0 {
                break;
            }
            let slot = item_slot(e, node);
            let item = e.mem.u32(slot);
            if e.mem.u32(item.wrapping_add(4)) != 0 {
                let slot = item_slot(e, node);
                let item = e.mem.u32(slot);
                let object = e.mem.u32(item.wrapping_add(4));
                if e.call(OBJECT_GROUP, &args![object]).u32() == group {
                    let slot = item_slot(e, node);
                    e.call(LOCAL_LIST_APPEND, &args![local, slot]);
                }
            }
            node = next_node(e, node);
        }
        let mut node = local.addr();
        while node != 0 {
            let slot = item_slot(e, node);
            if e.mem.u32(slot) == 0 {
                break;
            }
            let slot = item_slot(e, node);
            let tile = e.mem.u32(e.mem.u32(slot));
            list_box_remove_item(e, this, Ptr::new(tile));
            node = next_node(e, node);
        }
        e.call(LOCAL_LIST_REMOVE_ALL, &args![local]);
        e.call(LOCAL_LIST_DESTRUCT, &args![local]);
    });
}

// Translated from 0071a320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<ItemChange *>::AddItem` (Xbox PDB): renders the template
/// (`template`, or the list's `pTemplateName`) into the container tile,
/// gives the new tile an id (-1) when it has none and the string `text`
/// when given, wraps `object` in a new `ListBoxItem` and inserts it after
/// node `after` (then synchronizes when flag 0 is set) or appends it (then
/// positions the tile and recomputes the height when flag 0 is set, and
/// numbers the tile with the next index). With exactly one item and a
/// positive `_number_of_visible_items`, the container's height becomes the
/// tile's height times that number. Returns the new tile, or null without
/// a container or template.
// Translation incomplete: 0071a320 - SEH exception frame (FS:[0]) that frees the ListBoxItem if its constructor throws is not modelled.
pub fn list_box_add_item(
    e: &mut Engine,
    this: Ptr<ListBox>,
    object: u32,
    text: Ptr,
    after: u32,
    template: Ptr,
) -> Ptr {
    let container = e.get(this, ListBox::pContainerRect);
    if container.is_null() || (e.get(this, ListBox::pTemplateName).is_null() && template.is_null())
    {
        return Ptr::NULL;
    }
    let template = if template.is_null() {
        e.get(this, ListBox::pTemplateName)
    } else {
        template
    };
    let container = e.get(this, ListBox::pContainerRect);
    let menu = e.call(TILE_GET_MENU, &args![container]).u32();
    let tile = e
        .call(
            MENU_RENDER_TEMPLATE,
            &args![menu, container, template, 0u32],
        )
        .u32();
    if e.call(TILE_GET_VALUE, &args![tile, TRAIT_ID]).u32() == 0 {
        e.call(TILE_SET_INT, &args![tile, TRAIT_ID, 0xffff_ffffu32]);
    }
    if !text.is_null() {
        e.call(TILE_SET_STRING, &args![tile, TRAIT_STRING, text, 1u32]);
    }
    // The constructor gets the address of the `object` stack argument.
    e.with_stack(4, |e, object_slot| {
        e.mem.set_u32(object_slot.addr(), object);
        let list = this.addr().wrapping_add(4);
        if after != 0 {
            let block = e.call(ALLOCATE, &args![0xcu32]).u32();
            let item = if block != 0 {
                e.call(LIST_BOX_ITEM_CONSTRUCT, &args![block, tile, object_slot])
                    .u32()
            } else {
                0
            };
            e.call(LIST_INSERT_AFTER, &args![list, item, after]);
            if e.call(LIST_BOX_FLAG, &args![this, 0u32]).bool() {
                list_box_synchronize(e, this);
            }
        } else {
            let block = e.call(ALLOCATE, &args![0xcu32]).u32();
            let item = if block != 0 {
                e.call(LIST_BOX_ITEM_CONSTRUCT, &args![block, tile, object_slot])
                    .u32()
            } else {
                0
            };
            e.with_stack(4, |e, item_slot_on_stack| {
                e.mem.set_u32(item_slot_on_stack.addr(), item);
                e.call(LIST_APPEND, &args![list, item_slot_on_stack]);
            });
            if e.call(LIST_BOX_FLAG, &args![this, 0u32]).bool() {
                e.call(LIST_BOX_INIT_TILE_POSITION, &args![this, tile]);
                list_box_set_total_list_height(e, this);
            }
            let index = u32::from(e.get(this, ListBox::usNextIndex));
            let next = e.get(this, ListBox::usNextIndex).wrapping_add(1);
            e.set(this, ListBox::usNextIndex, next);
            e.call(TILE_SET_INT, &args![tile, TRAIT_LIST_INDEX, index]);
        }
    });
    if e.call(LIST_BOX_COUNT, &args![this]).u32() == 1 {
        let visible_trait = e
            .call(
                TILE_ADD_USER_TRAIT,
                &args![TEXT_NUMBER_OF_VISIBLE_ITEMS, 0xffff_ffffu32],
            )
            .u32();
        let container = e.get(this, ListBox::pContainerRect);
        let visible = e
            .call(TILE_GET_FLOAT, &args![container, visible_trait])
            .f64();
        // `TEST AH,0x41; JNZ`: on only when strictly above 0.0.
        if visible > e.global::<f64>(ZERO) {
            let container = e.get(this, ListBox::pContainerRect);
            let value = e
                .call(TILE_GET_VALUE_Q, &args![container, TRAIT_HEIGHT])
                .u32();
            e.call(VALUE_CLEAR_ACTIONS, &args![value]);
            e.call(
                VALUE_ADD_ACTION_TILE,
                &args![value, ACTION_COPY, tile, TRAIT_HEIGHT],
            );
            let visible_trait = e
                .call(
                    TILE_ADD_USER_TRAIT,
                    &args![TEXT_NUMBER_OF_VISIBLE_ITEMS, 0xffff_ffffu32],
                )
                .u32();
            let container = e.get(this, ListBox::pContainerRect);
            let visible = e
                .call(TILE_GET_FLOAT, &args![container, visible_trait])
                .f64();
            let visible = x87_store32(visible);
            e.call(VALUE_ADD_ACTION, &args![value, ACTION_ADD, visible]);
            e.call(VALUE_CALCULATE, &args![value, 0u32]);
        }
    }
    Ptr::new(tile)
}

// Translated from 0071a5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<ItemChange *>::RemoveItem` (Xbox PDB): finds the node whose
/// item's tile is `tile` (and the node before it) and removes it
/// (`0071aaa0`). False for a null `tile` or an empty list.
pub fn list_box_remove_item(e: &mut Engine, this: Ptr<ListBox>, tile: Ptr) -> bool {
    if tile.is_null() {
        return false;
    }
    let list = this.addr().wrapping_add(4);
    if e.call(LIST_IS_EMPTY, &args![list]).bool() {
        return false;
    }
    let mut node = first_node(this);
    let mut previous = first_node(this);
    while node != 0 {
        let slot = item_slot(e, node);
        if e.mem.u32(e.mem.u32(slot)) == tile.addr() {
            break;
        }
        previous = node;
        node = next_node(e, node);
    }
    fn_0071aaa0(e, this, node, previous)
}

// Translated from 0071a670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<ItemChange *>::Synchronize` (Xbox PDB): restarts the index
/// count and the total height, numbers and positions the tile of every
/// unfiltered item (byte +8 clear) in list order (stopping at an empty
/// node), then recomputes the height and the highlight box.
pub fn list_box_synchronize(e: &mut Engine, this: Ptr<ListBox>) {
    e.set(this, ListBox::usNextIndex, 0);
    e.set(this, ListBox::fTotalListHeight, 0.0);
    let mut node = first_node(this);
    while node != 0 {
        let slot = item_slot(e, node);
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = item_slot(e, node);
        let item = e.mem.u32(slot);
        if e.mem.u8(item.wrapping_add(8)) == 0 {
            let index = u32::from(e.get(this, ListBox::usNextIndex));
            let next = e.get(this, ListBox::usNextIndex).wrapping_add(1);
            e.set(this, ListBox::usNextIndex, next);
            let slot = item_slot(e, node);
            let tile = e.mem.u32(e.mem.u32(slot));
            e.call(TILE_SET_INT, &args![tile, TRAIT_LIST_INDEX, index]);
            let slot = item_slot(e, node);
            let tile = e.mem.u32(e.mem.u32(slot));
            e.call(LIST_BOX_INIT_TILE_POSITION, &args![this, tile]);
        }
        node = next_node(e, node);
    }
    list_box_set_total_list_height(e, this);
    list_box_update_highlight_box_pos(e, this);
}

/// Sets the +0x35 byte (`fn_0071aa60`) of the highlight box's four edge
/// tiles (only when `top` exists) and of the box, and marks each for
/// update (flag 8), in the exe's order.
fn mark_highlight_box(e: &mut Engine, container: Ptr, mark: u8) {
    let boxed = e
        .call(TILE_FIND, &args![container, TEXT_LB_HIGHLIGHT_BOX])
        .u32();
    if boxed == 0 {
        return;
    }
    let top = e
        .call(TILE_GET_CHILD_BY_NAME, &args![boxed, TEXT_TOP])
        .u32();
    if top != 0 {
        fn_0071aa60(e, Ptr::new(top), mark);
        e.call(TILE_ADD_NEEDS_UPDATE, &args![top, 8u32]);
        for name in [TEXT_BOTTOM, TEXT_LEFT, TEXT_RIGHT] {
            let edge = e.call(TILE_GET_CHILD_BY_NAME, &args![boxed, name]).u32();
            fn_0071aa60(e, Ptr::new(edge), mark);
            e.call(TILE_ADD_NEEDS_UPDATE, &args![edge, 8u32]);
        }
    }
    fn_0071aa60(e, Ptr::new(boxed), mark);
    e.call(TILE_ADD_NEEDS_UPDATE, &args![boxed, 8u32]);
}

// Translated from 0071a740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<ItemChange *>::UpdateHighlightBoxPos` (Xbox PDB): with a
/// container, either makes its `_highlight_y` and `_selected_height` copy
/// the highlighted tile's y (0xFA2) and height (0xFB0) and marks the
/// highlight box's tiles with the highlighted tile's +0x35 byte (as 1 or
/// 0), or, without a highlighted tile, sets them to -1 and 0. The two
/// trait ids are function-local statics made on first use.
// Translation incomplete: 0071a740 - SEH exception frame (FS:[0]) around the statics' initialisation is not modelled.
pub fn list_box_update_highlight_box_pos(e: &mut Engine, this: Ptr<ListBox>) {
    if e.global::<u32>(HIGHLIGHT_GUARD) & 1 == 0 {
        let guard = e.global::<u32>(HIGHLIGHT_GUARD) | 1;
        e.set_global(HIGHLIGHT_GUARD, guard);
        let id = e
            .call(
                TILE_ADD_USER_TRAIT,
                &args![TEXT_HIGHLIGHT_Y, 0xffff_ffffu32],
            )
            .u32();
        e.set_global(HIGHLIGHT_Y_TRAIT, id);
    }
    if e.global::<u32>(HIGHLIGHT_GUARD) & 2 == 0 {
        let guard = e.global::<u32>(HIGHLIGHT_GUARD) | 2;
        e.set_global(HIGHLIGHT_GUARD, guard);
        let id = e
            .call(
                TILE_ADD_USER_TRAIT,
                &args![TEXT_SELECTED_HEIGHT, 0xffff_ffffu32],
            )
            .u32();
        e.set_global(SELECTED_HEIGHT_TRAIT, id);
    }
    if e.get(this, ListBox::pContainerRect).is_null() {
        return;
    }
    if e.get(this, ListBox::pHighlightedItem).is_null() {
        let container = e.get(this, ListBox::pContainerRect);
        let id = e.global::<u32>(HIGHLIGHT_Y_TRAIT);
        e.call(TILE_SET_INT, &args![container, id, 0xffff_ffffu32]);
        let container = e.get(this, ListBox::pContainerRect);
        let id = e.global::<u32>(SELECTED_HEIGHT_TRAIT);
        e.call(TILE_SET_INT, &args![container, id, 0u32]);
        return;
    }
    let container = e.get(this, ListBox::pContainerRect);
    let id = e.global::<u32>(HIGHLIGHT_Y_TRAIT);
    let y = e.call(TILE_GET_VALUE, &args![container, id]).u32();
    let container = e.get(this, ListBox::pContainerRect);
    let id = e.global::<u32>(SELECTED_HEIGHT_TRAIT);
    let height = e.call(TILE_GET_VALUE, &args![container, id]).u32();
    let highlighted = e.get(this, ListBox::pHighlightedItem);
    e.call(
        VALUE_ADD_ACTION_TILE,
        &args![y, ACTION_COPY, highlighted, TRAIT_Y],
    );
    e.call(VALUE_CALCULATE, &args![y, 0u32]);
    let highlighted = e.get(this, ListBox::pHighlightedItem);
    e.call(
        VALUE_ADD_ACTION_TILE,
        &args![height, ACTION_COPY, highlighted, TRAIT_HEIGHT],
    );
    e.call(VALUE_CALCULATE, &args![height, 0u32]);
    let highlighted = e.get(this, ListBox::pHighlightedItem);
    let container = e.get(this, ListBox::pContainerRect);
    if fn_0071aa80(e, highlighted) != 0 {
        mark_highlight_box(e, container, 1);
    } else {
        mark_highlight_box(e, container, 0);
    }
}

// Translated from 0071aa60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the byte at +0x35 of a tile (the highlight box's
/// mark; the field is not named here).
pub fn fn_0071aa60(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr().wrapping_add(0x35), value);
}

// Translated from 0071aa80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at +0x35 of a tile (see `fn_0071aa60`), in `AL`.
pub fn fn_0071aa80(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr().wrapping_add(0x35))
}

// Translated from 0071aaa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `node` (whose predecessor is `previous`) from the list: moves
/// the highlight to the next item's tile (or the previous one's, or none)
/// when the removed tile was highlighted and focuses it; lowers the
/// container's `_num_filtered` for a filtered item; deletes the tile
/// (its virtual slot 0 with 1) and, when flag 1 is set, runs `004459e0`
/// on the item's object; frees the `ListBoxItem`; unlinks the node (the
/// embedded first node takes the second's item) and deletes it; then
/// synchronizes when flag 0 is set. False for a null `node`.
pub fn fn_0071aaa0(e: &mut Engine, this: Ptr<ListBox>, node: u32, previous: u32) -> bool {
    if node == 0 {
        return false;
    }
    let slot = item_slot(e, node);
    let item = e.mem.u32(slot);
    if e.mem.u32(item) == e.get(this, ListBox::pHighlightedItem).addr() {
        let new_highlight = if next_node(e, node) != 0 {
            let next = next_node(e, node);
            let slot = item_slot(e, next);
            e.mem.u32(e.mem.u32(slot))
        } else if previous != 0 && previous != node {
            let slot = item_slot(e, previous);
            e.mem.u32(e.mem.u32(slot))
        } else {
            0
        };
        e.set(this, ListBox::pHighlightedItem, Ptr::new(new_highlight));
        if !e.get(this, ListBox::pHighlightedItem).is_null() {
            let highlighted = e.get(this, ListBox::pHighlightedItem);
            let manager = e.call(INTERFACE_MANAGER, &[]).u32();
            e.call(
                SET_CURRENT_FOCUS_TARGET,
                &args![manager, highlighted, TRAIT_FOCUS, 1u32],
            );
        }
    }
    if !e.get(this, ListBox::pContainerRect).is_null() {
        let slot = item_slot(e, node);
        let item = e.mem.u32(slot);
        if e.mem.u8(item.wrapping_add(8)) != 0 {
            let id = e.call(TILE_TEXT_TO_TRAIT, &args![TEXT_NUM_FILTERED]).u32();
            let container = e.get(this, ListBox::pContainerRect);
            let filtered = e.call(TILE_GET_FLOAT, &args![container, id]).f64();
            let lowered = x87_store32(x87_sub(filtered, e.global::<f64>(ONE)));
            let id = e.call(TILE_TEXT_TO_TRAIT, &args![TEXT_NUM_FILTERED]).u32();
            let container = e.get(this, ListBox::pContainerRect);
            e.call(TILE_SET_FLOAT, &args![container, id, lowered, 1u32]);
        }
    }
    let slot = item_slot(e, node);
    let tile = e.mem.u32(e.mem.u32(slot));
    if tile != 0 {
        e.vcall(tile, 0x0, &args![1u32]);
    }
    if e.call(LIST_BOX_FLAG, &args![this, 1u32]).bool() {
        let slot = item_slot(e, node);
        let object = e.mem.u32(e.mem.u32(slot).wrapping_add(4));
        if object != 0 {
            e.call(OBJECT_RELEASE, &args![object, 1u32]);
        }
    }
    let slot = item_slot(e, node);
    let item = e.mem.u32(slot);
    e.call(OPERATOR_DELETE, &args![item]);
    if node == first_node(this) {
        let second = e.mem.u32(this.addr().wrapping_add(8));
        if second != 0 {
            let after = next_node(e, second);
            e.mem.set_u32(this.addr().wrapping_add(8), after);
            let slot = item_slot(e, second);
            let moved = e.mem.u32(slot);
            e.mem.set_u32(this.addr().wrapping_add(4), moved);
            e.call(NODE_SET_NEXT, &args![second, 0u32]);
            if second != 0 {
                e.call(NODE_DELETE, &args![second, 1u32]);
            }
        } else {
            e.mem.set_u32(this.addr().wrapping_add(4), 0);
        }
    } else {
        let after = next_node(e, node);
        e.call(NODE_SET_NEXT, &args![previous, after]);
        e.call(NODE_SET_NEXT, &args![node, 0u32]);
        if node != 0 {
            e.call(NODE_DELETE, &args![node, 1u32]);
        }
    }
    if e.call(LIST_BOX_FLAG, &args![this, 0u32]).bool() {
        list_box_synchronize(e, this);
    }
    true
}

// Translated from 0071ad30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<ItemChange *>::SetTotalListHeight` (Xbox PDB): with a
/// container whose height (0xFB0) is positive and below a positive total
/// height, the scroll steps are `00647b70(trunc(004e4430(unfiltered *
/// (total - height) / total)) + 1, 2)` (else 1); with more than one step
/// the container's `_scroll_delta` becomes `(total - height) / (steps -
/// 1)`. The scroll bar's `_number_of_items` gets the step count. (The 2 is
/// the word `NumUnfiltered`'s caller pushes and that function leaves on the
/// stack; `00647b70`'s `ADD ESP,8` takes it as its second argument.)
pub fn list_box_set_total_list_height(e: &mut Engine, this: Ptr<ListBox>) {
    let container = e.get(this, ListBox::pContainerRect);
    if container.is_null() {
        return;
    }
    let height = e
        .call(TILE_GET_FLOAT, &args![container, TRAIT_HEIGHT])
        .f64();
    let height = x87_store32(height);
    let mut steps = 1i32;
    let total = e.get(this, ListBox::fTotalListHeight).to_bits();
    let zero = e.global::<f64>(ZERO);
    if x87_m32(total) > zero && x87_m32(height) > zero {
        let total = e.get(this, ListBox::fTotalListHeight).to_bits();
        // `FCOMPP; TEST AH,5; JP`: on only when `height < total`.
        if x87_m32(height) < x87_m32(total) {
            let unfiltered = list_box_num_unfiltered(e, this);
            let count = f64::from(unfiltered as u32);
            let total = e.get(this, ListBox::fTotalListHeight).to_bits();
            let difference = x87_sub(x87_m32(total), x87_m32(height));
            let product = x87_mul(count, difference);
            let total = e.get(this, ListBox::fTotalListHeight).to_bits();
            let share = x87_store32(x87_div(product, x87_m32(total)));
            let rounded = e.call(SCROLL_STEPS, &args![share]).f64();
            let whole = e.call(FTOL, &args![rounded]).u32();
            steps = e
                .call(STEP_LIMIT, &args![whole.wrapping_add(1), 2u32])
                .i32();
        }
        if steps > 1 {
            let total = e.get(this, ListBox::fTotalListHeight).to_bits();
            let difference = x87_sub(x87_m32(total), x87_m32(height));
            let delta = x87_store32(x87_div(difference, f64::from(steps.wrapping_sub(1))));
            let id = e.call(TILE_TEXT_TO_TRAIT, &args![TEXT_SCROLL_DELTA]).u32();
            let container = e.get(this, ListBox::pContainerRect);
            e.call(TILE_SET_FLOAT, &args![container, id, delta, 1u32]);
        }
    }
    let scroll_bar = e.get(this, ListBox::pScrollBar);
    if !scroll_bar.is_null() {
        let id = e
            .call(TILE_TEXT_TO_TRAIT, &args![TEXT_NUMBER_OF_ITEMS])
            .u32();
        let scroll_bar = e.get(this, ListBox::pScrollBar);
        e.call(TILE_SET_INT, &args![scroll_bar, id, steps]);
    }
}

// Translated from 0071ae60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<ItemChange *>::NumUnfiltered` (Xbox PDB): the item count
/// (`00726c40`) minus the container's `_num_filtered`, truncated (`FISTP
/// qword` with the rounding control set to chop; the low word). The trait
/// id is a function-local static made on first use. Takes `this` only
/// (plain `RET`).
// Translation incomplete: 0071ae60 - SEH exception frame (FS:[0]) around the static's initialisation is not modelled.
pub fn list_box_num_unfiltered(e: &mut Engine, this: Ptr<ListBox>) -> i32 {
    if e.global::<u32>(NUM_FILTERED_GUARD) & 1 == 0 {
        let guard = e.global::<u32>(NUM_FILTERED_GUARD) | 1;
        e.set_global(NUM_FILTERED_GUARD, guard);
        let id = e
            .call(
                TILE_ADD_USER_TRAIT,
                &args![TEXT_NUM_FILTERED, 0xffff_ffffu32],
            )
            .u32();
        e.set_global(NUM_FILTERED_TRAIT, id);
    }
    let count = e.call(LIST_BOX_COUNT, &args![this]).u32();
    let id = e.global::<u32>(NUM_FILTERED_TRAIT);
    let container = e.get(this, ListBox::pContainerRect);
    let filtered = e.call(TILE_GET_FLOAT, &args![container, id]).f64();
    let filtered = x87_fistp64_chop(X87Ext::load64(filtered)) as u32;
    count.wrapping_sub(filtered) as i32
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0071_a200, list_box_remove_item_group(Ptr<ListBox>, u32)),
        entry!(
            0x0071_a320,
            list_box_add_item(Ptr<ListBox>, u32, Ptr, u32, Ptr) -> Ptr
        ),
        entry!(0x0071_a5c0, list_box_remove_item(Ptr<ListBox>, Ptr) -> bool),
        entry!(0x0071_a670, list_box_synchronize(Ptr<ListBox>)),
        entry!(0x0071_a740, list_box_update_highlight_box_pos(Ptr<ListBox>)),
        entry!(0x0071_aa60, fn_0071aa60(Ptr, u8)),
        entry!(0x0071_aa80, fn_0071aa80(Ptr) -> u8),
        entry!(0x0071_aaa0, fn_0071aaa0(Ptr<ListBox>, u32, u32) -> bool),
        entry!(0x0071_ad30, list_box_set_total_list_height(Ptr<ListBox>)),
        entry!(0x0071_ae60, list_box_num_unfiltered(Ptr<ListBox>) -> i32),
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

    const TILE_VTABLE: u32 = 0x0094_0000;
    const TILE_DELETE: u32 = 0x0094_1000;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// Doubles for the list's node accessors and helpers: a node's item is
    /// its first word and its next the second; flags are off.
    fn list_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011d_8000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.set_global(ZERO, 0.0f64);
        e.set_global(ONE, 1.0f64);
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(NODE_SET_NEXT, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(NODE_DELETE, |_, _| Ret::default());
        e.register(LIST_IS_EMPTY, |e, a| ret(u32::from(e.mem.u32(a[0]) == 0)));
        e.register(LIST_BOX_FLAG, |_, _| ret(0));
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.register(TILE_DELETE, |_, _| Ret::default());
        e.put_vtable(TILE_VTABLE, &[TILE_DELETE]);
        e.register(OBJECT_GROUP, |e, a| ret(e.mem.u32(a[0])));
        e.register(OBJECT_RELEASE, |_, _| Ret::default());
        e.register(TILE_ADD_USER_TRAIT, |_, a| ret(0x2000 + (a[0] & 0xfff)));
        e.register(TILE_TEXT_TO_TRAIT, |_, a| ret(0x3000 + (a[0] & 0xfff)));
        e.register(TILE_SET_INT, |_, _| Ret::default());
        e.register(TILE_SET_FLOAT, |_, _| Ret::default());
        e.register(LIST_BOX_INIT_TILE_POSITION, |_, _| Ret::default());
        e.register(INTERFACE_MANAGER, |_, _| ret(0x0095_0000));
        e.register(SET_CURRENT_FOCUS_TARGET, |_, _| Ret::default());
        e
    }

    /// A `ListBoxItem` with a fresh tile (with a vtable), `object` and the
    /// filtered byte; returns (item, tile).
    fn new_item(e: &mut Engine, object: u32, filtered: u8) -> (u32, u32) {
        let tile = e.mem.alloc(0x40);
        e.mem.set_u32(tile, TILE_VTABLE);
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item, tile);
        e.mem.set_u32(item + 4, object);
        e.mem.set_u8(item + 8, filtered);
        (item, tile)
    }

    /// Builds the list from `items` (first in the embedded node at +4).
    fn fill(e: &mut Engine, list: Ptr<ListBox>, items: &[u32]) -> Vec<u32> {
        let mut nodes = vec![list.addr() + 4];
        for _ in 1..items.len() {
            nodes.push(e.mem.alloc(8));
        }
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(nodes[i], *item);
            let next = nodes.get(i + 1).copied().unwrap_or(0);
            e.mem.set_u32(nodes[i] + 4, next);
        }
        nodes
    }

    fn object_with_group(e: &mut Engine, group: u32) -> u32 {
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, group);
        object
    }

    #[test]
    fn remove_item_group_removes_the_matching_items() {
        let mut e = list_engine();
        e.register(LOCAL_LIST_CONSTRUCT, |_, _| Ret::default());
        e.register(LOCAL_LIST_APPEND, |e, a| {
            let value = e.mem.u32(a[1]);
            let mut node = a[0];
            if e.mem.u32(node) == 0 {
                e.mem.set_u32(node, value);
                return Ret::default();
            }
            while e.mem.u32(node + 4) != 0 {
                node = e.mem.u32(node + 4);
            }
            let fresh = e.mem.alloc(8);
            e.mem.set_u32(fresh, value);
            e.mem.set_u32(node + 4, fresh);
            Ret::default()
        });
        e.register(LOCAL_LIST_REMOVE_ALL, |_, _| Ret::default());
        e.register(LOCAL_LIST_DESTRUCT, |_, _| Ret::default());
        let list = e.new_object::<ListBox>();
        let objects: Vec<u32> = [1, 2, 1]
            .iter()
            .map(|g| object_with_group(&mut e, *g))
            .collect();
        let (a, ta) = new_item(&mut e, objects[0], 0);
        let (b, _) = new_item(&mut e, objects[1], 0);
        let (c, tc) = new_item(&mut e, objects[2], 0);
        fill(&mut e, list, &[a, b, c]);
        e.call_log = Some(vec![]);
        e.call(0x0071_a200, &args![list, 1u32]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![a], vec![c]]);
        assert_eq!(calls_to(&e, TILE_DELETE), vec![vec![ta, 1], vec![tc, 1]]);
        assert_eq!(e.mem.u32(list.addr() + 4), b);
        assert_eq!(e.mem.u32(list.addr() + 8), 0);
        assert_eq!(calls_to(&e, LOCAL_LIST_REMOVE_ALL).len(), 1);
        assert_eq!(calls_to(&e, LOCAL_LIST_DESTRUCT).len(), 1);
    }

    /// Doubles for `AddItem`'s tile calls; the rendered tile is returned
    /// by the template double and the visible-item count is 3.
    fn add_item_engine(rendered: u32) -> Engine {
        let mut e = list_engine();
        e.register(TILE_GET_MENU, |_, _| ret(0x0096_0000));
        e.register_double(MENU_RENDER_TEMPLATE, move |_, _| ret(rendered));
        e.register(TILE_GET_VALUE, |_, _| ret(0));
        e.register(TILE_SET_STRING, |_, _| Ret::default());
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(LIST_BOX_ITEM_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            let object = e.mem.u32(a[2]);
            e.mem.set_u32(a[0] + 4, object);
            ret(a[0])
        });
        e.register(LIST_APPEND, |_, _| Ret::default());
        e.register(LIST_INSERT_AFTER, |_, _| Ret::default());
        e.register(LIST_BOX_COUNT, |_, _| ret(1));
        e.register(TILE_GET_FLOAT, |_, _| Ret {
            st0: 3.0,
            ..Ret::default()
        });
        e.register(TILE_GET_VALUE_Q, |_, _| ret(0x0097_0000));
        e.register(VALUE_CLEAR_ACTIONS, |_, _| Ret::default());
        e.register(VALUE_ADD_ACTION_TILE, |_, _| Ret::default());
        e.register(VALUE_ADD_ACTION, |_, _| Ret::default());
        e.register(VALUE_CALCULATE, |_, _| Ret::default());
        e
    }

    #[test]
    fn add_item_appends_and_numbers_the_tile() {
        let mut e = add_item_engine(0x0098_0000);
        let list = e.new_object::<ListBox>();
        e.set(list, ListBox::pContainerRect, Ptr::new(0x0099_0000));
        e.set(list, ListBox::pTemplateName, Ptr::new(0x0099_1000));
        e.set(list, ListBox::usNextIndex, 4);
        e.call_log = Some(vec![]);
        let words = args![list, 0x77u32, 0x0099_2000u32, 0u32, 0u32];
        let tile = e.call(0x0071_a320, &words);
        assert_eq!(tile.u32(), 0x0098_0000);
        assert_eq!(
            calls_to(&e, MENU_RENDER_TEMPLATE),
            vec![vec![0x0096_0000, 0x0099_0000, 0x0099_1000, 0]]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![
                vec![0x0098_0000, TRAIT_ID, 0xffff_ffff],
                vec![0x0098_0000, TRAIT_LIST_INDEX, 4]
            ]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![vec![0x0098_0000, TRAIT_STRING, 0x0099_2000, 1]]
        );
        let append = calls_to(&e, LIST_APPEND);
        assert_eq!(append.len(), 1);
        assert_eq!(append[0][0], list.addr() + 4);
        assert_eq!(e.get(list, ListBox::usNextIndex), 5);
        assert_eq!(
            calls_to(&e, VALUE_ADD_ACTION_TILE),
            vec![vec![0x0097_0000, ACTION_COPY, 0x0098_0000, TRAIT_HEIGHT]]
        );
        assert_eq!(
            calls_to(&e, VALUE_ADD_ACTION),
            vec![vec![0x0097_0000, ACTION_ADD, 3.0f32.to_bits()]]
        );
        // The item got the tile and the object through its address.
        let item = calls_to(&e, LIST_BOX_ITEM_CONSTRUCT)[0][0];
        assert_eq!(e.mem.u32(item), 0x0098_0000);
        assert_eq!(e.mem.u32(item + 4), 0x77);
    }

    #[test]
    fn add_item_inserts_after_a_node_or_refuses() {
        let mut e = add_item_engine(0x0098_0000);
        let list = e.new_object::<ListBox>();
        e.set(list, ListBox::pContainerRect, Ptr::new(0x0099_0000));
        e.call_log = Some(vec![]);
        // No template anywhere: null, nothing rendered.
        let words = args![list, 1u32, 0u32, 0u32, 0u32];
        assert_eq!(e.call(0x0071_a320, &words).u32(), 0);
        assert!(calls_to(&e, MENU_RENDER_TEMPLATE).is_empty());
        let words = args![list, 1u32, 0u32, 0x0099_3000u32, 0x0099_4000u32];
        let tile = e.call(0x0071_a320, &words);
        assert_eq!(tile.u32(), 0x0098_0000);
        assert_eq!(calls_to(&e, MENU_RENDER_TEMPLATE)[0][2], 0x0099_4000);
        let insert = calls_to(&e, LIST_INSERT_AFTER);
        assert_eq!(insert.len(), 1);
        assert_eq!(insert[0][0], list.addr() + 4);
        assert_eq!(insert[0][2], 0x0099_3000);
        assert!(calls_to(&e, LIST_APPEND).is_empty());
        assert_eq!(e.get(list, ListBox::usNextIndex), 0);
    }

    #[test]
    fn remove_item_refuses_null_tiles_and_empty_lists() {
        let mut e = list_engine();
        let list = e.new_object::<ListBox>();
        assert!(!e.call(0x0071_a5c0, &args![list, 0u32]).bool());
        assert!(!e.call(0x0071_a5c0, &args![list, 0x1234u32]).bool());
        let (a, _) = new_item(&mut e, 0, 0);
        let (b, tb) = new_item(&mut e, 0, 0);
        let nodes = fill(&mut e, list, &[a, b]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0071_a5c0, &args![list, tb]).bool());
        assert_eq!(calls_to(&e, NODE_DELETE), vec![vec![nodes[1], 1]]);
        assert_eq!(e.mem.u32(list.addr() + 8), 0);
        assert_eq!(e.mem.u32(list.addr() + 4), a);
        // A tile that is not in the list: the walk ends on a null node.
        assert!(!list_box_remove_item(&mut e, list, Ptr::new(0x0055_0000)));
        assert_eq!(e.mem.u32(list.addr() + 4), a);
    }

    #[test]
    fn synchronize_numbers_the_unfiltered_items() {
        let mut e = list_engine();
        let list = e.new_object::<ListBox>();
        e.set(list, ListBox::usNextIndex, 9);
        e.set(list, ListBox::fTotalListHeight, 5.0);
        let (a, ta) = new_item(&mut e, 0, 0);
        let (b, _) = new_item(&mut e, 0, 1);
        let (c, tc) = new_item(&mut e, 0, 0);
        fill(&mut e, list, &[a, b, c]);
        e.call_log = Some(vec![]);
        e.call(0x0071_a670, &args![list]);
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![vec![ta, TRAIT_LIST_INDEX, 0], vec![tc, TRAIT_LIST_INDEX, 1]]
        );
        assert_eq!(
            calls_to(&e, LIST_BOX_INIT_TILE_POSITION),
            vec![vec![list.addr(), ta], vec![list.addr(), tc]]
        );
        assert_eq!(e.get(list, ListBox::usNextIndex), 2);
        assert_eq!(e.get(list, ListBox::fTotalListHeight), 0.0);
        // The highlight statics were made on the way.
        assert_eq!(e.global::<u32>(HIGHLIGHT_GUARD), 3);
    }

    #[test]
    fn highlight_box_follows_the_highlighted_tile() {
        let mut e = list_engine();
        e.register(TILE_GET_VALUE, |_, a| ret(0x0097_0000 + (a[1] & 0xfff)));
        e.register(VALUE_ADD_ACTION_TILE, |_, _| Ret::default());
        e.register(VALUE_CALCULATE, |_, _| Ret::default());
        e.register(TILE_ADD_NEEDS_UPDATE, |_, _| Ret::default());
        let boxed = e.mem.alloc(0x40);
        let children: Vec<u32> = (0..4).map(|_| e.mem.alloc(0x40)).collect();
        let kids = children.clone();
        e.register_double(TILE_FIND, move |_, _| ret(boxed));
        e.register_double(TILE_GET_CHILD_BY_NAME, move |_, a| {
            let index = match a[1] {
                TEXT_TOP => 0,
                TEXT_BOTTOM => 1,
                TEXT_LEFT => 2,
                _ => 3,
            };
            ret(kids[index])
        });
        let list = e.new_object::<ListBox>();
        let highlighted = e.mem.alloc(0x40);
        e.mem.set_u8(highlighted + 0x35, 5);
        e.set(list, ListBox::pContainerRect, Ptr::new(0x0099_0000));
        e.set(list, ListBox::pHighlightedItem, Ptr::new(highlighted));
        e.call_log = Some(vec![]);
        e.call(0x0071_a740, &args![list]);
        let y_trait = e.global::<u32>(HIGHLIGHT_Y_TRAIT);
        let height_trait = e.global::<u32>(SELECTED_HEIGHT_TRAIT);
        let y_value = 0x0097_0000 + (y_trait & 0xfff);
        let height_value = 0x0097_0000 + (height_trait & 0xfff);
        assert_eq!(
            calls_to(&e, VALUE_ADD_ACTION_TILE),
            vec![
                vec![y_value, ACTION_COPY, highlighted, TRAIT_Y],
                vec![height_value, ACTION_COPY, highlighted, TRAIT_HEIGHT]
            ]
        );
        for tile in children.iter().chain([boxed].iter()) {
            assert_eq!(e.mem.u8(tile + 0x35), 1);
        }
        assert_eq!(calls_to(&e, TILE_ADD_NEEDS_UPDATE).len(), 5);
        // A highlighted tile with a zero byte marks them 0.
        e.mem.set_u8(highlighted + 0x35, 0);
        list_box_update_highlight_box_pos(&mut e, list);
        assert_eq!(e.mem.u8(boxed + 0x35), 0);
        assert_eq!(calls_to(&e, TILE_ADD_USER_TRAIT).len(), 2);
        // No highlighted tile: the traits are reset.
        e.set(list, ListBox::pHighlightedItem, Ptr::NULL);
        e.call_log = Some(vec![]);
        list_box_update_highlight_box_pos(&mut e, list);
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![
                vec![0x0099_0000, y_trait, 0xffff_ffff],
                vec![0x0099_0000, height_trait, 0]
            ]
        );
    }

    #[test]
    fn mark_byte_round_trip() {
        let mut e = Engine::new();
        let tile = e.mem.alloc(0x40);
        e.call(0x0071_aa60, &args![tile, 0x1ffu32]);
        assert_eq!(e.mem.u8(tile + 0x35), 0xff);
        assert_eq!(e.call(0x0071_aa80, &args![tile]).u8(), 0xff);
    }

    #[test]
    fn removing_the_highlighted_filtered_head() {
        let mut e = list_engine();
        e.register(LIST_BOX_FLAG, |_, a| ret(u32::from(a[1] == 1)));
        e.register(TILE_GET_FLOAT, |_, _| Ret {
            st0: 4.0,
            ..Ret::default()
        });
        let list = e.new_object::<ListBox>();
        let (a, ta) = new_item(&mut e, 0x0055_0000, 1);
        let (b, tb) = new_item(&mut e, 0, 0);
        let nodes = fill(&mut e, list, &[a, b]);
        e.set(list, ListBox::pHighlightedItem, Ptr::new(ta));
        e.set(list, ListBox::pContainerRect, Ptr::new(0x0099_0000));
        assert!(!e.call(0x0071_aaa0, &args![list, 0u32, 0u32]).bool());
        e.call_log = Some(vec![]);
        let head = list.addr() + 4;
        assert!(e.call(0x0071_aaa0, &args![list, head, head]).bool());
        assert_eq!(e.get(list, ListBox::pHighlightedItem).addr(), tb);
        assert_eq!(
            calls_to(&e, SET_CURRENT_FOCUS_TARGET),
            vec![vec![0x0095_0000, tb, TRAIT_FOCUS, 1]]
        );
        let set_float = calls_to(&e, TILE_SET_FLOAT);
        assert_eq!(set_float.len(), 1);
        assert_eq!(set_float[0][0], 0x0099_0000);
        assert_eq!(set_float[0][2], 3.0f32.to_bits());
        assert_eq!(calls_to(&e, TILE_DELETE), vec![vec![ta, 1]]);
        assert_eq!(calls_to(&e, OBJECT_RELEASE), vec![vec![0x0055_0000, 1]]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![a]]);
        assert_eq!(e.mem.u32(head), b);
        assert_eq!(e.mem.u32(list.addr() + 8), 0);
        assert_eq!(calls_to(&e, NODE_DELETE), vec![vec![nodes[1], 1]]);
        // Removing the last item empties the embedded node.
        e.set(list, ListBox::pHighlightedItem, Ptr::NULL);
        assert!(fn_0071aaa0(&mut e, list, head, head));
        assert_eq!(e.mem.u32(head), 0);
    }

    /// Doubles for the height computation: the container's height is
    /// `height`, the filtered count 0, ten items, ceiling and maximum.
    fn height_engine(height: f64) -> Engine {
        let mut e = list_engine();
        e.register_double(TILE_GET_FLOAT, move |_, a| Ret {
            st0: if a[1] == TRAIT_HEIGHT { height } else { 0.0 },
            ..Ret::default()
        });
        e.register(LIST_BOX_COUNT, |_, _| ret(10));
        e.register(SCROLL_STEPS, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])).ceil(),
            ..Ret::default()
        });
        e.register(FTOL, |_, a| ret(f64::take(a, &mut 0) as i32 as u32));
        e.register(STEP_LIMIT, |_, a| ret(a[0].max(a[1])));
        e
    }

    #[test]
    fn total_list_height_sets_the_scroll_steps() {
        let mut e = height_engine(100.0);
        let list = e.new_object::<ListBox>();
        e.set(list, ListBox::pContainerRect, Ptr::new(0x0099_0000));
        e.set(list, ListBox::pScrollBar, Ptr::new(0x0099_5000));
        e.set(list, ListBox::fTotalListHeight, 300.0);
        e.call_log = Some(vec![]);
        e.call(0x0071_ad30, &args![list]);
        let share = x87_store32(x87_div(x87_mul(10.0, 200.0), 300.0));
        assert_eq!(calls_to(&e, SCROLL_STEPS), vec![vec![share]]);
        assert_eq!(calls_to(&e, STEP_LIMIT), vec![vec![8, 2]]);
        let delta = x87_store32(x87_div(200.0, 7.0));
        let set_float = calls_to(&e, TILE_SET_FLOAT);
        assert_eq!(set_float.len(), 1);
        assert_eq!(set_float[0][0], 0x0099_0000);
        assert_eq!(&set_float[0][2..], &[delta, 1]);
        let set_int = calls_to(&e, TILE_SET_INT);
        assert_eq!(set_int.len(), 1);
        assert_eq!(set_int[0][0], 0x0099_5000);
        assert_eq!(set_int[0][2], 8);
    }

    #[test]
    fn total_list_height_without_overflow_has_one_step() {
        let mut e = height_engine(400.0);
        let list = e.new_object::<ListBox>();
        e.set(list, ListBox::pContainerRect, Ptr::new(0x0099_0000));
        e.set(list, ListBox::pScrollBar, Ptr::new(0x0099_5000));
        e.set(list, ListBox::fTotalListHeight, 300.0);
        e.call_log = Some(vec![]);
        list_box_set_total_list_height(&mut e, list);
        assert!(calls_to(&e, SCROLL_STEPS).is_empty());
        assert!(calls_to(&e, TILE_SET_FLOAT).is_empty());
        assert_eq!(calls_to(&e, TILE_SET_INT)[0][2], 1);
        // No container: nothing at all.
        e.set(list, ListBox::pContainerRect, Ptr::NULL);
        e.call_log = Some(vec![]);
        list_box_set_total_list_height(&mut e, list);
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    #[test]
    fn num_unfiltered_truncates_the_filtered_count() {
        let mut e = list_engine();
        e.register(LIST_BOX_COUNT, |_, _| ret(5));
        let value = std::rc::Rc::new(std::cell::Cell::new(2.9f64));
        let v = value.clone();
        e.register_double(TILE_GET_FLOAT, move |_, _| Ret {
            st0: v.get(),
            ..Ret::default()
        });
        let list = e.new_object::<ListBox>();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0071_ae60, &args![list]).i32(), 3);
        value.set(-1.5);
        assert_eq!(list_box_num_unfiltered(&mut e, list), 6);
        assert_eq!(calls_to(&e, TILE_ADD_USER_TRAIT).len(), 1);
    }
}
