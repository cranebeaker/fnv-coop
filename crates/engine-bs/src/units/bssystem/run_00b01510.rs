//! `BSSystem/run_00b01510` (Xbox PDB source unit), subsystem `BSSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSWin32FileCache`'s vtable (its RTTI name).
const WIN32_FILE_CACHE_VTABLE: u32 = 0x010a_483c;
/// `memcpy` (`00ec44d0`, cdecl).
const MEMCPY: u32 = 0x00ec_44d0;
/// The direct read of a file object (`00afb7a0`, `thiscall`, three words:
/// destination, size, and a third the code passes on).
const READ_DIRECT: u32 = 0x00af_b7a0;
/// The read buffer of the file object: its capacity (`+0x10`), the bytes
/// in it (`+0x14`), the read position in it (`+0x18`) and its storage
/// (`+0x20`).
const BUFFER_CAPACITY: u32 = 0x10;
const BUFFER_FILLED: u32 = 0x14;
const BUFFER_POSITION: u32 = 0x18;
const BUFFER_STORAGE: u32 = 0x20;

fn field(e: &Engine, this: Ptr, offset: u32) -> u32 {
    e.mem.u32(this.addr().wrapping_add(offset))
}

fn set_field(e: &mut Engine, this: Ptr, offset: u32, value: u32) {
    e.mem.set_u32(this.addr().wrapping_add(offset), value);
}

// Translated from 00b01800 (decompiled, FalloutNV.exe 1.4.0.525)
/// A buffered read of `size` bytes into `destination`: served from the
/// buffer when it holds enough. Otherwise the rest of the buffer is copied
/// first, the buffer emptied, and either the remainder read directly
/// (`00afb7a0`; more than the capacity) or the buffer refilled with up to
/// its capacity and the remainder (limited to what came in) copied from
/// it. Returns the bytes delivered.
pub fn fn_00b01800(e: &mut Engine, this: Ptr, destination: u32, size: u32) -> u32 {
    let mut destination = destination;
    let mut size = size;
    let mut done = 0u32;
    let available = field(e, this, BUFFER_FILLED).wrapping_sub(field(e, this, BUFFER_POSITION));
    if size > available {
        if available > 0 {
            let source =
                field(e, this, BUFFER_STORAGE).wrapping_add(field(e, this, BUFFER_POSITION));
            e.call(MEMCPY, &args![destination, source, available]);
            destination = destination.wrapping_add(available);
            size = size.wrapping_sub(available);
            done = available;
        }
        set_field(e, this, BUFFER_FILLED, 0);
        set_field(e, this, BUFFER_POSITION, 0);
        if size > field(e, this, BUFFER_CAPACITY) {
            let read = e
                .call(READ_DIRECT, &args![this, destination, size, done])
                .u32();
            return read.wrapping_add(done);
        }
        let storage = field(e, this, BUFFER_STORAGE);
        let capacity = field(e, this, BUFFER_CAPACITY);
        let filled = e
            .call(READ_DIRECT, &args![this, storage, capacity, done])
            .u32();
        set_field(e, this, BUFFER_FILLED, filled);
        if field(e, this, BUFFER_FILLED) < size {
            size = field(e, this, BUFFER_FILLED);
        }
    }
    let source = field(e, this, BUFFER_STORAGE).wrapping_add(field(e, this, BUFFER_POSITION));
    e.call(MEMCPY, &args![destination, source, size]);
    let position = field(e, this, BUFFER_POSITION).wrapping_add(size);
    set_field(e, this, BUFFER_POSITION, position);
    done.wrapping_add(size)
}

// Translated from 00b01bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// 1 when bit 1 of `flags` is clear, else 0 (the second word and `this`
/// are not read).
pub fn fn_00b01bc0(_e: &mut Engine, _this: Ptr, flags: u32, _unused_2: u32) -> u32 {
    u32::from(flags & 2 == 0)
}

// Translated from 00b01be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// 1 when bit 2 of `flags` is clear, else 0 (the second word and `this`
/// are not read).
pub fn fn_00b01be0(_e: &mut Engine, _this: Ptr, flags: u32, _unused_2: u32) -> u32 {
    u32::from(flags & 4 == 0)
}

// Translated from 00b01c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache`'s constructor: the `BSFileCache` base (`00b00f20`),
/// then its vtable. Returns `this`.
pub fn fn_00b01c00(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x00b0_0f20, &args![this]);
    e.mem.set_u32(this.addr(), WIN32_FILE_CACHE_VTABLE);
    this
}

// Translated from 00b01c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache`'s destructor: its vtable, then the `BSFileCache`
/// destructor body (`00b01020`).
pub fn fn_00b01c20(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), WIN32_FILE_CACHE_VTABLE);
    e.call(0x00b0_1020, &args![this]);
}

/// KERNEL32 import slots (`stdcall`).
const INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
const CREATE_SEMAPHORE_A: u32 = 0x00fd_f0b4;
const RELEASE_SEMAPHORE: u32 = 0x00fd_f1b4;
const CREATE_THREAD: u32 = 0x00fd_f124;
const RESUME_THREAD: u32 = 0x00fd_f158;
const EXIT_THREAD: u32 = 0x00fd_f148;
const SLEEP: u32 = 0x00fd_f104;
const WAIT_FOR_SINGLE_OBJECT: u32 = 0x00fd_f1c8;
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const INFINITE: u32 = 0xffff_ffff;
const CREATE_SUSPENDED: u32 = 4;

/// The two vtables: the base part's (`010a48ac`, `00b02720`) and the manager's (`010a4880`).
const BASE_VTABLE: u32 = 0x010a_48ac;
const MANAGER_VTABLE: u32 = 0x010a_4880;
/// The worker and starter thread starts.
const WORKER_START: u32 = 0x00b0_23f0;
const STARTER_START: u32 = 0x00b0_23b0;
/// `BSTaskletManager`'s two hook lists the worker runs around its loop (`00b00c90`, `00b00cf0`).
const WORKER_ENTER_HOOKS: u32 = 0x00b0_0c90;
const WORKER_LEAVE_HOOKS: u32 = 0x00b0_0cf0;
/// The manager's fields.
const STARTER: u32 = 0x14;
const THREAD_LIMIT: u32 = 0x18;
const THREAD_COUNT: u32 = 0x1c;
const THREAD_HANDLES: u32 = 0x20;
const THREAD_IDS: u32 = 0x38;
const STOPPING: u32 = 0x50;
const LOCK: u32 = 0x54;
const LISTS: u32 = 0x6c;
const TAILS: u32 = 0x16c;
const WORK: u32 = 0x26c;

/// A counted semaphore record as `00b02720` and `00b01d50` lay it out: the count at `+0`, the
/// handle at `+4`, the maximum at `+8`; `CreateSemaphoreA(0, count, maximum, 0)`.
fn create_counted_semaphore(e: &mut Engine, record: u32) {
    let maximum = e.mem.u32(record + 8);
    let count = e.mem.u32(record);
    let handle = e
        .call(CREATE_SEMAPHORE_A, &args![0u32, count, maximum, 0u32])
        .u32();
    e.mem.set_u32(record + 4, handle);
}

/// `ReleaseSemaphore(handle, 1, 0)` on a counted record, then `InterlockedIncrement` on its count.
fn release_counted(e: &mut Engine, record: u32) {
    let handle = e.mem.u32(record + 4);
    e.call(RELEASE_SEMAPHORE, &args![handle, 1u32, 0u32]);
    e.call(INTERLOCKED_INCREMENT, &args![record]);
}

/// `WaitForSingleObject(handle, INFINITE)` on a counted record, then `InterlockedDecrement`.
fn wait_counted(e: &mut Engine, record: u32) {
    let handle = e.mem.u32(record + 4);
    e.call(WAIT_FOR_SINGLE_OBJECT, &args![handle, INFINITE]);
    e.call(INTERLOCKED_DECREMENT, &args![record]);
}

// Translated from 00b02720 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words, plain `RET`; the tasklet manager's base
/// constructor): the vtable `010a48ac`, the bytes `+4` and `+5` cleared, and the semaphore record
/// at `+8` (count 0, maximum 1, `CreateSemaphoreA`). Answers `this`.
pub fn fn_00b02720(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, BASE_VTABLE);
    e.mem.set_u8(base + 4, 0);
    e.mem.set_u8(base + 5, 0);
    let record = base + 8;
    e.mem.set_u32(record, 0);
    e.mem.set_u32(record + 8, 1);
    create_counted_semaphore(e, record);
    this
}

// Translated from 00b01d50 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one stack word, `RET 4`; the tasklet manager's
/// constructor, `QInstance` asks for 6): the base (`00b02720`), the vtable `010a4880`, no starter
/// (`+0x14`), the thread limit `+0x18` = `threads` capped at 6, no threads yet (`+0x1c`), the
/// stopping byte `+0x50` cleared, the critical section at `+0x54`, the work semaphore record at
/// `+0x26c` (count 0, maximum 0x7FFFFFFF), the 64 list heads at `+0x6c` and the six handle and
/// id words (`+0x20`, `+0x38`) cleared. Then up to the limit, one suspended worker thread each
/// (`CreateThread(0, 0, 00b023f0, this, CREATE_SUSPENDED, &id)`), stopping at the first that
/// fails, counted in `+0x1c`; the byte `+4` says whether any was made, and then the starter
/// thread (`00b023b0`, running) goes to `+0x14`. Answers `this`.
pub fn fn_00b01d50(e: &mut Engine, this: Ptr, threads: u32) -> Ptr {
    let base = this.addr();
    fn_00b02720(e, this);
    e.mem.set_u32(base, MANAGER_VTABLE);
    e.mem.set_u32(base + STARTER, 0);
    let limit = if threads > 6 { 6 } else { threads };
    e.mem.set_u32(base + THREAD_LIMIT, limit);
    e.mem.set_u32(base + THREAD_COUNT, 0);
    e.mem.set_u8(base + STOPPING, 0);
    e.call(INITIALIZE_CRITICAL_SECTION, &args![base + LOCK]);
    let work = base + WORK;
    e.mem.set_u32(work, 0);
    e.mem.set_u32(work + 8, 0x7fff_ffff);
    create_counted_semaphore(e, work);
    for i in 0..0x40u32 {
        e.mem.set_u32(base + LISTS + 4 * i, 0);
    }
    for i in 0..6u32 {
        e.mem.set_u32(base + THREAD_HANDLES + 4 * i, 0);
        e.mem.set_u32(base + THREAD_IDS + 4 * i, 0);
    }
    let mut i = 0u32;
    while i < e.mem.u32(base + THREAD_LIMIT) {
        let id_out = base + THREAD_IDS + 4 * i;
        let handle = e
            .call(
                CREATE_THREAD,
                &args![0u32, 0u32, WORKER_START, base, CREATE_SUSPENDED, id_out],
            )
            .u32();
        e.mem.set_u32(base + THREAD_HANDLES + 4 * i, handle);
        if e.mem.u32(base + THREAD_HANDLES + 4 * i) == 0 {
            break;
        }
        let count = e.mem.u32(base + THREAD_COUNT);
        e.mem.set_u32(base + THREAD_COUNT, count.wrapping_add(1));
        i += 1;
    }
    let any = e.mem.u32(base + THREAD_COUNT) != 0;
    e.mem.set_u8(base + 4, u8::from(any));
    if e.mem.u8(base + 4) != 0 {
        e.with_stack(4, |e, id| {
            let handle = e
                .call(
                    CREATE_THREAD,
                    &args![0u32, 0u32, STARTER_START, base, 0u32, id],
                )
                .u32();
            e.mem.set_u32(base + STARTER, handle);
        });
    }
    this
}

// Translated from 00b023b0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The starter thread's start (`stdcall`, one word: the manager; it never returns): waits on the
/// base semaphore record at `+8` (`WaitForSingleObject` INFINITE, `InterlockedDecrement`),
/// resumes the workers (`00b02420`) and ends with `ExitThread(0)`.
pub fn fn_00b023b0(e: &mut Engine, manager: u32) -> u32 {
    wait_counted(e, manager + 8);
    fn_00b02420(e, Ptr::new(manager));
    e.call(EXIT_THREAD, &args![0u32]);
    // `ExitThread` does not come back on a game thread.
    0
}

// Translated from 00b023f0 (disassembled, FalloutNV.exe 1.4.0.525)
/// A worker thread's start (`stdcall`, one word: the manager; it never returns): the tasklet
/// manager's enter hooks (`00b00c90`), the work loop (`00b02460`), the leave hooks (`00b00cf0`),
/// then `ExitThread(0)`.
pub fn fn_00b023f0(e: &mut Engine, manager: u32) -> u32 {
    e.call(WORKER_ENTER_HOOKS, &args![manager]);
    fn_00b02460(e, Ptr::new(manager));
    e.call(WORKER_LEAVE_HOOKS, &args![manager]);
    e.call(EXIT_THREAD, &args![0u32]);
    // `ExitThread` does not come back on a game thread.
    0
}

// Translated from 00b02420 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words, plain `RET`): `ResumeThread` on each of
/// the `+0x1c` worker handles at `+0x20`.
pub fn fn_00b02420(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let mut i = 0u32;
    while i < e.mem.u32(base + THREAD_COUNT) {
        let handle = e.mem.u32(base + THREAD_HANDLES + 4 * i);
        e.call(RESUME_THREAD, &args![handle]);
        i += 1;
    }
}

// Translated from 00b02460 (disassembled, FalloutNV.exe 1.4.0.525)
/// A worker's loop (`thiscall`, no stack words, plain `RET` once the stopping byte `+0x50` is
/// set): waits for work (the record at `+0x26c`); unless stopping, takes the first task of the
/// 64 lists under the lock (the list head moves to the task's `+0x14`; its group is the task's
/// `+8`). A task with a group whose byte `+0x2e` is clear is run: when its byte `+0xc` is set,
/// slot `+4` decides (else it runs), then `+0xc` is cleared and slot `+8` runs it. Then, with a
/// group: a task whose byte `+4` is set in a group still open goes back to its list
/// (`00b02130`); otherwise slot `+0xc` finishes it, `+8` and `+0x10` are cleared and its group
/// counts it done (`00b02670`). Each pass ends with `Sleep(0)`.
pub fn fn_00b02460(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    loop {
        if e.mem.u8(base + STOPPING) != 0 {
            return;
        }
        let mut task = 0u32;
        let mut group = 0u32;
        wait_counted(e, base + WORK);
        if e.mem.u8(base + STOPPING) == 0 {
            e.call(ENTER_CRITICAL_SECTION, &args![base + LOCK]);
            let mut slot = base + LISTS;
            let mut i = 0u32;
            while i < 0x40 {
                task = e.mem.u32(slot);
                if task != 0 {
                    let next = e.mem.u32(task + 0x14);
                    e.mem.set_u32(slot, next);
                    group = e.mem.u32(task + 8);
                    break;
                }
                i += 1;
                slot += 4;
            }
            e.call(LEAVE_CRITICAL_SECTION, &args![base + LOCK]);
            // The exe tests the task twice (`00b02524`, `00b02530`).
            let tested = [task != 0, group != 0, task != 0];
            if tested.iter().all(|&t| t) && e.mem.u8(group + 0x2e) == 0 {
                let go = if e.mem.u8(task + 0xc) == 0 {
                    true
                } else {
                    e.vcall(task, 4, &args![]).u8() != 0
                };
                if go {
                    e.mem.set_u8(task + 0xc, 0);
                    e.vcall(task, 8, &args![]);
                }
            }
            if task != 0 && group != 0 {
                if e.mem.u8(task + 4) != 0 && e.mem.u8(group + 0x2e) == 0 {
                    fn_00b02130(e, this, task);
                } else {
                    e.vcall(task, 0xc, &args![]);
                    e.mem.set_u32(task + 8, 0);
                    e.mem.set_u32(task + 0x10, 0);
                    fn_00b02670(e, Ptr::new(group));
                }
            }
        }
        e.call(SLEEP, &args![0u32]);
    }
}

// Translated from 00b02130 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one stack word, `RET 4`: the task): a task without a
/// group (`+8`) answers 0. Otherwise, under the lock, it goes to the end of the list its group's
/// priority picks (`+0x30`, capped at 0x3F; an empty list's tail is its head), its `+0x14`
/// cleared and its byte `+4` cleared; then the work semaphore is released once. Answers 1.
pub fn fn_00b02130(e: &mut Engine, this: Ptr, task: u32) -> u8 {
    let base = this.addr();
    if e.mem.u32(task + 8) == 0 {
        return 0;
    }
    e.call(ENTER_CRITICAL_SECTION, &args![base + LOCK]);
    let group = e.mem.u32(task + 8);
    let mut list = e.mem.u32(group + 0x30);
    if list >= 0x40 {
        list = 0x3f;
    }
    if e.mem.u32(base + LISTS + 4 * list) == 0 {
        e.mem
            .set_u32(base + TAILS + 4 * list, base + LISTS + 4 * list);
    }
    let tail = e.mem.u32(base + TAILS + 4 * list);
    e.mem.set_u32(tail, task);
    e.mem.set_u32(base + TAILS + 4 * list, task + 0x14);
    e.mem.set_u32(task + 0x14, 0);
    e.mem.set_u8(task + 4, 0);
    e.call(LEAVE_CRITICAL_SECTION, &args![base + LOCK]);
    release_counted(e, base + WORK);
    1
}

// Translated from 00b02670 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words, plain `RET`; on a task group): under the
/// group's critical section (`+0x14`) counts one more task done (`+0x38`); when the group isn't
/// held open (byte `+0x2d`) and that reaches its task count (`+0x34`), its semaphore record at
/// `+8` is released once.
pub fn fn_00b02670(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(ENTER_CRITICAL_SECTION, &args![base + 0x14]);
    let done = e.mem.u32(base + 0x38).wrapping_add(1);
    e.mem.set_u32(base + 0x38, done);
    let finished = e.mem.u8(base + 0x2d) == 0 && e.mem.u32(base + 0x38) == e.mem.u32(base + 0x34);
    e.call(LEAVE_CRITICAL_SECTION, &args![base + 0x14]);
    if finished {
        release_counted(e, base + 8);
    }
}

/// KERNEL32 import slots the stop uses (`stdcall`).
const WAIT_FOR_MULTIPLE_OBJECTS: u32 = 0x00fd_f108;
const CLOSE_HANDLE: u32 = 0x00fd_f07c;

// Translated from 00b01f50 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words, plain `RET`; slot +8 of the manager's
/// vtable `010a4880`, at `010a4888`): sets the stopping byte `+0x50`; when the byte `+5` is set,
/// releases the work semaphore record at `+0x26c` once per worker (`ReleaseSemaphore(handle, 1,
/// 0)`, `InterlockedIncrement` on its count; the count `+0x1c` re-read each pass) and waits for
/// all of them (`WaitForMultipleObjects(+0x1c, +0x20, TRUE, INFINITE)`). Then `CloseHandle` on
/// each worker handle and on the starter's (`+0x14`).
pub fn fn_00b01f50(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u8(base + STOPPING, 1);
    if e.mem.u8(base + 5) != 0 {
        let mut i = 0u32;
        while i < e.mem.u32(base + THREAD_COUNT) {
            release_counted(e, base + WORK);
            i += 1;
        }
        let count = e.mem.u32(base + THREAD_COUNT);
        e.call(
            WAIT_FOR_MULTIPLE_OBJECTS,
            &args![count, base + THREAD_HANDLES, 1u32, INFINITE],
        );
    }
    let mut i = 0u32;
    while i < e.mem.u32(base + THREAD_COUNT) {
        let handle = e.mem.u32(base + THREAD_HANDLES + 4 * i);
        e.call(CLOSE_HANDLE, &args![handle]);
        i += 1;
    }
    let starter = e.mem.u32(base + STARTER);
    e.call(CLOSE_HANDLE, &args![starter]);
}

/// The memory manager (`011f6238`) and its deallocate (`00aa4060`, thiscall: block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;

// Translated from 00b01c40 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache` vtable +0x04 (`thiscall`, one word, `RET 4`): stores the byte `value` at
/// `+0x45`.
pub fn fn_00b01c40(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr().wrapping_add(0x45), value);
}

// Translated from 00b01c60 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache` vtable +0x08 (`thiscall`, no stack words, plain `RET`): does nothing.
pub fn fn_00b01c60(_e: &mut Engine, _this: Ptr) {}

// Translated from 00b01c70 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache` vtable +0x0C (`thiscall`, two words, `RET 8`): answers 0 (`XOR EAX,EAX`);
/// neither word is read.
pub fn fn_00b01c70(_e: &mut Engine, _this: Ptr, _unused_1: u32, _unused_2: u32) -> u32 {
    0
}

// Translated from 00b01c80 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache` vtable +0x10 (`thiscall`, one word, `RET 4`): does nothing.
pub fn fn_00b01c80(_e: &mut Engine, _this: Ptr, _unused_1: u32) {}

// Translated from 00b01c90 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache` vtable +0x14 (`thiscall`, one word, `RET 4`): answers 0 (`XOR EAX,EAX`);
/// the word (the path `00b011b0` passes) is not read.
pub fn fn_00b01c90(_e: &mut Engine, _this: Ptr, _unused_1: u32) -> u32 {
    0
}

// Translated from 00b01ca0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache` vtable +0x18 (`thiscall`, two words, `RET 8`): does nothing.
pub fn fn_00b01ca0(_e: &mut Engine, _this: Ptr, _unused_1: u32, _unused_2: u32) {}

// Translated from 00b01cb0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache` vtable +0x1C (`thiscall`, one word, `RET 4`): answers false (`XOR AL,AL`).
pub fn fn_00b01cb0(_e: &mut Engine, _this: Ptr, _unused_1: u32) -> bool {
    false
}

// Translated from 00b01cc0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache` vtable +0x20 (`thiscall`, one word, `RET 4`): does nothing.
pub fn fn_00b01cc0(_e: &mut Engine, _this: Ptr, _unused_1: u32) {}

// Translated from 00b01cd0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache` vtable +0x24 (`thiscall`, no stack words, plain `RET`): does nothing.
pub fn fn_00b01cd0(_e: &mut Engine, _this: Ptr) {}

// Translated from 00b01ce0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache` vtable +0x28 (`thiscall`, one word, `RET 4`): does nothing.
pub fn fn_00b01ce0(_e: &mut Engine, _this: Ptr, _unused_1: u32) {}

// Translated from 00b01cf0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache` vtable +0x2C (`thiscall`, four words, `RET 0x10`): does nothing.
pub fn fn_00b01cf0(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
) {
}

// Translated from 00b01d00 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache` vtable +0x30 (`thiscall`, no stack words, plain `RET`): does nothing.
pub fn fn_00b01d00(_e: &mut Engine, _this: Ptr) {}

// Translated from 00b01d10 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache` vtable +0x34 (`thiscall`, one word, `RET 4`): answers false (`XOR AL,AL`).
pub fn fn_00b01d10(_e: &mut Engine, _this: Ptr, _unused_1: u32) -> bool {
    false
}

// Translated from 00b01d20 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache` vtable +0x00 (`thiscall`, one word, `RET 4`): the scalar deleting
/// destructor. Runs the destructor (`fn_00b01c20`), then with bit 0 of `flags` frees `this`
/// through the memory manager (`00aa4060` on `011f6238`). Answers `this`.
pub fn fn_00b01d20(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b01c20(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
/// `00b01930`'s calls: KERNEL32 `FindFirstFileA`, `FindNextFileA` and `FindClose` (import
/// slots), `strlen` (`00ec6130`), `strrchr` (`00ec6e30`), `strcpy_s` (`00ec65a6`), the memory
/// manager's allocation (`00aa3e40`, `thiscall` on `011f6238`, one word) and the list's add
/// (`005ae3d0`, `thiscall`, one word: the address of a word holding the new name).
const FIND_FIRST_FILE_A: u32 = 0x00fd_f070;
const FIND_NEXT_FILE_A: u32 = 0x00fd_f068;
const FIND_CLOSE: u32 = 0x00fd_f064;
const STRLEN: u32 = 0x00ec_6130;
const STRRCHR: u32 = 0x00ec_6e30;
const STRCPY_S: u32 = 0x00ec_65a6;
const ALLOCATE: u32 = 0x00aa_3e40;
const LIST_ADD: u32 = 0x005a_e3d0;
/// `00b01930`'s frame (`SUB ESP,0x17c`), as offsets below `EBP`: the `WIN32_FIND_DATAA`
/// (`EBP-0x148`, its `cFileName` at `EBP-0x11c`) and the word holding the new name
/// (`EBP-0x15c`) whose address the list's add gets.
const LOOSE_FRAME: u32 = 0x17c;
const LOOSE_FIND_DATA: u32 = 0x148;
const LOOSE_FILE_NAME: u32 = 0x11c;
const LOOSE_NAME_WORD: u32 = 0x15c;

// Translated from 00b01930 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b01930 - the /GS stack cookie check (00ec408c) is not translated.
/// The loose files of a pattern added to a file list (no label; cdecl: the pattern, the path
/// the names are made from, the list). `FindFirstFileA(pattern)`; nothing more when it answers
/// `INVALID_HANDLE_VALUE`. The directory part of the path is its length less what follows its
/// last `'\\'` (`strrchr`; the whole path without one). For each file found (the first, then
/// while `FindNextFileA` answers nonzero): a block of that length plus the file name's plus 1
/// is allocated (`00aa3e40`), the directory part copied into it (`memcpy`) and ended with a
/// zero; when that text has a `'\\'` (`strrchr`), the file name is copied after its last one
/// (`strcpy_s` with the room left) and the name added to the list (`005ae3d0`). Then
/// `FindClose`.
pub fn add_loose_files(e: &mut Engine, pattern: u32, path: u32, list: u32) {
    e.with_stack(LOOSE_FRAME, |e, frame| {
        let at = |offset: u32| frame.addr() + LOOSE_FRAME - offset;
        let find_data = at(LOOSE_FIND_DATA);
        let file_name = at(LOOSE_FILE_NAME);
        let name_word = at(LOOSE_NAME_WORD);
        let handle = e.call(FIND_FIRST_FILE_A, &args![pattern, find_data]).u32();
        if handle == 0xffff_ffff {
            return;
        }
        let mut length = e.call(STRLEN, &args![path]).u32();
        let last = e.call(STRRCHR, &args![path, 0x5c_u32]).u32();
        if last != 0 {
            let tail = e.call(STRLEN, &args![last.wrapping_add(1)]).u32();
            length = length.wrapping_sub(tail);
        }
        loop {
            let name_length = e.call(STRLEN, &args![file_name]).u32();
            let total = length.wrapping_add(name_length).wrapping_add(1);
            let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, total]).u32();
            e.mem.set_u32(name_word, block);
            e.call(MEMCPY, &args![block, path, length]);
            e.mem.set_u8(block.wrapping_add(length), 0);
            let slash = e.call(STRRCHR, &args![block, 0x5c_u32]).u32();
            if slash != 0 {
                let after = slash.wrapping_add(1);
                let room = total.wrapping_sub(after.wrapping_sub(block));
                e.call(STRCPY_S, &args![after, room, file_name]);
                e.call(LIST_ADD, &args![list, name_word]);
            }
            if e.call(FIND_NEXT_FILE_A, &args![handle, find_data]).u32() == 0 {
                break;
            }
        }
        e.call(FIND_CLOSE, &args![handle]);
    })
}

/// More KERNEL32 import slots (`stdcall`): `DeleteCriticalSection`, `CompareFileTime` and
/// `InterlockedCompareExchange`; the C runtime's `strcat_s` (`00ec6bd2`, cdecl); `"*.*"`
/// (`010a4830`) and `"\"` (`010a4834`).
const DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
const COMPARE_FILE_TIME: u32 = 0x00fd_f06c;
const INTERLOCKED_COMPARE_EXCHANGE: u32 = 0x00fd_f1bc;
const STRCAT_S: u32 = 0x00ec_6bd2;
const ALL_FILES: u32 = 0x010a_4830;
const BACKSLASH: u32 = 0x010a_4834;
/// The callees of [`fn_00b01590`]: `00afd390` (`thiscall` on an 8-byte local, two words,
/// `RET 8`), `00af9c50` and `00afac90` (`thiscall`, three words, `RET 0xc`; answer `AL`).
const CALLEE_00AFD390: u32 = 0x00af_d390;
const CALLEE_00AF9C50: u32 = 0x00af_9c50;
const CALLEE_00AFAC90: u32 = 0x00af_ac90;
/// A `WIN32_FIND_DATAA`'s fields: the attributes (+0), the last write time (+0x14) and the
/// file name (+0x2c); the directory attribute bit.
const FIND_ATTRIBUTES: u32 = 0;
const FIND_LAST_WRITE: u32 = 0x14;
const FIND_NAME: u32 = 0x2c;
const DIRECTORY: u32 = 0x10;
const NO_HANDLE: u32 = 0xffff_ffff;

// Translated from 00b01510 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b01510 - the /GS security cookie check (00ec408c) is not translated.
/// Unnamed in the engine map (`thiscall`, one word it does not read, `RET 4`): whether the
/// object's file exists: its own vtable slot +0x18 gives the name, `FindFirstFileA` on it
/// (into a find data on the stack); a found file's search is closed (`FindClose`). Answers 1
/// when found, else 0 (`AL`).
pub fn fn_00b01510(e: &mut Engine, this: u32, _unused: u32) -> u8 {
    e.with_stack(0x140, |e, find| {
        let name = e.vcall(this, 0x18, &args![]).u32();
        let handle = e.call(FIND_FIRST_FILE_A, &args![name, find.addr()]).u32();
        if handle == NO_HANDLE {
            0
        } else {
            e.call(FIND_CLOSE, &args![handle]);
            1
        }
    })
}

// Translated from 00b01590 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b01590 - the /GS security cookie check (00ec408c) is not translated.
/// A walk over the files written after a time (unnamed in the engine map; `thiscall`, three
/// words: a root, a folder under it and a `FILETIME` address, `RET 0xc`), recursive.
/// `root + folder + "*.*"` (`strcpy_s`, `strcat_s`, 0x104) is searched (`FindFirstFileA`);
/// nothing found answers 0. Otherwise the folder is copied (0x104) and, when its length's low
/// byte is above 1, its last character cut; `00afd390` runs on an 8-byte local with the copy
/// and 2, then `00af9c50` on `this` with that local, a local word and 0. Then for each entry:
/// a directory whose name does not start with `'.'` is walked (`folder + name + "\"` as the
/// new folder) and its count added; any other entry, when `00af9c50` answered nonzero and the
/// entry was written after the time (`CompareFileTime` above 0), gets `00afd390` on a second
/// 8-byte local with its name and 0, then `00afac90` on `this` with the local word's value,
/// that local and 0, and is counted when that answers nonzero. `FindNextFileA` moves on;
/// `FindClose` ends. Answers the count. The locals keep the exe's frame offsets (one
/// 0x380-byte frame).
pub fn fn_00b01590(e: &mut Engine, this: u32, root: u32, folder: u32, time: u32) -> u32 {
    e.with_stack(0x380, |e, frame| {
        let at = |offset: u32| frame.addr().wrapping_add(0x380 - offset);
        let path = at(0x110);
        let find = at(0x370);
        let copy = at(0x218);
        let folder_name = at(0x224);
        let looked_up = at(0x21c);
        let file_name = at(0x37c);
        let mut count: u32 = 0;
        e.call(STRCPY_S, &args![path, 0x104u32, root]);
        e.call(STRCAT_S, &args![path, 0x104u32, folder]);
        e.call(STRCAT_S, &args![path, 0x104u32, ALL_FILES]);
        let handle = e.call(FIND_FIRST_FILE_A, &args![path, find]).u32();
        if handle == NO_HANDLE {
            return 0;
        }
        let mut more = true;
        e.call(STRCPY_S, &args![copy, 0x104u32, folder]);
        let length = e.call(STRLEN, &args![copy]).u32() as u8;
        if i32::from(length) > 1 {
            e.mem.set_u8(copy + u32::from(length) - 1, 0);
        }
        e.call(CALLEE_00AFD390, &args![folder_name, copy, 2u32]);
        let found = e
            .call(CALLEE_00AF9C50, &args![this, folder_name, looked_up, 0u32])
            .u8()
            != 0;
        while more {
            let attributes = e.mem.u32(find + FIND_ATTRIBUTES);
            if attributes & DIRECTORY != 0 && e.mem.i8(find + FIND_NAME) != 0x2e {
                e.call(STRCPY_S, &args![path, 0x104u32, folder]);
                e.call(STRCAT_S, &args![path, 0x104u32, find + FIND_NAME]);
                e.call(STRCAT_S, &args![path, 0x104u32, BACKSLASH]);
                let inner = fn_00b01590(e, this, root, path, time);
                count = inner.wrapping_add(count);
            } else if found
                && e.call(COMPARE_FILE_TIME, &args![find + FIND_LAST_WRITE, time])
                    .i32()
                    > 0
            {
                e.call(CALLEE_00AFD390, &args![file_name, find + FIND_NAME, 0u32]);
                let word = e.mem.u32(looked_up);
                if e.call(CALLEE_00AFAC90, &args![this, word, file_name, 0u32])
                    .u8()
                    != 0
                {
                    count = count.wrapping_add(1);
                }
            }
            more = e.call(FIND_NEXT_FILE_A, &args![handle, find]).u32() != 0;
        }
        e.call(FIND_CLOSE, &args![handle]);
        count
    })
}

// Translated from 00b01af0 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b01af0 - the /GS security cookie check (00ec408c) is not translated.
/// Unnamed in the engine map (`cdecl`, one word: a path): whether the path exists
/// (`FindFirstFileA` into a find data on the stack; a found file's search closed). Answers 1
/// or 0 (`AL`).
pub fn fn_00b01af0(e: &mut Engine, path: u32) -> u8 {
    e.with_stack(0x140, |e, find| {
        let handle = e.call(FIND_FIRST_FILE_A, &args![path, find.addr()]).u32();
        if handle == NO_HANDLE {
            0
        } else {
            e.call(FIND_CLOSE, &args![handle]);
            1
        }
    })
}

// Translated from 00b01b60 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b01b60 - the /GS security cookie check (00ec408c) is not translated.
/// Unnamed in the engine map (`cdecl`, two words: a path and a `FILETIME` out): for a path that
/// exists (`FindFirstFileA`), its last write time (both words) goes to the out and the search
/// is closed; answers 1, else 0 (`AL`) with the out untouched.
pub fn fn_00b01b60(e: &mut Engine, path: u32, time: u32) -> u8 {
    e.with_stack(0x140, |e, find| {
        let find = find.addr();
        let handle = e.call(FIND_FIRST_FILE_A, &args![path, find]).u32();
        if handle == NO_HANDLE {
            return 0;
        }
        let low = e.mem.u32(find + FIND_LAST_WRITE);
        e.mem.set_u32(time, low);
        let high = e.mem.u32(find + FIND_LAST_WRITE + 4);
        e.mem.set_u32(time.wrapping_add(4), high);
        e.call(FIND_CLOSE, &args![handle]);
        1
    })
}

// Translated from 00b01f00 (disassembled, FalloutNV.exe 1.4.0.525)
/// The tasklet manager's destructor body (unnamed in the engine map; `thiscall`, plain
/// `RET`): the vtable `010a4880`, the handle at +0x270 closed (`CloseHandle`), the critical
/// section at +0x54 deleted; then the base vtable `010a48ac` and the handle at +0xc closed.
pub fn fn_00b01f00(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, MANAGER_VTABLE);
    let work = e.mem.u32(this.wrapping_add(WORK + 4));
    e.call(CLOSE_HANDLE, &args![work]);
    e.call(DELETE_CRITICAL_SECTION, &args![this.wrapping_add(LOCK)]);
    e.mem.set_u32(this, BASE_VTABLE);
    let handle = e.mem.u32(this.wrapping_add(0xc));
    e.call(CLOSE_HANDLE, &args![handle]);
}

// Translated from 00b02220 (disassembled, FalloutNV.exe 1.4.0.525)
/// A task's queueing (unnamed in the engine map; `thiscall` on the manager, three words: the
/// group, the task and a flag whose low byte counts, `RET 0xc`). Under the group's critical
/// section (+0x14): the task's word at +0x10 is claimed (`InterlockedCompareExchange(+0x10,
/// 1, 0)`); a task already claimed answers 0. Otherwise, when the group runs (byte +0x2c),
/// is open (byte +0x2d) and the task has no group (+8), the task takes the group and a clear
/// byte +4, a set flag closes the group (+0x2d cleared), the task is queued
/// ([`fn_00b02130`], whose answer is answered) and the group's task count (+0x34) goes up;
/// else the claim is dropped (+0x10 cleared) and 0 answered.
pub fn fn_00b02220(e: &mut Engine, this: u32, group: u32, task: u32, flag: u32) -> u8 {
    e.call(ENTER_CRITICAL_SECTION, &args![group.wrapping_add(0x14)]);
    let previous = e
        .call(
            INTERLOCKED_COMPARE_EXCHANGE,
            &args![task.wrapping_add(0x10), 1u32, 0u32],
        )
        .u32();
    let answer = if previous == 0 {
        let ready = e.mem.u8(group.wrapping_add(0x2c)) != 0
            && e.mem.u8(group.wrapping_add(0x2d)) != 0
            && e.mem.u32(task.wrapping_add(8)) == 0;
        if ready {
            e.mem.set_u32(task.wrapping_add(8), group);
            e.mem.set_u8(task.wrapping_add(4), 0);
            if flag as u8 != 0 {
                e.mem.set_u8(group.wrapping_add(0x2d), 0);
            }
            let queued = fn_00b02130(e, Ptr::new(this), task);
            let tasks = e.mem.u32(group.wrapping_add(0x34)).wrapping_add(1);
            e.mem.set_u32(group.wrapping_add(0x34), tasks);
            queued
        } else {
            e.mem.set_u32(task.wrapping_add(0x10), 0);
            0
        }
    } else {
        0
    };
    e.call(LEAVE_CRITICAL_SECTION, &args![group.wrapping_add(0x14)]);
    answer
}

// Translated from 00b02310 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `ECX` unused, one word: a group, `RET 4`): under the
/// group's critical section (+0x14), when the group runs (byte +0x2c) it is closed (byte +0x2d
/// cleared) and 1 answered; else 0 (`AL`).
pub fn fn_00b02310(e: &mut Engine, _this: u32, group: u32) -> u8 {
    e.call(ENTER_CRITICAL_SECTION, &args![group.wrapping_add(0x14)]);
    let running = if e.mem.u8(group.wrapping_add(0x2c)) != 0 {
        e.mem.set_u8(group.wrapping_add(0x2d), 0);
        1
    } else {
        0
    };
    e.call(LEAVE_CRITICAL_SECTION, &args![group.wrapping_add(0x14)]);
    running
}

// Translated from 00b02360 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `ECX` unused, one word: an object, `RET 4`): a
/// non-null object is deleted through its vtable slot +0 with 1 (the scalar deleting
/// destructor). The inner null test repeats the outer one, so its null branch cannot run; it is
/// kept as the exe has it.
pub fn fn_00b02360(e: &mut Engine, _this: u32, object: u32) {
    if object != 0 {
        let target = object;
        if target != 0 {
            e.vcall(target, 0x0, &args![1u32]);
        }
    }
}

// Translated from 00b02600 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`): whether the word is one of the
/// six words at +0x38 (the worker thread ids, first match ends the search). Answers 1 or 0
/// (`AL`).
pub fn fn_00b02600(e: &mut Engine, this: u32, value: u32) -> u8 {
    for i in 0..6u32 {
        if e.mem.u32(this.wrapping_add(i * 4 + THREAD_IDS)) == value {
            return 1;
        }
    }
    0
}

// Translated from 00b02640 (disassembled, FalloutNV.exe 1.4.0.525)
/// The tasklet manager's scalar deleting destructor (unnamed in the engine map; `thiscall`,
/// one word: the flags, `RET 4`): [`fn_00b01f00`], then with bit 0 of the flags the block is
/// freed (`00aa4060`). Answers `this`.
pub fn fn_00b02640(e: &mut Engine, this: u32, flags: u32) -> u32 {
    fn_00b01f00(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00b02710 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words it does not read, `RET 8`): does nothing.
pub fn fn_00b02710(_e: &mut Engine, _this: u32, _unused_1: u32, _unused_2: u32) {}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b0_1510, fn_00b01510(u32, u32) -> u8),
        entry!(0x00b0_1590, fn_00b01590(u32, u32, u32, u32) -> u32),
        entry!(0x00b0_1af0, fn_00b01af0(u32) -> u8),
        entry!(0x00b0_1b60, fn_00b01b60(u32, u32) -> u8),
        entry!(0x00b0_1f00, fn_00b01f00(u32)),
        entry!(0x00b0_2220, fn_00b02220(u32, u32, u32, u32) -> u8),
        entry!(0x00b0_2310, fn_00b02310(u32, u32) -> u8),
        entry!(0x00b0_2360, fn_00b02360(u32, u32)),
        entry!(0x00b0_2600, fn_00b02600(u32, u32) -> u8),
        entry!(0x00b0_2640, fn_00b02640(u32, u32) -> u32),
        entry!(0x00b0_2710, fn_00b02710(u32, u32, u32)),
        entry!(0x00b0_1930, add_loose_files(u32, u32, u32)),
        entry!(0x00b0_1800, fn_00b01800(Ptr, u32, u32) -> u32),
        entry!(0x00b0_1bc0, fn_00b01bc0(Ptr, u32, u32) -> u32),
        entry!(0x00b0_1be0, fn_00b01be0(Ptr, u32, u32) -> u32),
        entry!(0x00b0_1c00, fn_00b01c00(Ptr) -> Ptr),
        entry!(0x00b0_1c20, fn_00b01c20(Ptr)),
        entry!(0x00b0_1c40, fn_00b01c40(Ptr, u8)),
        entry!(0x00b0_1c60, fn_00b01c60(Ptr)),
        entry!(0x00b0_1c70, fn_00b01c70(Ptr, u32, u32) -> u32),
        entry!(0x00b0_1c80, fn_00b01c80(Ptr, u32)),
        entry!(0x00b0_1c90, fn_00b01c90(Ptr, u32) -> u32),
        entry!(0x00b0_1ca0, fn_00b01ca0(Ptr, u32, u32)),
        entry!(0x00b0_1cb0, fn_00b01cb0(Ptr, u32) -> bool),
        entry!(0x00b0_1cc0, fn_00b01cc0(Ptr, u32)),
        entry!(0x00b0_1cd0, fn_00b01cd0(Ptr)),
        entry!(0x00b0_1ce0, fn_00b01ce0(Ptr, u32)),
        entry!(0x00b0_1cf0, fn_00b01cf0(Ptr, u32, u32, u32, u32)),
        entry!(0x00b0_1d00, fn_00b01d00(Ptr)),
        entry!(0x00b0_1d10, fn_00b01d10(Ptr, u32) -> bool),
        entry!(0x00b0_1d20, fn_00b01d20(Ptr, u32) -> Ptr),
        entry!(0x00b0_1d50, fn_00b01d50(Ptr, u32) -> Ptr),
        entry!(0x00b0_1f50, fn_00b01f50(Ptr)),
        entry!(0x00b0_2130, fn_00b02130(Ptr, u32) -> u8),
        entry!(0x00b0_23b0, fn_00b023b0(u32) -> u32),
        entry!(0x00b0_23f0, fn_00b023f0(u32) -> u32),
        entry!(0x00b0_2420, fn_00b02420(Ptr)),
        entry!(0x00b0_2460, fn_00b02460(Ptr)),
        entry!(0x00b0_2670, fn_00b02670(Ptr)),
        entry!(0x00b0_2720, fn_00b02720(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, target: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == target)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// A file object with a 16-byte buffer holding `filled`
    /// bytes read up to `position`.
    fn file(e: &mut Engine, filled: u32, position: u32) -> (u32, u32) {
        let this = e.mem.alloc(0x40);
        let storage = e.mem.alloc(0x10);
        e.mem.set_u32(this + BUFFER_CAPACITY, 0x10);
        e.mem.set_u32(this + BUFFER_FILLED, filled);
        e.mem.set_u32(this + BUFFER_POSITION, position);
        e.mem.set_u32(this + BUFFER_STORAGE, storage);
        (this, storage)
    }

    #[test]
    fn reads_from_the_buffer_when_it_holds_enough() {
        let mut e = Engine::new();
        e.register(MEMCPY, |_, _| Ret::default());
        let (this, storage) = file(&mut e, 0x10, 4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b0_1800, &args![this, 0x9000u32, 8u32]).u32(), 8);
        assert_eq!(calls_to(&e, MEMCPY), vec![vec![0x9000, storage + 4, 8]]);
        assert_eq!(e.mem.u32(this + BUFFER_POSITION), 12);
    }

    #[test]
    fn large_reads_go_direct_and_small_ones_refill() {
        let mut e = Engine::new();
        e.register(MEMCPY, |_, _| Ret::default());
        e.register(READ_DIRECT, |_, a| (a[2] / 2).into_ret());
        let (this, storage) = file(&mut e, 0x10, 0xc);
        e.call_log = Some(vec![]);
        // 4 buffered bytes, then 0x20 more read directly (half arrive).
        assert_eq!(
            e.call(0x00b0_1800, &args![this, 0x9000u32, 0x24u32]).u32(),
            4 + 0x10
        );
        assert_eq!(calls_to(&e, MEMCPY), vec![vec![0x9000, storage + 0xc, 4]]);
        assert_eq!(calls_to(&e, READ_DIRECT), vec![vec![this, 0x9004, 0x20, 4]]);
        assert_eq!(e.mem.u32(this + BUFFER_FILLED), 0);
        // Empty buffer, 12 wanted: a refill brings 8, so 8 are delivered.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b0_1800, &args![this, 0x9000u32, 12u32]).u32(), 8);
        assert_eq!(
            calls_to(&e, READ_DIRECT),
            vec![vec![this, storage, 0x10, 0]]
        );
        assert_eq!(calls_to(&e, MEMCPY), vec![vec![0x9000, storage, 8]]);
        assert_eq!(e.mem.u32(this + BUFFER_FILLED), 8);
        assert_eq!(e.mem.u32(this + BUFFER_POSITION), 8);
    }

    #[test]
    fn flag_tests() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00b0_1bc0, &args![0u32, 2u32, 9u32]).u32(), 0);
        assert_eq!(e.call(0x00b0_1bc0, &args![0u32, 4u32, 9u32]).u32(), 1);
        assert_eq!(e.call(0x00b0_1be0, &args![0u32, 4u32, 9u32]).u32(), 0);
        assert_eq!(e.call(0x00b0_1be0, &args![0u32, 2u32, 9u32]).u32(), 1);
    }

    #[test]
    fn win32_file_cache_wraps_the_base() {
        let mut e = Engine::new();
        e.register(0x00b0_0f20, |_, a| a[0].into_ret());
        e.register(0x00b0_1020, |_, _| Ret::default());
        let cache = e.mem.alloc(0x64);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b0_1c00, &args![cache]).u32(), cache);
        assert_eq!(e.mem.u32(cache), WIN32_FILE_CACHE_VTABLE);
        e.mem.set_u32(cache, 0);
        e.call(0x00b0_1c20, &args![cache]);
        assert_eq!(e.mem.u32(cache), WIN32_FILE_CACHE_VTABLE);
        assert_eq!(calls_to(&e, 0x00b0_0f20), vec![vec![cache]]);
        assert_eq!(calls_to(&e, 0x00b0_1020), vec![vec![cache]]);
    }
}

#[cfg(test)]
mod tests_branch {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// An engine whose KERNEL32 imports record their calls; `CreateThread` answers handles from
    /// 0x100 and `CreateSemaphoreA` 0x80.
    fn engine() -> (Engine, Log) {
        let mut e = Engine::new();
        e.map(0x0000_1000, 0x1000);
        let log: Log = Rc::default();
        for f in [
            INITIALIZE_CRITICAL_SECTION,
            ENTER_CRITICAL_SECTION,
            LEAVE_CRITICAL_SECTION,
            RELEASE_SEMAPHORE,
            RESUME_THREAD,
            EXIT_THREAD,
            WAIT_FOR_SINGLE_OBJECT,
            INTERLOCKED_INCREMENT,
            INTERLOCKED_DECREMENT,
            CREATE_SEMAPHORE_A,
            CREATE_THREAD,
            SLEEP,
            WORKER_ENTER_HOOKS,
            WORKER_LEAVE_HOOKS,
        ] {
            let log = log.clone();
            e.register_double(f, move |_, a| {
                let mut log = log.borrow_mut();
                let n = log.iter().filter(|(c, _)| *c == f).count() as u32;
                log.push((f, a.to_vec()));
                match f {
                    CREATE_THREAD => (0x100 + n).into_ret(),
                    CREATE_SEMAPHORE_A => 0x80u32.into_ret(),
                    _ => Ret::default(),
                }
            });
        }
        (e, log)
    }

    fn calls(log: &Log, f: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(c, _)| *c == f)
            .map(|(_, a)| a.clone())
            .collect()
    }

    /// Six suspended workers (a request of 9 is capped), then the running starter; the
    /// semaphores have counts 0 and maxima 1 and 0x7FFFFFFF.
    #[test]
    fn the_constructor_makes_capped_suspended_workers_and_a_starter() {
        let (mut e, log) = engine();
        let m = 0x1000u32;
        assert_eq!(e.call(0x00b0_1d50, &args![m, 9u32]).u32(), m);
        assert_eq!(e.mem.u32(m), MANAGER_VTABLE);
        assert_eq!(e.mem.u32(m + THREAD_LIMIT), 6);
        assert_eq!(e.mem.u32(m + THREAD_COUNT), 6);
        assert_eq!(e.mem.u8(m + 4), 1);
        assert_eq!(
            calls(&log, CREATE_SEMAPHORE_A),
            [vec![0, 0, 1, 0], vec![0, 0, 0x7fff_ffff, 0]]
        );
        let threads = calls(&log, CREATE_THREAD);
        assert_eq!(threads.len(), 7);
        for (i, t) in threads[..6].iter().enumerate() {
            assert_eq!(
                t,
                &vec![
                    0,
                    0,
                    WORKER_START,
                    m,
                    CREATE_SUSPENDED,
                    m + THREAD_IDS + 4 * i as u32
                ]
            );
            assert_eq!(
                e.mem.u32(m + THREAD_HANDLES + 4 * i as u32),
                0x100 + i as u32
            );
        }
        assert_eq!(&threads[6][..5], &[0, 0, STARTER_START, m, 0]);
        assert_eq!(e.mem.u32(m + STARTER), 0x106);
        assert_eq!(e.mem.u32(m + WORK + 4), 0x80);
    }

    /// The starter waits on its semaphore, resumes every worker and exits; a worker runs the
    /// hooks around its loop (stopped at once here) and exits.
    #[test]
    fn the_thread_starts_resume_and_run_the_loop() {
        let (mut e, log) = engine();
        let m = 0x1000u32;
        e.mem.set_u32(m + THREAD_COUNT, 2);
        e.mem.set_u32(m + THREAD_HANDLES, 0x200);
        e.mem.set_u32(m + THREAD_HANDLES + 4, 0x204);
        e.mem.set_u32(m + 0xc, 0x90);
        e.call(0x00b0_23b0, &args![m]);
        assert_eq!(calls(&log, WAIT_FOR_SINGLE_OBJECT), [vec![0x90, INFINITE]]);
        assert_eq!(calls(&log, INTERLOCKED_DECREMENT), [vec![m + 8]]);
        assert_eq!(calls(&log, RESUME_THREAD), [vec![0x200], vec![0x204]]);
        assert_eq!(calls(&log, EXIT_THREAD), [vec![0]]);
        e.mem.set_u8(m + STOPPING, 1);
        e.call(0x00b0_23f0, &args![m]);
        assert_eq!(calls(&log, WORKER_ENTER_HOOKS), [vec![m]]);
        assert_eq!(calls(&log, WORKER_LEAVE_HOOKS), [vec![m]]);
        assert_eq!(calls(&log, EXIT_THREAD).len(), 2);
    }

    /// One pass of the loop: the first task of list 2 runs (no decide byte), is finished and
    /// counted on its group; the next pass sees the stopping byte (set by `Sleep`'s double).
    #[test]
    fn a_worker_runs_the_first_task_and_counts_it_done() {
        let (mut e, log) = engine();
        let m = 0x1000u32;
        let (task, group, vtable) = (0x1400u32, 0x1500u32, 0x1600u32);
        e.mem.set_u32(m + LISTS + 8, task);
        e.mem.set_u32(task, vtable);
        e.mem.set_u32(task + 8, group);
        e.mem.set_u32(task + 0x14, 0x1450);
        e.mem.set_u32(group + 0x34, 1);
        e.mem.set_u32(group + 0xc, 0x91);
        e.put_vtable(vtable, &[0, 0x00ab_0004, 0x00ab_0008, 0x00ab_000c]);
        for f in [0x00ab_0004u32, 0x00ab_0008, 0x00ab_000c] {
            let log = log.clone();
            e.register_double(f, move |_, a| {
                log.borrow_mut().push((f, a.to_vec()));
                Ret::default()
            });
        }
        e.register(SLEEP, move |e, _| {
            e.mem.set_u8(0x1000 + STOPPING, 1);
            Ret::default()
        });
        e.call(0x00b0_2460, &args![m]);
        assert_eq!(e.mem.u32(m + LISTS + 8), 0x1450, "the list moves on");
        assert!(calls(&log, 0x00ab_0004).is_empty(), "no decide byte");
        assert_eq!(calls(&log, 0x00ab_0008), [vec![task]]);
        assert_eq!(calls(&log, 0x00ab_000c), [vec![task]]);
        assert_eq!((e.mem.u32(task + 8), e.mem.u32(task + 0x10)), (0, 0));
        assert_eq!(e.mem.u32(group + 0x38), 1);
        assert_eq!(calls(&log, RELEASE_SEMAPHORE), [vec![0x91, 1, 0]]);
        assert_eq!(calls(&log, INTERLOCKED_INCREMENT), [vec![group + 8]]);
    }

    /// Requeueing: a task goes to the end of its group's list (capped at 0x3F) and the work
    /// semaphore is released; a task without a group answers 0.
    #[test]
    fn a_task_goes_back_to_the_end_of_its_list() {
        let (mut e, log) = engine();
        let m = 0x1000u32;
        let (task, group) = (0x1400u32, 0x1500u32);
        e.mem.set_u32(m + WORK + 4, 0x92);
        assert_eq!(e.call(0x00b0_2130, &args![m, task]).u32() & 0xff, 0);
        e.mem.set_u32(task + 8, group);
        e.mem.set_u32(group + 0x30, 0x55);
        e.mem.set_u8(task + 4, 1);
        e.mem.set_u32(task + 0x14, 0xdead);
        assert_eq!(e.call(0x00b0_2130, &args![m, task]).u32() & 0xff, 1);
        assert_eq!(e.mem.u32(m + LISTS + 4 * 0x3f), task);
        assert_eq!(e.mem.u32(m + TAILS + 4 * 0x3f), task + 0x14);
        assert_eq!((e.mem.u32(task + 0x14), e.mem.u8(task + 4)), (0, 0));
        assert_eq!(calls(&log, RELEASE_SEMAPHORE), [vec![0x92, 1, 0]]);
        assert_eq!(calls(&log, INTERLOCKED_INCREMENT), [vec![m + WORK]]);
    }
}

#[cfg(test)]
mod tests_stop {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn engine() -> (Engine, Log) {
        let mut e = Engine::new();
        e.map(0x0000_1000, 0x1000);
        let log: Log = Rc::default();
        for f in [
            RELEASE_SEMAPHORE,
            INTERLOCKED_INCREMENT,
            WAIT_FOR_MULTIPLE_OBJECTS,
            CLOSE_HANDLE,
        ] {
            let log = log.clone();
            e.register_double(f, move |_, a| {
                log.borrow_mut().push((f, a.to_vec()));
                Ret::default()
            });
        }
        (e, log)
    }

    fn calls(log: &Log, f: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(c, _)| *c == f)
            .map(|(_, a)| a.clone())
            .collect()
    }

    /// Two workers with the byte `+5` set: the work semaphore is released twice, all are waited
    /// for, then both worker handles and the starter's are closed.
    #[test]
    fn the_stop_wakes_waits_for_and_closes_the_workers() {
        let (mut e, log) = engine();
        let m = 0x1000u32;
        e.mem.set_u8(m + 5, 1);
        e.mem.set_u32(m + THREAD_COUNT, 2);
        e.mem.set_u32(m + THREAD_HANDLES, 0x200);
        e.mem.set_u32(m + THREAD_HANDLES + 4, 0x204);
        e.mem.set_u32(m + STARTER, 0x300);
        e.mem.set_u32(m + WORK + 4, 0x80);
        e.call(0x00b0_1f50, &args![m]);
        assert_eq!(e.mem.u8(m + STOPPING), 1);
        assert_eq!(
            calls(&log, RELEASE_SEMAPHORE),
            [vec![0x80, 1, 0], vec![0x80, 1, 0]]
        );
        assert_eq!(
            calls(&log, INTERLOCKED_INCREMENT),
            [vec![m + WORK], vec![m + WORK]]
        );
        assert_eq!(
            calls(&log, WAIT_FOR_MULTIPLE_OBJECTS),
            [vec![2, m + THREAD_HANDLES, 1, INFINITE]]
        );
        assert_eq!(
            calls(&log, CLOSE_HANDLE),
            [vec![0x200], vec![0x204], vec![0x300]]
        );
    }

    /// With the byte `+5` clear nothing is released or waited for (the byte `+4` is not the
    /// one tested); the handles are still closed.
    #[test]
    fn the_stop_without_the_byte_only_closes_the_handles() {
        let (mut e, log) = engine();
        let m = 0x1000u32;
        e.mem.set_u8(m + 4, 1);
        e.mem.set_u32(m + THREAD_COUNT, 1);
        e.mem.set_u32(m + THREAD_HANDLES, 0x200);
        e.mem.set_u32(m + STARTER, 0x300);
        e.call(0x00b0_1f50, &args![m]);
        assert_eq!(e.mem.u8(m + STOPPING), 1);
        assert!(calls(&log, RELEASE_SEMAPHORE).is_empty());
        assert!(calls(&log, WAIT_FOR_MULTIPLE_OBJECTS).is_empty());
        assert_eq!(calls(&log, CLOSE_HANDLE), [vec![0x200], vec![0x300]]);
    }
}

#[cfg(test)]
mod tests_file_cache_slots {
    use super::*;

    #[test]
    fn the_lookup_slot_answers_zero_and_the_flag_slots_false() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x70);
        assert_eq!(e.call(0x00b0_1c90, &args![this, 0x1234u32]).u32(), 0);
        assert_eq!(e.call(0x00b0_1c70, &args![this, 1u32, 2u32]).u32(), 0);
        assert!(!e.call(0x00b0_1cb0, &args![this, 1u32]).bool());
        assert!(!e.call(0x00b0_1d10, &args![this, 1u32]).bool());
        e.call(0x00b0_1c40, &args![this, 0x5au32]);
        assert_eq!(e.mem.u8(this + 0x45), 0x5a);
    }

    #[test]
    fn the_deleting_destructor_frees_only_with_bit_zero() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x70);
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        e.register(0x00b0_1020, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b0_1d20, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), WIN32_FILE_CACHE_VTABLE);
        e.call(0x00b0_1d20, &args![this, 1u32]);
        let frees: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == MEMORY_MANAGER_DEALLOCATE)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(frees, [vec![MEMORY_MANAGER, this]]);
    }
}

#[cfg(test)]
mod tests_loose_files {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Names = Rc<RefCell<Vec<Vec<u8>>>>;

    /// Doubles for the find calls (answering `files` in turn into `cFileName` at +0x2c), the
    /// CRT string calls, the allocation and the list's add (recording the names added).
    fn engine(files: &'static [&'static [u8]]) -> (Engine, Names) {
        let mut e = Engine::new();
        let next = Rc::new(RefCell::new(0usize));
        let first = next.clone();
        e.register_double(FIND_FIRST_FILE_A, move |e, a| {
            if files.is_empty() {
                return 0xffff_ffffu32.into_ret();
            }
            e.mem.set_cstr(a[1] + 0x2c, files[0]);
            *first.borrow_mut() = 1;
            0x77u32.into_ret()
        });
        let more = next.clone();
        e.register_double(FIND_NEXT_FILE_A, move |e, a| {
            let i = *more.borrow();
            if i >= files.len() {
                return 0u32.into_ret();
            }
            e.mem.set_cstr(a[1] + 0x2c, files[i]);
            *more.borrow_mut() = i + 1;
            1u32.into_ret()
        });
        e.register(FIND_CLOSE, |_, _| 1u32.into_ret());
        e.register(STRLEN, |e, a| (e.mem.cstr(a[0]).len() as u32).into_ret());
        e.register(STRRCHR, |e, a| {
            let text = e.mem.cstr(a[0]);
            match text.iter().rposition(|&c| u32::from(c) == a[1]) {
                Some(i) => (a[0] + i as u32).into_ret(),
                None => 0u32.into_ret(),
            }
        });
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            assert!(text.len() < a[1] as usize, "room");
            e.mem.set_cstr(a[0], &text);
            0u32.into_ret()
        });
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        let names: Names = Rc::default();
        let seen = names.clone();
        e.register_double(LIST_ADD, move |e, a| {
            let name = e.mem.u32(a[1]);
            seen.borrow_mut().push(e.mem.cstr(name));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        (e, names)
    }

    fn text(e: &mut Engine, t: &[u8]) -> u32 {
        let at = e.mem.alloc(t.len() as u32 + 1);
        e.mem.set_cstr(at, t);
        at
    }

    #[test]
    fn each_found_file_gets_the_paths_directory() {
        let (mut e, names) = engine(&[b"a.nif", b"bb.dds"]);
        let pattern = text(&mut e, b"Data\\Meshes\\*.nif");
        let path = text(&mut e, b"Data\\Meshes\\x.nif");
        add_loose_files(&mut e, pattern, path, 0x1234);
        assert_eq!(
            *names.borrow(),
            vec![
                b"Data\\Meshes\\a.nif".to_vec(),
                b"Data\\Meshes\\bb.dds".to_vec()
            ]
        );
        let log = e.call_log.as_ref().unwrap();
        let closes: Vec<&Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == FIND_CLOSE)
            .map(|c| &c.1)
            .collect();
        assert_eq!(closes, vec![&vec![0x77]]);
        let adds: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == LIST_ADD)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(adds, vec![0x1234, 0x1234]);
    }

    #[test]
    fn a_path_without_a_backslash_adds_nothing() {
        let (mut e, names) = engine(&[b"a.nif"]);
        let pattern = text(&mut e, b"*.nif");
        let path = text(&mut e, b"x.nif");
        add_loose_files(&mut e, pattern, path, 0x1234);
        assert!(names.borrow().is_empty());
        let log = e.call_log.as_ref().unwrap();
        // The block is still allocated (the whole path, 5, + 5 + 1) and the search closed.
        let sizes: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == ALLOCATE)
            .map(|c| c.1[1])
            .collect();
        assert_eq!(sizes, vec![11]);
        assert_eq!(log.iter().filter(|c| c.0 == FIND_CLOSE).count(), 1);
    }

    #[test]
    fn nothing_found_closes_nothing() {
        let (mut e, names) = engine(&[]);
        let pattern = text(&mut e, b"*.nif");
        let path = text(&mut e, b"Data\\x");
        e.call(0x00b0_1930, &args![pattern, path, 0x1234u32]);
        assert!(names.borrow().is_empty());
        let log = e.call_log.as_ref().unwrap();
        assert!(log.iter().all(|c| c.0 != FIND_CLOSE && c.0 != STRLEN));
    }
}

#[cfg(test)]
mod tests_find_and_queue {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Texts = Rc<RefCell<Vec<(Vec<u8>, u32)>>>;

    /// One search entry: attributes, last write time (low word) and name.
    type Entry = (u32, u32, &'static [u8]);

    fn count(e: &Engine, addr: u32) -> usize {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .count()
    }

    fn text(e: &mut Engine, s: &[u8]) -> u32 {
        let p = e.mem.alloc(s.len() as u32 + 1);
        e.mem.write(p, s);
        p
    }

    fn put_entry(e: &mut Engine, find: u32, entry: Entry) {
        e.mem.set_u32(find, entry.0);
        e.mem.set_u32(find + 0x14, entry.1);
        e.mem.set_u32(find + 0x18, 0x55);
        e.mem.write(find + 0x2c, entry.2);
        e.mem.set_u8(find + 0x2c + entry.2.len() as u32, 0);
    }

    /// Searches by pattern: each pattern's entries, handle `index + 1`; the cursor per handle
    /// in `cursors`.
    fn searches(e: &mut Engine, tree: &'static [(&'static [u8], &'static [Entry])]) {
        let cursors = e.mem.alloc(0x40);
        e.register_double(FIND_FIRST_FILE_A, move |e, a| {
            let pattern = e.mem.cstr(a[0]);
            match tree.iter().position(|t| t.0 == pattern.as_slice()) {
                Some(i) if !tree[i].1.is_empty() => {
                    put_entry(e, a[1], tree[i].1[0]);
                    e.mem.set_u32(cursors + 4 * i as u32, 1);
                    (i as u32 + 1).into_ret()
                }
                _ => NO_HANDLE.into_ret(),
            }
        });
        e.register_double(FIND_NEXT_FILE_A, move |e, a| {
            let i = a[0] - 1;
            let at = e.mem.u32(cursors + 4 * i);
            let entries = tree[i as usize].1;
            if (at as usize) < entries.len() {
                put_entry(e, a[1], entries[at as usize]);
                e.mem.set_u32(cursors + 4 * i, at + 1);
                1u32.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.register(FIND_CLOSE, |_, _| 1u32.into_ret());
    }

    #[test]
    fn a_file_is_found_by_its_object_name_or_by_path() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        searches(&mut e, &[(b"x.esm", &[(0, 0x1234, b"x.esm")])]);
        let this = e.mem.alloc(8);
        let name = text(&mut e, b"x.esm");
        let other = text(&mut e, b"y.esm");
        e.put_vtable(0x0900_0000, &[0, 0, 0, 0, 0, 0, 0x0a00_0000]);
        e.mem.set_u32(this, 0x0900_0000);
        let mut names = vec![name, other].into_iter();
        e.register_double(0x0a00_0000, move |_, _| names.next().unwrap().into_ret());
        assert_eq!(e.call(0x00b0_1510, &args![this, 0u32]).u8(), 1);
        assert_eq!(fn_00b01510(&mut e, this, 0), 0);
        assert_eq!(e.call(0x00b0_1af0, &args![name]).u8(), 1);
        assert_eq!(fn_00b01af0(&mut e, other), 0);
        assert_eq!(count(&e, FIND_CLOSE), 2);
        let time = e.mem.alloc(8);
        e.mem.set_u32(time, 7);
        assert_eq!(fn_00b01b60(&mut e, other, time), 0);
        assert_eq!(e.mem.u32(time), 7);
        assert_eq!(e.call(0x00b0_1b60, &args![name, time]).u8(), 1);
        assert_eq!((e.mem.u32(time), e.mem.u32(time + 4)), (0x1234, 0x55));
        assert_eq!(count(&e, FIND_CLOSE), 3);
    }

    #[test]
    fn the_walk_counts_newer_files_through_subfolders() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        searches(
            &mut e,
            &[
                (
                    b"r\\a\\*.*",
                    &[
                        (0x10, 9, b"sub"),
                        (0x20, 9, b"x"),
                        (0x10, 1, b"."),
                        (0x20, 1, b"y"),
                    ],
                ),
                (b"r\\a\\sub\\*.*", &[(0x20, 9, b"z")]),
            ],
        );
        let names: Texts = Rc::new(RefCell::new(vec![]));
        let seen = names.clone();
        e.register_double(CALLEE_00AFD390, move |e, a| {
            let s = e.mem.cstr(a[1]);
            e.mem.set_u8(a[0], s[0]);
            seen.borrow_mut().push((s, a[2]));
            Ret::default()
        });
        e.register_double(CALLEE_00AF9C50, |e, a| {
            e.mem.set_u32(a[2], 0x77);
            1u32.into_ret()
        });
        e.register_double(CALLEE_00AFAC90, |e, a| {
            assert_eq!(a[1], 0x77);
            u32::from(e.mem.u8(a[2]) != b'q').into_ret()
        });
        // Written after the time when the entry's low word is above it.
        e.register_double(COMPARE_FILE_TIME, |e, a| {
            let (entry, time) = (e.mem.u32(a[0]), e.mem.u32(a[1]));
            (if entry > time { 1u32 } else { 0xffff_ffff }).into_ret()
        });
        // The exe's "*.*" and "\" (`.rdata`).
        e.map(ALL_FILES, 8);
        e.mem.write(ALL_FILES, b"*.*\0\\\0");
        let this = e.mem.alloc(8);
        let root = text(&mut e, b"r\\");
        let folder = text(&mut e, b"a\\");
        let time = e.mem.alloc(8);
        e.mem.set_u32(time, 5);
        assert_eq!(
            e.call(0x00b0_1590, &args![this, root, folder, time]).u32(),
            2
        );
        assert_eq!(
            *names.borrow(),
            vec![
                (b"a".to_vec(), 2),
                (b"a\\sub".to_vec(), 2),
                (b"z".to_vec(), 0),
                (b"x".to_vec(), 0)
            ]
        );
        assert_eq!(count(&e, FIND_CLOSE), 2);
        // Nothing to search: 0 and no other call.
        e.call_log = Some(vec![]);
        let none = text(&mut e, b"none\\");
        assert_eq!(fn_00b01590(&mut e, this, root, none, time), 0);
        assert_eq!(count(&e, CALLEE_00AFD390), 0);
        assert_eq!(count(&e, FIND_CLOSE), 0);
    }

    #[test]
    fn the_destructors_close_and_free() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        for slot in [
            CLOSE_HANDLE,
            DELETE_CRITICAL_SECTION,
            MEMORY_MANAGER_DEALLOCATE,
        ] {
            e.register(slot, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x280);
        e.mem.set_u32(this + 0xc, 0x11);
        e.mem.set_u32(this + 0x270, 0x22);
        assert_eq!(e.call(0x00b0_2640, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        let log = e.call_log.as_ref().unwrap();
        let closes: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == CLOSE_HANDLE)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(closes, vec![0x22, 0x11]);
        assert_eq!(count(&e, DELETE_CRITICAL_SECTION), 1);
        assert_eq!(count(&e, MEMORY_MANAGER_DEALLOCATE), 1);
        fn_00b02640(&mut e, this, 0);
        assert_eq!(count(&e, MEMORY_MANAGER_DEALLOCATE), 1);
        // 00b02360 deletes through slot 0 with 1.
        let object = e.mem.alloc(4);
        e.put_vtable(0x0900_0000, &[0x0a00_0000]);
        e.mem.set_u32(object, 0x0900_0000);
        e.register(0x0a00_0000, |_, _| Ret::default());
        e.call(0x00b0_2360, &args![0u32, object]);
        e.call(0x00b0_2360, &args![0u32, 0u32]);
        let log = e.call_log.as_ref().unwrap();
        let deletes: Vec<_> = log.iter().filter(|c| c.0 == 0x0a00_0000).collect();
        assert_eq!(deletes.len(), 1);
        assert_eq!(deletes[0].1, vec![object, 1]);
        e.call(0x00b0_2710, &args![this, 1u32, 2u32]);
    }

    #[test]
    fn the_thread_id_search_and_the_group_close() {
        let mut e = Engine::new();
        for slot in [ENTER_CRITICAL_SECTION, LEAVE_CRITICAL_SECTION] {
            e.register(slot, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x60);
        for i in 0..6 {
            e.mem.set_u32(this + 0x38 + 4 * i, 100 + i);
        }
        e.mem.set_u32(this + 0x50, 106);
        assert_eq!(e.call(0x00b0_2600, &args![this, 105u32]).u8(), 1);
        assert_eq!(fn_00b02600(&mut e, this, 106), 0);
        let group = e.mem.alloc(0x40);
        e.mem.set_u8(group + 0x2d, 1);
        assert_eq!(e.call(0x00b0_2310, &args![0u32, group]).u8(), 0);
        assert_eq!(e.mem.u8(group + 0x2d), 1);
        e.mem.set_u8(group + 0x2c, 1);
        assert_eq!(fn_00b02310(&mut e, 0, group), 1);
        assert_eq!(e.mem.u8(group + 0x2d), 0);
    }

    #[test]
    fn a_task_is_queued_once_on_a_running_open_group() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        for slot in [ENTER_CRITICAL_SECTION, LEAVE_CRITICAL_SECTION] {
            e.register(slot, |_, _| Ret::default());
        }
        e.register(INTERLOCKED_COMPARE_EXCHANGE, |e, a| {
            let old = e.mem.u32(a[0]);
            if old == a[2] {
                e.mem.set_u32(a[0], a[1]);
            }
            old.into_ret()
        });
        // The real queue (00b02130) on a manager: its semaphore release doubled.
        for slot in [RELEASE_SEMAPHORE, INTERLOCKED_INCREMENT] {
            e.register(slot, |_, _| Ret::default());
        }
        let manager = e.mem.alloc(0x280);
        let group = e.mem.alloc(0x40);
        let task = e.mem.alloc(0x18);
        e.mem.set_u8(task + 4, 9);
        // Not running: the claim is dropped.
        assert_eq!(
            e.call(0x00b0_2220, &args![manager, group, task, 1u32]).u8(),
            0
        );
        assert_eq!(e.mem.u32(task + 0x10), 0);
        assert_eq!(e.mem.u32(task + 8), 0);
        // Running and open: queued, the flag (low byte) closes the group.
        e.mem.set_u8(group + 0x2c, 1);
        e.mem.set_u8(group + 0x2d, 1);
        assert_eq!(fn_00b02220(&mut e, manager, group, task, 0x101), 1);
        assert_eq!(e.mem.u32(task + 8), group);
        assert_eq!(e.mem.u8(task + 4), 0);
        assert_eq!(e.mem.u8(group + 0x2d), 0);
        assert_eq!(e.mem.u32(group + 0x34), 1);
        assert_eq!(e.mem.u32(task + 0x10), 1);
        // The task heads the manager's first list.
        assert_eq!(e.mem.u32(manager + LISTS), task);
        // Claimed already: 0, nothing changes.
        assert_eq!(fn_00b02220(&mut e, manager, group, task, 0), 0);
        assert_eq!(e.mem.u32(group + 0x34), 1);
        assert_eq!(e.mem.u32(task + 0x10), 1);
    }
}
