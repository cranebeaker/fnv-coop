//! `BSMain/bsportalgraph.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BSPortalGraph` (0x78 bytes on PC, the same as the Xbox PDB) keeps the
//! portal/room lists, the "always render" array of `NiAVObject`s and two
//! queues (attach / detach) that other threads fill and the renderer drains
//! under the critical section at `01203328`. The functions here are the
//! destructor, the array's accessors, adding/removing an always-render
//! object, the queue drain and the unbound-node array.
//!
//! Reference-counted objects (`NiPointer` temporaries) are handled as the
//! exe does: `InterlockedIncrement` on the count at +4 when a non-null
//! pointer is copied and `InterlockedDecrement` when it goes out of scope,
//! calling virtual slot +4 (`DeleteThis`) when the count reaches zero.
//!
//! Functions whose disassembly has an `FS:[0]` exception frame (the
//! destructor body, the add/remove helpers, the `RemoveAt` helper and the
//! unbound-node add) are marked `// Translation incomplete:`: the unwinding
//! tables are not translated, everything else is.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleArray, NiTArray, NiTPointerList};

layout! {
    /// `BSPortalGraph` (Xbox PDB), 0x78 bytes on PC.
    pub struct BSPortalGraph: 0x78 {
        /// `lOccluderList` (Xbox PDB): an `NiTPointerList<BSOcclusionPlane *>`.
        0x08 lOccluderList: Inline<NiTPointerList>,
        /// `lPortalList` (Xbox PDB): an `NiTPointerList<BSPortal *>`.
        0x14 lPortalList: Inline<NiTPointerList>,
        /// `lMultiBoundRoomList` (Xbox PDB): a list of `NiPointer<BSMultiBoundRoom>`.
        0x20 lMultiBoundRoomList: Inline<NiTPointerList>,
        /// `spMultiBoundRoomRoot` (Xbox PDB): `NiPointer<BSMultiBoundRoom>`.
        0x2C spMultiBoundRoomRoot: Ptr,
        /// `lMultiBoundRoomAccumList` (Xbox PDB): a list of `NiPointer<BSMultiBoundRoom>`.
        0x30 lMultiBoundRoomAccumList: Inline<NiTPointerList>,
        /// `m_kAlwaysRenderChildren` (Xbox PDB): an `NiTObjectArray<NiPointer<NiAVObject>>`.
        0x3C m_kAlwaysRenderChildren: Inline<NiTArray>,
        /// `spPortalNodeRoot` (Xbox PDB): `NiPointer<NiNode>`.
        0x4C spPortalNodeRoot: Ptr,
        /// `kAttachAlwaysRenderQueue` (Xbox PDB): a list of `NiPointer<NiAVObject>`.
        0x50 kAttachAlwaysRenderQueue: Inline<NiTPointerList>,
        /// `kDetachAlwaysRenderQueue` (Xbox PDB): a list of `NiPointer<NiAVObject>`.
        0x5C kDetachAlwaysRenderQueue: Inline<NiTPointerList>,
        /// `kUnboundNodes` (Xbox PDB): a `BSSimpleArray<NiPointer<NiNode>, 1024>`.
        0x68 kUnboundNodes: Inline<BSSimpleArray>,
    }
}

/// `BSPortalGraph`'s vtable.
const VTABLE: u32 = 0x010c_2d44;
/// The `NiTArray` vtable the destructor stores in the always-render array.
const NI_TARRAY_VTABLE: u32 = 0x0101_fbcc;
/// `NiRefObject`'s vtable (the base-class state of the destructor).
const NI_REF_OBJECT_VTABLE: u32 = 0x0101_dce4;
/// `NiRefObject::ms_uiObjects`, decremented by the destructor.
const NI_REF_OBJECT_COUNT: u32 = 0x011f_4400;
/// The critical section guarding the always-render queues.
const ALWAYS_QUEUE_CRITICAL_SECTION: u32 = 0x0120_3328;
/// The size freed by the scalar deleting destructor.
const SIZE: u32 = 0x78;
/// The flag word of an always-render object (`NiAVObject` flags, +0x30);
/// bit 0 is set while the object is in the array.
const AV_OBJECT_FLAGS: u32 = 0x30;

/// `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `EnterCriticalSection` / `LeaveCriticalSection` (import slots).
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// `operator delete` with a size (`00aa1460`, cdecl: pointer, size).
const OPERATOR_DELETE_SIZED: u32 = 0x00aa_1460;

/// `00b71300` (engine map: `NiTPointerListBase<..char_P>::RemoveAll`): empties
/// the portal list.
const PORTAL_LIST_REMOVE_ALL: u32 = 0x00b7_1300;
/// `004a4650`: empties a pointer list (frees every node with `004a4930`
/// and zeroes head, tail and count).
const POINTER_LIST_CLEAR: u32 = 0x004a_4650;
/// `004ed900` (engine map: `NiTPointerListBase<..Script_P>::RemoveAll`):
/// empties the occluder list.
const OCCLUDER_LIST_REMOVE_ALL: u32 = 0x004e_d900;
/// `004dffa0` (engine map: `NiTArray<NiPointer<BSTempEffect>..>::RemoveAll`):
/// empties the always-render array.
const ARRAY_REMOVE_ALL: u32 = 0x004d_ffa0;
/// `0066b0d0(this, value)`: assigns a pointer to the `NiPointer` at `this`
/// (releases the old object, stores, adds a reference).
const NI_POINTER_ASSIGN_VALUE: u32 = 0x0066_b0d0;
/// `0087d480`: destructor of a `BSSimpleArray` of `NiPointer`s.
const SIMPLE_ARRAY_DESTRUCTOR: u32 = 0x0087_d480;
/// `00e74d40` (engine map: `NiTPointerListBase<..NiPointer<NiPSysModifier>>::RemoveAll`):
/// empties a list of `NiPointer`s (both queues).
const NI_POINTER_LIST_REMOVE_ALL: u32 = 0x00e7_4d40;
/// `00667120(base, 3)`: the vector deleting destructor `NiTArray`'s
/// storage goes through.
const ARRAY_STORAGE_DESTROY: u32 = 0x0066_7120;
/// `00a5eb20(this, &pointer)`: stores the `NiPointer` at the address passed
/// in the first empty slot of an `NiTArray` (or grows it) and returns the
/// index.
const ARRAY_ADD_FIRST_EMPTY: u32 = 0x00a5_eb20;
/// `006c61b0(this, &pointer)`: appends the `NiPointer` at the address passed
/// to a `BSSimpleArray` and returns the index.
const SIMPLE_ARRAY_PUSH: u32 = 0x006c_61b0;
/// `006e5cc0(this, &source)` (engine map: `NiPointer<..>::operator=`):
/// assigns the `NiPointer` at `source` to the one at `this`.
const NI_POINTER_ASSIGN: u32 = 0x006e_5cc0;
/// `008228f0(this, 0)`: destroys one array element in place.
const ELEMENT_DESTROY: u32 = 0x0082_28f0;
/// `0042fb60(this, destination, source, count)`: moves `count` words.
const MOVE_WORDS: u32 = 0x0042_fb60;

/// Adds a reference to `object` when it is not null.
fn add_ref(e: &mut Engine, object: u32) {
    if object != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
    }
}

/// Drops a reference to `object` when it is not null; at zero the object
/// deletes itself through virtual slot +4.
fn release(e: &mut Engine, object: u32) {
    if object != 0 {
        let count = e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)]);
        if count.u32() == 0 {
            e.vcall(object, 4, &[]);
        }
    }
}

// Translated from 00c5ac00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of `BSPortalGraph` (unnamed in the engine map): stores
/// the class vtable, empties the portal list, the multi-bound room list, the
/// occluder list and the always-render array, assigns null to the room root
/// and the portal node root, destroys the unbound-node array and empties both
/// queues; then runs the member destructors in reverse order (release of the
/// portal node root, the array's own vtable and storage, the accumulated room
/// list, release of the room root, the room list, the portal list and the
/// occluder list), stores `NiRefObject`'s vtable and decrements the object
/// count (`011f4400`).
// Translation incomplete: 00c5ac00 - SEH exception unwinding is unsupported.
pub fn fn_00c5ac00(e: &mut Engine, this: Ptr<BSPortalGraph>) {
    let t = this.addr();
    e.mem.set_u32(t, VTABLE);
    e.call(PORTAL_LIST_REMOVE_ALL, &args![t + 0x14]);
    e.call(POINTER_LIST_CLEAR, &args![t + 0x20]);
    e.call(OCCLUDER_LIST_REMOVE_ALL, &args![t + 0x08]);
    e.call(ARRAY_REMOVE_ALL, &args![t + 0x3c]);
    e.call(NI_POINTER_ASSIGN_VALUE, &args![t + 0x2c, 0u32]);
    e.call(NI_POINTER_ASSIGN_VALUE, &args![t + 0x4c, 0u32]);
    e.call(SIMPLE_ARRAY_DESTRUCTOR, &args![t + 0x68]);
    e.call(NI_POINTER_LIST_REMOVE_ALL, &args![t + 0x5c]);
    e.call(NI_POINTER_LIST_REMOVE_ALL, &args![t + 0x50]);
    // ~spPortalNodeRoot
    let portal_node_root = e.get(this, BSPortalGraph::spPortalNodeRoot);
    release(e, portal_node_root.addr());
    // ~m_kAlwaysRenderChildren: its vtable, then the storage when present.
    e.mem.set_u32(t + 0x3c, NI_TARRAY_VTABLE);
    let storage = e.mem.u32(t + 0x40);
    if storage != 0 {
        e.call(ARRAY_STORAGE_DESTROY, &args![storage, 3u32]);
    }
    e.call(POINTER_LIST_CLEAR, &args![t + 0x30]);
    // ~spMultiBoundRoomRoot
    let room_root = e.get(this, BSPortalGraph::spMultiBoundRoomRoot);
    release(e, room_root.addr());
    e.call(POINTER_LIST_CLEAR, &args![t + 0x20]);
    e.call(PORTAL_LIST_REMOVE_ALL, &args![t + 0x14]);
    e.call(OCCLUDER_LIST_REMOVE_ALL, &args![t + 0x08]);
    e.mem.set_u32(t, NI_REF_OBJECT_VTABLE);
    e.call(INTERLOCKED_DECREMENT, &args![NI_REF_OBJECT_COUNT]);
}

// Translated from 00c5b5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPortalGraph::GetAlwaysRenderAt` (Xbox PDB): the pointer at `index` of
/// the always-render array's storage (no bounds check).
pub fn bs_portal_graph_get_always_render_at(
    e: &mut Engine,
    this: Ptr<BSPortalGraph>,
    index: u32,
) -> u32 {
    let array = this.at(BSPortalGraph::m_kAlwaysRenderChildren);
    let base = e.get(array, NiTArray::m_pBase);
    e.mem.u32(base.wrapping_add(index.wrapping_mul(4)))
}

// Translated from 00c5b5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPortalGraph::GetAlwaysRenderArrayCount` (Xbox PDB): the always-render
/// array's count of non-null elements (`m_usESize`).
pub fn bs_portal_graph_get_always_render_array_count(
    e: &mut Engine,
    this: Ptr<BSPortalGraph>,
) -> u16 {
    let array = this.at(BSPortalGraph::m_kAlwaysRenderChildren);
    e.get(array, NiTArray::m_usESize)
}

// Translated from 00c5b5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: adds `node` to the always-render array unless
/// it is already in it. Walks the array's used slots (each element gets a
/// counted reference while it is compared); when no slot holds `node`, a
/// counted copy is passed to `00a5eb20`. A null `node` does nothing.
// Translation incomplete: 00c5b5f0 - SEH exception unwinding is unsupported.
pub fn fn_00c5b5f0(e: &mut Engine, this: Ptr<BSPortalGraph>, node: u32) {
    if node == 0 {
        return;
    }
    let array = this.at(BSPortalGraph::m_kAlwaysRenderChildren);
    let mut found = false;
    let mut index = 0u32;
    while index < e.get(array, NiTArray::m_usSize) as u32 {
        let base = e.get(array, NiTArray::m_pBase);
        let element = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
        add_ref(e, element);
        if element != 0 && element == node {
            found = true;
        }
        release(e, element);
        index += 1;
    }
    if !found {
        e.with_stack(4, |e, copy| {
            e.mem.set_u32(copy.addr(), node);
            add_ref(e, node);
            e.call(ARRAY_ADD_FIRST_EMPTY, &args![array, copy]);
            let held = e.mem.u32(copy.addr());
            release(e, held);
        });
    }
}

// Translated from 00c5b740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: removes `node` from the always-render array.
/// Finds the first slot holding `node` (counted reference while compared),
/// removes it with `00c5bb60`, releases the pointer that call hands back,
/// clears bit 0 of the node's flags (+0x30; the compiler folded the
/// "set" arm of the same flag expression away) and stops. Nothing happens
/// when the node is not in the array.
// Translation incomplete: 00c5b740 - SEH exception unwinding is unsupported.
pub fn fn_00c5b740(e: &mut Engine, this: Ptr<BSPortalGraph>, node: u32) {
    let array = this.at(BSPortalGraph::m_kAlwaysRenderChildren);
    let mut index = 0u32;
    while index < e.get(array, NiTArray::m_usSize) as u32 {
        let base = e.get(array, NiTArray::m_pBase);
        let element = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
        add_ref(e, element);
        if element != 0 && element == node {
            let removed = e.with_stack(4, |e, out| {
                fn_00c5bb60(e, array, out, index);
                e.mem.u32(out.addr())
            });
            release(e, removed);
            let flags = e.mem.u32(element + AV_OBJECT_FLAGS);
            e.mem.set_u32(element + AV_OBJECT_FLAGS, flags & !1);
            release(e, element);
            return;
        }
        release(e, element);
        index += 1;
    }
}

// Translated from 00c5b8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: drains the two always-render queues under the
/// critical section at `01203328`. Every non-null object of the attach
/// queue (+0x50) goes through `00c5b5f0` and then the queue is emptied
/// (`00e74d40`); every non-null object of the detach queue (+0x5c) goes
/// through `00c5b740` and that queue is emptied too.
pub fn fn_00c5b8d0(e: &mut Engine, this: Ptr<BSPortalGraph>) {
    e.call(
        ENTER_CRITICAL_SECTION,
        &args![ALWAYS_QUEUE_CRITICAL_SECTION],
    );
    let attach = this.at(BSPortalGraph::kAttachAlwaysRenderQueue);
    let mut item = e.get(attach, NiTPointerList::m_pkHead);
    while item != 0 {
        // NiTListItem: next at +0, element at +8.
        let next = e.mem.u32(item);
        let object = e.mem.u32(item + 8);
        item = next;
        if object != 0 {
            fn_00c5b5f0(e, this, object);
        }
    }
    e.call(NI_POINTER_LIST_REMOVE_ALL, &args![attach]);
    let detach = this.at(BSPortalGraph::kDetachAlwaysRenderQueue);
    let mut item = e.get(detach, NiTPointerList::m_pkHead);
    while item != 0 {
        let next = e.mem.u32(item);
        let object = e.mem.u32(item + 8);
        item = next;
        if object != 0 {
            fn_00c5b740(e, this, object);
        }
    }
    e.call(NI_POINTER_LIST_REMOVE_ALL, &args![detach]);
    e.call(
        LEAVE_CRITICAL_SECTION,
        &args![ALWAYS_QUEUE_CRITICAL_SECTION],
    );
}

// Translated from 00c5b9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: appends `node` to the unbound-node array
/// (+0x68) through `006c61b0`, which receives the address of a counted
/// `NiPointer` copy that is released afterwards. A null `node` does nothing.
// Translation incomplete: 00c5b9a0 - SEH exception unwinding is unsupported.
pub fn fn_00c5b9a0(e: &mut Engine, this: Ptr<BSPortalGraph>, node: u32) {
    if node == 0 {
        return;
    }
    let array = this.at(BSPortalGraph::kUnboundNodes);
    e.with_stack(4, |e, copy| {
        e.mem.set_u32(copy.addr(), node);
        add_ref(e, node);
        e.call(SIMPLE_ARRAY_PUSH, &args![array, copy]);
        let held = e.mem.u32(copy.addr());
        release(e, held);
    });
}

// Translated from 00c5ba50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: removes the first occurrence of `node` from the
/// unbound-node array (+0x68): scans the buffer for the pointer and, when
/// found, removes one element at that index with `00c5bef0`.
pub fn fn_00c5ba50(e: &mut Engine, this: Ptr<BSPortalGraph>, node: u32) {
    let array = this.at(BSPortalGraph::kUnboundNodes);
    let size = e.get(array, BSSimpleArray::iSize);
    let mut found = None;
    let mut index = 0u32;
    while index < size {
        let buffer = e.get(array, BSSimpleArray::pBuffer);
        if buffer != 0 && e.mem.u32(buffer.wrapping_add(index.wrapping_mul(4))) == node {
            found = Some(index);
            break;
        }
        index += 1;
    }
    if let Some(index) = found {
        fn_00c5bef0(e, array, index, 1);
    }
}

// Translated from 00c5bae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPortalGraph::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor body `00c5ac00` and, when bit 0 of `flags` is set, frees the
/// 0x78 bytes. Returns `this`.
pub fn bs_portal_graph_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSPortalGraph>,
    flags: u32,
) -> Ptr<BSPortalGraph> {
    fn_00c5ac00(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE_SIZED, &args![this, SIZE]);
    }
    this
}

// Translated from 00c5bb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: removes the pointer at `index` of an `NiTArray`
/// of `NiPointer`s by moving the last used slot over it, and stores the
/// removed pointer (counted) in `*out`; returns `out`.
///
/// An `index` at or past `m_usSize` stores null. Otherwise `m_usSize` drops
/// by one, the removed pointer is kept alive by a counted copy,
/// `006e5cc0` assigns the last slot to `index` and then null to the last
/// slot, `m_usESize` drops by one (the removed pointer is not null), and
/// `*out` receives the removed pointer with its own reference.
// Translation incomplete: 00c5bb60 - SEH exception unwinding is unsupported.
pub fn fn_00c5bb60(e: &mut Engine, this: Ptr<NiTArray>, out: Ptr, index: u32) -> Ptr {
    let size = e.get(this, NiTArray::m_usSize);
    if index >= size as u32 {
        e.mem.set_u32(out.addr(), 0);
        return out;
    }
    let size = size.wrapping_sub(1);
    e.set(this, NiTArray::m_usSize, size);
    let base = e.get(this, NiTArray::m_pBase);
    let removed = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
    add_ref(e, removed);
    // Move the (new) last slot over the removed one.
    let last = base.wrapping_add((size as u32) * 4);
    e.call(
        NI_POINTER_ASSIGN,
        &args![base.wrapping_add(index.wrapping_mul(4)), last],
    );
    // Assign a null pointer to the last slot.
    e.with_stack(4, |e, null| {
        let base = e.get(this, NiTArray::m_pBase);
        let size = e.get(this, NiTArray::m_usSize);
        e.call(
            NI_POINTER_ASSIGN,
            &args![base.wrapping_add((size as u32) * 4), null],
        );
    });
    // `removed != null` against a null temporary.
    if removed != 0 {
        let count = e.get(this, NiTArray::m_usESize);
        e.set(this, NiTArray::m_usESize, count.wrapping_sub(1));
    }
    e.mem.set_u32(out.addr(), removed);
    add_ref(e, removed);
    release(e, removed);
    out
}

// Translated from 00c5bef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: removes `count` words at `index` of a
/// `BSSimpleArray` of 4-byte elements. Destroys the `count` elements at
/// `index` (`008228f0`), moves the last `min(size - index - count, count)`
/// elements over the gap (`0042fb60`) and lowers `iSize` by `count`.
pub fn fn_00c5bef0(e: &mut Engine, this: Ptr<BSSimpleArray>, index: u32, count: u32) {
    let size = e.get(this, BSSimpleArray::iSize);
    let remaining = size.wrapping_sub(index).wrapping_sub(count);
    let moved = if remaining < count { remaining } else { count };
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let destroyed = buffer.wrapping_add(index.wrapping_mul(4));
    let mut n = 0u32;
    while n < count {
        e.call(
            ELEMENT_DESTROY,
            &args![destroyed.wrapping_add(n.wrapping_mul(4)), 0u32],
        );
        n += 1;
    }
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let size = e.get(this, BSSimpleArray::iSize);
    let source = buffer
        .wrapping_add(size.wrapping_mul(4))
        .wrapping_sub(moved.wrapping_shl(2));
    let gap = e
        .get(this, BSSimpleArray::pBuffer)
        .wrapping_add(index.wrapping_mul(4));
    e.call(MOVE_WORDS, &args![this, gap, source, moved]);
    let size = e.get(this, BSSimpleArray::iSize);
    e.set(this, BSSimpleArray::iSize, size.wrapping_sub(count));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c5ac00, fn_00c5ac00(Ptr<BSPortalGraph>)),
        entry!(
            0x00c5b5a0,
            bs_portal_graph_get_always_render_at(Ptr<BSPortalGraph>, u32) -> u32
        ),
        entry!(
            0x00c5b5d0,
            bs_portal_graph_get_always_render_array_count(Ptr<BSPortalGraph>) -> u16
        ),
        entry!(0x00c5b5f0, fn_00c5b5f0(Ptr<BSPortalGraph>, u32)),
        entry!(0x00c5b740, fn_00c5b740(Ptr<BSPortalGraph>, u32)),
        entry!(0x00c5b8d0, fn_00c5b8d0(Ptr<BSPortalGraph>)),
        entry!(0x00c5b9a0, fn_00c5b9a0(Ptr<BSPortalGraph>, u32)),
        entry!(0x00c5ba50, fn_00c5ba50(Ptr<BSPortalGraph>, u32)),
        entry!(
            0x00c5bae0,
            bs_portal_graph_scalar_deleting_destructor(
                Ptr<BSPortalGraph>,
                u32,
            ) -> Ptr<BSPortalGraph>
        ),
        entry!(0x00c5bb60, fn_00c5bb60(Ptr<NiTArray>, Ptr, u32) -> Ptr),
        entry!(0x00c5bef0, fn_00c5bef0(Ptr<BSSimpleArray>, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FAKE_VTABLE: u32 = 0x4300_0000;
    const FAKE_DELETE: u32 = 0x4400_0000;

    fn stub(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn call_order(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .skip(1)
            .map(|(a, _)| *a)
            .collect()
    }

    /// An engine with the interlocked and critical-section imports, the
    /// `NiRefObject` count mapped, and a fake delete slot that records the
    /// objects that were deleted.
    fn world() -> (Engine, Rc<RefCell<Vec<u32>>>) {
        let mut e = Engine::new();
        let imports = [
            (INTERLOCKED_INCREMENT, "InterlockedIncrement"),
            (INTERLOCKED_DECREMENT, "InterlockedDecrement"),
            (ENTER_CRITICAL_SECTION, "EnterCriticalSection"),
            (LEAVE_CRITICAL_SECTION, "LeaveCriticalSection"),
        ]
        .map(|(slot, name)| crate::exe::Import {
            slot,
            dll: "KERNEL32.dll".into(),
            name: name.into(),
        });
        assert_eq!(e.register_imports(&imports), 4);
        e.map(NI_REF_OBJECT_COUNT, 4);
        e.set_global::<u32>(NI_REF_OBJECT_COUNT, 5);
        e.put_vtable(FAKE_VTABLE, &[0, FAKE_DELETE]);
        let deleted = Rc::new(RefCell::new(Vec::new()));
        let log = deleted.clone();
        e.register_double(FAKE_DELETE, move |_, a| {
            log.borrow_mut().push(a[0]);
            Ret::default()
        });
        (e, deleted)
    }

    /// A reference-counted object with the given count.
    fn object(e: &mut Engine, count: u32) -> u32 {
        let o = e.mem.alloc(0x40);
        e.mem.set_u32(o, FAKE_VTABLE);
        e.mem.set_u32(o + 4, count);
        o
    }

    /// Registers a double at `addr` that records `(this, word the second
    /// argument points to)` of each call.
    fn recording_double(e: &mut Engine, addr: u32) -> Rc<RefCell<Vec<(u32, u32)>>> {
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(addr, move |e, a| {
            log.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        seen
    }

    fn graph(e: &mut Engine) -> Ptr<BSPortalGraph> {
        e.new_object::<BSPortalGraph>()
    }

    /// Fills the always-render array with `items` (capacity = len).
    fn fill_array(e: &mut Engine, g: Ptr<BSPortalGraph>, items: &[u32]) -> u32 {
        let array = g.at(BSPortalGraph::m_kAlwaysRenderChildren);
        let base = e.mem.alloc(4 * items.len().max(1) as u32);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(base + 4 * i as u32, *item);
        }
        e.set(array, NiTArray::m_pBase, base);
        e.set(array, NiTArray::m_usMaxSize, items.len() as u16);
        e.set(array, NiTArray::m_usSize, items.len() as u16);
        e.set(
            array,
            NiTArray::m_usESize,
            items.iter().filter(|i| **i != 0).count() as u16,
        );
        base
    }

    /// A list node chain holding `items`; returns the head.
    fn list_of(e: &mut Engine, items: &[u32]) -> u32 {
        let mut next = 0;
        for item in items.iter().rev() {
            let node = e.mem.alloc(0xc);
            e.mem.set_u32(node, next);
            e.mem.set_u32(node + 8, *item);
            next = node;
        }
        next
    }

    #[test]
    fn test_get_always_render_at_and_count() {
        let mut e = Engine::new();
        let g = graph(&mut e);
        fill_array(&mut e, g, &[0x11, 0, 0x33]);
        assert_eq!(e.call(0x00c5b5a0, &args![g, 0u32]).u32(), 0x11);
        assert_eq!(e.call(0x00c5b5a0, &args![g, 1u32]).u32(), 0);
        assert_eq!(e.call(0x00c5b5a0, &args![g, 2u32]).u32(), 0x33);
        assert_eq!(
            e.call(0x00c5b5d0, &args![g]).u32(),
            2,
            "the count is m_usESize, the non-null elements"
        );
    }

    /// Stubs for every callee of the destructor body.
    fn destructor_callees(e: &mut Engine) {
        stub(
            e,
            &[
                PORTAL_LIST_REMOVE_ALL,
                POINTER_LIST_CLEAR,
                OCCLUDER_LIST_REMOVE_ALL,
                ARRAY_REMOVE_ALL,
                NI_POINTER_ASSIGN_VALUE,
                SIMPLE_ARRAY_DESTRUCTOR,
                NI_POINTER_LIST_REMOVE_ALL,
                ARRAY_STORAGE_DESTROY,
            ],
        );
    }

    #[test]
    fn test_fn_00c5ac00_destroys_the_members_in_order() {
        let (mut e, deleted) = world();
        destructor_callees(&mut e);
        let g = graph(&mut e);
        let t = g.addr();
        let root = object(&mut e, 1);
        let portal_root = object(&mut e, 2);
        e.set(g, BSPortalGraph::spMultiBoundRoomRoot, Ptr::<()>::new(root));
        e.set(
            g,
            BSPortalGraph::spPortalNodeRoot,
            Ptr::<()>::new(portal_root),
        );
        let storage = e.mem.alloc(8);
        e.mem.set_u32(t + 0x40, storage);
        e.call_log = Some(vec![]);
        e.call(0x00c5ac00, &args![g]);
        assert_eq!(
            call_order(&e),
            vec![
                PORTAL_LIST_REMOVE_ALL,
                POINTER_LIST_CLEAR,
                OCCLUDER_LIST_REMOVE_ALL,
                ARRAY_REMOVE_ALL,
                NI_POINTER_ASSIGN_VALUE,
                NI_POINTER_ASSIGN_VALUE,
                SIMPLE_ARRAY_DESTRUCTOR,
                NI_POINTER_LIST_REMOVE_ALL,
                NI_POINTER_LIST_REMOVE_ALL,
                INTERLOCKED_DECREMENT, // ~spPortalNodeRoot
                ARRAY_STORAGE_DESTROY,
                POINTER_LIST_CLEAR,
                INTERLOCKED_DECREMENT, // ~spMultiBoundRoomRoot
                FAKE_DELETE,
                POINTER_LIST_CLEAR,
                PORTAL_LIST_REMOVE_ALL,
                OCCLUDER_LIST_REMOVE_ALL,
                INTERLOCKED_DECREMENT, // NiRefObject count
            ]
        );
        let args_of = |addr| calls_to(&e, addr);
        assert_eq!(args_of(PORTAL_LIST_REMOVE_ALL)[0], vec![t + 0x14]);
        assert_eq!(
            args_of(POINTER_LIST_CLEAR),
            vec![vec![t + 0x20], vec![t + 0x30], vec![t + 0x20]]
        );
        assert_eq!(args_of(OCCLUDER_LIST_REMOVE_ALL)[0], vec![t + 8]);
        assert_eq!(args_of(ARRAY_REMOVE_ALL), vec![vec![t + 0x3c]]);
        assert_eq!(
            args_of(NI_POINTER_ASSIGN_VALUE),
            vec![vec![t + 0x2c, 0], vec![t + 0x4c, 0]]
        );
        assert_eq!(args_of(SIMPLE_ARRAY_DESTRUCTOR), vec![vec![t + 0x68]]);
        assert_eq!(
            args_of(NI_POINTER_LIST_REMOVE_ALL),
            vec![vec![t + 0x5c], vec![t + 0x50]]
        );
        assert_eq!(args_of(ARRAY_STORAGE_DESTROY), vec![vec![storage, 3]]);
        assert_eq!(
            args_of(INTERLOCKED_DECREMENT),
            vec![
                vec![portal_root + 4],
                vec![root + 4],
                vec![NI_REF_OBJECT_COUNT]
            ]
        );
        assert_eq!(
            *deleted.borrow(),
            vec![root],
            "only the last reference deletes"
        );
        assert_eq!(e.mem.u32(portal_root + 4), 1);
        assert_eq!(e.mem.u32(t), NI_REF_OBJECT_VTABLE);
        assert_eq!(e.mem.u32(t + 0x3c), NI_TARRAY_VTABLE);
        assert_eq!(e.global::<u32>(NI_REF_OBJECT_COUNT), 4);
    }

    #[test]
    fn test_fn_00c5ac00_without_pointers_or_storage() {
        let (mut e, deleted) = world();
        destructor_callees(&mut e);
        let g = graph(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00c5ac00, &args![g]);
        assert!(calls_to(&e, ARRAY_STORAGE_DESTROY).is_empty());
        assert_eq!(
            calls_to(&e, INTERLOCKED_DECREMENT),
            vec![vec![NI_REF_OBJECT_COUNT]],
            "null NiPointers release nothing"
        );
        assert!(deleted.borrow().is_empty());
    }

    #[test]
    fn test_bs_portal_graph_scalar_deleting_destructor() {
        let (mut e, _) = world();
        destructor_callees(&mut e);
        stub(&mut e, &[OPERATOR_DELETE_SIZED]);
        let g = graph(&mut e);
        e.call_log = Some(vec![]);
        let r = e.call(0x00c5bae0, &args![g, 0u32]);
        assert_eq!(r.u32(), g.addr(), "returns this");
        assert_eq!(calls_to(&e, ARRAY_REMOVE_ALL).len(), 1, "the body runs");
        assert!(calls_to(&e, OPERATOR_DELETE_SIZED).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x00c5bae0, &args![g, 1u32]);
        assert_eq!(
            calls_to(&e, OPERATOR_DELETE_SIZED),
            vec![vec![g.addr(), 0x78]]
        );
        assert_eq!(*call_order(&e).last().unwrap(), OPERATOR_DELETE_SIZED);
    }

    #[test]
    fn test_fn_00c5b5f0_adds_a_missing_node() {
        let (mut e, deleted) = world();
        let added = recording_double(&mut e, ARRAY_ADD_FIRST_EMPTY);
        let g = graph(&mut e);
        let other = object(&mut e, 1);
        let node = object(&mut e, 1);
        fill_array(&mut e, g, &[other, 0]);
        e.call(0x00c5b5f0, &args![g, node]);
        let array = g.addr() + 0x3c;
        assert_eq!(
            *added.borrow(),
            vec![(array, node)],
            "the array gets a copy holding the node"
        );
        assert_eq!(e.mem.u32(node + 4), 1, "the copy's reference is released");
        assert_eq!(e.mem.u32(other + 4), 1);
        assert!(deleted.borrow().is_empty());
    }

    #[test]
    fn test_fn_00c5b5f0_keeps_a_present_node_and_ignores_null() {
        let (mut e, deleted) = world();
        stub(&mut e, &[ARRAY_ADD_FIRST_EMPTY]);
        let g = graph(&mut e);
        let node = object(&mut e, 1);
        fill_array(&mut e, g, &[0, node]);
        e.call_log = Some(vec![]);
        e.call(0x00c5b5f0, &args![g, node]);
        assert!(calls_to(&e, ARRAY_ADD_FIRST_EMPTY).is_empty());
        assert_eq!(e.mem.u32(node + 4), 1);
        e.call(0x00c5b5f0, &args![g, 0u32]);
        assert_eq!(
            calls_to(&e, INTERLOCKED_INCREMENT).len(),
            1,
            "null node touches nothing"
        );
        assert!(deleted.borrow().is_empty());
    }

    #[test]
    fn test_fn_00c5b5f0_a_dying_element_is_deleted() {
        let (mut e, deleted) = world();
        stub(&mut e, &[ARRAY_ADD_FIRST_EMPTY]);
        let g = graph(&mut e);
        // A stale element whose only reference is the array's (count 0 here,
        // so the temporary reference drops it back to zero).
        let stale = object(&mut e, 0);
        let node = object(&mut e, 1);
        fill_array(&mut e, g, &[stale]);
        e.call(0x00c5b5f0, &args![g, node]);
        assert_eq!(*deleted.borrow(), vec![stale]);
    }

    #[test]
    fn test_fn_00c5b740_removes_the_node_and_clears_its_flag() {
        let (mut e, deleted) = world();
        e.register(NI_POINTER_ASSIGN, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        let g = graph(&mut e);
        let first = object(&mut e, 2);
        let node = object(&mut e, 2);
        let last = object(&mut e, 2);
        e.mem.set_u32(node + AV_OBJECT_FLAGS, 0x0000_0103);
        let base = fill_array(&mut e, g, &[first, node, last]);
        e.call(0x00c5b740, &args![g, node]);
        let array = g.at(BSPortalGraph::m_kAlwaysRenderChildren);
        assert_eq!(e.get(array, NiTArray::m_usSize), 2);
        assert_eq!(e.get(array, NiTArray::m_usESize), 2);
        assert_eq!(e.mem.u32(base), first);
        assert_eq!(
            e.mem.u32(base + 4),
            last,
            "the last slot moved into the gap"
        );
        assert_eq!(e.mem.u32(base + 8), 0);
        assert_eq!(e.mem.u32(node + AV_OBJECT_FLAGS), 0x0000_0102);
        assert_eq!(e.mem.u32(node + 4), 2, "temporary references are balanced");
        assert_eq!(e.mem.u32(first + 4), 2);
        assert!(deleted.borrow().is_empty());
    }

    #[test]
    fn test_fn_00c5b740_ignores_a_node_that_is_not_there() {
        let (mut e, deleted) = world();
        stub(&mut e, &[NI_POINTER_ASSIGN]);
        let g = graph(&mut e);
        let a = object(&mut e, 1);
        let stranger = object(&mut e, 1);
        e.mem.set_u32(stranger + AV_OBJECT_FLAGS, 1);
        fill_array(&mut e, g, &[a, 0]);
        e.call_log = Some(vec![]);
        e.call(0x00c5b740, &args![g, stranger]);
        assert!(calls_to(&e, NI_POINTER_ASSIGN).is_empty());
        assert_eq!(e.mem.u32(stranger + AV_OBJECT_FLAGS), 1);
        assert_eq!(e.mem.u32(a + 4), 1);
        assert!(deleted.borrow().is_empty());
    }

    #[test]
    fn test_fn_00c5b8d0_drains_both_queues_under_the_lock() {
        let (mut e, _) = world();
        stub(&mut e, &[NI_POINTER_LIST_REMOVE_ALL]);
        let added = recording_double(&mut e, ARRAY_ADD_FIRST_EMPTY);
        let g = graph(&mut e);
        let t = g.addr();
        let n1 = object(&mut e, 1);
        let n2 = object(&mut e, 1);
        let gone = object(&mut e, 1);
        fill_array(&mut e, g, &[]);
        let attach = list_of(&mut e, &[n1, 0, n2]);
        e.mem.set_u32(t + 0x50, attach);
        let detach = list_of(&mut e, &[gone, 0]);
        e.mem.set_u32(t + 0x5c, detach);
        e.call_log = Some(vec![]);
        e.call(0x00c5b8d0, &args![g]);
        assert_eq!(
            call_order(&e)
                .into_iter()
                .filter(|a| {
                    [
                        ENTER_CRITICAL_SECTION,
                        LEAVE_CRITICAL_SECTION,
                        ARRAY_ADD_FIRST_EMPTY,
                        NI_POINTER_LIST_REMOVE_ALL,
                    ]
                    .contains(a)
                })
                .collect::<Vec<_>>(),
            vec![
                ENTER_CRITICAL_SECTION,
                ARRAY_ADD_FIRST_EMPTY,
                ARRAY_ADD_FIRST_EMPTY,
                NI_POINTER_LIST_REMOVE_ALL,
                NI_POINTER_LIST_REMOVE_ALL,
                LEAVE_CRITICAL_SECTION,
            ]
        );
        assert_eq!(
            calls_to(&e, ENTER_CRITICAL_SECTION),
            vec![vec![ALWAYS_QUEUE_CRITICAL_SECTION]]
        );
        assert_eq!(
            calls_to(&e, NI_POINTER_LIST_REMOVE_ALL),
            vec![vec![t + 0x50], vec![t + 0x5c]]
        );
        let array = t + 0x3c;
        assert_eq!(*added.borrow(), vec![(array, n1), (array, n2)]);
    }

    #[test]
    fn test_fn_00c5b8d0_detach_queue_removes_nodes() {
        let (mut e, _) = world();
        stub(&mut e, &[NI_POINTER_LIST_REMOVE_ALL]);
        e.register(NI_POINTER_ASSIGN, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        let g = graph(&mut e);
        let t = g.addr();
        let a = object(&mut e, 1);
        let b = object(&mut e, 1);
        e.mem.set_u32(b + AV_OBJECT_FLAGS, 1);
        let base = fill_array(&mut e, g, &[a, b]);
        let detach = list_of(&mut e, &[b]);
        e.mem.set_u32(t + 0x5c, detach);
        e.call(0x00c5b8d0, &args![g]);
        assert_eq!(e.mem.u32(base), a);
        assert_eq!(e.mem.u32(base + 4), 0);
        assert_eq!(e.mem.u32(b + AV_OBJECT_FLAGS), 0);
    }

    #[test]
    fn test_fn_00c5b9a0_appends_a_counted_copy() {
        let (mut e, deleted) = world();
        let pushed = recording_double(&mut e, SIMPLE_ARRAY_PUSH);
        let g = graph(&mut e);
        let node = object(&mut e, 1);
        e.call(0x00c5b9a0, &args![g, node]);
        assert_eq!(*pushed.borrow(), vec![(g.addr() + 0x68, node)]);
        assert_eq!(e.mem.u32(node + 4), 1, "the copy is released again");
        assert!(deleted.borrow().is_empty());
        e.call_log = Some(vec![]);
        e.call(0x00c5b9a0, &args![g, 0u32]);
        assert_eq!(pushed.borrow().len(), 1, "null node pushes nothing");
    }

    /// A graph whose unbound-node array holds `items`.
    fn unbound(e: &mut Engine, items: &[u32]) -> (Ptr<BSPortalGraph>, u32) {
        let g = graph(e);
        let array = g.at(BSPortalGraph::kUnboundNodes);
        let buffer = e.mem.alloc(4 * items.len().max(1) as u32);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * i as u32, *item);
        }
        e.set(array, BSSimpleArray::pBuffer, buffer);
        e.set(array, BSSimpleArray::iSize, items.len() as u32);
        (g, buffer)
    }

    #[test]
    fn test_fn_00c5ba50_removes_the_first_match() {
        let mut e = Engine::new();
        stub(&mut e, &[ELEMENT_DESTROY, MOVE_WORDS]);
        let (g, buffer) = unbound(&mut e, &[0x10, 0x20, 0x30]);
        e.call_log = Some(vec![]);
        e.call(0x00c5ba50, &args![g, 0x20u32]);
        let array = g.addr() + 0x68;
        assert_eq!(calls_to(&e, ELEMENT_DESTROY), vec![vec![buffer + 4, 0]]);
        assert_eq!(
            calls_to(&e, MOVE_WORDS),
            vec![vec![array, buffer + 4, buffer + 8, 1]]
        );
        assert_eq!(e.mem.u32(array + 8), 2);
    }

    #[test]
    fn test_fn_00c5ba50_missing_node_or_empty_array_changes_nothing() {
        let mut e = Engine::new();
        stub(&mut e, &[ELEMENT_DESTROY, MOVE_WORDS]);
        let (g, _) = unbound(&mut e, &[0x10, 0x20]);
        e.call_log = Some(vec![]);
        e.call(0x00c5ba50, &args![g, 0x99u32]);
        assert!(calls_to(&e, ELEMENT_DESTROY).is_empty());
        assert_eq!(e.mem.u32(g.addr() + 0x68 + 8), 2);
        // Size 2 but no buffer: nothing is read or removed.
        e.mem.set_u32(g.addr() + 0x68 + 4, 0);
        e.call(0x00c5ba50, &args![g, 0u32]);
        assert!(calls_to(&e, ELEMENT_DESTROY).is_empty());
    }

    #[test]
    fn test_fn_00c5bef0_moves_the_tail_over_the_gap() {
        let mut e = Engine::new();
        stub(&mut e, &[ELEMENT_DESTROY, MOVE_WORDS]);
        let (g, buffer) = unbound(&mut e, &[1, 2, 3, 4, 5, 6]);
        let array = g.addr() + 0x68;
        e.call_log = Some(vec![]);
        // size 6, index 1, count 2: 3 elements remain after the gap, so 2 move.
        e.call(0x00c5bef0, &args![array, 1u32, 2u32]);
        assert_eq!(
            calls_to(&e, ELEMENT_DESTROY),
            vec![vec![buffer + 4, 0], vec![buffer + 8, 0]]
        );
        assert_eq!(
            calls_to(&e, MOVE_WORDS),
            vec![vec![array, buffer + 4, buffer + 24 - 8, 2]]
        );
        assert_eq!(e.mem.u32(array + 8), 4);
    }

    #[test]
    fn test_fn_00c5bef0_moves_fewer_when_little_is_left() {
        let mut e = Engine::new();
        stub(&mut e, &[ELEMENT_DESTROY, MOVE_WORDS]);
        let (g, buffer) = unbound(&mut e, &[1, 2, 3, 4]);
        let array = g.addr() + 0x68;
        e.call_log = Some(vec![]);
        // size 4, index 2, count 2: nothing remains after the gap.
        e.call(0x00c5bef0, &args![array, 2u32, 2u32]);
        assert_eq!(
            calls_to(&e, MOVE_WORDS),
            vec![vec![array, buffer + 8, buffer + 16, 0]]
        );
        assert_eq!(e.mem.u32(array + 8), 2);
        // size 2, index 1, count 1: the removed element is the last one.
        e.call_log = Some(vec![]);
        e.call(0x00c5bef0, &args![array, 1u32, 1u32]);
        assert_eq!(
            calls_to(&e, MOVE_WORDS),
            vec![vec![array, buffer + 4, buffer + 8, 0]]
        );
        assert_eq!(e.mem.u32(array + 8), 1);
    }

    #[test]
    fn test_fn_00c5bb60_remove_at_moves_the_last_slot() {
        let (mut e, deleted) = world();
        e.register(NI_POINTER_ASSIGN, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        let g = graph(&mut e);
        let first = object(&mut e, 1);
        let second = object(&mut e, 1);
        let third = object(&mut e, 1);
        let base = fill_array(&mut e, g, &[first, second, third]);
        let array = g.at(BSPortalGraph::m_kAlwaysRenderChildren);
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0xdead_beef);
        let r = e.call(0x00c5bb60, &args![array, out, 0u32]);
        assert_eq!(r.u32(), out);
        assert_eq!(e.mem.u32(out), first);
        assert_eq!(e.mem.u32(base), third);
        assert_eq!(e.mem.u32(base + 4), second);
        assert_eq!(e.mem.u32(base + 8), 0);
        assert_eq!(e.get(array, NiTArray::m_usSize), 2);
        assert_eq!(e.get(array, NiTArray::m_usESize), 2);
        assert_eq!(
            e.mem.u32(first + 4),
            2,
            "the caller's pointer holds its own reference"
        );
        assert!(deleted.borrow().is_empty());
    }

    #[test]
    fn test_fn_00c5bb60_index_past_the_end_gives_null() {
        let (mut e, _) = world();
        e.register(NI_POINTER_ASSIGN, |_, _| panic!("nothing is moved"));
        let g = graph(&mut e);
        let first = object(&mut e, 1);
        fill_array(&mut e, g, &[first]);
        let array = g.at(BSPortalGraph::m_kAlwaysRenderChildren);
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0xdead_beef);
        e.call(0x00c5bb60, &args![array, out, 1u32]);
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(e.get(array, NiTArray::m_usSize), 1);
        assert_eq!(e.get(array, NiTArray::m_usESize), 1);
    }

    #[test]
    fn test_fn_00c5bb60_a_null_slot_keeps_the_element_count() {
        let (mut e, _) = world();
        stub(&mut e, &[NI_POINTER_ASSIGN]);
        let g = graph(&mut e);
        let item = object(&mut e, 1);
        fill_array(&mut e, g, &[0, item]);
        let array = g.at(BSPortalGraph::m_kAlwaysRenderChildren);
        let out = e.mem.alloc(4);
        e.call(0x00c5bb60, &args![array, out, 0u32]);
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(e.get(array, NiTArray::m_usSize), 1);
        assert_eq!(
            e.get(array, NiTArray::m_usESize),
            1,
            "a null slot is not counted"
        );
    }
}
