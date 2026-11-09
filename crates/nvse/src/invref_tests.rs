//! Tests of the inventory references (I-12), with the game's functions they call stood in by
//! small test functions; each against the xNVSE 6.4.9 source it translates.

use super::*;
use std::cell::RefCell;

const AREA: u32 = 0x0200_0000;
/// A bump allocator's next block, and its blocks.
const HEAP_TOP: u32 = AREA + 0xfffc;
const HEAP: u32 = AREA + 0x8000;
/// The temporary references' vtable, the container's, an item's.
const REF_VTABLE: u32 = AREA + 0x100;
const CONTAINER_VTABLE: u32 = AREA + 0x600;
const ITEM_VTABLE: u32 = AREA + 0xb00;
/// A container reference (an actor) with its base form (an actor base: its container at
/// `+0x64`), two items, extra data lists and changes.
const CONTAINER: u32 = AREA + 0x1000;
const BASE: u32 = AREA + 0x1100;
const ITEM_A: u32 = AREA + 0x2000;
const ITEM_B: u32 = AREA + 0x3000;
const CHANGES: u32 = AREA + 0x4000;
const DATA: u32 = AREA + 0x4100;
const ENTRY_A: u32 = AREA + 0x4200;
const ENTRY_B: u32 = AREA + 0x4300;
const LISTS_A: u32 = AREA + 0x4400;
const X_A: u32 = AREA + 0x4500;
const X_B: u32 = AREA + 0x4600;
const COUNT_A: u32 = AREA + 0x4700;
const WORN_B: u32 = AREA + 0x4800;
const FORM_COUNTS: u32 = AREA + 0x4900;
const NEXT_ID: u32 = AREA + 0xfff8;

thread_local! {
    static LOG: RefCell<Vec<(&'static str, Vec<u32>)>> = const { RefCell::new(Vec::new()) };
}

fn log(name: &'static str, a: &[u32]) {
    LOG.with(|l| l.borrow_mut().push((name, a.to_vec())));
}

fn calls() -> Vec<(&'static str, Vec<u32>)> {
    LOG.with(|l| l.borrow_mut().drain(..).collect())
}

fn eax(v: u32) -> Ret {
    Ret {
        eax: v,
        ..Ret::default()
    }
}

fn alloc(e: &mut Engine, a: &[u32]) -> Ret {
    let at = e.mem.u32(HEAP_TOP).max(HEAP);
    e.mem.set_u32(HEAP_TOP, at + a[0].next_multiple_of(8));
    for i in 0..a[0] {
        e.mem.set_u8(at + i, 0);
    }
    eax(at)
}

fn free(_: &mut Engine, a: &[u32]) -> Ret {
    log("free", a);
    Ret::default()
}

/// `TESObjectREFR`'s constructor: the vtable and a new id.
fn refr_init(e: &mut Engine, a: &[u32]) -> Ret {
    let id = e.mem.u32(NEXT_ID).max(0xff00_0800);
    e.mem.set_u32(NEXT_ID, id + 1);
    e.mem.set_u32(a[0], REF_VTABLE);
    e.mem.set_u32(a[0] + 0xc, id);
    eax(a[0])
}

fn destroy(_: &mut Engine, a: &[u32]) -> Ret {
    log("destroy", a);
    Ret::default()
}

fn yes(_: &mut Engine, _: &[u32]) -> Ret {
    eax(1)
}

fn remove_item(_: &mut Engine, a: &[u32]) -> Ret {
    log("remove", a);
    Ret::default()
}

/// `BaseExtraList::GetByType` over the list's chain (`BSExtraData`: type at `+4`, next at `+8`).
fn get_by_type(e: &mut Engine, a: &[u32]) -> Ret {
    let mut x = e.mem.u32(a[0] + 4);
    while x != 0 {
        if u32::from(e.mem.u8(x + 4)) == a[1] {
            return eax(x);
        }
        x = e.mem.u32(x + 8);
    }
    eax(0)
}

/// A `BSExtraData` of `kind` at `at`, put at the head of `list` and marked present.
fn add_extra(e: &mut Engine, list: u32, at: u32, kind: u32) {
    e.mem.set_u8(at + 4, kind as u8);
    e.mem.set_u32(at + 8, e.mem.u32(list + 4));
    e.mem.set_u32(list + 4, at);
    let bits = e.mem.u8(list + 8 + (kind >> 3));
    e.mem
        .set_u8(list + 8 + (kind >> 3), bits | (1 << (kind & 7)));
}

/// The container: a base count of 3 of item A; changes: item A with -1 and two lists (A with an
/// `ExtraCount` of 1, B without one, worn), item B (only in the changes) with 2.
fn engine() -> Engine {
    LOG.with(|l| l.borrow_mut().clear());
    let mut e = Engine::new();
    e.map(AREA, 0x1_0000);
    e.register(FORM_HEAP_ALLOCATE, alloc);
    e.register(FORM_HEAP_FREE, free);
    e.register(REFR_INIT, refr_init);
    e.register(EXTRA_GET_BY_TYPE, get_by_type);
    e.register(AREA + 0xf00, destroy);
    e.register(AREA + 0xf10, yes);
    e.register(AREA + 0xf20, remove_item);
    e.register(EXTRA_REMOVE_ALL, free);
    e.mem.set_u32(REF_VTABLE + FORM_DESTROY, AREA + 0xf00);
    e.mem.set_u32(CONTAINER_VTABLE + IS_ACTOR, AREA + 0xf10);
    e.mem
        .set_u32(CONTAINER_VTABLE + REFR_REMOVE_ITEM, AREA + 0xf20);
    for (item, id) in [(ITEM_A, 0x0001_0a00u32), (ITEM_B, 0x0001_0b00)] {
        e.mem.set_u32(item, ITEM_VTABLE);
        e.mem.set_u8(item + 4, 0x28);
        e.mem.set_u32(item + 0xc, id);
    }
    e.mem.set_u32(CONTAINER, CONTAINER_VTABLE);
    e.mem.set_u32(CONTAINER + 0x20, BASE);
    // The base container's list: one entry, 3 of item A.
    e.mem.set_u32(FORM_COUNTS, 3);
    e.mem.set_u32(FORM_COUNTS + 4, ITEM_A);
    e.mem.set_u32(BASE + 0x64 + 4, FORM_COUNTS);
    // The changes.
    add_extra(
        &mut e,
        CONTAINER + REFR_EXTRA,
        CHANGES,
        EXTRA_CONTAINER_CHANGES,
    );
    e.mem.set_u32(CHANGES + 0xc, DATA);
    let objects = alloc(&mut e, &[8]).eax;
    e.mem.set_u32(DATA, objects);
    let second = alloc(&mut e, &[8]).eax;
    e.mem.set_u32(objects, ENTRY_A);
    e.mem.set_u32(objects + 4, second);
    e.mem.set_u32(second, ENTRY_B);
    e.mem.set_u32(ENTRY_A, LISTS_A);
    e.mem.set_u32(ENTRY_A + 4, (-1i32) as u32);
    e.mem.set_u32(ENTRY_A + 8, ITEM_A);
    e.mem.set_u32(ENTRY_B + 4, 2);
    e.mem.set_u32(ENTRY_B + 8, ITEM_B);
    let node = alloc(&mut e, &[8]).eax;
    e.mem.set_u32(LISTS_A, X_A);
    e.mem.set_u32(LISTS_A + 4, node);
    e.mem.set_u32(node, X_B);
    add_extra(&mut e, X_A, COUNT_A, EXTRA_COUNT);
    e.mem.set_u16(COUNT_A + 0xc, 1);
    add_extra(&mut e, X_B, WORN_B, EXTRA_WORN);
    e
}

fn id_of(e: &Engine, refr: u32) -> u32 {
    e.mem.u32(refr + 0xc)
}

/// `CreateInventoryRefEntry`: the temporary reference stands for the item with the list's data;
/// the clean-up destroys it (its base and data cleared first) and frees the entry it made.
#[test]
fn a_reference_with_an_entry_of_its_own_is_made_and_cleaned_up() {
    let mut e = engine();
    let refr = create_entry(&mut e, CONTAINER, ITEM_A, 1, X_A);
    let id = id_of(&e, refr);
    assert_eq!(e.mem.u32(refr + REFR_BASE_FORM), ITEM_A);
    assert_eq!(
        e.mem.u32(refr + REFR_EXTRA + 4),
        COUNT_A,
        "the list's data copied"
    );
    assert!(for_ref_id(&mut e, id));
    let entry = get(&mut e, id).unwrap().data.entry;
    assert_eq!(count(&mut e, id), 1);
    clear(&mut e);
    assert!(!any(&mut e));
    assert_eq!(e.mem.u32(refr + REFR_BASE_FORM), 0);
    assert_eq!(e.mem.u32(refr + REFR_EXTRA + 4), 0);
    let log = calls();
    assert_eq!(log[0], ("destroy", vec![refr, 1]));
    let extend = e.mem.u32(entry);
    assert_eq!(&log[1..], [("free", vec![extend]), ("free", vec![entry])]);
}

/// The stacks of an item (`GetInvRefsForItem`): the base count with the changes' (3 - 1), the
/// lists with their counts first (A's 1, B's 1), nothing left for the rest; an item only in the
/// changes is all one stack.
#[test]
fn an_items_stacks_are_its_lists_then_the_rest() {
    let mut e = engine();
    let ids = crate::commands::inventory_ref::stacks_for_item(&mut e, CONTAINER, ITEM_A);
    assert_eq!(ids.len(), 2);
    let stacks: Vec<_> = ids
        .iter()
        .map(|&id| {
            let r = get(&mut e, id).unwrap().data;
            (r.xdata, e.mem.u32(r.entry + 4))
        })
        .collect();
    assert_eq!(stacks, [(X_A, 1), (X_B, 1)]);
    let ids = crate::commands::inventory_ref::stacks_for_item(&mut e, CONTAINER, ITEM_B);
    let r = get(&mut e, ids[0]).unwrap().data;
    assert_eq!((ids.len(), r.xdata, e.mem.u32(r.entry + 4)), (1, 0, 2));
}

/// `GetInventoryItems`: the base container's items, then the changes' own, in the order of an
/// `UnorderedMap` of 0x40 buckets by address: both items fall in bucket 0, so the one added
/// last comes first.
#[test]
fn a_containers_items_come_in_the_maps_order() {
    let mut e = engine();
    assert_eq!(
        inventory_items(&mut e, CONTAINER),
        [(ITEM_B, 2, ENTRY_B), (ITEM_A, 2, ENTRY_A)]
    );
}

/// A worn stack's removal waits for the clean-up (`kAction_Remove`), which removes the stack's
/// count of the item from the container; an unworn one goes from its entry at once.
#[test]
fn a_worn_stacks_removal_is_deferred() {
    let mut e = engine();
    let ids = crate::commands::inventory_ref::stacks_for_item(&mut e, CONTAINER, ITEM_A);
    let (a, b) = (ids[0], ids[1]);
    calls();
    // B is worn: deferred.
    assert!(remove_from_container(&mut e, b));
    assert!(calls().is_empty());
    // A is not: its list leaves the entry, its count with it (A's entry is the temporary one).
    assert!(remove_from_container(&mut e, a));
    clear(&mut e);
    let removes: Vec<_> = calls()
        .into_iter()
        .filter(|(n, _)| *n == "remove")
        .collect();
    assert_eq!(
        removes,
        [(
            "remove",
            vec![CONTAINER, ITEM_A, X_B, 1, 0, 0, 0, 0, 0, 1, 0]
        )]
    );
}

/// With validation, a stack whose list left the container's entry is no longer valid.
#[test]
fn validation_follows_the_containers_lists() {
    let mut e = engine();
    let refr = create(
        &mut e,
        CONTAINER,
        Data {
            item: ITEM_A,
            entry: ENTRY_A,
            xdata: X_B,
        },
        true,
    );
    let id = id_of(&e, refr);
    assert!(for_ref_id(&mut e, id));
    assert_eq!(get_ref(&mut e, id), refr);
    // X_B leaves A's lists.
    e.mem.set_u32(e.mem.u32(LISTS_A + 4), 0);
    assert!(!for_ref_id(&mut e, id));
    assert_eq!(get_ref(&mut e, id), 0);
}

/// OnSell's item asked for during the handlers: the first list with a count (A's, 1), made once.
#[test]
fn the_sold_item_is_its_first_stack() {
    let mut e = engine();
    assert_eq!(sold_item_ref(&mut e), 0);
    set_sold(&mut e, ENTRY_A, CONTAINER);
    let refr = sold_item_ref(&mut e);
    let id = id_of(&e, refr);
    let r = get(&mut e, id).unwrap().data;
    assert_eq!((r.item, r.xdata, e.mem.u32(r.entry + 4)), (ITEM_A, X_A, 1));
    assert_eq!(sold_item_ref(&mut e), refr);
    clear_sold(&mut e);
    assert_eq!(sold_item_ref(&mut e), 0);
}

/// `BaseExtraList::Add`: the extra data at the head of the list, marked present.
fn extra_add(e: &mut Engine, a: &[u32]) -> Ret {
    let kind = u32::from(e.mem.u8(a[1] + 4));
    add_extra(e, a[0], a[1], kind);
    eax(a[1])
}

/// `BaseExtraList::Copy`: the source's data words (the chain shared, enough here).
fn extra_copy(e: &mut Engine, a: &[u32]) -> Ret {
    let mut bytes = [0u8; 0x1c];
    e.mem.read(a[1] + 4, &mut bytes);
    e.mem.write(a[0] + 4, &bytes);
    Ret::default()
}

fn extra_remove_by_type(_: &mut Engine, a: &[u32]) -> Ret {
    log("remove type", a);
    Ret::default()
}

/// A second container: a reference with container changes whose data has no entry list yet.
const DEST: u32 = AREA + 0x5000;
const DEST_CHANGES: u32 = AREA + 0x5100;
const DEST_DATA: u32 = AREA + 0x5200;

fn copy_engine() -> Engine {
    let mut e = engine();
    e.register(EXTRA_ADD, extra_add);
    e.register(EXTRA_COPY, extra_copy);
    e.register(EXTRA_REMOVE_BY_TYPE, extra_remove_by_type);
    e.mem.set_u32(DEST, CONTAINER_VTABLE);
    add_extra(
        &mut e,
        DEST + REFR_EXTRA,
        DEST_CHANGES,
        EXTRA_CONTAINER_CHANGES,
    );
    e.mem.set_u32(DEST_CHANGES + 0xc, DEST_DATA);
    e
}

/// `CopyToContainer` into changes without an entry list: the new list starts with xNVSE's empty
/// entry, then the item's entry (count 0 + 1) holding a new extra data list copied from the
/// temporary reference's with the worn flag removed; the copy's own inventory reference
/// (`CopyIRAlt`) stands for that list.
#[test]
fn a_copy_gets_a_new_list_in_a_new_entry() {
    let mut e = copy_engine();
    let refr = create_entry(&mut e, CONTAINER, ITEM_A, 1, X_B);
    let id = id_of(&e, refr);
    calls();
    let (done, copy) = copy_to_container(&mut e, id, DEST, true);
    assert!(done);
    let entries = e.mem.u32(DEST_DATA);
    let empty = e.mem.u32(entries);
    assert_eq!(
        (e.mem.u32(empty), e.mem.u32(empty + 4), e.mem.u32(empty + 8)),
        (0, 0, 0),
        "the empty first entry"
    );
    let node = e.mem.u32(entries + 4);
    let entry = e.mem.u32(node);
    assert_eq!(e.mem.u32(entry + ENTRY_TYPE), ITEM_A);
    assert_eq!(e.mem.u32(entry + ENTRY_COUNT_DELTA), 1);
    let list = e.mem.u32(e.mem.u32(entry + ENTRY_EXTEND_DATA));
    assert_eq!(e.mem.u32(list), EXTRA_LIST_VTBL);
    assert_eq!(e.mem.u32(list + 4), WORN_B, "the reference's data copied");
    assert_eq!(calls(), [("remove type", vec![list, EXTRA_WORN])]);
    let copy_id = id_of(&e, copy);
    let r = get(&mut e, copy_id).unwrap();
    assert_eq!(
        (r.container, r.data.item, r.data.entry, r.data.xdata),
        (DEST, ITEM_A, entry, list)
    );
    // Again: the same entry, the second list after the first, the count 2.
    let (done, _) = copy_to_container(&mut e, id, DEST, false);
    assert!(done);
    assert_eq!(e.mem.u32(entry + ENTRY_COUNT_DELTA), 2);
    let lists = list_items(&e, e.mem.u32(entry + ENTRY_EXTEND_DATA));
    assert_eq!((lists.len(), lists[0]), (2, list));
}

/// Into the container's own entry of the item: the list goes after its lists, the count from
/// -1 to 0 (the copied data has no count: 1).
#[test]
fn a_copy_joins_the_items_entry() {
    let mut e = copy_engine();
    let refr = create_entry(&mut e, CONTAINER, ITEM_A, 1, X_B);
    let id = id_of(&e, refr);
    let (done, copy) = copy_to_container(&mut e, id, CONTAINER, false);
    assert!(done);
    assert_eq!(copy, 0);
    assert_eq!(e.mem.u32(ENTRY_A + ENTRY_COUNT_DELTA), 0);
    let lists = list_items(&e, LISTS_A);
    assert_eq!(lists.len(), 3);
    assert_eq!(&lists[..2], [X_A, X_B]);
}

/// Where the stand-in `RemoveItem` below keeps the temporary references it made.
const MADE: u32 = AREA + 0x6800;

/// `RemoveItem`'s stand-in whose handlers make two inventory references: one with the first
/// id again (a destroyed reference's id comes back: bucket 0, before the one being cleared)
/// and one in a later bucket (`0xFF000802`: 26).
fn remove_item_making_references(e: &mut Engine, a: &[u32]) -> Ret {
    log("remove", a);
    e.mem.set_u32(NEXT_ID, 0xff00_0800);
    let early = create_entry(e, CONTAINER, ITEM_B, 2, 0);
    e.mem.set_u32(NEXT_ID, 0xff00_0802);
    let late = create_entry(e, CONTAINER, ITEM_B, 2, 0);
    e.mem.set_u32(MADE, early);
    e.mem.set_u32(MADE + 4, late);
    Ret::default()
}

/// The review of #349: references made while the clean-up runs (a deferred removal's handlers
/// using `GetInvRefsForItem` or `ForEach`) go with it. The map's walk (A in bucket 0, B in 13)
/// destroys B, whose removal makes two; the one in a later bucket goes in the same walk, as in
/// xNVSE, and the one in bucket 0, which xNVSE leaves behind, in a second walk.
#[test]
fn references_made_during_the_clean_up_go_with_it() {
    let mut e = engine();
    e.register(AREA + 0xf20, remove_item_making_references);
    let ids = crate::commands::inventory_ref::stacks_for_item(&mut e, CONTAINER, ITEM_A);
    let refs: Vec<u32> = ids
        .iter()
        .map(|&id| get(&mut e, id).unwrap().temp_ref)
        .collect();
    // B is worn: its removal waits for the clean-up.
    assert!(remove_from_container(&mut e, ids[1]));
    calls();
    clear(&mut e);
    assert!(!any(&mut e), "none left");
    let (early, late) = (e.mem.u32(MADE), e.mem.u32(MADE + 4));
    let destroyed: Vec<u32> = calls()
        .into_iter()
        .filter(|(n, _)| *n == "destroy")
        .map(|(_, a)| a[0])
        .collect();
    assert_eq!(destroyed, [refs[0], refs[1], late, early]);
}

/// The review of #349: `RemoveMeIR` on an unworn stack with data of its own, through a reference
/// with an entry of its own (`GetInvRefsForItem`'s). Both ways the list leaves the reference's
/// entry (its count 1 - 1) and is freed once. Vanilla (xNVSE) leaves the container's entry
/// holding the freed list with its count; the row takes it out of that entry too (the count
/// -1 - 1). With the row, a list no entry of the container holds is not freed.
#[test]
fn an_unworn_stacks_removal_leaves_the_containers_entry() {
    for checks in [false, true] {
        let mut e = engine();
        set_checks(&mut e, checks);
        let ids = crate::commands::inventory_ref::stacks_for_item(&mut e, CONTAINER, ITEM_A);
        let own = get(&mut e, ids[0]).unwrap().data.entry;
        assert_ne!(own, ENTRY_A);
        calls();
        assert!(remove_from_container(&mut e, ids[0]));
        assert!(list_items(&e, e.mem.u32(own)).is_empty(), "checks {checks}");
        assert_eq!(e.mem.u32(own + ENTRY_COUNT_DELTA), 0);
        let frees: Vec<_> = calls().into_iter().filter(|(_, a)| a[0] == X_A).collect();
        assert_eq!(frees, [("free", vec![X_A, 1]), ("free", vec![X_A])]);
        let (lists, count) = (
            list_items(&e, LISTS_A),
            e.mem.u32(ENTRY_A + ENTRY_COUNT_DELTA) as i32,
        );
        if checks {
            assert_eq!((lists, count), (vec![X_B], -2));
        } else {
            assert_eq!((lists, count), (vec![X_A, X_B], -1), "the freed list kept");
        }
    }
    // The row, with the list already gone from the container: nothing freed.
    let mut e = engine();
    set_checks(&mut e, true);
    let ids = crate::commands::inventory_ref::stacks_for_item(&mut e, CONTAINER, ITEM_A);
    e.mem.set_u32(LISTS_A, 0);
    calls();
    assert!(!remove_from_container(&mut e, ids[0]));
    assert!(calls().is_empty());
}

/// The review of #349: a base container entry without a form. xNVSE reads its form type through
/// the null pointer and the game stops; vanilla does too. The row passes over it.
#[test]
fn a_base_entry_without_a_form() {
    for checks in [false, true] {
        let mut e = engine();
        set_checks(&mut e, checks);
        // A second base entry after item A's: 2 of no form.
        let (node, counted) = (AREA + 0x6900, AREA + 0x6910);
        e.mem.set_u32(counted, 2);
        e.mem.set_u32(node, counted);
        e.mem.set_u32(BASE + 0x64 + 8, node);
        let items = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            inventory_items(&mut e, CONTAINER)
        }));
        if checks {
            assert_eq!(items.unwrap(), [(ITEM_B, 2, ENTRY_B), (ITEM_A, 2, ENTRY_A)]);
        } else {
            assert!(items.is_err(), "vanilla stops, as xNVSE");
        }
    }
}

/// The review of #349: a container loop that ends releases its inventory reference before it
/// frees its stacks, so a deferred removal reads the stack's count (1) from an entry not yet
/// freed (xNVSE frees first). The loop's variable is cleared.
#[test]
fn a_container_loop_releases_before_freeing_its_stacks() {
    use crate::commands::scripting::{drop_loop, Loop, LoopInfo, SavedIp};
    let mut e = engine();
    let refr = create(&mut e, CONTAINER, Data::default(), false);
    let id = id_of(&e, refr);
    let stack = create_temp_entry(&mut e, ITEM_A, 1, X_B);
    let data = Data {
        item: ITEM_A,
        entry: stack,
        xdata: X_B,
    };
    set_data(&mut e, id, data);
    // X_B is worn: the removal waits for the release.
    assert!(remove_from_container(&mut e, id));
    calls();
    let local = AREA + 0x6a00;
    e.mem.set_f64(local + 8, 1.0);
    let info = LoopInfo {
        kind: Loop::Container {
            invref: id,
            local,
            elements: vec![stack],
            index: 0,
        },
        saved: SavedIp::default(),
        end_ip: 0,
        depth: 0,
    };
    drop_loop(&mut e, &info);
    let log = calls();
    let removed = log.iter().position(|(n, _)| *n == "remove").unwrap();
    let freed = log
        .iter()
        .position(|c| *c == ("free", vec![stack]))
        .unwrap();
    assert!(removed < freed, "{log:x?}");
    assert_eq!(log[removed].1[3], 1, "the stack's count");
    assert_eq!(e.mem.f64(local + 8), 0.0);
}

/// A destination without container changes gets new ones (no data) added to its list, as
/// xNVSE's `GetForRef` does; the copy then fails. `MoveToContainer` adds them the same way and
/// still defers the move.
#[test]
fn a_destination_without_changes_gets_empty_ones() {
    let mut e = copy_engine();
    let other = AREA + 0x6000;
    e.mem.set_u32(other, CONTAINER_VTABLE);
    let refr = create_entry(&mut e, CONTAINER, ITEM_A, 1, X_A);
    let id = id_of(&e, refr);
    assert_eq!(copy_to_container(&mut e, id, other, true), (false, 0));
    let changes = e.mem.u32(other + REFR_EXTRA + 4);
    assert_eq!(e.mem.u32(changes), CHANGES_VTBL);
    assert_eq!(u32::from(e.mem.u8(changes + 4)), EXTRA_CONTAINER_CHANGES);
    assert_eq!(e.mem.u32(changes + CHANGES_DATA), 0);
    let third = AREA + 0x6100;
    e.mem.set_u32(third, CONTAINER_VTABLE);
    assert!(move_to_container(&mut e, id, third));
    assert_ne!(e.mem.u32(third + REFR_EXTRA + 4), 0);
}

/// Lists that run back into themselves: every walk of the game's lists stops the game past
/// `MAX_LIST_NODES` nodes, or with the list guard row ends there.
#[test]
fn cyclic_lists_stop_the_game_or_end_the_walk() {
    let mut e = Engine::new();
    e.map(AREA, 0x10000);
    let list = AREA + 0x9000;
    e.mem.set_u32(list, 0x77);
    e.mem.set_u32(list + 4, list + 8);
    e.mem.set_u32(list + 8, 0x78);
    e.mem.set_u32(list + 12, list);
    for walk in [
        |e: &mut Engine, l: u32| {
            list_index_of(e, l, 0x99);
        },
        |e: &mut Engine, l: u32| {
            list_count(e, l);
        },
        |e: &mut Engine, l: u32| {
            list_nth(e, l, u32::MAX);
        },
        |e: &mut Engine, l: u32| {
            list_items(e, l);
        },
    ] {
        let stop = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| walk(&mut e, list)));
        assert!(stop.is_err());
    }
    crate::commands::array::set_fix(&mut e, crate::commands::array::LIST_GUARD, true);
    assert_eq!(list_index_of(&e, list, 0x99), -1);
    assert_eq!(list_count(&e, list) as usize, MAX_LIST_NODES + 1);
    assert_eq!(list_nth(&e, list, u32::MAX), 0);
    assert_eq!(list_items(&e, list).len(), MAX_LIST_NODES);
}
