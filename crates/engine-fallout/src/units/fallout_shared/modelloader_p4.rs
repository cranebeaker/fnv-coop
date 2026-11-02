//! `fallout shared/modelloader.cpp` (Xbox PDB source unit), part 4: its functions from `00449c00` up to
//! (not including) `ffffffff` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::modelloader`]; anything public there may be used here.
//!
//! State of this file: the whole range is translated. The first 40
//! functions, `00449c00` to `0044b2c0`, are the compiler-emitted glue of the
//! loader's queues and reference-counted handles: the destructor and
//! constructors of the `LockFreeMap` instances,
//! `BSTaskManager<__int64>::CancelTask`, the one-bit flag accessors of the
//! queued files (`cFlags` bits 0 to 5), and the smart pointers of `Model`,
//! `LoadedFile` and two other counted objects. The next 37, `0044b2e0` to
//! `00ae78e0`, are the constructors and destructors of the lock-free queues
//! and their enqueue/dequeue protocol, the settings collection
//! (`SettingCollectionList<Setting>`) and a few small accessors.
//!
//! Not translated: the compiler's exception-unwinding frames (the functions
//! that have one say so).

#[allow(unused_imports)]
use super::modelloader::*;
#[allow(unused_imports)]
use crate::prelude::*;

/// `MemoryManager` deallocation, `__cdecl(block)` (`00401030`).
const MEMORY_FREE: u32 = 0x0040_1030;
/// `InterlockedIncrement` wrapper, `__cdecl(target) -> new value` (`0040b460`).
const INTERLOCKED_INCREMENT: u32 = 0x0040_b460;
/// `InterlockedDecrement` wrapper, `__cdecl(target) -> new value` (`004019a0`).
const INTERLOCKED_DECREMENT: u32 = 0x0040_19a0;
/// `Sleep` wrapper, `__cdecl(milliseconds)` (`0040fca0`).
const SLEEP: u32 = 0x0040_fca0;
/// `NiMemObject` deallocation, `__cdecl(block, size)` (`00aa1460`).
const NI_FREE: u32 = 0x00aa_1460;

/// The `LockFreeMapIterator` base destructor body (`00449a00`): sets the
/// base class's virtual table.
const LOCK_FREE_MAP_ITERATOR_BASE_DESTRUCT: u32 = 0x0044_9a00;
/// The three `LockFreeMap` destructor bodies the wrappers `00449c30`,
/// `00449d40` and `00449ea0` forward to.
const LOCK_FREE_MAP_DESTRUCT_00449C50: u32 = 0x0044_9c50;
const LOCK_FREE_MAP_DESTRUCT_00449D60: u32 = 0x0044_9d60;
const LOCK_FREE_MAP_DESTRUCT_00449EC0: u32 = 0x0044_9ec0;
/// The three `LockFreeMap` base constructors (`__thiscall(a, b, c)`) and the
/// virtual tables the instance constructors store after them.
const LOCK_FREE_MAP_CONSTRUCT_0044C040: u32 = 0x0044_c040;
const LOCK_FREE_MAP_CONSTRUCT_0044C270: u32 = 0x0044_c270;
const LOCK_FREE_MAP_CONSTRUCT_0044C920: u32 = 0x0044_c920;
const LOCK_FREE_MAP_VTABLE_00449F50: u32 = 0x0101_72dc;
const LOCK_FREE_MAP_VTABLE_00449FA0: u32 = 0x0101_732c;
const LOCK_FREE_MAP_VTABLE_0044A100: u32 = 0x0101_737c;
/// The routine `0044a130` hands the result of its virtual call to,
/// `__thiscall(this, value, a, b, c) -> byte` (`RET 0x10`).
const LOCK_FREE_MAP_HANDLE: u32 = 0x0044_d240;
/// The byte global `0044ada0` writes (`01202df0`).
const TASK_MANAGER_FLAG: u32 = 0x0120_2df0;
/// Tries to move a task's state (+0xC) to `6`, `__thiscall(task, state,
/// 6) -> bool` (`00449190`; its body is a compare-and-exchange on the state).
const TASK_TRY_SET_STATE: u32 = 0x0044_9190;
/// Adds a reference to a task, `__thiscall(task)` (`0092c870`: increments
/// the counter at +8).
const TASK_ADD_REFERENCE: u32 = 0x0092_c870;
/// `__thiscall(manager, task)` (`0044e050`): takes the task out of the
/// manager's table.
const TASK_MANAGER_REMOVE: u32 = 0x0044_e050;
/// `__thiscall(task)` (`0044dd60`): drops a reference of the task and
/// deletes it (virtual destructor, flag 1) when none is left.
const TASK_RELEASE: u32 = 0x0044_dd60;
/// Slot (byte offset) of the virtual function `CancelTask` calls on a task
/// with `(state, argument)`.
const TASK_CANCEL_SLOT: u32 = 0xc;
/// The state `CancelTask` moves a task to.
const TASK_STATE_CANCELLED: u32 = 6;
/// `__thiscall(element)` (`00442600`), run on every element of the array of
/// `0044adb0`.
const ARRAY_ELEMENT_ACTION: u32 = 0x0044_2600;
/// The stream's binary output, `__thiscall(stream) -> output` (`008d8b00`:
/// the dword at +0x250), and the routine that writes `count` four-byte
/// values to it, `__cdecl(output, values, count)` (`0044e0f0`).
const STREAM_OUTPUT: u32 = 0x008d_8b00;
const WRITE_BINARY: u32 = 0x0044_e0f0;
/// The release and add-reference calls of `Model`'s pointer type
/// (`__thiscall(model)`): `0040c130` and `0040f6e0`.
const MODEL_RELEASE: u32 = 0x0040_c130;
const MODEL_ADD_REFERENCE: u32 = 0x0040_f6e0;
/// The critical section (`011c3b6c`, the address is the object) that guards
/// the reference counter of a `LoadedFile`; entered with the import
/// `TryEnterCriticalSection` (slot `00fdf1b0`) and left with `0082f1f0`.
const LOADED_FILE_LOCK: u32 = 0x011c_3b6c;
const TRY_ENTER_CRITICAL_SECTION: u32 = 0x00fd_f1b0;
const LEAVE_CRITICAL_SECTION: u32 = 0x0082_f1f0;
/// `LoadedFile::~LoadedFile` (Xbox PDB, `0043bb80`).
const LOADED_FILE_DESTRUCT: u32 = 0x0043_bb80;
/// Size of a `LoadedFile` block (`0x10`), as `NiMemObject` frees it.
const LOADED_FILE_SIZE: u32 = 0x10;

/// Sets (`value` non-zero) or clears `mask` in the flag byte at `flags`.
fn write_flag_bit(e: &mut Engine, value: u8, flags: Ptr, mask: u8) {
    let byte = e.mem.u8(flags.addr());
    let byte = if value != 0 {
        byte | mask
    } else {
        byte & !mask
    };
    e.mem.set_u8(flags.addr(), byte);
}

// Translated from 00449c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreeMap<TESObjectREFR*,NiPointer<QueuedHelmet>>::LockFreeMapIterator::
/// scalar deleting destructor` (Xbox PDB): runs the base destructor body
/// and, when bit 0 of `flags` is set, frees the block; returns `this`.
pub fn lock_free_map_iterator_queued_helmet_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(LOCK_FREE_MAP_ITERATOR_BASE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 00449c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of a `LockFreeMap` instance (its scalar deleting destructor is
/// `00449a50`): forwards to the body at `00449c50`.
pub fn fn_00449c30(e: &mut Engine, this: Ptr) {
    e.call(LOCK_FREE_MAP_DESTRUCT_00449C50, &args![this]);
}

// Translated from 00449d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of a `LockFreeMap` instance: forwards to the body at
/// `00449d60`.
pub fn fn_00449d40(e: &mut Engine, this: Ptr) {
    e.call(LOCK_FREE_MAP_DESTRUCT_00449D60, &args![this]);
}

// Translated from 00449ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of a `LockFreeMap` instance: forwards to the body at
/// `00449ec0`.
pub fn fn_00449ea0(e: &mut Engine, this: Ptr) {
    e.call(LOCK_FREE_MAP_DESTRUCT_00449EC0, &args![this]);
}

// Translated from 00449f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a `LockFreeMap` instance: the base constructor
/// `0044c040(a, b, c)`, then the instance's virtual table (`010172dc`);
/// returns `this`.
pub fn fn_00449f50(e: &mut Engine, this: Ptr, a: u32, b: u32, c: u32) -> Ptr {
    e.call(LOCK_FREE_MAP_CONSTRUCT_0044C040, &args![this, a, b, c]);
    e.mem.set_u32(this.addr(), LOCK_FREE_MAP_VTABLE_00449F50);
    this
}

// Translated from 00449fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a `LockFreeMap` instance: the base constructor
/// `0044c270(a, b, c)`, then the virtual table `0101732c`; returns `this`.
pub fn fn_00449fa0(e: &mut Engine, this: Ptr, a: u32, b: u32, c: u32) -> Ptr {
    e.call(LOCK_FREE_MAP_CONSTRUCT_0044C270, &args![this, a, b, c]);
    e.mem.set_u32(this.addr(), LOCK_FREE_MAP_VTABLE_00449FA0);
    this
}

// Translated from 0044a100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a `LockFreeMap` instance: the base constructor
/// `0044c920(a, b, c)`, then the virtual table `0101737c`; returns `this`.
pub fn fn_0044a100(e: &mut Engine, this: Ptr, a: u32, b: u32, c: u32) -> Ptr {
    e.call(LOCK_FREE_MAP_CONSTRUCT_0044C920, &args![this, a, b, c]);
    e.mem.set_u32(this.addr(), LOCK_FREE_MAP_VTABLE_0044A100);
    this
}

// Translated from 0044a130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls slot `0x24` of the object the first word of `this` points to
/// (with `a`), then `0044d240(this, result, a, b, c)`; returns the byte
/// that routine returns. The decompiler shows the arguments of the second
/// call shifted; the disassembly pushes `c`, `b`, `a`, and `a` again for
/// the virtual call, which takes one word.
pub fn fn_0044a130(e: &mut Engine, this: Ptr, a: u32, b: u32, c: u8) -> u8 {
    let owner = e.mem.u32(this.addr());
    let result = e.vcall(owner, 0x24, &args![a]).u32();
    e.call(LOCK_FREE_MAP_HANDLE, &args![this, result, a, b, c as u32])
        .u32() as u8
}

// Translated from 0044ac40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTaskManager<__int64>::CancelTask` (Xbox PDB): cancels `task`
/// whatever state (+0xC) it is in, then returns.
///
/// First clears the byte at `01202df0` ([`fn_0044ada0`]). Then, repeating:
/// states 0 and 5 move the task to state 6 (`00449190`) and, when that
/// succeeds, call the task's slot `0xC` with `(state, argument)`; state 1
/// does the same after taking a reference and removing the task from the
/// manager (`0044e050`), and releases it afterwards (`0044dd60`); states 3
/// and 4 sleep (`Sleep(0)`) while the task stays in state 3 or 4; state 2
/// goes round again at once (the game's loop there tests for state 3, as
/// state 3's does); any other state returns. A failed move simply reads the
/// state again.
pub fn bs_task_manager_cancel_task(e: &mut Engine, this: Ptr, task: Ptr, argument: u32) {
    fn_0044ada0(e, 0);
    loop {
        let state = e.mem.u32(task.addr() + 0xc);
        match state {
            0 | 5 => {
                if try_set_cancelled(e, task, state) {
                    e.vcall(task.addr(), TASK_CANCEL_SLOT, &args![state, argument]);
                    return;
                }
            }
            1 => {
                if try_set_cancelled(e, task, state) {
                    e.call(TASK_ADD_REFERENCE, &args![task]);
                    e.call(TASK_MANAGER_REMOVE, &args![this, task]);
                    e.vcall(task.addr(), TASK_CANCEL_SLOT, &args![state, argument]);
                    e.call(TASK_RELEASE, &args![task]);
                    return;
                }
            }
            2 | 3 => {
                while e.mem.u32(task.addr() + 0xc) == 3 {
                    e.call(SLEEP, &args![0u32]);
                }
            }
            4 => {
                while e.mem.u32(task.addr() + 0xc) == 4 {
                    e.call(SLEEP, &args![0u32]);
                }
            }
            _ => return,
        }
    }
}

/// `00449190(task, state, 6)`: tries to move the task from `state` to the
/// cancelled state.
fn try_set_cancelled(e: &mut Engine, task: Ptr, state: u32) -> bool {
    e.call(
        TASK_TRY_SET_STATE,
        &args![task, state, TASK_STATE_CANCELLED],
    )
    .bool()
}

// Translated from 0044ada0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte `value` in the global at `01202df0`.
pub fn fn_0044ada0(e: &mut Engine, value: u8) {
    e.set_global(TASK_MANAGER_FLAG, value);
}

// Translated from 0044adb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `00442600` on each element of the pointer array at +0x50 (count at
/// +0x4C; the count is read again before every element).
pub fn fn_0044adb0(e: &mut Engine, this: Ptr) {
    let mut index = 0u32;
    while index < e.mem.u32(this.addr() + 0x4c) {
        let array = e.mem.u32(this.addr() + 0x50);
        let element = e.mem.u32(array + index * 4);
        e.call(ARRAY_ELEMENT_ACTION, &args![element]);
        index += 1;
    }
}

// Translated from 0044adf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStreamSaveBinary<unsigned int>` (Xbox PDB): writes the four-byte value
/// at `value` to the stream's binary output (the dword at stream +0x250,
/// `008d8b00`) as one element (`0044e0f0(output, value, 1)`).
pub fn ni_stream_save_binary_u32(e: &mut Engine, stream: Ptr, value: Ptr) {
    let output = e.call(STREAM_OUTPUT, &args![stream]).u32();
    e.call(WRITE_BINARY, &args![output, value, 1u32]);
}

// Translated from 0044ae10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0 (`0x01`) of the flag byte `flags` is set.
pub fn fn_0044ae10(_e: &mut Engine, flags: u8) -> bool {
    flags & 0x01 != 0
}

// Translated from 0044ae20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`value` non-zero) or clears bit 0 (`0x01`) of the flag byte at
/// `flags`.
pub fn fn_0044ae20(e: &mut Engine, value: u8, flags: Ptr) {
    write_flag_bit(e, value, flags, 0x01);
}

// Translated from 0044ae50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 1 (`0x02`) of the flag byte `flags` is set.
pub fn fn_0044ae50(_e: &mut Engine, flags: u8) -> bool {
    flags & 0x02 != 0
}

// Translated from 0044ae60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`value` non-zero) or clears bit 1 (`0x02`) of the flag byte at
/// `flags`.
pub fn fn_0044ae60(e: &mut Engine, value: u8, flags: Ptr) {
    write_flag_bit(e, value, flags, 0x02);
}

// Translated from 0044ae90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 2 (`0x04`) of the flag byte `flags` is set.
pub fn fn_0044ae90(_e: &mut Engine, flags: u8) -> bool {
    flags & 0x04 != 0
}

// Translated from 0044aea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`value` non-zero) or clears bit 2 (`0x04`) of the flag byte at
/// `flags`.
pub fn fn_0044aea0(e: &mut Engine, value: u8, flags: Ptr) {
    write_flag_bit(e, value, flags, 0x04);
}

// Translated from 0044aed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPointer<Model>::operator=(Model*)` (Xbox PDB name
/// `NiPointer<Model>::operator_`): when `model` differs from the pointer
/// stored at `this`, releases the old model (`0040c130`), stores the new one
/// and adds a reference to it (`0040f6e0`); returns `this`.
pub fn ni_pointer_model_assign(e: &mut Engine, this: Ptr, model: Ptr<Model>) -> Ptr {
    if e.mem.u32(this.addr()) != model.addr() {
        let old = e.mem.u32(this.addr());
        if old != 0 {
            e.call(MODEL_RELEASE, &args![old]);
        }
        e.mem.set_u32(this.addr(), model.addr());
        let new = e.mem.u32(this.addr());
        if new != 0 {
            e.call(MODEL_ADD_REFERENCE, &args![new]);
        }
    }
    this
}

// Translated from 0044af20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 3 (`0x08`) of the flag byte `flags` is set.
pub fn fn_0044af20(_e: &mut Engine, flags: u8) -> bool {
    flags & 0x08 != 0
}

// Translated from 0044af30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`value` non-zero) or clears bit 3 (`0x08`) of the flag byte at
/// `flags`.
pub fn fn_0044af30(e: &mut Engine, value: u8, flags: Ptr) {
    write_flag_bit(e, value, flags, 0x08);
}

// Translated from 0044af60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 4 (`0x10`) of the flag byte `flags` is set.
pub fn fn_0044af60(_e: &mut Engine, flags: u8) -> bool {
    flags & 0x10 != 0
}

// Translated from 0044af70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`value` non-zero) or clears bit 4 (`0x10`) of the flag byte at
/// `flags`.
pub fn fn_0044af70(e: &mut Engine, value: u8, flags: Ptr) {
    write_flag_bit(e, value, flags, 0x10);
}

// Translated from 0044afa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 5 (`0x20`) of the flag byte `flags` is set.
pub fn fn_0044afa0(_e: &mut Engine, flags: u8) -> bool {
    flags & 0x20 != 0
}

// Translated from 0044afb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`value` non-zero) or clears bit 5 (`0x20`) of the flag byte at
/// `flags`.
pub fn fn_0044afb0(e: &mut Engine, value: u8, flags: Ptr) {
    write_flag_bit(e, value, flags, 0x20);
}

// Translated from 0044afe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Smart-pointer constructor for an object counted at +0xC (`QueuedKF`'s
/// constructor uses it): stores `object` at `this` and, when it is not null,
/// adds a reference ([`fn_0044b010`]); returns `this`.
pub fn fn_0044afe0(e: &mut Engine, this: Ptr, object: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), object.addr());
    if e.mem.u32(this.addr()) != 0 {
        let held = Ptr::new(e.mem.u32(this.addr()));
        fn_0044b010(e, held);
    }
    this
}

// Translated from 0044b010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a reference: interlocked increment of the counter at +0xC.
pub fn fn_0044b010(e: &mut Engine, this: Ptr) {
    e.call(INTERLOCKED_INCREMENT, &args![this.byte_add(0xc)]);
}

// Translated from 0044b030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Smart-pointer destructor for the object counted at +0xC: when the
/// pointer at `this` is not null, drops a reference ([`fn_0044b050`]). The
/// object is not deleted here.
pub fn fn_0044b030(e: &mut Engine, this: Ptr) {
    let held = e.mem.u32(this.addr());
    if held != 0 {
        fn_0044b050(e, Ptr::new(held));
    }
}

// Translated from 0044b050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Drops a reference: interlocked decrement of the counter at +0xC.
pub fn fn_0044b050(e: &mut Engine, this: Ptr) {
    e.call(INTERLOCKED_DECREMENT, &args![this.byte_add(0xc)]);
}

// Translated from 0044b070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Smart-pointer assignment for the object counted at +0xC: when `object`
/// differs from the pointer stored at `this`, drops a reference of the old
/// one, stores the new one and adds a reference to it; returns `this`.
pub fn fn_0044b070(e: &mut Engine, this: Ptr, object: Ptr) -> Ptr {
    if e.mem.u32(this.addr()) != object.addr() {
        let old = e.mem.u32(this.addr());
        if old != 0 {
            fn_0044b050(e, Ptr::new(old));
        }
        e.mem.set_u32(this.addr(), object.addr());
        let new = e.mem.u32(this.addr());
        if new != 0 {
            fn_0044b010(e, Ptr::new(new));
        }
    }
    this
}

// Translated from 0044b0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Smart-pointer constructor for a `LoadedFile`: stores `file` at `this`
/// and, when it is not null, adds a reference ([`fn_0044b0f0`]); returns
/// `this`.
pub fn fn_0044b0c0(e: &mut Engine, this: Ptr, file: Ptr<LoadedFile>) -> Ptr {
    e.mem.set_u32(this.addr(), file.addr());
    if e.mem.u32(this.addr()) != 0 {
        let held = Ptr::new(e.mem.u32(this.addr()));
        fn_0044b0f0(e, held);
    }
    this
}

// Translated from 0044b0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a reference to a `LoadedFile`: waits until the critical section
/// `011c3b6c` can be entered ([`fn_0044b130`]), increments the counter at
/// +0 with an interlocked operation and leaves the section.
pub fn fn_0044b0f0(e: &mut Engine, this: Ptr) {
    while !fn_0044b130(e, Ptr::new(LOADED_FILE_LOCK)) {}
    e.call(INTERLOCKED_INCREMENT, &args![this]);
    e.call(
        LEAVE_CRITICAL_SECTION,
        &args![Ptr::<()>::new(LOADED_FILE_LOCK)],
    );
}

// Translated from 0044b130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TryEnterCriticalSection(section)` through the import (slot `00fdf1b0`),
/// as a bool.
pub fn fn_0044b130(e: &mut Engine, section: Ptr) -> bool {
    e.call(TRY_ENTER_CRITICAL_SECTION, &args![section]).i32() != 0
}

// Translated from 0044b160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Smart-pointer destructor for a `LoadedFile`: when the pointer at `this`
/// is not null, releases a reference ([`fn_0044b180`]).
pub fn fn_0044b160(e: &mut Engine, this: Ptr) {
    let held = e.mem.u32(this.addr());
    if held != 0 {
        fn_0044b180(e, Ptr::new(held));
    }
}

// Translated from 0044b180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases a reference to a `LoadedFile`: waits for the critical section
/// `011c3b6c`, decrements the counter at +0 with an interlocked operation,
/// leaves the section, and when the counter reads 0 deletes the file with
/// its scalar deleting destructor ([`fn_0044b1f0`], flag 1).
pub fn fn_0044b180(e: &mut Engine, this: Ptr) {
    while !fn_0044b130(e, Ptr::new(LOADED_FILE_LOCK)) {}
    e.call(INTERLOCKED_DECREMENT, &args![this]);
    e.call(
        LEAVE_CRITICAL_SECTION,
        &args![Ptr::<()>::new(LOADED_FILE_LOCK)],
    );
    if e.mem.u32(this.addr()) == 0 {
        fn_0044b1f0(e, this, 1);
    }
}

// Translated from 0044b1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadedFile` scalar deleting destructor: `LoadedFile::~LoadedFile`
/// (`0043bb80`), then, when bit 0 of `flags` is set, frees the block
/// (`00aa1460`, size `0x10`); returns `this`.
pub fn fn_0044b1f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(LOADED_FILE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_FREE, &args![this, LOADED_FILE_SIZE]);
    }
    this
}

// Translated from 0044b220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Smart-pointer assignment for a `LoadedFile`: when `file` differs from the
/// pointer stored at `this`, releases the old one ([`fn_0044b180`]), stores
/// the new one and adds a reference to it ([`fn_0044b0f0`]); returns `this`.
pub fn fn_0044b220(e: &mut Engine, this: Ptr, file: Ptr<LoadedFile>) -> Ptr {
    if e.mem.u32(this.addr()) != file.addr() {
        let old = e.mem.u32(this.addr());
        if old != 0 {
            fn_0044b180(e, Ptr::new(old));
        }
        e.mem.set_u32(this.addr(), file.addr());
        let new = e.mem.u32(this.addr());
        if new != 0 {
            fn_0044b0f0(e, Ptr::new(new));
        }
    }
    this
}

// Translated from 0044b270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Smart-pointer constructor for an object counted at +0x20 (the face-gen
/// file queueing functions use it): stores `object` at `this` and, when it
/// is not null, adds a reference ([`fn_0044b2a0`]); returns `this`.
pub fn fn_0044b270(e: &mut Engine, this: Ptr, object: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), object.addr());
    if e.mem.u32(this.addr()) != 0 {
        let held = Ptr::new(e.mem.u32(this.addr()));
        fn_0044b2a0(e, held);
    }
    this
}

// Translated from 0044b2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a reference: interlocked increment of the counter at +0x20.
pub fn fn_0044b2a0(e: &mut Engine, this: Ptr) {
    e.call(INTERLOCKED_INCREMENT, &args![this.byte_add(0x20)]);
}

// Translated from 0044b2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Smart-pointer destructor for the object counted at +0x20: when the
/// pointer at `this` is not null, drops a reference ([`fn_0044b2e0`]).
pub fn fn_0044b2c0(e: &mut Engine, this: Ptr) {
    let held = e.mem.u32(this.addr());
    if held != 0 {
        fn_0044b2e0(e, Ptr::new(held));
    }
}

// ---------------------------------------------------------------------------
// The loader's lock-free queues, the settings collection and the counted
// helpers that follow (`0044b2e0` up to `00ae78e0`).

/// `MemoryManager` allocation, `__cdecl(size) -> block` (`00401000`).
const MEMORY_ALLOCATE: u32 = 0x0040_1000;
/// Constructs `count` elements of `element_size` bytes in a block,
/// `__stdcall(block, element_size, count, constructor)` (`00401050`).
const VECTOR_CONSTRUCT: u32 = 0x0040_1050;
/// The element constructor the vector construction of `0044bbe0` runs on
/// every four-byte slot (`0044dee0`).
const QUEUE_SLOT_CONSTRUCT: u32 = 0x0044_dee0;
/// The memory-context scope guard: `__thiscall(guard, context, 1, source
/// file, line)` and its destructor `__thiscall(guard)`.
const SCOPE_GUARD_ENTER: u32 = 0x0040_4eb0;
const SCOPE_GUARD_LEAVE: u32 = 0x0040_4ee0;
/// The memory context `ThreadSafeStructures.inl` enters, and its source
/// file name (`d:\_fallout3\platforms\common\code\bsmain\ThreadSafeStructures.inl`).
const QUEUE_MEMORY_CONTEXT: u32 = 6;
const THREAD_SAFE_STRUCTURES_SOURCE: u32 = 0x0101_73f8;
/// The memory context `Setting.h` enters, and its source file name
/// (`d:\_fallout3\platforms\common\code\bsmain\Setting.h`).
const SETTING_MEMORY_CONTEXT: u32 = 9;
const SETTING_SOURCE: u32 = 0x0101_608c;
/// Source lines the constructors and members pass to the scope guard.
const QUEUE_CONSTRUCTOR_LINE: u32 = 0xfd;
const SLOT_QUEUE_CONSTRUCTOR_LINE: u32 = 0x390;
const QUEUE_ENQUEUE_LINE: u32 = 0x76;
const SETTING_ADD_LINE: u32 = 0x2fd;
const SETTING_GET_VIEWER_STRINGS_LINE: u32 = 0x326;

/// The constructor of the member at +0x20 of the queues and at +0x10c of
/// the settings class, `__thiscall(member)` (`0096a2d0`).
const QUEUE_MEMBER_CONSTRUCT: u32 = 0x0096_a2d0;
/// Constructs a queue's first node in a block of 8 bytes,
/// `__thiscall(block) -> block` (`0044dfc0`).
const QUEUE_NODE_CONSTRUCT: u32 = 0x0044_dfc0;
/// Constructs the 0x10-byte object a queue keeps at +0x14,
/// `__thiscall(block, size) -> block` (`006c73b0`).
const QUEUE_COUNTER_CONSTRUCT: u32 = 0x006c_73b0;
/// The virtual table `0044d7f0` and `0044b4b0` store on the queues' base
/// part.
const INTERFACED_CLASS_VTABLE: u32 = 0x0101_7458;
/// The virtual table of the queue `0044b350` constructs and `0044d770`
/// destroys.
const QUEUE_VTABLE_A: u32 = 0x0101_7440;
/// The virtual table of the queue `0044b500` constructs and `0044d840`
/// destroys.
const QUEUE_VTABLE_B: u32 = 0x0101_7464;
/// The virtual tables of the three slot queues (`0044bbe0`, `0044be80`,
/// `0044c760`).
const SLOT_QUEUE_VTABLE_BBE0: u32 = 0x0101_751c;
const SLOT_QUEUE_VTABLE_BE80: u32 = 0x0101_756c;
const SLOT_QUEUE_VTABLE_C760: u32 = 0x0101_765c;
/// The virtual tables of the five small 0x10-byte objects `0044cb70` to
/// `0044cc90` construct.
const SMALL_OBJECT_VTABLE_CB70: u32 = 0x0101_7294;
const SMALL_OBJECT_VTABLE_CBB0: u32 = 0x0101_72ac;
const SMALL_OBJECT_VTABLE_CC10: u32 = 0x0101_72b8;
const SMALL_OBJECT_VTABLE_CC50: u32 = 0x0101_72c4;
const SMALL_OBJECT_VTABLE_CC90: u32 = 0x0101_72d0;
/// Tears down a queue's contents, `__thiscall(queue, 1)` (`0044e170`).
const QUEUE_CLEAR: u32 = 0x0044_e170;

/// Compare-and-exchange on a queue link, `__cdecl(target, new value,
/// expected value) -> byte` (`004491c0`, true when the exchange happened).
const COMPARE_EXCHANGE: u32 = 0x0044_91c0;
/// `NiPointer` copy assignment (the argument is the address of the source
/// pointer slot), `__thiscall(slot, &source)` (`0092c820`).
const POINTER_COPY: u32 = 0x0092_c820;
/// Returns a dequeued node to the queue, `__thiscall(interface, node)`
/// (`0044e4f0`).
const RECYCLE_NODE: u32 = 0x0044_e4f0;

/// `__thiscall(interface, a, b) -> byte` (`0044e820`): the step `0044d3f0`
/// repeats while it succeeds.
const EXCHANGE_STEP: u32 = 0x0044_e820;
/// Tagged-pointer constructor, `__thiscall(out, tag, pointer)`
/// (`006ecba0`), and its reader `__thiscall(tagged) -> value` (`00559450`).
const TAGGED_CONSTRUCT: u32 = 0x006e_cba0;
const TAGGED_VALUE: u32 = 0x0055_9450;
/// The pointer part (bit 0 cleared) of the tagged word at `this`,
/// `__thiscall(this) -> pointer` (`0044dec0`), and its twin `0044df00`.
const TAGGED_POINTER: u32 = 0x0044_dec0;
const TAGGED_POINTER_TWIN: u32 = 0x0044_df00;
/// `__thiscall(interface, pointer)` (`00666510`), run when the final
/// exchange of `0044d3f0` succeeds.
const EXCHANGE_COMMIT: u32 = 0x0066_6510;
/// `__thiscall(interface)` (`00529000`), run when `0044d3f0` is done.
const EXCHANGE_FINISH: u32 = 0x0052_9000;

/// The settings list's helpers: `__thiscall(list) -> bool` (`008256d0`),
/// the node's item slot `__thiscall(node) -> slot` (`006815c0`), the next
/// node `__thiscall(node) -> node` (`00726070`), the first node of the
/// collection `__thiscall(collection) -> node` (`0044faa0`), and the next
/// node of the collection `__thiscall(collection, node) -> node`
/// (`0044fac0`).
const SETTING_LIST_IS_EMPTY: u32 = 0x0082_56d0;
const LIST_NODE_ITEM: u32 = 0x0068_15c0;
const LIST_NODE_NEXT: u32 = 0x0072_6070;
const COLLECTION_FIRST: u32 = 0x0044_faa0;
const COLLECTION_NEXT: u32 = 0x0044_fac0;
/// `Setting` key getter `__thiscall(setting) -> key` (`0044ddc0`), the
/// string comparison `__cdecl(a, b) -> int` (`00404dc0`), the warning
/// `__cdecl(format, ...)` (`00c335a0`, `SettingWarning` in the Xbox PDB), and
/// the duplicate-key message (`01017780`).
const SETTING_KEY: u32 = 0x0044_ddc0;
const STRING_COMPARE: u32 = 0x0040_4dc0;
const SETTING_WARNING: u32 = 0x00c3_35a0;
const DUPLICATE_KEY_MESSAGE: u32 = 0x0101_7780;
/// Appends / removes a pointer to a setting in the list,
/// `__thiscall(list, &setting)` (`005ae3d0`, `00905330`).
const SETTING_LIST_ADD: u32 = 0x005a_e3d0;
const SETTING_LIST_REMOVE: u32 = 0x0090_5330;
/// `Setting::GetViewerStrings` (Xbox PDB), `__thiscall(setting, argument)`
/// (`00c33280`).
const SETTING_GET_VIEWER_STRINGS: u32 = 0x00c3_3280;

/// The settings object's singleton (`011f96a0`) and its size.
const SETTINGS_INSTANCE: u32 = 0x011f_96a0;
const SETTINGS_INSTANCE_SIZE: u32 = 0x114;
/// Base constructor / destructor pieces of the settings classes:
/// `0044f700` calls `00404d00`; `0044f4f0` calls `00404950`; `0044f650`
/// calls `00470470`, `0046ffb0` and `00404c80`; `0044f620` calls `00877a10`.
const SETTINGS_BASE_CONSTRUCT: u32 = 0x0040_4d00;
const INI_COLLECTION_BASE_DESTRUCT: u32 = 0x0040_4950;
const SETTINGS_LIST_DESTRUCT_FIRST: u32 = 0x0047_0470;
const SETTINGS_LIST_DESTRUCT_SECOND: u32 = 0x0046_ffb0;
const SETTINGS_BASE_DESTRUCT: u32 = 0x0040_4c80;
const SETTINGS_OBJECT_DESTRUCT: u32 = 0x0087_7a10;
/// The virtual tables of the settings classes (`0044f4f0`, `0044f600`,
/// `0044f700` and `0044f650`).
const INI_COLLECTION_VTABLE: u32 = 0x0101_7720;
const SETTINGS_DERIVED_VTABLE: u32 = 0x0101_772c;
const SETTINGS_VTABLE: u32 = 0x0101_7758;
/// Offset of the list a settings collection keeps its settings in.
const SETTINGS_LIST_OFFSET: u32 = 0x10c;

/// Imported `ReleaseSemaphore` and `InterlockedIncrement` (import slots).
const RELEASE_SEMAPHORE: u32 = 0x00fd_f1b4;
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;

/// Runs `body` between the memory-context scope guard's constructor and
/// destructor (the guard is a four-byte local).
fn in_memory_context<R>(
    e: &mut Engine,
    context: u32,
    source: u32,
    line: u32,
    body: impl FnOnce(&mut Engine) -> R,
) -> R {
    e.with_stack(4, |e, guard| {
        e.call(
            SCOPE_GUARD_ENTER,
            &args![guard, context, 1u32, source, line],
        );
        let result = body(e);
        e.call(SCOPE_GUARD_LEAVE, &args![guard]);
        result
    })
}

/// The byte size of an array of `count` elements of `element_size` bytes as
/// the compiler's checked `new[]` computes it: all ones when the product
/// does not fit 32 bits.
fn array_byte_size(count: u32, element_size: u32) -> u32 {
    u32::try_from(u64::from(count) * u64::from(element_size)).unwrap_or(u32::MAX)
}

/// The 32-bit word at `address + offset`.
fn word(e: &Engine, address: u32, offset: u32) -> u32 {
    e.mem.u32(address.wrapping_add(offset))
}

/// Allocates a `size`-byte block and, when that worked, runs `construct`
/// on it; the constructed object, or 0.
fn allocate_and_construct(
    e: &mut Engine,
    size: u32,
    construct: impl FnOnce(&mut Engine, u32) -> u32,
) -> u32 {
    let block = e.call(MEMORY_ALLOCATE, &args![size]).u32();
    if block == 0 {
        0
    } else {
        construct(e, block)
    }
}

// Translated from 0044b2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Drops a reference: interlocked decrement of the counter at +0x20. The
/// object is not deleted here (the decompiler's name for the call is wrong).
pub fn fn_0044b2e0(e: &mut Engine, this: Ptr) {
    e.call(INTERLOCKED_DECREMENT, &args![this.byte_add(0x20)]);
}

// Translated from 0044b300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Smart-pointer assignment for the object counted at +0x20: when `object`
/// differs from the pointer stored at `this`, drops a reference of the old
/// one, stores the new one and adds a reference to it; returns `this`.
pub fn fn_0044b300(e: &mut Engine, this: Ptr, object: Ptr) -> Ptr {
    if e.mem.u32(this.addr()) != object.addr() {
        let old = e.mem.u32(this.addr());
        if old != 0 {
            fn_0044b2e0(e, Ptr::new(old));
        }
        e.mem.set_u32(this.addr(), object.addr());
        let new = e.mem.u32(this.addr());
        if new != 0 {
            fn_0044b2a0(e, Ptr::new(new));
        }
    }
    this
}

/// The shared body of the queue constructors `0044b350` and `0044b500`.
fn queue_construct(e: &mut Engine, this: Ptr, size: u32, extra: u32, vtable: u32) -> Ptr {
    fn_0044d7f0(e, this);
    e.mem.set_u32(this.addr(), vtable);
    e.call(QUEUE_MEMBER_CONSTRUCT, &args![this.byte_add(0x20)]);
    in_memory_context(
        e,
        QUEUE_MEMORY_CONTEXT,
        THREAD_SAFE_STRUCTURES_SOURCE,
        QUEUE_CONSTRUCTOR_LINE,
        |e| {
            e.mem.set_u32(this.addr() + 0x18, 0);
            let first_node = allocate_and_construct(e, 8, |e, block| {
                e.call(QUEUE_NODE_CONSTRUCT, &args![block]).u32()
            });
            e.mem.set_u32(this.addr() + 4, first_node);
            let head = e.mem.u32(this.addr() + 4);
            e.mem.set_u32(this.addr() + 8, head);
            e.mem.set_u32(this.addr() + 0xc, extra);
            let slots = e
                .call(
                    MEMORY_ALLOCATE,
                    &args![array_byte_size(size.wrapping_mul(2), 4)],
                )
                .u32();
            e.mem.set_u32(this.addr() + 0x10, slots);
            let counter = allocate_and_construct(e, 0x10, |e, block| {
                e.call(QUEUE_COUNTER_CONSTRUCT, &args![block, size]).u32()
            });
            e.mem.set_u32(this.addr() + 0x14, counter);
        },
    );
    this
}

// Translated from 0044b350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the lock-free queue whose virtual table is `01017440`
/// (the one `0044d770` destroys): the base part, a first node (stored as
/// head and tail at +4 and +8), `extra` at +0xc, a block of `size * 2`
/// words at +0x10 and a counter object for `size` at +0x14. The
/// compiler's exception-unwinding frame is not translated.
pub fn fn_0044b350(e: &mut Engine, this: Ptr, size: u32, extra: u32) -> Ptr {
    queue_construct(e, this, size, extra, QUEUE_VTABLE_A)
}

// Translated from 0044b4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The base destructor of the queue classes: stores the base class's
/// virtual table (`01017458`).
pub fn fn_0044b4b0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), INTERFACED_CLASS_VTABLE);
}

// Translated from 0044b4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfacedClass::scalar deleting destructor` (Xbox PDB): runs the base
/// destructor body and, when bit 0 of `flags` is set, frees the block;
/// returns `this`.
pub fn interfaced_class_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0044b4b0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 0044b500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the lock-free queue whose virtual table is `01017464`
/// (the one `0044d840` destroys); the same as [`fn_0044b350`] with that
/// table.
pub fn fn_0044b500(e: &mut Engine, this: Ptr, size: u32, extra: u32) -> Ptr {
    queue_construct(e, this, size, extra, QUEUE_VTABLE_B)
}

/// The shared body of the slot-queue constructors `0044bbe0`, `0044be80`
/// and `0044c760`.
fn slot_queue_construct(
    e: &mut Engine,
    this: Ptr,
    size: u32,
    count: u32,
    extra: u32,
    vtable: u32,
) -> Ptr {
    fn_0044d7f0(e, this);
    e.mem.set_u32(this.addr(), vtable);
    e.call(QUEUE_MEMBER_CONSTRUCT, &args![this.byte_add(0x20)]);
    in_memory_context(
        e,
        QUEUE_MEMORY_CONTEXT,
        THREAD_SAFE_STRUCTURES_SOURCE,
        SLOT_QUEUE_CONSTRUCTOR_LINE,
        |e| {
            e.mem.set_u32(this.addr() + 0x18, 0);
            e.mem.set_u32(this.addr() + 8, count);
            let slot_count = e.mem.u32(this.addr() + 8);
            // `count` four-byte slots, each constructed in turn.
            let slots = e
                .call(MEMORY_ALLOCATE, &args![array_byte_size(slot_count, 4)])
                .u32();
            if slots != 0 {
                e.call(
                    VECTOR_CONSTRUCT,
                    &args![slots, 4u32, slot_count, QUEUE_SLOT_CONSTRUCT],
                );
            }
            e.mem.set_u32(this.addr() + 0xc, slots);
            let entries = e
                .call(
                    MEMORY_ALLOCATE,
                    &args![array_byte_size(size.wrapping_mul(3), 4)],
                )
                .u32();
            e.mem.set_u32(this.addr() + 4, entries);
            e.mem.set_u32(this.addr() + 0x10, extra);
            let counter = allocate_and_construct(e, 0x10, |e, block| {
                e.call(QUEUE_COUNTER_CONSTRUCT, &args![block, size]).u32()
            });
            e.mem.set_u32(this.addr() + 0x14, counter);
        },
    );
    this
}

// Translated from 0044bbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a slot queue (virtual table `0101751c`): the base part,
/// `count` constructed four-byte slots at +0xc, `size * 3` words at +4,
/// `extra` at +0x10 and a counter object for `size` at +0x14.
pub fn fn_0044bbe0(e: &mut Engine, this: Ptr, size: u32, count: u32, extra: u32) -> Ptr {
    slot_queue_construct(e, this, size, count, extra, SLOT_QUEUE_VTABLE_BBE0)
}

// Translated from 0044be80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a slot queue (virtual table `0101756c`); the same as
/// [`fn_0044bbe0`] with that table.
pub fn fn_0044be80(e: &mut Engine, this: Ptr, size: u32, count: u32, extra: u32) -> Ptr {
    slot_queue_construct(e, this, size, count, extra, SLOT_QUEUE_VTABLE_BE80)
}

// Translated from 0044c760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a slot queue (virtual table `0101765c`); the same as
/// [`fn_0044bbe0`] with that table.
pub fn fn_0044c760(e: &mut Engine, this: Ptr, size: u32, count: u32, extra: u32) -> Ptr {
    slot_queue_construct(e, this, size, count, extra, SLOT_QUEUE_VTABLE_C760)
}

/// Constructor of a 0x10-byte object: the virtual table, two zeroed words
/// and a zeroed byte (the decompiler labels these `TaskStack` constructors,
/// a library label that does not fit).
fn small_object_construct(e: &mut Engine, this: Ptr, vtable: u32) -> Ptr {
    e.mem.set_u32(this.addr(), vtable);
    e.mem.set_u32(this.addr() + 4, 0);
    e.mem.set_u32(this.addr() + 8, 0);
    e.mem.set_u8(this.addr() + 0xc, 0);
    this
}

// Translated from 0044cb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a 0x10-byte object with virtual table `01017294`.
pub fn fn_0044cb70(e: &mut Engine, this: Ptr) -> Ptr {
    small_object_construct(e, this, SMALL_OBJECT_VTABLE_CB70)
}

// Translated from 0044cbb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a 0x10-byte object with virtual table `010172ac`.
pub fn fn_0044cbb0(e: &mut Engine, this: Ptr) -> Ptr {
    small_object_construct(e, this, SMALL_OBJECT_VTABLE_CBB0)
}

// Translated from 0044cc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a 0x10-byte object with virtual table `010172b8`.
pub fn fn_0044cc10(e: &mut Engine, this: Ptr) -> Ptr {
    small_object_construct(e, this, SMALL_OBJECT_VTABLE_CC10)
}

// Translated from 0044cc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a 0x10-byte object with virtual table `010172c4`.
pub fn fn_0044cc50(e: &mut Engine, this: Ptr) -> Ptr {
    small_object_construct(e, this, SMALL_OBJECT_VTABLE_CC50)
}

// Translated from 0044cc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a 0x10-byte object with virtual table `010172d0`.
pub fn fn_0044cc90(e: &mut Engine, this: Ptr) -> Ptr {
    small_object_construct(e, this, SMALL_OBJECT_VTABLE_CC90)
}

// Translated from 0044cd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends the pointer held in the slot at `item_slot` to a lock-free queue through its per-thread interface
/// `this` (`this[0]` the queue, `this[1]` and `this[2]` the interface's
/// published slots): builds a node holding a copy of it, then retries the usual
/// compare-and-exchange protocol on the tail until the node is linked, calls
/// the queue's virtual function at +8 once it is, and swings the tail to
/// the new node. The compiler's exception-unwinding frame is not
/// translated.
pub fn fn_0044cd40(e: &mut Engine, this: Ptr, item_slot: u32) {
    in_memory_context(
        e,
        QUEUE_MEMORY_CONTEXT,
        THREAD_SAFE_STRUCTURES_SOURCE,
        QUEUE_ENQUEUE_LINE,
        |e| {
            let node = allocate_and_construct(e, 8, |e, block| {
                e.call(QUEUE_NODE_CONSTRUCT, &args![block]).u32()
            });
            e.call(POINTER_COPY, &args![node + 4, item_slot]);
            let mut tail;
            loop {
                let queue = e.mem.u32(this.addr());
                tail = word(e, queue, 8);
                let published = e.mem.u32(this.addr() + 4);
                e.mem.set_u32(published, tail);
                if tail != word(e, e.mem.u32(this.addr()), 8) {
                    continue;
                }
                let next = e.mem.u32(tail);
                if tail != word(e, e.mem.u32(this.addr()), 8) {
                    continue;
                }
                let queue = e.mem.u32(this.addr());
                if next != 0 {
                    // The tail is behind: help it along.
                    e.call(COMPARE_EXCHANGE, &args![queue + 8, next, tail]);
                    continue;
                }
                if e.call(COMPARE_EXCHANGE, &args![tail, node, 0u32]).bool() {
                    e.vcall(queue, 8, &args![]);
                    break;
                }
            }
            let queue = e.mem.u32(this.addr());
            e.call(COMPARE_EXCHANGE, &args![queue + 8, node, tail]);
            let published = e.mem.u32(this.addr() + 4);
            e.mem.set_u32(published, 0);
        },
    );
}

// Translated from 0044cec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes the first item off a lock-free queue through its per-thread
/// interface `this` (see [`fn_0044cd40`]) into the pointer slot `out`.
/// Returns false, leaving `out` null, when the queue is empty. Otherwise it
/// copies the item to `out`, moves the head past the node with a
/// compare-and-exchange, calls the queue's virtual function at +0xc, clears
/// the node's item, clears the published slots and gives the node back
/// (`0044e4f0`).
pub fn fn_0044cec0(e: &mut Engine, this: Ptr, out: Ptr) -> bool {
    let mut head;
    loop {
        let queue = e.mem.u32(this.addr());
        head = word(e, queue, 4);
        let published_head = e.mem.u32(this.addr() + 4);
        e.mem.set_u32(published_head, head);
        if head != word(e, e.mem.u32(this.addr()), 4) {
            continue;
        }
        let tail = word(e, e.mem.u32(this.addr()), 8);
        let next = e.mem.u32(head);
        let published_next = e.mem.u32(this.addr() + 8);
        e.mem.set_u32(published_next, next);
        if head != word(e, e.mem.u32(this.addr()), 4) {
            continue;
        }
        if next == 0 {
            let published_head = e.mem.u32(this.addr() + 4);
            e.mem.set_u32(published_head, 0);
            ni_pointer_queued_file_assign(e, out, Ptr::NULL);
            return false;
        }
        let queue = e.mem.u32(this.addr());
        if head == tail {
            e.call(COMPARE_EXCHANGE, &args![queue + 8, next, tail]);
            continue;
        }
        e.call(POINTER_COPY, &args![out, next + 4]);
        if e.call(COMPARE_EXCHANGE, &args![queue + 4, next, head])
            .bool()
        {
            e.vcall(queue, 0xc, &args![]);
            ni_pointer_queued_file_assign(e, Ptr::new(next + 4), Ptr::NULL);
            break;
        }
    }
    let published_head = e.mem.u32(this.addr() + 4);
    e.mem.set_u32(published_head, 0);
    let published_next = e.mem.u32(this.addr() + 8);
    e.mem.set_u32(published_next, 0);
    e.call(RECYCLE_NODE, &args![this, head]);
    true
}

// Translated from 0044d3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Repeats the exchange step `0044e820(a, b)` for as long as it succeeds
/// and the compare-and-exchange of the tagged pointers it prepares fails;
/// returns false as soon as the step itself fails. When the retry ends in a
/// successful exchange on the tagged word at +0x10 it runs `00666510`,
/// otherwise it runs the step once more; `00529000` always runs last.
pub fn fn_0044d3f0(e: &mut Engine, this: Ptr, a: u32, b: u32) -> bool {
    let result = e.with_stack(12, |e, tagged| {
        let first = tagged;
        let second = tagged.byte_add(4);
        let third = tagged.byte_add(8);
        loop {
            if !e.call(EXCHANGE_STEP, &args![this, a, b]).bool() {
                return false;
            }
            let pointer = e.call(TAGGED_POINTER, &args![this.byte_add(0x18)]).u32();
            e.call(TAGGED_CONSTRUCT, &args![first, 0u32, pointer]);
            let pointer = e.call(TAGGED_POINTER, &args![this.byte_add(0x18)]).u32();
            e.call(TAGGED_CONSTRUCT, &args![second, 1u32, pointer]);
            let old = e.call(TAGGED_VALUE, &args![first]).u32();
            let new = e.call(TAGGED_VALUE, &args![second]).u32();
            let target = e
                .call(TAGGED_POINTER_TWIN, &args![this.byte_add(0x14)])
                .u32();
            if e.call(COMPARE_EXCHANGE, &args![target + 8, new, old])
                .bool()
            {
                break;
            }
        }
        let pointer = e.call(TAGGED_POINTER, &args![this.byte_add(0x14)]).u32();
        e.call(TAGGED_CONSTRUCT, &args![third, 0u32, pointer]);
        let replacement = e.call(TAGGED_VALUE, &args![third]).u32();
        let old = e.call(TAGGED_VALUE, &args![first]).u32();
        let word_at_0x10 = e.mem.u32(this.addr() + 0x10);
        if e.call(COMPARE_EXCHANGE, &args![word_at_0x10, old, replacement])
            .bool()
        {
            let pointer = e.call(TAGGED_POINTER, &args![this.byte_add(0x14)]).u32();
            e.call(EXCHANGE_COMMIT, &args![this, pointer]);
        } else {
            e.call(EXCHANGE_STEP, &args![this, a, b]);
        }
        true
    });
    e.call(EXCHANGE_FINISH, &args![this]);
    result
}

// Translated from 0044d740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreeQueue<NiPointer<QueuedReference>>::scalar deleting destructor`
/// (Xbox PDB): runs the destructor body [`fn_0044d770`] and, when bit 0 of
/// `flags` is set, frees the block; returns `this`.
pub fn lock_free_queue_queued_reference_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_0044d770(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

/// The shared body of the queue destructors `0044d770` and `0044d840`.
fn queue_destruct(e: &mut Engine, this: Ptr, vtable: u32) {
    e.mem.set_u32(this.addr(), vtable);
    e.call(QUEUE_CLEAR, &args![this, 1u32]);
    let slots = e.mem.u32(this.addr() + 0x10);
    e.call(MEMORY_FREE, &args![slots]);
    fn_0044b4b0(e, this);
}

// Translated from 0044d770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the queue with virtual table `01017440`: clears the
/// queue (`0044e170`), frees the block at +0x10 and runs the base
/// destructor. The compiler's exception-unwinding frame is not translated.
pub fn fn_0044d770(e: &mut Engine, this: Ptr) {
    queue_destruct(e, this, QUEUE_VTABLE_A);
}

// Translated from 0044d7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The base constructor of the queue classes: stores the base class's
/// virtual table (`01017458`) and returns `this`.
pub fn fn_0044d7f0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), INTERFACED_CLASS_VTABLE);
    this
}

// Translated from 0044d810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreeQueue<NiPointer<AttachDistant3DTask>>::scalar deleting
/// destructor` (Xbox PDB): runs the destructor body [`fn_0044d840`] and,
/// when bit 0 of `flags` is set, frees the block; returns `this`.
pub fn lock_free_queue_attach_distant_3d_task_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_0044d840(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 0044d840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the queue with virtual table `01017464`; the same as
/// [`fn_0044d770`] with that table.
pub fn fn_0044d840(e: &mut Engine, this: Ptr) {
    queue_destruct(e, this, QUEUE_VTABLE_B);
}

// Translated from 0044f4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SettingT<INISettingCollection>::scalar deleting destructor` (Xbox PDB):
/// runs the destructor body [`fn_0044f4f0`] and, when bit 0 of `flags` is
/// set, frees the block; returns `this`.
pub fn setting_t_ini_setting_collection_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_0044f4f0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 0044f4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the object with virtual table `01017720`: asks the
/// settings singleton (made on first use, [`fn_0044f560`]) to run its
/// virtual function at +8 on `this`, then runs the base destructor
/// `00404950`. The compiler's exception-unwinding frame is not translated.
pub fn fn_0044f4f0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), INI_COLLECTION_VTABLE);
    let instance = fn_0044f560(e);
    e.vcall(instance.addr(), 8, &args![this]);
    e.call(INI_COLLECTION_BASE_DESTRUCT, &args![this]);
}

// Translated from 0044f560 (decompiled, FalloutNV.exe 1.4.0.525)
/// The settings singleton (`011f96a0`), created by [`fn_0044f570`] on first
/// use.
pub fn fn_0044f560(e: &mut Engine) -> Ptr {
    fn_0044f570(e);
    Ptr::new(e.global::<u32>(SETTINGS_INSTANCE))
}

// Translated from 0044f570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Creates the settings singleton when it does not exist yet: a 0x114-byte
/// object built by [`fn_0044f600`], stored at `011f96a0`. The compiler's
/// exception-unwinding frame is not translated.
pub fn fn_0044f570(e: &mut Engine) {
    if e.global::<u32>(SETTINGS_INSTANCE) == 0 {
        let instance = allocate_and_construct(e, SETTINGS_INSTANCE_SIZE, |e, block| {
            fn_0044f600(e, Ptr::new(block)).addr()
        });
        e.set_global(SETTINGS_INSTANCE, instance);
    }
}

// Translated from 0044f600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the settings class with virtual table `0101772c`: the
/// base constructor [`fn_0044f700`], then its own table; returns `this`.
pub fn fn_0044f600(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0044f700(e, this);
    e.mem.set_u32(this.addr(), SETTINGS_DERIVED_VTABLE);
    this
}

// Translated from 0044f620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of a settings object: runs `00877a10` and,
/// when bit 0 of `flags` is set, frees the block; returns `this`.
pub fn fn_0044f620(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(SETTINGS_OBJECT_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 0044f650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the settings class with virtual table `01017758`:
/// destroys the list at +0x10c (`00470470`, `0046ffb0`) and runs the base
/// destructor `00404c80`. The compiler's exception-unwinding frame is not
/// translated.
pub fn fn_0044f650(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SETTINGS_VTABLE);
    let list = this.byte_add(SETTINGS_LIST_OFFSET);
    e.call(SETTINGS_LIST_DESTRUCT_FIRST, &args![list]);
    e.call(SETTINGS_LIST_DESTRUCT_SECOND, &args![list]);
    e.call(SETTINGS_BASE_DESTRUCT, &args![this]);
}

// Translated from 0044f6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the settings class: runs [`fn_0044f650`]
/// and, when bit 0 of `flags` is set, frees the block; returns `this`.
pub fn fn_0044f6d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0044f650(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 0044f700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Base constructor of the settings class: `00404d00`, the class's virtual
/// table (`01017758`) and the constructor `0096a2d0` of the member at
/// +0x10c; returns `this`. The compiler's exception-unwinding frame is not
/// translated.
pub fn fn_0044f700(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(SETTINGS_BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), SETTINGS_VTABLE);
    e.call(
        QUEUE_MEMBER_CONSTRUCT,
        &args![this.byte_add(SETTINGS_LIST_OFFSET)],
    );
    this
}

// Translated from 0044f770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SettingCollectionList<Setting>::Add` (Xbox PDB): adds `setting` to the
/// list at +0x10c unless a setting with the same key is already in it, in
/// which case it reports `DEFAULT: Setting key '%s' already used in list.`
/// through `SettingWarning` and adds nothing. Runs in the `Setting.h` memory
/// context. The compiler's exception-unwinding frame is not translated.
pub fn setting_collection_list_setting_add(e: &mut Engine, this: Ptr, setting: u32) {
    in_memory_context(
        e,
        SETTING_MEMORY_CONTEXT,
        SETTING_SOURCE,
        SETTING_ADD_LINE,
        |e| {
            let list = this.byte_add(SETTINGS_LIST_OFFSET);
            if !e.call(SETTING_LIST_IS_EMPTY, &args![list]).bool() {
                let mut node = list.addr();
                while node != 0 {
                    let item_slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
                    let existing = e.mem.u32(item_slot);
                    let new_key = e.call(SETTING_KEY, &args![setting]).u32();
                    let existing_key = e.call(SETTING_KEY, &args![existing]).u32();
                    let order = e.call(STRING_COMPARE, &args![existing_key, new_key]).i32();
                    if order == 0 {
                        let key = e.call(SETTING_KEY, &args![setting]).u32();
                        e.call(SETTING_WARNING, &args![DUPLICATE_KEY_MESSAGE, key]);
                        return;
                    }
                    node = e.call(LIST_NODE_NEXT, &args![node]).u32();
                }
            }
            e.with_stack(4, |e, argument| {
                e.mem.set_u32(argument.addr(), setting);
                e.call(SETTING_LIST_ADD, &args![list, argument]);
            });
        },
    );
}

// Translated from 0044f870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SettingCollectionList<Setting>::Remove` (Xbox PDB): removes `setting`
/// from the list at +0x10c (`00905330`, which takes the address of the
/// argument).
pub fn setting_collection_list_setting_remove(e: &mut Engine, this: Ptr, setting: u32) {
    e.with_stack(4, |e, argument| {
        e.mem.set_u32(argument.addr(), setting);
        e.call(
            SETTING_LIST_REMOVE,
            &args![this.byte_add(SETTINGS_LIST_OFFSET), argument],
        );
    });
}

// Translated from 0044f890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SettingCollectionList<Setting>::GetViewerStrings` (Xbox PDB): walks the
/// collection and calls `Setting::GetViewerStrings` (`00c33280`) with
/// `argument` on every setting; returns how many there were. Runs in the
/// `Setting.h` memory context. The compiler's exception-unwinding frame is
/// not translated.
pub fn setting_collection_list_setting_get_viewer_strings(
    e: &mut Engine,
    this: Ptr,
    argument: u32,
) -> i32 {
    in_memory_context(
        e,
        SETTING_MEMORY_CONTEXT,
        SETTING_SOURCE,
        SETTING_GET_VIEWER_STRINGS_LINE,
        |e| {
            let mut node = e.call(COLLECTION_FIRST, &args![this]).u32();
            let mut count = 0;
            while node != 0 {
                // `006815c0` takes no argument and leaves the pushed word
                // for the call that follows it.
                let item_slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
                let setting = e.mem.u32(item_slot);
                e.call(SETTING_GET_VIEWER_STRINGS, &args![setting, argument]);
                node = e.call(COLLECTION_NEXT, &args![this, node]).u32();
                count += 1;
            }
            count
        },
    )
}

// Translated from 006f74f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPointer<QueuedFile>::operator=` (Xbox PDB): when `value` differs from
/// the pointer stored at `this`, releases the old one (`0044dd60`), stores
/// the new one and adds a reference to it (`0092c870`); returns `this`.
pub fn ni_pointer_queued_file_assign(e: &mut Engine, this: Ptr, value: Ptr) -> Ptr {
    if e.mem.u32(this.addr()) != value.addr() {
        let old = e.mem.u32(this.addr());
        if old != 0 {
            e.call(TASK_RELEASE, &args![old]);
        }
        e.mem.set_u32(this.addr(), value.addr());
        let new = e.mem.u32(this.addr());
        if new != 0 {
            e.call(TASK_ADD_REFERENCE, &args![new]);
        }
    }
    this
}

// Translated from 00712380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedFileLoad::GetFileIndex` (Xbox PDB): the byte at +0x34.
pub fn queued_file_load_get_file_index(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x34)
}

// Translated from 00ae78e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSemaphore::Signal` (Xbox PDB): when the byte at +0x48 is set,
/// releases the semaphore handle at +0x34 by one (`ReleaseSemaphore`) and
/// increments the counter at +0x30 (`InterlockedIncrement`).
pub fn bs_semaphore_signal(e: &mut Engine, this: Ptr) {
    if e.mem.u8(this.addr() + 0x48) != 0 {
        let counter = this.byte_add(0x30);
        let handle = e.mem.u32(counter.addr() + 4);
        e.call(RELEASE_SEMAPHORE, &args![handle, 1u32, 0u32]);
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![counter]);
    }
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00449c00,
            lock_free_map_iterator_queued_helmet_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00449c30, fn_00449c30(Ptr)),
        entry!(0x00449d40, fn_00449d40(Ptr)),
        entry!(0x00449ea0, fn_00449ea0(Ptr)),
        entry!(0x00449f50, fn_00449f50(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00449fa0, fn_00449fa0(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x0044a100, fn_0044a100(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x0044a130, fn_0044a130(Ptr, u32, u32, u8) -> u8),
        entry!(0x0044ac40, bs_task_manager_cancel_task(Ptr, Ptr, u32)),
        entry!(0x0044ada0, fn_0044ada0(u8)),
        entry!(0x0044adb0, fn_0044adb0(Ptr)),
        entry!(0x0044adf0, ni_stream_save_binary_u32(Ptr, Ptr)),
        entry!(0x0044ae10, fn_0044ae10(u8) -> bool),
        entry!(0x0044ae20, fn_0044ae20(u8, Ptr)),
        entry!(0x0044ae50, fn_0044ae50(u8) -> bool),
        entry!(0x0044ae60, fn_0044ae60(u8, Ptr)),
        entry!(0x0044ae90, fn_0044ae90(u8) -> bool),
        entry!(0x0044aea0, fn_0044aea0(u8, Ptr)),
        entry!(0x0044aed0, ni_pointer_model_assign(Ptr, Ptr<Model>) -> Ptr),
        entry!(0x0044af20, fn_0044af20(u8) -> bool),
        entry!(0x0044af30, fn_0044af30(u8, Ptr)),
        entry!(0x0044af60, fn_0044af60(u8) -> bool),
        entry!(0x0044af70, fn_0044af70(u8, Ptr)),
        entry!(0x0044afa0, fn_0044afa0(u8) -> bool),
        entry!(0x0044afb0, fn_0044afb0(u8, Ptr)),
        entry!(0x0044afe0, fn_0044afe0(Ptr, Ptr) -> Ptr),
        entry!(0x0044b010, fn_0044b010(Ptr)),
        entry!(0x0044b030, fn_0044b030(Ptr)),
        entry!(0x0044b050, fn_0044b050(Ptr)),
        entry!(0x0044b070, fn_0044b070(Ptr, Ptr) -> Ptr),
        entry!(0x0044b0c0, fn_0044b0c0(Ptr, Ptr<LoadedFile>) -> Ptr),
        entry!(0x0044b0f0, fn_0044b0f0(Ptr)),
        entry!(0x0044b130, fn_0044b130(Ptr) -> bool),
        entry!(0x0044b160, fn_0044b160(Ptr)),
        entry!(0x0044b180, fn_0044b180(Ptr)),
        entry!(0x0044b1f0, fn_0044b1f0(Ptr, u32) -> Ptr),
        entry!(0x0044b220, fn_0044b220(Ptr, Ptr<LoadedFile>) -> Ptr),
        entry!(0x0044b270, fn_0044b270(Ptr, Ptr) -> Ptr),
        entry!(0x0044b2a0, fn_0044b2a0(Ptr)),
        entry!(0x0044b2c0, fn_0044b2c0(Ptr)),
        entry!(0x0044b2e0, fn_0044b2e0(Ptr)),
        entry!(0x0044b300, fn_0044b300(Ptr, Ptr) -> Ptr),
        entry!(0x0044b350, fn_0044b350(Ptr, u32, u32) -> Ptr),
        entry!(0x0044b4b0, fn_0044b4b0(Ptr)),
        entry!(
            0x0044b4d0,
            interfaced_class_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0044b500, fn_0044b500(Ptr, u32, u32) -> Ptr),
        entry!(0x0044bbe0, fn_0044bbe0(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x0044be80, fn_0044be80(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x0044c760, fn_0044c760(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x0044cb70, fn_0044cb70(Ptr) -> Ptr),
        entry!(0x0044cbb0, fn_0044cbb0(Ptr) -> Ptr),
        entry!(0x0044cc10, fn_0044cc10(Ptr) -> Ptr),
        entry!(0x0044cc50, fn_0044cc50(Ptr) -> Ptr),
        entry!(0x0044cc90, fn_0044cc90(Ptr) -> Ptr),
        entry!(0x0044cd40, fn_0044cd40(Ptr, u32)),
        entry!(0x0044cec0, fn_0044cec0(Ptr, Ptr) -> bool),
        entry!(0x0044d3f0, fn_0044d3f0(Ptr, u32, u32) -> bool),
        entry!(
            0x0044d740,
            lock_free_queue_queued_reference_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0044d770, fn_0044d770(Ptr)),
        entry!(0x0044d7f0, fn_0044d7f0(Ptr) -> Ptr),
        entry!(
            0x0044d810,
            lock_free_queue_attach_distant_3d_task_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0044d840, fn_0044d840(Ptr)),
        entry!(
            0x0044f4c0,
            setting_t_ini_setting_collection_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0044f4f0, fn_0044f4f0(Ptr)),
        entry!(0x0044f560, fn_0044f560() -> Ptr),
        entry!(0x0044f570, fn_0044f570()),
        entry!(0x0044f600, fn_0044f600(Ptr) -> Ptr),
        entry!(0x0044f620, fn_0044f620(Ptr, u32) -> Ptr),
        entry!(0x0044f650, fn_0044f650(Ptr)),
        entry!(0x0044f6d0, fn_0044f6d0(Ptr, u32) -> Ptr),
        entry!(0x0044f700, fn_0044f700(Ptr) -> Ptr),
        entry!(0x0044f770, setting_collection_list_setting_add(Ptr, u32)),
        entry!(0x0044f870, setting_collection_list_setting_remove(Ptr, u32)),
        entry!(
            0x0044f890,
            setting_collection_list_setting_get_viewer_strings(Ptr, u32) -> i32
        ),
        entry!(0x006f74f0, ni_pointer_queued_file_assign(Ptr, Ptr) -> Ptr),
        entry!(0x00712380, queued_file_load_get_file_index(Ptr) -> u8),
        entry!(0x00ae78e0, bs_semaphore_signal(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The argument lists of the calls made to `addr` since `call_log` was
    /// switched on.
    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// The addresses called, in order (the top-level call comes first).
    fn call_order(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(addr, _)| *addr)
            .collect()
    }

    /// An engine with the pages of the byte global and of the lock mapped,
    /// and a zeroed block of `size` bytes.
    fn engine_with_block(size: u32) -> (Engine, Ptr) {
        let mut e = Engine::new();
        e.map(0x0120_2000, 0x1000);
        e.map(0x011c_3000, 0x1000);
        let block = Ptr::new(e.mem.alloc(size));
        (e, block)
    }

    /// Registers a double that does nothing at each address.
    fn ignore(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    #[test]
    fn iterator_destructor_frees_only_with_flag_bit_0() {
        let (mut e, this) = engine_with_block(8);
        ignore(&mut e, &[LOCK_FREE_MAP_ITERATOR_BASE_DESTRUCT, MEMORY_FREE]);
        e.call_log = Some(vec![]);
        let back = e.call(0x0044_9c00, &args![this, 0u32]).ptr::<()>();
        assert_eq!(back, this);
        assert_eq!(
            calls_to(&e, LOCK_FREE_MAP_ITERATOR_BASE_DESTRUCT),
            vec![vec![this.addr()]]
        );
        assert!(calls_to(&e, MEMORY_FREE).is_empty());
        e.call(0x0044_9c00, &args![this, 3u32]);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![this.addr()]]);
    }

    #[test]
    fn map_destructor_wrappers_forward_this() {
        let (mut e, this) = engine_with_block(8);
        let bodies = [
            LOCK_FREE_MAP_DESTRUCT_00449C50,
            LOCK_FREE_MAP_DESTRUCT_00449D60,
            LOCK_FREE_MAP_DESTRUCT_00449EC0,
        ];
        ignore(&mut e, &bodies);
        e.call_log = Some(vec![]);
        e.call(0x0044_9c30, &args![this]);
        e.call(0x0044_9d40, &args![this]);
        e.call(0x0044_9ea0, &args![this]);
        for body in bodies {
            assert_eq!(calls_to(&e, body), vec![vec![this.addr()]]);
        }
    }

    #[test]
    fn map_constructors_call_the_base_then_store_their_vtable() {
        let (mut e, this) = engine_with_block(0x40);
        ignore(
            &mut e,
            &[
                LOCK_FREE_MAP_CONSTRUCT_0044C040,
                LOCK_FREE_MAP_CONSTRUCT_0044C270,
                LOCK_FREE_MAP_CONSTRUCT_0044C920,
            ],
        );
        e.call_log = Some(vec![]);
        for (entry, base, vtable) in [
            (
                0x0044_9f50,
                LOCK_FREE_MAP_CONSTRUCT_0044C040,
                LOCK_FREE_MAP_VTABLE_00449F50,
            ),
            (
                0x0044_9fa0,
                LOCK_FREE_MAP_CONSTRUCT_0044C270,
                LOCK_FREE_MAP_VTABLE_00449FA0,
            ),
            (
                0x0044_a100,
                LOCK_FREE_MAP_CONSTRUCT_0044C920,
                LOCK_FREE_MAP_VTABLE_0044A100,
            ),
        ] {
            e.mem.set_u32(this.addr(), 0);
            let back = e.call(entry, &args![this, 1u32, 2u32, 3u32]).ptr::<()>();
            assert_eq!(back, this);
            assert_eq!(calls_to(&e, base).pop(), Some(vec![this.addr(), 1, 2, 3]));
            assert_eq!(e.mem.u32(this.addr()), vtable);
        }
    }

    #[test]
    fn virtual_call_result_is_handed_on_with_the_original_arguments() {
        let (mut e, this) = engine_with_block(8);
        let owner = Ptr::<()>::new(e.mem.alloc(8));
        e.mem.set_u32(this.addr(), owner.addr());
        e.put_vtable(0x0ff0_0000, &[0; 10]);
        e.mem.set_u32(0x0ff0_0000 + 0x24, 0x0ff1_0000);
        e.mem.set_u32(owner.addr(), 0x0ff0_0000);
        e.register(0x0ff1_0000, |_, a| (a[1] + 100).into_ret());
        e.register(LOCK_FREE_MAP_HANDLE, |_, _| 0xffff_ff01u32.into_ret());
        e.call_log = Some(vec![]);
        let result = e.call(0x0044_a130, &args![this, 7u32, 8u32, 1u8]).u32();
        // Only the low byte of the handler's result is the return value.
        assert_eq!(result & 0xff, 1);
        assert_eq!(calls_to(&e, 0x0ff1_0000), vec![vec![owner.addr(), 7]]);
        assert_eq!(
            calls_to(&e, LOCK_FREE_MAP_HANDLE),
            vec![vec![this.addr(), 107, 7, 8, 1]]
        );
    }

    #[test]
    fn byte_global_is_stored() {
        let (mut e, _) = engine_with_block(8);
        e.call(0x0044_ada0, &args![5u8]);
        assert_eq!(e.global::<u8>(TASK_MANAGER_FLAG), 5);
        e.call(0x0044_ada0, &args![0u8]);
        assert_eq!(e.global::<u8>(TASK_MANAGER_FLAG), 0);
    }

    /// A task in `state` whose cancel slot (`0xC`) is a double at
    /// `0ff10100`.
    fn task_in_state(e: &mut Engine, state: u32) -> Ptr {
        let task = Ptr::new(e.mem.alloc(0x20));
        e.put_vtable(0x0ff0_0100, &[0, 0, 0, 0x0ff1_0100]);
        e.mem.set_u32(task.addr(), 0x0ff0_0100);
        e.mem.set_u32(task.addr() + 0xc, state);
        e.register(0x0ff1_0100, |_, _| Ret::default());
        task
    }

    /// Makes `TASK_TRY_SET_STATE` answer `answer` and, like the game's
    /// compare-and-exchange, store 6 in the state when it succeeds.
    fn answer_set_state(e: &mut Engine, answer: bool) {
        e.register_double(TASK_TRY_SET_STATE, move |e, a| {
            if answer {
                e.mem.set_u32(a[0] + 0xc, 6);
            }
            answer.into_ret()
        });
    }

    #[test]
    fn cancel_in_state_0_moves_to_6_and_calls_the_slot() {
        let (mut e, manager) = engine_with_block(8);
        let task = task_in_state(&mut e, 0);
        answer_set_state(&mut e, true);
        ignore(
            &mut e,
            &[TASK_ADD_REFERENCE, TASK_MANAGER_REMOVE, TASK_RELEASE],
        );
        e.set_global(TASK_MANAGER_FLAG, 9u8);
        e.call_log = Some(vec![]);
        e.call(0x0044_ac40, &args![manager, task, 0x55u32]);
        assert_eq!(e.global::<u8>(TASK_MANAGER_FLAG), 0);
        assert_eq!(
            calls_to(&e, TASK_TRY_SET_STATE),
            vec![vec![task.addr(), 0, 6]]
        );
        assert_eq!(calls_to(&e, 0x0ff1_0100), vec![vec![task.addr(), 0, 0x55]]);
        // State 0 takes no reference and does not remove the task.
        assert!(calls_to(&e, TASK_ADD_REFERENCE).is_empty());
        assert!(calls_to(&e, TASK_RELEASE).is_empty());
    }

    #[test]
    fn cancel_in_state_1_references_removes_calls_and_releases() {
        let (mut e, manager) = engine_with_block(8);
        let task = task_in_state(&mut e, 1);
        answer_set_state(&mut e, true);
        ignore(
            &mut e,
            &[TASK_ADD_REFERENCE, TASK_MANAGER_REMOVE, TASK_RELEASE],
        );
        e.call_log = Some(vec![]);
        e.call(0x0044_ac40, &args![manager, task, 0x66u32]);
        assert_eq!(
            call_order(&e),
            vec![
                0x0044_ac40,
                TASK_TRY_SET_STATE,
                TASK_ADD_REFERENCE,
                TASK_MANAGER_REMOVE,
                0x0ff1_0100,
                TASK_RELEASE,
            ]
        );
        assert_eq!(
            calls_to(&e, TASK_MANAGER_REMOVE),
            vec![vec![manager.addr(), task.addr()]]
        );
        assert_eq!(calls_to(&e, 0x0ff1_0100), vec![vec![task.addr(), 1, 0x66]]);
        assert_eq!(calls_to(&e, TASK_RELEASE), vec![vec![task.addr()]]);
    }

    #[test]
    fn cancel_in_state_5_moves_to_6_and_calls_the_slot() {
        let (mut e, manager) = engine_with_block(8);
        let task = task_in_state(&mut e, 5);
        answer_set_state(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x0044_ac40, &args![manager, task, 3u32]);
        assert_eq!(calls_to(&e, 0x0ff1_0100), vec![vec![task.addr(), 5, 3]]);
        assert!(calls_to(&e, TASK_ADD_REFERENCE).is_empty());
    }

    #[test]
    fn cancel_retries_when_the_state_move_fails() {
        let (mut e, manager) = engine_with_block(8);
        let task = task_in_state(&mut e, 0);
        // The first move fails and another thread has since made the task
        // state 5; the second succeeds.
        let mut attempts = 0;
        e.register_double(TASK_TRY_SET_STATE, move |e, a| {
            attempts += 1;
            if attempts == 1 {
                e.mem.set_u32(a[0] + 0xc, 5);
                false.into_ret()
            } else {
                true.into_ret()
            }
        });
        e.call_log = Some(vec![]);
        e.call(0x0044_ac40, &args![manager, task, 1u32]);
        assert_eq!(
            calls_to(&e, TASK_TRY_SET_STATE),
            vec![vec![task.addr(), 0, 6], vec![task.addr(), 5, 6]]
        );
        assert_eq!(calls_to(&e, 0x0ff1_0100), vec![vec![task.addr(), 5, 1]]);
    }

    #[test]
    fn cancel_in_states_3_and_4_sleeps_until_the_state_changes() {
        for state in [3u32, 4] {
            let (mut e, manager) = engine_with_block(8);
            let task = task_in_state(&mut e, state);
            answer_set_state(&mut e, true);
            // The third sleep ends the wait (another thread's work).
            let mut sleeps = 0;
            e.register_double(SLEEP, move |e, a| {
                assert_eq!(a, &[0]);
                sleeps += 1;
                if sleeps == 3 {
                    e.mem.set_u32(task.addr() + 0xc, 5);
                }
                Ret::default()
            });
            e.call_log = Some(vec![]);
            e.call(0x0044_ac40, &args![manager, task, 2u32]);
            assert_eq!(calls_to(&e, SLEEP).len(), 3);
            // Once the state is 5 the usual move and slot call follow.
            assert_eq!(calls_to(&e, 0x0ff1_0100), vec![vec![task.addr(), 5, 2]]);
        }
    }

    #[test]
    fn cancel_in_a_later_state_returns_at_once() {
        let (mut e, manager) = engine_with_block(8);
        let task = task_in_state(&mut e, 6);
        e.call_log = Some(vec![]);
        e.call(0x0044_ac40, &args![manager, task, 2u32]);
        assert!(calls_to(&e, TASK_TRY_SET_STATE).is_empty());
        assert!(calls_to(&e, 0x0ff1_0100).is_empty());
        // (State 2 has no test: its loop makes no call and ends only when
        // another thread changes the state.)
    }

    #[test]
    fn element_action_runs_on_every_element() {
        let (mut e, this) = engine_with_block(0x60);
        let array = e.mem.alloc(12);
        e.mem.set_u32(array, 0xa0);
        e.mem.set_u32(array + 4, 0xb0);
        e.mem.set_u32(array + 8, 0xc0);
        e.mem.set_u32(this.addr() + 0x4c, 3);
        e.mem.set_u32(this.addr() + 0x50, array);
        ignore(&mut e, &[ARRAY_ELEMENT_ACTION]);
        e.call_log = Some(vec![]);
        e.call(0x0044_adb0, &args![this]);
        assert_eq!(
            calls_to(&e, ARRAY_ELEMENT_ACTION),
            vec![vec![0xa0], vec![0xb0], vec![0xc0]]
        );
        // An empty array calls nothing.
        e.mem.set_u32(this.addr() + 0x4c, 0);
        e.call_log = Some(vec![]);
        e.call(0x0044_adb0, &args![this]);
        assert!(calls_to(&e, ARRAY_ELEMENT_ACTION).is_empty());
    }

    #[test]
    fn save_binary_writes_one_value_to_the_streams_output() {
        let (mut e, stream) = engine_with_block(0x300);
        let value = Ptr::<()>::new(e.mem.alloc(4));
        e.register(STREAM_OUTPUT, |e, a| e.mem.u32(a[0] + 0x250).into_ret());
        ignore(&mut e, &[WRITE_BINARY]);
        e.mem.set_u32(stream.addr() + 0x250, 0x0abc);
        e.call_log = Some(vec![]);
        e.call(0x0044_adf0, &args![stream, value]);
        assert_eq!(
            calls_to(&e, WRITE_BINARY),
            vec![vec![0x0abc, value.addr(), 1]]
        );
    }

    #[test]
    fn flag_bit_tests_and_setters_use_their_own_bit() {
        let (mut e, _) = engine_with_block(8);
        let tests = [
            (0x0044_ae10u32, 0x01u8),
            (0x0044_ae50, 0x02),
            (0x0044_ae90, 0x04),
            (0x0044_af20, 0x08),
            (0x0044_af60, 0x10),
            (0x0044_afa0, 0x20),
        ];
        let setters = [
            0x0044_ae20u32,
            0x0044_ae60,
            0x0044_aea0,
            0x0044_af30,
            0x0044_af70,
            0x0044_afb0,
        ];
        let flags = Ptr::<()>::new(e.mem.alloc(4));
        for (index, (test, mask)) in tests.iter().enumerate() {
            // The test sees only its bit.
            assert!(e.call(*test, &args![*mask]).bool());
            assert!(e.call(*test, &args![0xffu8]).bool());
            assert!(!e.call(*test, &args![!*mask]).bool());
            assert!(!e.call(*test, &args![0u8]).bool());
            // The setter sets and clears only its bit, whatever non-zero
            // value it is given.
            e.mem.set_u8(flags.addr(), 0x40);
            e.call(setters[index], &args![0x80u8, flags]);
            assert_eq!(e.mem.u8(flags.addr()), 0x40 | mask);
            e.call(setters[index], &args![0u8, flags]);
            assert_eq!(e.mem.u8(flags.addr()), 0x40);
            e.mem.set_u8(flags.addr(), 0xff);
            e.call(setters[index], &args![0u8, flags]);
            assert_eq!(e.mem.u8(flags.addr()), !mask);
        }
    }

    #[test]
    fn model_pointer_assignment_releases_stores_and_references() {
        let (mut e, slot) = engine_with_block(8);
        ignore(&mut e, &[MODEL_RELEASE, MODEL_ADD_REFERENCE]);
        e.mem.set_u32(slot.addr(), 0x1000);
        e.call_log = Some(vec![]);
        let back = e
            .call(0x0044_aed0, &args![slot, Ptr::<Model>::new(0x2000)])
            .ptr::<()>();
        assert_eq!(back, slot);
        assert_eq!(e.mem.u32(slot.addr()), 0x2000);
        assert_eq!(calls_to(&e, MODEL_RELEASE), vec![vec![0x1000]]);
        assert_eq!(calls_to(&e, MODEL_ADD_REFERENCE), vec![vec![0x2000]]);

        // The same model: nothing happens.
        e.call_log = Some(vec![]);
        e.call(0x0044_aed0, &args![slot, Ptr::<Model>::new(0x2000)]);
        assert_eq!(call_order(&e).len(), 1);

        // From null to a model: no release; to null: no reference.
        e.mem.set_u32(slot.addr(), 0);
        e.call_log = Some(vec![]);
        e.call(0x0044_aed0, &args![slot, Ptr::<Model>::new(0x3000)]);
        assert!(calls_to(&e, MODEL_RELEASE).is_empty());
        assert_eq!(calls_to(&e, MODEL_ADD_REFERENCE), vec![vec![0x3000]]);
        e.call_log = Some(vec![]);
        e.call(0x0044_aed0, &args![slot, Ptr::<Model>::NULL]);
        assert_eq!(calls_to(&e, MODEL_RELEASE), vec![vec![0x3000]]);
        assert!(calls_to(&e, MODEL_ADD_REFERENCE).is_empty());
        assert_eq!(e.mem.u32(slot.addr()), 0);
    }

    /// An engine whose interlocked operations really change the counter, so
    /// the tests see the counters move.
    fn counting_engine() -> (Engine, Ptr) {
        let (mut e, block) = engine_with_block(0x40);
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let value = e.mem.i32(a[0]).wrapping_add(1);
            e.mem.set_i32(a[0], value);
            value.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let value = e.mem.i32(a[0]).wrapping_sub(1);
            e.mem.set_i32(a[0], value);
            value.into_ret()
        });
        (e, block)
    }

    #[test]
    fn counted_at_0xc_reference_functions() {
        let (mut e, object) = counting_engine();
        let slot = Ptr::<()>::new(e.mem.alloc(4));
        e.call(0x0044_b010, &args![object]);
        assert_eq!(e.mem.i32(object.addr() + 0xc), 1);
        e.call(0x0044_b050, &args![object]);
        assert_eq!(e.mem.i32(object.addr() + 0xc), 0);

        // Constructor: stores and references; null stores only.
        let back = e.call(0x0044_afe0, &args![slot, object]).ptr::<()>();
        assert_eq!(back, slot);
        assert_eq!(e.mem.u32(slot.addr()), object.addr());
        assert_eq!(e.mem.i32(object.addr() + 0xc), 1);
        // Destructor: drops one reference, keeps the pointer.
        e.call(0x0044_b030, &args![slot]);
        assert_eq!(e.mem.i32(object.addr() + 0xc), 0);
        assert_eq!(e.mem.u32(slot.addr()), object.addr());
        e.call(0x0044_afe0, &args![slot, Ptr::<()>::NULL]);
        assert_eq!(e.mem.u32(slot.addr()), 0);
        e.call(0x0044_b030, &args![slot]);
        assert_eq!(e.mem.i32(object.addr() + 0xc), 0);
    }

    #[test]
    fn counted_at_0xc_assignment_moves_the_reference() {
        let (mut e, first) = counting_engine();
        let second = Ptr::<()>::new(e.mem.alloc(0x40));
        let slot = Ptr::<()>::new(e.mem.alloc(4));
        e.call(0x0044_afe0, &args![slot, first]);
        assert_eq!(e.mem.i32(first.addr() + 0xc), 1);
        // Assigning the same object changes nothing.
        e.call(0x0044_b070, &args![slot, first]);
        assert_eq!(e.mem.i32(first.addr() + 0xc), 1);
        let back = e.call(0x0044_b070, &args![slot, second]).ptr::<()>();
        assert_eq!(back, slot);
        assert_eq!(e.mem.u32(slot.addr()), second.addr());
        assert_eq!(e.mem.i32(first.addr() + 0xc), 0);
        assert_eq!(e.mem.i32(second.addr() + 0xc), 1);
        // Assigning null drops the reference.
        e.call(0x0044_b070, &args![slot, Ptr::<()>::NULL]);
        assert_eq!(e.mem.u32(slot.addr()), 0);
        assert_eq!(e.mem.i32(second.addr() + 0xc), 0);
    }

    /// Adds the lock doubles: `TryEnterCriticalSection` fails `refusals`
    /// times, then succeeds; leaving does nothing.
    fn lock_doubles(e: &mut Engine, refusals: u32) {
        let mut remaining = refusals;
        e.register_double(TRY_ENTER_CRITICAL_SECTION, move |_, _| {
            if remaining > 0 {
                remaining -= 1;
                0i32.into_ret()
            } else {
                1i32.into_ret()
            }
        });
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
    }

    #[test]
    fn try_enter_returns_whether_the_section_was_entered() {
        let (mut e, section) = engine_with_block(0x20);
        e.register(TRY_ENTER_CRITICAL_SECTION, |_, a| {
            u32::from(a[0] == LOADED_FILE_LOCK).into_ret()
        });
        assert!(e
            .call(0x0044_b130, &args![Ptr::<()>::new(LOADED_FILE_LOCK)])
            .bool());
        assert!(!e.call(0x0044_b130, &args![section]).bool());
    }

    #[test]
    fn loaded_file_reference_is_added_under_the_lock() {
        let (mut e, file) = counting_engine();
        lock_doubles(&mut e, 2);
        e.call_log = Some(vec![]);
        e.call(0x0044_b0f0, &args![file]);
        assert_eq!(e.mem.i32(file.addr()), 1);
        // Two refused attempts, then the entry that worked.
        assert_eq!(
            calls_to(&e, TRY_ENTER_CRITICAL_SECTION),
            vec![vec![LOADED_FILE_LOCK]; 3]
        );
        assert_eq!(
            calls_to(&e, LEAVE_CRITICAL_SECTION),
            vec![vec![LOADED_FILE_LOCK]]
        );
        // The counter changes after the entry and before the leave.
        let order = call_order(&e);
        assert_eq!(
            &order[order.len() - 2..],
            &[INTERLOCKED_INCREMENT, LEAVE_CRITICAL_SECTION]
        );
    }

    #[test]
    fn loaded_file_release_deletes_at_zero_only() {
        let (mut e, file) = counting_engine();
        lock_doubles(&mut e, 1);
        e.register(LOADED_FILE_DESTRUCT, |_, _| Ret::default());
        e.register(NI_FREE, |_, _| Ret::default());
        e.mem.set_i32(file.addr(), 2);
        e.call_log = Some(vec![]);
        e.call(0x0044_b180, &args![file]);
        assert_eq!(e.mem.i32(file.addr()), 1);
        assert!(calls_to(&e, LOADED_FILE_DESTRUCT).is_empty());
        assert_eq!(calls_to(&e, TRY_ENTER_CRITICAL_SECTION).len(), 2);
        // The last reference: destroyed and freed (flag 1) after the leave.
        e.call_log = Some(vec![]);
        e.call(0x0044_b180, &args![file]);
        assert_eq!(e.mem.i32(file.addr()), 0);
        let order = call_order(&e);
        assert_eq!(
            &order[order.len() - 3..],
            &[LEAVE_CRITICAL_SECTION, LOADED_FILE_DESTRUCT, NI_FREE]
        );
        assert_eq!(
            calls_to(&e, NI_FREE),
            vec![vec![file.addr(), LOADED_FILE_SIZE]]
        );
    }

    #[test]
    fn loaded_file_scalar_deleting_destructor_frees_only_with_flag_1() {
        let (mut e, file) = engine_with_block(0x10);
        e.register(LOADED_FILE_DESTRUCT, |_, _| Ret::default());
        e.register(NI_FREE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let back = e.call(0x0044_b1f0, &args![file, 0u32]).ptr::<()>();
        assert_eq!(back, file);
        assert_eq!(calls_to(&e, LOADED_FILE_DESTRUCT), vec![vec![file.addr()]]);
        assert!(calls_to(&e, NI_FREE).is_empty());
        e.call(0x0044_b1f0, &args![file, 1u32]);
        assert_eq!(calls_to(&e, NI_FREE), vec![vec![file.addr(), 0x10]]);
    }

    #[test]
    fn loaded_file_pointer_constructor_destructor_and_assignment() {
        let (mut e, first) = counting_engine();
        let second = Ptr::<()>::new(e.mem.alloc(0x10));
        let slot = Ptr::<()>::new(e.mem.alloc(4));
        lock_doubles(&mut e, 0);
        e.register(LOADED_FILE_DESTRUCT, |_, _| Ret::default());
        e.register(NI_FREE, |_, _| Ret::default());
        let first_file = Ptr::<LoadedFile>::new(first.addr());
        let second_file = Ptr::<LoadedFile>::new(second.addr());

        // Constructor.
        let back = e.call(0x0044_b0c0, &args![slot, first_file]).ptr::<()>();
        assert_eq!(back, slot);
        assert_eq!(e.mem.u32(slot.addr()), first.addr());
        assert_eq!(e.mem.i32(first.addr()), 1);

        // Assigning the same file changes nothing; another one moves the
        // reference (and deletes the old file at zero).
        e.call(0x0044_b220, &args![slot, first_file]);
        assert_eq!(e.mem.i32(first.addr()), 1);
        e.call_log = Some(vec![]);
        e.call(0x0044_b220, &args![slot, second_file]);
        assert_eq!(e.mem.u32(slot.addr()), second.addr());
        assert_eq!(e.mem.i32(first.addr()), 0);
        assert_eq!(e.mem.i32(second.addr()), 1);
        assert_eq!(calls_to(&e, LOADED_FILE_DESTRUCT), vec![vec![first.addr()]]);

        // Destructor releases; a null pointer does nothing.
        e.call_log = Some(vec![]);
        e.call(0x0044_b160, &args![slot]);
        assert_eq!(e.mem.i32(second.addr()), 0);
        assert_eq!(
            calls_to(&e, LOADED_FILE_DESTRUCT),
            vec![vec![second.addr()]]
        );
        e.mem.set_u32(slot.addr(), 0);
        e.call_log = Some(vec![]);
        e.call(0x0044_b160, &args![slot]);
        assert_eq!(call_order(&e).len(), 1);
        // Assigning null releases the old one only.
        e.mem.set_u32(slot.addr(), second.addr());
        e.mem.set_i32(second.addr(), 1);
        e.call(0x0044_b220, &args![slot, Ptr::<LoadedFile>::NULL]);
        assert_eq!(e.mem.u32(slot.addr()), 0);
        assert_eq!(e.mem.i32(second.addr()), 0);
    }

    #[test]
    fn counted_at_0x20_constructor_reference_and_destructor() {
        let (mut e, object) = counting_engine();
        let slot = Ptr::<()>::new(e.mem.alloc(4));
        e.call(0x0044_b2a0, &args![object]);
        assert_eq!(e.mem.i32(object.addr() + 0x20), 1);
        let back = e.call(0x0044_b270, &args![slot, object]).ptr::<()>();
        assert_eq!(back, slot);
        assert_eq!(e.mem.u32(slot.addr()), object.addr());
        assert_eq!(e.mem.i32(object.addr() + 0x20), 2);
        // A null pointer is stored without a reference.
        e.call(0x0044_b270, &args![slot, Ptr::<()>::NULL]);
        assert_eq!(e.mem.u32(slot.addr()), 0);

        // The destructor drops a reference only for a non-null pointer.
        e.call_log = Some(vec![]);
        e.mem.set_u32(slot.addr(), 0);
        e.call(0x0044_b2c0, &args![slot]);
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).is_empty());
        e.mem.set_u32(slot.addr(), object.addr());
        e.call(0x0044_b2c0, &args![slot]);
        assert_eq!(
            calls_to(&e, INTERLOCKED_DECREMENT),
            vec![vec![object.addr() + 0x20]]
        );
    }

    // ----- 0044b2e0 up to 00ae78e0 -------------------------------------

    /// An engine whose allocation, scope guard and queue-member doubles do
    /// what the game's do for the loader's queue code: the allocator hands
    /// out `Mem` blocks (at most 0x100 bytes whatever the request, the
    /// request itself stays in the call log), the constructors return their
    /// block.
    fn queue_engine() -> Engine {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, a| {
            e.mem.alloc(a[0].min(0x100)).into_ret()
        });
        e.register(QUEUE_NODE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(QUEUE_COUNTER_CONSTRUCT, |_, a| a[0].into_ret());
        ignore(
            &mut e,
            &[
                QUEUE_MEMBER_CONSTRUCT,
                SCOPE_GUARD_ENTER,
                SCOPE_GUARD_LEAVE,
                VECTOR_CONSTRUCT,
                MEMORY_FREE,
            ],
        );
        e
    }

    #[test]
    fn counted_at_0x20_release_and_assignment() {
        let (mut e, first) = counting_engine();
        let second = Ptr::<()>::new(e.mem.alloc(0x40));
        let slot = Ptr::<()>::new(e.mem.alloc(4));
        e.mem.set_i32(first.addr() + 0x20, 1);
        e.call(0x0044_b2e0, &args![first]);
        assert_eq!(e.mem.i32(first.addr() + 0x20), 0);

        // Assigning moves the references; assigning the same object and a
        // null pointer behave as the smart pointer does.
        e.mem.set_u32(slot.addr(), first.addr());
        e.mem.set_i32(first.addr() + 0x20, 1);
        let back = e.call(0x0044_b300, &args![slot, second]).ptr::<()>();
        assert_eq!(back, slot);
        assert_eq!(e.mem.u32(slot.addr()), second.addr());
        assert_eq!(e.mem.i32(first.addr() + 0x20), 0);
        assert_eq!(e.mem.i32(second.addr() + 0x20), 1);
        e.call(0x0044_b300, &args![slot, second]);
        assert_eq!(e.mem.i32(second.addr() + 0x20), 1);
        e.call(0x0044_b300, &args![slot, Ptr::<()>::NULL]);
        assert_eq!(e.mem.u32(slot.addr()), 0);
        assert_eq!(e.mem.i32(second.addr() + 0x20), 0);
        e.call(0x0044_b300, &args![slot, first]);
        assert_eq!(e.mem.i32(first.addr() + 0x20), 1);
    }

    #[test]
    fn queue_constructors_fill_the_fields_in_the_scope_guard() {
        for (entry, vtable) in [(0x0044_b350, QUEUE_VTABLE_A), (0x0044_b500, QUEUE_VTABLE_B)] {
            let mut e = queue_engine();
            let this = Ptr::<()>::new(e.mem.alloc(0x40));
            e.call_log = Some(vec![]);
            let back = e.call(entry, &args![this, 5u32, 0xabcd_u32]).ptr::<()>();
            assert_eq!(back, this);
            assert_eq!(e.mem.u32(this.addr()), vtable);
            let first_node = e.mem.u32(this.addr() + 4);
            assert_ne!(first_node, 0);
            assert_eq!(e.mem.u32(this.addr() + 8), first_node);
            assert_eq!(e.mem.u32(this.addr() + 0xc), 0xabcd);
            assert_ne!(e.mem.u32(this.addr() + 0x10), 0);
            assert_eq!(e.mem.u32(this.addr() + 0x14) % 8, 0);
            assert_ne!(e.mem.u32(this.addr() + 0x14), 0);
            assert_eq!(e.mem.u32(this.addr() + 0x18), 0);
            // The base part first, the member, the guard around the rest.
            let order = call_order(&e);
            assert_eq!(order[1], QUEUE_MEMBER_CONSTRUCT);
            assert_eq!(order[2], SCOPE_GUARD_ENTER);
            assert_eq!(*order.last().unwrap(), SCOPE_GUARD_LEAVE);
            let enter = &calls_to(&e, SCOPE_GUARD_ENTER)[0];
            assert_eq!(
                &enter[1..],
                &[6, 1, THREAD_SAFE_STRUCTURES_SOURCE, QUEUE_CONSTRUCTOR_LINE]
            );
            // Block sizes: first node, `size * 2` words, the counter.
            let sizes: Vec<u32> = calls_to(&e, MEMORY_ALLOCATE).iter().map(|a| a[0]).collect();
            assert_eq!(sizes, vec![8, 40, 0x10]);
            assert_eq!(
                calls_to(&e, QUEUE_COUNTER_CONSTRUCT),
                vec![vec![e.mem.u32(this.addr() + 0x14), 5]]
            );
        }
    }

    #[test]
    fn queue_constructor_saturates_the_slot_size_and_survives_failed_allocations() {
        let mut e = queue_engine();
        let this = Ptr::<()>::new(e.mem.alloc(0x40));
        e.call_log = Some(vec![]);
        // `0x2000_0000 * 2 * 4` does not fit 32 bits: all ones is asked for.
        e.call(0x0044_b350, &args![this, 0x2000_0000u32, 0u32]);
        let sizes: Vec<u32> = calls_to(&e, MEMORY_ALLOCATE).iter().map(|a| a[0]).collect();
        assert_eq!(sizes[1], u32::MAX);

        // Every allocation fails: no constructor runs and the pointers are 0.
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        e.mem.set_u32(this.addr() + 4, 7);
        e.mem.set_u32(this.addr() + 0x14, 7);
        e.call_log = Some(vec![]);
        e.call(0x0044_b350, &args![this, 3u32, 0u32]);
        assert!(calls_to(&e, QUEUE_NODE_CONSTRUCT).is_empty());
        assert!(calls_to(&e, QUEUE_COUNTER_CONSTRUCT).is_empty());
        assert_eq!(e.mem.u32(this.addr() + 4), 0);
        assert_eq!(e.mem.u32(this.addr() + 8), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x14), 0);
    }

    #[test]
    fn slot_queue_constructors_build_their_slots() {
        for (entry, vtable) in [
            (0x0044_bbe0, SLOT_QUEUE_VTABLE_BBE0),
            (0x0044_be80, SLOT_QUEUE_VTABLE_BE80),
            (0x0044_c760, SLOT_QUEUE_VTABLE_C760),
        ] {
            let mut e = queue_engine();
            let this = Ptr::<()>::new(e.mem.alloc(0x40));
            e.call_log = Some(vec![]);
            let back = e.call(entry, &args![this, 4u32, 6u32, 0x77u32]).ptr::<()>();
            assert_eq!(back, this);
            assert_eq!(e.mem.u32(this.addr()), vtable);
            assert_eq!(e.mem.u32(this.addr() + 8), 6);
            assert_eq!(e.mem.u32(this.addr() + 0x10), 0x77);
            assert_eq!(e.mem.u32(this.addr() + 0x18), 0);
            let slots = e.mem.u32(this.addr() + 0xc);
            assert_ne!(slots, 0);
            assert_ne!(e.mem.u32(this.addr() + 4), 0);
            assert_eq!(
                calls_to(&e, VECTOR_CONSTRUCT),
                vec![vec![slots, 4, 6, QUEUE_SLOT_CONSTRUCT]]
            );
            let sizes: Vec<u32> = calls_to(&e, MEMORY_ALLOCATE).iter().map(|a| a[0]).collect();
            // `count` slots of four bytes, `size * 3` words, the counter.
            assert_eq!(sizes, vec![24, 48, 0x10]);
            let enter = &calls_to(&e, SCOPE_GUARD_ENTER)[0];
            assert_eq!(enter[4], SLOT_QUEUE_CONSTRUCTOR_LINE);
        }
    }

    #[test]
    fn slot_queue_constructor_skips_the_slot_construction_without_memory() {
        let mut e = queue_engine();
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        let this = Ptr::<()>::new(e.mem.alloc(0x40));
        e.mem.set_u32(this.addr() + 0xc, 9);
        e.call_log = Some(vec![]);
        e.call(0x0044_bbe0, &args![this, 4u32, 6u32, 0u32]);
        assert!(calls_to(&e, VECTOR_CONSTRUCT).is_empty());
        assert_eq!(e.mem.u32(this.addr() + 0xc), 0);
        assert_eq!(e.mem.u32(this.addr() + 4), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x14), 0);
    }

    #[test]
    fn small_object_constructors_store_vtable_and_zeros() {
        let mut e = Engine::new();
        for (entry, vtable) in [
            (0x0044_cb70, SMALL_OBJECT_VTABLE_CB70),
            (0x0044_cbb0, SMALL_OBJECT_VTABLE_CBB0),
            (0x0044_cc10, SMALL_OBJECT_VTABLE_CC10),
            (0x0044_cc50, SMALL_OBJECT_VTABLE_CC50),
            (0x0044_cc90, SMALL_OBJECT_VTABLE_CC90),
        ] {
            let this = Ptr::<()>::new(e.mem.alloc(0x10));
            for offset in 0..4 {
                e.mem.set_u32(this.addr() + 4 * offset, 0xffff_ffff);
            }
            let back = e.call(entry, &args![this]).ptr::<()>();
            assert_eq!(back, this);
            assert_eq!(e.mem.u32(this.addr()), vtable);
            assert_eq!(e.mem.u32(this.addr() + 4), 0);
            assert_eq!(e.mem.u32(this.addr() + 8), 0);
            assert_eq!(e.mem.u8(this.addr() + 0xc), 0);
            // The byte after the flag is not touched.
            assert_eq!(e.mem.u8(this.addr() + 0xd), 0xff);
        }
    }

    /// A lock-free queue in memory: the queue object (head at +4, tail at
    /// +8, virtual functions at slots 2 and 3 that bump counters in the
    /// queue's +0x20 and +0x24), a dummy first node, and an interface whose
    /// published slots are real words. Returns (interface, queue, dummy).
    fn lock_free_engine() -> (Engine, Ptr, u32, u32) {
        let mut e = queue_engine();
        e.register(COMPARE_EXCHANGE, |e, a| {
            if e.mem.u32(a[0]) == a[2] {
                e.mem.set_u32(a[0], a[1]);
                1u32.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.register(POINTER_COPY, |e, a| {
            // NiPointer copy assignment: the argument is the source slot.
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            a[0].into_ret()
        });
        e.register(0x0010_0008, |e, a| {
            let count = e.mem.u32(a[0] + 0x20);
            e.mem.set_u32(a[0] + 0x20, count + 1);
            Ret::default()
        });
        e.register(0x0010_000c, |e, a| {
            let count = e.mem.u32(a[0] + 0x24);
            e.mem.set_u32(a[0] + 0x24, count + 1);
            Ret::default()
        });
        e.put_vtable(0x0300_0000, &[0, 0, 0x0010_0008, 0x0010_000c]);
        let queue = e.mem.alloc(0x40);
        let dummy = e.mem.alloc(8);
        e.mem.set_u32(queue, 0x0300_0000);
        e.mem.set_u32(queue + 4, dummy);
        e.mem.set_u32(queue + 8, dummy);
        let interface = Ptr::<()>::new(e.mem.alloc(0x10));
        let slot_one = e.mem.alloc(4);
        let slot_two = e.mem.alloc(4);
        e.mem.set_u32(interface.addr(), queue);
        e.mem.set_u32(interface.addr() + 4, slot_one);
        e.mem.set_u32(interface.addr() + 8, slot_two);
        (e, interface, queue, dummy)
    }

    #[test]
    fn enqueue_links_a_node_after_the_tail() {
        let (mut e, interface, queue, dummy) = lock_free_engine();
        let item = e.mem.alloc(4);
        e.mem.set_u32(item, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x0044_cd40, &args![interface, item]);
        let node = e.mem.u32(dummy);
        assert_ne!(node, 0);
        // The node holds the item, is the new tail, and the queue was told.
        assert_eq!(e.mem.u32(node + 4), 0x1234);
        assert_eq!(e.mem.u32(queue + 8), node);
        assert_eq!(e.mem.u32(queue + 4), dummy);
        assert_eq!(e.mem.u32(queue + 0x20), 1);
        assert_eq!(e.mem.u32(queue + 0x24), 0);
        // The published slot is cleared and the guard surrounds everything.
        assert_eq!(e.mem.u32(e.mem.u32(interface.addr() + 4)), 0);
        let enter = &calls_to(&e, SCOPE_GUARD_ENTER)[0];
        assert_eq!(
            &enter[1..],
            &[6, 1, THREAD_SAFE_STRUCTURES_SOURCE, QUEUE_ENQUEUE_LINE]
        );
        let order = call_order(&e);
        assert_eq!(*order.last().unwrap(), SCOPE_GUARD_LEAVE);
    }

    #[test]
    fn enqueue_helps_a_lagging_tail_first() {
        let (mut e, interface, queue, dummy) = lock_free_engine();
        // Another thread linked a node but did not move the tail yet.
        let other = e.mem.alloc(8);
        e.mem.set_u32(dummy, other);
        let item = e.mem.alloc(4);
        e.mem.set_u32(item, 0x55);
        e.call_log = Some(vec![]);
        e.call(0x0044_cd40, &args![interface, item]);
        let node = e.mem.u32(other);
        assert_ne!(node, 0);
        assert_eq!(e.mem.u32(node + 4), 0x55);
        assert_eq!(e.mem.u32(queue + 8), node);
        // The help was one exchange moving the tail to the other node.
        assert_eq!(
            calls_to(&e, COMPARE_EXCHANGE)[0],
            vec![queue + 8, other, dummy]
        );
    }

    #[test]
    fn dequeue_returns_false_on_an_empty_queue() {
        let (mut e, interface, queue, _) = lock_free_engine();
        e.register(TASK_RELEASE, |_, _| Ret::default());
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0044_cec0, &args![interface, out]).bool());
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(e.mem.u32(e.mem.u32(interface.addr() + 4)), 0);
        assert_eq!(e.mem.u32(queue + 0x24), 0);
        assert!(calls_to(&e, RECYCLE_NODE).is_empty());
    }

    #[test]
    fn dequeue_takes_the_first_item_and_recycles_the_node() {
        let (mut e, interface, queue, dummy) = lock_free_engine();
        e.register(TASK_RELEASE, |_, _| Ret::default());
        e.register(TASK_ADD_REFERENCE, |_, _| Ret::default());
        ignore(&mut e, &[RECYCLE_NODE]);
        // dummy -> first (holding 0x77) -> second.
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(dummy, first);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 4, 0x77);
        e.mem.set_u32(queue + 8, second);
        let out = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0044_cec0, &args![interface, out]).bool());
        // The item is in `out`, the head moved to the first node, the node's
        // item was released (and its slot cleared), the queue was told (+0xc).
        assert_eq!(e.mem.u32(out), 0x77);
        assert_eq!(e.mem.u32(queue + 4), first);
        assert_eq!(e.mem.u32(first + 4), 0);
        assert_eq!(e.mem.u32(queue + 0x24), 1);
        assert_eq!(e.mem.u32(queue + 0x20), 0);
        assert_eq!(
            calls_to(&e, RECYCLE_NODE),
            vec![vec![interface.addr(), dummy]]
        );
        assert_eq!(calls_to(&e, TASK_RELEASE), vec![vec![0x77]]);
        assert_eq!(e.mem.u32(e.mem.u32(interface.addr() + 4)), 0);
        assert_eq!(e.mem.u32(e.mem.u32(interface.addr() + 8)), 0);
    }

    #[test]
    fn dequeue_helps_a_lagging_tail_before_taking_the_item() {
        let (mut e, interface, queue, dummy) = lock_free_engine();
        e.register(TASK_RELEASE, |_, _| Ret::default());
        e.register(TASK_ADD_REFERENCE, |_, _| Ret::default());
        ignore(&mut e, &[RECYCLE_NODE]);
        // The head and the tail are both the dummy, but a node follows it.
        let first = e.mem.alloc(8);
        e.mem.set_u32(dummy, first);
        e.mem.set_u32(first + 4, 0x66);
        let out = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0044_cec0, &args![interface, out]).bool());
        assert_eq!(
            calls_to(&e, COMPARE_EXCHANGE)[0],
            vec![queue + 8, first, dummy]
        );
        assert_eq!(e.mem.u32(out), 0x66);
        assert_eq!(e.mem.u32(queue + 4), first);
    }

    #[test]
    fn exchange_loop_returns_false_when_the_step_fails() {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(0x20));
        e.register(EXCHANGE_STEP, |_, _| 0u32.into_ret());
        ignore(&mut e, &[EXCHANGE_FINISH]);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0044_d3f0, &args![this, 1u32, 2u32]).bool());
        assert_eq!(
            call_order(&e),
            vec![0x0044_d3f0, EXCHANGE_STEP, EXCHANGE_FINISH]
        );
        assert_eq!(calls_to(&e, EXCHANGE_STEP), vec![vec![this.addr(), 1, 2]]);
    }

    /// Doubles for the tagged-pointer helpers of `0044d3f0`: the tagged
    /// word is `pointer | tag`, `00559450` reads it, `0044dec0` clears bit 0
    /// of the word at its argument.
    fn exchange_engine(exchange_results: &'static [u32]) -> (Engine, Ptr) {
        let mut e = Engine::new();
        e.register(EXCHANGE_STEP, |_, _| 1u32.into_ret());
        e.register(TAGGED_POINTER, |e, a| (e.mem.u32(a[0]) & !1).into_ret());
        e.register(TAGGED_POINTER_TWIN, |e, a| {
            (e.mem.u32(a[0]) & !1).into_ret()
        });
        e.register(TAGGED_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[2] | a[1]);
            Ret::default()
        });
        e.register(TAGGED_VALUE, |e, a| e.mem.u32(a[0]).into_ret());
        ignore(&mut e, &[EXCHANGE_COMMIT, EXCHANGE_FINISH]);
        let results = exchange_results;
        let mut index = 0usize;
        e.register_double(COMPARE_EXCHANGE, move |_, _| {
            index += 1;
            results[(index - 1).min(results.len() - 1)].into_ret()
        });
        let this = Ptr::<()>::new(e.mem.alloc(0x20));
        e.mem.set_u32(this.addr() + 0x10, 0x2000);
        e.mem.set_u32(this.addr() + 0x14, 0x3000);
        e.mem.set_u32(this.addr() + 0x18, 0x4000);
        (e, this)
    }

    #[test]
    fn exchange_loop_commits_when_the_final_exchange_works() {
        // The first exchange (inside the loop) fails once, then both succeed.
        let (mut e, this) = exchange_engine(&[0, 1, 1]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0044_d3f0, &args![this, 9u32, 8u32]).bool());
        assert_eq!(calls_to(&e, EXCHANGE_STEP).len(), 2);
        let exchanges = calls_to(&e, COMPARE_EXCHANGE);
        assert_eq!(exchanges.len(), 3);
        // Loop: target is the pointer part at +0x14, plus 8; new = tag 1,
        // expected = tag 0, both of the pointer at +0x18.
        assert_eq!(exchanges[0], vec![0x3000 + 8, 0x4001, 0x4000]);
        // Final: the word at +0x10 becomes the tagged pointer at +0x14.
        assert_eq!(exchanges[2], vec![0x2000, 0x4000, 0x3000]);
        assert_eq!(
            calls_to(&e, EXCHANGE_COMMIT),
            vec![vec![this.addr(), 0x3000]]
        );
        assert_eq!(*call_order(&e).last().unwrap(), EXCHANGE_FINISH);
    }

    #[test]
    fn exchange_loop_retries_the_step_when_the_final_exchange_fails() {
        let (mut e, this) = exchange_engine(&[1, 0]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0044_d3f0, &args![this, 9u32, 8u32]).bool());
        assert!(calls_to(&e, EXCHANGE_COMMIT).is_empty());
        assert_eq!(
            calls_to(&e, EXCHANGE_STEP),
            vec![vec![this.addr(), 9, 8]; 2]
        );
        assert_eq!(*call_order(&e).last().unwrap(), EXCHANGE_FINISH);
    }

    #[test]
    fn queue_destructors_clear_free_and_reset_the_base() {
        for (body, scalar, vtable) in [
            (0x0044_d770, 0x0044_d740, QUEUE_VTABLE_A),
            (0x0044_d840, 0x0044_d810, QUEUE_VTABLE_B),
        ] {
            let mut e = queue_engine();
            ignore(&mut e, &[QUEUE_CLEAR]);
            let this = Ptr::<()>::new(e.mem.alloc(0x40));
            e.mem.set_u32(this.addr() + 0x10, 0x5555);
            e.call_log = Some(vec![]);
            e.call(body, &args![this]);
            assert_eq!(call_order(&e), vec![body, QUEUE_CLEAR, MEMORY_FREE]);
            assert_eq!(calls_to(&e, QUEUE_CLEAR), vec![vec![this.addr(), 1]]);
            assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![0x5555]]);
            // The base table ends up in the object.
            assert_eq!(e.mem.u32(this.addr()), INTERFACED_CLASS_VTABLE);
            let _ = vtable;

            // The scalar deleting form also frees the object with flag 1.
            e.call_log = Some(vec![]);
            let back = e.call(scalar, &args![this, 0u32]).ptr::<()>();
            assert_eq!(back, this);
            assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![0x5555]]);
            e.call_log = Some(vec![]);
            e.call(scalar, &args![this, 1u32]);
            assert_eq!(
                calls_to(&e, MEMORY_FREE),
                vec![vec![0x5555], vec![this.addr()]]
            );
        }
    }

    #[test]
    fn queue_destructor_stores_its_own_table_before_clearing() {
        let mut e = queue_engine();
        e.register(QUEUE_CLEAR, |e, a| {
            // The clear runs with the queue's own table already stored.
            let table = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 0x30, table);
            Ret::default()
        });
        let this = Ptr::<()>::new(e.mem.alloc(0x40));
        e.call(0x0044_d770, &args![this]);
        assert_eq!(e.mem.u32(this.addr() + 0x30), QUEUE_VTABLE_A);
        e.call(0x0044_d840, &args![this]);
        assert_eq!(e.mem.u32(this.addr() + 0x30), QUEUE_VTABLE_B);
    }

    #[test]
    fn interfaced_class_base_functions() {
        let mut e = queue_engine();
        let this = Ptr::<()>::new(e.mem.alloc(8));
        let back = e.call(0x0044_d7f0, &args![this]).ptr::<()>();
        assert_eq!(back, this);
        assert_eq!(e.mem.u32(this.addr()), INTERFACED_CLASS_VTABLE);
        e.mem.set_u32(this.addr(), 0);
        e.call(0x0044_b4b0, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), INTERFACED_CLASS_VTABLE);

        e.call_log = Some(vec![]);
        let back = e.call(0x0044_b4d0, &args![this, 0u32]).ptr::<()>();
        assert_eq!(back, this);
        assert!(calls_to(&e, MEMORY_FREE).is_empty());
        e.call(0x0044_b4d0, &args![this, 1u32]);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![this.addr()]]);
    }

    /// The settings classes' doubles.
    fn settings_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        ignore(
            &mut e,
            &[
                SETTINGS_BASE_CONSTRUCT,
                QUEUE_MEMBER_CONSTRUCT,
                INI_COLLECTION_BASE_DESTRUCT,
                SETTINGS_LIST_DESTRUCT_FIRST,
                SETTINGS_LIST_DESTRUCT_SECOND,
                SETTINGS_BASE_DESTRUCT,
                SETTINGS_OBJECT_DESTRUCT,
                SCOPE_GUARD_ENTER,
                SCOPE_GUARD_LEAVE,
                MEMORY_FREE,
            ],
        );
        e
    }

    #[test]
    fn settings_constructors_store_tables_and_construct_the_list() {
        let mut e = settings_engine();
        let this = Ptr::<()>::new(e.mem.alloc(0x114));
        e.call_log = Some(vec![]);
        let back = e.call(0x0044_f700, &args![this]).ptr::<()>();
        assert_eq!(back, this);
        assert_eq!(e.mem.u32(this.addr()), SETTINGS_VTABLE);
        assert_eq!(
            call_order(&e),
            vec![0x0044_f700, SETTINGS_BASE_CONSTRUCT, QUEUE_MEMBER_CONSTRUCT]
        );
        assert_eq!(
            calls_to(&e, QUEUE_MEMBER_CONSTRUCT),
            vec![vec![this.addr() + 0x10c]]
        );
        let back = e.call(0x0044_f600, &args![this]).ptr::<()>();
        assert_eq!(back, this);
        assert_eq!(e.mem.u32(this.addr()), SETTINGS_DERIVED_VTABLE);
    }

    #[test]
    fn settings_singleton_is_created_once() {
        let mut e = settings_engine();
        e.call_log = Some(vec![]);
        let first = e.call(0x0044_f560, &args![]).ptr::<()>();
        assert!(!first.is_null());
        assert_eq!(e.global::<u32>(SETTINGS_INSTANCE), first.addr());
        assert_eq!(e.mem.u32(first.addr()), SETTINGS_DERIVED_VTABLE);
        assert_eq!(calls_to(&e, MEMORY_ALLOCATE), vec![vec![0x114]]);
        let second = e.call(0x0044_f560, &args![]).ptr::<()>();
        assert_eq!(second, first);
        assert_eq!(calls_to(&e, MEMORY_ALLOCATE).len(), 1);
        // The creator alone does the same.
        e.set_global(SETTINGS_INSTANCE, 0u32);
        e.call(0x0044_f570, &args![]);
        assert_ne!(e.global::<u32>(SETTINGS_INSTANCE), 0);
    }

    #[test]
    fn settings_singleton_stays_null_when_the_allocation_fails() {
        let mut e = settings_engine();
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        e.call(0x0044_f570, &args![]);
        assert_eq!(e.global::<u32>(SETTINGS_INSTANCE), 0);
    }

    #[test]
    fn ini_collection_destructor_asks_the_singleton() {
        let mut e = settings_engine();
        e.register(0x0010_0010, |e, a| {
            // Slot 2 of the singleton: records the object it was given.
            e.mem.set_u32(a[0] + 0x40, a[1]);
            Ret::default()
        });
        e.put_vtable(0x0300_0000, &[0, 0, 0x0010_0010]);
        let instance = e.mem.alloc(0x114);
        e.mem.set_u32(instance, 0x0300_0000);
        e.set_global(SETTINGS_INSTANCE, instance);
        let this = Ptr::<()>::new(e.mem.alloc(0x20));
        e.call_log = Some(vec![]);
        e.call(0x0044_f4f0, &args![this]);
        assert_eq!(e.mem.u32(instance + 0x40), this.addr());
        assert_eq!(e.mem.u32(this.addr()), INI_COLLECTION_VTABLE);
        assert_eq!(
            calls_to(&e, INI_COLLECTION_BASE_DESTRUCT),
            vec![vec![this.addr()]]
        );
        // The scalar deleting form frees only with flag 1.
        let back = e.call(0x0044_f4c0, &args![this, 0u32]).ptr::<()>();
        assert_eq!(back, this);
        assert!(calls_to(&e, MEMORY_FREE).is_empty());
        e.call(0x0044_f4c0, &args![this, 1u32]);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![this.addr()]]);
    }

    #[test]
    fn settings_destructors() {
        let mut e = settings_engine();
        let this = Ptr::<()>::new(e.mem.alloc(0x114));
        e.call_log = Some(vec![]);
        e.call(0x0044_f650, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), SETTINGS_VTABLE);
        assert_eq!(
            call_order(&e),
            vec![
                0x0044_f650,
                SETTINGS_LIST_DESTRUCT_FIRST,
                SETTINGS_LIST_DESTRUCT_SECOND,
                SETTINGS_BASE_DESTRUCT
            ]
        );
        assert_eq!(
            calls_to(&e, SETTINGS_LIST_DESTRUCT_FIRST),
            vec![vec![this.addr() + 0x10c]]
        );
        assert_eq!(
            calls_to(&e, SETTINGS_BASE_DESTRUCT),
            vec![vec![this.addr()]]
        );

        // Scalar deleting destructors: the object's own, then the free.
        e.call_log = Some(vec![]);
        let back = e.call(0x0044_f6d0, &args![this, 0u32]).ptr::<()>();
        assert_eq!(back, this);
        assert!(calls_to(&e, MEMORY_FREE).is_empty());
        e.call(0x0044_f6d0, &args![this, 1u32]);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![this.addr()]]);

        e.call_log = Some(vec![]);
        e.call(0x0044_f620, &args![this, 0u32]);
        assert_eq!(
            calls_to(&e, SETTINGS_OBJECT_DESTRUCT),
            vec![vec![this.addr()]]
        );
        assert!(calls_to(&e, MEMORY_FREE).is_empty());
        e.call(0x0044_f620, &args![this, 1u32]);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![this.addr()]]);
    }

    /// Doubles for the settings list: the node is its own item slot holder;
    /// a "list" of keys given as words at the node address + 0 (the item) and
    /// +4 (the next node). The key getter returns the setting's first word.
    fn setting_list_engine(existing_keys: &[u32]) -> (Engine, Ptr) {
        let mut e = settings_engine();
        e.register(SETTING_LIST_IS_EMPTY, |e, a| {
            // Empty when the list head's marker word at +0x100 is zero.
            u32::from(e.mem.u32(a[0] - 0x10c + 0x100) == 0).into_ret()
        });
        e.register(LIST_NODE_ITEM, |_, a| a[0].into_ret());
        e.register(LIST_NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(SETTING_KEY, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(STRING_COMPARE, |_, a| a[0].wrapping_sub(a[1]).into_ret());
        e.register(SETTING_WARNING, |_, _| Ret::default());
        e.register(SETTING_LIST_ADD, |e, a| {
            let setting = e.mem.u32(a[1]);
            e.mem.set_u32(a[0] - 0x10c + 0xf0, setting);
            Ret::default()
        });
        e.register(SETTING_LIST_REMOVE, |e, a| {
            let setting = e.mem.u32(a[1]);
            e.mem.set_u32(a[0] - 0x10c + 0xf4, setting);
            Ret::default()
        });
        let this = Ptr::<()>::new(e.mem.alloc(0x114));
        // Chain of nodes whose item word points at a setting with that key.
        let mut previous = 0;
        for key in existing_keys.iter().rev() {
            let setting = e.mem.alloc(4);
            e.mem.set_u32(setting, *key);
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, setting);
            e.mem.set_u32(node + 4, previous);
            previous = node;
        }
        // The list's own first node is the head at +0x10c; its item slot is
        // the head itself, so make the head hold the chain's first setting.
        if previous != 0 {
            let first_setting = e.mem.u32(previous);
            e.mem.set_u32(this.addr() + 0x10c, first_setting);
            e.mem
                .set_u32(this.addr() + 0x10c + 4, e.mem.u32(previous + 4));
            e.mem.set_u32(this.addr() + 0x100, 1);
        }
        (e, this)
    }

    #[test]
    fn setting_add_appends_a_setting_with_a_new_key() {
        let (mut e, this) = setting_list_engine(&[10, 20]);
        let setting = e.mem.alloc(4);
        e.mem.set_u32(setting, 30);
        e.call_log = Some(vec![]);
        e.call(0x0044_f770, &args![this, setting]);
        assert_eq!(e.mem.u32(this.addr() + 0xf0), setting);
        assert!(calls_to(&e, SETTING_WARNING).is_empty());
        let enter = &calls_to(&e, SCOPE_GUARD_ENTER)[0];
        assert_eq!(&enter[1..], &[9, 1, SETTING_SOURCE, SETTING_ADD_LINE]);
        assert_eq!(*call_order(&e).last().unwrap(), SCOPE_GUARD_LEAVE);
    }

    #[test]
    fn setting_add_reports_a_duplicate_key_and_adds_nothing() {
        let (mut e, this) = setting_list_engine(&[10, 20]);
        let setting = e.mem.alloc(4);
        e.mem.set_u32(setting, 20);
        e.call_log = Some(vec![]);
        e.call(0x0044_f770, &args![this, setting]);
        assert_eq!(e.mem.u32(this.addr() + 0xf0), 0);
        assert!(calls_to(&e, SETTING_LIST_ADD).is_empty());
        assert_eq!(
            calls_to(&e, SETTING_WARNING),
            vec![vec![DUPLICATE_KEY_MESSAGE, 20]]
        );
        assert_eq!(*call_order(&e).last().unwrap(), SCOPE_GUARD_LEAVE);
    }

    #[test]
    fn setting_add_skips_the_search_in_an_empty_list() {
        let (mut e, this) = setting_list_engine(&[]);
        let setting = e.mem.alloc(4);
        e.mem.set_u32(setting, 5);
        e.call_log = Some(vec![]);
        e.call(0x0044_f770, &args![this, setting]);
        assert!(calls_to(&e, SETTING_KEY).is_empty());
        assert_eq!(e.mem.u32(this.addr() + 0xf0), setting);
    }

    #[test]
    fn setting_remove_passes_the_address_of_the_argument() {
        let (mut e, this) = setting_list_engine(&[]);
        e.call_log = Some(vec![]);
        e.call(0x0044_f870, &args![this, 0x4242u32]);
        assert_eq!(e.mem.u32(this.addr() + 0xf4), 0x4242);
        let call = &calls_to(&e, SETTING_LIST_REMOVE)[0];
        assert_eq!(call[0], this.addr() + 0x10c);
    }

    #[test]
    fn get_viewer_strings_visits_every_setting_and_counts_them() {
        let mut e = settings_engine();
        e.register(COLLECTION_FIRST, |_, a| a[0].into_ret());
        e.register(LIST_NODE_ITEM, |_, a| a[0].into_ret());
        e.register(SETTING_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(COLLECTION_NEXT, |e, a| e.mem.u32(a[1] + 4).into_ret());
        // Two nodes: item word at +0 (the setting), next at +4.
        let collection = e.mem.alloc(0x10);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, 0x222);
        e.mem.set_u32(second + 4, 0);
        // The first node is the collection itself.
        e.mem.set_u32(collection, 0x111);
        e.mem.set_u32(collection + 4, second);
        e.call_log = Some(vec![]);
        let count = e.call(0x0044_f890, &args![collection, 0x99u32]).i32();
        assert_eq!(count, 2);
        assert_eq!(
            calls_to(&e, SETTING_GET_VIEWER_STRINGS),
            vec![vec![0x111, 0x99], vec![0x222, 0x99]]
        );
        let enter = &calls_to(&e, SCOPE_GUARD_ENTER)[0];
        assert_eq!(
            &enter[1..],
            &[9, 1, SETTING_SOURCE, SETTING_GET_VIEWER_STRINGS_LINE]
        );
        // An empty collection counts zero.
        e.register(COLLECTION_FIRST, |_, _| Ret::default());
        assert_eq!(e.call(0x0044_f890, &args![collection, 0u32]).i32(), 0);
    }

    #[test]
    fn queued_file_pointer_assignment_moves_the_reference() {
        let mut e = Engine::new();
        e.register(TASK_RELEASE, |_, _| Ret::default());
        e.register(TASK_ADD_REFERENCE, |_, _| Ret::default());
        let slot = Ptr::<()>::new(e.mem.alloc(4));
        e.mem.set_u32(slot.addr(), 0x100);
        e.call_log = Some(vec![]);
        // The same value: nothing happens.
        let back = e
            .call(0x006f_74f0, &args![slot, Ptr::<()>::new(0x100)])
            .ptr::<()>();
        assert_eq!(back, slot);
        assert_eq!(call_order(&e).len(), 1);
        // Another value: the old one is released, the new one referenced.
        e.call(0x006f_74f0, &args![slot, Ptr::<()>::new(0x200)]);
        assert_eq!(e.mem.u32(slot.addr()), 0x200);
        assert_eq!(calls_to(&e, TASK_RELEASE), vec![vec![0x100]]);
        assert_eq!(calls_to(&e, TASK_ADD_REFERENCE), vec![vec![0x200]]);
        // Null: only the release.
        e.call(0x006f_74f0, &args![slot, Ptr::<()>::NULL]);
        assert_eq!(e.mem.u32(slot.addr()), 0);
        assert_eq!(calls_to(&e, TASK_RELEASE).len(), 2);
        assert_eq!(calls_to(&e, TASK_ADD_REFERENCE).len(), 1);
        // From null: only the reference.
        e.call(0x006f_74f0, &args![slot, Ptr::<()>::new(0x300)]);
        assert_eq!(calls_to(&e, TASK_RELEASE).len(), 2);
        assert_eq!(calls_to(&e, TASK_ADD_REFERENCE).len(), 2);
    }

    #[test]
    fn queued_file_load_file_index_reads_the_byte_at_0x34() {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(0x40));
        e.mem.set_u8(this.addr() + 0x34, 0x9c);
        assert_eq!(e.call(0x0071_2380, &args![this]).u32() & 0xff, 0x9c);
    }

    #[test]
    fn semaphore_signal_releases_one_only_when_enabled() {
        let mut e = Engine::new();
        ignore(&mut e, &[RELEASE_SEMAPHORE, IMPORT_INTERLOCKED_INCREMENT]);
        let this = Ptr::<()>::new(e.mem.alloc(0x50));
        e.mem.set_u32(this.addr() + 0x34, 0xbeef);
        e.call_log = Some(vec![]);
        e.call(0x00ae_78e0, &args![this]);
        assert_eq!(call_order(&e).len(), 1);
        e.mem.set_u8(this.addr() + 0x48, 1);
        e.call(0x00ae_78e0, &args![this]);
        assert_eq!(calls_to(&e, RELEASE_SEMAPHORE), vec![vec![0xbeef, 1, 0]]);
        assert_eq!(
            calls_to(&e, IMPORT_INTERLOCKED_INCREMENT),
            vec![vec![this.addr() + 0x30]]
        );
    }
}
