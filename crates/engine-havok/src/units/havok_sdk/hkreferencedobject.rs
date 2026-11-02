//! `Havok SDK/hkreferencedobject.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkReferencedObject` keeps its size at `+4` (a `u16`; 0 means the object is not reference
//! counted) and its reference count at `+6` (a `u16`). The lock-taking variants guard the
//! count with a recursive lock: a record at the pointer held in `0x012681d4` with the mode word
//! at `+8` (1 = locking enabled), the recursion depth at `+0xC` and the critical section at
//! `+0x18`; the owning thread is marked in the TLS slot whose index is held in `0x012681d8`.

#[allow(unused_imports)]
use crate::prelude::*;

/// Pointer to the lock record, and the TLS index of the owner mark.
const LOCK_RECORD: u32 = 0x0126_81d4;
const LOCK_TLS_INDEX: u32 = 0x0126_81d8;
/// The value stored in the TLS slot while this thread holds the lock.
const OWNER_MARK: u32 = 0x23df_4554;
/// Import slots: `TlsGetValue`, `TlsSetValue`, `EnterCriticalSection`, `LeaveCriticalSection`.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const TLS_SET_VALUE: u32 = 0x00fd_f04c;
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;

/// Takes the recursive lock of the record at `lock`: bumps the depth when this thread already
/// owns it, otherwise enters the critical section, sets depth 1 and marks the thread.
fn lock_enter(e: &mut Engine, lock: u32) {
    let index = e.mem.u32(LOCK_TLS_INDEX);
    let owner = e.call(TLS_GET_VALUE, &args![index]).u32();
    if owner == OWNER_MARK {
        let depth = e.mem.u32(lock + 0xc);
        e.mem.set_u32(lock + 0xc, depth.wrapping_add(1));
        return;
    }
    e.call(ENTER_CRITICAL_SECTION, &args![lock + 0x18]);
    e.mem.set_u32(lock + 0xc, 1);
    let index = e.mem.u32(LOCK_TLS_INDEX);
    e.call(TLS_SET_VALUE, &args![index, OWNER_MARK]);
}

/// Drops one level of the recursive lock; at depth 0 or below the owner mark is cleared and
/// the critical section left.
fn lock_leave(e: &mut Engine, lock: u32) {
    let depth = (e.mem.u32(lock + 0xc) as i32).wrapping_sub(1);
    e.mem.set_u32(lock + 0xc, depth as u32);
    if depth < 1 {
        let index = e.mem.u32(LOCK_TLS_INDEX);
        e.call(TLS_SET_VALUE, &args![index, 0u32]);
        e.call(LEAVE_CRITICAL_SECTION, &args![lock + 0x18]);
    }
}

/// Whether the locked path applies: the mode word at `+8` is 1 and this thread is not
/// already marked as the owner.
fn needs_lock(e: &mut Engine, lock: u32) -> bool {
    if e.mem.u32(lock + 8) != 1 {
        return false;
    }
    let index = e.mem.u32(LOCK_TLS_INDEX);
    e.call(TLS_GET_VALUE, &args![index]).u32() != OWNER_MARK
}

/// Adds `delta` to the `u16` reference count at `this + 6`.
fn adjust_count(e: &mut Engine, this: Ptr, delta: u16) {
    let count = e.mem.u16(this.addr() + 6);
    e.mem.set_u16(this.addr() + 6, count.wrapping_add(delta));
}

// Translated from 00c903a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkReferencedObject::addReferenceLockUnchecked` (Xbox PDB): bumps the reference count
/// (`+6`) unless the object is not counted (size word `+4` is 0). No lock is taken.
pub fn add_reference_lock_unchecked(e: &mut Engine, this: Ptr) {
    if e.mem.u16(this.addr() + 4) != 0 {
        adjust_count(e, this, 1);
    }
}

// Translated from 00c903b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkReferencedObject::removeReferenceLockUnchecked` (Xbox PDB): drops the reference count
/// and, when it reaches 0, calls the deleting destructor (vtable slot 0) with flag 1. Not
/// counted objects (size word 0) are left alone. No lock is taken.
pub fn remove_reference_lock_unchecked(e: &mut Engine, this: Ptr) {
    if e.mem.u16(this.addr() + 4) == 0 {
        return;
    }
    adjust_count(e, this, 0xffff);
    if e.mem.u16(this.addr() + 6) == 0 {
        e.vcall(this.addr(), 0, &args![1u32]);
    }
}

// Translated from 00c90490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes the Havok reference-count lock (recursive; see the module comment). The function
/// has no `this`; its name is unconfirmed.
pub fn fn_00c90490(e: &mut Engine) {
    let lock = e.mem.u32(LOCK_RECORD);
    lock_enter(e, lock);
}

// Translated from 00c904e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases one level of the Havok reference-count lock taken by `00c90490`.
pub fn fn_00c904e0(e: &mut Engine) {
    let lock = e.mem.u32(LOCK_RECORD);
    lock_leave(e, lock);
}

// Translated from 00c90510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Locked `hkReferencedObject::addReference`: for a counted object, bumps the count under
/// the recursive lock when locking is enabled and this thread does not own it, otherwise
/// bumps it directly.
pub fn fn_00c90510(e: &mut Engine, this: Ptr) {
    if e.mem.u16(this.addr() + 4) == 0 {
        return;
    }
    let lock = e.mem.u32(LOCK_RECORD);
    if needs_lock(e, lock) {
        lock_enter(e, lock);
        adjust_count(e, this, 1);
        lock_leave(e, lock);
    } else {
        adjust_count(e, this, 1);
    }
}

// Translated from 00c905b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Locked `hkReferencedObject::removeReference`: for a counted object whose count is 1,
/// clears the count and calls the deleting destructor (slot 0, flag 1) without locking;
/// otherwise decrements the count under the recursive lock when locking is enabled and this
/// thread does not own it, or directly.
pub fn fn_00c905b0(e: &mut Engine, this: Ptr) {
    if e.mem.u16(this.addr() + 4) == 0 {
        return;
    }
    if e.mem.u16(this.addr() + 6) == 1 {
        e.mem.set_u16(this.addr() + 6, 0);
        e.vcall(this.addr(), 0, &args![1u32]);
        return;
    }
    let lock = e.mem.u32(LOCK_RECORD);
    if needs_lock(e, lock) {
        lock_enter(e, lock);
        adjust_count(e, this, 0xffff);
        lock_leave(e, lock);
    } else {
        adjust_count(e, this, 0xffff);
    }
}

// Translated from 00c903d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x0102_e388.
pub fn return_constant_00c903d0(_e: &mut Engine) -> u32 {
    0x0102_e388
}

/// `hkStatisticsCollectorUtil::hasContents` (`00cf6170`, cdecl `(out byte, class)`, returns
/// `out`) and `addClassContentsAll` (`00cf6920`, cdecl `(object, class, collector)`).
const HAS_CONTENTS: u32 = 0x00cf_6170;
const ADD_CLASS_CONTENTS_ALL: u32 = 0x00cf_6920;
/// The `hkClass` record at `0x01268748` that `calcContentStatistics` leaves out.
const SKIPPED_CLASS: u32 = 0x0126_8748;

// Translated from 00c90360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkReferencedObject::calcContentStatistics` (Xbox PDB), `this` = the object, `(collector,
/// class)`: for a class that is not null and not the record at `0x01268748`, asks
/// `hasContents` (the byte goes into the stack slot of the class argument) and when it is
/// nonzero calls `addClassContentsAll(this, class, collector)`.
pub fn calc_content_statistics(e: &mut Engine, this: Ptr, collector: Ptr, class: Ptr) {
    if class.addr() == 0 || class.addr() == SKIPPED_CLASS {
        return;
    }
    let has_contents = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), class.addr());
        let out = e.call(HAS_CONTENTS, &args![slot, class]).u32();
        e.mem.u8(out)
    });
    if has_contents != 0 {
        e.call(ADD_CLASS_CONTENTS_ALL, &args![this, class, collector]);
    }
}

// Translated from 00c90410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkReferencedObject::setLockMode` (Xbox PDB), cdecl `(mode)`: stores `mode` as the mode word
/// (`+8`) of the lock record held at `0x012681d4`.
pub fn set_lock_mode(e: &mut Engine, mode: u32) {
    let lock = e.mem.u32(LOCK_RECORD);
    e.mem.set_u32(lock + 8, mode);
}

// Translated from 00c90670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkReferencedObject::addReferences` (Xbox PDB), cdecl `(objects, count, stride)`: adds a
/// reference (`addReferenceLockUnchecked`) to each of `count` objects whose pointers are
/// `stride` bytes apart from `objects`. With a nonzero lock mode (`+8` of the lock record)
/// and a thread that is not the owner the whole run is done under the recursive lock.
pub fn add_references(e: &mut Engine, objects: Ptr, count: i32, stride: u32) {
    let lock = e.mem.u32(LOCK_RECORD);
    let mut locked = false;
    if e.mem.u32(lock + 8) != 0 {
        let index = e.mem.u32(LOCK_TLS_INDEX);
        locked = e.call(TLS_GET_VALUE, &args![index]).u32() != OWNER_MARK;
    }
    if locked {
        lock_enter(e, lock);
    }
    let mut cursor = objects.addr();
    let mut remaining = count;
    if remaining > 0 {
        loop {
            let object = e.mem.u32(cursor);
            add_reference_lock_unchecked(e, Ptr::new(object));
            cursor = cursor.wrapping_add(stride);
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
    }
    if locked {
        lock_leave(e, lock);
    }
}

// Translated from 00c903e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: cdecl, no stack words. Allocates 0x30 bytes through the memory
/// router's allocator (`00c85750`; virtual function at +4 of the object at router + 0x10), stores
/// the size as a 16-bit word at +4 and ends with a jump to the constructor `00c90830` (`this` in
/// `ECX`, no stack words), whose answer it returns. Ghidra made no function here; the address is stored in the data word at 011b0cfc.
pub fn fn_00c903e0(e: &mut Engine) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x30u32]).u32();
    e.mem.set_u16(block + 4, 0x30);
    e.call(0x00c9_0830, &args![block]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c903d0, return_constant_00c903d0() -> u32),
        entry!(0x00c9_03a0, add_reference_lock_unchecked(Ptr)),
        entry!(0x00c9_03b0, remove_reference_lock_unchecked(Ptr)),
        entry!(0x00c9_0490, fn_00c90490()),
        entry!(0x00c9_04e0, fn_00c904e0()),
        entry!(0x00c9_0510, fn_00c90510(Ptr)),
        entry!(0x00c9_05b0, fn_00c905b0(Ptr)),
        entry!(0x00c9_0360, calc_content_statistics(Ptr, Ptr, Ptr)),
        entry!(0x00c9_0410, set_lock_mode(u32)),
        entry!(0x00c9_0670, add_references(Ptr, i32, u32)),
        entry!(0x00c9_03e0, fn_00c903e0() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const LOCK: u32 = 0x0900_0000;
    const DESTRUCTOR_DOUBLE: u32 = 0x0900_0100;

    type CallLog = Rc<RefCell<Vec<(u32, u32)>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.map(LOCK, 0x100);
        e.mem.set_u32(LOCK_RECORD, LOCK);
        e.mem.set_u32(LOCK_TLS_INDEX, 3);
        e
    }

    /// Doubles for the four imports; the TLS slot is shared state, and every call is logged
    /// as (import address, last argument or the critical section).
    fn install_imports(e: &mut Engine) -> (Rc<RefCell<u32>>, CallLog) {
        let slot = Rc::new(RefCell::new(0u32));
        let log: CallLog = Rc::new(RefCell::new(Vec::new()));
        let (s, l) = (Rc::clone(&slot), Rc::clone(&log));
        e.register_double(TLS_GET_VALUE, move |_, a| {
            l.borrow_mut().push((TLS_GET_VALUE, a[0]));
            Ret {
                eax: *s.borrow(),
                ..Ret::default()
            }
        });
        let (s, l) = (Rc::clone(&slot), Rc::clone(&log));
        e.register_double(TLS_SET_VALUE, move |_, a| {
            l.borrow_mut().push((TLS_SET_VALUE, a[1]));
            *s.borrow_mut() = a[1];
            Ret::default()
        });
        for import in [ENTER_CRITICAL_SECTION, LEAVE_CRITICAL_SECTION] {
            let l = Rc::clone(&log);
            e.register_double(import, move |_, a| {
                l.borrow_mut().push((import, a[0]));
                Ret::default()
            });
        }
        (slot, log)
    }

    /// An object with a vtable whose slot 0 is a logging double.
    fn object(e: &mut Engine, size: u16, count: u16) -> (u32, Rc<RefCell<Vec<u32>>>) {
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, DESTRUCTOR_DOUBLE);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.mem.set_u16(object + 4, size);
        e.mem.set_u16(object + 6, count);
        let log = Rc::new(RefCell::new(Vec::new()));
        let l = Rc::clone(&log);
        e.register_double(DESTRUCTOR_DOUBLE, move |_, a| {
            l.borrow_mut().extend_from_slice(a);
            Ret::default()
        });
        (object, log)
    }

    fn entered(log: &CallLog) -> bool {
        log.borrow().iter().any(|c| c.0 == ENTER_CRITICAL_SECTION)
    }

    #[test]
    fn add_reference_lock_unchecked_counts_only_counted_objects() {
        let mut e = engine();
        let (counted, _) = object(&mut e, 0x20, 5);
        e.call(0x00c9_03a0, &args![counted]);
        assert_eq!(e.mem.u16(counted + 6), 6);
        let (plain, _) = object(&mut e, 0, 5);
        e.call(0x00c9_03a0, &args![plain]);
        assert_eq!(e.mem.u16(plain + 6), 5);
    }

    #[test]
    fn remove_reference_lock_unchecked_destroys_at_zero() {
        let mut e = engine();
        let (obj, log) = object(&mut e, 0x20, 2);
        e.call(0x00c9_03b0, &args![obj]);
        assert_eq!(e.mem.u16(obj + 6), 1);
        assert!(log.borrow().is_empty());
        e.call(0x00c9_03b0, &args![obj]);
        assert_eq!(e.mem.u16(obj + 6), 0);
        assert_eq!(*log.borrow(), vec![obj, 1]);
        let (plain, plain_log) = object(&mut e, 0, 1);
        e.call(0x00c9_03b0, &args![plain]);
        assert_eq!(e.mem.u16(plain + 6), 1);
        assert!(plain_log.borrow().is_empty());
    }

    #[test]
    fn lock_enter_takes_the_critical_section_once_and_recurses() {
        let mut e = engine();
        let (slot, log) = install_imports(&mut e);
        e.call(0x00c9_0490, &args![]);
        assert_eq!(e.mem.u32(LOCK + 0xc), 1);
        assert_eq!(*slot.borrow(), OWNER_MARK);
        assert!(log
            .borrow()
            .contains(&(ENTER_CRITICAL_SECTION, LOCK + 0x18)));
        log.borrow_mut().clear();
        e.call(0x00c9_0490, &args![]);
        assert_eq!(e.mem.u32(LOCK + 0xc), 2);
        assert!(!entered(&log));
    }

    #[test]
    fn lock_leave_releases_at_depth_zero() {
        let mut e = engine();
        let (slot, log) = install_imports(&mut e);
        e.mem.set_u32(LOCK + 0xc, 2);
        *slot.borrow_mut() = OWNER_MARK;
        e.call(0x00c9_04e0, &args![]);
        assert_eq!(e.mem.u32(LOCK + 0xc), 1);
        assert!(!log.borrow().iter().any(|c| c.0 == LEAVE_CRITICAL_SECTION));
        e.call(0x00c9_04e0, &args![]);
        assert_eq!(e.mem.u32(LOCK + 0xc), 0);
        assert_eq!(*slot.borrow(), 0);
        assert!(log
            .borrow()
            .contains(&(LEAVE_CRITICAL_SECTION, LOCK + 0x18)));
    }

    #[test]
    fn add_reference_locks_only_when_enabled_and_not_owner() {
        let mut e = engine();
        let (_, log) = install_imports(&mut e);
        let (obj, _) = object(&mut e, 0x20, 1);
        // Locking disabled: plain increment.
        e.call(0x00c9_0510, &args![obj]);
        assert_eq!(e.mem.u16(obj + 6), 2);
        assert!(!entered(&log));
        // Locking enabled, thread not the owner: locked.
        e.mem.set_u32(LOCK + 8, 1);
        e.call(0x00c9_0510, &args![obj]);
        assert_eq!(e.mem.u16(obj + 6), 3);
        assert!(log
            .borrow()
            .contains(&(ENTER_CRITICAL_SECTION, LOCK + 0x18)));
        assert!(log
            .borrow()
            .contains(&(LEAVE_CRITICAL_SECTION, LOCK + 0x18)));
        // Not counted: untouched.
        let (plain, _) = object(&mut e, 0, 1);
        e.call(0x00c9_0510, &args![plain]);
        assert_eq!(e.mem.u16(plain + 6), 1);
    }

    #[test]
    fn add_reference_skips_the_lock_for_the_owner() {
        let mut e = engine();
        let (slot, log) = install_imports(&mut e);
        *slot.borrow_mut() = OWNER_MARK;
        e.mem.set_u32(LOCK + 8, 1);
        let (obj, _) = object(&mut e, 0x20, 1);
        e.call(0x00c9_0510, &args![obj]);
        assert_eq!(e.mem.u16(obj + 6), 2);
        assert!(!entered(&log));
    }

    #[test]
    fn remove_reference_destroys_the_last_reference_without_locking() {
        let mut e = engine();
        let (_, log) = install_imports(&mut e);
        e.mem.set_u32(LOCK + 8, 1);
        let (obj, destroyed) = object(&mut e, 0x20, 1);
        e.call(0x00c9_05b0, &args![obj]);
        assert_eq!(e.mem.u16(obj + 6), 0);
        assert_eq!(*destroyed.borrow(), vec![obj, 1]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn remove_reference_decrements_under_the_lock() {
        let mut e = engine();
        let (_, log) = install_imports(&mut e);
        let (obj, destroyed) = object(&mut e, 0x20, 3);
        e.call(0x00c9_05b0, &args![obj]);
        assert_eq!(e.mem.u16(obj + 6), 2);
        assert!(log.borrow().is_empty());
        e.mem.set_u32(LOCK + 8, 1);
        e.call(0x00c9_05b0, &args![obj]);
        assert_eq!(e.mem.u16(obj + 6), 1);
        assert!(entered(&log));
        assert!(destroyed.borrow().is_empty());
        let (plain, _) = object(&mut e, 0, 3);
        e.call(0x00c9_05b0, &args![plain]);
        assert_eq!(e.mem.u16(plain + 6), 3);
    }

    #[test]
    fn return_constant_00c903d0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c9_03d0, &args![]).u32(), 0x0102_e388);
    }

    #[test]
    fn set_lock_mode_stores_the_word_in_the_lock_record() {
        let mut e = engine();
        e.call(0x00c9_0410, &args![2u32]);
        assert_eq!(e.mem.u32(LOCK + 8), 2);
    }

    fn add_references_objects(e: &mut Engine) -> (u32, u32, u32) {
        let (first, _) = object(e, 0x20, 1);
        let (second, _) = object(e, 0x20, 7);
        // Pointers 8 bytes apart.
        let array = e.mem.alloc(0x20);
        e.mem.set_u32(array, first);
        e.mem.set_u32(array + 8, second);
        (array, first, second)
    }

    #[test]
    fn add_references_without_locking_counts_each_object() {
        let mut e = engine();
        let (_, log) = install_imports(&mut e);
        let (array, first, second) = add_references_objects(&mut e);
        e.call(0x00c9_0670, &args![array, 2u32, 8u32]);
        assert_eq!(e.mem.u16(first + 6), 2);
        assert_eq!(e.mem.u16(second + 6), 8);
        assert!(log.borrow().is_empty());
        // A count of zero touches nothing.
        e.call(0x00c9_0670, &args![array, 0u32, 8u32]);
        assert_eq!(e.mem.u16(first + 6), 2);
    }

    #[test]
    fn add_references_under_the_lock_enters_and_leaves_it() {
        let mut e = engine();
        let (slot, log) = install_imports(&mut e);
        e.mem.set_u32(LOCK + 8, 1);
        let (array, first, second) = add_references_objects(&mut e);
        e.call(0x00c9_0670, &args![array, 2u32, 8u32]);
        assert_eq!(e.mem.u16(first + 6), 2);
        assert_eq!(e.mem.u16(second + 6), 8);
        assert!(log
            .borrow()
            .contains(&(ENTER_CRITICAL_SECTION, LOCK + 0x18)));
        assert!(log
            .borrow()
            .contains(&(LEAVE_CRITICAL_SECTION, LOCK + 0x18)));
        assert_eq!(*slot.borrow(), 0);
    }

    #[test]
    fn add_references_as_the_owner_skips_the_lock() {
        let mut e = engine();
        let (slot, log) = install_imports(&mut e);
        *slot.borrow_mut() = OWNER_MARK;
        e.mem.set_u32(LOCK + 8, 1);
        let (array, first, _) = add_references_objects(&mut e);
        e.call(0x00c9_0670, &args![array, 1u32, 8u32]);
        assert_eq!(e.mem.u16(first + 6), 2);
        assert!(!entered(&log));
    }

    #[test]
    fn calc_content_statistics_adds_the_contents_of_classes_that_have_some() {
        let mut e = engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register(HAS_CONTENTS, |e, a| {
            // The class record's first word says whether it has contents.
            let has = e.mem.u32(a[1]) != 0;
            e.mem.set_u8(a[0], u8::from(has));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register_double(ADD_CLASS_CONTENTS_ALL, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let with = e.mem.alloc(8);
        e.mem.set_u32(with, 1);
        let without = e.mem.alloc(8);
        e.call(0x00c9_0360, &args![0x10u32, 0x20u32, with]);
        e.call(0x00c9_0360, &args![0x10u32, 0x20u32, without]);
        e.call(0x00c9_0360, &args![0x10u32, 0x20u32, 0u32]);
        e.call(0x00c9_0360, &args![0x10u32, 0x20u32, SKIPPED_CLASS]);
        assert_eq!(*seen.borrow(), vec![vec![0x10, with, 0x20]]);
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The slot of the allocator's vtable (offset 4) that allocates, as a test double.
    const ALLOCATE_SLOT_DOUBLE: u32 = 0x0900_0001;

    /// A memory router (`00c85750`) whose allocator allocates through a double that logs the
    /// size it was asked for and answers a zeroed block.
    fn allocator_engine() -> (Engine, Rc<RefCell<Vec<u32>>>) {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sizes = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&sizes);
        e.register_double(ALLOCATE_SLOT_DOUBLE, move |e, a| {
            log.borrow_mut().push(a[1]);
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        (e, sizes)
    }

    /// The constructor sees the block with its size stored and its answer is returned.
    #[test]
    fn fn_00c903e0_allocates_and_hands_the_block_to_the_constructor() {
        let (mut e, sizes) = allocator_engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00c9_0830, move |e, a| {
            log.borrow_mut().push((a.to_vec(), e.mem.u16(a[0] + 4)));
            Ret {
                eax: 0x1234,
                ..Ret::default()
            }
        });
        assert_eq!(e.call(0x00c9_03e0, &args![]).u32(), 0x1234);
        assert_eq!(*sizes.borrow(), vec![0x30]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0.len(), 1);
        assert_eq!(seen[0].1, 0x30);
    }
}
