//! `BSCore/oei_io_thread.cpp` (Xbox PDB source unit), subsystem `BSCore`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `OEI::IO` runs C file operations (`fseek`, `fread`, `fwrite`) on one
//! "Serialized IO Thread". A caller builds an `IO_Request` on its stack,
//! takes a wait semaphore from a pool of 32, queues the request (under a
//! critical section), signals the thread's request semaphore and waits on
//! its own semaphore until the thread has run the request and released it.
//! A request of type 4 makes the thread release the shutdown semaphore and
//! end.
//!
//! The globals (`011f6bf0` to `011f6c08`) are named here by the role the
//! code gives them; the exe has no names for them.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::platform::MEMORY_MANAGER;

/// `MemoryManager::Allocate` (`00aa3e40`) and `Deallocate` (`00aa4060`).
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
/// `BSThread::BSThread` (Xbox PDB, `00aa6360`, `this` only).
const BS_THREAD_CONSTRUCTOR: u32 = 0x00aa_6360;
/// `BSThread`'s destructor body (`00aa63c0`, `this` only).
const BS_THREAD_DESTRUCTOR: u32 = 0x00aa_63c0;
/// `BSThread::Initialize` (Xbox PDB, `00aa6430`): `this`, stack size, name,
/// one more word.
const BS_THREAD_INITIALIZE: u32 = 0x00aa_6430;
/// `00aa64f0` (`bsthread_xenon.cpp` range, `this` only): called on the IO
/// thread at shutdown, before it is deleted.
const BS_THREAD_SHUTDOWN: u32 = 0x00aa_64f0;
/// `BSThread::SetThreadProcessor` (Xbox PDB, `00aa6550`): `this`, processor.
const BS_THREAD_SET_THREAD_PROCESSOR: u32 = 0x00aa_6550;
/// `_eh_vector_constructor_iterator_` (CRT: block, element size, count,
/// constructor, destructor).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
/// `_eh_vector_destructor_iterator_` (CRT: block, element size, count,
/// destructor).
const VECTOR_DESTRUCTOR_ITERATOR: u32 = 0x00ec_5fce;
/// The constructor (`0078d3b0`) and destructor (`0055a2d0`) the semaphore
/// array's iterators run on each 12-byte element.
const SEMAPHORE_CONSTRUCTOR: u32 = 0x0078_d3b0;
const SEMAPHORE_DESTRUCTOR: u32 = 0x0055_a2d0;
/// `fseek`, `fread`, `fwrite` (CRT, cdecl).
const FSEEK: u32 = 0x00ec_b65a;
const FREAD: u32 = 0x00ec_b3a8;
const FWRITE: u32 = 0x00ec_b086;

/// Import slots (`kernel32`, stdcall).
const CREATE_SEMAPHORE_A: u32 = 0x00fd_f0b4;
const INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;
const DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
const WAIT_FOR_SINGLE_OBJECT: u32 = 0x00fd_f1c8;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const RELEASE_SEMAPHORE: u32 = 0x00fd_f1b4;
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const CLOSE_HANDLE: u32 = 0x00fd_f07c;
/// `INFINITE` (the wait's timeout).
const INFINITE: u32 = 0xffff_ffff;

/// The IO thread (`IO_Thread*`).
pub const IO_THREAD: u32 = 0x011f_6bf0;
/// The semaphore the thread waits on: released once per queued request
/// (`BSSemaphore*`, maximum count 0xffff).
pub const REQUEST_SEMAPHORE: u32 = 0x011f_6bf4;
/// The semaphore the thread releases when it ends (`BSSemaphore*`,
/// maximum count 1).
pub const SHUTDOWN_SEMAPHORE: u32 = 0x011f_6bf8;
/// The critical section (`CRITICAL_SECTION*`, 0x18 bytes) guarding both
/// lists.
pub const LIST_LOCK: u32 = 0x011f_6bfc;
/// The queued requests (`BSList<OEI::IO::IO_Request *>*`).
pub const REQUEST_LIST: u32 = 0x011f_6c00;
/// The wait semaphores not in use (`BSList<BSSemaphore *>*`).
pub const SEMAPHORE_LIST: u32 = 0x011f_6c04;
/// The array of 32 wait semaphores (`BSSemaphore*`, its count in the word
/// before it).
pub const SEMAPHORE_ARRAY: u32 = 0x011f_6c08;
/// Wait semaphores in the array.
const SEMAPHORE_COUNT: u32 = 0x20;

/// Vtables (RTTI checked): `BSListBase`, `BSPointerListBase` and `BSList`
/// of `OEI::IO::IO_Request *` and of `BSSemaphore *`, and `IO_Thread`.
const REQUEST_LIST_BASE_VTABLE: u32 = 0x010a_29b0;
const REQUEST_POINTER_LIST_BASE_VTABLE: u32 = 0x010a_29a0;
const REQUEST_LIST_VTABLE: u32 = 0x010a_2990;
const SEMAPHORE_LIST_BASE_VTABLE: u32 = 0x010a_2980;
const SEMAPHORE_POINTER_LIST_BASE_VTABLE: u32 = 0x010a_2970;
const SEMAPHORE_LIST_VTABLE: u32 = 0x010a_2960;
const IO_THREAD_VTABLE: u32 = 0x010a_29c0;
/// `"Serialized IO Thread"`.
const THREAD_NAME: u32 = 0x010a_2944;

/// List vtable slots: `NewItem` (+4) and `DeleteItem` (+8).
const SLOT_NEW_ITEM: u32 = 0x04;
const SLOT_DELETE_ITEM: u32 = 0x08;

/// `IO_Request_Type` values the thread handles.
const REQUEST_SEEK: u32 = 1;
const REQUEST_READ: u32 = 2;
const REQUEST_WRITE: u32 = 3;
const REQUEST_QUIT: u32 = 4;

layout! {
    /// `OEI::IO::IO_Request` (Xbox PDB, 0x24 bytes there); 0x1C bytes on the
    /// PC, which has no `m_hFile` and `m_bWinResult`. Offsets are the PC
    /// code's.
    pub struct IoRequest: 0x1C {
        /// `m_pFile` (Xbox PDB): `FILE*`.
        0x00 m_pFile: u32,
        /// `m_eType` (Xbox PDB).
        0x04 m_eType: u32,
        /// `m_eWhence` (Xbox PDB).
        0x08 m_eWhence: u32,
        /// `m_pMemory` (Xbox PDB).
        0x0C m_pMemory: u32,
        /// `m_uiSize` (Xbox PDB): the byte count, or the seek offset.
        0x10 m_uiSize: u32,
        /// `m_wait_handle` (Xbox PDB): `BSSemaphore*`.
        0x14 m_wait_handle: u32,
        /// `result` (Xbox PDB): what the C function returned.
        0x18 result: u32,
    }

    /// `BSListBase<T>` (Xbox PDB), 0x10 bytes: the layout of every list
    /// instance here.
    pub struct BSListBase: 0x10 {
        /// `m_pkHead` (Xbox PDB).
        0x04 m_pkHead: u32,
        /// `m_pkTail` (Xbox PDB).
        0x08 m_pkTail: u32,
        /// `m_uiCount` (Xbox PDB).
        0x0C m_uiCount: u32,
    }

    /// `BSListItem<T>` (Xbox PDB), 0x0C bytes.
    pub struct BSListItem: 0x0C {
        /// `m_pkNext` (Xbox PDB).
        0x00 m_pkNext: u32,
        /// `m_pkPrev` (Xbox PDB).
        0x04 m_pkPrev: u32,
        /// `m_element` (Xbox PDB).
        0x08 m_element: u32,
    }
}

/// `BSSemaphore::iCount` (Xbox PDB) +0x00, `hSemaphore` +0x04, `iMaxCount`
/// +0x08 (the PC code uses the same offsets).
const SEMAPHORE_COUNT_FIELD: u32 = 0x00;
const SEMAPHORE_HANDLE_FIELD: u32 = 0x04;
const SEMAPHORE_MAX_FIELD: u32 = 0x08;

/// The inlined `BSSemaphore` wait: waits on the handle, then decrements the
/// count.
fn semaphore_wait(e: &mut Engine, semaphore: u32) {
    let handle = e.mem.u32(semaphore.wrapping_add(SEMAPHORE_HANDLE_FIELD));
    e.call(WAIT_FOR_SINGLE_OBJECT, &args![handle, INFINITE]);
    e.call(INTERLOCKED_DECREMENT, &args![semaphore]);
}

/// The inlined `BSSemaphore` signal: releases the handle once, then
/// increments the count.
fn semaphore_signal(e: &mut Engine, semaphore: u32) {
    let handle = e.mem.u32(semaphore.wrapping_add(SEMAPHORE_HANDLE_FIELD));
    e.call(RELEASE_SEMAPHORE, &args![handle, 1u32, 0u32]);
    e.call(INTERLOCKED_INCREMENT, &args![semaphore]);
}

/// The inlined `BSSemaphore` constructor on a fresh 12-byte block.
fn semaphore_construct(e: &mut Engine, semaphore: u32, max_count: u32) {
    e.mem
        .set_u32(semaphore.wrapping_add(SEMAPHORE_COUNT_FIELD), 0);
    e.mem
        .set_u32(semaphore.wrapping_add(SEMAPHORE_MAX_FIELD), max_count);
    let initial = e.mem.u32(semaphore.wrapping_add(SEMAPHORE_COUNT_FIELD));
    let max = e.mem.u32(semaphore.wrapping_add(SEMAPHORE_MAX_FIELD));
    let handle = e
        .call(CREATE_SEMAPHORE_A, &args![0u32, initial, max, 0u32])
        .u32();
    e.mem
        .set_u32(semaphore.wrapping_add(SEMAPHORE_HANDLE_FIELD), handle);
}

/// The inlined `BSList` constructor: the three vtables in order, then an
/// empty list.
fn list_construct(e: &mut Engine, list: u32, vtables: [u32; 3]) {
    let l: Ptr<BSListBase> = Ptr::new(list);
    e.mem.set_u32(list, vtables[0]);
    e.set(l, BSListBase::m_uiCount, 0);
    e.set(l, BSListBase::m_pkHead, 0);
    e.set(l, BSListBase::m_pkTail, 0);
    e.mem.set_u32(list, vtables[1]);
    e.mem.set_u32(list, vtables[2]);
}

fn allocate(e: &mut Engine, size: u32) -> u32 {
    e.call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, size])
        .u32()
}

fn deallocate(e: &mut Engine, block: u32) {
    e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, block]);
}

fn enter_list_lock(e: &mut Engine) {
    let section = e.global::<u32>(LIST_LOCK);
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
}

fn leave_list_lock(e: &mut Engine) {
    let section = e.global::<u32>(LIST_LOCK);
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
}

/// A new item from the list (`NewItem`, slot +4) holding `element`, added at
/// the tail.
fn queue_request(e: &mut Engine, request: u32) {
    let list = e.global::<u32>(REQUEST_LIST);
    let item = e.vcall(list, SLOT_NEW_ITEM, &args![]).u32();
    e.set(Ptr::<BSListItem>::new(item), BSListItem::m_element, request);
    fn_00aa8b80(e, Ptr::new(list), item);
}

// Translated from 00aa7eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `OEI::IO::initialize` (Xbox PDB): creates the request semaphore (maximum
/// 0xffff), the shutdown semaphore (maximum 1), the critical section, the
/// request and semaphore lists and the array of 32 wait semaphores (each
/// added to the semaphore list's head), then the IO thread, which it
/// starts as "Serialized IO Thread" with a 0x4000-byte stack. A failed
/// allocation leaves its global null.
// Translation incomplete: 00aa7eb0 - SEH exception unwinding is unsupported.
pub fn oei_io_initialize(e: &mut Engine) {
    let semaphore = allocate(e, 0xc);
    if semaphore != 0 {
        semaphore_construct(e, semaphore, 0xffff);
    }
    e.set_global(REQUEST_SEMAPHORE, semaphore);

    let semaphore = allocate(e, 0xc);
    if semaphore != 0 {
        semaphore_construct(e, semaphore, 1);
    }
    e.set_global(SHUTDOWN_SEMAPHORE, semaphore);

    let section = allocate(e, 0x18);
    if section != 0 {
        e.call(INITIALIZE_CRITICAL_SECTION, &args![section]);
    }
    e.set_global(LIST_LOCK, section);

    let list = allocate(e, 0x10);
    if list != 0 {
        list_construct(
            e,
            list,
            [
                REQUEST_LIST_BASE_VTABLE,
                REQUEST_POINTER_LIST_BASE_VTABLE,
                REQUEST_LIST_VTABLE,
            ],
        );
    }
    e.set_global(REQUEST_LIST, list);

    let list = allocate(e, 0x10);
    if list != 0 {
        list_construct(
            e,
            list,
            [
                SEMAPHORE_LIST_BASE_VTABLE,
                SEMAPHORE_POINTER_LIST_BASE_VTABLE,
                SEMAPHORE_LIST_VTABLE,
            ],
        );
    }
    e.set_global(SEMAPHORE_LIST, list);

    let block = allocate(e, 0x184);
    let mut array = 0;
    if block != 0 {
        e.mem.set_u32(block, SEMAPHORE_COUNT);
        e.call(
            VECTOR_CONSTRUCTOR_ITERATOR,
            &args![
                block + 4,
                0xcu32,
                SEMAPHORE_COUNT,
                SEMAPHORE_CONSTRUCTOR,
                SEMAPHORE_DESTRUCTOR
            ],
        );
        array = block + 4;
    }
    e.set_global(SEMAPHORE_ARRAY, array);

    for i in 0..SEMAPHORE_COUNT {
        e.with_stack(4, |e, element| {
            let semaphore = (i * 0xc).wrapping_add(e.global::<u32>(SEMAPHORE_ARRAY));
            e.mem.set_u32(element.addr(), semaphore);
            let list = e.global::<u32>(SEMAPHORE_LIST);
            fn_00aa8a40(e, Ptr::new(list), element);
        });
    }

    let block = allocate(e, 0x30);
    let thread = if block != 0 {
        fn_00aa8410(e, Ptr::new(block)).addr()
    } else {
        0
    };
    e.set_global(IO_THREAD, thread);
    let thread = e.global::<u32>(IO_THREAD);
    e.call(
        BS_THREAD_INITIALIZE,
        &args![thread, 0x4000u32, THREAD_NAME, 0u32],
    );
}

// Translated from 00aa81d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Shuts the IO thread down: queues a quit request, waits on the shutdown
/// semaphore, calls `00aa64f0` on the thread, deletes the thread, the
/// semaphore array, both lists (through their scalar deleting destructors),
/// the critical section and both semaphores (closing their handles), and
/// clears each global.
pub fn fn_00aa81d0(e: &mut Engine) {
    e.with_stack(0x1c, |e, local| {
        let request: Ptr<IoRequest> = local.cast();
        e.set(request, IoRequest::m_pFile, 0);
        e.set(request, IoRequest::m_eType, 0);
        e.set(request, IoRequest::m_pMemory, 0);
        e.set(request, IoRequest::m_wait_handle, 0);
        e.set(request, IoRequest::m_eType, REQUEST_QUIT);
        fn_00aa86b0(e, local);
    });
    let shutdown = e.global::<u32>(SHUTDOWN_SEMAPHORE);
    semaphore_wait(e, shutdown);
    let thread = e.global::<u32>(IO_THREAD);
    e.call(BS_THREAD_SHUTDOWN, &args![thread]);

    let thread = e.global::<u32>(IO_THREAD);
    if thread != 0 {
        e.vcall(thread, 0, &args![1u32]);
    }
    e.set_global(IO_THREAD, 0u32);

    let array = e.global::<u32>(SEMAPHORE_ARRAY);
    if array != 0 {
        fn_00aa8850(e, Ptr::new(array), 3);
    }
    e.set_global(SEMAPHORE_ARRAY, 0u32);

    let list = e.global::<u32>(SEMAPHORE_LIST);
    if list != 0 {
        e.vcall(list, 0, &args![1u32]);
    }
    e.set_global(SEMAPHORE_LIST, 0u32);

    let list = e.global::<u32>(REQUEST_LIST);
    if list != 0 {
        e.vcall(list, 0, &args![1u32]);
    }
    e.set_global(REQUEST_LIST, 0u32);

    let section = e.global::<u32>(LIST_LOCK);
    if section != 0 {
        e.call(DELETE_CRITICAL_SECTION, &args![section]);
        deallocate(e, section);
    }
    e.set_global(LIST_LOCK, 0u32);

    for global in [SHUTDOWN_SEMAPHORE, REQUEST_SEMAPHORE] {
        let semaphore = e.global::<u32>(global);
        if semaphore != 0 {
            let handle = e.mem.u32(semaphore.wrapping_add(SEMAPHORE_HANDLE_FIELD));
            e.call(CLOSE_HANDLE, &args![handle]);
            deallocate(e, semaphore);
        }
        e.set_global(global, 0u32);
    }
}

// Translated from 00aa8410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IO_Thread`'s constructor: the `BSThread` constructor, then the
/// `IO_Thread` vtable. Returns `this`.
pub fn fn_00aa8410(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BS_THREAD_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), IO_THREAD_VTABLE);
    this
}

// Translated from 00aa8430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `OEI::IO::IO_Thread::ThreadProc` (Xbox PDB): runs on processor 1; for
/// each signal of the request semaphore takes the first queued request and
/// runs it (1: `fseek(file, size, whence)`, 2: `fread(memory, 1, size,
/// file)`, 3: `fwrite(memory, 1, size, file)`, storing the result; other
/// types do nothing), then signals the request's wait semaphore if it has
/// one. A type-4 request signals the shutdown semaphore instead and ends
/// the loop. Returns 0.
pub fn oei_io_io_thread_thread_proc(e: &mut Engine, this: Ptr) -> u32 {
    e.call(BS_THREAD_SET_THREAD_PROCESSOR, &args![this, 1u32]);
    loop {
        let semaphore = e.global::<u32>(REQUEST_SEMAPHORE);
        semaphore_wait(e, semaphore);
        enter_list_lock(e);
        let list = e.global::<u32>(REQUEST_LIST);
        let request: Ptr<IoRequest> = Ptr::new(fn_00aa8a80(e, Ptr::new(list)));
        leave_list_lock(e);
        let mut quit = false;
        match e.get(request, IoRequest::m_eType) {
            REQUEST_SEEK => {
                let whence = e.get(request, IoRequest::m_eWhence);
                let offset = e.get(request, IoRequest::m_uiSize);
                let file = e.get(request, IoRequest::m_pFile);
                let result = e.call(FSEEK, &args![file, offset, whence]).u32();
                e.set(request, IoRequest::result, result);
            }
            REQUEST_READ => {
                let file = e.get(request, IoRequest::m_pFile);
                let size = e.get(request, IoRequest::m_uiSize);
                let memory = e.get(request, IoRequest::m_pMemory);
                let result = e.call(FREAD, &args![memory, 1u32, size, file]).u32();
                e.set(request, IoRequest::result, result);
            }
            REQUEST_WRITE => {
                let file = e.get(request, IoRequest::m_pFile);
                let size = e.get(request, IoRequest::m_uiSize);
                let memory = e.get(request, IoRequest::m_pMemory);
                let result = e.call(FWRITE, &args![memory, 1u32, size, file]).u32();
                e.set(request, IoRequest::result, result);
            }
            REQUEST_QUIT => quit = true,
            _ => {}
        }
        if quit {
            let shutdown = e.global::<u32>(SHUTDOWN_SEMAPHORE);
            semaphore_signal(e, shutdown);
            return 0;
        }
        let wait = e.get(request, IoRequest::m_wait_handle);
        if wait != 0 {
            semaphore_signal(e, wait);
        }
    }
}

/// Builds a request on the stack (the words the three wrappers clear
/// first, then type, file, memory, size and whence as given), runs it
/// through [`fn_00aa8740`] and returns its result.
fn run_request(
    e: &mut Engine,
    kind: u32,
    file: u32,
    memory: Option<u32>,
    size: u32,
    whence: Option<u32>,
) -> u32 {
    e.with_stack(0x1c, |e, local| {
        let request: Ptr<IoRequest> = local.cast();
        e.set(request, IoRequest::m_pFile, 0);
        e.set(request, IoRequest::m_eType, 0);
        e.set(request, IoRequest::m_pMemory, 0);
        e.set(request, IoRequest::m_wait_handle, 0);
        e.set(request, IoRequest::m_eType, kind);
        e.set(request, IoRequest::m_pFile, file);
        if let Some(whence) = whence {
            e.set(request, IoRequest::m_eWhence, whence);
        }
        if let Some(memory) = memory {
            e.set(request, IoRequest::m_pMemory, memory);
        }
        e.set(request, IoRequest::m_uiSize, size);
        fn_00aa8740(e, local);
        e.get(request, IoRequest::result)
    })
}

// Translated from 00aa85c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Seeks `file` on the IO thread (request type 1: offset, whence) and
/// returns `fseek`'s result.
pub fn fn_00aa85c0(e: &mut Engine, file: u32, offset: u32, whence: u32) -> u32 {
    run_request(e, REQUEST_SEEK, file, None, offset, Some(whence))
}

// Translated from 00aa8610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads `size` bytes of `file` into `memory` on the IO thread (request
/// type 2) and returns `fread`'s result. The second word (in `fread`'s
/// element-size position) is not read; the thread always passes 1.
pub fn fn_00aa8610(e: &mut Engine, memory: u32, _unused_1: u32, size: u32, file: u32) -> u32 {
    run_request(e, REQUEST_READ, file, Some(memory), size, None)
}

// Translated from 00aa8660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes `size` bytes from `memory` to `file` on the IO thread (request
/// type 3) and returns `fwrite`'s result. The second word is not read.
pub fn fn_00aa8660(e: &mut Engine, memory: u32, _unused_1: u32, size: u32, file: u32) -> u32 {
    run_request(e, REQUEST_WRITE, file, Some(memory), size, None)
}

// Translated from 00aa86b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues a request without waiting for it: adds it at the request list's
/// tail (under the critical section) and signals the request semaphore.
pub fn fn_00aa86b0(e: &mut Engine, request: Ptr) {
    enter_list_lock(e);
    queue_request(e, request.addr());
    leave_list_lock(e);
    let semaphore = e.global::<u32>(REQUEST_SEMAPHORE);
    semaphore_signal(e, semaphore);
}

// Translated from 00aa8740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs a request and waits for it: takes a wait semaphore from the
/// semaphore list's head into the request and queues the request (under
/// the critical section), signals the request semaphore, waits on the wait
/// semaphore, then puts the semaphore back at the list's head.
pub fn fn_00aa8740(e: &mut Engine, request: Ptr) {
    let request: Ptr<IoRequest> = request.cast();
    enter_list_lock(e);
    let list = e.global::<u32>(SEMAPHORE_LIST);
    let semaphore = fn_00aa8a80(e, Ptr::new(list));
    e.set(request, IoRequest::m_wait_handle, semaphore);
    queue_request(e, request.addr());
    leave_list_lock(e);
    let signal = e.global::<u32>(REQUEST_SEMAPHORE);
    semaphore_signal(e, signal);
    let wait = e.get(request, IoRequest::m_wait_handle);
    semaphore_wait(e, wait);
    enter_list_lock(e);
    let list = e.global::<u32>(SEMAPHORE_LIST);
    let item = e.vcall(list, SLOT_NEW_ITEM, &args![]).u32();
    let wait = e.get(request, IoRequest::m_wait_handle);
    e.set(Ptr::<BSListItem>::new(item), BSListItem::m_element, wait);
    fn_00aa8c40(e, Ptr::new(list), item);
    leave_list_lock(e);
}

// Translated from 00aa8850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSemaphore`'s vector deleting destructor: with flag 2, destroys the
/// array (`count` in the word before it) through the CRT iterator, frees
/// the block with flag 1, and returns the block; otherwise closes the one
/// semaphore's handle, frees it with flag 1, and returns `this`.
pub fn fn_00aa8850(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    if flags & 2 != 0 {
        let count = e.mem.u32(this.addr().wrapping_sub(4));
        e.call(
            VECTOR_DESTRUCTOR_ITERATOR,
            &args![this, 0xcu32, count, SEMAPHORE_DESTRUCTOR],
        );
        let block = this.addr().wrapping_sub(4);
        if flags & 1 != 0 {
            deallocate(e, block);
        }
        return Ptr::new(block);
    }
    let handle = e.mem.u32(this.addr().wrapping_add(SEMAPHORE_HANDLE_FIELD));
    e.call(CLOSE_HANDLE, &args![handle]);
    if flags & 1 != 0 {
        deallocate(e, this.addr());
    }
    this
}

/// The pointer-list destructors' shared body: empties the list between the
/// two base vtables, then frees with flag 1.
fn pointer_list_destroy(e: &mut Engine, this: Ptr, flags: u32, vtables: [u32; 2]) -> Ptr {
    e.mem.set_u32(this.addr(), vtables[0]);
    bs_list_base_io_request_remove_all(e, this.cast());
    e.mem.set_u32(this.addr(), vtables[1]);
    if flags & 1 != 0 {
        deallocate(e, this.addr());
    }
    this
}

// Translated from 00aa88d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPointerListBase<OEI::IO::IO_Request *>::scalar deleting destructor`
/// (Xbox PDB), the first slot of `BSList<OEI::IO::IO_Request *>`'s vtable:
/// empties the list, restores the base vtables and frees with flag 1.
/// Returns `this`.
// Translation incomplete: 00aa88d0 - SEH exception unwinding is unsupported.
pub fn bs_pointer_list_base_io_request_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    pointer_list_destroy(
        e,
        this,
        flags,
        [REQUEST_POINTER_LIST_BASE_VTABLE, REQUEST_LIST_BASE_VTABLE],
    )
}

// Translated from 00aa8950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSList<BSSemaphore *>::scalar deleting destructor` (Xbox PDB): empties
/// the list (through the shared `RemoveAll`), restores the base vtables
/// and frees with flag 1. Returns `this`.
// Translation incomplete: 00aa8950 - SEH exception unwinding is unsupported.
pub fn bs_list_bs_semaphore_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    pointer_list_destroy(
        e,
        this,
        flags,
        [
            SEMAPHORE_POINTER_LIST_BASE_VTABLE,
            SEMAPHORE_LIST_BASE_VTABLE,
        ],
    )
}

// Translated from 00aa89d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IO_Thread`'s scalar deleting destructor: the `BSThread` destructor
/// body (`00aa63c0`), then frees with flag 1. Returns `this`.
pub fn fn_00aa89d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(BS_THREAD_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        deallocate(e, this.addr());
    }
    this
}

// Translated from 00aa8a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSListBase<OEI::IO::IO_Request *>`'s destructor: restores its vtable.
pub fn fn_00aa8a00(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), REQUEST_LIST_BASE_VTABLE);
}

// Translated from 00aa8a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSListBase<BSSemaphore *>`'s destructor: restores its vtable.
pub fn fn_00aa8a20(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SEMAPHORE_LIST_BASE_VTABLE);
}

// Translated from 00aa8a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `*element` at the list's head in a new item (`NewItem`, slot +4).
pub fn fn_00aa8a40(e: &mut Engine, this: Ptr<BSListBase>, element: Ptr) {
    let item = e.vcall(this.addr(), SLOT_NEW_ITEM, &args![]).u32();
    let value = e.mem.u32(element.addr());
    e.set(Ptr::<BSListItem>::new(item), BSListItem::m_element, value);
    fn_00aa8c40(e, this, item);
}

// Translated from 00aa8a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the list's head item (the list must not be empty), deletes it
/// (`DeleteItem`, slot +8) and returns its element.
pub fn fn_00aa8a80(e: &mut Engine, this: Ptr<BSListBase>) -> u32 {
    let item: Ptr<BSListItem> = Ptr::new(e.get(this, BSListBase::m_pkHead));
    let next = e.get(item, BSListItem::m_pkNext);
    e.set(this, BSListBase::m_pkHead, next);
    if next != 0 {
        e.set(Ptr::<BSListItem>::new(next), BSListItem::m_pkPrev, 0);
    } else {
        e.set(this, BSListBase::m_pkTail, 0);
    }
    let element = e.get(item, BSListItem::m_element);
    e.vcall(this.addr(), SLOT_DELETE_ITEM, &args![item]);
    let count = e.get(this, BSListBase::m_uiCount).wrapping_sub(1);
    e.set(this, BSListBase::m_uiCount, count);
    element
}

// Translated from 00aa8b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NewItem` of both lists: allocates a 12-byte item (`this` is not read).
pub fn fn_00aa8b00(e: &mut Engine, _this: Ptr) -> u32 {
    allocate(e, 0xc)
}

// Translated from 00aa8b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSListBase<OEI::IO::IO_Request *>::scalar deleting destructor` (Xbox
/// PDB): restores the vtable and frees with flag 1. Returns `this`.
pub fn bs_list_base_io_request_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), REQUEST_LIST_BASE_VTABLE);
    if flags & 1 != 0 {
        deallocate(e, this.addr());
    }
    this
}

// Translated from 00aa8b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSListBase<BSSemaphore *>`'s scalar deleting destructor: restores the
/// vtable and frees with flag 1. Returns `this`.
pub fn fn_00aa8b50(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), SEMAPHORE_LIST_BASE_VTABLE);
    if flags & 1 != 0 {
        deallocate(e, this.addr());
    }
    this
}

// Translated from 00aa8b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Links `item` at the list's tail and counts it.
pub fn fn_00aa8b80(e: &mut Engine, this: Ptr<BSListBase>, item: u32) {
    let node: Ptr<BSListItem> = Ptr::new(item);
    e.set(node, BSListItem::m_pkNext, 0);
    let tail = e.get(this, BSListBase::m_pkTail);
    e.set(node, BSListItem::m_pkPrev, tail);
    if tail != 0 {
        e.set(Ptr::<BSListItem>::new(tail), BSListItem::m_pkNext, item);
    } else {
        e.set(this, BSListBase::m_pkHead, item);
    }
    e.set(this, BSListBase::m_pkTail, item);
    let count = e.get(this, BSListBase::m_uiCount).wrapping_add(1);
    e.set(this, BSListBase::m_uiCount, count);
}

// Translated from 00aa8be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSListBase<OEI::IO::IO_Request *>::RemoveAll` (Xbox PDB; both lists
/// share it): deletes every item (`DeleteItem`, slot +8) from the head and
/// empties the list.
pub fn bs_list_base_io_request_remove_all(e: &mut Engine, this: Ptr<BSListBase>) {
    let mut item = e.get(this, BSListBase::m_pkHead);
    while item != 0 {
        let current = item;
        item = e.get(Ptr::<BSListItem>::new(current), BSListItem::m_pkNext);
        e.vcall(this.addr(), SLOT_DELETE_ITEM, &args![current]);
    }
    e.set(this, BSListBase::m_uiCount, 0);
    e.set(this, BSListBase::m_pkHead, 0);
    e.set(this, BSListBase::m_pkTail, 0);
}

// Translated from 00aa8c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Links `item` at the list's head and counts it.
pub fn fn_00aa8c40(e: &mut Engine, this: Ptr<BSListBase>, item: u32) {
    let node: Ptr<BSListItem> = Ptr::new(item);
    e.set(node, BSListItem::m_pkPrev, 0);
    let head = e.get(this, BSListBase::m_pkHead);
    e.set(node, BSListItem::m_pkNext, head);
    if head != 0 {
        e.set(Ptr::<BSListItem>::new(head), BSListItem::m_pkPrev, item);
    } else {
        e.set(this, BSListBase::m_pkTail, item);
    }
    e.set(this, BSListBase::m_pkHead, item);
    let count = e.get(this, BSListBase::m_uiCount).wrapping_add(1);
    e.set(this, BSListBase::m_uiCount, count);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa7eb0, oei_io_initialize()),
        entry!(0x00aa81d0, fn_00aa81d0()),
        entry!(0x00aa8410, fn_00aa8410(Ptr) -> Ptr),
        entry!(0x00aa8430, oei_io_io_thread_thread_proc(Ptr) -> u32),
        entry!(0x00aa85c0, fn_00aa85c0(u32, u32, u32) -> u32),
        entry!(0x00aa8610, fn_00aa8610(u32, u32, u32, u32) -> u32),
        entry!(0x00aa8660, fn_00aa8660(u32, u32, u32, u32) -> u32),
        entry!(0x00aa86b0, fn_00aa86b0(Ptr)),
        entry!(0x00aa8740, fn_00aa8740(Ptr)),
        entry!(0x00aa8850, fn_00aa8850(Ptr, u32) -> Ptr),
        entry!(
            0x00aa88d0,
            bs_pointer_list_base_io_request_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00aa8950,
            bs_list_bs_semaphore_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00aa89d0, fn_00aa89d0(Ptr, u32) -> Ptr),
        entry!(0x00aa8a00, fn_00aa8a00(Ptr)),
        entry!(0x00aa8a20, fn_00aa8a20(Ptr)),
        entry!(0x00aa8a40, fn_00aa8a40(Ptr<BSListBase>, Ptr)),
        entry!(0x00aa8a80, fn_00aa8a80(Ptr<BSListBase>) -> u32),
        entry!(0x00aa8b00, fn_00aa8b00(Ptr) -> u32),
        entry!(
            0x00aa8b20,
            bs_list_base_io_request_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00aa8b50, fn_00aa8b50(Ptr, u32) -> Ptr),
        entry!(0x00aa8b80, fn_00aa8b80(Ptr<BSListBase>, u32)),
        entry!(
            0x00aa8be0,
            bs_list_base_io_request_remove_all(Ptr<BSListBase>)
        ),
        entry!(0x00aa8c40, fn_00aa8c40(Ptr<BSListBase>, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `DeleteItem` of both lists (`00aa4b20`, another unit).
    const DELETE_ITEM: u32 = 0x00aa_4b20;
    /// Where the inline "thread" double records the request it ran (a
    /// spare spot on the globals' page).
    const SEEN: u32 = 0x011f_6f00;

    fn no_op(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn interlocked_increment(e: &mut Engine, a: &[u32]) -> Ret {
        let v = e.mem.u32(a[0]).wrapping_add(1);
        e.mem.set_u32(a[0], v);
        ret(v)
    }

    fn interlocked_decrement(e: &mut Engine, a: &[u32]) -> Ret {
        let v = e.mem.u32(a[0]).wrapping_sub(1);
        e.mem.set_u32(a[0], v);
        ret(v)
    }

    /// Doubles for the imports, the CRT and the other units' callees; the
    /// globals' page and the list vtables (`NewItem` is this unit's, the
    /// destructors and `DeleteItem` are doubles).
    fn io_engine() -> Engine {
        let mut e = Engine::new();
        for addr in [
            INITIALIZE_CRITICAL_SECTION,
            DELETE_CRITICAL_SECTION,
            ENTER_CRITICAL_SECTION,
            LEAVE_CRITICAL_SECTION,
            WAIT_FOR_SINGLE_OBJECT,
            RELEASE_SEMAPHORE,
            CLOSE_HANDLE,
            BS_THREAD_CONSTRUCTOR,
            BS_THREAD_DESTRUCTOR,
            BS_THREAD_INITIALIZE,
            BS_THREAD_SHUTDOWN,
            BS_THREAD_SET_THREAD_PROCESSOR,
            VECTOR_CONSTRUCTOR_ITERATOR,
            VECTOR_DESTRUCTOR_ITERATOR,
            DELETE_ITEM,
            MEMORY_MANAGER_DEALLOCATE,
        ] {
            e.register(addr, no_op);
        }
        e.register(INTERLOCKED_INCREMENT, interlocked_increment);
        e.register(INTERLOCKED_DECREMENT, interlocked_decrement);
        e.register(CREATE_SEMAPHORE_A, |_, a| ret(0x5000 + a[2]));
        e.register(FSEEK, |_, _| ret(0));
        e.register(FREAD, |_, a| ret(a[2]));
        e.register(FWRITE, |_, a| ret(a[2] / 2));
        e.map(0x011f_6000, 0x1000);
        for vtable in [REQUEST_LIST_VTABLE, SEMAPHORE_LIST_VTABLE] {
            e.put_vtable(vtable, &[0x00aa_0001, 0x00aa_8b00, DELETE_ITEM]);
        }
        e.put_vtable(IO_THREAD_VTABLE, &[0x00aa_0002, 0x00aa_8430]);
        e.register(0x00aa_0001, no_op);
        e.register(0x00aa_0002, no_op);
        e
    }

    /// A list with the given vtable.
    fn new_list(e: &mut Engine, vtable: u32) -> Ptr<BSListBase> {
        let list: Ptr<BSListBase> = e.new_object();
        e.mem.set_u32(list.addr(), vtable);
        list
    }

    /// The list's elements from the head, checking the back links.
    fn elements(e: &Engine, list: Ptr<BSListBase>) -> Vec<u32> {
        let mut out = vec![];
        let mut previous = 0;
        let mut item = e.get(list, BSListBase::m_pkHead);
        while item != 0 {
            let node: Ptr<BSListItem> = Ptr::new(item);
            assert_eq!(e.get(node, BSListItem::m_pkPrev), previous);
            out.push(e.get(node, BSListItem::m_element));
            previous = item;
            item = e.get(node, BSListItem::m_pkNext);
        }
        assert_eq!(e.get(list, BSListBase::m_pkTail), previous);
        assert_eq!(e.get(list, BSListBase::m_uiCount), out.len() as u32);
        out
    }

    fn item(e: &mut Engine, element: u32) -> u32 {
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item + 8, element);
        item
    }

    /// Globals for one request round: semaphores, lock and both lists.
    fn set_up_globals(e: &mut Engine) {
        for global in [REQUEST_SEMAPHORE, SHUTDOWN_SEMAPHORE] {
            let s = e.mem.alloc(0xc);
            e.mem.set_u32(s + 4, 0x6000 + global);
            e.set_global(global, s);
        }
        let section = e.mem.alloc(0x18);
        e.set_global(LIST_LOCK, section);
        let requests = new_list(e, REQUEST_LIST_VTABLE);
        e.set_global(REQUEST_LIST, requests.addr());
        let semaphores = new_list(e, SEMAPHORE_LIST_VTABLE);
        e.set_global(SEMAPHORE_LIST, semaphores.addr());
    }

    #[test]
    fn initialize_builds_every_global_and_starts_the_thread() {
        let mut e = io_engine();
        e.call_log = Some(vec![]);
        e.call(0x00aa_7eb0, &args![]);
        let request_semaphore = e.global::<u32>(REQUEST_SEMAPHORE);
        assert_eq!(e.mem.u32(request_semaphore), 0);
        assert_eq!(e.mem.u32(request_semaphore + 4), 0x5000 + 0xffff);
        assert_eq!(e.mem.u32(request_semaphore + 8), 0xffff);
        let shutdown = e.global::<u32>(SHUTDOWN_SEMAPHORE);
        assert_eq!(e.mem.u32(shutdown + 8), 1);
        assert_ne!(e.global::<u32>(LIST_LOCK), 0);
        let requests = e.global::<u32>(REQUEST_LIST);
        assert_eq!(e.mem.u32(requests), REQUEST_LIST_VTABLE);
        assert!(elements(&e, Ptr::new(requests)).is_empty());
        let semaphores = e.global::<u32>(SEMAPHORE_LIST);
        assert_eq!(e.mem.u32(semaphores), SEMAPHORE_LIST_VTABLE);
        let array = e.global::<u32>(SEMAPHORE_ARRAY);
        assert_eq!(e.mem.u32(array - 4), 0x20);
        // Each added at the head: the last element first.
        let expected: Vec<u32> = (0..0x20).rev().map(|i| array + 0xc * i).collect();
        assert_eq!(elements(&e, Ptr::new(semaphores)), expected);
        let thread = e.global::<u32>(IO_THREAD);
        assert_eq!(e.mem.u32(thread), IO_THREAD_VTABLE);

        let log = e.call_log.take().unwrap();
        let create: Vec<&Vec<u32>> = log
            .iter()
            .filter(|(a, _)| *a == CREATE_SEMAPHORE_A)
            .map(|(_, w)| w)
            .collect();
        assert_eq!(*create[0], vec![0, 0, 0xffff, 0]);
        assert_eq!(*create[1], vec![0, 0, 1, 0]);
        let iterator = log
            .iter()
            .find(|(a, _)| *a == VECTOR_CONSTRUCTOR_ITERATOR)
            .unwrap();
        assert_eq!(
            iterator.1,
            vec![
                array,
                0xc,
                0x20,
                SEMAPHORE_CONSTRUCTOR,
                SEMAPHORE_DESTRUCTOR
            ]
        );
        let start = log
            .iter()
            .find(|(a, _)| *a == BS_THREAD_INITIALIZE)
            .unwrap();
        assert_eq!(start.1, vec![thread, 0x4000, THREAD_NAME, 0]);
    }

    #[test]
    fn initialize_leaves_failed_allocations_null() {
        let mut e = io_engine();
        // Every allocation fails except the semaphore list (the fifth) and
        // its 32 items (the seventh to the 38th): the loop pushes onto that
        // list whatever the array.
        e.register_double(MEMORY_MANAGER_ALLOCATE, {
            let mut n = 0;
            move |e, a| {
                n += 1;
                if n == 5 || (7..=38).contains(&n) {
                    ret(e.mem.alloc(a[1]))
                } else {
                    ret(0)
                }
            }
        });
        e.call_log = Some(vec![]);
        e.call(0x00aa_7eb0, &args![]);
        for global in [
            REQUEST_SEMAPHORE,
            SHUTDOWN_SEMAPHORE,
            LIST_LOCK,
            REQUEST_LIST,
        ] {
            assert_eq!(e.global::<u32>(global), 0);
        }
        assert_eq!(e.global::<u32>(SEMAPHORE_ARRAY), 0);
        assert_eq!(e.global::<u32>(IO_THREAD), 0);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == CREATE_SEMAPHORE_A
            || *a == INITIALIZE_CRITICAL_SECTION
            || *a == VECTOR_CONSTRUCTOR_ITERATOR
            || *a == BS_THREAD_CONSTRUCTOR));
        // With no array, the elements are the offsets i * 12.
        let semaphores = e.global::<u32>(SEMAPHORE_LIST);
        let expected: Vec<u32> = (0..0x20).rev().map(|i| 0xc * i).collect();
        assert_eq!(elements(&e, Ptr::new(semaphores)), expected);
        let start = log
            .iter()
            .find(|(a, _)| *a == BS_THREAD_INITIALIZE)
            .unwrap();
        assert_eq!(start.1[0], 0);
    }

    #[test]
    fn shutdown_queues_quit_waits_and_deletes_everything() {
        let mut e = io_engine();
        set_up_globals(&mut e);
        let thread = e.mem.alloc(0x30);
        e.mem.set_u32(thread, IO_THREAD_VTABLE);
        e.set_global(IO_THREAD, thread);
        let block = e.mem.alloc(0x184);
        e.mem.set_u32(block, 0x20);
        e.set_global(SEMAPHORE_ARRAY, block + 4);
        let shutdown = e.global::<u32>(SHUTDOWN_SEMAPHORE);
        let request_semaphore = e.global::<u32>(REQUEST_SEMAPHORE);
        let section = e.global::<u32>(LIST_LOCK);
        let requests = e.global::<u32>(REQUEST_LIST);
        let semaphores = e.global::<u32>(SEMAPHORE_LIST);
        e.call_log = Some(vec![]);
        e.call(0x00aa_81d0, &args![]);
        let log = e.call_log.take().unwrap();
        // The quit request went through the list (its item was linked).
        let signal = log
            .iter()
            .position(|(a, w)| *a == INTERLOCKED_INCREMENT && w[0] == request_semaphore)
            .unwrap();
        let wait = log
            .iter()
            .position(|(a, w)| *a == INTERLOCKED_DECREMENT && w[0] == shutdown)
            .unwrap();
        assert!(signal < wait);
        let order: Vec<(u32, u32)> = log
            .iter()
            .filter(|(a, _)| {
                [
                    0x00aa_0001,
                    0x00aa_0002,
                    VECTOR_DESTRUCTOR_ITERATOR,
                    DELETE_CRITICAL_SECTION,
                    CLOSE_HANDLE,
                    MEMORY_MANAGER_DEALLOCATE,
                    BS_THREAD_SHUTDOWN,
                ]
                .contains(a)
            })
            .map(|(a, w)| (*a, w.get(1).copied().unwrap_or(w[0])))
            .collect();
        assert_eq!(
            order,
            vec![
                (BS_THREAD_SHUTDOWN, thread),
                (0x00aa_0002, 1),
                (VECTOR_DESTRUCTOR_ITERATOR, 0xc),
                (MEMORY_MANAGER_DEALLOCATE, block),
                (0x00aa_0001, 1),
                (0x00aa_0001, 1),
                (DELETE_CRITICAL_SECTION, section),
                (MEMORY_MANAGER_DEALLOCATE, section),
                (CLOSE_HANDLE, 0x6000 + SHUTDOWN_SEMAPHORE),
                (MEMORY_MANAGER_DEALLOCATE, shutdown),
                (CLOSE_HANDLE, 0x6000 + REQUEST_SEMAPHORE),
                (MEMORY_MANAGER_DEALLOCATE, request_semaphore),
            ]
        );
        let destroyed: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == 0x00aa_0001)
            .map(|(_, w)| w[0])
            .collect();
        assert_eq!(destroyed, vec![semaphores, requests]);
        for global in [
            IO_THREAD,
            REQUEST_SEMAPHORE,
            SHUTDOWN_SEMAPHORE,
            LIST_LOCK,
            REQUEST_LIST,
            SEMAPHORE_LIST,
            SEMAPHORE_ARRAY,
        ] {
            assert_eq!(e.global::<u32>(global), 0);
        }
    }

    #[test]
    fn shutdown_skips_null_globals() {
        let mut e = io_engine();
        set_up_globals(&mut e);
        // The thread and the array are null.
        e.call_log = Some(vec![]);
        e.call(0x00aa_81d0, &args![]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == 0x00aa_0002));
        assert!(!log.iter().any(|(a, _)| *a == VECTOR_DESTRUCTOR_ITERATOR));
        let shutdown_call = log.iter().find(|(a, _)| *a == BS_THREAD_SHUTDOWN).unwrap();
        assert_eq!(shutdown_call.1, vec![0]);
        assert_eq!(e.global::<u32>(REQUEST_LIST), 0);
    }

    #[test]
    fn constructor_sets_the_vtable() {
        let mut e = io_engine();
        let thread = Ptr::new(e.mem.alloc(0x30));
        e.call_log = Some(vec![]);
        let back = e.call(0x00aa_8410, &args![thread]).ptr::<()>();
        assert_eq!(back, thread);
        assert_eq!(e.mem.u32(thread.addr()), IO_THREAD_VTABLE);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|(a, w)| *a == BS_THREAD_CONSTRUCTOR && w[0] == thread.addr()));
    }

    /// A request with the given fields; returns its address.
    fn request(e: &mut Engine, kind: u32, file: u32, memory: u32, size: u32, whence: u32) -> u32 {
        let r: Ptr<IoRequest> = e.new_object();
        e.set(r, IoRequest::m_eType, kind);
        e.set(r, IoRequest::m_pFile, file);
        e.set(r, IoRequest::m_pMemory, memory);
        e.set(r, IoRequest::m_uiSize, size);
        e.set(r, IoRequest::m_eWhence, whence);
        r.addr()
    }

    #[test]
    fn thread_proc_runs_each_request_until_quit() {
        let mut e = io_engine();
        set_up_globals(&mut e);
        let requests: Ptr<BSListBase> = Ptr::new(e.global::<u32>(REQUEST_LIST));
        let wait = e.mem.alloc(0xc);
        e.mem.set_u32(wait + 4, 0x7777);
        let seek = request(&mut e, REQUEST_SEEK, 0x100, 0, 0x40, 2);
        let read = request(&mut e, REQUEST_READ, 0x100, 0x2000, 0x30, 0);
        e.set(Ptr::<IoRequest>::new(read), IoRequest::m_wait_handle, wait);
        let write = request(&mut e, REQUEST_WRITE, 0x100, 0x3000, 0x20, 0);
        let other = request(&mut e, 9, 0, 0, 0, 0);
        let quit = request(&mut e, REQUEST_QUIT, 0, 0, 0, 0);
        let after = request(&mut e, REQUEST_READ, 0, 0, 0, 0);
        for r in [seek, read, write, other, quit, after] {
            let i = item(&mut e, r);
            e.call(0x00aa_8b80, &args![requests, i]);
        }
        e.register(FSEEK, |_, _| ret(0xffff_ffff));
        let thread = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_8430, &args![thread]).u32(), 0);
        let log = e.call_log.take().unwrap();
        let processor = log
            .iter()
            .find(|(a, _)| *a == BS_THREAD_SET_THREAD_PROCESSOR)
            .unwrap();
        assert_eq!(processor.1, vec![thread, 1]);
        let get = |a: u32, f: Field<IoRequest, u32>| e.get(Ptr::<IoRequest>::new(a), f);
        assert_eq!(get(seek, IoRequest::result), 0xffff_ffff);
        assert_eq!(get(read, IoRequest::result), 0x30);
        assert_eq!(get(write, IoRequest::result), 0x10);
        assert_eq!(get(other, IoRequest::result), 0);
        let call = |addr: u32| log.iter().find(|(a, _)| *a == addr).unwrap().1.clone();
        assert_eq!(call(FSEEK), vec![0x100, 0x40, 2]);
        assert_eq!(call(FREAD), vec![0x2000, 1, 0x30, 0x100]);
        assert_eq!(call(FWRITE), vec![0x3000, 1, 0x20, 0x100]);
        // The read's waiter was signalled; the quit signalled shutdown.
        let released: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == RELEASE_SEMAPHORE)
            .map(|(_, w)| w[0])
            .collect();
        assert_eq!(released, vec![0x7777, 0x6000 + SHUTDOWN_SEMAPHORE]);
        assert_eq!(e.mem.u32(wait), 1);
        // The request after the quit is still queued.
        assert_eq!(elements(&e, requests), vec![after]);
        let waits = log
            .iter()
            .filter(|(a, _)| *a == WAIT_FOR_SINGLE_OBJECT)
            .count();
        assert_eq!(waits, 5);
    }

    /// A double for `fn_00aa8740`'s view of the thread: when the request
    /// semaphore is released, the "thread" runs the queued request at once
    /// by storing `0x55` as its result.
    fn run_requests_inline(e: &mut Engine) {
        e.register(RELEASE_SEMAPHORE, |e, _| {
            let list = Ptr::new(e.global::<u32>(REQUEST_LIST));
            if e.get(list, BSListBase::m_uiCount) != 0 {
                let request = fn_00aa8a80(e, list);
                e.mem.set_u32(request + 0x18, 0x55);
                // Records the request's first six words where the test
                // reads them.
                for i in 0..6 {
                    let word = e.mem.u32(request + 4 * i);
                    e.mem.set_u32(SEEN + 4 * i, word);
                }
            }
            Ret::default()
        });
    }

    fn with_one_wait_semaphore(e: &mut Engine) -> u32 {
        set_up_globals(e);
        let semaphores: Ptr<BSListBase> = Ptr::new(e.global::<u32>(SEMAPHORE_LIST));
        let wait = e.mem.alloc(0xc);
        e.mem.set_u32(wait + 4, 0x8888);
        let i = item(e, wait);
        e.call(0x00aa_8c40, &args![semaphores, i]);
        wait
    }

    #[test]
    fn seek_read_and_write_build_their_requests() {
        let mut e = io_engine();
        run_requests_inline(&mut e);
        let wait = with_one_wait_semaphore(&mut e);
        // (file, type, whence, memory, size, wait) as the thread saw them.
        assert_eq!(
            e.call(0x00aa_85c0, &args![0x100u32, 0x40u32, 2u32]).u32(),
            0x55
        );
        let seen: Vec<u32> = (0..6).map(|i| e.mem.u32(SEEN + 4 * i)).collect();
        assert_eq!(seen, vec![0x100, 1, 2, 0, 0x40, wait]);
        assert_eq!(
            e.call(0x00aa_8610, &args![0x2000u32, 9u32, 0x30u32, 0x100u32])
                .u32(),
            0x55
        );
        let seen: Vec<u32> = (0..6).map(|i| e.mem.u32(SEEN + 4 * i)).collect();
        assert_eq!(seen, vec![0x100, 2, 0, 0x2000, 0x30, wait]);
        assert_eq!(
            e.call(0x00aa_8660, &args![0x3000u32, 9u32, 0x20u32, 0x100u32])
                .u32(),
            0x55
        );
        let seen: Vec<u32> = (0..6).map(|i| e.mem.u32(SEEN + 4 * i)).collect();
        assert_eq!(seen, vec![0x100, 3, 0, 0x3000, 0x20, wait]);
        // The semaphore is back in the list each time.
        let semaphores = Ptr::new(e.global::<u32>(SEMAPHORE_LIST));
        assert_eq!(elements(&e, semaphores), vec![wait]);
    }

    #[test]
    fn queue_without_wait_adds_at_the_tail_and_signals() {
        let mut e = io_engine();
        set_up_globals(&mut e);
        let requests: Ptr<BSListBase> = Ptr::new(e.global::<u32>(REQUEST_LIST));
        let request_semaphore = e.global::<u32>(REQUEST_SEMAPHORE);
        let first = item(&mut e, 0x1111);
        e.call(0x00aa_8b80, &args![requests, first]);
        e.call_log = Some(vec![]);
        e.call(0x00aa_86b0, &args![0x2222u32]);
        assert_eq!(elements(&e, requests), vec![0x1111, 0x2222]);
        assert_eq!(e.mem.u32(request_semaphore), 1);
        let log = e.call_log.take().unwrap();
        let enter = log
            .iter()
            .position(|(a, _)| *a == ENTER_CRITICAL_SECTION)
            .unwrap();
        let leave = log
            .iter()
            .position(|(a, _)| *a == LEAVE_CRITICAL_SECTION)
            .unwrap();
        let release = log
            .iter()
            .position(|(a, _)| *a == RELEASE_SEMAPHORE)
            .unwrap();
        assert!(enter < leave && leave < release);
        assert_eq!(log[release].1, vec![0x6000 + REQUEST_SEMAPHORE, 1, 0]);
    }

    #[test]
    fn run_and_wait_borrows_and_returns_a_wait_semaphore() {
        let mut e = io_engine();
        let wait = with_one_wait_semaphore(&mut e);
        let requests: Ptr<BSListBase> = Ptr::new(e.global::<u32>(REQUEST_LIST));
        let r = request(&mut e, REQUEST_READ, 0, 0, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x00aa_8740, &args![r]);
        assert_eq!(
            e.get(Ptr::<IoRequest>::new(r), IoRequest::m_wait_handle),
            wait
        );
        // Nobody ran it: still queued.
        assert_eq!(elements(&e, requests), vec![r]);
        let semaphores = Ptr::new(e.global::<u32>(SEMAPHORE_LIST));
        assert_eq!(elements(&e, semaphores), vec![wait]);
        let log = e.call_log.take().unwrap();
        let waited = log
            .iter()
            .find(|(a, _)| *a == WAIT_FOR_SINGLE_OBJECT)
            .unwrap();
        assert_eq!(waited.1, vec![0x8888, INFINITE]);
        assert_eq!(e.mem.u32(wait), 0xffff_ffff);
    }

    #[test]
    fn semaphore_array_and_single_destructors() {
        let mut e = io_engine();
        let block = e.mem.alloc(0x184);
        e.mem.set_u32(block, 0x20);
        e.call_log = Some(vec![]);
        let back = e.call(0x00aa_8850, &args![block + 4, 3u32]).u32();
        assert_eq!(back, block);
        let log = e.call_log.take().unwrap();
        let iterator = log
            .iter()
            .find(|(a, _)| *a == VECTOR_DESTRUCTOR_ITERATOR)
            .unwrap();
        assert_eq!(iterator.1, vec![block + 4, 0xc, 0x20, SEMAPHORE_DESTRUCTOR]);
        assert!(log
            .iter()
            .any(|(a, w)| *a == MEMORY_MANAGER_DEALLOCATE && w[1] == block));

        // Flag 2 without 1: no free.
        e.call_log = Some(vec![]);
        e.call(0x00aa_8850, &args![block + 4, 2u32]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == MEMORY_MANAGER_DEALLOCATE));

        // One semaphore: close its handle, free with flag 1.
        let s = e.mem.alloc(0xc);
        e.mem.set_u32(s + 4, 0x4242);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_8850, &args![s, 1u32]).u32(), s);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|(a, w)| *a == CLOSE_HANDLE && w[0] == 0x4242));
        assert!(log
            .iter()
            .any(|(a, w)| *a == MEMORY_MANAGER_DEALLOCATE && w[1] == s));
        e.call_log = Some(vec![]);
        e.call(0x00aa_8850, &args![s, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == MEMORY_MANAGER_DEALLOCATE));
    }

    #[test]
    fn pointer_list_destructors_empty_the_list_and_restore_bases() {
        let cases = [
            (
                0x00aa_88d0u32,
                REQUEST_LIST_VTABLE,
                REQUEST_LIST_BASE_VTABLE,
            ),
            (
                0x00aa_8950,
                SEMAPHORE_LIST_VTABLE,
                SEMAPHORE_LIST_BASE_VTABLE,
            ),
        ];
        for (addr, vtable, base) in cases {
            let mut e = io_engine();
            let list = new_list(&mut e, vtable);
            let a = item(&mut e, 1);
            let b = item(&mut e, 2);
            e.call(0x00aa_8b80, &args![list, a]);
            e.call(0x00aa_8b80, &args![list, b]);
            // RemoveAll runs between the two base vtables: its DeleteItem
            // calls go through the pointer-list base's table.
            let pointer_base = if vtable == REQUEST_LIST_VTABLE {
                REQUEST_POINTER_LIST_BASE_VTABLE
            } else {
                SEMAPHORE_POINTER_LIST_BASE_VTABLE
            };
            e.put_vtable(pointer_base, &[0, 0x00aa_8b00, DELETE_ITEM]);
            e.call_log = Some(vec![]);
            let back = e.call(addr, &args![list, 1u32]).ptr::<()>();
            assert_eq!(back.addr(), list.addr());
            assert_eq!(e.mem.u32(list.addr()), base);
            assert!(elements(&e, list).is_empty());
            let log = e.call_log.take().unwrap();
            let deleted: Vec<u32> = log
                .iter()
                .filter(|(a, _)| *a == DELETE_ITEM)
                .map(|(_, w)| w[1])
                .collect();
            assert_eq!(deleted, vec![a, b]);
            assert!(log
                .iter()
                .any(|(a, w)| *a == MEMORY_MANAGER_DEALLOCATE && w[1] == list.addr()));
            e.call_log = Some(vec![]);
            e.call(addr, &args![list, 0u32]);
            let log = e.call_log.take().unwrap();
            assert!(!log.iter().any(|(a, _)| *a == MEMORY_MANAGER_DEALLOCATE));
        }
    }

    #[test]
    fn io_thread_destructor_runs_the_base_and_frees_with_flag_1() {
        let mut e = io_engine();
        let thread = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_89d0, &args![thread, 1u32]).u32(), thread);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|(a, w)| *a == BS_THREAD_DESTRUCTOR && w[0] == thread));
        assert!(log
            .iter()
            .any(|(a, w)| *a == MEMORY_MANAGER_DEALLOCATE && w[1] == thread));
        e.call_log = Some(vec![]);
        e.call(0x00aa_89d0, &args![thread, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == MEMORY_MANAGER_DEALLOCATE));
    }

    #[test]
    fn list_base_destructors_restore_their_vtables() {
        let mut e = io_engine();
        let list = e.mem.alloc(0x10);
        e.call(0x00aa_8a00, &args![list]);
        assert_eq!(e.mem.u32(list), REQUEST_LIST_BASE_VTABLE);
        e.call(0x00aa_8a20, &args![list]);
        assert_eq!(e.mem.u32(list), SEMAPHORE_LIST_BASE_VTABLE);
        for (addr, vtable) in [
            (0x00aa_8b20u32, REQUEST_LIST_BASE_VTABLE),
            (0x00aa_8b50, SEMAPHORE_LIST_BASE_VTABLE),
        ] {
            e.mem.set_u32(list, 0);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(addr, &args![list, 1u32]).u32(), list);
            assert_eq!(e.mem.u32(list), vtable);
            let log = e.call_log.take().unwrap();
            assert!(log
                .iter()
                .any(|(a, w)| *a == MEMORY_MANAGER_DEALLOCATE && w[1] == list));
            e.call_log = Some(vec![]);
            e.call(addr, &args![list, 0u32]);
            let log = e.call_log.take().unwrap();
            assert!(!log.iter().any(|(a, _)| *a == MEMORY_MANAGER_DEALLOCATE));
        }
    }

    #[test]
    fn add_element_at_head_through_new_item() {
        let mut e = io_engine();
        let list = new_list(&mut e, SEMAPHORE_LIST_VTABLE);
        let element = e.mem.alloc(4);
        for value in [0x10u32, 0x20] {
            e.mem.set_u32(element, value);
            e.call(0x00aa_8a40, &args![list, element]);
        }
        assert_eq!(elements(&e, list), vec![0x20, 0x10]);
    }

    #[test]
    fn remove_head_returns_the_element_and_deletes_the_item() {
        let mut e = io_engine();
        let list = new_list(&mut e, REQUEST_LIST_VTABLE);
        let a = item(&mut e, 0xa);
        let b = item(&mut e, 0xb);
        e.call(0x00aa_8b80, &args![list, a]);
        e.call(0x00aa_8b80, &args![list, b]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_8a80, &args![list]).u32(), 0xa);
        assert_eq!(elements(&e, list), vec![0xb]);
        assert_eq!(e.call(0x00aa_8a80, &args![list]).u32(), 0xb);
        assert!(elements(&e, list).is_empty());
        let log = e.call_log.take().unwrap();
        let deleted: Vec<Vec<u32>> = log
            .iter()
            .filter(|(addr, _)| *addr == DELETE_ITEM)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(deleted, vec![vec![list.addr(), a], vec![list.addr(), b]]);
    }

    #[test]
    fn new_item_allocates_12_bytes() {
        let mut e = io_engine();
        e.call_log = Some(vec![]);
        let item = e.call(0x00aa_8b00, &args![0u32]).u32();
        assert_ne!(item, 0);
        let log = e.call_log.take().unwrap();
        let alloc = log
            .iter()
            .find(|(a, _)| *a == MEMORY_MANAGER_ALLOCATE)
            .unwrap();
        assert_eq!(alloc.1, vec![MEMORY_MANAGER, 0xc]);
    }

    #[test]
    fn tail_and_head_links() {
        let mut e = io_engine();
        let list = new_list(&mut e, REQUEST_LIST_VTABLE);
        let a = item(&mut e, 1);
        let b = item(&mut e, 2);
        let c = item(&mut e, 3);
        e.call(0x00aa_8b80, &args![list, a]);
        e.call(0x00aa_8b80, &args![list, b]);
        e.call(0x00aa_8c40, &args![list, c]);
        assert_eq!(elements(&e, list), vec![3, 1, 2]);
        let other = new_list(&mut e, REQUEST_LIST_VTABLE);
        let d = item(&mut e, 4);
        e.call(0x00aa_8c40, &args![other, d]);
        assert_eq!(elements(&e, other), vec![4]);
    }

    #[test]
    fn remove_all_deletes_every_item() {
        let mut e = io_engine();
        let list = new_list(&mut e, SEMAPHORE_LIST_VTABLE);
        let items: Vec<u32> = (0..3).map(|i| item(&mut e, i)).collect();
        for i in &items {
            e.call(0x00aa_8b80, &args![list, *i]);
        }
        e.call_log = Some(vec![]);
        e.call(0x00aa_8be0, &args![list]);
        assert!(elements(&e, list).is_empty());
        let log = e.call_log.take().unwrap();
        let deleted: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == DELETE_ITEM)
            .map(|(_, w)| w[1])
            .collect();
        assert_eq!(deleted, items);
    }
}
