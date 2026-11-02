//! `BSMain/run_00c457e0` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit is the code of one global array of reference-counted objects
//! (`01202e20`, laid out like an `NiTArray`: element pointer at +4, then the
//! 16-bit capacity, size, element count and growth step), the functions that
//! add to it, empty it and trim it under the guard at `01202e40`, and the
//! array's own element handling: the `SetSize`-like resize (`00c45b20`), the
//! smart-pointer assignment and construction helpers it uses, the array
//! allocation and its destruction. Which class the array belongs to is not
//! confirmed from the exe: the engine map names none of these functions, and
//! the one name Ghidra shows for `00c457e0` belongs to a library body the
//! compiler folded.
//!
//! The guard at `01202e40` is an object entered with `0040fbf0` (owner thread
//! word at +0, nesting count at +4); the functions that hold it leave it with
//! the inlined decrement of the count at `01202e44` and the clearing of the
//! owner word when the count reaches zero.
//!
//! Functions that carry the compiler's C++ exception-unwinding frame
//! (`FS:[0]`) are marked `Translation incomplete`: the unwinding is not
//! translated.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;

/// The global array (`01202e20`).
pub(crate) const GLOBAL_ARRAY: u32 = 0x0120_2e20;
/// The guard object (`01202e40`): owner word at +0, count at +4.
const GUARD: u32 = 0x0120_2e40;
/// The guard's nesting count (`01202e44`).
const GUARD_COUNT: u32 = 0x0120_2e44;
/// The function-local static null smart pointer (`01202e14`) and its
/// "constructed" word (`01202e18`, bit 0).
const NULL_POINTER_VALUE: u32 = 0x0120_2e14;
const NULL_POINTER_GUARD: u32 = 0x0120_2e18;
/// A word (`011af758`) the time-budgeted trim adds to the tick count.
const TIME_BUDGET: u32 = 0x011a_f758;
/// The memory manager singleton (`MemoryManager::Allocate` and `Deallocate` are
/// its methods).
const MEMORY_MANAGER: u32 = crate::units::platform::MEMORY_MANAGER;
/// The critical section (`011f4380`) the flush tries to enter.
const CRITICAL_SECTION: u32 = 0x011f_4380;

/// Enters the guard (`0040fbf0(guard, flag)`).
const GUARD_ENTER: u32 = 0x0040_fbf0;
/// Tries to enter the guard (`0078d200`, `this` the guard): bool.
const GUARD_TRY_ENTER: u32 = 0x0078_d200;
/// The unit's neighbour in `BSMain/bsninode.obj` that the object at
/// `00c457e0` runs first (`this` only).
const DESTRUCTOR_BODY: u32 = 0x00c5_faa0;
/// `operator delete(block, size)` (cdecl).
const OPERATOR_DELETE_SIZED: u32 = 0x00aa_1460;
/// `MemoryManager::Allocate(this, size)` and `Deallocate(this, block)`.
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// `_eh_vector_constructor_iterator_` (block, element size, count,
/// constructor, destructor) and `_eh_vector_destructor_iterator_` (block,
/// element size, count, destructor).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
const VECTOR_DESTRUCTOR_ITERATOR: u32 = 0x00ec_5fce;
/// The smart-pointer destructor the vector helpers run per element.
const SMART_POINTER_DESTRUCTOR: u32 = 0x00ae_9290;
/// `_atexit` and the exit thunk registered for the null pointer static.
const ATEXIT: u32 = 0x00ec_658f;
const NULL_POINTER_EXIT: u32 = 0x00fd_d690;

/// Import slots: `InterlockedIncrement`, `InterlockedDecrement`,
/// `GetTickCount`, `TryEnterCriticalSection`, `LeaveCriticalSection`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const GET_TICK_COUNT: u32 = 0x00fd_f060;
const TRY_ENTER_CRITICAL_SECTION: u32 = 0x00fd_f1b0;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;

/// `NiRefObject::DeleteThis` (vtable slot 4), run when a count reaches zero.
const SLOT_DELETE_THIS: u32 = 0x04;

/// Starts a reference: the count is the word at +4.
fn add_ref(e: &mut Engine, object: u32) {
    e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
}

/// Drops a reference; `DeleteThis` runs when the count reaches zero.
fn release(e: &mut Engine, object: u32) {
    let count = e
        .call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32();
    if count == 0 {
        e.vcall(object, SLOT_DELETE_THIS, &args![]);
    }
}

/// Leaves the guard the way the callers inline it: decrement the count and
/// clear the owner when it reaches zero.
fn guard_leave(e: &mut Engine) {
    let count = e.mem.u32(GUARD_COUNT).wrapping_sub(1);
    e.mem.set_u32(GUARD_COUNT, count);
    if count == 0 {
        e.mem.set_u32(GUARD, 0);
    }
}

/// The array's element pointer plus `index` elements.
fn slot_address(e: &Engine, array: Ptr<NiTArray>, index: u32) -> u32 {
    e.get(array, NiTArray::m_pBase)
        .wrapping_add(index.wrapping_mul(4))
}

/// Assigns a null smart pointer to the element at `index` (a null temporary
/// is built, assigned, and destroyed; its construction and destruction do
/// nothing for a null pointer).
fn clear_slot(e: &mut Engine, array: Ptr<NiTArray>, index: u32) {
    let slot = slot_address(e, array, index);
    e.with_stack(4, |e, null_holder| {
        fn_00c46270(e, Ptr::new(slot), null_holder);
    });
}

// Translated from 00c457e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of a 0xb0-byte object (the engine map has
/// no name; Ghidra's library name for the shared body is not trusted): runs
/// the destructor body `00c5faa0` and, when bit 0 of `flags` is set, frees
/// the 0xb0 bytes. Returns `this`.
pub fn fn_00c457e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE_SIZED, &args![this, 0xb0u32]);
    }
    this
}

// Translated from 00c45820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores its argument in the word `011af758` (cdecl), which
/// [`fn_00c458f0`] adds to the tick count to get its time limit.
pub fn fn_00c45820(e: &mut Engine, value: u32) {
    e.mem.set_u32(TIME_BUDGET, value);
}

// Translated from 00c45830 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c45830 - C++ exception unwinding frame (FS:[0]) is not translated.
/// Adds `object` (cdecl; may be null) to the global array under the guard:
/// enters the guard, takes a reference in a temporary smart pointer, appends
/// it with [`fn_00c45e00`], drops the temporary and leaves the guard.
pub fn fn_00c45830(e: &mut Engine, object: Ptr) {
    e.call(GUARD_ENTER, &args![GUARD, 0u32]);
    e.with_stack(4, |e, holder| {
        e.mem.set_u32(holder.addr(), object.addr());
        if !object.is_null() {
            add_ref(e, object.addr());
        }
        fn_00c45e00(e, Ptr::new(GLOBAL_ARRAY), holder);
        let held = e.mem.u32(holder.addr());
        if held != 0 {
            release(e, held);
        }
    });
    guard_leave(e);
}

// Translated from 00c458f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Trims the global array within a time limit (cdecl, no arguments): under
/// the guard, from the last element down, removes elements with
/// [`fn_00c45e50`] and drops the references they return until either the
/// element count (read once, `01202e2c`) is used up or the tick count reaches
/// the start tick plus `011af758` (plus 1000 more when the count was above
/// 200); when the count is used up it also runs [`fn_00c45a80`].
pub fn fn_00c458f0(e: &mut Engine) {
    e.call(GUARD_ENTER, &args![GUARD, 0u32]);
    let array = Ptr::new(GLOBAL_ARRAY);
    let start = e.call(GET_TICK_COUNT, &args![]).u32();
    let mut deadline = start.wrapping_add(e.mem.u32(TIME_BUDGET));
    let mut now = 0u32;
    let mut remaining = e.get(array, NiTArray::m_usESize) as u32;
    if remaining > 200 {
        deadline = deadline.wrapping_add(1000);
    }
    while remaining != 0 && now < deadline {
        e.with_stack(4, |e, removed| {
            fn_00c45e50(e, array, removed, remaining - 1);
            let object = e.mem.u32(removed.addr());
            if object != 0 {
                release(e, object);
            }
        });
        now = e.call(GET_TICK_COUNT, &args![]).u32();
        remaining -= 1;
    }
    if remaining == 0 {
        fn_00c45a80(e);
    }
    guard_leave(e);
}

// Translated from 00c459d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the global array (cdecl, one byte argument): with `try_only` set
/// it only tries to enter the guard (`0078d200`) and does nothing when that
/// fails (the guard is then not held), otherwise it enters the guard. With
/// the guard held it tries the critical section `011f4380`; when that is
/// entered it clears the array ([`fn_00c46080`]), trims it
/// ([`fn_00c45a80`]) and leaves the critical section. The guard is left on
/// every path that held it.
pub fn fn_00c459d0(e: &mut Engine, try_only: bool) {
    let held = if try_only {
        e.call(GUARD_TRY_ENTER, &args![GUARD]).bool()
    } else {
        e.call(GUARD_ENTER, &args![GUARD, 0u32]);
        true
    };
    if held {
        let entered = e
            .call(TRY_ENTER_CRITICAL_SECTION, &args![CRITICAL_SECTION])
            .u32()
            != 0;
        if entered {
            fn_00c46080(e, Ptr::new(GLOBAL_ARRAY));
            fn_00c45a80(e);
            e.call(LEAVE_CRITICAL_SECTION, &args![CRITICAL_SECTION]);
        }
        guard_leave(e);
    }
}

// Translated from 00c45a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Shrinks the global array's capacity to 100 when its size (`01202e2a`) is
/// above 100.
pub fn fn_00c45a80(e: &mut Engine) {
    let array = Ptr::new(GLOBAL_ARRAY);
    if e.get(array, NiTArray::m_usSize) > 100 {
        fn_00c45b20(e, array, 100);
    }
}

// Translated from 00c45ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the global array holds no elements (its count `01202e2c` is
/// zero); returned in `AL`.
pub fn fn_00c45ab0(e: &mut Engine) -> bool {
    e.get(Ptr::<NiTArray>::new(GLOBAL_ARRAY), NiTArray::m_usESize) == 0
}

// Translated from 00c45ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The global array's element count (`01202e2c`, 16 bits).
pub fn fn_00c45ad0(e: &mut Engine) -> u16 {
    e.get(Ptr::<NiTArray>::new(GLOBAL_ARRAY), NiTArray::m_usESize)
}

// Translated from 00c45ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Grows the global array's capacity to `size` when its size (`01202e2a`) is
/// below it (cdecl, unsigned compare).
pub fn fn_00c45ae0(e: &mut Engine, size: u32) {
    let array = Ptr::new(GLOBAL_ARRAY);
    if (e.get(array, NiTArray::m_usSize) as u32) < size {
        fn_00c45b20(e, array, size);
    }
}

// Translated from 00c45b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the global array's growth step (`01202e2e`) from the low 16 bits of
/// its argument (cdecl).
pub fn fn_00c45b10(e: &mut Engine, grow_by: u16) {
    e.set(
        Ptr::<NiTArray>::new(GLOBAL_ARRAY),
        NiTArray::m_usGrowBy,
        grow_by,
    );
}

// Translated from 00c45b20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c45b20 - C++ exception unwinding frame (FS:[0]) is not translated.
/// Sets the capacity of a smart-pointer array (`this` an array laid out like
/// `NiTArray`) to `new_max`, as the `SetSize` of that template does: nothing
/// when it is the current capacity; when it is below the size, the elements
/// from `new_max` up are assigned null (the count of non-null elements drops
/// for each one that was set) and the size becomes `new_max`; then a new
/// block of `new_max` elements is allocated ([`fn_00c46170`], or no block for
/// zero), the elements below the size are copied over with the smart-pointer
/// assignment, the rest are assigned null, and the old block is destroyed
/// ([`fn_00c46420`] with flags 3).
pub fn fn_00c45b20(e: &mut Engine, this: Ptr<NiTArray>, new_max: u32) {
    if new_max == e.get(this, NiTArray::m_usMaxSize) as u32 {
        return;
    }
    if new_max < e.get(this, NiTArray::m_usSize) as u32 {
        let mut index = new_max as u16;
        while (index as u32) < e.get(this, NiTArray::m_usSize) as u32 {
            let slot = slot_address(e, this, index as u32);
            if e.mem.u32(slot) != 0 {
                clear_slot(e, this, index as u32);
                let count = e.get(this, NiTArray::m_usESize).wrapping_sub(1);
                e.set(this, NiTArray::m_usESize, count);
            }
            index = index.wrapping_add(1);
        }
        e.set(this, NiTArray::m_usSize, new_max as u16);
    }
    let old_base = e.get(this, NiTArray::m_pBase);
    e.set(this, NiTArray::m_usMaxSize, new_max as u16);
    if new_max != 0 {
        let capacity = e.get(this, NiTArray::m_usMaxSize) as u32;
        let fresh = fn_00c46170(e, capacity);
        e.set(this, NiTArray::m_pBase, fresh.addr());
        let mut index = 0u16;
        while (index as u32) < e.get(this, NiTArray::m_usSize) as u32 {
            let destination = slot_address(e, this, index as u32);
            let source = old_base.wrapping_add((index as u32).wrapping_mul(4));
            fn_00c46270(e, Ptr::new(destination), Ptr::new(source));
            index = index.wrapping_add(1);
        }
        index = e.get(this, NiTArray::m_usSize);
        while (index as u32) < e.get(this, NiTArray::m_usMaxSize) as u32 {
            clear_slot(e, this, index as u32);
            index = index.wrapping_add(1);
        }
    } else {
        e.set(this, NiTArray::m_pBase, 0);
    }
    if old_base != 0 {
        fn_00c46420(e, Ptr::new(old_base), 3);
    }
}

// Translated from 00c45e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends the smart pointer `holder` (a pointer to the pointer) to the
/// array: the index is the current size; when the size has reached the
/// capacity the array first grows to the size plus its growth step
/// ([`fn_00c45b20`]). The element is stored with [`fn_00c462f0`]. Returns the
/// index.
pub fn fn_00c45e00(e: &mut Engine, this: Ptr<NiTArray>, holder: Ptr) -> u32 {
    let index = e.get(this, NiTArray::m_usSize) as u32;
    if index >= e.get(this, NiTArray::m_usMaxSize) as u32 {
        let grow_by = e.get(this, NiTArray::m_usGrowBy) as u32;
        fn_00c45b20(e, this, grow_by.wrapping_add(index));
    }
    fn_00c462f0(e, this, index, holder);
    index
}

// Translated from 00c45e50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c45e50 - C++ exception unwinding frame (FS:[0]) is not translated.
/// Removes the element at `index` and returns it through `out` (a pointer to
/// a smart pointer; returned): an index at or above the size stores null.
/// Otherwise the element is held, the slot is assigned null, the count of
/// non-null elements drops when the element was set, the size drops by one
/// when `index` was the last used slot, and `out` receives the element with
/// its own reference (the temporary reference is then dropped).
pub fn fn_00c45e50(e: &mut Engine, this: Ptr<NiTArray>, out: Ptr, index: u32) -> Ptr {
    if index >= e.get(this, NiTArray::m_usSize) as u32 {
        e.mem.set_u32(out.addr(), 0);
        return out;
    }
    let element = e.mem.u32(slot_address(e, this, index));
    if element != 0 {
        add_ref(e, element);
    }
    clear_slot(e, this, index);
    if element != 0 {
        let count = e.get(this, NiTArray::m_usESize).wrapping_sub(1);
        e.set(this, NiTArray::m_usESize, count);
    }
    let size = e.get(this, NiTArray::m_usSize);
    if index == size.wrapping_sub(1) as u32 {
        e.set(this, NiTArray::m_usSize, size.wrapping_sub(1));
    }
    e.mem.set_u32(out.addr(), element);
    let result = e.mem.u32(out.addr());
    if result != 0 {
        add_ref(e, result);
    }
    if element != 0 {
        release(e, element);
    }
    out
}

// Translated from 00c46080 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c46080 - C++ exception unwinding frame (FS:[0]) is not translated.
/// Empties the array: every slot below the size (re-read each step) is
/// assigned null, then the size and the element count are set to zero.
pub fn fn_00c46080(e: &mut Engine, this: Ptr<NiTArray>) {
    let mut index = 0u16;
    while (index as u32) < e.get(this, NiTArray::m_usSize) as u32 {
        clear_slot(e, this, index as u32);
        index = index.wrapping_add(1);
    }
    e.set(this, NiTArray::m_usSize, 0);
    e.set(this, NiTArray::m_usESize, 0);
}

// Translated from 00c46170 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c46170 - C++ exception unwinding frame (FS:[0]) is not translated.
/// Allocates an array of `count` smart pointers (cdecl): `count * 4 + 4`
/// bytes (a product or sum that overflows saturates to 0xffffffff) from the
/// memory manager, stores the count in the first word, constructs the
/// elements with the smart-pointer constructor ([`fn_00c46230`]) and returns
/// the address of the first element (null when the allocation failed).
pub fn fn_00c46170(e: &mut Engine, count: u32) -> Ptr {
    let product = count as u64 * 4;
    let bytes = if product >> 32 != 0 {
        u32::MAX
    } else {
        product as u32
    };
    let size = bytes.saturating_add(4);
    let block = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.mem.set_u32(block, count);
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![
            block + 4,
            4u32,
            count,
            0x00c4_6230u32,
            SMART_POINTER_DESTRUCTOR
        ],
    );
    Ptr::new(block + 4)
}

// Translated from 00c46230 (decompiled, FalloutNV.exe 1.4.0.525)
/// The smart-pointer constructor: stores null (the increment that would
/// follow for a non-null pointer never runs).
pub fn fn_00c46230(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0);
}

// Translated from 00c46270 (decompiled, FalloutNV.exe 1.4.0.525)
/// The smart-pointer assignment: `this` is the holder, `other` a pointer to
/// the holder to copy from. When they hold different objects the old one is
/// released (`DeleteThis` at a count of zero), the new one stored and
/// referenced. Returns `this`.
pub fn fn_00c46270(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    if e.mem.u32(this.addr()) != e.mem.u32(other.addr()) {
        let old = e.mem.u32(this.addr());
        if old != 0 {
            release(e, old);
        }
        let new = e.mem.u32(other.addr());
        e.mem.set_u32(this.addr(), new);
        let stored = e.mem.u32(this.addr());
        if stored != 0 {
            add_ref(e, stored);
        }
    }
    this
}

// Translated from 00c462f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the smart pointer `holder` (a pointer to the pointer) at `index`
/// and keeps the counts: the first call builds the function-local static
/// null pointer (`01202e14`, guard `01202e18` bit 0, exit thunk
/// `00fdd690`). An index below the size adjusts the element count by whether
/// the old and new values are null; an index at or above the size sets the
/// size to `index + 1` (16 bits) and counts the element when it is not null.
/// The slot is then assigned with [`fn_00c46270`].
pub fn fn_00c462f0(e: &mut Engine, this: Ptr<NiTArray>, index: u32, holder: Ptr) {
    if e.mem.u32(NULL_POINTER_GUARD) & 1 == 0 {
        let guard = e.mem.u32(NULL_POINTER_GUARD) | 1;
        e.mem.set_u32(NULL_POINTER_GUARD, guard);
        e.mem.set_u32(NULL_POINTER_VALUE, 0);
        e.call(ATEXIT, &args![NULL_POINTER_EXIT]);
    }
    let null_value = e.mem.u32(NULL_POINTER_VALUE);
    let new_is_set = e.mem.u32(holder.addr()) != null_value;
    if index < e.get(this, NiTArray::m_usSize) as u32 {
        let old_is_set = e.mem.u32(slot_address(e, this, index)) != null_value;
        if !new_is_set {
            if old_is_set {
                let count = e.get(this, NiTArray::m_usESize).wrapping_sub(1);
                e.set(this, NiTArray::m_usESize, count);
            }
        } else if !old_is_set {
            let count = e.get(this, NiTArray::m_usESize).wrapping_add(1);
            e.set(this, NiTArray::m_usESize, count);
        }
    } else {
        e.set(this, NiTArray::m_usSize, index.wrapping_add(1) as u16);
        if new_is_set {
            let count = e.get(this, NiTArray::m_usESize).wrapping_add(1);
            e.set(this, NiTArray::m_usESize, count);
        }
    }
    let slot = slot_address(e, this, index);
    fn_00c46270(e, Ptr::new(slot), holder);
}

// Translated from 00c46420 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar/vector deleting destructor of a smart-pointer array block
/// (`this` the first element). With bit 1 of `flags` set it destroys the
/// `this[-1]` elements (`_eh_vector_destructor_iterator_`) and, with bit 0,
/// frees the block at `this - 4`; returns `this - 4`. Otherwise it releases
/// the one pointer held at `this` and, with bit 0, frees `this`; returns
/// `this`.
pub fn fn_00c46420(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    if flags & 2 != 0 {
        let count = e.mem.u32(this.addr().wrapping_sub(4));
        e.call(
            VECTOR_DESTRUCTOR_ITERATOR,
            &args![this, 4u32, count, SMART_POINTER_DESTRUCTOR],
        );
        if flags & 1 != 0 {
            e.call(
                MEMORY_DEALLOCATE,
                &args![MEMORY_MANAGER, this.addr().wrapping_sub(4)],
            );
        }
        return Ptr::new(this.addr().wrapping_sub(4));
    }
    let held = e.mem.u32(this.addr());
    if held != 0 {
        release(e, held);
    }
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c457e0, fn_00c457e0(Ptr, u32) -> Ptr),
        entry!(0x00c45820, fn_00c45820(u32)),
        entry!(0x00c45830, fn_00c45830(Ptr)),
        entry!(0x00c458f0, fn_00c458f0()),
        entry!(0x00c459d0, fn_00c459d0(bool)),
        entry!(0x00c45a80, fn_00c45a80()),
        entry!(0x00c45ab0, fn_00c45ab0() -> bool),
        entry!(0x00c45ad0, fn_00c45ad0() -> u16),
        entry!(0x00c45ae0, fn_00c45ae0(u32)),
        entry!(0x00c45b10, fn_00c45b10(u16)),
        entry!(0x00c45b20, fn_00c45b20(Ptr<NiTArray>, u32)),
        entry!(0x00c45e00, fn_00c45e00(Ptr<NiTArray>, Ptr) -> u32),
        entry!(0x00c45e50, fn_00c45e50(Ptr<NiTArray>, Ptr, u32) -> Ptr),
        entry!(0x00c46080, fn_00c46080(Ptr<NiTArray>)),
        entry!(0x00c46170, fn_00c46170(u32) -> Ptr),
        entry!(0x00c46230, fn_00c46230(Ptr)),
        entry!(0x00c46270, fn_00c46270(Ptr, Ptr) -> Ptr),
        entry!(0x00c462f0, fn_00c462f0(Ptr<NiTArray>, u32, Ptr)),
        entry!(0x00c46420, fn_00c46420(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The vtable all test objects share and the function it runs for
    /// `DeleteThis`.
    const OBJECT_VTABLE: u32 = 0x7000_0000;
    const DELETE_THIS: u32 = 0x7000_1000;

    /// Objects `DeleteThis` ran on.
    type Deleted = Rc<RefCell<Vec<u32>>>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// An engine with the globals mapped and stand-ins for the imports and
    /// the CRT/guard callees outside this file. Returns the list of objects
    /// deleted.
    fn engine() -> (Engine, Deleted) {
        let mut e = Engine::new();
        e.map(0x0120_2e00, 0x100);
        e.map(TIME_BUDGET, 4);
        e.map(CRITICAL_SECTION, 0x20);
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], value);
            ret(value)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], value);
            ret(value)
        });
        // The guard: entering sets the owner and counts.
        e.register(GUARD_ENTER, |e, a| {
            assert_eq!(a, [GUARD, 0]);
            e.mem.set_u32(GUARD, 77);
            let count = e.mem.u32(GUARD_COUNT) + 1;
            e.mem.set_u32(GUARD_COUNT, count);
            ret(0)
        });
        e.register(ATEXIT, |_, _| ret(0));
        // The vector helpers run the element routine over the block.
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |e, a| {
            for i in 0..a[2] {
                e.call(a[3], &[a[0] + i * a[1]]);
            }
            ret(0)
        });
        e.register(VECTOR_DESTRUCTOR_ITERATOR, |e, a| {
            for i in 0..a[2] {
                e.call(a[3], &[a[0] + i * a[1]]);
            }
            ret(0)
        });
        // The smart-pointer destructor: releases what the holder holds.
        e.register(SMART_POINTER_DESTRUCTOR, |e, a| {
            let held = e.mem.u32(a[0]);
            if held != 0 {
                release(e, held);
            }
            ret(0)
        });
        let deleted: Deleted = Rc::new(RefCell::new(vec![]));
        let log = deleted.clone();
        e.register_double(DELETE_THIS, move |_, a| {
            log.borrow_mut().push(a[0]);
            ret(0)
        });
        e.put_vtable(OBJECT_VTABLE, &[0, DELETE_THIS]);
        (e, deleted)
    }

    /// A reference-counted object with `count` references.
    fn object(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, OBJECT_VTABLE);
        e.mem.set_u32(object + 4, count);
        object
    }

    fn count_of(e: &Engine, object: u32) -> u32 {
        e.mem.u32(object + 4)
    }

    /// The global array with room for `capacity` elements, all null.
    fn global_array(e: &mut Engine, capacity: u32) -> Ptr<NiTArray> {
        let array = Ptr::<NiTArray>::new(GLOBAL_ARRAY);
        let base = e.call(0x00c46170, &args![capacity]).u32();
        e.set(array, NiTArray::m_pBase, base);
        e.set(array, NiTArray::m_usMaxSize, capacity as u16);
        e.set(array, NiTArray::m_usGrowBy, 4);
        array
    }

    /// Stores `object` (with a reference taken) at `index` the way the game
    /// does, through `00c462f0`.
    fn store(e: &mut Engine, array: Ptr<NiTArray>, index: u32, object: u32) {
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, object);
        e.call(0x00c462f0, &args![array, index, Ptr::<()>::new(holder)]);
    }

    #[test]
    fn deleting_destructor_runs_the_body_and_frees_on_bit_0() {
        let (mut e, _) = engine();
        e.register(DESTRUCTOR_BODY, |_, _| ret(0));
        e.register(OPERATOR_DELETE_SIZED, |_, _| ret(0));
        e.call_log = Some(vec![]);
        let result = e.call(0x00c457e0, &args![Ptr::<()>::new(0x1234), 1u32]);
        assert_eq!(result.u32(), 0x1234);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log,
            vec![
                (0x00c457e0, vec![0x1234, 1]),
                (DESTRUCTOR_BODY, vec![0x1234]),
                (OPERATOR_DELETE_SIZED, vec![0x1234, 0xb0]),
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x00c457e0, &args![Ptr::<()>::new(0x1234), 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn sets_the_time_budget_word() {
        let (mut e, _) = engine();
        e.call(0x00c45820, &args![0x1234u32]);
        assert_eq!(e.mem.u32(TIME_BUDGET), 0x1234);
    }

    #[test]
    fn adds_an_object_under_the_guard_keeping_one_reference() {
        let (mut e, deleted) = engine();
        let array = global_array(&mut e, 4);
        let item = object(&mut e, 1);
        e.call(0x00c45830, &args![Ptr::<()>::new(item)]);
        assert_eq!(e.get(array, NiTArray::m_usSize), 1);
        assert_eq!(e.get(array, NiTArray::m_usESize), 1);
        assert_eq!(e.mem.u32(slot_address(&e, array, 0)), item);
        // One reference from the caller, one from the array; the temporary's
        // is gone.
        assert_eq!(count_of(&e, item), 2);
        assert!(deleted.borrow().is_empty());
        // The guard was left.
        assert_eq!(e.mem.u32(GUARD_COUNT), 0);
        assert_eq!(e.mem.u32(GUARD), 0);
        // A null object is stored without touching any count.
        e.call(0x00c45830, &args![Ptr::<()>::NULL]);
        assert_eq!(e.get(array, NiTArray::m_usSize), 2);
        assert_eq!(e.get(array, NiTArray::m_usESize), 1);
    }

    #[test]
    fn adding_grows_the_array_by_its_growth_step_when_full() {
        let (mut e, _) = engine();
        let array = global_array(&mut e, 1);
        let first = object(&mut e, 1);
        let second = object(&mut e, 1);
        e.call(0x00c45830, &args![Ptr::<()>::new(first)]);
        e.call(0x00c45830, &args![Ptr::<()>::new(second)]);
        assert_eq!(e.get(array, NiTArray::m_usMaxSize), 5);
        assert_eq!(e.get(array, NiTArray::m_usSize), 2);
        assert_eq!(e.mem.u32(slot_address(&e, array, 0)), first);
        assert_eq!(e.mem.u32(slot_address(&e, array, 1)), second);
        // Moving the elements to the new block took no extra references.
        assert_eq!(count_of(&e, first), 2);
    }

    #[test]
    fn timed_trim_stops_at_the_deadline() {
        let (mut e, _) = engine();
        let array = global_array(&mut e, 8);
        let items: Vec<u32> = (0..3).map(|_| object(&mut e, 1)).collect();
        for (i, item) in items.iter().enumerate() {
            store(&mut e, array, i as u32, *item);
        }
        e.mem.set_u32(TIME_BUDGET, 50);
        // The clock: 100 at the start, then 110, then past the deadline.
        let ticks = Rc::new(RefCell::new(vec![100u32, 110, 200].into_iter()));
        let source = ticks.clone();
        e.register_double(GET_TICK_COUNT, move |_, _| {
            ret(source.borrow_mut().next().unwrap())
        });
        e.call(0x00c458f0, &args![]);
        // Two elements were taken off the end, then the time ran out.
        assert_eq!(e.get(array, NiTArray::m_usESize), 1);
        assert_eq!(e.get(array, NiTArray::m_usSize), 1);
        assert_eq!(e.mem.u32(slot_address(&e, array, 0)), items[0]);
        assert_eq!(count_of(&e, items[2]), 1);
        assert_eq!(e.mem.u32(GUARD_COUNT), 0);
    }

    #[test]
    fn timed_trim_runs_the_shrink_when_the_count_is_used_up() {
        let (mut e, _) = engine();
        let array = global_array(&mut e, 120);
        let item = object(&mut e, 1);
        store(&mut e, array, 0, item);
        e.set(array, NiTArray::m_usSize, 101);
        e.mem.set_u32(TIME_BUDGET, 1000);
        e.register(GET_TICK_COUNT, |_, _| ret(5));
        e.call(0x00c458f0, &args![]);
        assert_eq!(e.get(array, NiTArray::m_usESize), 0);
        // The removed slot was not the last used one, so the size stayed 101;
        // the count reached zero, so the trim ran and cut it to 100.
        assert_eq!(e.get(array, NiTArray::m_usSize), 100);
        assert_eq!(e.get(array, NiTArray::m_usMaxSize), 100);
    }

    #[test]
    fn timed_trim_adds_a_second_when_the_count_is_above_200() {
        let (mut e, _) = engine();
        let array = global_array(&mut e, 4);
        e.set(array, NiTArray::m_usESize, 201);
        e.set(array, NiTArray::m_usSize, 0);
        e.mem.set_u32(TIME_BUDGET, 0);
        // The clock stays at 500: the deadline 500 + 0 + 1000 is never reached,
        // so all 201 steps run (each removing from an index past the size).
        e.register(GET_TICK_COUNT, |_, _| ret(500));
        e.call(0x00c458f0, &args![]);
        // Every step took the "index past the size" path and left the count.
        assert_eq!(e.get(array, NiTArray::m_usESize), 201);
        // The loop ended by count, so the shrink check ran (size 0: nothing).
        assert_eq!(e.get(array, NiTArray::m_usMaxSize), 4);
    }

    #[test]
    fn flush_clears_and_trims_when_the_critical_section_is_entered() {
        let (mut e, _) = engine();
        let array = global_array(&mut e, 4);
        let item = object(&mut e, 1);
        store(&mut e, array, 0, item);
        e.register(TRY_ENTER_CRITICAL_SECTION, |_, a| {
            assert_eq!(a, [CRITICAL_SECTION]);
            ret(1)
        });
        e.register(LEAVE_CRITICAL_SECTION, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x00c459d0, &args![false]);
        let log: Vec<u32> = e.call_log.take().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            log,
            vec![
                0x00c459d0,
                GUARD_ENTER,
                TRY_ENTER_CRITICAL_SECTION,
                INTERLOCKED_DECREMENT,
                LEAVE_CRITICAL_SECTION
            ]
        );
        assert_eq!(e.get(array, NiTArray::m_usSize), 0);
        assert_eq!(e.get(array, NiTArray::m_usESize), 0);
        assert_eq!(count_of(&e, item), 1);
        assert_eq!(e.mem.u32(GUARD_COUNT), 0);
        assert_eq!(e.mem.u32(GUARD), 0);
    }

    #[test]
    fn flush_leaves_the_guard_when_the_critical_section_is_busy() {
        let (mut e, _) = engine();
        let array = global_array(&mut e, 4);
        let item = object(&mut e, 1);
        store(&mut e, array, 0, item);
        e.register(TRY_ENTER_CRITICAL_SECTION, |_, _| ret(0));
        e.register(LEAVE_CRITICAL_SECTION, |_, _| panic!("not entered"));
        e.call(0x00c459d0, &args![false]);
        assert_eq!(e.get(array, NiTArray::m_usSize), 1);
        assert_eq!(e.mem.u32(GUARD_COUNT), 0);
    }

    #[test]
    fn flush_with_try_only_does_nothing_when_the_guard_is_busy() {
        let (mut e, _) = engine();
        let array = global_array(&mut e, 4);
        let item = object(&mut e, 1);
        store(&mut e, array, 0, item);
        e.mem.set_u32(GUARD_COUNT, 3);
        e.register(GUARD_TRY_ENTER, |_, a| {
            assert_eq!(a, [GUARD]);
            ret(0)
        });
        e.register(TRY_ENTER_CRITICAL_SECTION, |_, _| panic!("guard not held"));
        e.call(0x00c459d0, &args![true]);
        // The guard's count is untouched: it was never entered.
        assert_eq!(e.mem.u32(GUARD_COUNT), 3);
        assert_eq!(e.get(array, NiTArray::m_usSize), 1);
        // With the guard obtained it works as the plain flush and leaves it.
        e.mem.set_u32(GUARD_COUNT, 1);
        e.register(GUARD_TRY_ENTER, |_, _| ret(1));
        e.register(TRY_ENTER_CRITICAL_SECTION, |_, _| ret(1));
        e.register(LEAVE_CRITICAL_SECTION, |_, _| ret(0));
        e.call(0x00c459d0, &args![true]);
        assert_eq!(e.get(array, NiTArray::m_usSize), 0);
        assert_eq!(e.mem.u32(GUARD_COUNT), 0);
        assert_eq!(e.mem.u32(GUARD), 0);
    }

    #[test]
    fn shrink_to_100_only_above_100() {
        let (mut e, _) = engine();
        let array = global_array(&mut e, 150);
        e.set(array, NiTArray::m_usSize, 100);
        e.call(0x00c45a80, &args![]);
        assert_eq!(e.get(array, NiTArray::m_usMaxSize), 150);
        e.set(array, NiTArray::m_usSize, 101);
        e.call(0x00c45a80, &args![]);
        assert_eq!(e.get(array, NiTArray::m_usMaxSize), 100);
        assert_eq!(e.get(array, NiTArray::m_usSize), 100);
    }

    #[test]
    fn empty_test_and_count_getters_read_the_element_count() {
        let (mut e, _) = engine();
        let array = Ptr::<NiTArray>::new(GLOBAL_ARRAY);
        assert!(e.call(0x00c45ab0, &args![]).bool());
        assert_eq!(e.call(0x00c45ad0, &args![]).u16(), 0);
        e.set(array, NiTArray::m_usESize, 0x0102);
        assert!(!e.call(0x00c45ab0, &args![]).bool());
        assert_eq!(e.call(0x00c45ad0, &args![]).u16(), 0x0102);
    }

    #[test]
    fn grow_to_size_only_when_the_size_is_below_it() {
        let (mut e, _) = engine();
        let array = global_array(&mut e, 4);
        e.set(array, NiTArray::m_usSize, 3);
        e.call(0x00c45ae0, &args![3u32]);
        assert_eq!(e.get(array, NiTArray::m_usMaxSize), 4);
        e.call(0x00c45ae0, &args![9u32]);
        assert_eq!(e.get(array, NiTArray::m_usMaxSize), 9);
    }

    #[test]
    fn grow_by_takes_the_low_16_bits() {
        let (mut e, _) = engine();
        e.call(0x00c45b10, &args![0x0001_0007u32]);
        assert_eq!(
            e.get(Ptr::<NiTArray>::new(GLOBAL_ARRAY), NiTArray::m_usGrowBy),
            7
        );
    }

    #[test]
    fn set_size_grows_keeping_elements_and_references() {
        let (mut e, deleted) = engine();
        let array = global_array(&mut e, 2);
        let a = object(&mut e, 1);
        let b = object(&mut e, 1);
        store(&mut e, array, 0, a);
        store(&mut e, array, 1, b);
        let old_base = e.get(array, NiTArray::m_pBase);
        e.call(0x00c45b20, &args![array, 6u32]);
        assert_eq!(e.get(array, NiTArray::m_usMaxSize), 6);
        assert_eq!(e.get(array, NiTArray::m_usSize), 2);
        let base = e.get(array, NiTArray::m_pBase);
        assert_ne!(base, old_base);
        assert_eq!(e.mem.u32(base), a);
        assert_eq!(e.mem.u32(base + 4), b);
        assert_eq!(e.mem.u32(base + 8), 0);
        // The old block's holders released theirs: net one reference each.
        assert_eq!(count_of(&e, a), 2);
        assert_eq!(count_of(&e, b), 2);
        assert!(deleted.borrow().is_empty());
        // The old block was freed with its count word.
        assert_eq!(e.mem.block_size(old_base - 4), None);
        // Same capacity: nothing happens.
        e.call_log = Some(vec![]);
        e.call(0x00c45b20, &args![array, 6u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn set_size_below_the_size_drops_the_elements_above_it() {
        let (mut e, deleted) = engine();
        let array = global_array(&mut e, 4);
        let items: Vec<u32> = (0..3).map(|_| object(&mut e, 1)).collect();
        for (i, item) in items.iter().enumerate() {
            store(&mut e, array, i as u32, *item);
        }
        // The caller keeps no references: only the array's one remains.
        for item in &items {
            e.mem.set_u32(item + 4, 1);
        }
        e.call(0x00c45b20, &args![array, 1u32]);
        assert_eq!(e.get(array, NiTArray::m_usMaxSize), 1);
        assert_eq!(e.get(array, NiTArray::m_usSize), 1);
        assert_eq!(e.get(array, NiTArray::m_usESize), 1);
        assert_eq!(e.mem.u32(e.get(array, NiTArray::m_pBase)), items[0]);
        assert_eq!(*deleted.borrow(), vec![items[1], items[2]]);
    }

    #[test]
    fn set_size_to_zero_frees_the_block() {
        let (mut e, _) = engine();
        let array = global_array(&mut e, 3);
        let old_base = e.get(array, NiTArray::m_pBase);
        e.call(0x00c45b20, &args![array, 0u32]);
        assert_eq!(e.get(array, NiTArray::m_usMaxSize), 0);
        assert_eq!(e.get(array, NiTArray::m_pBase), 0);
        assert_eq!(e.mem.block_size(old_base - 4), None);
    }

    #[test]
    fn append_returns_the_index_and_grows_at_capacity() {
        let (mut e, _) = engine();
        let array = global_array(&mut e, 1);
        let a = object(&mut e, 1);
        let b = object(&mut e, 1);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, a);
        assert_eq!(
            e.call(0x00c45e00, &args![array, Ptr::<()>::new(holder)])
                .u32(),
            0
        );
        e.mem.set_u32(holder, b);
        assert_eq!(
            e.call(0x00c45e00, &args![array, Ptr::<()>::new(holder)])
                .u32(),
            1
        );
        // Capacity 1 grew by the growth step (4) from the size 1.
        assert_eq!(e.get(array, NiTArray::m_usMaxSize), 5);
        assert_eq!(e.get(array, NiTArray::m_usESize), 2);
    }

    #[test]
    fn remove_at_returns_the_element_and_shrinks_the_size_when_last() {
        let (mut e, deleted) = engine();
        let array = global_array(&mut e, 4);
        let a = object(&mut e, 1);
        let b = object(&mut e, 1);
        store(&mut e, array, 0, a);
        store(&mut e, array, 1, b);
        // Taking the middle... the last one: the size drops.
        let out = e.mem.alloc(4);
        let returned = e.call(0x00c45e50, &args![array, Ptr::<()>::new(out), 1u32]);
        assert_eq!(returned.u32(), out);
        assert_eq!(e.mem.u32(out), b);
        assert_eq!(e.get(array, NiTArray::m_usSize), 1);
        assert_eq!(e.get(array, NiTArray::m_usESize), 1);
        assert_eq!(e.mem.u32(slot_address(&e, array, 1)), 0);
        // The slot's reference moved to `out`: the count is unchanged.
        assert_eq!(count_of(&e, b), 2);
        // An element that is not last leaves the size.
        store(&mut e, array, 1, b);
        let c = object(&mut e, 1);
        store(&mut e, array, 2, c);
        e.call(0x00c45e50, &args![array, Ptr::<()>::new(out), 1u32]);
        assert_eq!(e.get(array, NiTArray::m_usSize), 3);
        assert_eq!(e.get(array, NiTArray::m_usESize), 2);
        assert!(deleted.borrow().is_empty());
    }

    #[test]
    fn remove_at_an_index_past_the_size_stores_null() {
        let (mut e, _) = engine();
        let array = global_array(&mut e, 4);
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0xdead);
        e.call(0x00c45e50, &args![array, Ptr::<()>::new(out), 0u32]);
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(e.get(array, NiTArray::m_usSize), 0);
    }

    #[test]
    fn remove_at_a_null_slot_keeps_the_count() {
        let (mut e, _) = engine();
        let array = global_array(&mut e, 4);
        let a = object(&mut e, 1);
        store(&mut e, array, 0, a);
        store(&mut e, array, 1, 0);
        let out = e.mem.alloc(4);
        e.call(0x00c45e50, &args![array, Ptr::<()>::new(out), 1u32]);
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(e.get(array, NiTArray::m_usESize), 1);
        assert_eq!(e.get(array, NiTArray::m_usSize), 1);
    }

    #[test]
    fn clear_releases_every_element_and_zeroes_size_and_count() {
        let (mut e, deleted) = engine();
        let array = global_array(&mut e, 4);
        let a = object(&mut e, 1);
        let b = object(&mut e, 1);
        store(&mut e, array, 0, a);
        store(&mut e, array, 2, b);
        e.mem.set_u32(a + 4, 1);
        e.call(0x00c46080, &args![array]);
        assert_eq!(e.get(array, NiTArray::m_usSize), 0);
        assert_eq!(e.get(array, NiTArray::m_usESize), 0);
        assert_eq!(*deleted.borrow(), vec![a]);
        assert_eq!(e.mem.u32(slot_address(&e, array, 0)), 0);
        assert_eq!(e.mem.u32(slot_address(&e, array, 2)), 0);
        assert_eq!(count_of(&e, b), 1);
    }

    #[test]
    fn allocates_the_count_word_and_constructs_the_elements() {
        let (mut e, _) = engine();
        e.call_log = Some(vec![]);
        let first = e.call(0x00c46170, &args![3u32]).u32();
        assert_eq!(e.mem.u32(first - 4), 3);
        assert_eq!(e.mem.block_size(first - 4), Some(16));
        assert_eq!(e.mem.bytes(first, 12), vec![0; 12]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.iter()
                .find(|c| c.0 == VECTOR_CONSTRUCTOR_ITERATOR)
                .unwrap()
                .1,
            vec![first, 4, 3, 0x00c4_6230, SMART_POINTER_DESTRUCTOR]
        );
    }

    #[test]
    fn allocation_size_saturates_on_overflow_and_failure_returns_null() {
        let (mut e, _) = engine();
        let sizes = Rc::new(RefCell::new(vec![]));
        let log = sizes.clone();
        e.register_double(MEMORY_ALLOCATE, move |_, a| {
            log.borrow_mut().push(a[1]);
            ret(0)
        });
        for count in [0x4000_0000u32, 0x3fff_ffff, 0xffff_ffff] {
            assert_eq!(e.call(0x00c46170, &args![count]).u32(), 0);
        }
        assert_eq!(*sizes.borrow(), vec![u32::MAX, u32::MAX, u32::MAX]);
    }

    #[test]
    fn element_constructor_stores_null() {
        let (mut e, _) = engine();
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x55);
        e.call(0x00c46230, &args![Ptr::<()>::new(slot)]);
        assert_eq!(e.mem.u32(slot), 0);
    }

    #[test]
    fn assignment_swaps_references_and_skips_equal_values() {
        let (mut e, deleted) = engine();
        let old = object(&mut e, 1);
        let new = object(&mut e, 1);
        let holder = e.mem.alloc(4);
        let source = e.mem.alloc(4);
        e.mem.set_u32(holder, old);
        e.mem.set_u32(source, new);
        let back = e.call(
            0x00c46270,
            &args![Ptr::<()>::new(holder), Ptr::<()>::new(source)],
        );
        assert_eq!(back.u32(), holder);
        assert_eq!(e.mem.u32(holder), new);
        assert_eq!(count_of(&e, new), 2);
        assert_eq!(*deleted.borrow(), vec![old]);
        // The same value again changes nothing.
        e.call(
            0x00c46270,
            &args![Ptr::<()>::new(holder), Ptr::<()>::new(source)],
        );
        assert_eq!(count_of(&e, new), 2);
        // Assigning null releases without referencing.
        e.mem.set_u32(source, 0);
        e.call(
            0x00c46270,
            &args![Ptr::<()>::new(holder), Ptr::<()>::new(source)],
        );
        assert_eq!(e.mem.u32(holder), 0);
        assert_eq!(count_of(&e, new), 1);
    }

    #[test]
    fn store_builds_the_null_static_once_and_counts() {
        let (mut e, _) = engine();
        let array = global_array(&mut e, 4);
        let exits = Rc::new(RefCell::new(0));
        let counter = exits.clone();
        e.register_double(ATEXIT, move |_, a| {
            assert_eq!(a, [NULL_POINTER_EXIT]);
            *counter.borrow_mut() += 1;
            ret(0)
        });
        e.mem.set_u32(NULL_POINTER_VALUE, 0x99);
        let a = object(&mut e, 1);
        store(&mut e, array, 0, a);
        assert_eq!(*exits.borrow(), 1);
        assert_eq!(e.mem.u32(NULL_POINTER_GUARD) & 1, 1);
        assert_eq!(e.mem.u32(NULL_POINTER_VALUE), 0);
        // Index at the size: size becomes index + 1 and the element is counted.
        assert_eq!(e.get(array, NiTArray::m_usSize), 1);
        assert_eq!(e.get(array, NiTArray::m_usESize), 1);
        // Replacing a set slot with another set value keeps the count.
        let b = object(&mut e, 1);
        store(&mut e, array, 0, b);
        assert_eq!(e.get(array, NiTArray::m_usESize), 1);
        // Replacing with null drops it; a second null does not drop again.
        store(&mut e, array, 0, 0);
        assert_eq!(e.get(array, NiTArray::m_usESize), 0);
        store(&mut e, array, 0, 0);
        assert_eq!(e.get(array, NiTArray::m_usESize), 0);
        // Filling a null slot counts again; a gap index sets the size past it.
        store(&mut e, array, 0, a);
        assert_eq!(e.get(array, NiTArray::m_usESize), 1);
        store(&mut e, array, 3, 0);
        assert_eq!(e.get(array, NiTArray::m_usSize), 4);
        assert_eq!(e.get(array, NiTArray::m_usESize), 1);
        assert_eq!(*exits.borrow(), 1);
    }

    #[test]
    fn deleting_destructor_of_a_block_destroys_all_elements() {
        let (mut e, deleted) = engine();
        let first = e.call(0x00c46170, &args![2u32]).u32();
        let a = object(&mut e, 1);
        e.mem.set_u32(first, a);
        let result = e.call(0x00c46420, &args![Ptr::<()>::new(first), 3u32]);
        assert_eq!(result.u32(), first - 4);
        assert_eq!(*deleted.borrow(), vec![a]);
        assert_eq!(e.mem.block_size(first - 4), None);
    }

    #[test]
    fn deleting_destructor_of_a_single_pointer_releases_it() {
        let (mut e, deleted) = engine();
        let holder = e.mem.alloc(4);
        let a = object(&mut e, 1);
        e.mem.set_u32(holder, a);
        let result = e.call(0x00c46420, &args![Ptr::<()>::new(holder), 0u32]);
        assert_eq!(result.u32(), holder);
        assert_eq!(*deleted.borrow(), vec![a]);
        assert!(e.mem.block_size(holder).is_some());
        e.mem.set_u32(holder, 0);
        let result = e.call(0x00c46420, &args![Ptr::<()>::new(holder), 1u32]);
        assert_eq!(result.u32(), holder);
        assert_eq!(e.mem.block_size(holder), None);
    }
}
