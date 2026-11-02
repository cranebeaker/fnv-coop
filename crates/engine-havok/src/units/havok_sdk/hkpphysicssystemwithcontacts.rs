//! `Havok SDK/hkpphysicssystemwithcontacts.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The object extends an `hkpPhysicsSystem` (0x44 bytes) with an array of referenced objects
//! (data `+0x44`, size `+0x48`, capacity word `+0x4c`: the low 30 bits are the capacity, bit 31
//! means the storage is not owned).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserveMore` (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkReferencedObject::addReference` (`00c90510`) and `removeReference` (`00c905b0`), both
/// with `this` only.
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// The `hkpPhysicsSystem` copy function the copy runs first (`00d06290`, `this`, the source) and
/// the base destructor (`00d06640`, `this` only).
const SYSTEM_COPY: u32 = 0x00d0_6290;
const SYSTEM_BASE_DESTRUCT: u32 = 0x00d0_6640;
/// The vtable the destructor sets (`010d07f4`).
const VTABLE: u32 = 0x010d_07f4;
/// The array of referenced objects.
const CONTACTS: u32 = 0x44;

// Translated from 00dceb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhysicsSystemWithContacts::clone` (engine map, `__fastcall`): answers null (the class
/// cannot be cloned).
pub fn hkp_physics_system_with_contacts_clone(_e: &mut Engine, _this: Ptr) -> u32 {
    0
}

// Translated from 00dcebd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (an assignment): `ECX` = this system and the source system on the stack. Runs the
/// `hkpPhysicsSystem` copy (`00d06290`) on it, then makes the array of referenced objects the
/// size of the source's and copies its words. When this array's capacity (low 30 bits of the
/// capacity word) is smaller than the source's size it is replaced: the old storage goes back
/// to the allocator (slot `0x10`, `(data, capacity * 4)`) unless the capacity word has bit 31
/// set, new storage is asked from slot `0xc` (the size in bytes, `size * 4`, in a stack word
/// that receives the size really given), and the capacity becomes that size divided by 4
/// (rounding toward zero).
pub fn fn_00dcebd0(e: &mut Engine, this: Ptr, source: Ptr) {
    let (me, source) = (this.addr(), source.addr());
    e.call(SYSTEM_COPY, &args![this, source]);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity_word = e.mem.u32(me + CONTACTS + 8);
    let capacity = (capacity_word & 0x3fff_ffff) as i32;
    if capacity < e.mem.i32(source + CONTACTS + 4) {
        if (capacity_word as i32) >= 0 {
            let data = e.mem.u32(me + CONTACTS);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity as u32).wrapping_mul(4)],
            );
        }
        let bytes = e.mem.u32(source + CONTACTS + 4).wrapping_mul(4);
        let (block, given) = e.with_stack(4, |e, size| {
            e.mem.set_u32(size.addr(), bytes);
            let block = e.vcall(allocator, 0xc, &args![size]).u32();
            (block, e.mem.i32(size.addr()))
        });
        e.mem.set_u32(me + CONTACTS, block);
        e.mem.set_i32(
            me + CONTACTS + 8,
            given.wrapping_add((given >> 31) & 3) >> 2,
        );
    }
    let count = e.mem.i32(source + CONTACTS + 4);
    let to = e.mem.u32(me + CONTACTS);
    e.mem.set_i32(me + CONTACTS + 4, count);
    let from = e.mem.u32(source + CONTACTS);
    for index in 0..count.max(0) as u32 {
        let word = e.mem.u32(from.wrapping_add(index * 4));
        e.mem.set_u32(to.wrapping_add(index * 4), word);
    }
}

// Translated from 00dcec70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (an add): `ECX` = this system and an object on the stack. A null object is ignored;
/// otherwise takes a reference on it (`00c90510`) and appends it to the array of referenced
/// objects, growing the array first (`00c90940` with the allocator and element size 4) when its
/// size equals its capacity.
pub fn fn_00dcec70(e: &mut Engine, this: Ptr, object: u32) {
    let me = this.addr();
    if object != 0 {
        e.call(ADD_REFERENCE, &args![object]);
        let array = me + CONTACTS;
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        if e.mem.u32(array + 4) == e.mem.u32(array + 8) & 0x3fff_ffff {
            let allocator = e.mem.u32(router + 0x10);
            e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
        }
        let size = e.mem.u32(array + 4);
        let data = e.mem.u32(array);
        e.mem
            .set_u32(data.wrapping_add(size.wrapping_mul(4)), object);
        e.mem.set_u32(array + 4, size.wrapping_add(1));
    }
}

// Translated from 00dcecc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhysicsSystemWithContacts::~hkpPhysicsSystemWithContacts` (engine map, `__fastcall`):
/// sets the vtable `010d07f4`, releases each of the objects in the array (`00c905b0`, the count
/// read once at the start), sets the size to 0, gives the storage back to the allocator (slot
/// `0x10`, `(data, capacity * 4)`) unless the capacity word has bit 31 set, resets the array
/// (data null, capacity word `0x80000000`) and ends in the base destructor (`00d06640`).
pub fn hkp_physics_system_with_contacts_destruct(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let count = e.mem.i32(me + CONTACTS + 4);
    e.mem.set_u32(me, VTABLE);
    for index in 0..count.max(0) as u32 {
        let object = e.mem.u32(e.mem.u32(me + CONTACTS) + index * 4);
        e.call(REMOVE_REFERENCE, &args![object]);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(me + CONTACTS + 8);
    e.mem.set_u32(me + CONTACTS + 4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(me + CONTACTS);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff).wrapping_mul(4)],
        );
    }
    e.mem.set_u32(me + CONTACTS, 0);
    e.mem.set_u32(me + CONTACTS + 8, 0x8000_0000);
    e.call(SYSTEM_BASE_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00dc_eb80,
            hkp_physics_system_with_contacts_clone(Ptr) -> u32
        ),
        entry!(0x00dc_ebd0, fn_00dcebd0(Ptr, Ptr)),
        entry!(0x00dc_ec70, fn_00dcec70(Ptr, u32)),
        entry!(0x00dc_ecc0, hkp_physics_system_with_contacts_destruct(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const SLOT_ALLOCATE_BYTES: u32 = 0x0900_0201;
    const SLOT_FREE: u32 = 0x0900_0202;

    fn record(e: &mut Engine, log: &Log, address: u32, result: u32) {
        let sink = Rc::clone(log);
        e.register_double(address, move |_, a| {
            sink.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax: result,
                ..Ret::default()
            }
        });
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(at, _)| *at == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// An engine with a router whose allocator records frees (slot `0x10`) and hands out blocks
    /// of `given` bytes (slot `0xc`: reads the wanted size word, writes `given` back).
    fn setup(given: u32) -> (Engine, Log, u32) {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0xc, SLOT_ALLOCATE_BYTES);
        e.mem.set_u32(table + 0x10, SLOT_FREE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, table);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(SLOT_ALLOCATE_BYTES, move |e, a| {
            sink.borrow_mut()
                .push((SLOT_ALLOCATE_BYTES, vec![a[0], e.mem.u32(a[1])]));
            e.mem.set_u32(a[1], given);
            Ret {
                eax: e.mem.alloc(given),
                ..Ret::default()
            }
        });
        record(&mut e, &log, SLOT_FREE, 0);
        (e, log, allocator)
    }

    fn set_contacts(e: &mut Engine, system: u32, words: &[u32], capacity_word: u32) -> u32 {
        let data = e.mem.alloc(4 * words.len().max(1) as u32);
        for (index, word) in words.iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, *word);
        }
        e.mem.set_u32(system + CONTACTS, data);
        e.mem.set_u32(system + CONTACTS + 4, words.len() as u32);
        e.mem.set_u32(system + CONTACTS + 8, capacity_word);
        data
    }

    fn contacts(e: &Engine, system: u32) -> Vec<u32> {
        let data = e.mem.u32(system + CONTACTS);
        (0..e.mem.u32(system + CONTACTS + 4))
            .map(|index| e.mem.u32(data + 4 * index))
            .collect()
    }

    #[test]
    fn clone_answers_null() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00dc_eb80, &args![0x1234u32]).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn assignment_copies_the_words_into_storage_that_is_large_enough() {
        let (mut e, log, _) = setup(64);
        record(&mut e, &log, SYSTEM_COPY, 0);
        let this = e.mem.alloc(0x60);
        let source = e.mem.alloc(0x60);
        let own = set_contacts(&mut e, this, &[9, 9, 9, 9], 4);
        set_contacts(&mut e, source, &[1, 2, 3], 3);
        e.call(0x00dc_ebd0, &args![this, source]);
        assert_eq!(calls_to(&log, SYSTEM_COPY), vec![vec![this, source]]);
        assert_eq!(contacts(&e, this), vec![1, 2, 3]);
        assert_eq!(e.mem.u32(this + CONTACTS), own);
        assert_eq!(e.mem.u32(this + CONTACTS + 8), 4);
        assert!(calls_to(&log, SLOT_FREE).is_empty());
        assert!(calls_to(&log, SLOT_ALLOCATE_BYTES).is_empty());
    }

    #[test]
    fn assignment_replaces_storage_that_is_too_small() {
        let (mut e, log, allocator) = setup(41);
        record(&mut e, &log, SYSTEM_COPY, 0);
        let this = e.mem.alloc(0x60);
        let source = e.mem.alloc(0x60);
        let old = set_contacts(&mut e, this, &[7], 2);
        set_contacts(&mut e, source, &[1, 2, 3, 4, 5], 5);
        e.call(0x00dc_ebd0, &args![this, source]);
        assert_eq!(calls_to(&log, SLOT_FREE), vec![vec![allocator, old, 8]]);
        assert_eq!(
            calls_to(&log, SLOT_ALLOCATE_BYTES),
            vec![vec![allocator, 20]]
        );
        assert_eq!(contacts(&e, this), vec![1, 2, 3, 4, 5]);
        // 41 bytes given: the capacity is 41 / 4 rounded toward zero.
        assert_eq!(e.mem.u32(this + CONTACTS + 8), 10);
        // Storage that is not owned is not freed.
        let (mut e, log, _) = setup(64);
        record(&mut e, &log, SYSTEM_COPY, 0);
        let this = e.mem.alloc(0x60);
        let source = e.mem.alloc(0x60);
        set_contacts(&mut e, this, &[], 0x8000_0001);
        set_contacts(&mut e, source, &[1, 2], 2);
        e.call(0x00dc_ebd0, &args![this, source]);
        assert!(calls_to(&log, SLOT_FREE).is_empty());
        assert_eq!(contacts(&e, this), vec![1, 2]);
        assert_eq!(e.mem.u32(this + CONTACTS + 8), 16);
    }

    #[test]
    fn adding_takes_a_reference_and_grows_a_full_array() {
        let (mut e, log, allocator) = setup(0);
        record(&mut e, &log, ADD_REFERENCE, 0);
        let sink = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            sink.borrow_mut().push((ARRAY_RESERVE_MORE, a.to_vec()));
            let room = e.mem.alloc(32);
            let first = e.mem.u32(e.mem.u32(a[1]));
            e.mem.set_u32(room, first);
            e.mem.set_u32(a[1], room);
            e.mem.set_u32(a[1] + 8, 8);
            Ret::default()
        });
        let this = e.mem.alloc(0x60);
        set_contacts(&mut e, this, &[0xaa], 1);
        e.call(0x00dc_ec70, &args![this, 0xbbu32]);
        assert_eq!(calls_to(&log, ADD_REFERENCE), vec![vec![0xbb]]);
        assert_eq!(
            calls_to(&log, ARRAY_RESERVE_MORE),
            vec![vec![allocator, this + CONTACTS, 4]]
        );
        assert_eq!(contacts(&e, this), vec![0xaa, 0xbb]);
        e.call(0x00dc_ec70, &args![this, 0xccu32]);
        assert_eq!(calls_to(&log, ARRAY_RESERVE_MORE).len(), 1);
        assert_eq!(contacts(&e, this), vec![0xaa, 0xbb, 0xcc]);
        // A null object changes nothing.
        e.call(0x00dc_ec70, &args![this, 0u32]);
        assert_eq!(calls_to(&log, ADD_REFERENCE).len(), 2);
        assert_eq!(contacts(&e, this), vec![0xaa, 0xbb, 0xcc]);
    }

    #[test]
    fn the_destructor_releases_frees_and_ends_in_the_base_destructor() {
        let (mut e, log, allocator) = setup(0);
        record(&mut e, &log, REMOVE_REFERENCE, 0);
        record(&mut e, &log, SYSTEM_BASE_DESTRUCT, 0);
        let this = e.mem.alloc(0x60);
        let data = set_contacts(&mut e, this, &[0x11, 0x22], 4);
        e.call(0x00dc_ecc0, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(
            log.borrow().clone(),
            vec![
                (REMOVE_REFERENCE, vec![0x11]),
                (REMOVE_REFERENCE, vec![0x22]),
                (SLOT_FREE, vec![allocator, data, 16]),
                (SYSTEM_BASE_DESTRUCT, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this + CONTACTS), 0);
        assert_eq!(e.mem.u32(this + CONTACTS + 4), 0);
        assert_eq!(e.mem.u32(this + CONTACTS + 8), 0x8000_0000);
        // Borrowed storage is not freed.
        log.borrow_mut().clear();
        let other = e.mem.alloc(0x60);
        set_contacts(&mut e, other, &[], 0x8000_0002);
        e.call(0x00dc_ecc0, &args![other]);
        assert_eq!(
            log.borrow().clone(),
            vec![(SYSTEM_BASE_DESTRUCT, vec![other])]
        );
    }
}
