//! `BSMain/run_00c5ae70` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Methods of `BSPortalGraph` (Xbox PDB layout, declared in
//! `bsportalgraph.rs`): adding to and removing from its occluder, portal
//! and room lists, queueing always-render objects for attaching or
//! detaching, and clearing the always-render array. The class of `this` is
//! confirmed by a caller: `0054aa11` (in `0054a070`, `TESObjectCELL` unit)
//! passes the cell's `spPortalGraph` (+0xD4 on PC, read by `009d9f20`); the
//! field offsets used match the PDB layout.
//!
//! Reference-counted objects (`NiPointer` temporaries) are handled as the
//! exe does: `InterlockedIncrement` on the count at +4 when a non-null
//! pointer is copied, `InterlockedDecrement` when it goes out of scope and
//! virtual slot +4 (`DeleteThis`) when the count reaches zero. The
//! `FS:[0]` exception frames of four functions are not translated (marked
//! `Translation incomplete`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiTArray, NiTListItem, NiTPointerList};
use crate::units::bsmain::bsportalgraph::BSPortalGraph;

/// `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `EnterCriticalSection` / `LeaveCriticalSection` (import slots).
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// `BSPortalGraph::kAlwaysQueueCrit` (Xbox PDB static), the critical section
/// guarding the two always-render queues.
const ALWAYS_QUEUE_CRITICAL_SECTION: u32 = 0x0120_3328;

/// `0043a010(allocator)` (engine map: `NiTPointerListBase<..>::AddHead`, a
/// folded name): a new list item from the list's allocator (+8 of the list).
const LIST_NEW_ITEM: u32 = 0x0043_a010;
/// `00559a70(list, item)`: links an item into a pointer list.
const LIST_ADD_ITEM: u32 = 0x0055_9a70;
/// `00b5a9a0(list, item)`: links an item into an `NiPointer` list.
const POINTER_LIST_ADD_ITEM: u32 = 0x00b5_a9a0;
/// `00c3a250(list, &element)`: removes an element from the occluder list.
const OCCLUDER_LIST_REMOVE: u32 = 0x00c3_a250;
/// `0057c730(list, &element)`: removes an element from the portal list.
const PORTAL_LIST_REMOVE: u32 = 0x0057_c730;
/// `00a5a310(list, &pointer)`: adds an `NiPointer` to the room list.
const ROOM_LIST_ADD: u32 = 0x00a5_a310;
/// `00c5bda0(list, &out, &pointer)`: removes an `NiPointer` from the room
/// list, the removed pointer going to `out`.
const ROOM_LIST_REMOVE: u32 = 0x00c5_bda0;
/// `00c5bfb0(list, &out, &item)`: unlinks the item from a queue, its
/// `NiPointer` going to `out`.
const QUEUE_REMOVE_ITEM: u32 = 0x00c5_bfb0;
/// `006e5cc0(this, &source)` (engine map: `NiPointer<..>::operator=`).
const NI_POINTER_ASSIGN: u32 = 0x006e_5cc0;
/// `004dffa0` (engine map: `NiTArray<NiPointer<BSTempEffect>..>::RemoveAll`).
const ARRAY_REMOVE_ALL: u32 = 0x004d_ffa0;

fn add_ref(e: &mut Engine, object: u32) {
    if object != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
    }
}

fn release(e: &mut Engine, object: u32) {
    if object != 0 {
        let count = e
            .call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
            .u32();
        if count == 0 {
            e.vcall(object, 4, &[]);
        }
    }
}

/// A new item holding `element`, linked into the pointer list at `list`.
fn list_add(e: &mut Engine, list: u32, element: u32) {
    let item = e.call(LIST_NEW_ITEM, &args![list + 8]).u32();
    e.mem
        .set_u32(item.wrapping_add(NiTListItem::m_element.off), element);
    e.call(LIST_ADD_ITEM, &args![list, item]);
}

/// The address of a list field of the graph.
fn list_of(this: Ptr<BSPortalGraph>, field: Field<BSPortalGraph, Inline<NiTPointerList>>) -> u32 {
    this.at(field).addr()
}

// Translated from 00c5ae70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a non-null occluder to the head of `lOccluderList` (Xbox PDB).
pub fn fn_00c5ae70(e: &mut Engine, this: Ptr<BSPortalGraph>, occluder: u32) {
    if occluder != 0 {
        list_add(e, list_of(this, BSPortalGraph::lOccluderList), occluder);
    }
}

// Translated from 00c5aec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes a non-null occluder from `lOccluderList` (`00c3a250` with the
/// address of the argument).
pub fn fn_00c5aec0(e: &mut Engine, this: Ptr<BSPortalGraph>, occluder: u32) {
    if occluder != 0 {
        let list = list_of(this, BSPortalGraph::lOccluderList);
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), occluder);
            e.call(OCCLUDER_LIST_REMOVE, &args![list, slot]);
        });
    }
}

// Translated from 00c5aef0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c5aef0 - FS:[0] exception frame (unwinding of the NiPointer temporary) not translated
/// Adds a non-null room to `lMultiBoundRoomList` (Xbox PDB) unless the list
/// already holds it (`00a5a310` with an `NiPointer` temporary). The search
/// holds a reference to the room while it walks.
pub fn fn_00c5aef0(e: &mut Engine, this: Ptr<BSPortalGraph>, room: u32) {
    if room == 0 {
        return;
    }
    add_ref(e, room);
    let list = list_of(this, BSPortalGraph::lMultiBoundRoomList);
    let mut item = e.mem.u32(list + NiTPointerList::m_pkHead.off);
    let mut found = 0;
    while item != 0 {
        let current = item;
        let element = e.mem.u32(item + NiTListItem::m_element.off);
        item = e.mem.u32(item + NiTListItem::m_pkNext.off);
        if room == element {
            found = current;
            break;
        }
    }
    let absent = found == 0;
    release(e, room);
    if absent {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), room);
            add_ref(e, room);
            e.call(ROOM_LIST_ADD, &args![list, slot]);
            let pointer = e.mem.u32(slot.addr());
            release(e, pointer);
        });
    }
}

// Translated from 00c5b060 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c5b060 - FS:[0] exception frame (unwinding of the NiPointer temporaries) not translated
/// Removes a non-null room from `lMultiBoundRoomList` (`00c5bda0`) and
/// releases the removed pointer and the temporary.
pub fn fn_00c5b060(e: &mut Engine, this: Ptr<BSPortalGraph>, room: u32) {
    if room == 0 {
        return;
    }
    let list = list_of(this, BSPortalGraph::lMultiBoundRoomList);
    // `EBP-0x14` (the removed pointer) and `EBP-0x10` (the temporary).
    e.with_stack(8, |e, locals| {
        let removed = locals.addr();
        let temporary = removed + 4;
        e.mem.set_u32(temporary, room);
        add_ref(e, room);
        e.call(ROOM_LIST_REMOVE, &args![list, removed, temporary]);
        let pointer = e.mem.u32(removed);
        release(e, pointer);
        let pointer = e.mem.u32(temporary);
        release(e, pointer);
    });
}

// Translated from 00c5b140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a non-null portal to the head of `lPortalList` (Xbox PDB).
pub fn fn_00c5b140(e: &mut Engine, this: Ptr<BSPortalGraph>, portal: u32) {
    if portal != 0 {
        list_add(e, list_of(this, BSPortalGraph::lPortalList), portal);
    }
}

// Translated from 00c5b190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes a non-null portal from `lPortalList` (`0057c730` with the
/// address of the argument).
pub fn fn_00c5b190(e: &mut Engine, this: Ptr<BSPortalGraph>, portal: u32) {
    if portal != 0 {
        let list = list_of(this, BSPortalGraph::lPortalList);
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), portal);
            e.call(PORTAL_LIST_REMOVE, &args![list, slot]);
        });
    }
}

/// A new item holding an `NiPointer` to `object` (assigned from a
/// temporary), linked into the queue at `queue`.
fn queue_add(e: &mut Engine, queue: u32, object: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), object);
        add_ref(e, object);
        let item = e.call(LIST_NEW_ITEM, &args![queue + 8]).u32();
        e.call(
            NI_POINTER_ASSIGN,
            &args![item.wrapping_add(NiTListItem::m_element.off), slot],
        );
        e.call(POINTER_LIST_ADD_ITEM, &args![queue, item]);
        let pointer = e.mem.u32(slot.addr());
        release(e, pointer);
    });
}

/// Unlinks `item` from the queue at `queue` (`00c5bfb0`) and releases the
/// pointer it held.
fn queue_remove(e: &mut Engine, queue: u32, item: u32) {
    e.with_stack(8, |e, locals| {
        let removed = locals.addr();
        let item_slot = removed + 4;
        e.mem.set_u32(item_slot, item);
        e.call(QUEUE_REMOVE_ITEM, &args![queue, removed, item_slot]);
        let pointer = e.mem.u32(removed);
        release(e, pointer);
    });
}

// Translated from 00c5b1c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c5b1c0 - FS:[0] exception frame (unwinding of the NiPointer temporary) not translated
/// Queues a non-null object for attaching as an always-render child: under
/// the queue critical section, unless the attach queue
/// (`kAttachAlwaysRenderQueue`, Xbox PDB) already holds it, adds it there
/// and removes every entry for it from the detach queue. The search stops
/// at the matching item but tests the item after it, so a match in the
/// queue's last item counts as absent (as in the exe).
pub fn fn_00c5b1c0(e: &mut Engine, this: Ptr<BSPortalGraph>, object: u32) {
    if object == 0 {
        return;
    }
    e.call(
        ENTER_CRITICAL_SECTION,
        &args![ALWAYS_QUEUE_CRITICAL_SECTION],
    );
    let attach = list_of(this, BSPortalGraph::kAttachAlwaysRenderQueue);
    let mut item = e.mem.u32(attach + NiTPointerList::m_pkHead.off);
    while item != 0 {
        let element = e.mem.u32(item + NiTListItem::m_element.off);
        item = e.mem.u32(item + NiTListItem::m_pkNext.off);
        if element == object {
            break;
        }
    }
    if item == 0 {
        queue_add(e, attach, object);
        let detach = list_of(this, BSPortalGraph::kDetachAlwaysRenderQueue);
        let mut item = e.mem.u32(detach + NiTPointerList::m_pkHead.off);
        while item != 0 {
            let current = item;
            let element = e.mem.u32(item + NiTListItem::m_element.off);
            item = e.mem.u32(item + NiTListItem::m_pkNext.off);
            if element == object {
                queue_remove(e, detach, current);
            }
        }
    }
    e.call(
        LEAVE_CRITICAL_SECTION,
        &args![ALWAYS_QUEUE_CRITICAL_SECTION],
    );
}

// Translated from 00c5b370 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c5b370 - FS:[0] exception frame (unwinding of the NiPointer temporary) not translated
/// Queues an object (null included) for detaching: under the queue critical
/// section, adds it to the detach queue (`kDetachAlwaysRenderQueue`, Xbox
/// PDB) and removes the first entry for it from the attach queue.
pub fn fn_00c5b370(e: &mut Engine, this: Ptr<BSPortalGraph>, object: u32) {
    e.call(
        ENTER_CRITICAL_SECTION,
        &args![ALWAYS_QUEUE_CRITICAL_SECTION],
    );
    let detach = list_of(this, BSPortalGraph::kDetachAlwaysRenderQueue);
    queue_add(e, detach, object);
    let attach = list_of(this, BSPortalGraph::kAttachAlwaysRenderQueue);
    let mut item = e.mem.u32(attach + NiTPointerList::m_pkHead.off);
    while item != 0 {
        let current = item;
        let element = e.mem.u32(item + NiTListItem::m_element.off);
        item = e.mem.u32(item + NiTListItem::m_pkNext.off);
        if element == object {
            queue_remove(e, attach, current);
            break;
        }
    }
    e.call(
        LEAVE_CRITICAL_SECTION,
        &args![ALWAYS_QUEUE_CRITICAL_SECTION],
    );
}

// Translated from 00c5b4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears bit 0 of the flags (+0x30) of every object in
/// `m_kAlwaysRenderChildren` (Xbox PDB), holding a reference while it does
/// (the compiler folded the set/clear condition to "clear"), then empties
/// the array (`004dffa0`).
pub fn fn_00c5b4d0(e: &mut Engine, this: Ptr<BSPortalGraph>) {
    let array = this.at(BSPortalGraph::m_kAlwaysRenderChildren);
    let mut i = 0u32;
    while i < e.get(array, NiTArray::m_usSize) as u32 {
        let base = e.get(array, NiTArray::m_pBase);
        let object = e.mem.u32(base.wrapping_add(4 * i));
        add_ref(e, object);
        if object != 0 {
            let flags = e.mem.u32(object.wrapping_add(0x30));
            e.mem.set_u32(object.wrapping_add(0x30), flags & !1);
        }
        release(e, object);
        i = i.wrapping_add(1);
    }
    e.call(ARRAY_REMOVE_ALL, &args![array]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c5ae70, fn_00c5ae70(Ptr<BSPortalGraph>, u32)),
        entry!(0x00c5aec0, fn_00c5aec0(Ptr<BSPortalGraph>, u32)),
        entry!(0x00c5aef0, fn_00c5aef0(Ptr<BSPortalGraph>, u32)),
        entry!(0x00c5b060, fn_00c5b060(Ptr<BSPortalGraph>, u32)),
        entry!(0x00c5b140, fn_00c5b140(Ptr<BSPortalGraph>, u32)),
        entry!(0x00c5b190, fn_00c5b190(Ptr<BSPortalGraph>, u32)),
        entry!(0x00c5b1c0, fn_00c5b1c0(Ptr<BSPortalGraph>, u32)),
        entry!(0x00c5b370, fn_00c5b370(Ptr<BSPortalGraph>, u32)),
        entry!(0x00c5b4d0, fn_00c5b4d0(Ptr<BSPortalGraph>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// Doubles for the reference counting (a counter at +4) and the
    /// critical section; `DeleteThis` (slot +4) records nothing.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let n = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], n);
            ret(n)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let n = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], n);
            ret(n)
        });
        e.register(ENTER_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(0x0050_0004, |_, _| Ret::default());
        e.put_vtable(0x0090_0000, &[0, 0x0050_0004]);
        e
    }

    /// A reference-counted object with count 1.
    fn object(e: &mut Engine) -> u32 {
        let o = e.mem.alloc(0x40);
        e.mem.set_u32(o, 0x0090_0000);
        e.mem.set_u32(o + 4, 1);
        o
    }

    /// Links items holding `elements` into the list at `list`.
    fn fill(e: &mut Engine, list: u32, elements: &[u32]) -> Vec<u32> {
        let items: Vec<u32> = elements.iter().map(|_| e.mem.alloc(12)).collect();
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(item + 8, elements[i]);
            let next = items.get(i + 1).copied().unwrap_or(0);
            e.mem.set_u32(*item, next);
        }
        e.mem.set_u32(list, items.first().copied().unwrap_or(0));
        items
    }

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn occluder_and_portal_adds_link_a_new_item() {
        let mut e = engine();
        let graph: Ptr<BSPortalGraph> = e.new_object();
        let item = e.mem.alloc(12);
        e.register_double(LIST_NEW_ITEM, move |_, _| ret(item));
        e.register(LIST_ADD_ITEM, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c5_ae70, &args![graph, 0x1111u32]);
        assert_eq!(e.mem.u32(item + 8), 0x1111);
        e.call(0x00c5_b140, &args![graph, 0x2222u32]);
        assert_eq!(e.mem.u32(item + 8), 0x2222);
        e.call(0x00c5_b140, &args![graph, 0u32]);
        let log = e.call_log.take().unwrap();
        let g = graph.addr();
        assert_eq!(
            calls_to(&log, LIST_NEW_ITEM),
            vec![vec![g + 0x10], vec![g + 0x1c]]
        );
        assert_eq!(
            calls_to(&log, LIST_ADD_ITEM),
            vec![vec![g + 8, item], vec![g + 0x14, item]]
        );
    }

    #[test]
    fn occluder_and_portal_removes_pass_the_argument_address() {
        let mut e = engine();
        let graph: Ptr<BSPortalGraph> = e.new_object();
        let seen = Rc::new(RefCell::new(vec![]));
        let (a, b) = (seen.clone(), seen.clone());
        e.register_double(OCCLUDER_LIST_REMOVE, move |e, w| {
            a.borrow_mut().push((w[0], e.mem.u32(w[1])));
            Ret::default()
        });
        e.register_double(PORTAL_LIST_REMOVE, move |e, w| {
            b.borrow_mut().push((w[0], e.mem.u32(w[1])));
            Ret::default()
        });
        e.call(0x00c5_aec0, &args![graph, 0x33u32]);
        e.call(0x00c5_aec0, &args![graph, 0u32]);
        e.call(0x00c5_b190, &args![graph, 0x44u32]);
        e.call(0x00c5_b190, &args![graph, 0u32]);
        let g = graph.addr();
        assert_eq!(*seen.borrow(), vec![(g + 8, 0x33), (g + 0x14, 0x44)]);
    }

    #[test]
    fn room_add_skips_rooms_already_listed() {
        let mut e = engine();
        let graph: Ptr<BSPortalGraph> = e.new_object();
        let (listed, new) = (object(&mut e), object(&mut e));
        fill(&mut e, graph.addr() + 0x20, &[0x99, listed]);
        e.register(ROOM_LIST_ADD, |e, a| {
            // The list takes its own reference.
            let room = e.mem.u32(a[1]);
            let n = e.mem.u32(room + 4) + 1;
            e.mem.set_u32(room + 4, n);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c5_aef0, &args![graph, listed]);
        e.call(0x00c5_aef0, &args![graph, new]);
        e.call(0x00c5_aef0, &args![graph, 0u32]);
        let log = e.call_log.take().unwrap();
        let adds = calls_to(&log, ROOM_LIST_ADD);
        assert_eq!(adds.len(), 1);
        assert_eq!(adds[0][0], graph.addr() + 0x20);
        // Counts are balanced: the listed room unchanged, the new one +1.
        assert_eq!(e.mem.u32(listed + 4), 1);
        assert_eq!(e.mem.u32(new + 4), 2);
    }

    #[test]
    fn room_remove_releases_the_removed_pointer() {
        let mut e = engine();
        let graph: Ptr<BSPortalGraph> = e.new_object();
        let room = object(&mut e);
        e.mem.set_u32(room + 4, 2);
        e.register(ROOM_LIST_REMOVE, |e, a| {
            // Hands back the list's pointer (its reference moves out).
            let room = e.mem.u32(a[2]);
            e.mem.set_u32(a[1], room);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c5_b060, &args![graph, room]);
        e.call(0x00c5_b060, &args![graph, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ROOM_LIST_REMOVE).len(), 1);
        assert_eq!(calls_to(&log, ROOM_LIST_REMOVE)[0][0], graph.addr() + 0x20);
        // +1 temporary, -1 removed, -1 temporary.
        assert_eq!(e.mem.u32(room + 4), 1);
    }

    /// Doubles for the queue helpers: a new item is a fresh block, the
    /// assignment copies the pointer (taking a reference), adding records the
    /// list, removing hands back the item's pointer.
    fn queue_engine() -> Engine {
        let mut e = engine();
        e.register(LIST_NEW_ITEM, |e, _| ret(e.mem.alloc(12)));
        e.register(NI_POINTER_ASSIGN, |e, a| {
            let object = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], object);
            if object != 0 {
                let n = e.mem.u32(object + 4) + 1;
                e.mem.set_u32(object + 4, n);
            }
            Ret::default()
        });
        e.register(POINTER_LIST_ADD_ITEM, |_, _| Ret::default());
        e.register(QUEUE_REMOVE_ITEM, |e, a| {
            let item = e.mem.u32(a[2]);
            let object = e.mem.u32(item + 8);
            e.mem.set_u32(a[1], object);
            Ret::default()
        });
        e
    }

    #[test]
    fn attach_queue_add_removes_detach_entries() {
        let mut e = queue_engine();
        let graph: Ptr<BSPortalGraph> = e.new_object();
        let g = graph.addr();
        let target = object(&mut e);
        e.mem.set_u32(target + 4, 3);
        // Not in the attach queue; twice in the detach queue.
        fill(&mut e, g + 0x50, &[0x77, 0x78]);
        fill(&mut e, g + 0x5c, &[target, 0x55, target]);
        e.call_log = Some(vec![]);
        e.call(0x00c5_b1c0, &args![graph, target]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1],
            (ENTER_CRITICAL_SECTION, vec![ALWAYS_QUEUE_CRITICAL_SECTION])
        );
        assert_eq!(log.last().unwrap().0, LEAVE_CRITICAL_SECTION);
        assert_eq!(calls_to(&log, POINTER_LIST_ADD_ITEM)[0][0], g + 0x50);
        let removed: Vec<u32> = calls_to(&log, QUEUE_REMOVE_ITEM)
            .iter()
            .map(|w| {
                assert_eq!(w[0], g + 0x5c);
                w[2]
            })
            .collect();
        assert_eq!(removed.len(), 2);
        // +1 for the new item; -1 for each removed detach entry.
        assert_eq!(e.mem.u32(target + 4), 2);
    }

    #[test]
    fn attach_queue_search_misses_a_match_in_the_last_item() {
        let mut e = queue_engine();
        let graph: Ptr<BSPortalGraph> = e.new_object();
        let g = graph.addr();
        let target = object(&mut e);
        // In the first of two items: found, nothing is added.
        fill(&mut e, g + 0x50, &[target, 0x78]);
        e.call_log = Some(vec![]);
        e.call(0x00c5_b1c0, &args![graph, target]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, POINTER_LIST_ADD_ITEM).is_empty());
        // In the last item: treated as absent and added again.
        fill(&mut e, g + 0x50, &[0x78, target]);
        e.call_log = Some(vec![]);
        e.call(0x00c5_b1c0, &args![graph, target]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, POINTER_LIST_ADD_ITEM).len(), 1);
        // Null: nothing at all, not even the critical section.
        e.call_log = Some(vec![]);
        e.call(0x00c5_b1c0, &args![graph, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn detach_queue_add_removes_the_first_attach_entry() {
        let mut e = queue_engine();
        let graph: Ptr<BSPortalGraph> = e.new_object();
        let g = graph.addr();
        let target = object(&mut e);
        e.mem.set_u32(target + 4, 3);
        let attach = fill(&mut e, g + 0x50, &[0x11, target, target]);
        let unlinked = Rc::new(RefCell::new(vec![]));
        let record = unlinked.clone();
        e.register_double(QUEUE_REMOVE_ITEM, move |e, a| {
            let item = e.mem.u32(a[2]);
            record.borrow_mut().push(item);
            let object = e.mem.u32(item + 8);
            e.mem.set_u32(a[1], object);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c5_b370, &args![graph, target]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, POINTER_LIST_ADD_ITEM)[0][0], g + 0x5c);
        let removes = calls_to(&log, QUEUE_REMOVE_ITEM);
        assert_eq!(removes.len(), 1);
        assert_eq!(removes[0][0], g + 0x50);
        assert_eq!(*unlinked.borrow(), vec![attach[1]]);
        assert_eq!(e.mem.u32(target + 4), 3);
        assert_eq!(log.last().unwrap().0, LEAVE_CRITICAL_SECTION);
    }

    #[test]
    fn clear_always_render_clears_bit_zero_and_empties() {
        let mut e = engine();
        let graph: Ptr<BSPortalGraph> = e.new_object();
        let g = graph.addr();
        let (a, b) = (object(&mut e), object(&mut e));
        e.mem.set_u32(a + 0x30, 0x13);
        e.mem.set_u32(b + 0x30, 0x02);
        let elements = e.mem.alloc(12);
        e.mem.set_u32(elements, a);
        e.mem.set_u32(elements + 4, 0);
        e.mem.set_u32(elements + 8, b);
        e.mem.set_u32(g + 0x40, elements);
        e.mem.set_u16(g + 0x46, 3);
        e.register(ARRAY_REMOVE_ALL, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c5_b4d0, &args![graph]);
        assert_eq!(e.mem.u32(a + 0x30), 0x12);
        assert_eq!(e.mem.u32(b + 0x30), 0x02);
        assert_eq!(e.mem.u32(a + 4), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ARRAY_REMOVE_ALL), vec![vec![g + 0x3c]]);
    }
}
