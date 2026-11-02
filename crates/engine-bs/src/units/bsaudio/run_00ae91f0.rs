//! `BSAudio/run_00ae91f0` (Xbox PDB source unit), subsystem `BSAudio`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A run of unoptimised helpers next to `BSGameSound`: a hash map's
//! remove-all, a smart-pointer release, and `BSGameSound`'s constructor and
//! destructor bodies.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiRefObject, NiTPointerList, NiTPointerMap};

/// `InterlockedDecrement` (KERNEL32 import slot).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `BSGameSound`'s vtable.
const BS_GAME_SOUND_VTABLE: u32 = 0x010a_3964;
/// `BSGameSound`'s field reset (`00ae95e0`, `units/bsaudio/bsgamesound.rs`).
const BS_GAME_SOUND_RESET: u32 = 0x00ae_95e0;
/// The removal of every item of the `QueuedMessages` list (`00ae8370`,
/// `this` = the list).
const QUEUED_MESSAGES_REMOVE_ALL: u32 = 0x00ae_8370;
/// `BSGameSound::QueuedMessages` (Xbox PDB): an `NiTPointerList` at +0x18C.
const QUEUED_MESSAGES: u32 = 0x18c;

// Translated from 00ae91f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A hash map's remove-all (the `NiTMapBase` shape, shared by many map
/// types): for every bucket, unlinks its items one by one (each item's
/// first word is the next item) and hands each to the map's virtual slots
/// +0x10 and then +0x18 (in the Xbox PDB's `NiTMapBase` order the value
/// clear and the item free; not confirmed here); then the item count is 0.
pub fn fn_00ae91f0(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    let mut bucket = 0u32;
    while bucket < e.get(this, NiTPointerMap::m_uiHashSize) {
        loop {
            let slot = e
                .get(this, NiTPointerMap::m_ppkHashTable)
                .wrapping_add(bucket.wrapping_mul(4));
            let item = e.mem.u32(slot);
            if item == 0 {
                break;
            }
            let next = e.mem.u32(item);
            let slot = e
                .get(this, NiTPointerMap::m_ppkHashTable)
                .wrapping_add(bucket.wrapping_mul(4));
            e.mem.set_u32(slot, next);
            e.vcall(this.addr(), 0x10, &args![item]);
            e.vcall(this.addr(), 0x18, &args![item]);
        }
        bucket = bucket.wrapping_add(1);
    }
    e.set(this, NiTPointerMap::m_uiCount, 0);
}

// Translated from 00ae9290 (decompiled, FalloutNV.exe 1.4.0.525)
/// A smart pointer's release (`NiPointer`-style, `this` = the pointer
/// slot): when it holds an object, decrements the object's reference count
/// (`InterlockedDecrement` on +4) and, at zero, calls the object's virtual
/// slot +4 (its deleting method, no arguments). The slot is not cleared.
pub fn fn_00ae9290(e: &mut Engine, this: Ptr) {
    if e.mem.u32(this.addr()) != 0 {
        let object = Ptr::<NiRefObject>::new(e.mem.u32(this.addr()));
        let count = object.byte_add(NiRefObject::m_uiRefCount.off);
        let left = e.call(INTERLOCKED_DECREMENT, &args![count]).u32();
        if left == 0 {
            e.vcall(object.addr(), 0x4, &[]);
        }
    }
}

// Translated from 00ae9360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSGameSound`'s constructor body: sets the vtable (`010a3964`), empties
/// the `QueuedMessages` list (count, head, tail), resets the fields
/// (`00ae95e0`) and returns `this`.
// Translation incomplete: 00ae9360 - SEH exception frame (FS:[0]) for the member list's unwinding is not modelled.
pub fn fn_00ae9360(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), BS_GAME_SOUND_VTABLE);
    let list = Ptr::<NiTPointerList>::new(this.addr().wrapping_add(QUEUED_MESSAGES));
    e.set(list, NiTPointerList::m_uiCount, 0);
    e.set(list, NiTPointerList::m_pkHead, 0);
    e.set(list, NiTPointerList::m_pkTail, 0);
    e.call(BS_GAME_SOUND_RESET, &args![this]);
    this
}

// Translated from 00ae93f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSGameSound`'s destructor body: sets the vtable (`010a3964`) and
/// removes every queued message (`00ae8370` on the list at +0x18C).
// Translation incomplete: 00ae93f0 - SEH exception frame (FS:[0]) for the member list's unwinding is not modelled.
pub fn fn_00ae93f0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), BS_GAME_SOUND_VTABLE);
    let list = this.addr().wrapping_add(QUEUED_MESSAGES);
    e.call(QUEUED_MESSAGES_REMOVE_ALL, &args![list]);
}

// Translation incomplete: 00ae92d0 - SEH exception frame (FS:[0], with its /GS cookie word) for the member list's unwinding is not modelled.
// Translated from 00ae92d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no code references it): a `BSGameSound` constructor
/// (`thiscall`, one stack word it does not read, `RET 4`) with the body of `00ae9360`: sets
/// the vtable (`010a3964`), empties the `QueuedMessages` list (count, head, tail), resets
/// the fields (`00ae95e0`) and returns `this`.
pub fn fn_00ae92d0(e: &mut Engine, this: Ptr, _unused_1: u32) -> Ptr {
    e.mem.set_u32(this.addr(), BS_GAME_SOUND_VTABLE);
    let list = Ptr::<NiTPointerList>::new(this.addr().wrapping_add(QUEUED_MESSAGES));
    e.set(list, NiTPointerList::m_uiCount, 0);
    e.set(list, NiTPointerList::m_pkHead, 0);
    e.set(list, NiTPointerList::m_pkTail, 0);
    e.call(BS_GAME_SOUND_RESET, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ae_92d0, fn_00ae92d0(Ptr, u32) -> Ptr),
        entry!(0x00ae_91f0, fn_00ae91f0(Ptr<NiTPointerMap>)),
        entry!(0x00ae_9290, fn_00ae9290(Ptr)),
        entry!(0x00ae_9360, fn_00ae9360(Ptr) -> Ptr),
        entry!(0x00ae_93f0, fn_00ae93f0(Ptr)),
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

    #[test]
    fn remove_all_hands_every_item_to_both_slots() {
        let mut e = Engine::new();
        e.register(0x0091_0010, |_, _| Ret::default());
        e.register(0x0091_0018, |_, _| Ret::default());
        e.put_vtable(0x0091_1000, &[0, 0, 0, 0, 0x0091_0010, 0, 0x0091_0018]);
        let map = e.new_object::<NiTPointerMap>();
        e.mem.set_u32(map.addr(), 0x0091_1000);
        let table = e.mem.alloc(12);
        let a = e.mem.alloc(12);
        let b = e.mem.alloc(12);
        let c = e.mem.alloc(12);
        e.mem.set_u32(a, b);
        e.mem.set_u32(table, a);
        e.mem.set_u32(table + 8, c);
        e.set(map, NiTPointerMap::m_uiHashSize, 3);
        e.set(map, NiTPointerMap::m_ppkHashTable, table);
        e.set(map, NiTPointerMap::m_uiCount, 3);
        e.call_log = Some(vec![]);
        e.call(0x00ae_91f0, &args![map]);
        let m = map.addr();
        assert_eq!(
            calls_to(&e, 0x0091_0010),
            vec![vec![m, a], vec![m, b], vec![m, c]]
        );
        assert_eq!(
            calls_to(&e, 0x0091_0018),
            vec![vec![m, a], vec![m, b], vec![m, c]]
        );
        assert_eq!(e.mem.u32(table), 0);
        assert_eq!(e.mem.u32(table + 8), 0);
        assert_eq!(e.get(map, NiTPointerMap::m_uiCount), 0);
    }

    #[test]
    fn release_deletes_at_zero() {
        let mut e = Engine::new();
        // The import slot is filled from the exe's import table at run
        // time; here a double decrements.
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            Ret {
                eax: v,
                ..Ret::default()
            }
        });
        e.register(0x0091_2004, |_, _| Ret::default());
        e.put_vtable(0x0091_3000, &[0, 0x0091_2004]);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0091_3000);
        e.mem.set_u32(object + 4, 2);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, object);
        e.call_log = Some(vec![]);
        e.call(0x00ae_9290, &args![slot]);
        assert_eq!(e.mem.u32(object + 4), 1);
        assert!(calls_to(&e, 0x0091_2004).is_empty());
        e.call(0x00ae_9290, &args![slot]);
        assert_eq!(e.mem.u32(object + 4), 0);
        assert_eq!(calls_to(&e, 0x0091_2004), vec![vec![object]]);
        assert_eq!(e.mem.u32(slot), object);
        // An empty slot: nothing.
        let empty = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(0x00ae_9290, &args![empty]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn constructor_and_destructor_bodies() {
        let mut e = Engine::new();
        e.register(BS_GAME_SOUND_RESET, |_, _| Ret::default());
        e.register(QUEUED_MESSAGES_REMOVE_ALL, |_, _| Ret::default());
        let sound = e.mem.alloc(0x198);
        e.mem.write(sound + 0x18c, &[0xffu8; 12]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ae_9360, &args![sound]).u32(), sound);
        assert_eq!(e.mem.u32(sound), BS_GAME_SOUND_VTABLE);
        assert_eq!(e.mem.bytes(sound + 0x18c, 12), vec![0u8; 12]);
        assert_eq!(calls_to(&e, BS_GAME_SOUND_RESET), vec![vec![sound]]);
        e.mem.set_u32(sound, 0);
        e.call(0x00ae_93f0, &args![sound]);
        assert_eq!(e.mem.u32(sound), BS_GAME_SOUND_VTABLE);
        assert_eq!(
            calls_to(&e, QUEUED_MESSAGES_REMOVE_ALL),
            vec![vec![sound + 0x18c]]
        );
    }
}

#[cfg(test)]
mod n082_tests {
    use super::*;

    #[test]
    fn fn_00ae92d0_constructs_like_the_constructor_body() {
        let mut e = Engine::new();
        e.register(BS_GAME_SOUND_RESET, |_, _| Ret::default());
        let sound = e.mem.alloc(0x198);
        e.mem.write(sound + 0x18c, &[0xffu8; 12]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ae_92d0, &args![sound, 0x77u32]).u32(), sound);
        assert_eq!(e.mem.u32(sound), BS_GAME_SOUND_VTABLE);
        assert_eq!(e.mem.bytes(sound + 0x18c, 12), vec![0u8; 12]);
        let resets: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == BS_GAME_SOUND_RESET)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(resets, vec![vec![sound]]);
    }
}
