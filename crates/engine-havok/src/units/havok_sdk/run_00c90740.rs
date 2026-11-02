//! `Havok SDK/run_00c90740` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TlsGetValue`, `TlsSetValue`, `EnterCriticalSection` and `LeaveCriticalSection` import
/// slots.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const TLS_SET_VALUE: u32 = 0x00fd_f04c;
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// The Havok reference-counting lock: a structure (`012681d4` holds its address) whose word
/// +8 says whether locking is on, word +0xC counts recursive holds and +0x18 is the critical
/// section; `012681d8` holds the TLS index that marks the owning thread.
const LOCK_STRUCTURE: u32 = 0x0126_81d4;
const LOCK_TLS_INDEX: u32 = 0x0126_81d8;
/// The value stored in the TLS slot of the thread that holds the lock.
const LOCK_OWNER_MARK: u32 = 0x23df_4554;
/// `hkReferencedObject::removeReferenceLockUnchecked` (Xbox PDB, `00c903b0`, `this`).
const REMOVE_REFERENCE_UNCHECKED: u32 = 0x00c9_03b0;

/// Releases a reference on each of `count` objects whose pointers are `stride` bytes apart.
fn remove_references(e: &mut Engine, objects: u32, count: i32, stride: u32) {
    let mut cursor = objects;
    let mut remaining = count;
    while remaining > 0 {
        let object = e.mem.u32(cursor);
        e.call(REMOVE_REFERENCE_UNCHECKED, &args![object]);
        cursor = cursor.wrapping_add(stride);
        remaining -= 1;
    }
}

// Translated from 00c90740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes a reference from each of `count` objects (pointers `stride` bytes apart from
/// `objects`) with `00c903b0`. When the reference lock is on (word +8 of the structure at
/// `012681d4`) and the calling thread is not marked as the owner (`TlsGetValue` of the
/// index at `012681d8` is not `0x23df4554`), the loop runs holding the lock: the recursive
/// count (+0xC) rises, or when the thread is still unmarked at the second look the critical
/// section (+0x18) is entered, the count set to 1 and the mark stored; afterwards the count
/// drops and, when it reaches zero, the mark is cleared and the section left. Otherwise
/// the loop runs unlocked. The class is unconfirmed.
pub fn fn_00c90740(e: &mut Engine, objects: Ptr, count: i32, stride: u32) {
    let lock = e.global::<u32>(LOCK_STRUCTURE);
    if e.mem.u32(lock + 8) != 0 {
        let index: u32 = e.global(LOCK_TLS_INDEX);
        if e.call(TLS_GET_VALUE, &args![index]).u32() != LOCK_OWNER_MARK {
            let index: u32 = e.global(LOCK_TLS_INDEX);
            if e.call(TLS_GET_VALUE, &args![index]).u32() == LOCK_OWNER_MARK {
                let holds = e.mem.u32(lock + 0xc);
                e.mem.set_u32(lock + 0xc, holds.wrapping_add(1));
            } else {
                e.call(ENTER_CRITICAL_SECTION, &args![lock + 0x18]);
                e.mem.set_u32(lock + 0xc, 1);
                let index: u32 = e.global(LOCK_TLS_INDEX);
                e.call(TLS_SET_VALUE, &args![index, LOCK_OWNER_MARK]);
            }
            remove_references(e, objects.addr(), count, stride);
            let holds = e.mem.u32(lock + 0xc).wrapping_sub(1);
            e.mem.set_u32(lock + 0xc, holds);
            if (holds as i32) <= 0 {
                let index: u32 = e.global(LOCK_TLS_INDEX);
                e.call(TLS_SET_VALUE, &args![index, 0u32]);
                e.call(LEAVE_CRITICAL_SECTION, &args![lock + 0x18]);
            }
            return;
        }
    }
    remove_references(e, objects.addr(), count, stride);
}

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkCriticalSection::hkCriticalSection` (`00cf6da0`, `this` = the section, one word).
const CRITICAL_SECTION_CONSTRUCT: u32 = 0x00cf_6da0;
/// The call `00c90830` makes on `this + 0x10` once the section is built (`00cf6d30`).
const SUB_OBJECT_INIT: u32 = 0x00cf_6d30;
/// `DeleteCriticalSection` import slot.
const DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
/// The vtable of the object `00c90830` builds (`010c6334`) and the base vtable (`0102e388`).
const OBJECT_VTABLE: u32 = 0x010c_6334;
const BASE_VTABLE: u32 = 0x0102_e388;

// Translated from 00c90830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the object whose vtable is `010c6334` (its destructor is `00c90880`; the
/// class is unconfirmed): 16-bit word `1` at `+6`, the vtable word, the 16-bit word `0x8000`
/// at `+0x14` and the word `0xfffffff1` at `+0x10`, a critical section built at `+0x18`
/// (`00cf6da0` with `4000`), `1` at `+8`, `00cf6d30` run on `this + 0x10` and `0` at `+0xc`.
/// Returns `this`.
pub fn fn_00c90830(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u16(base + 6, 1);
    e.mem.set_u32(base, OBJECT_VTABLE);
    e.mem.set_u16(base + 0x14, 0x8000);
    e.mem.set_u32(base + 0x10, 0xffff_fff1);
    e.call(CRITICAL_SECTION_CONSTRUCT, &args![base + 0x18, 4000u32]);
    e.mem.set_u32(base + 8, 1);
    e.call(SUB_OBJECT_INIT, &args![base + 0x10]);
    e.mem.set_u32(base + 0xc, 0);
    this
}

// Translated from 00c90880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor of the object `00c90830` builds: deletes the critical section at
/// `+0x18` (`DeleteCriticalSection`, import slot `00fdf058`), writes the vtable word
/// `0102e388`, and when bit 0 of `flags` is set frees the block through the memory router's
/// allocator (slot `8`) with the 16-bit size kept at `this + 4`. Returns `this`.
pub fn fn_00c90880(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    let base = this.addr();
    e.call(DELETE_CRITICAL_SECTION, &args![base + 0x18]);
    e.mem.set_u32(base, BASE_VTABLE);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(base + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c9_0740, fn_00c90740(Ptr, i32, u32)),
        entry!(0x00c9_0830, fn_00c90830(Ptr) -> Ptr),
        entry!(0x00c9_0880, fn_00c90880(Ptr, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    type Log = Rc<RefCell<Vec<(&'static str, u32, u32)>>>;

    /// The lock structure, three objects 8 bytes apart, and doubles for the imports; the
    /// thread's TLS slot starts at `tls`.
    fn rig(lock_on: bool, tls: u32) -> (Engine, Log, u32, u32) {
        let mut e = test_engine();
        let lock = e.mem.alloc(0x40);
        e.map(LOCK_STRUCTURE, 8);
        e.set_global(LOCK_STRUCTURE, lock);
        e.set_global(LOCK_TLS_INDEX, 5u32);
        e.mem.set_u32(lock + 8, u32::from(lock_on));
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let slot = Rc::new(RefCell::new(tls));
        let reads = Rc::clone(&slot);
        e.register_double(TLS_GET_VALUE, move |_, a| {
            assert_eq!(a[0], 5);
            Ret {
                eax: *reads.borrow(),
                ..Ret::default()
            }
        });
        let writes = Rc::clone(&slot);
        let sink = Rc::clone(&log);
        e.register_double(TLS_SET_VALUE, move |_, a| {
            sink.borrow_mut().push(("set", a[0], a[1]));
            *writes.borrow_mut() = a[1];
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(ENTER_CRITICAL_SECTION, move |_, a| {
            sink.borrow_mut().push(("enter", a[0], 0));
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(LEAVE_CRITICAL_SECTION, move |_, a| {
            sink.borrow_mut().push(("leave", a[0], 0));
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(REMOVE_REFERENCE_UNCHECKED, move |_, a| {
            sink.borrow_mut().push(("release", a[0], 0));
            Ret::default()
        });
        let objects = e.mem.alloc(0x20);
        for i in 0..3 {
            e.mem.set_u32(objects + 8 * i, 0x100 + i);
        }
        (e, log, lock, objects)
    }

    fn releases(log: &Log) -> Vec<u32> {
        log.borrow()
            .iter()
            .filter(|(what, _, _)| *what == "release")
            .map(|(_, object, _)| *object)
            .collect()
    }

    #[test]
    fn without_the_lock_the_objects_are_released_directly() {
        let (mut e, log, _, objects) = rig(false, 0);
        e.call(0x00c9_0740, &args![objects, 3u32, 8u32]);
        assert_eq!(releases(&log), vec![0x100, 0x101, 0x102]);
        assert_eq!(log.borrow().len(), 3);
        // A count of zero (or less) releases nothing.
        e.call(0x00c9_0740, &args![objects, 0u32, 8u32]);
        e.call(0x00c9_0740, &args![objects, -2i32, 8u32]);
        assert_eq!(log.borrow().len(), 3);
    }

    #[test]
    fn an_unmarked_thread_takes_the_section_and_marks_itself() {
        let (mut e, log, lock, objects) = rig(true, 0);
        e.call(0x00c9_0740, &args![objects, 2u32, 8u32]);
        let seen: Vec<(&str, u32, u32)> = log.borrow().clone();
        assert_eq!(
            seen,
            vec![
                ("enter", lock + 0x18, 0),
                ("set", 5, LOCK_OWNER_MARK),
                ("release", 0x100, 0),
                ("release", 0x101, 0),
                ("set", 5, 0),
                ("leave", lock + 0x18, 0),
            ]
        );
        assert_eq!(e.mem.u32(lock + 0xc), 0);
    }

    #[test]
    fn the_marked_owner_releases_without_locking() {
        let (mut e, log, lock, objects) = rig(true, LOCK_OWNER_MARK);
        e.call(0x00c9_0740, &args![objects, 2u32, 8u32]);
        assert_eq!(releases(&log), vec![0x100, 0x101]);
        assert_eq!(log.borrow().len(), 2);
        assert_eq!(e.mem.u32(lock + 0xc), 0);
    }
}

#[cfg(test)]
mod tests_00c90830 {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// The constructor sets its words and runs the two helpers in order on the right objects.
    #[test]
    fn constructor_builds_the_section_and_sets_the_words() {
        let mut e = engine();
        let log = Rc::new(RefCell::new(Vec::new()));
        let section_log = Rc::clone(&log);
        e.register_double(CRITICAL_SECTION_CONSTRUCT, move |e, a| {
            // The words written before the section is built are already in place.
            let before = (e.mem.u32(a[0] - 0x18 + 0x10), e.mem.u16(a[0] - 0x18 + 0x14));
            section_log
                .borrow_mut()
                .push((1, a[0], a[1], before.0, u32::from(before.1)));
            Ret::default()
        });
        let init_log = Rc::clone(&log);
        e.register_double(SUB_OBJECT_INIT, move |e, a| {
            let flag = e.mem.u32(a[0] - 0x10 + 8);
            init_log.borrow_mut().push((2, a[0], flag, 0, 0));
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xc, 0x77);
        let got = e.call(0x00c9_0830, &args![this]).u32();
        assert_eq!(got, this);
        assert_eq!(
            *log.borrow(),
            vec![
                (1, this + 0x18, 4000, 0xffff_fff1, 0x8000),
                (2, this + 0x10, 1, 0, 0)
            ]
        );
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), OBJECT_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 1);
        assert_eq!(e.mem.u32(this + 0xc), 0);
    }

    /// The section is deleted, the vtable word reset and the block freed only with bit 0.
    #[test]
    fn destructor_deletes_the_section_and_frees_with_bit_zero() {
        let mut e = engine();
        let log = Rc::new(RefCell::new(Vec::new()));
        let section_log = Rc::clone(&log);
        e.register_double(DELETE_CRITICAL_SECTION, move |_, a| {
            section_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x30);
        let got = e.call(0x00c9_0880, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        assert_eq!(*log.borrow(), vec![1, this + 0x18]);
        log.borrow_mut().clear();
        e.call(0x00c9_0880, &args![this, 1u32]);
        assert_eq!(*log.borrow(), vec![1, this + 0x18, 2, this, 0x30]);
    }
}
