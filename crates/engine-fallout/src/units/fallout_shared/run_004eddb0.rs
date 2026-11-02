//! `fallout shared/run_004eddb0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Gamebryo container templates the compiler emitted here: the deleting
//! destructors and constructors of `NiTArray<BSTextureManager::eTextureType,
//! NiTMallocInterface<..>>` and `NiTArray<NiTexturingProperty::FilterMode,
//! NiTMallocInterface<..>>` and of their `NiTPrimitiveArray` subclasses
//! (class names from the RTTI type descriptors of the vtables at
//! `010232b4`, `010232bc`, `010232c4`, `010232cc`), the malloc interface's
//! free, and two `NiTPointerList` node operations.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiTArray, NiTListItem, NiTPointerList};

/// Deallocation (`00401030`, cdecl, pointer).
const DEALLOCATE: u32 = 0x0040_1030;
/// Frees a block through the memory manager (`00aa10f0`, cdecl, pointer).
const FREE: u32 = 0x00aa_10f0;
/// Allocates `count` 4-byte elements (`0096afc0`, cdecl, count).
const ALLOCATE_ELEMENTS: u32 = 0x0096_afc0;
/// `NiTArray<eTextureType, NiTMallocInterface<eTextureType>>` vtable.
const TEXTURE_TYPE_ARRAY_VTABLE: u32 = 0x0102_32b4;
/// `NiTArray<FilterMode, NiTMallocInterface<FilterMode>>` vtable.
const FILTER_MODE_ARRAY_VTABLE: u32 = 0x0102_32c4;

/// A deleting destructor: `destructor` on `this`, then `00401030` frees
/// `this` when bit 0 of `flags` is set. Returns `this`.
fn deleting_destructor(e: &mut Engine, destructor: u32, this: Ptr, flags: u32) -> Ptr {
    e.call(destructor, &args![this]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

/// An `NiTArray` constructor with its vtable: `m_usMaxSize = max_size`,
/// `m_usGrowBy = grow_by`, `m_usSize` and `m_usESize` 0, and `m_pBase` a new
/// block of `max_size` elements (`0096afc0`) or null when `max_size` is 0.
fn array_constructor(
    e: &mut Engine,
    vtable: u32,
    this: Ptr<NiTArray>,
    max_size: u16,
    grow_by: u16,
) -> Ptr<NiTArray> {
    e.mem.set_u32(this.addr(), vtable);
    e.set(this, NiTArray::m_usMaxSize, max_size);
    e.set(this, NiTArray::m_usGrowBy, grow_by);
    e.set(this, NiTArray::m_usSize, 0);
    e.set(this, NiTArray::m_usESize, 0);
    let max_size = e.get(this, NiTArray::m_usMaxSize);
    // `MOVZX`, `TEST`, `JLE`: positive, so any non-zero count.
    let base = if max_size != 0 {
        e.call(ALLOCATE_ELEMENTS, &args![max_size as u32]).u32()
    } else {
        0
    };
    e.set(this, NiTArray::m_pBase, base);
    this
}

// Translated from 004eddb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The deleting destructor of `NiTArray<eTextureType, ..>` (vtable
/// `010232b4` slot 0): its destructor `004edc60`, then the free.
pub fn fn_004eddb0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    deleting_destructor(e, 0x004e_dc60, this, flags)
}

// Translated from 004edde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The deleting destructor of `NiTPrimitiveArray<eTextureType>` (vtable
/// `010232bc` slot 0): its destructor `004ebb30`, then the free.
pub fn fn_004edde0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    deleting_destructor(e, 0x004e_bb30, this, flags)
}

// Translated from 004ede10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The deleting destructor of `NiTArray<FilterMode, ..>` (vtable
/// `010232c4` slot 0): its destructor `004edcc0`, then the free.
pub fn fn_004ede10(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    deleting_destructor(e, 0x004e_dcc0, this, flags)
}

// Translated from 004ede40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The deleting destructor of `NiTPrimitiveArray<FilterMode>` (vtable
/// `010232cc` slot 0): its destructor `004ebb50`, then the free.
pub fn fn_004ede40(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    deleting_destructor(e, 0x004e_bb50, this, flags)
}

// Translated from 004ede70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The malloc interface's free (the arrays' destructors pass `m_pBase`):
/// `00aa10f0` on `block`.
pub fn fn_004ede70(e: &mut Engine, block: u32) {
    e.call(FREE, &args![block]);
}

// Translated from 004ede90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Links `node` into the list before `position` (which must be a node of
/// the list): the node's next is `position`, its previous is `position`'s
/// previous (or it becomes the head), `position`'s previous becomes the
/// node, and the count goes up. Returns `node`.
pub fn fn_004ede90(e: &mut Engine, this: Ptr<NiTPointerList>, position: u32, node: u32) -> u32 {
    let position = Ptr::<NiTListItem>::new(position);
    let item = Ptr::<NiTListItem>::new(node);
    e.set(item, NiTListItem::m_pkNext, position.addr());
    let previous = e.get(position, NiTListItem::m_pkPrev);
    e.set(item, NiTListItem::m_pkPrev, previous);
    if previous == 0 {
        e.set(this, NiTPointerList::m_pkHead, node);
    } else {
        e.set(
            Ptr::<NiTListItem>::new(previous),
            NiTListItem::m_pkNext,
            node,
        );
    }
    e.set(position, NiTListItem::m_pkPrev, node);
    let count = e.get(this, NiTPointerList::m_uiCount);
    e.set(this, NiTPointerList::m_uiCount, count.wrapping_add(1));
    node
}

// Translated from 004edf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the list's tail node and returns its element: unlinks the tail
/// (the new tail's next, or the head when the list becomes empty, set to
/// null), frees the node (`004a49e0`), decrements the count. The list must
/// not be empty.
pub fn fn_004edf00(e: &mut Engine, this: Ptr<NiTPointerList>) -> u32 {
    let node = e.get(this, NiTPointerList::m_pkTail);
    let previous = e.get(Ptr::<NiTListItem>::new(node), NiTListItem::m_pkPrev);
    e.set(this, NiTPointerList::m_pkTail, previous);
    if previous == 0 {
        e.set(this, NiTPointerList::m_pkHead, 0);
    } else {
        e.set(Ptr::<NiTListItem>::new(previous), NiTListItem::m_pkNext, 0);
    }
    let element = e.get(Ptr::<NiTListItem>::new(node), NiTListItem::m_element);
    e.call(0x004a_49e0, &args![this, node]);
    let count = e.get(this, NiTPointerList::m_uiCount);
    e.set(this, NiTPointerList::m_uiCount, count.wrapping_sub(1));
    element
}

// Translated from 004edf70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTArray<eTextureType, ..>` constructor (vtable `010232b4`).
/// Returns `this`.
pub fn fn_004edf70(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u16,
    grow_by: u16,
) -> Ptr<NiTArray> {
    array_constructor(e, TEXTURE_TYPE_ARRAY_VTABLE, this, max_size, grow_by)
}

// Translated from 004edfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTArray<FilterMode, ..>` constructor (vtable `010232c4`).
/// Returns `this`.
pub fn fn_004edfe0(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u16,
    grow_by: u16,
) -> Ptr<NiTArray> {
    array_constructor(e, FILTER_MODE_ARRAY_VTABLE, this, max_size, grow_by)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004eddb0, fn_004eddb0(Ptr, u32) -> Ptr),
        entry!(0x004edde0, fn_004edde0(Ptr, u32) -> Ptr),
        entry!(0x004ede10, fn_004ede10(Ptr, u32) -> Ptr),
        entry!(0x004ede40, fn_004ede40(Ptr, u32) -> Ptr),
        entry!(0x004ede70, fn_004ede70(u32)),
        entry!(
            0x004ede90,
            fn_004ede90(Ptr<NiTPointerList>, u32, u32) -> u32
        ),
        entry!(0x004edf00, fn_004edf00(Ptr<NiTPointerList>) -> u32),
        entry!(
            0x004edf70,
            fn_004edf70(Ptr<NiTArray>, u16, u16) -> Ptr<NiTArray>
        ),
        entry!(
            0x004edfe0,
            fn_004edfe0(Ptr<NiTArray>, u16, u16) -> Ptr<NiTArray>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type DeletingDestructor = fn(&mut Engine, Ptr, u32) -> Ptr;
    type ArrayConstructor = fn(&mut Engine, Ptr<NiTArray>, u16, u16) -> Ptr<NiTArray>;

    fn quiet_double(e: &mut Engine, addr: u32) {
        e.register(addr, |_, _| Ret::default());
    }

    #[test]
    fn deleting_destructors_free_on_bit_zero() {
        let mut e = Engine::new();
        let cases: [(DeletingDestructor, u32); 4] = [
            (fn_004eddb0, 0x004e_dc60),
            (fn_004edde0, 0x004e_bb30),
            (fn_004ede10, 0x004e_dcc0),
            (fn_004ede40, 0x004e_bb50),
        ];
        for (function, destructor) in cases {
            quiet_double(&mut e, destructor);
            quiet_double(&mut e, DEALLOCATE);
            e.call_log = Some(vec![]);
            assert_eq!(function(&mut e, Ptr::new(0x800), 0).addr(), 0x800);
            assert_eq!(e.call_log.take().unwrap(), vec![(destructor, vec![0x800])]);
            e.call_log = Some(vec![]);
            function(&mut e, Ptr::new(0x800), 3);
            assert_eq!(
                e.call_log.take().unwrap(),
                vec![(destructor, vec![0x800]), (DEALLOCATE, vec![0x800])]
            );
        }
    }

    #[test]
    fn malloc_interface_free() {
        let mut e = Engine::new();
        quiet_double(&mut e, FREE);
        e.call_log = Some(vec![]);
        fn_004ede70(&mut e, 0x1234);
        assert_eq!(e.call_log.take().unwrap(), vec![(FREE, vec![0x1234])]);
    }

    fn node(e: &mut Engine, next: u32, previous: u32, element: u32) -> u32 {
        let n = e.mem.alloc(0xc);
        e.mem.set_u32(n, next);
        e.mem.set_u32(n + 4, previous);
        e.mem.set_u32(n + 8, element);
        n
    }

    #[test]
    fn insert_before_links_the_node() {
        let mut e = Engine::new();
        let list = e.new_object::<NiTPointerList>();
        let only = node(&mut e, 0, 0, 1);
        e.set(list, NiTPointerList::m_pkHead, only);
        e.set(list, NiTPointerList::m_pkTail, only);
        e.set(list, NiTPointerList::m_uiCount, 1);

        // Before the head: becomes the head.
        let first = node(&mut e, 0, 0, 2);
        assert_eq!(fn_004ede90(&mut e, list, only, first), first);
        assert_eq!(e.get(list, NiTPointerList::m_pkHead), first);
        assert_eq!(e.mem.u32(first), only);
        assert_eq!(e.mem.u32(first + 4), 0);
        assert_eq!(e.mem.u32(only + 4), first);
        assert_eq!(e.get(list, NiTPointerList::m_uiCount), 2);

        // In the middle: the previous node links to it.
        let middle = node(&mut e, 0, 0, 3);
        fn_004ede90(&mut e, list, only, middle);
        assert_eq!(e.get(list, NiTPointerList::m_pkHead), first);
        assert_eq!(e.mem.u32(first), middle);
        assert_eq!(e.mem.u32(middle), only);
        assert_eq!(e.mem.u32(middle + 4), first);
        assert_eq!(e.mem.u32(only + 4), middle);
        assert_eq!(e.get(list, NiTPointerList::m_uiCount), 3);
    }

    #[test]
    fn pop_tail_returns_the_element() {
        let mut e = Engine::new();
        quiet_double(&mut e, 0x004a_49e0);
        let list = e.new_object::<NiTPointerList>();
        let a = node(&mut e, 0, 0, 0xaa);
        let b = node(&mut e, 0, a, 0xbb);
        e.mem.set_u32(a, b);
        e.set(list, NiTPointerList::m_pkHead, a);
        e.set(list, NiTPointerList::m_pkTail, b);
        e.set(list, NiTPointerList::m_uiCount, 2);

        e.call_log = Some(vec![]);
        assert_eq!(fn_004edf00(&mut e, list), 0xbb);
        assert_eq!(
            e.call_log.take().unwrap(),
            vec![(0x004a_49e0, vec![list.addr(), b])]
        );
        assert_eq!(e.get(list, NiTPointerList::m_pkTail), a);
        assert_eq!(e.mem.u32(a), 0);
        assert_eq!(e.get(list, NiTPointerList::m_uiCount), 1);

        assert_eq!(fn_004edf00(&mut e, list), 0xaa);
        assert_eq!(e.get(list, NiTPointerList::m_pkHead), 0);
        assert_eq!(e.get(list, NiTPointerList::m_pkTail), 0);
        assert_eq!(e.get(list, NiTPointerList::m_uiCount), 0);
    }

    #[test]
    fn array_constructors() {
        let mut e = Engine::new();
        e.register(ALLOCATE_ELEMENTS, |_, a| Ret {
            eax: 0x9000 + a[0],
            ..Ret::default()
        });
        let cases: [(ArrayConstructor, u32); 2] = [
            (fn_004edf70, TEXTURE_TYPE_ARRAY_VTABLE),
            (fn_004edfe0, FILTER_MODE_ARRAY_VTABLE),
        ];
        for (function, vtable) in cases {
            let array = e.new_object::<NiTArray>();
            e.set(array, NiTArray::m_usSize, 9);
            e.set(array, NiTArray::m_usESize, 9);
            assert_eq!(function(&mut e, array, 0x8000, 4), array);
            assert_eq!(e.mem.u32(array.addr()), vtable);
            assert_eq!(e.get(array, NiTArray::m_usMaxSize), 0x8000);
            assert_eq!(e.get(array, NiTArray::m_usGrowBy), 4);
            assert_eq!(e.get(array, NiTArray::m_usSize), 0);
            assert_eq!(e.get(array, NiTArray::m_usESize), 0);
            assert_eq!(e.get(array, NiTArray::m_pBase), 0x9000 + 0x8000);

            e.call_log = Some(vec![]);
            e.set(array, NiTArray::m_pBase, 5);
            function(&mut e, array, 0, 1);
            assert!(e.call_log.take().unwrap().is_empty());
            assert_eq!(e.get(array, NiTArray::m_pBase), 0);
        }
    }
}
