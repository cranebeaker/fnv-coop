//! `BSShader/run_00b5a9a0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiTListItem, NiTPointerList};

// Translated from 00b5a9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends `item` at the tail of a doubly linked list (the
/// `NiTPointerList` shape: head, tail, count): the item's next becomes
/// null and its previous the old tail; the old tail (if any) links to it,
/// otherwise it also becomes the head; the count goes up by one. The
/// item's element (+8) is not touched.
pub fn fn_00b5a9a0(e: &mut Engine, this: Ptr<NiTPointerList>, item: Ptr<NiTListItem>) {
    e.set(item, NiTListItem::m_pkNext, 0);
    let tail = e.get(this, NiTPointerList::m_pkTail);
    e.set(item, NiTListItem::m_pkPrev, tail);
    let tail = e.get(this, NiTPointerList::m_pkTail);
    if tail != 0 {
        e.mem.set_u32(tail, item.addr());
        let count = e.get(this, NiTPointerList::m_uiCount).wrapping_add(1);
        e.set(this, NiTPointerList::m_uiCount, count);
        e.set(this, NiTPointerList::m_pkTail, item.addr());
        return;
    }
    let count = e.get(this, NiTPointerList::m_uiCount).wrapping_add(1);
    e.set(this, NiTPointerList::m_uiCount, count);
    e.set(this, NiTPointerList::m_pkHead, item.addr());
    e.set(this, NiTPointerList::m_pkTail, item.addr());
}

// Translated from 00b5a980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ghidra made no function here: the comparison `ShadowSceneNode`'s callers pass to the array
/// search `00b5a930` (`cdecl`, two words: the addresses of two elements): whether the two
/// words are equal (`SETZ` into a cleared `EAX`).
pub fn fn_00b5a980(e: &mut Engine, left: Ptr, right: Ptr) -> bool {
    e.mem.u32(left.addr()) == e.mem.u32(right.addr())
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b5_a9a0,
            fn_00b5a9a0(Ptr<NiTPointerList>, Ptr<NiTListItem>)
        ),
        entry!(0x00b5_a980, fn_00b5a980(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appends_at_the_tail() {
        let mut e = Engine::new();
        let list = e.new_object::<NiTPointerList>();
        let a = e.new_object::<NiTListItem>();
        let b = e.new_object::<NiTListItem>();
        e.set(a, NiTListItem::m_pkNext, 0x55);
        e.set(a, NiTListItem::m_element, 0x66);
        e.call(0x00b5_a9a0, &args![list, a]);
        assert_eq!(e.get(list, NiTPointerList::m_pkHead), a.addr());
        assert_eq!(e.get(list, NiTPointerList::m_pkTail), a.addr());
        assert_eq!(e.get(list, NiTPointerList::m_uiCount), 1);
        assert_eq!(e.get(a, NiTListItem::m_pkNext), 0);
        assert_eq!(e.get(a, NiTListItem::m_pkPrev), 0);
        assert_eq!(e.get(a, NiTListItem::m_element), 0x66);
        e.call(0x00b5_a9a0, &args![list, b]);
        assert_eq!(e.get(list, NiTPointerList::m_pkHead), a.addr());
        assert_eq!(e.get(list, NiTPointerList::m_pkTail), b.addr());
        assert_eq!(e.get(list, NiTPointerList::m_uiCount), 2);
        assert_eq!(e.get(a, NiTListItem::m_pkNext), b.addr());
        assert_eq!(e.get(b, NiTListItem::m_pkPrev), a.addr());
    }

    #[test]
    fn fn_00b5a980_compares_the_words() {
        let mut e = Engine::new();
        let a = e.mem.alloc(4);
        let b = e.mem.alloc(4);
        e.mem.set_u32(a, 0x1234);
        e.mem.set_u32(b, 0x1234);
        assert_eq!(e.call(0x00b5_a980, &args![a, b]).u32(), 1);
        e.mem.set_u32(b, 0x1235);
        assert_eq!(e.call(0x00b5_a980, &args![a, b]).u32(), 0);
    }
}
