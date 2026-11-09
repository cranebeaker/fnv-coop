//! Inventory references (ADR-0016 build step I-12): translated from xNVSE 6.4.9's
//! `InventoryReference.cpp`. A temporary reference (a `TESObjectREFR` made with the game's own
//! constructor, its id from the game) stands for one stack of an item in a container: its base
//! form is the item and its extra data list a copy of the stack's, so reference commands read it.
//! xNVSE keeps them in a map by the temporary reference's id and destroys them all each frame
//! (the main loop, before the event manager's tick) and before a game is loaded; destroying one
//! first runs what was deferred on it (an equip or a removal that could free the stack's data
//! while scripts still hold it).
//!
//! The map's order (xNVSE's `UnorderedMap`, 0x80 buckets at first) is kept with
//! [`vars::HashOrder`]: the per-frame clean-up runs the deferred actions in that order.

use crate::vars::HashOrder;
use crate::Nvse;
use engine_core::prelude::*;
use std::collections::HashMap;

/// `FormHeap_Allocate`, `FormHeap_Free`.
const FORM_HEAP_ALLOCATE: u32 = 0x0040_1000;
const FORM_HEAP_FREE: u32 = 0x0040_1030;
/// `TESObjectREFR`'s constructor (`s_TESObject_REFR_init`) and size.
const REFR_INIT: u32 = 0x0055_a2f0;
const REFR_SIZE: u32 = 0x68;
/// `TESForm::Destroy(bool doFree)`: the fifth virtual function (after `BaseFormComponent`'s four).
const FORM_DESTROY: u32 = 0x10;
/// `TESForm::refID` (`+0xC`), `TESObjectREFR::baseForm` (`+0x20`), `extraDataList` (`+0x44`).
const FORM_ID: u32 = 0xc;
const REFR_BASE_FORM: u32 = 0x20;
pub const REFR_EXTRA: u32 = 0x44;
/// `BaseExtraList::m_data` and the presence bits after it: the 0x1C bytes xNVSE copies.
const EXTRA_DATA: u32 = 4;
const EXTRA_DATA_LEN: u32 = 0x1c;
const EXTRA_PRESENCE: u32 = 8;
/// `BaseExtraList::GetByType` and the extra data types read here.
const EXTRA_GET_BY_TYPE: u32 = 0x0041_0220;
pub const EXTRA_CONTAINER_CHANGES: u32 = 0x15;
pub const EXTRA_WORN: u32 = 0x16;
pub const EXTRA_COUNT: u32 = 0x24;
pub const EXTRA_LEVELED_ITEM: u32 = 0x2f;
/// `ExtraContainerChanges::data` (`+0xC`), its `objList` (`+0`); `EntryData`: `extendData`
/// (`+0`), `countDelta` (`+4`), `type` (`+8`); `ExtraCount::count` (`+0xC`, a short).
const CHANGES_DATA: u32 = 0xc;
pub const ENTRY_EXTEND_DATA: u32 = 0;
pub const ENTRY_COUNT_DELTA: u32 = 4;
pub const ENTRY_TYPE: u32 = 8;
const EXTRA_COUNT_VALUE: u32 = 0xc;
/// `EntryData`'s size and a `tList` node's.
const ENTRY_SIZE: u32 = 0xc;
const LIST_NODE_SIZE: u32 = 8;
/// `Actor::EquipItem`, `Actor::UnequipItem` (`thiscall`), `TESObjectREFR::RemoveItem` (virtual
/// `0x17C`), `TESForm::IsActor` (virtual `0x100`), `kFormType_TESAmmo`.
const ACTOR_EQUIP_ITEM: u32 = 0x0088_c650;
const ACTOR_UNEQUIP_ITEM: u32 = 0x0088_c790;
const REFR_REMOVE_ITEM: u32 = 0x17c;
const IS_ACTOR: u32 = 0x100;
const FORM_TYPE_AMMO: u8 = 0x29;

/// `InventoryReference::Data`: the item, its changes entry and its extra data list.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Data {
    pub item: u32,
    pub entry: u32,
    pub xdata: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Action {
    Equip,
    Remove,
}

/// `InventoryReference::DeferredAction`.
#[derive(Clone, Copy, Debug)]
struct Deferred {
    action: Action,
    data: Data,
    target: u32,
}

/// `InventoryReference`.
#[derive(Debug)]
pub struct InvRef {
    pub data: Data,
    pub container: u32,
    pub temp_ref: u32,
    deferred: Vec<Deferred>,
    temp_entry: u32,
    validate: bool,
    removed: bool,
}

/// `s_invRefMap` (by the temporary reference's id) with its iteration order; and the item an
/// OnSell handler may ask for (`OnSell::g_soldItemData`, `g_sellerRef`, `g_soldItemInvRef`).
pub struct State {
    refs: HashMap<u32, InvRef>,
    order: HashOrder,
    sold: Option<(u32, u32)>,
    sold_ref: u32,
    /// The fixes preset's row [`set_checks`].
    checks: bool,
}

impl Default for State {
    fn default() -> State {
        State {
            refs: HashMap::new(),
            order: HashOrder::with_buckets(0x80),
            sold: None,
            sold_ref: 0,
            checks: false,
        }
    }
}

/// The fixes preset's row "Inventory reference checks" (docs/FIXES.md), where xNVSE frees, reads
/// or keeps memory it should not: an unworn stack's removal ([`remove_from_container`]) takes the
/// list out of the container's own entry before freeing it; a sale ends with its event
/// (`crate::call_sites`); a base container entry without a form is passed over
/// ([`inventory_items`]). Off (vanilla), each is xNVSE's.
pub fn set_checks(e: &mut Engine, on: bool) {
    state(e).checks = on;
}

/// Whether the row [`set_checks`] is on.
pub fn checks(e: &mut Engine) -> bool {
    state(e).checks
}

/// OnSell's entry and seller while they are kept (`GetSoldItemInvRef` reads them).
#[cfg(test)]
pub fn sale(e: &mut Engine) -> Option<(u32, u32)> {
    state(e).sold
}

/// OnSell's entry and seller, kept while its handlers run.
pub fn set_sold(e: &mut Engine, entry: u32, seller: u32) {
    let s = state(e);
    s.sold = Some((entry, seller));
    s.sold_ref = 0;
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.cpp: OnSell::PostDispatchCleanup.
/// After OnSell's handlers: nothing sold any more.
pub fn clear_sold(e: &mut Engine) {
    let s = state(e);
    s.sold = None;
    s.sold_ref = 0;
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.cpp:
// OnSell::GetSoldItemInvRef.
/// The sold item as an inventory reference (made at the first ask): the first stack of the
/// entry's lists with a count, else the rest, counted as `GetInvRefsForItem` counts; 0 when
/// nothing is being sold.
pub fn sold_item_ref(e: &mut Engine) -> u32 {
    let s = state(e);
    if s.sold_ref != 0 {
        return s.sold_ref;
    }
    let Some((entry, seller)) = s.sold else {
        return 0;
    };
    let container = crate::commands::inventory_ref::container_of(e, seller);
    let item = e.mem.u32(entry + ENTRY_TYPE);
    let delta = e.mem.u32(entry + ENTRY_COUNT_DELTA) as i32;
    let mut count = if container != 0 {
        crate::commands::inventory_ref::base_count(e, container, item)
    } else {
        0
    };
    if count != 0 {
        if has_extra_leveled_item(e, entry) {
            count = delta;
        } else {
            count += delta;
        }
    } else {
        count = delta;
    }
    // `xCount`: the entry's count, then each list's; the fallback passes the last one.
    let mut x = delta;
    let mut made = 0;
    let extend = e.mem.u32(entry + ENTRY_EXTEND_DATA);
    if count > 0 && extend != 0 {
        for xdata in list_items(e, extend) {
            x = count_for_extra_list(e, xdata);
            if x < 1 {
                continue;
            }
            if x > count {
                x = count;
            }
            made = create_entry(e, seller, item, x, xdata);
            break;
        }
        if made == 0 {
            made = create_entry(e, seller, item, x, 0);
        }
    }
    state(e).sold_ref = made;
    made
}

fn state(e: &mut Engine) -> &mut State {
    &mut e.ext::<Nvse>().invrefs
}

// ---- the game's lists ----

/// The most nodes a walk of one of the game's lists goes through. The game's container lists hold
/// a few hundred; one that runs back into itself would keep xNVSE walking for ever, so past this
/// the walk ends with the list guard row (`NV_NVSE_LIST_GUARD`) and stops the game without it.
pub const MAX_LIST_NODES: usize = 1 << 20;

/// The data of each node of a `tList` (`ListNode<T>`: data, next) in order: the non-empty ones, or
/// (`until_empty`) those up to the first empty one, as `tList::GetNthItem` reads them; at most
/// [`MAX_LIST_NODES`] nodes.
/// One more node of a walk of a game list, counted in `walked`: true within [`MAX_LIST_NODES`]; past
/// it the list guard row ends the walk (false), and without the row the game stops.
pub fn list_step(e: &Engine, walked: &mut usize) -> bool {
    *walked += 1;
    if *walked <= MAX_LIST_NODES {
        return true;
    }
    if crate::commands::array::fix_on(e, crate::commands::array::LIST_GUARD) {
        return false;
    }
    panic!("a game list of more than {MAX_LIST_NODES} nodes: it runs back into itself, and xNVSE would walk it for ever");
}

fn walk_list(e: &Engine, list: u32, until_empty: bool) -> Vec<u32> {
    let mut out = Vec::new();
    let mut node = list;
    let mut walked = 0usize;
    while node != 0 {
        if !list_step(e, &mut walked) {
            break;
        }
        let data = e.mem.u32(node);
        if data != 0 {
            out.push(data);
        } else if until_empty {
            break;
        }
        node = e.mem.u32(node + 4);
    }
    out
}

/// The first data of each node of a `tList` (`ListNode<T>`: data, next), skipping empty ones.
pub fn list_items(e: &Engine, list: u32) -> Vec<u32> {
    walk_list(e, list, false)
}

/// A `tList`'s data up to its first empty node (`GetNthItem` from 0 until it answers none).
pub fn list_items_until_empty(e: &Engine, list: u32) -> Vec<u32> {
    walk_list(e, list, true)
}

/// `BaseExtraList::HasType`.
pub fn has_type(e: &Engine, list: u32, kind: u32) -> bool {
    e.mem.u8(list + EXTRA_PRESENCE + (kind >> 3)) & (1 << (kind & 7)) != 0
}

/// `BaseExtraList::GetByType`.
pub fn get_by_type(e: &mut Engine, list: u32, kind: u32) -> u32 {
    e.call(EXTRA_GET_BY_TYPE, &args![list, kind]).u32()
}

/// `GetCountForExtraDataList`: its `ExtraCount`, else 1 (and 1 for no list).
pub fn count_for_extra_list(e: &mut Engine, list: u32) -> i32 {
    if list == 0 {
        return 1;
    }
    let count = get_by_type(e, list, EXTRA_COUNT);
    if count != 0 {
        i32::from(e.mem.u16(count + EXTRA_COUNT_VALUE) as i16)
    } else {
        1
    }
}

/// `ExtraContainerChanges`'s entry list of a reference (`xChanges->data->objList`), or 0.
pub fn entry_list(e: &mut Engine, reference: u32) -> u32 {
    let changes = get_by_type(e, reference + REFR_EXTRA, EXTRA_CONTAINER_CHANGES);
    if changes == 0 {
        return 0;
    }
    let data = e.mem.u32(changes + CHANGES_DATA);
    if data == 0 {
        0
    } else {
        e.mem.u32(data)
    }
}

/// `EntryData::HasExtraLeveledItem`.
pub fn has_extra_leveled_item(e: &Engine, entry: u32) -> bool {
    let extend = e.mem.u32(entry + ENTRY_EXTEND_DATA);
    extend != 0
        && list_items(e, extend)
            .into_iter()
            .any(|x| has_type(e, x, EXTRA_LEVELED_ITEM))
}

/// `EntryDataList::FindForItem`.
pub fn find_for_item(e: &Engine, list: u32, item: u32) -> u32 {
    list_items(e, list)
        .into_iter()
        .find(|&en| e.mem.u32(en + ENTRY_TYPE) == item)
        .unwrap_or(0)
}

// ---- InventoryReference ----

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), InventoryReference.cpp:
// WriteToExtraDataList.
/// Copies a list's data words (`m_data` and the presence bits) over another's; no source clears
/// them.
fn write_to_extra_list(e: &mut Engine, from: u32, to: u32) {
    if to == 0 {
        return;
    }
    if from != 0 {
        let mut bytes = vec![0u8; EXTRA_DATA_LEN as usize];
        e.mem.read(from + EXTRA_DATA, &mut bytes);
        e.mem.write(to + EXTRA_DATA, &bytes);
    } else {
        e.mem
            .write(to + EXTRA_DATA, &[0u8; EXTRA_DATA_LEN as usize]);
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), InventoryReference.cpp:
// CreateInventoryRef.
/// A new inventory reference for `data` in `container`; answers the temporary reference.
pub fn create(e: &mut Engine, container: u32, data: Data, validate: bool) -> u32 {
    let refr = e.call(FORM_HEAP_ALLOCATE, &args![REFR_SIZE]).u32();
    e.call(REFR_INIT, &args![refr]);
    let id = e.mem.u32(refr + FORM_ID);
    let s = state(e);
    s.order.insert(id);
    s.refs.insert(
        id,
        InvRef {
            data: Data::default(),
            container,
            temp_ref: refr,
            deferred: Vec::new(),
            temp_entry: 0,
            validate,
            removed: false,
        },
    );
    set_data(e, id, data);
    refr
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), InventoryReference.cpp: CreateTempEntry.
/// A changes entry of its own for `item` (`countDelta` the count), with `xdata` as its one list.
pub fn create_temp_entry(e: &mut Engine, item: u32, count: i32, xdata: u32) -> u32 {
    let entry = e.call(FORM_HEAP_ALLOCATE, &args![ENTRY_SIZE]).u32();
    let extend = if xdata != 0 {
        let list = e.call(FORM_HEAP_ALLOCATE, &args![LIST_NODE_SIZE]).u32();
        e.mem.set_u32(list, xdata);
        e.mem.set_u32(list + 4, 0);
        list
    } else {
        0
    };
    e.mem.set_u32(entry + ENTRY_EXTEND_DATA, extend);
    e.mem.set_u32(entry + ENTRY_COUNT_DELTA, count as u32);
    e.mem.set_u32(entry + ENTRY_TYPE, item);
    entry
}

/// Frees an entry [`create_temp_entry`] made.
pub fn free_temp_entry(e: &mut Engine, entry: u32) {
    let extend = e.mem.u32(entry + ENTRY_EXTEND_DATA);
    if extend != 0 {
        e.call(FORM_HEAP_FREE, &args![extend]);
    }
    e.call(FORM_HEAP_FREE, &args![entry]);
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), InventoryReference.cpp:
// CreateInventoryRefEntry.
/// An inventory reference with a changes entry of its own (freed with it); the temporary reference.
pub fn create_entry(e: &mut Engine, container: u32, item: u32, count: i32, xdata: u32) -> u32 {
    let entry = create_temp_entry(e, item, count, xdata);
    let refr = create(e, container, Data { item, entry, xdata }, false);
    let id = e.mem.u32(refr + FORM_ID);
    if let Some(r) = state(e).refs.get_mut(&id) {
        r.temp_entry = entry;
    }
    refr
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), GameObjects.cpp:
// TESObjectREFR::GetInventoryItems.
/// A container's items with their counts and changes entries, in the order of xNVSE's
/// `InventoryItemsMap` (an `UnorderedMap` by the item's address, 0x40 buckets at first): the
/// base container's items first (leveled lists left out; a count with changes is the changes'
/// when the entry came from a leveled list, else the sum), then items only in the changes, each
/// with a count above 0. None without a container or changes.
///
/// A base container entry without a form: xNVSE reads its form type through the null pointer and
/// the game stops, and so does vanilla here; the row [`set_checks`] passes over it.
pub fn inventory_items(e: &mut Engine, reference: u32) -> Vec<(u32, i32, u32)> {
    let container = crate::commands::inventory_ref::container_of(e, reference);
    if container == 0 {
        return Vec::new();
    }
    let list = entry_list(e, reference);
    if list == 0 {
        return Vec::new();
    }
    let checks = state(e).checks;
    let mut order = HashOrder::with_buckets(0x40);
    let mut items: HashMap<u32, (i32, u32)> = HashMap::new();
    for fc in list_items(e, container + 4) {
        let item = e.mem.u32(fc + 4);
        if item == 0 && checks {
            continue;
        }
        if e.mem.u8(item + 4) == FORM_TYPE_LEVELED_ITEM || items.contains_key(&item) {
            continue;
        }
        let mut count: i32 = list_items(e, container + 4)
            .into_iter()
            .filter(|&f| e.mem.u32(f + 4) == item)
            .map(|f| e.mem.u32(f) as i32)
            .sum();
        let entry = find_for_item(e, list, item);
        if entry != 0 {
            let delta = e.mem.u32(entry + ENTRY_COUNT_DELTA) as i32;
            if has_extra_leveled_item(e, entry) {
                count = delta;
            } else {
                count += delta;
            }
        }
        if count > 0 {
            items.insert(item, (count, entry));
            order.insert(item);
        }
    }
    for entry in list_items(e, list) {
        let item = e.mem.u32(entry + ENTRY_TYPE);
        if items.contains_key(&item) {
            continue;
        }
        let delta = e.mem.u32(entry + ENTRY_COUNT_DELTA) as i32;
        if delta > 0 {
            items.insert(item, (delta, entry));
            order.insert(item);
        }
    }
    order
        .keys()
        .into_iter()
        .map(|item| (item, items[&item].0, items[&item].1))
        .collect()
}

/// `kFormType_TESLevItem`.
const FORM_TYPE_LEVELED_ITEM: u8 = 0x34;

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), InventoryReference.cpp: SetData.
/// The reference stands for `data`: its base form the item, its extra data a copy of the list's.
pub fn set_data(e: &mut Engine, id: u32, data: Data) {
    let Some(r) = state(e).refs.get_mut(&id) else {
        return;
    };
    r.removed = false;
    r.data = data;
    let refr = r.temp_ref;
    e.mem.set_u32(refr + REFR_BASE_FORM, data.item);
    write_to_extra_list(e, data.xdata, refr + REFR_EXTRA);
}

/// The inventory reference of temporary reference `id` (without its validation).
pub fn get(e: &mut Engine, id: u32) -> Option<&mut InvRef> {
    state(e).refs.get_mut(&id)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), InventoryReference.cpp: Validate.
/// Whether the stack is still where it was: with validation, the container's entry for the item
/// still holds the extra data list.
pub fn validate(e: &mut Engine, id: u32) -> bool {
    let Some(r) = state(e).refs.get(&id) else {
        return false;
    };
    if !r.validate {
        return true;
    }
    let (container, entry, xdata) = (r.container, r.data.entry, r.data.xdata);
    let list = entry_list(e, container);
    if list == 0 || entry == 0 {
        return false;
    }
    let item = e.mem.u32(entry + ENTRY_TYPE);
    entry_holding(e, list, item, xdata) != 0
}

/// The first entry of the entry list `list` for `item` whose lists hold `xdata`, or 0
/// ([`validate`]'s walk).
fn entry_holding(e: &Engine, list: u32, item: u32, xdata: u32) -> u32 {
    if list == 0 {
        return 0;
    }
    for en in list_items(e, list) {
        if e.mem.u32(en + ENTRY_TYPE) != item {
            continue;
        }
        let extend = e.mem.u32(en + ENTRY_EXTEND_DATA);
        if extend != 0 && list_items(e, extend).contains(&xdata) {
            return en;
        }
    }
    0
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), InventoryReference.cpp: GetForRefID.
/// The inventory reference whose temporary reference has id `id`, when valid.
pub fn for_ref_id(e: &mut Engine, id: u32) -> bool {
    state(e).refs.contains_key(&id) && validate(e, id)
}

/// `GetRef`: the temporary reference when valid, else 0.
pub fn get_ref(e: &mut Engine, id: u32) -> u32 {
    if validate(e, id) {
        state(e).refs.get(&id).map_or(0, |r| r.temp_ref)
    } else {
        0
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), InventoryReference.cpp:
// WriteRefDataToContainer.
/// Copies the temporary reference's extra data back to the stack's list.
pub fn write_ref_data_to_container(e: &mut Engine, id: u32) -> bool {
    let Some(r) = state(e).refs.get(&id) else {
        return false;
    };
    let (refr, xdata, removed) = (r.temp_ref, r.data.xdata, r.removed);
    if refr == 0 || !validate(e, id) {
        return false;
    }
    if removed {
        return true;
    }
    if xdata != 0 {
        write_to_extra_list(e, refr + REFR_EXTRA, xdata);
    }
    true
}

/// `GetCount`: the entry's count.
pub fn count(e: &mut Engine, id: u32) -> i32 {
    let entry = state(e).refs.get(&id).map_or(0, |r| r.data.entry);
    if entry == 0 {
        return 0;
    }
    e.mem.u32(entry + ENTRY_COUNT_DELTA) as i32
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), InventoryReference.cpp:
// RemoveFromContainer.
/// Takes the stack out of the container: a worn stack waits for the release (deferred).
///
/// An unworn stack with extra data (a weapon with its condition) leaves the reference's entry
/// and its list is freed. xNVSE does that through the entry the reference holds, which for a
/// reference with an entry of its own (`GetInvRefsForItem`, `ForEach`, OnSell, OnRepair) is that
/// entry, not the container's: the container's entry keeps the freed list and its count, so the
/// game later reads freed memory as the item's data. Vanilla does the same. The row
/// [`set_checks`] takes the list out of the container's own entry too (its count with it) before
/// freeing it, and frees nothing when no entry of the container holds it.
pub fn remove_from_container(e: &mut Engine, id: u32) -> bool {
    if !validate(e, id) {
        return false;
    }
    let Some(r) = state(e).refs.get(&id) else {
        return false;
    };
    let (refr, container, data) = (r.temp_ref, r.container, r.data);
    if refr == 0 || container == 0 {
        return false;
    }
    if data.xdata != 0 && has_type(e, data.xdata, EXTRA_WORN) {
        push_deferred(e, id, Action::Remove, data, 0);
        return true;
    }
    if let Some(r) = state(e).refs.get_mut(&id) {
        r.removed = true;
    }
    if data.xdata != 0 && state(e).checks {
        let list = entry_list(e, container);
        let own = entry_holding(e, list, data.item, data.xdata);
        if own == 0 {
            return false;
        }
        if own != data.entry {
            // The reference's own entry: the list and its count leave it as in xNVSE, unfreed.
            entry_remove(e, data.entry, data.xdata, false);
        }
        entry_remove(e, own, data.xdata, true)
    } else if data.xdata != 0 {
        entry_remove(e, data.entry, data.xdata, true)
    } else {
        let changes = get_by_type(e, container + REFR_EXTRA, EXTRA_CONTAINER_CHANGES);
        changes_remove(e, changes, data.item, 0);
        false
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), GameTypes.h: ListNode::RemoveMe,
// ListNode::RemoveNext, tList::RemoveNth.
/// Removes item `index` of a `tList` (its head node is the list): the head takes its next
/// node's item (that node is freed), or empties; another node is unlinked and freed.
fn list_remove_nth(e: &mut Engine, list: u32, index: i32) {
    if index <= 0 {
        let next = e.mem.u32(list + 4);
        if next != 0 {
            e.mem.set_u32(list, e.mem.u32(next));
            e.mem.set_u32(list + 4, e.mem.u32(next + 4));
            e.call(FORM_HEAP_FREE, &args![next]);
        } else {
            e.mem.set_u32(list, 0);
        }
        return;
    }
    let mut node = list;
    let mut index = index;
    let mut walked = 0usize;
    loop {
        if !list_step(e, &mut walked) {
            return;
        }
        let next = e.mem.u32(node + 4);
        if next == 0 {
            break;
        }
        index -= 1;
        if index == 0 {
            break;
        }
        node = next;
    }
    if index == 0 {
        let next = e.mem.u32(node + 4);
        e.mem.set_u32(node + 4, e.mem.u32(next + 4));
        e.call(FORM_HEAP_FREE, &args![next]);
    }
}

/// `tList::GetIndexOf`: the index of the node holding `item` (empty nodes counted), or -1.
fn list_index_of(e: &Engine, list: u32, item: u32) -> i32 {
    let (mut node, mut index, mut walked) = (list, 0, 0usize);
    while node != 0 {
        if !list_step(e, &mut walked) {
            return -1;
        }
        let data = e.mem.u32(node);
        if data != 0 && data == item {
            return index;
        }
        index += 1;
        node = e.mem.u32(node + 4);
    }
    -1
}

/// `tList::Count`: 0 when the head holds nothing, else every node.
fn list_count(e: &Engine, list: u32) -> u32 {
    if e.mem.u32(list) == 0 {
        return 0;
    }
    let (mut node, mut count, mut walked) = (e.mem.u32(list + 4), 1, 0usize);
    while node != 0 {
        if !list_step(e, &mut walked) {
            break;
        }
        count += 1;
        node = e.mem.u32(node + 4);
    }
    count
}

/// `tList::GetNthItem`: the item of node `n` (counting every node), or 0.
fn list_nth(e: &Engine, list: u32, n: u32) -> u32 {
    let mut node = list;
    let mut walked = 0usize;
    for _ in 0..n {
        if !list_step(e, &mut walked) {
            return 0;
        }
        node = e.mem.u32(node + 4);
        if node == 0 {
            return 0;
        }
    }
    e.mem.u32(node)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), GameExtraData.cpp:
// ExtraContainerChanges::EntryData::Remove.
/// Takes `list` out of the entry's lists, lowers the count by its count, and frees it when asked.
pub fn entry_remove(e: &mut Engine, entry: u32, list: u32, free: bool) -> bool {
    let extend = e.mem.u32(entry + ENTRY_EXTEND_DATA);
    if extend == 0 || list == 0 {
        return false;
    }
    let index = list_index_of(e, extend, list);
    if index < 0 {
        return false;
    }
    let count = count_for_extra_list(e, list);
    list_remove_nth(e, extend, index);
    let delta = e.mem.u32(entry + ENTRY_COUNT_DELTA) as i32;
    e.mem
        .set_u32(entry + ENTRY_COUNT_DELTA, delta.wrapping_sub(count) as u32);
    if free {
        e.call(EXTRA_REMOVE_ALL, &args![list, 1u32]);
        e.call(FORM_HEAP_FREE, &args![list]);
    }
    true
}

/// `BaseExtraList::RemoveAll(bool doFree)`.
const EXTRA_REMOVE_ALL: u32 = 0x0040_fae0;

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), GameExtraData.cpp:
// ExtraContainerChanges::Remove.
/// For every entry of `item`: with `list`, that list taken out of its lists; without one, an
/// entry with no lists taken out of the changes. (xNVSE answers false whatever it did.) Without
/// changes, their data or an entry list xNVSE reads through a null pointer; here nothing is done.
fn changes_remove(e: &mut Engine, changes: u32, item: u32, list: u32) {
    if changes == 0 {
        return;
    }
    let data = e.mem.u32(changes + CHANGES_DATA);
    if data == 0 || e.mem.u32(data) == 0 {
        return;
    }
    let objects = e.mem.u32(data);
    let mut i = 0;
    while i < list_count(e, objects) {
        let found = list_nth(e, objects, i);
        if e.mem.u32(found + ENTRY_TYPE) == item {
            let extend = e.mem.u32(found + ENTRY_EXTEND_DATA);
            if list != 0 && extend != 0 {
                let mut j = 0;
                while j < list_count(e, extend) {
                    if list_nth(e, extend, j) == list {
                        list_remove_nth(e, extend, j as i32);
                    }
                    j += 1;
                }
            } else if list == 0 && extend == 0 {
                list_remove_nth(e, objects, i as i32);
            }
        }
        i += 1;
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), InventoryReference.cpp: MoveToContainer.
/// Moves the stack to `dest` when it is released (deferred). A `dest` without container changes
/// gets empty ones first (xNVSE's `GetForRef`).
pub fn move_to_container(e: &mut Engine, id: u32, dest: u32) -> bool {
    changes_for_ref(e, dest);
    if !validate(e, id) {
        return false;
    }
    let Some(r) = state(e).refs.get(&id) else {
        return false;
    };
    let (refr, container, data) = (r.temp_ref, r.container, r.data);
    if refr == 0 || container == 0 {
        return false;
    }
    push_deferred(e, id, Action::Remove, data, dest);
    true
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), InventoryReference.cpp: CopyToContainer.
/// Adds a copy of the stack (a new extra data list copied from the temporary reference's, not
/// worn) to `dest`'s changes, at once. With `want_ref`, also an inventory reference for the copy
/// (its temporary reference; xNVSE's `outInvRef`). False when `dest`'s changes have no data, as a
/// `dest` that had none gets (the new list is then left unused, as in xNVSE).
pub fn copy_to_container(e: &mut Engine, id: u32, dest: u32, want_ref: bool) -> (bool, u32) {
    let changes = changes_for_ref(e, dest);
    if !validate(e, id) {
        return (false, 0);
    }
    let refr = state(e).refs.get(&id).map_or(0, |r| r.temp_ref);
    if refr == 0 {
        return (false, 0);
    }
    let list = e.call(FORM_HEAP_ALLOCATE, &args![EXTRA_LIST_SIZE]).u32();
    e.mem.write(list, &[0u8; EXTRA_LIST_SIZE as usize]);
    e.mem.set_u32(list, EXTRA_LIST_VTBL);
    e.call(EXTRA_COPY, &args![list, refr + REFR_EXTRA]);
    e.call(EXTRA_REMOVE_BY_TYPE, &args![list, EXTRA_WORN]);
    let item = e.mem.u32(refr + REFR_BASE_FORM);
    let (extend, entry) = changes_add(e, changes, item, list);
    if extend == 0 {
        return (false, 0);
    }
    let copy = if want_ref {
        create(
            e,
            dest,
            Data {
                item,
                entry,
                xdata: list,
            },
            false,
        )
    } else {
        0
    };
    (true, copy)
}

// ---- xNVSE's own container changes code (GameExtraData.cpp, GameTypes.h) ----

/// `ExtraDataList` and `ExtraContainerChanges`: vtable and size (`s_ExtraDataListVtbl`,
/// `s_ExtraContainerChangesVtbl` (xNVSE)); `BSExtraData::type` (`+4`).
const EXTRA_LIST_VTBL: u32 = 0x0101_43e8;
const EXTRA_LIST_SIZE: u32 = 0x20;
const CHANGES_VTBL: u32 = 0x0101_5bb8;
const CHANGES_SIZE: u32 = 0x10;
const EXTRA_TYPE: u32 = 4;
/// `BaseExtraList::Add(extra)`, `Copy(from)`, `RemoveByType(type)` (`thiscall`, the list).
const EXTRA_ADD: u32 = 0x0040_ff60;
const EXTRA_COPY: u32 = 0x0041_1ec0;
const EXTRA_REMOVE_BY_TYPE: u32 = 0x0041_0140;

/// `BaseExtraList::Remove(extra, free)` (`thiscall`).
const EXTRA_REMOVE: u32 = 0x0041_0020;

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), GameExtraData.cpp: BSExtraData::Create.
/// A new extra data block: zeroed, with its vtable and type.
pub fn create_extra(e: &mut Engine, kind: u32, size: u32, vtable: u32) -> u32 {
    let extra = heap_zeroed(e, size);
    e.mem.set_u32(extra, vtable);
    e.mem.set_u8(extra + EXTRA_TYPE, kind as u8);
    extra
}

/// `BaseExtraList::Add`.
pub fn extra_add(e: &mut Engine, list: u32, extra: u32) {
    e.call(EXTRA_ADD, &args![list, extra]);
}

/// `BaseExtraList::Remove`.
pub fn extra_remove(e: &mut Engine, list: u32, extra: u32, free: bool) {
    e.call(EXTRA_REMOVE, &args![list, extra, u32::from(free)]);
}

/// A zeroed block of the form heap.
fn heap_zeroed(e: &mut Engine, size: u32) -> u32 {
    let block = e.call(FORM_HEAP_ALLOCATE, &args![size]).u32();
    e.mem.write(block, &vec![0u8; size as usize]);
    block
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), GameExtraData.cpp:
// ExtraContainerChanges::GetForRef, ExtraContainerChanges::Create, BSExtraData::Create.
/// A reference's container changes; when it has none, new ones with no data are added to its
/// extra data list.
pub fn changes_for_ref(e: &mut Engine, reference: u32) -> u32 {
    let list = reference + REFR_EXTRA;
    let found = get_by_type(e, list, EXTRA_CONTAINER_CHANGES);
    if found != 0 {
        return found;
    }
    let changes = heap_zeroed(e, CHANGES_SIZE);
    e.mem.set_u32(changes, CHANGES_VTBL);
    e.mem
        .set_u8(changes + EXTRA_TYPE, EXTRA_CONTAINER_CHANGES as u8);
    e.call(EXTRA_ADD, &args![list, changes]);
    changes
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), GameTypes.h: tList::AddAt (at the end),
// ListNode::Append.
/// Puts `item` at the end of the `tList` at `list`: in the last node when it is empty, else in a
/// new node after it.
fn list_append(e: &mut Engine, list: u32, item: u32) {
    if item == 0 {
        return;
    }
    let mut node = list;
    let mut walked = 0usize;
    while e.mem.u32(node + 4) != 0 {
        if !list_step(e, &mut walked) {
            return;
        }
        node = e.mem.u32(node + 4);
    }
    if e.mem.u32(node) == 0 {
        e.mem.set_u32(node, item);
        return;
    }
    let added = e.call(FORM_HEAP_ALLOCATE, &args![LIST_NODE_SIZE]).u32();
    e.mem.set_u32(added, item);
    e.mem.set_u32(added + 4, e.mem.u32(node + 4));
    e.mem.set_u32(node + 4, added);
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), GameExtraData.cpp:
// ExtraContainerChanges::Add, ExtraContainerChangesEntryDataListCreate, EntryData::Create,
// EntryData::Add, ExtraContainerChangesExtendDataListCreate.
/// Adds the extra data list `list` of `item` to the changes: the item's entry (a new one with a
/// count of 0 at the end when there is none) gets the list at the end of its lists and its count
/// raised by the list's count (1 without one). Answers the entry's lists and the entry; (0, 0)
/// when the changes have no data. A new entry list starts with an empty entry, as xNVSE's
/// default `ExtraContainerChangesEntryDataListCreate()` makes it.
fn changes_add(e: &mut Engine, changes: u32, item: u32, list: u32) -> (u32, u32) {
    let data = e.mem.u32(changes + CHANGES_DATA);
    if data == 0 {
        return (0, 0);
    }
    let mut entries = e.mem.u32(data);
    if entries == 0 {
        entries = heap_zeroed(e, LIST_NODE_SIZE);
        let empty = heap_zeroed(e, ENTRY_SIZE);
        list_append(e, entries, empty);
        e.mem.set_u32(data, entries);
    }
    let mut entry = find_for_item(e, entries, item);
    if entry == 0 {
        entry = heap_zeroed(e, ENTRY_SIZE);
        e.mem.set_u32(entry + ENTRY_TYPE, item);
        list_append(e, entries, entry);
    }
    let mut extend = e.mem.u32(entry + ENTRY_EXTEND_DATA);
    if extend != 0 {
        list_append(e, extend, list);
    } else {
        extend = heap_zeroed(e, LIST_NODE_SIZE);
        list_append(e, extend, list);
        e.mem.set_u32(entry + ENTRY_EXTEND_DATA, extend);
    }
    let count = get_by_type(e, list, EXTRA_COUNT);
    let added = if count != 0 {
        i32::from(e.mem.u16(count + EXTRA_COUNT_VALUE) as i16)
    } else {
        1
    };
    let delta = e.mem.u32(entry + ENTRY_COUNT_DELTA) as i32;
    e.mem
        .set_u32(entry + ENTRY_COUNT_DELTA, delta.wrapping_add(added) as u32);
    (extend, entry)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), InventoryReference.cpp: SetEquipped.
/// Equips or unequips the stack when it is released (deferred); false when it already is.
pub fn set_equipped(e: &mut Engine, id: u32, equipped: bool) -> bool {
    let Some(r) = state(e).refs.get(&id) else {
        return false;
    };
    let data = r.data;
    if data.xdata != 0 {
        if has_type(e, data.xdata, EXTRA_WORN) == equipped {
            return false;
        }
    } else if !equipped {
        return false;
    }
    push_deferred(e, id, Action::Equip, data, 0);
    true
}

fn push_deferred(e: &mut Engine, id: u32, action: Action, data: Data, target: u32) {
    if let Some(r) = state(e).refs.get_mut(&id) {
        r.deferred.push(Deferred {
            action,
            data,
            target,
        });
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), InventoryReference.cpp:
// DoDeferredActions, DeferredAction::Execute.
/// Runs the deferred actions, the last one first.
fn do_deferred_actions(e: &mut Engine, id: u32) {
    loop {
        let Some(action) = state(e)
            .refs
            .get(&id)
            .and_then(|r| r.deferred.last().copied())
        else {
            return;
        };
        set_data(e, id, action.data);
        let container = state(e).refs.get(&id).map_or(0, |r| r.container);
        if validate(e, id) && container != 0 {
            execute(e, id, container, action);
        }
        if let Some(r) = state(e).refs.get_mut(&id) {
            r.deferred.pop();
        }
    }
}

fn execute(e: &mut Engine, id: u32, container: u32, action: Deferred) {
    let data = action.data;
    match action.action {
        Action::Equip => {
            if !e.vcall(container, IS_ACTOR, &[]).bool() {
                return;
            }
            if data.xdata != 0 && has_type(e, data.xdata, EXTRA_WORN) {
                e.call(
                    ACTOR_UNEQUIP_ITEM,
                    &args![container, data.item, 1u32, data.xdata, 0u32, 0u32, 0u32],
                );
            } else {
                let count = if e.mem.u8(data.item + 4) == FORM_TYPE_AMMO {
                    count(e, id) as u32
                } else {
                    1
                };
                e.call(
                    ACTOR_EQUIP_ITEM,
                    &args![container, data.item, count, data.xdata, 1u32, 0u32, 1u32],
                );
            }
        }
        Action::Remove => {
            let count = count(e, id) as u32;
            e.vcall(
                container,
                REFR_REMOVE_ITEM,
                &args![
                    data.item,
                    data.xdata,
                    count,
                    0u32,
                    0u32,
                    action.target,
                    0u32,
                    0u32,
                    1u32,
                    0u32
                ],
            );
            if let Some(r) = state(e).refs.get_mut(&id) {
                r.removed = true;
            }
            set_data(e, id, Data::default());
        }
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), InventoryReference.cpp: Release.
/// The deferred actions run, and the reference stands for nothing.
pub fn release(e: &mut Engine, id: u32) {
    do_deferred_actions(e, id);
    set_data(e, id, Data::default());
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), InventoryReference.cpp:
// ~InventoryReference.
/// Out of the map's order first (xNVSE's clean-up takes the chain's head before destroying it),
/// then the deferred actions, then the temporary reference and the entry of its own.
fn destroy(e: &mut Engine, id: u32) {
    state(e).order.remove(id);
    if state(e).refs.get(&id).is_some_and(|r| r.data.item != 0) {
        do_deferred_actions(e, id);
    }
    let Some(r) = state(e).refs.remove(&id) else {
        return;
    };
    if r.temp_ref != 0 {
        e.mem.set_u32(r.temp_ref + REFR_BASE_FORM, 0);
        e.mem.write(
            r.temp_ref + REFR_EXTRA + EXTRA_DATA,
            &[0u8; EXTRA_DATA_LEN as usize],
        );
        e.vcall(r.temp_ref, FORM_DESTROY, &args![1u32]);
    }
    if r.temp_entry != 0 {
        free_temp_entry(e, r.temp_entry);
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), containers.h: UnorderedMap::Clear,
// Bucket::Clear.
/// `s_invRefMap.Clear()`: every inventory reference destroyed, in the map's order (the main loop
/// each frame, and before a game is loaded), until none is left.
///
/// A destroyed reference's deferred equip or removal raises the game's events, and their
/// handlers can make new references (`ForEach` over a container, `GetInvRefsForItem`). xNVSE
/// walks the buckets in order and takes each chain's head before destroying it, so one made in
/// the bucket being cleared or a later one goes in the same walk; one made in an earlier bucket
/// stays there with the map's count set to 0, out of reach of lookups, until a later clean-up
/// with a new reference in the map. Here the walk starts again from the first bucket until the
/// map is empty, so that one goes now.
pub fn clear(e: &mut Engine) {
    let mut from = 0;
    loop {
        match state(e).order.head_from(from) {
            Some((bucket, id)) => {
                from = bucket;
                destroy(e, id);
            }
            None if from != 0 => from = 0,
            None => break,
        }
    }
    state(e).order.clear();
}

/// Whether any inventory reference exists (`!s_invRefMap.Empty()`).
pub fn any(e: &mut Engine) -> bool {
    !state(e).refs.is_empty()
}

#[cfg(test)]
#[path = "invref_tests.rs"]
mod tests;
